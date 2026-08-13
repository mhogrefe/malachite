// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson Copyright © 2019 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{AddMul, DivMod, Mod, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

const TWELVE: Natural = Natural::const_from(12);
const THREE: Integer = Integer::const_from_unsigned(3);

// The Dedekind sum for 2 < k < 2^63, by the alternating-quotient formula with all state in machine
// words. The alternating sum of the continued-fraction quotients of b/a is less than a in absolute
// value, so it fits a signed word, and the numerator fits a signed double word.
fn dedekind_sum_word(mut a: u64, mut b: u64) -> Rational {
    let mut t = 0i64;
    let (mut m11, mut m12, mut m21, mut m22) = (1u64, 0u64, 0u64, 1u64);
    let mut det_pos = true;
    while b != 0 {
        let (q, r) = a.div_mod(b);
        a = b;
        b = r;
        if det_pos {
            t += i64::wrapping_from(q);
        } else {
            t -= i64::wrapping_from(q);
        }
        det_pos.not_assign();
        (m11, m12) = (q * m11 + m12, m11);
        (m21, m22) = (q * m21 + m22, m21);
    }
    // After an odd number of quotients the matrix determinant is -1 and the reciprocity constant 3
    // joins the alternating sum; the off-diagonal correction changes sign with the parity.
    let num = if det_pos {
        i128::from(t) * i128::from(m11) + i128::from(m21) - i128::from(m12)
    } else {
        i128::from(t - 3) * i128::from(m11) + i128::from(m21) + i128::from(m12)
    };
    Rational::from_integers(
        Integer::from(num),
        Integer::from(Natural::from(m11) * TWELVE),
    )
}

// The same alternating-quotient formula with bignum state, for k too large for the word version.
// FLINT instead runs its continued-fraction ball machinery in exact mode here, which is
// subquadratic; this is the classical algorithm, quadratic in the bit length of k. The exact ball
// mode is the same missing piece as fmpq_get_cfrac, and this function is where it would plug in.
fn dedekind_sum_big(mut a: Natural, mut b: Natural) -> Rational {
    let mut t = Integer::ZERO;
    let mut m11 = Natural::ONE;
    let mut m12 = Natural::ZERO;
    let mut m21 = Natural::ZERO;
    let mut m22 = Natural::ONE;
    let mut det_pos = true;
    while b != 0u32 {
        let (q, r) = (&a).div_mod(&b);
        a = b;
        b = r;
        if det_pos {
            t += Integer::from(&q);
        } else {
            t -= Integer::from(&q);
        }
        det_pos.not_assign();
        (m11, m12) = ((&m12).add_mul(&q, &m11), m11);
        (m21, m22) = ((&m22).add_mul(&q, &m21), m21);
    }
    let num = if det_pos {
        (Integer::from(m21) - Integer::from(m12)).add_mul(t, Integer::from(&m11))
    } else {
        (Integer::from(m21) + Integer::from(m12)).add_mul(t - THREE, Integer::from(&m11))
    };
    Rational::from_integers(num, Integer::from(m11 * TWELVE))
}

impl Rational {
    /// Computes the Dedekind sum:
    ///
    /// $$
    /// s(h, k) = \sum_{i=1}^{k-1}
    /// \left(\left(\frac{i}{k}\right)\right)\left(\left(\frac{hk}{k}\right)\right),
    /// $$
    ///
    /// where $((x))$ is the sawtooth function, $x - \lfloor x \rfloor - 1/2$ for non-integer $x$
    /// and 0 for integer $x$. The sum is evaluated through the alternating sum of the
    /// continued-fraction quotients of $h/k$, not term by term. Following FLINT, the result is 0
    /// whenever $k \leq 2$, including for negative $k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(h.significant_bits(),
    /// k.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::dedekind_sum(&Integer::from(1), &Integer::from(3)).to_string(),
    ///     "1/18"
    /// );
    /// assert_eq!(
    ///     Rational::dedekind_sum(&Integer::from(3), &Integer::from(7)).to_string(),
    ///     "-1/14"
    /// );
    /// assert_eq!(
    ///     Rational::dedekind_sum(&Integer::from(5), &Integer::from(2)),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_dedekind_sum` from `fmpq/dedekind_sum.c`, FLINT 3.6.0.
    pub fn dedekind_sum(h: &Integer, k: &Integer) -> Self {
        if *k <= 2u32 || *h == 0u32 {
            return Self::ZERO;
        }
        // Only h's residue matters, and k > 2 here, so the sum is over 0 <= b < k.
        let b = h.mod_op(k).unsigned_abs();
        if k.significant_bits() < 64 {
            // k has under 64 significant bits, so both fit
            dedekind_sum_word(
                u64::wrapping_from(&k.unsigned_abs()),
                u64::wrapping_from(&b),
            )
        } else {
            dedekind_sum_big(k.unsigned_abs(), b)
        }
    }
}
