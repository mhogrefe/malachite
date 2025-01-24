// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2016 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModInverse;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::rounding_modes::RoundingMode::*;

// This is a variation of `n_xgcd` from `ulong_extras/xgcd.c`, FLINT 2.7.1.
pub_test! {mod_inverse_binary<
    U: WrappingFrom<S> + PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    m: U,
) -> Option<U> {
    assert_ne!(x, U::ZERO);
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    let mut u1 = S::ONE;
    let mut v2 = S::ONE;
    let mut u2 = S::ZERO;
    let mut v1 = S::ZERO;
    let mut u3 = m;
    let mut v3 = x;
    let mut d;
    let mut t2;
    let mut t1;
    if (m & x).get_highest_bit() {
        d = u3 - v3;
        t2 = v2;
        t1 = u2;
        u2 = u1 - u2;
        u1 = t1;
        u3 = v3;
        v2 = v1 - v2;
        v1 = t2;
        v3 = d;
    }
    while v3.get_bit(U::WIDTH - 2) {
        d = u3 - v3;
        if d < v3 {
            // quot = 1
            t2 = v2;
            t1 = u2;
            u2 = u1 - u2;
            u1 = t1;
            u3 = v3;
            v2 = v1 - v2;
            v1 = t2;
            v3 = d;
        } else if d < (v3 << 1) {
            // quot = 2
            t1 = u2;
            u2 = u1 - (u2 << 1);
            u1 = t1;
            u3 = v3;
            t2 = v2;
            v2 = v1 - (v2 << 1);
            v1 = t2;
            v3 = d - u3;
        } else {
            // quot = 3
            t1 = u2;
            u2 = u1 - S::wrapping_from(3) * u2;
            u1 = t1;
            u3 = v3;
            t2 = v2;
            v2 = v1 - S::wrapping_from(3) * v2;
            v1 = t2;
            v3 = d - (u3 << 1);
        }
    }
    while v3 != U::ZERO {
        d = u3 - v3;
        // overflow not possible, top 2 bits of v3 not set
        if u3 < (v3 << 2) {
            if d < v3 {
                // quot = 1
                t2 = v2;
                t1 = u2;
                u2 = u1 - u2;
                u1 = t1;
                u3 = v3;
                v2 = v1 - v2;
                v1 = t2;
                v3 = d;
            } else if d < (v3 << 1) {
                // quot = 2
                t1 = u2;
                u2 = u1.wrapping_sub(u2 << 1);
                u1 = t1;
                u3 = v3;
                t2 = v2;
                v2 = v1.wrapping_sub(v2 << 1);
                v1 = t2;
                v3 = d - u3;
            } else {
                // quot = 3
                t1 = u2;
                u2 = u1.wrapping_sub(S::wrapping_from(3).wrapping_mul(u2));
                u1 = t1;
                u3 = v3;
                t2 = v2;
                v2 = v1.wrapping_sub(S::wrapping_from(3).wrapping_mul(v2));
                v1 = t2;
                v3 = d.wrapping_sub(u3 << 1);
            }
        } else {
            let (quot, rem) = u3.div_rem(v3);
            t1 = u2;
            u2 = u1.wrapping_sub(S::wrapping_from(quot).wrapping_mul(u2));
            u1 = t1;
            u3 = v3;
            t2 = v2;
            v2 = v1.wrapping_sub(S::wrapping_from(quot).wrapping_mul(v2));
            v1 = t2;
            v3 = rem;
        }
    }
    if u3 != U::ONE {
        return None;
    }
    let mut inverse = U::wrapping_from(v1);
    if u1 <= S::ZERO {
        inverse.wrapping_sub_assign(m);
    }
    let limit = (m >> 1u32).wrapping_neg();
    if inverse < limit {
        let k = (limit - inverse).div_round(m, Ceiling).0;
        inverse.wrapping_add_assign(m.wrapping_mul(k));
    }
    Some(if inverse.get_highest_bit() {
        inverse.wrapping_add(m)
    } else {
        inverse
    })
}}

macro_rules! impl_mod_inverse {
    ($u:ident, $s:ident) => {
        impl ModInverse<$u> for $u {
            type Output = $u;

            /// Computes the multiplicative inverse of a number modulo another number $m$. The input
            /// must be already reduced modulo $m$.
            ///
            /// Returns `None` if $x$ and $m$ are not coprime.
            ///
            /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), m.significant_bits())`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_inverse#mod_inverse).
            #[inline]
            fn mod_inverse(self, m: $u) -> Option<$u> {
                mod_inverse_binary::<$u, $s>(self, m)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_inverse);
