// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use core::ops::{Shl, ShlAssign};
use malachite_nz::integer::Integer;

macro_rules! impl_shl {
    ($t:ident) => {
        impl Shl<$t> for BigExponentFloat {
            type Output = BigExponentFloat;

            /// Left-shifts a [`BigExponentFloat`] (multiplies it by a power of 2), taking it by
            /// value.
            ///
            /// NaN, infinities, and zero are unchanged. The result is exact. A negative shift
            /// amount shifts in the opposite direction.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the number of bits of
            /// `self`'s exponent.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> BigExponentFloat {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &BigExponentFloat {
            type Output = BigExponentFloat;

            /// Left-shifts a [`BigExponentFloat`] (multiplies it by a power of 2), taking it by
            /// reference.
            ///
            /// NaN, infinities, and zero are unchanged. The result is exact. A negative shift
            /// amount shifts in the opposite direction.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> BigExponentFloat {
                self.clone() << bits
            }
        }

        impl ShlAssign<$t> for BigExponentFloat {
            /// Left-shifts a [`BigExponentFloat`] (multiplies it by a power of 2), in place.
            ///
            /// NaN, infinities, and zero are unchanged. The result is exact. A negative shift
            /// amount shifts in the opposite direction.
            ///
            /// $$
            /// x \gets x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the number of bits of
            /// `self`'s exponent.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                if let BigExponentFloat(Finite { exponent, .. }) = self {
                    *exponent += Integer::from(bits);
                }
            }
        }
    };
}
apply_to_primitive_ints!(impl_shl);
