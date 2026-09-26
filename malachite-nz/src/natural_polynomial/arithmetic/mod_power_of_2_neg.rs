// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Negates every coefficient modulo 2^pow. The coefficients are reduced, so a nonzero one stays
// nonzero and nothing needs trimming.
fn negate(coefficients: &mut [Natural], pow: u64) {
    for c in coefficients {
        c.mod_power_of_2_neg_assign(pow);
    }
}

impl ModPowerOf2Neg for NaturalPolynomial {
    type Output = Self;

    /// Negates a [`NaturalPolynomial`] modulo $2^k$, taking the polynomial by value. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// Each nonzero coefficient $c$ becomes $2^k - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, k) = -p \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!(p.clone().mod_power_of_2_neg(3).to_string(), "3*x^2+7*x+5");
    /// let p = NaturalPolynomial::from_str("x").unwrap();
    /// assert_eq!(p.clone().mod_power_of_2_neg(8).to_string(), "255*x");
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.mod_power_of_2_neg(3),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_neg` from `nmod_poly/neg.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_neg(mut self, pow: u64) -> Self {
        self.mod_power_of_2_neg_assign(pow);
        self
    }
}

impl ModPowerOf2Neg for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Negates a [`NaturalPolynomial`] modulo $2^k$, taking the polynomial by reference. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// Each nonzero coefficient $c$ becomes $2^k - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, k) = -p \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!((&p).mod_power_of_2_neg(3).to_string(), "3*x^2+7*x+5");
    /// let p = NaturalPolynomial::from_str("x").unwrap();
    /// assert_eq!((&p).mod_power_of_2_neg(8).to_string(), "255*x");
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.mod_power_of_2_neg(3),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_neg` from `nmod_poly/neg.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_neg(self, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        let mut coefficients = self.coefficients.clone();
        negate(&mut coefficients, pow);
        NaturalPolynomial { coefficients }
    }
}

impl ModPowerOf2NegAssign for NaturalPolynomial {
    /// Negates a [`NaturalPolynomial`] modulo $2^k$, in place. The coefficients must already be
    /// reduced modulo $2^k$.
    ///
    /// See [`mod_power_of_2_neg`](ModPowerOf2Neg::mod_power_of_2_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_power_of_2_neg_assign(3);
    /// assert_eq!(p.to_string(), "3*x^2+7*x+5");
    /// ```
    #[inline]
    fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
        assert_reduced(self, pow);
        negate(&mut self.coefficients, pow);
    }
}
