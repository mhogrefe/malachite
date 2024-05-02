// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{OverflowingPow, OverflowingPowAssign, Parity, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::OverflowingFrom;
use crate::num::logic::traits::BitIterable;

fn overflowing_pow_unsigned<T: PrimitiveUnsigned>(x: T, exp: u64) -> (T, bool) {
    if exp == 0 {
        (T::ONE, false)
    } else if x < T::TWO {
        (x, false)
    } else {
        let (mut power, mut overflow) = (x, false);
        for bit in exp.bits().rev().skip(1) {
            overflow |= power.overflowing_square_assign();
            if bit {
                overflow |= power.overflowing_mul_assign(x);
            }
        }
        (power, overflow)
    }
}

fn overflowing_unsigned_to_signed_neg<
    U: PrimitiveUnsigned,
    S: OverflowingFrom<U> + PrimitiveSigned,
>(
    x: U,
) -> (S, bool) {
    let (signed_x, overflow) = S::overflowing_from(x);
    if signed_x == S::MIN {
        (signed_x, false)
    } else {
        (signed_x.wrapping_neg(), overflow)
    }
}

fn overflowing_pow_signed<
    U: PrimitiveUnsigned,
    S: OverflowingFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    exp: u64,
) -> (S, bool) {
    let (p_abs, overflow) = OverflowingPow::overflowing_pow(x.unsigned_abs(), exp);
    let (p, overflow_2) = if x >= S::ZERO || exp.even() {
        S::overflowing_from(p_abs)
    } else {
        overflowing_unsigned_to_signed_neg(p_abs)
    };
    (p, overflow || overflow_2)
}

macro_rules! impl_overflowing_pow_unsigned {
    ($t:ident) => {
        impl OverflowingPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_pow` functions in the standard library, for
            /// example [this one](u32::overflowing_pow).
            #[inline]
            fn overflowing_pow(self, exp: u64) -> ($t, bool) {
                overflowing_pow_unsigned(self, exp)
            }
        }
    };
}
apply_to_unsigneds!(impl_overflowing_pow_unsigned);

macro_rules! impl_overflowing_pow_signed {
    ($t:ident) => {
        impl OverflowingPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_pow` functions in the standard library, for
            /// example [this one](i32::overflowing_pow).
            #[inline]
            fn overflowing_pow(self, exp: u64) -> ($t, bool) {
                overflowing_pow_signed(self, exp)
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_pow_signed);

macro_rules! impl_overflowing_pow_primitive_int {
    ($t:ident) => {
        impl OverflowingPowAssign<u64> for $t {
            /// Raises a number to a power, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::overflowing_pow#overflowing_pow_assign).
            #[inline]
            fn overflowing_pow_assign(&mut self, exp: u64) -> bool {
                let overflow;
                (*self, overflow) = OverflowingPow::overflowing_pow(*self, exp);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_pow_primitive_int);
