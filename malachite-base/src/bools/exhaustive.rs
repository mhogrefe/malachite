use std::iter::Cloned;
use std::slice::Iter;

/// Generates both `bool`s.
///
/// Length is 2.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::bools::exhaustive::exhaustive_bools;
///
/// assert_eq!(exhaustive_bools().collect::<Vec<bool>>(), &[false, true]);
/// ```
#[inline]
pub fn exhaustive_bools() -> Cloned<Iter<'static, bool>> {
    [false, true].iter().cloned()
}
