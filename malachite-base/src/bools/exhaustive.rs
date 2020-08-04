use std::iter::Cloned;
use std::slice::Iter;

/// Generates both `bool`s.
///
/// The output length is 2.
///
/// # Worst-case complexity
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::bools::exhaustive::exhaustive_bools;
///
/// assert_eq!(exhaustive_bools().collect::<Vec<_>>(), &[false, true]);
/// ```
#[inline]
pub fn exhaustive_bools() -> Cloned<Iter<'static, bool>> {
    [false, true].iter().cloned()
}
