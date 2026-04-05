use koshadb::{EntryValue, MemTable, SkipListMemTable};

#[test]
fn new_skiplist_memtable_starts_empty() {
    let memtable = SkipListMemTable::new();

    assert_eq!(memtable.len(), 0);
    assert!(memtable.is_empty());
}

#[test]
fn put_and_get_work() {
    let mut memtable = SkipListMemTable::new();

    memtable.put(b"name".to_vec(), b"kosha".to_vec()).unwrap();

    assert_eq!(
        memtable.get(b"name"),
        Some(&EntryValue::Value(b"kosha".to_vec()))
    );
}

#[test]
fn put_overwrite_existing_key() {
    let mut memtable = SkipListMemTable::new();

    memtable.put(b"name".to_vec(), b"first".to_vec()).unwrap();
    memtable.put(b"name".to_vec(), b"second".to_vec()).unwrap();

    assert_eq!(
        memtable.get(b"name"),
        Some(&EntryValue::Value(b"second".to_vec()))
    );
    assert_eq!(memtable.len(), 1);
}

#[test]
fn delete_writes_tombstone() {
    let mut memtable = SkipListMemTable::new();

    memtable.put(b"name".to_vec(), b"kosha".to_vec()).unwrap();
    memtable.delete(b"name".to_vec()).unwrap();

    assert_eq!(memtable.get(b"name"), Some(&EntryValue::Tombstone));
    assert_eq!(memtable.len(), 1);
}

#[test]
fn delete_missing_key_still_records_tombstone() {
    let mut memtable = SkipListMemTable::new();

    memtable.delete(b"ghost".to_vec()).unwrap();

    assert_eq!(memtable.get(b"ghost"), Some(&EntryValue::Tombstone));
    assert_eq!(memtable.len(), 1);
}

#[test]
fn empty_key_is_rejected_for_put() {
    let mut memtable = SkipListMemTable::new();

    let result = memtable.put(Vec::new(), b"value".to_vec());

    assert!(result.is_err());
}

#[test]
fn empty_key_is_rejected_for_delete() {
    let mut memtable = SkipListMemTable::new();

    let result = memtable.delete(Vec::new());

    assert!(result.is_err());
}

#[test]
fn lookup_works_after_inserting_keys_in_random_order() {
    let mut memtable = SkipListMemTable::new();

    memtable.put(b"k3".to_vec(), b"v3".to_vec()).unwrap();
    memtable.put(b"k1".to_vec(), b"v1".to_vec()).unwrap();
    memtable.put(b"k4".to_vec(), b"v4".to_vec()).unwrap();
    memtable.put(b"k2".to_vec(), b"v2".to_vec()).unwrap();

    assert_eq!(
        memtable.get(b"k1"),
        Some(&EntryValue::Value(b"v1".to_vec()))
    );
    assert_eq!(
        memtable.get(b"k2"),
        Some(&EntryValue::Value(b"v2".to_vec()))
    );
    assert_eq!(
        memtable.get(b"k3"),
        Some(&EntryValue::Value(b"v3".to_vec()))
    );
    assert_eq!(
        memtable.get(b"k4"),
        Some(&EntryValue::Value(b"v4".to_vec()))
    );
}
