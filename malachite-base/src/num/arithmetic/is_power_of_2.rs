// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::IsPowerOf2;
use crate::num::conversion::traits::IntegerMantissaAndExponent;

macro_rules! impl_is_power_of_2_unsigned {
    ($t:ident) => {
        impl IsPowerOf2 for $t {
            /// This is a wrapper over the `is_power_of_two` functions in the standard library, for
            /// example [this one](u32::is_power_of_two).
            #[inline]
            fn is_power_of_2(&self) -> bool {
                $t::is_power_of_two(*self)
            }
        }
    };
}
apply_to_unsigneds!(impl_is_power_of_2_unsigned);

macro_rules! impl_is_power_of_2_primitive_float {
    ($t:ident) => {
        impl IsPowerOf2 for $t {
            /// Determines whether a number is an integer power of 2.
            ///
            /// $f(x) = (\exists n \in \Z : 2^n = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_power_of_2#is_power_of_2).
            #[inline]
            fn is_power_of_2(&self) -> bool {
                self.is_finite() && *self > 0.0 && self.integer_mantissa() == 1
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_power_of_2_primitive_float);
