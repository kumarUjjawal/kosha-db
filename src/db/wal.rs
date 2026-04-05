use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use crate::{
    common::types::{Key, Value},
    db::errors::{DbError, Result},
};

pub(crate) const WAL_FILE_NAME: &str = "wal.log";

const OP_PUT: u8 = 1;
const OP_DELETE: u8 = 2;
const MAX_RECORD_SIZE: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WalRecord {
    Put(Key, Value),
    Delete(Key),
}

#[derive(Debug)]
pub(crate) struct Wal {
    file: File,
}

impl Wal {
    pub(crate) fn open(path: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)?;
        Ok(Self { file })
    }

    pub(crate) fn append(&mut self, record: &WalRecord, sync_write: bool) -> Result<()> {
        let payload = encode_record(record)?;
        let payload_len_u32 = u32::try_from(payload.len())
            .map_err(|_| DbError::Corruption("wal payload length overflow"))?;
        let checksum = crc32fast::hash(&payload);

        self.file.write_all(&payload_len_u32.to_le_bytes())?;
        self.file.write_all(&payload)?;
        self.file.write_all(&checksum.to_le_bytes())?;

        if sync_write {
            self.file.sync_data()?;
        }

        Ok(())
    }
}

pub(crate) fn replay(path: &Path) -> Result<Vec<WalRecord>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut records = Vec::new();

    loop {
        let Some(payload_len) = read_payload_len(&mut file)? else {
            break;
        };

        if payload_len > MAX_RECORD_SIZE {
            break;
        }

        let Some(payload) = read_exact_or_tail(&mut file, payload_len)? else {
            break;
        };

        let Some(checksum_bytes) = read_exact_or_tail(&mut file, 4)? else {
            break;
        };

        let mut checksum_buf = [0_u8; 4];
        checksum_buf.copy_from_slice(&checksum_bytes);
        let expected_checksum = u32::from_le_bytes(checksum_buf);
        let actual_checksum = crc32fast::hash(&payload);

        if actual_checksum != expected_checksum {
            break;
        }

        match decode_record(&payload) {
            Ok(record) => records.push(record),
            Err(DbError::Corruption(_)) => break,
            Err(err) => return Err(err),
        }
    }

    Ok(records)
}

fn read_payload_len(file: &mut File) -> Result<Option<usize>> {
    let Some(bytes) = read_exact_or_tail(file, 4)? else {
        return Ok(None);
    };

    let mut len_buf = [0_u8; 4];
    len_buf.copy_from_slice(&bytes);
    Ok(Some(u32::from_le_bytes(len_buf) as usize))
}

fn read_exact_or_tail(file: &mut File, len: usize) -> Result<Option<Vec<u8>>> {
    let mut buf = vec![0_u8; len];

    match file.read_exact(&mut buf) {
        Ok(()) => Ok(Some(buf)),
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(err) => Err(err.into()),
    }
}

fn encode_record(record: &WalRecord) -> Result<Vec<u8>> {
    match record {
        WalRecord::Put(key, value) => {
            ensure_len_u32(key.len(), "key")?;
            ensure_len_u32(value.len(), "value")?;

            let mut payload = Vec::with_capacity(1 + 4 + key.len() + 4 + value.len());
            payload.push(OP_PUT);
            push_bytes_with_len(&mut payload, key);
            push_bytes_with_len(&mut payload, value);

            ensure_payload_size(payload.len())?;
            Ok(payload)
        }
        WalRecord::Delete(key) => {
            ensure_len_u32(key.len(), "key")?;

            let mut payload = Vec::with_capacity(1 + 4 + key.len());
            payload.push(OP_DELETE);
            push_bytes_with_len(&mut payload, key);

            ensure_payload_size(payload.len())?;
            Ok(payload)
        }
    }
}

fn decode_record(payload: &[u8]) -> Result<WalRecord> {
    if payload.is_empty() {
        return Err(DbError::Corruption("wal payload is empty"));
    }

    let op = payload[0];
    let mut cursor = 1;

    let key = read_len_prefixed_bytes(payload, &mut cursor, "key")?;

    match op {
        OP_PUT => {
            let value = read_len_prefixed_bytes(payload, &mut cursor, "value")?;
            if cursor != payload.len() {
                return Err(DbError::Corruption("wal put record has trailing bytes"));
            }
            Ok(WalRecord::Put(key, value))
        }
        OP_DELETE => {
            if cursor != payload.len() {
                return Err(DbError::Corruption("wal delete record has trailing bytes"));
            }
            Ok(WalRecord::Delete(key))
        }
        _ => Err(DbError::Corruption("unknown wal operation")),
    }
}

fn read_len_prefixed_bytes(
    payload: &[u8],
    cursor: &mut usize,
    field: &'static str,
) -> Result<Vec<u8>> {
    if payload.len().saturating_sub(*cursor) < 4 {
        return Err(DbError::Corruption(match field {
            "key" => "wal key length is truncated",
            "value" => "wal value length is truncated",
            _ => "wal field length is truncated",
        }));
    }

    let mut len_buf = [0_u8; 4];
    len_buf.copy_from_slice(&payload[*cursor..*cursor + 4]);
    *cursor += 4;
    let field_len = u32::from_le_bytes(len_buf) as usize;

    let end = cursor
        .checked_add(field_len)
        .ok_or(DbError::Corruption("wal field length overflow"))?;
    if end > payload.len() {
        return Err(DbError::Corruption(match field {
            "key" => "wal key bytes are truncated",
            "value" => "wal value bytes are truncated",
            _ => "wal field bytes are truncated",
        }));
    }

    let bytes = payload[*cursor..end].to_vec();
    *cursor = end;
    Ok(bytes)
}

fn push_bytes_with_len(buffer: &mut Vec<u8>, bytes: &[u8]) {
    let len = bytes.len() as u32;
    buffer.extend_from_slice(&len.to_le_bytes());
    buffer.extend_from_slice(bytes);
}

fn ensure_len_u32(len: usize, field: &'static str) -> Result<()> {
    if u32::try_from(len).is_err() {
        return Err(DbError::Corruption(match field {
            "key" => "key is too large for wal record",
            "value" => "value is too large for wal record",
            _ => "field is too large for wal record",
        }));
    }
    Ok(())
}

fn ensure_payload_size(len: usize) -> Result<()> {
    if len > MAX_RECORD_SIZE {
        return Err(DbError::Corruption("wal record is too large"));
    }
    Ok(())
}
