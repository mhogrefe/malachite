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
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced,
};
use malachite_base::polynomial::{ModPowerOf2Derivative, ModPowerOf2DerivativeAssign};

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Multiplies the coefficient of x^i, for i at least 1, by i, reduces it modulo 2^pow, and moves it
// to x^(i-1). The products can be zero, so the result is trimmed.
fn mod_power_of_2_derivative_ref(p: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    assert_reduced(p, pow);
    let mut q = NaturalPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| (c * Natural::from(i)).mod_power_of_2(pow))
            .collect(),
    };
    q.trim();
    q
}

fn mod_power_of_2_derivative_in_place(p: &mut NaturalPolynomial, pow: u64) {
    assert_reduced(p, pow);
    if p.coefficients.is_empty() {
        return;
    }
    p.coefficients.remove(0);
    for (i, c) in p.coefficients.iter_mut().enumerate() {
        *c *= Natural::from(i + 1);
        c.mod_power_of_2_assign(pow);
    }
    p.trim();
}

impl ModPowerOf2Derivative for NaturalPolynomial {
    type Output = Self;

    /// Computes the derivative of a [`NaturalPolynomial`] modulo $2^k$, taking the polynomial by
    /// value. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p' \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
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
    /// use malachite_base::polynomial::ModPowerOf2Derivative;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_derivative(2).to_string(), "3*x^2+2*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^5+x").unwrap();
    /// assert_eq!(p.mod_power_of_2_derivative(1).to_string(), "x^4+1");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = NaturalPolynomial::from_str("x^4+x^3+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_derivative(1).to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_derivative(mut self, pow: u64) -> Self {
        mod_power_of_2_derivative_in_place(&mut self, pow);
        self
    }
}

impl ModPowerOf2Derivative for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the derivative of a [`NaturalPolynomial`] modulo $2^k$, taking the polynomial by
    /// reference. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p' \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
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
    /// use malachite_base::polynomial::ModPowerOf2Derivative;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!((&p).mod_power_of_2_derivative(2).to_string(), "3*x^2+2*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^5+x").unwrap();
    /// assert_eq!((&p).mod_power_of_2_derivative(1).to_string(), "x^4+1");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = NaturalPolynomial::from_str("x^4+x^3+1").unwrap();
    /// assert_eq!((&p).mod_power_of_2_derivative(1).to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_derivative(self, pow: u64) -> NaturalPolynomial {
        mod_power_of_2_derivative_ref(self, pow)
    }
}

impl ModPowerOf2DerivativeAssign for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its derivative modulo $2^k$, in place. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p' \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
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
    /// use malachite_base::polynomial::ModPowerOf2DerivativeAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+1").unwrap();
    /// p.mod_power_of_2_derivative_assign(2);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+x").unwrap();
    /// p.mod_power_of_2_derivative_assign(1);
    /// assert_eq!(p.to_string(), "x^4+1");
    ///
    /// // The derivative can lose more than one degree.
    /// let mut p = NaturalPolynomial::from_str("x^4+x^3+1").unwrap();
    /// p.mod_power_of_2_derivative_assign(1);
    /// assert_eq!(p.to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_derivative_assign(&mut self, pow: u64) {
        mod_power_of_2_derivative_in_place(self, pow);
    }
}
