// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_div::gcdinv;
use crate::num::arithmetic::traits::ModDivList;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// Computes the solutions `q` of `qc ≡ b mod m` as `(start, stride, length)`: the solutions are
// exactly `start + stride * i` for `0 <= i < length`, and `start` is the smallest. `b` and `c` must
// be reduced mod `m`. Unlike a quotient from `mod_div`, the result is canonical: it does not depend
// on the extended GCD's choice of cofactor.
//
// This is fmpz_divides_mod_list from fmpz/divides_mod_list.c, FLINT 3.6.0, where the inputs are
// word-sized and reduced mod the modulus, and the solutions are returned as an Option.
private_test_fn! {mod_div_list_unsigned<
    U: WrappingFrom<S> + PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    b: U,
    c: U,
    m: U,
) -> Option<(U, U, U)> {
    assert!(b < m, "b must be reduced mod m, but {b} >= {m}");
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
    // Solve d = cx + my, where d = gcd(c, m). (FLINT reduces the divisor mod m here; the
    // precondition makes that a no-op.)
    let (d, x) = gcdinv::<U, S>(c, m);
    let (q, r) = b.div_rem(d);
    if r != U::ZERO {
        return None;
    }
    let stride = m / d;
    let start = (x % stride).mod_mul(q % stride, stride);
    Some((start, stride, d))
}}

macro_rules! impl_mod_div_list {
    ($u:ident, $s:ident) => {
        impl ModDivList<$u> for $u {
            type Output = $u;

            /// Finds all quotients of a number and another number modulo a third number $m$,
            /// returning `None` if no quotient exists. The inputs must be already reduced modulo
            /// $m$.
            ///
            /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the
            /// quotients are exactly the numbers $\text{start} + \text{stride} \cdot i$ for $0 \leq
            /// i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride} =
            /// m/g$, and $\text{length} = g$. Unlike the quotient returned by
            /// [`ModDiv`](super::traits::ModDiv), the result is canonical.
            ///
            /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and
            /// only if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
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
            /// See [here](super::mod_div_list#mod_div_list).
            #[inline]
            fn mod_div_list(self, other: $u, m: $u) -> Option<($u, $u, $u)> {
                mod_div_list_unsigned::<$u, $s>(self, other, m)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_div_list);
