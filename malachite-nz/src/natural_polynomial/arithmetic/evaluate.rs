// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::evaluate::evaluate;
use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModAddAssign, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign, ModPowerOf2AddAssign,
    ModPowerOf2IsReduced, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    Evaluate, EvaluateMany, EvaluateManyMod, EvaluateMod, EvaluateModPowerOf2,
};

impl Evaluate<&Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], taking both by reference.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used unless the polynomial is long compared with the size of `x`, in which
    /// case divide and conquer, which pairs off coefficients and merges the pairs so that each
    /// multiplication has operands of about the same size, is faster.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).evaluate(&Natural::ZERO), 2);
    /// assert_eq!((&p).evaluate(&Natural::ONE), 6);
    /// assert_eq!((&p).evaluate(&Natural::from(10u32)), 132);
    ///
    /// let q = NaturalPolynomial::from_str("x^100+1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(&Natural::TWO).to_string(),
    ///     "1267650600228229401496703205377"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT
    /// 3.6.0, for a polynomial whose coefficients are all nonnegative, evaluated at a nonnegative
    /// value.
    #[inline]
    fn evaluate(self, x: &Natural) -> Natural {
        evaluate(&self.coefficients, x)
    }
}

impl Evaluate<Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], taking the polynomial by reference and
    /// the value by value.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used unless the polynomial is long compared with the size of `x`, in which
    /// case divide and conquer, which pairs off coefficients and merges the pairs so that each
    /// multiplication has operands of about the same size, is faster.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).evaluate(Natural::ZERO), 2);
    /// assert_eq!((&p).evaluate(Natural::ONE), 6);
    /// assert_eq!((&p).evaluate(Natural::from(10u32)), 132);
    ///
    /// let q = NaturalPolynomial::from_str("x^100+1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(Natural::TWO).to_string(),
    ///     "1267650600228229401496703205377"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT
    /// 3.6.0, for a polynomial whose coefficients are all nonnegative, evaluated at a nonnegative
    /// value.
    #[inline]
    fn evaluate(self, x: Natural) -> Natural {
        evaluate(&self.coefficients, &x)
    }
}

// Checks that a polynomial's coefficients and x are reduced modulo 2^pow.
fn assert_reduced_mod_power_of_2(p: &NaturalPolynomial, x: &Natural, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
}

// Evaluates a polynomial at x modulo 2^pow with Horner's rule, after checking that the coefficients
// and x are reduced.
fn evaluate_mod_power_of_2_ref(p: &NaturalPolynomial, x: &Natural, pow: u64) -> Natural {
    assert_reduced_mod_power_of_2(p, x, pow);
    let Some((leading, rest)) = p.coefficients.split_last() else {
        return Natural::ZERO;
    };
    if rest.is_empty() || *x == 0u32 {
        return p.coefficients[0].clone();
    }
    let mut value = leading.clone();
    for c in rest.iter().rev() {
        value.mod_power_of_2_mul_assign(x, pow);
        value.mod_power_of_2_add_assign(c, pow);
    }
    value
}

// The same as `evaluate_mod_power_of_2_ref`, but taking the polynomial by value, so that the
// coefficient that is returned outright, or that starts Horner's rule, is moved rather than cloned.
fn evaluate_mod_power_of_2_val(p: NaturalPolynomial, x: &Natural, pow: u64) -> Natural {
    assert_reduced_mod_power_of_2(&p, x, pow);
    let mut coefficients = p.coefficients;
    if coefficients.len() <= 1 || *x == 0u32 {
        return coefficients.into_iter().next().unwrap_or(Natural::ZERO);
    }
    let mut value = coefficients.pop().unwrap();
    for c in coefficients.iter().rev() {
        value.mod_power_of_2_mul_assign(x, pow);
        value.mod_power_of_2_add_assign(c, pow);
    }
    value
}

impl EvaluateModPowerOf2<&Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo $2^k$, taking both by reference.
    /// The coefficients and the value must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value exceeds $2^k$ and
    /// no step multiplies numbers larger than that.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!((&p).evaluate_mod_power_of_2(&Natural::from(6u32), 4), 13);
    /// assert_eq!((&p).evaluate_mod_power_of_2(&Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!((&p).evaluate_mod_power_of_2(&Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn evaluate_mod_power_of_2(self, x: &Natural, pow: u64) -> Natural {
        evaluate_mod_power_of_2_ref(self, x, pow)
    }
}

