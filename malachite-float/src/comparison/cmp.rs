use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering;

impl PartialOrd for Float {
    /// Compares two [`Float`]s.
    ///
    /// This implementation follows the IEEE 754 standard. `NaN` is not comparable to anything, not
    /// even itself. Positive zero is equal to negative zero. [`Float`]s with different precisions
    /// are equal if they represent the same numeric value.
    ///
    /// For different comparison behavior that provides a total order, consider using
    /// [`ComparableFloat`] or [`ComparableFloatRef`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero
    /// };
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Float::NAN.partial_cmp(&Float::NAN), None);
    /// assert_eq!(Float::ZERO.partial_cmp(&Float::NEGATIVE_ZERO), Some(Ordering::Equal));
    /// assert_eq!(Float::ONE.partial_cmp(&Float::one_prec(100)), Some(Ordering::Equal));
    /// assert!(Float::INFINITY > Float::ONE);
    /// assert!(Float::NEGATIVE_INFINITY < Float::ONE);
    /// assert!(Float::ONE_HALF < Float::ONE);
    /// assert!(Float::ONE_HALF > Float::NEGATIVE_ONE);
    /// ```
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) | (_, float_nan!()) => None,
            (float_infinity!(), float_infinity!())
            | (float_negative_infinity!(), float_negative_infinity!())
            | (float_either_zero!(), float_either_zero!()) => Some(Ordering::Equal),
            (float_infinity!(), _) | (_, float_negative_infinity!()) => Some(Ordering::Greater),
            (float_negative_infinity!(), _) | (_, float_infinity!()) => Some(Ordering::Less),
            (Float(Finite { sign, .. }), float_either_zero!()) => Some(if *sign {
                Ordering::Greater
            } else {
                Ordering::Less
            }),
            (float_either_zero!(), Float(Finite { sign, .. })) => Some(if *sign {
                Ordering::Less
            } else {
                Ordering::Greater
            }),
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                Float(Finite {
                    sign: s_y,
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => Some(s_x.cmp(s_y).then_with(|| {
                let abs_cmp = e_x.cmp(e_y).then_with(|| x.cmp_normalized_no_shift(y));
                if *s_x {
                    abs_cmp
                } else {
                    abs_cmp.reverse()
                }
            })),
        }
    }
}

impl<'a> Ord for ComparableFloatRef<'a> {
    /// Compares two [`ComparableFloatRef`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableFloatRef`]s are ordered, least to greatest:
    ///   - Negative infinity
    ///   - Negative nonzero finite floats
    ///   - Negative zero
    ///   - NaN
    ///   - Positive zero
    ///   - Positive nonzero finite floats
    ///   - Positive infinity
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`Float`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero
    /// };
    /// use malachite_float::{ComparableFloatRef, Float};
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::NAN).partial_cmp(&ComparableFloatRef(&Float::NAN)),
    ///     Some(Ordering::Equal)
    /// );
    /// assert!(ComparableFloatRef(&Float::ZERO) > ComparableFloatRef(&Float::NEGATIVE_ZERO));
    /// assert!(ComparableFloatRef(&Float::ONE) < ComparableFloatRef(&Float::one_prec(100)));
    /// assert!(ComparableFloatRef(&Float::INFINITY) > ComparableFloatRef(&Float::ONE));
    /// assert!(ComparableFloatRef(&Float::NEGATIVE_INFINITY) < ComparableFloatRef(&Float::ONE));
    /// assert!(ComparableFloatRef(&Float::ONE_HALF) < ComparableFloatRef(&Float::ONE));
    /// assert!(ComparableFloatRef(&Float::ONE_HALF) > ComparableFloatRef(&Float::NEGATIVE_ONE));
    /// ```
    fn cmp(&self, other: &ComparableFloatRef<'a>) -> Ordering {
        match (&self.0, &other.0) {
            (float_nan!(), float_nan!())
            | (float_infinity!(), float_infinity!())
            | (float_negative_infinity!(), float_negative_infinity!()) => Ordering::Equal,
            (Float(Zero { sign: s_x }), Float(Zero { sign: s_y })) => s_x.cmp(s_y),
            (float_infinity!(), _) | (_, float_negative_infinity!()) => Ordering::Greater,
            (float_negative_infinity!(), _) | (_, float_infinity!()) => Ordering::Less,
            (float_nan!(), Float(Finite { sign, .. }))
            | (float_nan!(), Float(Zero { sign }))
            | (float_either_zero!(), Float(Finite { sign, .. })) => {
                if *sign {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Float(Finite { sign, .. }), float_nan!())
            | (Float(Zero { sign }), float_nan!())
            | (Float(Finite { sign, .. }), float_either_zero!()) => {
                if *sign {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    precision: p_x,
                    significand: x,
                }),
                Float(Finite {
                    sign: s_y,
                    exponent: e_y,
                    precision: p_y,
                    significand: y,
                }),
            ) => s_x.cmp(s_y).then_with(|| {
                let abs_cmp = e_x
                    .cmp(e_y)
                    .then_with(|| x.cmp_normalized_no_shift(y))
                    .then_with(|| p_x.cmp(p_y));
                if *s_x {
                    abs_cmp
                } else {
                    abs_cmp.reverse()
                }
            }),
        }
    }
}

impl<'a> PartialOrd for ComparableFloatRef<'a> {
    /// Compares two [`ComparableFloatRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &ComparableFloatRef) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableFloat {
    /// Compares two [`ComparableFloat`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableFloat`]s are ordered, least to greatest:
    ///   - Negative infinity
    ///   - Negative nonzero finite floats
    ///   - Negative zero
    ///   - NaN
    ///   - Positive zero
    ///   - Positive nonzero finite floats
    ///   - Positive infinity
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`Float`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///         ComparableFloat(Float::NAN).partial_cmp(&ComparableFloat(Float::NAN)),
    ///         Some(Ordering::Equal)
    /// );
    /// assert!(ComparableFloat(Float::ZERO) > ComparableFloat(Float::NEGATIVE_ZERO));
    /// assert!(ComparableFloat(Float::ONE) < ComparableFloat(Float::one_prec(100)));
    /// assert!(ComparableFloat(Float::INFINITY) > ComparableFloat(Float::ONE));
    /// assert!(ComparableFloat(Float::NEGATIVE_INFINITY) < ComparableFloat(Float::ONE));
    /// assert!(ComparableFloat(Float::ONE_HALF) < ComparableFloat(Float::ONE));
    /// assert!(ComparableFloat(Float::ONE_HALF) > ComparableFloat(Float::NEGATIVE_ONE));
    /// ```
    #[inline]
    fn cmp(&self, other: &ComparableFloat) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl PartialOrd for ComparableFloat {
    /// Compares two [`ComparableFloat`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &ComparableFloat) -> Option<Ordering> {
        Some(self.as_ref().cmp(&other.as_ref()))
    }
}
