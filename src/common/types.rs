pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryValue {
    Value(Value),
    Tombstone,
}

impl EntryValue {
    pub fn is_tombstone(&self) -> bool {
        matches!(self, Self::Tombstone)
    }

    pub fn as_value(&self) -> Option<&[u8]> {
        match self {
            Self::Value(value) => Some(value.as_slice()),
            Self::Tombstone => None,
        }
    }
}
