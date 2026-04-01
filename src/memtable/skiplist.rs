use std::cmp::Ordering;

use crate::{
    common::comparator::{BytewiseComparator, Comparator},
    common::types::{EntryValue, Key, Value},
    db::errors::{DbError, Result},
};

use super::memtable::MemTable;

const MAX_HEIGHT: usize = 12;

#[derive(Debug, Clone)]
struct Node {
    key: Key,
    value: EntryValue,
    next: Vec<Option<usize>>,
}

impl Node {
    fn new(key: Key, value: EntryValue, height: usize) -> Self {
        Self {
            key,
            value,
            next: vec![None; height],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkipListMemTable<C = BytewiseComparator> {
    comparator: C,
    head: Vec<Option<usize>>,
    nodes: Vec<Node>,
    height: usize,
    len: usize,
    rng_state: u64,
}

impl SkipListMemTable<BytewiseComparator> {
    pub fn new() -> Self {
        Self::with_comparator(BytewiseComparator)
    }
}

impl Default for SkipListMemTable<BytewiseComparator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Comparator> SkipListMemTable<C> {
    pub fn with_comparator(comparator: C) -> Self {
        Self {
            comparator,
            head: vec![None; MAX_HEIGHT],
            nodes: Vec::new(),
            height: 1,
            len: 0,
            rng_state: 0x9E37_79B9_7F4A_7C15,
        }
    }
}
