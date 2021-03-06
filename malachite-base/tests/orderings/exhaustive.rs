use itertools::Itertools;
use malachite_base::orderings::exhaustive::{exhaustive_orderings, orderings_increasing};
use std::cmp::Ordering;

#[test]
fn test_orderings_increasing() {
    assert_eq!(
        orderings_increasing().collect_vec(),
        &[Ordering::Less, Ordering::Equal, Ordering::Greater]
    );
}

#[test]
fn test_exhaustive_orderings() {
    assert_eq!(
        exhaustive_orderings().collect_vec(),
        &[Ordering::Equal, Ordering::Less, Ordering::Greater]
    );
}
