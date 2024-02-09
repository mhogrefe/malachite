use core::cmp::Ordering;

pub(crate) const ORDERINGS: [Ordering; 3] = [Ordering::Equal, Ordering::Less, Ordering::Greater];

/// Converts a string to an [`Ordering`].
///
/// If the string` does not represent a valid [`Ordering`], `None` is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::orderings::ordering_from_str;
/// use std::cmp::Ordering;
///
/// assert_eq!(ordering_from_str("Equal"), Some(Ordering::Equal));
/// assert_eq!(ordering_from_str("Less"), Some(Ordering::Less));
/// assert_eq!(ordering_from_str("Greater"), Some(Ordering::Greater));
/// assert_eq!(ordering_from_str("abc"), None);
/// ```
#[inline]
pub fn ordering_from_str(src: &str) -> Option<Ordering> {
    match src {
        "Equal" => Some(Ordering::Equal),
        "Less" => Some(Ordering::Less),
        "Greater" => Some(Ordering::Greater),
        _ => None,
    }
}

/// Iterators that generate [`Ordering`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Ordering`]s randomly.
pub mod random;
