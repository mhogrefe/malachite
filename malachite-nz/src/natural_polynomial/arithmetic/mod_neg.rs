// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Negates every coefficient modulo m. The coefficients are reduced, so a nonzero one stays nonzero
// and nothing needs trimming.
fn negate(coefficients: &mut [Natural], m: &Natural) {
    for c in coefficients {
        c.mod_neg_assign(m);
    }
}

fn mod_neg_ref(p: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert_reduced(p, m);
    let mut coefficients = p.coefficients.clone();
    negate(&mut coefficients, m);
    NaturalPolynomial { coefficients }
}

fn mod_neg_assign(p: &mut NaturalPolynomial, m: &Natural) {
    assert_reduced(p, m);
    negate(&mut p.coefficients, m);
}

impl ModNeg<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Negates a [`NaturalPolynomial`] modulo `m`, taking the polynomial by value and the modulus
    /// by value. The coefficients must already be reduced modulo `m`.
    ///
    /// Each nonzero coefficient $c$ becomes $m - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, m) = -p \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_neg(Natural::from(7u32)).to_string(),
    ///     "2*x^2+6*x+4"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.mod_neg(Natural::from(7u32)),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_neg` from `fmpz_mod_poly/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: Natural) -> Self {
        mod_neg_assign(&mut self, &m);
        self
    }
}

impl ModNeg<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Negates a [`NaturalPolynomial`] modulo `m`, taking the polynomial by value and the modulus
    /// by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// Each nonzero coefficient $c$ becomes $m - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, m) = -p \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_neg(&Natural::from(7u32)).to_string(),
    ///     "2*x^2+6*x+4"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.mod_neg(&Natural::from(7u32)),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_neg` from `fmpz_mod_poly/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: &Natural) -> Self {
        mod_neg_assign(&mut self, m);
        self
    }
}

impl ModNeg<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Negates a [`NaturalPolynomial`] modulo `m`, taking the polynomial by reference and the
    /// modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// Each nonzero coefficient $c$ becomes $m - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, m) = -p \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!((&p).mod_neg(Natural::from(7u32)).to_string(), "2*x^2+6*x+4");
    /// assert_eq!(
    ///     (&NaturalPolynomial::ZERO).mod_neg(Natural::from(7u32)),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_neg` from `fmpz_mod_poly/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: Natural) -> NaturalPolynomial {
        mod_neg_ref(self, &m)
    }
}

impl ModNeg<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Negates a [`NaturalPolynomial`] modulo `m`, taking the polynomial by reference and the
    /// modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// Each nonzero coefficient $c$ becomes $m - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, m) = -p \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_neg(&Natural::from(7u32)).to_string(),
    ///     "2*x^2+6*x+4"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::ZERO).mod_neg(&Natural::from(7u32)),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_neg` from `fmpz_mod_poly/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: &Natural) -> NaturalPolynomial {
        mod_neg_ref(self, m)
    }
}

impl ModNegAssign<Natural> for NaturalPolynomial {
    /// Negates a [`NaturalPolynomial`] modulo `m`, in place, taking the modulus by value. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_neg_assign(Natural::from(7u32));
    /// assert_eq!(p.to_string(), "2*x^2+6*x+4");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: Natural) {
        mod_neg_assign(self, &m);
    }
}

impl ModNegAssign<&Natural> for NaturalPolynomial {
    /// Negates a [`NaturalPolynomial`] modulo `m`, in place, taking the modulus by reference. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_neg_assign(&Natural::from(7u32));
    /// assert_eq!(p.to_string(), "2*x^2+6*x+4");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: &Natural) {
        mod_neg_assign(self, m);
    }
}
