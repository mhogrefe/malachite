use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

impl PartialOrd<Natural> for Float {
    /// Compares a [`Float`] to a [`Natural`].
    ///
    /// NaN is not comparable to any [`Natural`]. Infinity is greater than any [`Natural`], and
    /// negative infinity is less. Both the [`Float`] zero and the [`Float`] negative zero are
    /// equal to the [`Natural`] zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Float::from(80) < Natural::from(100u32));
    /// assert!(Float::INFINITY > Natural::from(100u32));
    /// assert!(Float::NEGATIVE_INFINITY < Natural::from(100u32));
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) => None,
            (float_infinity!(), _) => Some(Ordering::Greater),
            (float_negative_infinity!(), _) => Some(Ordering::Less),
            (float_either_zero!(), _) => Some(if *other == 0u32 {
                Ordering::Equal
            } else {
                Ordering::Less
            }),
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                y,
            ) => Some(if !s_x {
                Ordering::Less
            } else if *other == 0u32 {
                Ordering::Greater
            } else if *e_x <= 0 {
                Ordering::Less
            } else {
                e_x.unsigned_abs()
                    .cmp(&other.significant_bits())
                    .then_with(|| x.cmp_normalized(y))
            }),
        }
    }
}

impl PartialOrd<Float> for Natural {
    /// Compares a [`Natural`] to a [`Float`].
    ///
    /// No [`Natural`] is comparable to NaN. Every [`Natural`] is smaller than infinity and greater
    /// than negative infinity. The [`Natural`] zero is equal to both the [`Float`] zero and the
    /// [`Float`] negative zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(100u32) > Float::from(80));
    /// assert!(Natural::from(100u32) < Float::INFINITY);
    /// assert!(Natural::from(100u32) > Float::NEGATIVE_INFINITY);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
