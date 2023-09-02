use crate::Rational;
use malachite_base::num::arithmetic::traits::{FloorLogBase2, IsPowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for Rational {
            /// Determines whether a [`Rational`] is equal to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is
            /// `max(self.significant_bits(), other.sci_exponent().abs())`, and $m$ is
            /// `other.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[allow(clippy::cmp_owned)]
            fn eq(&self, other: &$t) -> bool {
                if !other.is_finite() {
                    false
                } else if *other == 0.0 {
                    *self == 0u32
                } else {
                    *self != 0u32
                        && self.sign == (*other > 0.0)
                        && self.denominator.is_power_of_2()
                        && self.floor_log_base_2_abs() == other.abs().floor_log_base_2()
                        && *self == Rational::exact_from(*other)
                }
            }
        }

        impl PartialEq<Rational> for $t {
            /// Determines whether a primitive float is equal to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is
            /// `max(self.sci_exponent().abs(), other.significant_bits())`, and $m$ is
            /// `self.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &Rational) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
