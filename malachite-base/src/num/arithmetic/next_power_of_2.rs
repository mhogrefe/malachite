use num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign, PowerOf2, Sign};
use num::basic::floats::PrimitiveFloat;
use num::conversion::traits::SciMantissaAndExponent;
use std::cmp::Ordering;

macro_rules! impl_next_power_of_2_unsigned {
    ($t:ident) => {
        impl NextPowerOf2 for $t {
            type Output = $t;

            #[inline]
            fn next_power_of_2(self) -> $t {
                $t::next_power_of_two(self)
            }
        }

        impl NextPowerOf2Assign for $t {
            /// Replaces `self` with the smallest power of 2 greater than or equal to `self`.
            ///
            /// $x \gets 2^{\lceil \log_2 x \rceil}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the next power of 2 is greater than the type's maximum value.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::next_power_of_2` module.
            #[inline]
            fn next_power_of_2_assign(&mut self) {
                *self = $t::next_power_of_2(*self);
            }
        }
    };
}
apply_to_unsigneds!(impl_next_power_of_2_unsigned);

macro_rules! impl_next_power_of_2_primitive_float {
    ($t:ident) => {
        impl NextPowerOf2 for $t {
            type Output = $t;

            #[inline]
            fn next_power_of_2(self) -> $t {
                assert_eq!(self.sign(), Ordering::Greater);
                assert!(self.is_finite());
                if self == 0.0 {
                    return $t::MIN_POSITIVE_SUBNORMAL;
                }
                let (mantissa, exponent) = self.sci_mantissa_and_exponent();
                if mantissa == 1.0 {
                    self
                } else if exponent == $t::MAX_EXPONENT {
                    panic!("Next power of 2 is too large to represent");
                } else {
                    $t::power_of_2(exponent + 1)
                }
            }
        }

        impl NextPowerOf2Assign for $t {
            /// Replaces `self` with the smallest power of 2 greater than or equal to `self`.
            ///
            /// $x \gets 2^{\lceil \log_2 x \rceil}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the next power of 2 is greater than the type's maximum value.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::next_power_of_2` module.
            #[inline]
            fn next_power_of_2_assign(&mut self) {
                *self = $t::next_power_of_2(*self);
            }
        }
    };
}
apply_to_primitive_floats!(impl_next_power_of_2_primitive_float);
