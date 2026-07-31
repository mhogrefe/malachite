// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{BalancedMod, BalancedModAssign};
use crate::num::conversion::traits::WrappingFrom;

macro_rules! impl_balanced_mod_unsigned {
    ($u:ident, $s:ident) => {
        impl BalancedMod<$u> for $u {
            type Output = $s;

            /// Divides a number by another number, returning the balanced remainder: the
            /// representative of `self` modulo `other` that is closest to zero.
            ///
            /// The remainder $r$ satisfies $-y/2 < r \leq y/2$ and $r \equiv x \bmod y$, which
            /// determine it uniquely. A remainder of exactly $y/2$ is positive, so the result may
            /// be negative and is returned as the signed type of the same width. It always fits:
            /// the magnitude never exceeds $y/2$, which is at most half the unsigned maximum.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero.
            ///
            /// # Examples
            /// See [here](super::balanced_mod#balanced_mod).
            #[inline]
            fn balanced_mod(self, other: $u) -> $s {
                let r = self % other;
                if r <= other >> 1 {
                    $s::wrapping_from(r)
                } else {
                    // `r - other` is negative and small enough to fit, but the subtraction has to
                    // happen before the conversion, where it would not
                    $s::wrapping_from(r.wrapping_sub(other))
                }
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_balanced_mod_unsigned);

macro_rules! impl_balanced_mod_signed {
    ($t:ident) => {
        impl BalancedMod<$t> for $t {
            type Output = $t;

            /// Divides a number by another number, returning the balanced remainder: the
            /// representative of `self` modulo `other` that is closest to zero.
            ///
            /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$ and $r \equiv x \bmod y$, which
            /// determine it uniquely. A remainder of exactly $|y|/2$ is positive. Only the
            /// magnitude of `other` matters, so negating it leaves the result unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero.
            ///
            /// # Examples
            /// See [here](super::balanced_mod#balanced_mod).
            #[inline]
            fn balanced_mod(self, other: $t) -> $t {
                // Working in the unsigned domain keeps the most negative divisor, whose magnitude
                // is not representable, from overflowing.
                let abs_other = other.unsigned_abs();
                let r = self.unsigned_abs() % abs_other;
                let r = if self < 0 && r != 0 { abs_other - r } else { r };
                if r <= abs_other >> 1 {
                    $t::wrapping_from(r)
                } else {
                    $t::wrapping_from(r.wrapping_sub(abs_other))
                }
            }
        }

        impl BalancedModAssign<$t> for $t {
            /// Divides a number by another number, replacing the first number by the balanced
            /// remainder: the representative of `self` modulo `other` that is closest to zero.
            ///
            /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$; a remainder of exactly $|y|/2$
            /// is positive.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero.
            ///
            /// # Examples
            /// See [here](super::balanced_mod#balanced_mod_assign).
            #[inline]
            fn balanced_mod_assign(&mut self, other: $t) {
                *self = self.balanced_mod(other);
            }
        }
    };
}
apply_to_signeds!(impl_balanced_mod_signed);
