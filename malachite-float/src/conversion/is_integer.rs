use crate::Float;
use malachite_base::num::conversion::traits::{ConvertibleFrom, IsInteger};
use malachite_nz::integer::Integer;

impl<'a> IsInteger for &'a Float {
    /// Determines whether a [`Float`] is an integer.
    ///
    /// $f(x) = x \in \Z$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::ZERO.is_integer(), true);
    /// assert_eq!(Float::ONE.is_integer(), true);
    /// assert_eq!(Float::from(100).is_integer(), true);
    /// assert_eq!(Float::from(-100).is_integer(), true);
    /// assert_eq!(Float::ONE_HALF.is_integer(), false);
    /// assert_eq!((-Float::ONE_HALF).is_integer(), false);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        Integer::convertible_from(self)
    }
}
