use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use std::cmp::Ordering;

impl PartialEq<Integer> for Float {
    /// Determines whether a [`Float`] is equal to an [`Integer`].
    ///
    /// Infinity, negative infinity, and NaN are not equal to any [`Integer`]. Both the [`Float`]
    /// zero and the [`Float`] negative zero are equal to the [`Integer`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Float::from(123) == Integer::from(123));
    /// assert!(Float::from(-123) == Integer::from(-123));
    /// assert!(Float::ONE_HALF != Integer::ONE);
    /// ```
    fn eq(&self, other: &Integer) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                *other != 0u32
                    && *sign == (*other >= 0u32)
                    && *exponent >= 0
                    && other.significant_bits() == exponent.unsigned_abs()
                    && significand.cmp_normalized(other.unsigned_abs_ref()) == Ordering::Equal
            }
            _ => false,
        }
    }
}

impl PartialEq<Float> for Integer {
    /// Determines whether an [`Integer`] is equal to a [`Float`].
    ///
    /// No [`Integer`] is equal to infinity, negative infinity, or NaN. The [`Integer`] zero is
    /// equal to both the [`Float`] zero and the [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(123) == Float::from(123));
    /// assert!(Integer::from(-123) == Float::from(-123));
    /// assert!(Integer::ONE != Float::ONE_HALF);
    /// ```
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        other == self
    }
}
