// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{BalancedModPowerOf2, BalancedModPowerOf2Assign};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};

// The balanced remainder of `x` modulo $2^k$, for $k$ at most the width, as a signed value of the
// same width. Only the low $k$ bits of `x` matter. The result fits except when $k$ is the width and
// those bits are exactly $2^{k-1}$, whose balanced remainder is itself.
fn balanced_mod_power_of_2_low<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: ExactFrom<U> + PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    pow: u64,
) -> S {
    if pow == 0 {
        return S::ZERO;
    }
    let r = x.mod_power_of_2(pow);
    if r <= U::power_of_2(pow - 1) {
        S::exact_from(r)
    } else if pow == U::WIDTH {
        // `r - 2^k` is `r` itself, reinterpreted as negative.
        S::wrapping_from(r)
    } else {
        // `r - 2^k` is negative and fits, but the subtraction has to happen before the conversion.
        S::wrapping_from(r.wrapping_sub(U::power_of_2(pow)))
    }
}

fn balanced_mod_power_of_2_unsigned<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: ExactFrom<U> + PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    pow: u64,
) -> S {
    if pow > U::WIDTH {
        // Every value of the type lies below $2^{k-1}$, so it is its own balanced remainder.
        S::exact_from(x)
    } else {
        balanced_mod_power_of_2_low(x, pow)
    }
}

fn balanced_mod_power_of_2_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: ExactFrom<U> + PrimitiveSigned + WrappingFrom<U>,
>(
    x: S,
    pow: u64,
) -> S {
    if pow > S::WIDTH {
        // Every value of the type lies in $(-2^{k-1}, 2^{k-1}]$, so it is its own balanced
        // remainder.
        x
    } else {
        // Reducing modulo $2^k$ only looks at the low $k$ bits, which the two's-complement
        // reinterpretation keeps.
        balanced_mod_power_of_2_low(U::wrapping_from(x), pow)
    }
}

macro_rules! impl_balanced_mod_power_of_2_unsigned {
    ($u:ident, $s:ident) => {
        impl BalancedModPowerOf2 for $u {
            type Output = $s;

            /// Divides a number by $2^k$, returning the balanced remainder: the representative of
            /// `self` modulo $2^k$ that is closest to zero.
            ///
            /// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$ and $r \equiv x \bmod 2^k$,
            /// which determine it uniquely; for $k = 0$ it is 0. A remainder of exactly $2^{k-1}$
            /// is positive, so the result may be negative and is returned as the signed type of the
            /// same width. This is the same as [`balanced_mod`](super::traits::BalancedMod) with
            /// modulus $2^k$, when $2^k$ fits in the type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result does not fit in the signed type: if `pow` is `Self::WIDTH` and
            /// `self` is $2^{W-1}$, or if `pow` is greater than `Self::WIDTH` and `self` is at
            /// least $2^{W-1}$, where $W$ is `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::balanced_mod_power_of_2#balanced_mod_power_of_2).
            #[inline]
            fn balanced_mod_power_of_2(self, pow: u64) -> $s {
                balanced_mod_power_of_2_unsigned(self, pow)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_balanced_mod_power_of_2_unsigned);

macro_rules! impl_balanced_mod_power_of_2_signed {
    ($u:ident, $s:ident) => {
        impl BalancedModPowerOf2 for $s {
            type Output = $s;

            /// Divides a number by $2^k$, returning the balanced remainder: the representative of
            /// `self` modulo $2^k$ that is closest to zero.
            ///
            /// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$ and $r \equiv x \bmod 2^k$,
            /// which determine it uniquely; for $k = 0$ it is 0. A remainder of exactly $2^{k-1}$
            /// is positive. When `pow` is greater than `Self::WIDTH`, every value is its own
            /// balanced remainder.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is `Self::WIDTH` and `self` is `Self::MIN`, whose balanced remainder
            /// $2^{W-1}$ does not fit in the type.
            ///
            /// # Examples
            /// See [here](super::balanced_mod_power_of_2#balanced_mod_power_of_2).
            #[inline]
            fn balanced_mod_power_of_2(self, pow: u64) -> $s {
                balanced_mod_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl BalancedModPowerOf2Assign for $s {
            /// Divides a number by $2^k$, replacing it by the balanced remainder: the
            /// representative of `self` modulo $2^k$ that is closest to zero.
            ///
            /// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$; a remainder of exactly
            /// $2^{k-1}$ is positive.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is `Self::WIDTH` and `self` is `Self::MIN`.
            ///
            /// # Examples
            /// See [here](super::balanced_mod_power_of_2#balanced_mod_power_of_2_assign).
            #[inline]
            fn balanced_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.balanced_mod_power_of_2(pow);
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_balanced_mod_power_of_2_signed);
