// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Parity, SaturatingPow, SaturatingPowAssign};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn saturating_pow_unsigned<T: PrimitiveUnsigned>(x: T, exp: u64) -> T {
    if exp == 0 {
        T::ONE
    } else if x < T::TWO {
        x
    } else if let Some(p) = x.checked_pow(exp) {
        p
    } else {
        T::MAX
    }
}

fn saturating_pow_signed<T: PrimitiveSigned>(x: T, exp: u64) -> T {
    if exp == 0 {
        T::ONE
    } else if x == T::ZERO || x == T::ONE {
        x
    } else if x == T::NEGATIVE_ONE {
        if exp.even() {
            T::ONE
        } else {
            T::NEGATIVE_ONE
        }
    } else if let Some(p) = x.checked_pow(exp) {
        p
    } else if x > T::ZERO || exp.even() {
        T::MAX
    } else {
        T::MIN
    }
}

macro_rules! impl_saturating_pow_unsigned {
    ($t:ident) => {
        impl SaturatingPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_pow` functions in the standard library, for
            /// example [this one](u32::saturating_pow).
            #[inline]
            fn saturating_pow(self, exp: u64) -> $t {
                saturating_pow_unsigned(self, exp)
            }
        }
    };
}
apply_to_unsigneds!(impl_saturating_pow_unsigned);

macro_rules! impl_saturating_pow_signed {
    ($t:ident) => {
        impl SaturatingPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_pow` functions in the standard library, for
            /// example [this one](i32::saturating_pow).
            #[inline]
            fn saturating_pow(self, exp: u64) -> $t {
                saturating_pow_signed(self, exp)
            }
        }
    };
}
apply_to_signeds!(impl_saturating_pow_signed);

macro_rules! impl_saturating_pow_primitive_int {
    ($t:ident) => {
        impl SaturatingPowAssign<u64> for $t {
            /// Raises a number to a power, in place, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x^y & \text{if} \\quad m \leq x^y \leq M, \\\\
            ///     M & \text{if} \\quad x^y > M, \\\\
            ///     m & \text{if} \\quad x^y < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_pow#saturating_pow_assign).
            #[inline]
            fn saturating_pow_assign(&mut self, exp: u64) {
                *self = SaturatingPow::saturating_pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_pow_primitive_int);
