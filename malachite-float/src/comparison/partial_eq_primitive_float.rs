use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

fn float_partial_eq_primitive_float<T: PrimitiveFloat>(x: &Float, y: &T) -> bool {
    match x {
        float_nan!() => false,
        float_infinity!() => *y == T::INFINITY,
        float_negative_infinity!() => *y == T::NEGATIVE_INFINITY,
        float_either_zero!() => *y == T::ZERO,
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            y.is_finite()
                && *y != T::ZERO
                && *sign == (*y > T::ZERO)
                && *exponent - 1 == y.sci_exponent()
                && significand.cmp_normalized(&Natural::from(y.integer_mantissa()))
                    == Ordering::Equal
        }
    }
}

macro_rules! impl_partial_eq_primitive_float {
    ($t: ident) => {
        impl PartialEq<$t> for Float {
            /// Determines whether a [`Float`] is equal to a primitive float.
            ///
            /// The [`Float`] infinity is equal to the primitive float infinity, and the [`Float`]
            /// negative infinity is equal to the primitive float negative infinity. The [`Float`]
            /// NaN is not equal to anything, not even the primitive float NaN. Every [`Float`]
            /// zero is equal to every primitive float zero, regardless of sign.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.sci_exponent().abs())`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                float_partial_eq_primitive_float(self, other)
            }
        }

        impl PartialEq<Float> for $t {
            /// Determines whether a primitive float is equal to a [`Float`].
            ///
            /// The primitive float infinity is equal to the [`Float`] infinity, and the primitive
            /// float negative infinity is equal to the [`Float`] negative infinity. The primitive
            /// float NaN is not equal to anything, not even the [`Float`] NaN. Every primitive
            /// float zero is equal to every [`Float`] zero, regardless of sign.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.sci_exponent().abs())`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &Float) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_partial_eq_primitive_float);
