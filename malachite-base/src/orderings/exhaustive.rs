use std::cmp::Ordering;
use std::iter::Cloned;
use std::slice::Iter;

/// Generates all `Ordering`s, in increasing order.
///
/// Length is 3.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::orderings::exhaustive::exhaustive_orderings_increasing;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     exhaustive_orderings_increasing().collect::<Vec<Ordering>>(),
///     &[Ordering::Less, Ordering::Equal, Ordering::Greater]
/// );
/// ```
#[inline]
pub fn exhaustive_orderings_increasing() -> Cloned<Iter<'static, Ordering>> {
    [Ordering::Less, Ordering::Equal, Ordering::Greater]
        .iter()
        .cloned()
}

/// Generates all `Ordering`s. `Equal` comes first.
///
/// Length is 3.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::orderings::exhaustive::exhaustive_orderings;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     exhaustive_orderings().collect::<Vec<Ordering>>(),
///     &[Ordering::Equal, Ordering::Less, Ordering::Greater]
/// );
/// ```
#[inline]
pub fn exhaustive_orderings() -> Cloned<Iter<'static, Ordering>> {
    [Ordering::Equal, Ordering::Less, Ordering::Greater]
        .iter()
        .cloned()
}
