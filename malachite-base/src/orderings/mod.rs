use std::cmp::Ordering;

pub(crate) const ORDERINGS: [Ordering; 3] = [Ordering::Equal, Ordering::Less, Ordering::Greater];

pub mod exhaustive;
pub mod random;
