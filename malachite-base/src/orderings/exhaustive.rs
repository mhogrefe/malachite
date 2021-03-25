use orderings::ORDERINGS;
use std::cmp::Ordering;
use std::iter::Cloned;
use std::slice::Iter;

pub type ExhaustiveOrderings = Cloned<Iter<'static, Ordering>>;

/// Generates all `Ordering`s, in increasing order.
///
/// The output length is 3.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::orderings::exhaustive::orderings_increasing;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     orderings_increasing().collect_vec(),
///     &[Ordering::Less, Ordering::Equal, Ordering::Greater]
/// );
/// ```
#[inline]
pub fn orderings_increasing() -> ExhaustiveOrderings {
    [Ordering::Less, Ordering::Equal, Ordering::Greater]
        .iter()
        .cloned()
}

/// Generates all `Ordering`s, with `Equal` coming first.
///
/// The output length is 3.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::orderings::exhaustive::exhaustive_orderings;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     exhaustive_orderings().collect_vec(),
///     &[Ordering::Equal, Ordering::Less, Ordering::Greater]
/// );
/// ```
#[inline]
pub fn exhaustive_orderings() -> Cloned<Iter<'static, Ordering>> {
    ORDERINGS.iter().cloned()
}