impl EvaluateModPowerOf2<Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo $2^k$, taking the polynomial by
    /// reference and the value by value. The coefficients and the value must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value exceeds $2^k$ and
    /// no step multiplies numbers larger than that.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!((&p).evaluate_mod_power_of_2(Natural::from(6u32), 4), 13);
    /// assert_eq!((&p).evaluate_mod_power_of_2(Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!((&p).evaluate_mod_power_of_2(Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn evaluate_mod_power_of_2(self, x: Natural, pow: u64) -> Natural {
        evaluate_mod_power_of_2_ref(self, &x, pow)
    }
}

impl EvaluateModPowerOf2<Natural> for NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo $2^k$, taking both by value. The
    /// coefficients and the value must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value exceeds $2^k$ and
    /// no step multiplies numbers larger than that.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!(
    ///     p.clone().evaluate_mod_power_of_2(Natural::from(6u32), 4),
    ///     13
    /// );
    /// assert_eq!(p.clone().evaluate_mod_power_of_2(Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!(p.evaluate_mod_power_of_2(Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn evaluate_mod_power_of_2(self, x: Natural, pow: u64) -> Natural {
        evaluate_mod_power_of_2_val(self, &x, pow)
    }
}

impl EvaluateModPowerOf2<&Natural> for NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], modulo $2^k$, taking the polynomial by
    /// value and the value by reference. The coefficients and the value must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used, reducing after every step, so no intermediate value exceeds $2^k$ and
    /// no step multiplies numbers larger than that.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `x` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!(
    ///     p.clone().evaluate_mod_power_of_2(&Natural::from(6u32), 4),
    ///     13
    /// );
    /// assert_eq!(p.clone().evaluate_mod_power_of_2(&Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!(p.evaluate_mod_power_of_2(&Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn evaluate_mod_power_of_2(self, x: &Natural, pow: u64) -> Natural {
        evaluate_mod_power_of_2_val(self, x, pow)
    }
}

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
fn precompute_mod_mul_data(m: &Natural) -> ModMulData {
    <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m)
}

fn horner_mod(
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

// Evaluates a polynomial at x modulo m, after checking that the coefficients and x are reduced.
fn evaluate_mod_ref(p: &NaturalPolynomial, x: &Natural, m: &Natural) -> Natural {
    assert_reduced_mod(p, x, m);
    let Some((leading, rest)) = p.coefficients.split_last() else {
        return Natural::ZERO;
    };
    if rest.is_empty() || *x == 0u32 {
        return p.coefficients[0].clone();
    }
    horner_mod(leading.clone(), rest, x, m, &precompute_mod_mul_data(m))
}

// The same as `evaluate_mod_ref`, but taking the polynomial by value, so that the coefficient that
// is returned outright, or that starts Horner's rule, is moved rather than cloned.
fn evaluate_mod_val(p: NaturalPolynomial, x: &Natural, m: &Natural) -> Natural {
    assert_reduced_mod(&p, x, m);
    let mut coefficients = p.coefficients;
    if coefficients.len() <= 1 || *x == 0u32 {
        return coefficients.into_iter().next().unwrap_or(Natural::ZERO);
    }
    let leading = coefficients.pop().unwrap();
    horner_mod(leading, &coefficients, x, m, &precompute_mod_mul_data(m))
}

impl EvaluateMod<&Natural, &Natural> for &NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).evaluate_mod(&Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!((&p).evaluate_mod(&Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).evaluate_mod(&Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: &Natural, m: &Natural) -> Natural {
        evaluate_mod_ref(self, x, m)
    }
}

impl EvaluateMod<&Natural, Natural> for &NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).evaluate_mod(&Natural::from(6u32), m.clone()), 7);
    /// assert_eq!((&p).evaluate_mod(&Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).evaluate_mod(&Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: &Natural, m: Natural) -> Natural {
        evaluate_mod_ref(self, x, &m)
    }
}

impl EvaluateMod<Natural, &Natural> for &NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).evaluate_mod(Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!((&p).evaluate_mod(Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).evaluate_mod(Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: Natural, m: &Natural) -> Natural {
        evaluate_mod_ref(self, &x, m)
    }
}

impl EvaluateMod<Natural, Natural> for &NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!((&p).evaluate_mod(Natural::from(6u32), m.clone()), 7);
    /// assert_eq!((&p).evaluate_mod(Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!((&p).evaluate_mod(Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: Natural, m: Natural) -> Natural {
        evaluate_mod_ref(self, &x, &m)
    }
}

impl EvaluateMod<&Natural, &Natural> for NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().evaluate_mod(&Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!(p.clone().evaluate_mod(&Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.evaluate_mod(&Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: &Natural, m: &Natural) -> Natural {
        evaluate_mod_val(self, x, m)
    }
}

impl EvaluateMod<&Natural, Natural> for NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().evaluate_mod(&Natural::from(6u32), m.clone()), 7);
    /// assert_eq!(p.clone().evaluate_mod(&Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.evaluate_mod(&Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: &Natural, m: Natural) -> Natural {
        evaluate_mod_val(self, x, &m)
    }
}

