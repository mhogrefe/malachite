// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use core::ops::{Shr, ShrAssign};
use malachite_nz::integer::Integer;

macro_rules! impl_shr {
    ($t:ident) => {
        impl Shr<$t> for BigExponentFloat {
            type Output = BigExponentFloat;

            /// Right-shifts a [`BigExponentFloat`] (divides it by a power of 2), taking it by
            /// value.
            ///
            /// NaN, infinities, and zero are unchanged. The result is exact. A negative shift
            /// amount shifts in the opposite direction.
            ///
            /// $$
            /// f(x, k) = x/2^k.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> BigExponentFloat {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &BigExponentFloat {
            type Output = BigExponentFloat;

            /// Right-shifts a [`BigExponentFloat`] (divides it by a power of 2), taking it by
            /// reference.
            ///
            /// NaN, infinities, and zero are unchanged. The result is exact. A negative shift
            /// amount shifts in the opposite direction.
            ///
            /// $$
            /// f(x, k) = x/2^k.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> BigExponentFloat {
                self.clone() >> bits
            }
        }

        impl ShrAssign<$t> for BigExponentFloat {
            /// Right-shifts a [`BigExponentFloat`] (divides it by a power of 2), in place.
            ///
            /// NaN, infinities, and zero are unchanged. The result is exact. A negative shift
            /// amount shifts in the opposite direction.
            ///
            /// $$
            /// x \gets x/2^k.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                if let BigExponentFloat(Finite { exponent, .. }) = self {
                    *exponent -= Integer::from(bits);
                }
            }
        }
    };
}
apply_to_primitive_ints!(impl_shr);
