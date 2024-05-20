// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign, PowerOf2, Sign};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::conversion::traits::SciMantissaAndExponent;
use core::cmp::Ordering::*;

macro_rules! impl_next_power_of_2_unsigned {
    ($t:ident) => {
        impl NextPowerOf2 for $t {
            type Output = $t;

            /// This is a wrapper over the `next_power_of_two` functions in the standard library,
            /// for example [this one](u32::next_power_of_two).
            #[inline]
            fn next_power_of_2(self) -> $t {
                $t::next_power_of_two(self)
            }
        }

        impl NextPowerOf2Assign for $t {
            /// Replaces a number with the smallest power of 2 greater than or equal to it.
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
            /// See [here](super::next_power_of_2#next_power_of_2_assign).
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

            /// Finds the smallest power of 2 greater than or equal to a number.
            ///
            /// $x \gets 2^{\lceil \log_2 x \rceil}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` has a negative sign (positive zero is allowed, but negative zero is
            /// not), or if the next power of 2 is greater than the type's maximum value.
            ///
            /// # Examples
            /// See [here](super::next_power_of_2#next_power_of_2).
            #[inline]
            fn next_power_of_2(self) -> $t {
                assert_eq!(self.sign(), Greater);
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
            /// Replaces a number with the smallest power of 2 greater than or equal to it.
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
            /// See [here](super::next_power_of_2#next_power_of_2_assign).
            #[inline]
            fn next_power_of_2_assign(&mut self) {
                *self = $t::next_power_of_2(*self);
            }
        }
    };
}
apply_to_primitive_floats!(impl_next_power_of_2_primitive_float);
