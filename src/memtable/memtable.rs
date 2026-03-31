use crate::{
    common::types::{EntryValue, Key, Value},
    db::errors::Result,
};

pub trait MemTable {
    fn put(&mut self, key: Key, value: Value) -> Result<()>;
    fn get(&mut self, key: &[u8]) -> Option<&EntryValue>;
    fn delete(&mut self, key: Key) -> Result<()>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
