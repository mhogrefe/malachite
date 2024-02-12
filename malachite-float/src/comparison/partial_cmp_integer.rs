use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;

impl PartialOrd<Integer> for Float {
    /// Compares a [`Float`] to an [`Integer`].
    ///
    /// NaN is not comparable to any [`Integer`]. Infinity is greater than any [`Integer`], and
    /// negative infinity is less. Both the [`Float`] zero and the [`Float`] negative zero are equal
    /// to the [`Integer`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Float::from(80) < Integer::from(100));
    /// assert!(Float::from(-80) > Integer::from(-100));
    /// assert!(Float::INFINITY > Integer::from(100));
    /// assert!(Float::NEGATIVE_INFINITY < Integer::from(-100));
    /// ```
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) => None,
            (float_infinity!(), _) => Some(Ordering::Greater),
            (float_negative_infinity!(), _) => Some(Ordering::Less),
            (float_either_zero!(), y) => 0u32.partial_cmp(y),
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                y,
            ) => {
                let s_y = *other > 0;
                let s_cmp = s_x.cmp(&s_y);
                if s_cmp != Ordering::Equal {
                    return Some(s_cmp);
                }
                let abs_cmp = if *other == 0u32 {
                    Ordering::Greater
                } else if *e_x <= 0 {
                    Ordering::Less
                } else {
                    e_x.unsigned_abs()
                        .cmp(&other.significant_bits())
                        .then_with(|| x.cmp_normalized(y.unsigned_abs_ref()))
                };
                Some(if s_y { abs_cmp } else { abs_cmp.reverse() })
            }
        }
    }
}

impl PartialOrd<Float> for Integer {
    /// Compares an [`Integer`] to a [`Float`].
    ///
    /// No [`Integer`] is comparable to NaN. Every [`Integer`] is smaller than infinity and greater
    /// than negative infinity. The [`Integer`] zero is equal to both the [`Float`] zero and the
    /// [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(100) > Float::from(80));
    /// assert!(Integer::from(-100) < Float::from(-80));
    /// assert!(Integer::from(100) < Float::INFINITY);
    /// assert!(Integer::from(-100) > Float::NEGATIVE_INFINITY);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
