// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2MulAssign,
};
use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;

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
fn mod_power_of_2_evaluate_ref(p: &NaturalPolynomial, x: &Natural, pow: u64) -> Natural {
    assert_reduced_mod_power_of_2(p, x, pow);
    mod_power_of_2_evaluate_coefficients(&p.coefficients, x, pow)
}

// Continues Horner's rule from `value`, the leading coefficient, down through `rest`, modulo 2^pow.
fn horner_mod_power_of_2(mut value: Natural, rest: &[Natural], x: &Natural, pow: u64) -> Natural {
    for c in rest.iter().rev() {
        value.mod_power_of_2_mul_assign(x, pow);
        value.mod_power_of_2_add_assign(c, pow);
    }
    value
}

// Evaluates the polynomial with coefficients `coefficients` at x modulo 2^pow. The coefficients and
// x must be reduced; this is not checked.
pub(crate) fn mod_power_of_2_evaluate_coefficients(
    coefficients: &[Natural],
    x: &Natural,
    pow: u64,
) -> Natural {
    let Some((leading, rest)) = coefficients.split_last() else {
        return Natural::ZERO;
    };
    if rest.is_empty() || *x == 0u32 {
        return coefficients[0].clone();
    }
    horner_mod_power_of_2(leading.clone(), rest, x, pow)
}

// The same as `mod_power_of_2_evaluate_ref`, but taking the polynomial by value, so that the
// coefficient that is returned outright, or that starts Horner's rule, is moved rather than cloned.
fn mod_power_of_2_evaluate_val(p: NaturalPolynomial, x: &Natural, pow: u64) -> Natural {
    assert_reduced_mod_power_of_2(&p, x, pow);
    let mut coefficients = p.coefficients;
    if coefficients.len() <= 1 || *x == 0u32 {
        return coefficients.into_iter().next().unwrap_or(Natural::ZERO);
    }
    let leading = coefficients.pop().unwrap();
    horner_mod_power_of_2(leading, &coefficients, x, pow)
}

impl ModPowerOf2Evaluate<&Natural> for &NaturalPolynomial {
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!((&p).mod_power_of_2_evaluate(&Natural::from(6u32), 4), 13);
    /// assert_eq!((&p).mod_power_of_2_evaluate(&Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!((&p).mod_power_of_2_evaluate(&Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn mod_power_of_2_evaluate(self, x: &Natural, pow: u64) -> Natural {
        mod_power_of_2_evaluate_ref(self, x, pow)
    }
}

impl ModPowerOf2Evaluate<Natural> for &NaturalPolynomial {
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!((&p).mod_power_of_2_evaluate(Natural::from(6u32), 4), 13);
    /// assert_eq!((&p).mod_power_of_2_evaluate(Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!((&p).mod_power_of_2_evaluate(Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn mod_power_of_2_evaluate(self, x: Natural, pow: u64) -> Natural {
        mod_power_of_2_evaluate_ref(self, &x, pow)
    }
}

impl ModPowerOf2Evaluate<Natural> for NaturalPolynomial {
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!(
    ///     p.clone().mod_power_of_2_evaluate(Natural::from(6u32), 4),
    ///     13
    /// );
    /// assert_eq!(p.clone().mod_power_of_2_evaluate(Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!(p.mod_power_of_2_evaluate(Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn mod_power_of_2_evaluate(self, x: Natural, pow: u64) -> Natural {
        mod_power_of_2_evaluate_val(self, &x, pow)
    }
}

impl ModPowerOf2Evaluate<&Natural> for NaturalPolynomial {
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!(
    ///     p.clone().mod_power_of_2_evaluate(&Natural::from(6u32), 4),
    ///     13
    /// );
    /// assert_eq!(p.clone().mod_power_of_2_evaluate(&Natural::ZERO, 4), 7);
    /// // 5 + 3 + 7 = 15.
    /// assert_eq!(p.mod_power_of_2_evaluate(&Natural::ONE, 4), 15);
    /// ```
    #[inline]
    fn mod_power_of_2_evaluate(self, x: &Natural, pow: u64) -> Natural {
        mod_power_of_2_evaluate_val(self, x, pow)
    }
}
