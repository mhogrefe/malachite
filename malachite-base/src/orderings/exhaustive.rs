use std::cmp::Ordering;
use std::iter::Cloned;
use std::slice::Iter;

use orderings::ORDERINGS;

/// Generates all `Ordering`s, in increasing order.
///
/// The output length is 3.
///
/// # Worst-case complexity
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::orderings::exhaustive::orderings_increasing;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     orderings_increasing().collect::<Vec<_>>(),
///     &[Ordering::Less, Ordering::Equal, Ordering::Greater]
/// );
/// ```
#[inline]
pub fn orderings_increasing() -> Cloned<Iter<'static, Ordering>> {
    [Ordering::Less, Ordering::Equal, Ordering::Greater]
        .iter()
        .cloned()
}

/// Generates all `Ordering`s, with `Equal` coming first.
///
/// The output length is 3.
///
/// # Worst-case complexity
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::orderings::exhaustive::exhaustive_orderings;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     exhaustive_orderings().collect::<Vec<_>>(),
///     &[Ordering::Equal, Ordering::Less, Ordering::Greater]
/// );
/// ```
#[inline]
pub fn exhaustive_orderings() -> Cloned<Iter<'static, Ordering>> {
    ORDERINGS.iter().cloned()
}
