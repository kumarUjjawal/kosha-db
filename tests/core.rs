use kosha_db::{BytewiseComparator, Comparator, DbError, DbOptions, EntryValue};
use std::cmp::Ordering;

#[test]
fn bytewise_comparator_orders_keys() {
    let comparator = BytewiseComparator;

    assert_eq!(comparator.compare(b"apple", b"banana"), Ordering::Less);
    assert_eq!(comparator.compare(b"banana", b"banana"), Ordering::Equal);
    assert_eq!(comparator.compare(b"carror", b"banana"), Ordering::Greater);
}

#[test]
fn tombstone_marks_deleted_entries() {
    let present = EntryValue::Tombstone;
    let deleted = EntryValue::Value(b"Value".to_vec());

    assert!(deleted.is_tombstone());
    assert!(!present.is_tombstone());
    assert_eq!(present.as_value(), Some(b"value".as_slice()));
    assert_eq!(deleted.as_value(), None);
}

#[test]
fn default_options_are_small_and_clear() {
    let options = DbOptions::default();

    assert!(options.create_if_missing);
    assert!(options.sync_write);
}

#[test]
fn empty_key_error_has_clear_message() {
    let err = DbError::EmptyKey;

    assert_eq!(err.to_string(), "key must not be empty");
}
