// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2014 William Hart
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Crt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Computes the unique `x` with `x ≡ r1 mod m1`, `x ≡ r2 mod m2`, and `0 <= x < m1 * m2`, or
// `None` if the moduli are not coprime. The residues must be reduced mod their moduli, and the
// product of the moduli must fit in a word.
//
// This is fmpz_CRT from fmpz/CRT.c, FLINT 3.6.0, where all values are word-sized, the residues are
// nonnegative, and a noninvertible modulus is reported as an Option rather than thrown.
private_test_fn! {crt_unsigned<U: PrimitiveUnsigned>(
    r1: U,
    m1: U,
    r2: U,
    m2: U,
) -> Option<U> {
    assert!(r1 < m1, "r1 must be reduced mod m1, but {r1} >= {m1}");
    assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
    assert!(
        m1.checked_mul(m2).is_some(),
        "m1 * m2 must be representable, but {m1} * {m2} overflows"
    );
    let c = m1 % m2;
    if c == U::ZERO {
        // m2 divides m1, so the moduli are coprime only if m2 is 1, and then the second congruence
        // is vacuous.
        return if m2 == U::ONE { Some(r1) } else { None };
    }
    let s = r2.mod_sub(r1 % m2, m2).mod_mul(c.mod_inverse(m2)?, m2);
    // s < m2, so r1 + m1 * s < m1 * m2, which is representable.
    Some(r1 + m1 * s)
}}

macro_rules! impl_crt {
    ($t:ident) => {
        impl Crt for $t {
            type Output = $t;

            /// Combines two congruences by the Chinese remainder theorem: finds the unique number
            /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`.
            ///
            /// Returns `None` if the moduli are not coprime. The residues must be already reduced
            /// modulo their moduli, and the product of the moduli must be representable.
            ///
            /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1
            /// \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `m2.significant_bits()`: the
            /// cost is one modular inversion by the extended Euclidean algorithm on words, which
            /// performs $O(n)$ iterations of constant-cost word operations, with no allocation.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m1`, if `r2` is greater than or equal
            /// to `m2`, or if `m1 * m2` overflows.
            ///
            /// # Examples
            /// See [here](super::crt#crt).
            #[inline]
            fn crt(self, m1: $t, r2: $t, m2: $t) -> Option<$t> {
                crt_unsigned(self, m1, r2, m2)
            }
        }
    };
}
apply_to_unsigneds!(impl_crt);
