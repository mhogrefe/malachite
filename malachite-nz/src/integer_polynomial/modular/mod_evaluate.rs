// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_base::unsigned_polynomial::modular::mod_evaluate::mod_evaluate_slice;

// Returns `c` modulo `m`, in $[0, m)$, where `m_natural` is `m` as a `Natural`.
fn integer_mod_u64(c: &Integer, m: u64, m_natural: &Natural) -> u64 {
    let r = u64::exact_from(&(&c.abs % m_natural));
    if c.sign || r == 0 { r } else { m - r }
}

impl ModEvaluate<u64> for &IntegerPolynomial {
    type Output = u64;

    /// Evaluates an [`IntegerPolynomial`] at a [`u64`], modulo a [`u64`]. The coefficients may be
    /// any [`Integer`]s, and are reduced as the evaluation goes; `x` must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, x, m) = \left ( \sum_{i=0}^{n-1} c_i x^i \right ) \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the result is in $[0,
    /// m)$. The zero polynomial evaluates to 0 everywhere.
    ///
    /// Each coefficient is reduced to a word, and the words are then evaluated as by
    /// [`UnsignedPolynomial::mod_evaluate`](ModEvaluate::mod_evaluate), with Horner's rule and, for
    /// longer polynomials, Shoup's method.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-5*x^2+3*x-7").unwrap();
    /// // -5 * 36 + 3 * 6 - 7 = -169, which is 7 mod 11.
    /// assert_eq!((&p).mod_evaluate(6, 11), 7);
    /// // The coefficients need not be reduced.
    /// let p = IntegerPolynomial::from_str("100*x+1").unwrap();
    /// assert_eq!((&p).mod_evaluate(3, 10), 1);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_mod` from `fmpz_poly/mod_evaluate.c`, FLINT 3.6.0,
    /// except that `x` must be reduced.
    fn mod_evaluate(self, x: u64, m: u64) -> u64 {
        assert_ne!(m, 0, "m cannot be 0");
        assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
        let m_natural = Natural::from(m);
        match self.coefficients.as_slice() {
            [] => 0,
            [c, ..] if x == 0 => integer_mod_u64(c, m, &m_natural),
            coefficients => {
                let reduced: Vec<u64> = coefficients
                    .iter()
                    .map(|c| integer_mod_u64(c, m, &m_natural))
                    .collect();
                mod_evaluate_slice(&reduced, x, m)
            }
        }
    }
}