impl EvaluateMod<Natural, &Natural> for NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().evaluate_mod(Natural::from(6u32), &m.clone()), 7);
    /// assert_eq!(p.clone().evaluate_mod(Natural::ZERO, &m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.evaluate_mod(Natural::ONE, &m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: Natural, m: &Natural) -> Natural {
        evaluate_mod_val(self, &x, m)
    }
}

impl EvaluateMod<Natural, Natural> for NaturalPolynomial {
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
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let m = Natural::from(11u32);
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 7 mod 11.
    /// assert_eq!(p.clone().evaluate_mod(Natural::from(6u32), m.clone()), 7);
    /// assert_eq!(p.clone().evaluate_mod(Natural::ZERO, m.clone()), 7);
    /// // 5 + 3 + 7 = 15, which is 4 mod 11.
    /// assert_eq!(p.evaluate_mod(Natural::ONE, m), 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz` from `fmpz_mod_poly/evaluate_fmpz.c`,
    /// FLINT 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: Natural, m: Natural) -> Natural {
        evaluate_mod_val(self, &x, &m)
    }
}

fn evaluate_many_mod(p: &NaturalPolynomial, xs: &[Natural], m: &Natural) -> Vec<Natural> {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    for x in xs {
        assert!(*x < *m, "x must be reduced mod m, but {x} >= {m}");
    }
    let Some((leading, rest)) = p.coefficients.split_last() else {
        return vec![Natural::ZERO; xs.len()];
    };
    let data = precompute_mod_mul_data(m);
    xs.iter()
        .map(|x| {
            if rest.is_empty() || *x == 0u32 {
                p.coefficients[0].clone()
            } else {
                horner_mod(leading.clone(), rest, x, m, &data)
            }
        })
        .collect()
}

impl EvaluateManyMod<Natural, &Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s, modulo a [`Natural`],
    /// taking the modulus by reference. The coefficients and the values must already be reduced
    /// modulo `m`.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`evaluate_mod`](malachite_base::polynomial::EvaluateMod::evaluate_mod) at each value, but
    /// the polynomial is checked, and the data for modular multiplication by `m` computed, once.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(nkb \log b \log\log b)$
    ///
    /// $M(k, b) = O(kb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `xs.len()`, and $b$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or any value in `xs` is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateManyMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     (&p).evaluate_many_mod(&xs, &Natural::from(13u32)),
    ///     [7u32, 2, 7, 9].map(Natural::from)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz_vec_iter` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that the values must be reduced.
    #[inline]
    fn evaluate_many_mod(self, xs: &[Natural], m: &Natural) -> Vec<Natural> {
        evaluate_many_mod(self, xs, m)
    }
}

impl EvaluateManyMod<Natural, Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s, modulo a [`Natural`],
    /// taking the modulus by value. The coefficients and the values must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`evaluate_mod`](malachite_base::polynomial::EvaluateMod::evaluate_mod) at each value, but
    /// the polynomial is checked, and the data for modular multiplication by `m` computed, once.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(nkb \log b \log\log b)$
    ///
    /// $M(k, b) = O(kb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `xs.len()`, and $b$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or any value in `xs` is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateManyMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     (&p).evaluate_many_mod(&xs, Natural::from(13u32)),
    ///     [7u32, 2, 7, 9].map(Natural::from)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz_vec_iter` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that the values must be reduced.
    #[inline]
    fn evaluate_many_mod(self, xs: &[Natural], m: Natural) -> Vec<Natural> {
        evaluate_many_mod(self, xs, &m)
    }
}

impl EvaluateMany<Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// Each value is found as by [`evaluate`](malachite_base::polynomial::Evaluate::evaluate),
    /// which chooses between Horner's rule and divide and conquer by the length of the polynomial
    /// and the size of the value.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(kn \log^2 n \log\log n)$
    ///
    /// $M(n, k) = O(kn \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $n$ is `self.len()`
    /// times the larger of the greatest number of bits of any coefficient and the greatest number
    /// of bits of any value in `xs`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateMany;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// assert_eq!(
    ///     (&p).evaluate_many(&xs),
    ///     [7u32, 15, 33, 61].map(Natural::from)
    /// );
    /// ```
    #[inline]
    fn evaluate_many(self, xs: &[Natural]) -> Vec<Natural> {
        xs.iter().map(|x| self.evaluate(x)).collect()
    }
}
