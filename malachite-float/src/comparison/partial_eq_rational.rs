use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use malachite_base::num::arithmetic::traits::CheckedLogBase2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use std::cmp::Ordering;

impl PartialEq<Rational> for Float {
    /// Determines whether a [`Float`] is equal to a [`Rational`].
    ///
    /// Infinity, negative infinity, and NaN are not equal to any [`Rational`]. Both the [`Float`]
    /// zero and the [`Float`] negative zero are equal to the [`Rational`] zero.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!(Float::from(123) == Rational::from(123));
    /// assert!(Float::from(-123) == Rational::from(-123));
    /// assert!(Float::ONE_HALF == Rational::ONE_HALF);
    /// assert!(Float::from(1.0f64 / 3.0) != Rational::from_unsigneds(1u8, 3));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                *other != 0
                    && *sign == (*other > 0)
                    && if let Some(log_d) = other.denominator_ref().checked_log_base_2() {
                        let n = other.numerator_ref();
                        *exponent == i64::exact_from(n.significant_bits()) - i64::exact_from(log_d)
                            && significand.cmp_normalized(n) == Ordering::Equal
                    } else {
                        false
                    }
            }
            _ => false,
        }
    }
}

impl PartialEq<Float> for Rational {
    /// Determines whether a [`Rational`] is equal to a [`Float`].
    ///
    /// No [`Rational`] is equal to infinity, negative infinity, or NaN. The [`Rational`] zero is
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
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(123) == Float::from(123));
    /// assert!(Rational::from(-123) == Float::from(-123));
    /// assert!(Rational::ONE_HALF == Float::ONE_HALF);
    /// assert!(Rational::from_unsigneds(1u8, 3) != Float::from(1.0f64 / 3.0));
    /// ```
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        other == self
    }
}
