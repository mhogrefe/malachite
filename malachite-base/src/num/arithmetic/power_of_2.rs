// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::PowerOf2;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::IntegerMantissaAndExponent;

fn power_of_2_unsigned<T: PrimitiveUnsigned>(pow: u64) -> T {
    assert!(pow < T::WIDTH);
    T::ONE << pow
}

macro_rules! impl_power_of_2_unsigned {
    ($t:ident) => {
        impl PowerOf2<u64> for $t {
            /// Raises 2 to an integer power.
            ///
            /// $f(k) = 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result is not representable.
            ///
            /// # Examples
            /// See [here](super::power_of_2#power_of_2).
            #[inline]
            fn power_of_2(pow: u64) -> $t {
                power_of_2_unsigned(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_power_of_2_unsigned);

fn power_of_2_signed<T: PrimitiveSigned>(pow: u64) -> T {
    assert!(pow < T::WIDTH - 1);
    T::ONE << pow
}

macro_rules! impl_power_of_2_signed {
    ($t:ident) => {
        impl PowerOf2<u64> for $t {
            /// Raises 2 to an integer power.
            ///
            /// $f(k) = 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result is not representable.
            ///
            /// # Examples
            /// See [here](super::power_of_2#power_of_2).
            #[inline]
            fn power_of_2(pow: u64) -> $t {
                power_of_2_signed(pow)
            }
        }
    };
}
apply_to_signeds!(impl_power_of_2_signed);

macro_rules! impl_power_of_2_primitive_float {
    ($t:ident) => {
        impl PowerOf2<i64> for $t {
            /// Raises 2 to an integer power.
            ///
            /// $f(k) = 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the power is smaller than `Self::MIN_EXPONENT` or greater than
            /// `Self::MAX_EXPONENT`.
            ///
            /// # Examples
            /// See [here](super::power_of_2#power_of_2).
            #[inline]
            fn power_of_2(pow: i64) -> $t {
                $t::from_integer_mantissa_and_exponent(1, pow).unwrap()
            }
        }
    };
}
apply_to_primitive_floats!(impl_power_of_2_primitive_float);
