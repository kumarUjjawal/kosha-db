use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    common::types::{EntryValue, Key, Value},
    db::{
        errors::{DbError, Result},
        options::DbOptions,
        wal::{self, Wal, WalRecord},
    },
    memtable::{MemTable, SkipListMemTable},
};

#[derive(Debug)]
pub struct Db {
    options: DbOptions,
    memtable: SkipListMemTable,
    wal: Wal,
}

impl Db {
    pub fn open(path: impl AsRef<Path>, options: DbOptions) -> Result<Self> {
        let db_path = path.as_ref().to_path_buf();
        Self::ensure_db_directory(&db_path, options.create_if_missing)?;

        let wal_path = db_path.join(wal::WAL_FILE_NAME);
        let mut memtable = SkipListMemTable::new();

        for record in wal::replay(&wal_path)? {
            match record {
                WalRecord::Put(key, value) => memtable.put(key, value)?,
                WalRecord::Delete(key) => memtable.delete(key)?,
            }
        }

        let wal = Wal::open(&wal_path)?;

        Ok(Self {
            options,
            memtable,
            wal,
        })
    }

    pub fn put(&mut self, key: Key, value: Value) -> Result<()> {
        self.wal.append(
            &WalRecord::Put(key.clone(), value.clone()),
            self.options.sync_write,
        )?;
        self.memtable.put(key, value)
    }

    pub fn delete(&mut self, key: Key) -> Result<()> {
        self.wal
            .append(&WalRecord::Delete(key.clone()), self.options.sync_write)?;
        self.memtable.delete(key)
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Value> {
        match self.memtable.get(key) {
            Some(EntryValue::Value(value)) => Some(value.clone()),
            Some(EntryValue::Tombstone) | None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.memtable.len()
    }

    pub fn is_empty(&self) -> bool {
        self.memtable.is_empty()
    }

    fn ensure_db_directory(path: &PathBuf, create_if_missing: bool) -> Result<()> {
        if path.exists() {
            if path.is_dir() {
                return Ok(());
            }
            return Err(DbError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "database path is not a directory",
            )));
        }

        if !create_if_missing {
            return Err(DbError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "database directory does not exist",
            )));
        }

        fs::create_dir_all(path)?;
        Ok(())
    }
}
