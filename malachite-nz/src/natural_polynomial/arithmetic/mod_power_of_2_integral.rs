// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2MulAssign, Parity,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{ModPowerOf2Integral, ModPowerOf2IntegralAssign};

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The coefficients of the integral modulo $2^k$, where $k$ is `pow`, of the polynomial with
// coefficients `xs`, which is nonempty and reduced modulo $2^k$. The coefficient of $x^{i-1}$,
// divided by $i$, goes to $x^i$; a zero coefficient stays zero. The indices of the nonzero
// coefficients must all be odd, so their product is a unit, and all the divisions share its
// inverse, as in `mod_integral`. The result is not trimmed.
fn mod_power_of_2_integral_coefficients(xs: Vec<Natural>, pow: u64) -> Vec<Natural> {
    let n = xs.len();
    let mut out = Vec::with_capacity(n + 1);
    out.push(Natural::ZERO);
    out.extend(xs);
    // The product of the indices above i whose coefficients are nonzero.
    let mut product = Natural::ONE.mod_power_of_2(pow);
    for (i, c) in out.iter_mut().enumerate().skip(2).rev() {
        if *c != 0u32 {
            assert!(
                i.odd(),
                "The integral modulo 2^pow is only defined if every nonzero coefficient belongs to \
                an even power of x, but the coefficient of x^{} is nonzero",
                i - 1
            );
            c.mod_power_of_2_mul_assign(&product, pow);
            product.mod_power_of_2_mul_assign(Natural::from(i).mod_power_of_2(pow), pow);
        }
    }
    if n >= 2 {
        // `product` is odd, so it has an inverse.
        let mut inverse = product.mod_power_of_2_inverse(pow).unwrap();
        for (i, c) in out.iter_mut().enumerate().skip(2) {
            if *c != 0u32 {
                c.mod_power_of_2_mul_assign(&inverse, pow);
                inverse.mod_power_of_2_mul_assign(Natural::from(i).mod_power_of_2(pow), pow);
            }
        }
    }
    out
}

fn mod_power_of_2_integral_owned(p: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    assert_reduced(&p, pow);
    if p.coefficients.is_empty() {
        return p;
    }
    let mut q = NaturalPolynomial {
        coefficients: mod_power_of_2_integral_coefficients(p.coefficients, pow),
    };
    q.trim();
    q
}

impl ModPowerOf2Integral for NaturalPolynomial {
    type Output = Self;

    /// Computes the integral modulo $2^k$ of a [`NaturalPolynomial`] whose constant term is zero,
    /// taking it by value. Its coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = \int_0^x p(t)\,dt \bmod 2^k = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are
    /// units modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$; a zero
    /// coefficient stays zero. The divisions share a single inversion modulo $2^k$. The integral of
    /// zero is zero, for every $k$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient is greater than or equal to $2^k$, or if the coefficient of an odd
    /// power of $x$ is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Integral;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // Dividing by 3 is multiplying by 3 modulo 8.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2")
    ///         .unwrap()
    ///         .mod_power_of_2_integral(3)
    ///         .to_string(),
    ///     "3*x^3"
    /// );
    /// // Dividing by 5 is multiplying by 13 modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^4+1")
    ///         .unwrap()
    ///         .mod_power_of_2_integral(4)
    ///         .to_string(),
    ///     "13*x^5+x"
    /// );
    /// ```
    ///
    /// This is `nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that FLINT needs every index from 1 to the degree plus 1 to be a unit, even
    /// when its coefficient is zero, and so cannot integrate any polynomial of degree at least 1
    /// modulo $2^k$.
    #[inline]
    fn mod_power_of_2_integral(self, pow: u64) -> Self {
        mod_power_of_2_integral_owned(self, pow)
    }
}

impl ModPowerOf2Integral for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the integral modulo $2^k$ of a [`NaturalPolynomial`] whose constant term is zero,
    /// taking it by reference. Its coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = \int_0^x p(t)\,dt \bmod 2^k = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are
    /// units modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$; a zero
    /// coefficient stays zero. The divisions share a single inversion modulo $2^k$. The integral of
    /// zero is zero, for every $k$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient is greater than or equal to $2^k$, or if the coefficient of an odd
    /// power of $x$ is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Integral;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // Dividing by 3 is multiplying by 3 modulo 8.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2").unwrap())
    ///         .mod_power_of_2_integral(3)
    ///         .to_string(),
    ///     "3*x^3"
    /// );
    /// // Dividing by 5 is multiplying by 13 modulo 16.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^4+1").unwrap())
    ///         .mod_power_of_2_integral(4)
    ///         .to_string(),
    ///     "13*x^5+x"
    /// );
    /// ```
    ///
    /// This is `nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that FLINT needs every index from 1 to the degree plus 1 to be a unit, even
    /// when its coefficient is zero, and so cannot integrate any polynomial of degree at least 1
    /// modulo $2^k$.
    #[inline]
    fn mod_power_of_2_integral(self, pow: u64) -> NaturalPolynomial {
        mod_power_of_2_integral_owned(self.clone(), pow)
    }
}

impl ModPowerOf2IntegralAssign for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its integral modulo $2^k$ whose constant term is zero.
    /// Its coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod 2^k = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are
    /// units modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$; a zero
    /// coefficient stays zero. The divisions share a single inversion modulo $2^k$. The integral of
    /// zero is zero, for every $k$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient is greater than or equal to $2^k$, or if the coefficient of an odd
    /// power of $x$ is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2IntegralAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2").unwrap();
    /// p.mod_power_of_2_integral_assign(3);
    /// assert_eq!(p.to_string(), "3*x^3");
    /// ```
    ///
    /// This is `nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that FLINT needs every index from 1 to the degree plus 1 to be a unit, even
    /// when its coefficient is zero, and so cannot integrate any polynomial of degree at least 1
    /// modulo $2^k$.
    #[inline]
    fn mod_power_of_2_integral_assign(&mut self, pow: u64) {
        *self = mod_power_of_2_integral_owned(core::mem::take(self), pow);
    }
}
