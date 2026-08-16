// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2015 William Hart
//
//      Copyright © 2019 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModDiv;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// Computes `(gcd(x, y), s)`, where `s < y` and `sx ≡ gcd(x, y) mod y`. `x` must be reduced mod
// `y`.
//
// This is n_gcdinv from ulong_extras/gcdinv.c, FLINT 3.6.0, where the GCD is returned along with
// the cofactor.
crate_test_fn! {gcdinv<
    U: WrappingFrom<S> + PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    y: U,
) -> (U, U) {
    assert!(x < y, "x must be reduced mod y, but {x} >= {y}");
    let mut v1 = S::ZERO;
    let mut v2 = S::ONE;
    let mut r = x;
    let mut x = y;
    let mut d;
    let mut t2;
    // y and x both have their highest bit set
    if (x & r).get_highest_bit() {
        d = x - r;
        t2 = v2;
        x = r;
        v2 = v1 - v2;
        v1 = t2;
        r = d;
    }
    // second value has its second-highest bit set
    while r.get_bit(U::WIDTH - 2) {
        d = x - r;
        r = if d < r {
            // quot = 1
            t2 = v2;
            x = r;
            v2 = v1 - v2;
            v1 = t2;
            d
        } else if d < (r << 1) {
            // quot = 2
            x = r;
            t2 = v2;
            v2 = v1 - (v2 << 1);
            v1 = t2;
            d - x
        } else {
            // quot = 3
            x = r;
            t2 = v2;
            v2 = v1 - S::wrapping_from(3) * v2;
            v1 = t2;
            d - (x << 1)
        };
    }
    while r != U::ZERO {
        // overflow not possible, top 2 bits of r not set
        r = if x < (r << 2) {
            // quot < 4
            d = x - r;
            if d < r {
                // quot = 1
                t2 = v2;
                x = r;
                v2 = v1 - v2;
                v1 = t2;
                d
            } else if d < (r << 1) {
                // quot = 2
                x = r;
                t2 = v2;
                v2 = v1.wrapping_sub(v2 << 1);
                v1 = t2;
                d - x
            } else {
                // quot = 3
                x = r;
                t2 = v2;
                v2 = v1.wrapping_sub(S::wrapping_from(3).wrapping_mul(v2));
                v1 = t2;
                d.wrapping_sub(x << 1)
            }
        } else {
            let (quot, rem) = x.div_rem(r);
            x = r;
            t2 = v2;
            v2 = v1.wrapping_sub(S::wrapping_from(quot).wrapping_mul(v2));
            v1 = t2;
            rem
        };
    }
    let mut s = U::wrapping_from(v1);
    if v1 < S::ZERO {
        s.wrapping_add_assign(y);
    }
    (x, s)
}}

// Computes a quotient of `b` and `c` modulo `m`: a `q` such that `qc ≡ b mod m`. `b` and `c` must
// be reduced mod `m`.
//
// This is fmpz_mod_divides from fmpz_mod/divides.c, FLINT 3.6.0, where b and c are word-sized and
// reduced mod the modulus, and the quotient is returned as an Option.
private_test_fn! {mod_div_unsigned<
    U: WrappingFrom<S> + PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    b: U,
    c: U,
    m: U,
) -> Option<U> {
    assert!(b < m, "b must be reduced mod m, but {b} >= {m}");
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
    if c == U::ZERO {
        return if b == U::ZERO {
            Some(U::ZERO)
        } else {
            None
        };
    }
    if b == U::ZERO {
        return Some(U::ZERO);
    }
    // b and c are both nonzero now, so m >= 2. Solve g = cx + my, where g = gcd(c, m).
    let (g, x) = gcdinv::<U, S>(c, m);
    let (q, r) = b.div_rem(g);
    if r == U::ZERO {
        Some(q.mod_mul(x, m))
    } else {
        None
    }
}}

macro_rules! impl_mod_div {
    ($u:ident, $s:ident) => {
        impl ModDiv<$u> for $u {
            type Output = $u;

            /// Divides a number by another number modulo a third number $m$, returning `None` if no
            /// quotient exists. The inputs must be already reduced modulo $m$.
            ///
            /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible
            /// modulo $m$, the quotient is not unique; all quotients differ by multiples of
            /// $m/\gcd(y, m)$, and this function returns one of them.
            ///
            /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$,
            /// if such a $q$ exists.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
            /// extended Euclidean algorithm on words performs $O(n)$ iterations of constant-cost
            /// word operations, with no allocation.
            ///
            /// # Panics
            /// Panics if `self` or `other` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_div#mod_div).
            #[inline]
            fn mod_div(self, other: $u, m: $u) -> Option<$u> {
                mod_div_unsigned::<$u, $s>(self, other, m)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_div);
