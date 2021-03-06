use std::iter::Cloned;
use std::slice::Iter;

/// Generates both `bool`s.
///
/// The output length is 2.
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
/// use malachite_base::bools::exhaustive::exhaustive_bools;
///
/// assert_eq!(exhaustive_bools().collect_vec(), &[false, true]);
/// ```
#[inline]
pub fn exhaustive_bools() -> Cloned<Iter<'static, bool>> {
    [false, true].iter().cloned()
}
