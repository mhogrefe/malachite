// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Parity, UnsignedAbs, WrappingPow, WrappingPowAssign};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::logic::traits::BitIterable;

fn wrapping_pow_unsigned<T: PrimitiveUnsigned>(x: T, exp: u64) -> T {
    if exp == 0 {
        T::ONE
    } else if x < T::TWO {
        x
    } else {
        let mut power = x;
        for bit in exp.bits().rev().skip(1) {
            power.wrapping_square_assign();
            if bit {
                power.wrapping_mul_assign(x);
            }
        }
        power
    }
}

fn wrapping_pow_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    exp: u64,
) -> S {
    let p_abs = x.unsigned_abs().wrapping_pow(exp);
    if x >= S::ZERO || exp.even() {
        S::wrapping_from(p_abs)
    } else {
        S::wrapping_from(p_abs).wrapping_neg()
    }
}

macro_rules! impl_wrapping_pow_unsigned {
    ($t:ident) => {
        impl WrappingPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_pow` functions in the standard library, for
            /// example [this one](u32::wrapping_pow).
            #[inline]
            fn wrapping_pow(self, exp: u64) -> $t {
                wrapping_pow_unsigned(self, exp)
            }
        }
    };
}
apply_to_unsigneds!(impl_wrapping_pow_unsigned);

macro_rules! impl_wrapping_pow_signed {
    ($t:ident) => {
        impl WrappingPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_pow` functions in the standard library, for
            /// example [this one](i32::wrapping_pow).
            #[inline]
            fn wrapping_pow(self, exp: u64) -> $t {
                wrapping_pow_signed(self, exp)
            }
        }
    };
}
apply_to_signeds!(impl_wrapping_pow_signed);

macro_rules! impl_wrapping_pow_primitive_int {
    ($t:ident) => {
        impl WrappingPowAssign<u64> for $t {
            /// Raises a number to a power, in place, wrapping around at the boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv x^n \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_pow#wrapping_pow_assign).
            #[inline]
            fn wrapping_pow_assign(&mut self, exp: u64) {
                *self = WrappingPow::wrapping_pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_pow_primitive_int);
