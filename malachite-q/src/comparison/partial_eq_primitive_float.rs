use malachite_base::num::arithmetic::traits::{FloorLogBase2, IsPowerOf2};
use Rational;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for Rational {
            /// Determines whether a `Rational` is equal to a value of primitive float type.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_eq_primitive_float` module.
            fn eq(&self, other: &$t) -> bool {
                if !other.is_finite() {
                    false
                } else if *other == 0.0 {
                    *self == 0u32
                } else {
                    *self != 0u32
                        && self.sign == (*other > 0.0)
                        && self.denominator.is_power_of_2()
                        && self.floor_log_base_2_of_abs() == other.abs().floor_log_base_2()
                        && *self == Rational::from(*other)
                }
            }
        }

        impl PartialEq<Rational> for $t {
            /// Determines whether a value of primitive float type is equal to a `Rational`.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_eq_primitive_float` module.
            #[inline]
            fn eq(&self, other: &Rational) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
