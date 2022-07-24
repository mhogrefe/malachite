use std::iter::Cloned;
use std::slice::Iter;

/// An iterator that generates both [`bool`]s.
///
/// This `struct` is created by [`exhaustive_bools`]; see its documentation for more.
pub type ExhaustiveBools = Cloned<Iter<'static, bool>>;

/// Generates both [`bool`]s.
///
/// The output length is 2.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
///
/// assert_eq!(exhaustive_bools().collect_vec(), &[false, true]);
/// ```
#[inline]
pub fn exhaustive_bools() -> ExhaustiveBools {
    [false, true].iter().cloned()
}
