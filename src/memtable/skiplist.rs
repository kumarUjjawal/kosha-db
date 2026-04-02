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

    fn validate_key(key: &[u8]) -> Result<()> {
        if key.is_empty() {
            return Err(DbError::EmptyKey);
        }
        Ok(())
    }

    fn next_index(&self, current: Option<usize>, level: usize) -> Option<usize> {
        match current {
            None => self.head[level],
            Some(index) => self.nodes[index].next[level],
        }
    }

    fn set_next(&mut self, current: Option<usize>, level: usize, next: Option<usize>) {
        match current {
            None => self.head[level] = next,
            Some(index) => self.nodes[index].next[level] = next,
        }
    }

    fn find_preprocessor(&self, key: &[u8]) -> [Option<usize>; MAX_HEIGHT] {
        let mut update = [None; MAX_HEIGHT];
        let mut current = None;

        for level in (0..self.height).rev() {
            loop {
                let Some(next_index) = self.next_index(current, level) else {
                    break;
                };

                match self
                    .comparator
                    .compare(self.nodes[next_index].key.as_slice(), key)
                {
                    Ordering::Less => current = Some(next_index),
                    Ordering::Equal | Ordering::Greater => break,
                }
            }

            update[level] = current;
        }
        update
    }

    fn find_existing_index(
        &self,
        key: &[u8],
        update: &[Option<usize>; MAX_HEIGHT],
    ) -> Option<usize> {
        let next = self.next_index(update[0], 0)?;

        match self
            .comparator
            .compare(self.nodes[next].key.as_slice(), key)
        {
            Ordering::Equal => Some(next),
            Ordering::Less | Ordering::Greater => None,
        }
    }

    fn next_random(&mut self) -> u64 {
        let mut value = self.rng_state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.rng_state = value;
        value
    }

    fn random_height(&mut self) -> usize {
        let mut height = 1;

        while height < MAX_HEIGHT && (self.next_random() & 0b11) == 0 {
            height += 1;
        }

        height
    }

    fn insert_entry(&mut self, key: Key, value: EntryValue) -> Result<()> {
        Self::validate_key(&key)?;

        let mut update = self.find_preprocessor(&key);

        if let Some(existing_index) = self.find_existing_index(&key, &update) {
            self.nodes[existing_index].value = value;
            return Ok(());
        }

        let node_height = self.random_height();

        if node_height > self.height {
            for slot in update.iter_mut().take(node_height).skip(self.height) {
                *slot = None;
            }
            self.height = node_height;
        }

        let mut node = Node::new(key, value, node_height);

        for (level, next_slot) in node.next.iter_mut().enumerate() {
            *next_slot = self.next_index(update[level], level);
        }

        let new_index = self.nodes.len();
        self.nodes.push(node);

        for (level, predecessor) in update.iter().copied().take(node_height).enumerate() {
            self.set_next(predecessor, level, Some(new_index));
        }

        self.len += 1;
        Ok(())
    }
}

impl<C: Comparator> MemTable for SkipListMemTable<C> {
    fn put(&mut self, key: Key, value: Value) -> Result<()> {
        self.insert_entry(key, EntryValue::Value(value))
    }

    fn delete(&mut self, key: Key) -> Result<()> {
        self.insert_entry(key, EntryValue::Tombstone)
    }

    fn get(&mut self, key: &[u8]) -> Option<&EntryValue> {
        let update = self.find_preprocessor(key);
        let index = self.find_existing_index(key, &update)?;
        Some(&self.nodes[index].value)
    }

    fn len(&self) -> usize {
        self.len
    }
}
