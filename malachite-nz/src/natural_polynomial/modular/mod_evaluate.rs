// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::modular::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::modular::traits::{
    ModAddAssign, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::polynomial::modular::traits::ModEvaluate;

// Checks that a polynomial's coefficients and x are reduced modulo m.
fn assert_reduced_mod(p: &NaturalPolynomial, x: &Natural, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(*x < *m, "x must be reduced mod m, but {x} >= {m}");
}

// Continues Horner's rule from `value`, the leading coefficient, down through `rest`, modulo m.
// Every step multiplies by the same x modulo the same m, so the data for that multiplication is
// computed once.
#[inline]
pub(crate) fn precompute_mod_mul_data(m: &Natural) -> ModMulData {
    <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m)
}

pub(crate) fn horner_mod(
    mut value: Natural,
    rest: &[Natural],
    x: &Natural,
    m: &Natural,
    data: &ModMulData,
) -> Natural {
    for c in rest.iter().rev() {
        value.mod_mul_precomputed_assign(x, m, data);
        value.mod_add_assign(c, m);
    }
    value
}

// Evaluates the polynomial with coefficients `coefficients` at x modulo m, with `data` precomputed
// for multiplying modulo m. The coefficients and x must be reduced; this is not checked.
pub(crate) fn mod_evaluate_coefficients(
    coefficients: &[Natural],
    x: &Natural,
    m: &Natural,
    data: &ModMulData,
) -> Natural {
    let Some((leading, rest)) = coefficients.split_last() else {
        return Natural::ZERO;
    };
    if rest.is_empty() || *x == 0u32 {
        return coefficients[0].clone();
    }
    horner_mod(leading.clone(), rest, x, m, data)
}

// Evaluates a polynomial at x modulo m, after checking that the coefficients and x are reduced.
fn mod_evaluate_ref(p: &NaturalPolynomial, x: &Natural, m: &Natural) -> Natural {
    assert_reduced_mod(p, x, m);
    mod_evaluate_coefficients(&p.coefficients, x, m, &precompute_mod_mul_data(m))
}

// The same as `mod_evaluate_ref`, but taking the polynomial by value, so that the coefficient that
// is returned outright, or that starts Horner's rule, is moved rather than cloned.
fn mod_evaluate_val(p: NaturalPolynomial, x: &Natural, m: &Natural) -> Natural {
    assert_reduced_mod(&p, x, m);
    let mut coefficients = p.coefficients;
    if coefficients.len() <= 1 || *x == 0u32 {
        return coefficients.into_iter().next().unwrap_or(Natural::ZERO);
    }
    let leading = coefficients.pop().unwrap();
    horner_mod(leading, &coefficients, x, m, &precompute_mod_mul_data(m))
}

impl ModEvaluate<&Natural, &Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// all three by reference. The coefficients and the value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).mod_evaluate(&Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!((&p).mod_evaluate(&Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).mod_evaluate(&Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: &Natural, m: &Natural) -> Natural {
        mod_evaluate_ref(self, x, m)
    }
}

impl ModEvaluate<&Natural, Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// the polynomial and the value by reference and the modulus by value. The coefficients and the
    /// value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).mod_evaluate(&Natural::from(6u32), m.clone()), 7);
    /// assert_eq!((&p).mod_evaluate(&Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).mod_evaluate(&Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: &Natural, m: Natural) -> Natural {
        mod_evaluate_ref(self, x, &m)
    }
}

impl ModEvaluate<Natural, &Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// the polynomial and the modulus by reference and the value by value. The coefficients and the
    /// value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).mod_evaluate(Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!((&p).mod_evaluate(Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).mod_evaluate(Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: Natural, m: &Natural) -> Natural {
        mod_evaluate_ref(self, &x, m)
    }
}

impl ModEvaluate<Natural, Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// the polynomial by reference and the value and the modulus by value. The coefficients and the
    /// value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).mod_evaluate(Natural::from(6u32), m.clone()), 7);
    /// assert_eq!((&p).mod_evaluate(Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).mod_evaluate(Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: Natural, m: Natural) -> Natural {
        mod_evaluate_ref(self, &x, &m)
    }
}

impl ModEvaluate<&Natural, &Natural> for NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// the polynomial by value and the value and the modulus by reference. The coefficients and the
    /// value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().mod_evaluate(&Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!(p.clone().mod_evaluate(&Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.mod_evaluate(&Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: &Natural, m: &Natural) -> Natural {
        mod_evaluate_val(self, x, m)
    }
}

impl ModEvaluate<&Natural, Natural> for NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// the polynomial and the modulus by value and the value by reference. The coefficients and the
    /// value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().mod_evaluate(&Natural::from(6u32), m.clone()), 7);
    /// assert_eq!(p.clone().mod_evaluate(&Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.mod_evaluate(&Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: &Natural, m: Natural) -> Natural {
        mod_evaluate_val(self, x, &m)
    }
}

impl ModEvaluate<Natural, &Natural> for NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// the polynomial and the value by value and the modulus by reference. The coefficients and the
    /// value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().mod_evaluate(Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!(p.clone().mod_evaluate(Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.mod_evaluate(Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: Natural, m: &Natural) -> Natural {
        mod_evaluate_val(self, &x, m)
    }
}

impl ModEvaluate<Natural, Natural> for NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo another [`Natural`] $m$, taking
    /// all three by value. The coefficients and the value must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value reaches $m$ and
    /// every multiplication is by the same $x$ modulo the same $m$, whose precomputed data is
    /// reused.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().mod_evaluate(Natural::from(6u32), m.clone()), 7);
    /// assert_eq!(p.clone().mod_evaluate(Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.mod_evaluate(Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: Natural, m: Natural) -> Natural {
        mod_evaluate_val(self, &x, &m)
    }
}
