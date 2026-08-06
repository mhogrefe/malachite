// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::natural::arithmetic::multi_crt::MultiCrt;

impl Integer {
    /// Combines residues modulo pairwise-coprime moduli into the balanced representative: the
    /// unique [`Integer`] $x$ with $-P/2 < x \leq P/2$, where $P$ is the moduli product, that is
    /// congruent to each residue modulo the corresponding modulus. Returns `None` if the moduli are
    /// unusable. The residues must be already reduced.
    ///
    /// The moduli are usable under the same conditions as for
    /// [`Natural::multi_crt`](Natural::multi_crt), which produces the canonical representative
    /// instead.
    ///
    /// $f((m_1, \ldots, m_k), (r_1, \ldots, r_k)) = \operatorname{Some}(x)$, where $-P/2 < x \leq
    /// P/2$, $P = \prod_i m_i$, and $x \equiv r_i \mod m_i$ for all $i$, if the moduli are nonzero,
    /// pairwise coprime, and, when there are at least two, none is 1.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of significant bits of
    /// the product of the moduli.
    ///
    /// # Panics
    /// Panics if `moduli` is empty, if the number of values differs from the number of moduli, or
    /// if any value is greater than or equal to its modulus.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::multi_balanced_crt(
    ///         &[Natural::from(3u32), Natural::from(5u32)],
    ///         &[Natural::from(2u32), Natural::from(3u32)],
    ///     ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::multi_balanced_crt(
    ///         &[Natural::from(4u32), Natural::from(6u32)],
    ///         &[Natural::from(1u32), Natural::from(3u32)],
    ///     ),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpz_multi_CRT from fmpz/multi_CRT.c, FLINT 3.6.0, with sign = 1 and the residues
    /// required to be reduced.
    pub fn multi_balanced_crt(moduli: &[Natural], values: &[Natural]) -> Option<Self> {
        Some(MultiCrt::new(moduli)?.apply_balanced(values))
    }
}
