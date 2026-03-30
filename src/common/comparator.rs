use std::cmp::Ordering;

pub trait Comparator: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn compare(&self, left: &[u8], right: &[u8]) -> Ordering;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytewiseComparator;

impl Comparator for BytewiseComparator {
    fn name(&self) -> &'static str {
        "byteswise"
    }

    fn compare(&self, left: &[u8], right: &[u8]) -> Ordering {
        left.cmp(right)
    }
}
