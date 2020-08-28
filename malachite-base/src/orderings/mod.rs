use std::cmp::Ordering;

pub(crate) const ORDERINGS: [Ordering; 3] = [Ordering::Equal, Ordering::Less, Ordering::Greater];

/// This module contains iterators that generate `Ordering`s without repetition.
pub mod exhaustive;
/// This module contains iterators that generate `Ordering`s randomly.
pub mod random;
