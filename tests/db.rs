use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use koshadb::{Db, DbError, DbOptions};

const WAL_FILE_NAME: &str = "wal.log";

struct TempDbDir {
    path: PathBuf,
}

impl TempDbDir {
    fn new(label: &str) -> Self {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();

        path.push(format!("koshadb-{label}-{}-{nanos}", std::process::id()));
        Self { path }
    }

    fn as_path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDbDir {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[test]
fn open_fails_when_directory_missing_and_create_if_missing_disabled() {
    let temp_dir = TempDbDir::new("open-missing");
    let options = DbOptions {
        create_if_missing: false,
        ..DbOptions::default()
    };

    let result = Db::open(temp_dir.as_path(), options);

    assert!(matches!(
        result,
        Err(DbError::Io(err)) if err.kind() == ErrorKind::NotFound
    ));
}

#[test]
fn put_get_delete_work_through_db_api() {
    let temp_dir = TempDbDir::new("db-api");
    let mut db = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();

    assert!(db.is_empty());

    db.put(b"name".to_vec(), b"kosha".to_vec()).unwrap();
    assert_eq!(db.get(b"name"), Some(b"kosha".to_vec()));
    assert_eq!(db.len(), 1);

    db.delete(b"name".to_vec()).unwrap();
    assert_eq!(db.get(b"name"), None);
    assert_eq!(db.len(), 1);
}

#[test]
fn reopen_recovers_puts_and_deletes_from_wal() {
    let temp_dir = TempDbDir::new("recovery");

    {
        let mut db = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();
        db.put(b"k1".to_vec(), b"v1".to_vec()).unwrap();
        db.put(b"k2".to_vec(), b"v2".to_vec()).unwrap();
        db.delete(b"k2".to_vec()).unwrap();
    }

    let mut recovered = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();

    assert_eq!(recovered.get(b"k1"), Some(b"v1".to_vec()));
    assert_eq!(recovered.get(b"k2"), None);
    assert_eq!(recovered.len(), 2);
}

#[test]
fn reopen_ignores_partial_wal_tail() {
    let temp_dir = TempDbDir::new("partial-tail");

    {
        let mut db = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();
        db.put(b"stable".to_vec(), b"value".to_vec()).unwrap();
    }

    let wal_path = temp_dir.as_path().join(WAL_FILE_NAME);
    let mut wal_file = OpenOptions::new().append(true).open(wal_path).unwrap();

    wal_file.write_all(&32_u32.to_le_bytes()).unwrap();
    wal_file.write_all(&[1_u8, 0_u8, 0_u8]).unwrap();
    wal_file.flush().unwrap();

    let mut recovered = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();

    assert_eq!(recovered.get(b"stable"), Some(b"value".to_vec()));
}

#[test]
fn reopen_ignores_checksum_mismatch_tail() {
    let temp_dir = TempDbDir::new("bad-checksum-tail");

    {
        let mut db = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();
        db.put(b"stable".to_vec(), b"value".to_vec()).unwrap();
    }

    let wal_path = temp_dir.as_path().join(WAL_FILE_NAME);
    let mut wal_file = OpenOptions::new().append(true).open(wal_path).unwrap();
    let payload = encode_put_payload(b"bad", b"tail");

    wal_file
        .write_all(&(payload.len() as u32).to_le_bytes())
        .unwrap();
    wal_file.write_all(&payload).unwrap();
    wal_file.write_all(&0_u32.to_le_bytes()).unwrap();
    wal_file.flush().unwrap();

    let mut recovered = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();

    assert_eq!(recovered.get(b"stable"), Some(b"value".to_vec()));
}

#[test]
fn reopen_ignores_malformed_but_checksum_valid_tail() {
    let temp_dir = TempDbDir::new("malformed-tail");

    {
        let mut db = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();
        db.put(b"stable".to_vec(), b"value".to_vec()).unwrap();
    }

    let wal_path = temp_dir.as_path().join(WAL_FILE_NAME);
    let mut wal_file = OpenOptions::new().append(true).open(wal_path).unwrap();
    let payload = vec![99_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    let checksum = crc32fast::hash(&payload);

    wal_file
        .write_all(&(payload.len() as u32).to_le_bytes())
        .unwrap();
    wal_file.write_all(&payload).unwrap();
    wal_file.write_all(&checksum.to_le_bytes()).unwrap();
    wal_file.flush().unwrap();

    let mut recovered = Db::open(temp_dir.as_path(), DbOptions::default()).unwrap();

    assert_eq!(recovered.get(b"stable"), Some(b"value".to_vec()));
}

fn encode_put_payload(key: &[u8], value: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(1 + 4 + key.len() + 4 + value.len());
    payload.push(1_u8);
    payload.extend_from_slice(&(key.len() as u32).to_le_bytes());
    payload.extend_from_slice(key);
    payload.extend_from_slice(&(value.len() as u32).to_le_bytes());
    payload.extend_from_slice(value);
    payload
}
