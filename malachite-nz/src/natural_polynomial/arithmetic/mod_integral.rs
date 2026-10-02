// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModAddAssign, ModInverse, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{ModIntegral, ModIntegralAssign};

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// The index `k` reduced modulo `m`.
fn index_mod(k: usize, m: &Natural) -> Natural {
    Natural::from(k) % m
}

// The coefficients of the integral modulo `m` of the polynomial with coefficients `xs`, which is
// nonempty and reduced modulo `m`. The coefficient of $x^{k-1}$, divided by $k$, goes to $x^k$.
// Going down from the top, the coefficient of $x^k$ is first multiplied by the product of the
// larger indices, and that product is extended by $k$; once it is $2 \cdot 3 \cdots n$, it is
// inverted, and going up, multiplying by the inverse with the smaller indices put back divides each
// coefficient by its own index. The result is not trimmed.
//
// This is `_nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any
// size.
fn mod_integral_coefficients(xs: Vec<Natural>, m: &Natural) -> Vec<Natural> {
    let n = xs.len();
    let mut out = vec![Natural::ZERO; n + 1];
    for (o, x) in out[1..].iter_mut().zip(xs) {
        *o = x;
    }
    if n >= 2 {
        let data = <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
        // The product of the indices from k + 1 to n.
        let mut product = index_mod(n, m);
        for k in (2..n).rev() {
            out[k].mod_mul_precomputed_assign(&product, m, &data);
            product.mod_mul_precomputed_assign(index_mod(k, m), m, &data);
        }
        let inverse = if product == 0u32 {
            None
        } else {
            (&product).mod_inverse(m)
        };
        let Some(mut inverse) = inverse else {
            panic!(
                "The integral of a polynomial of degree {} is only defined modulo m if every k \
                from 1 to {} is a unit modulo m, but m is {m}",
                n - 1,
                n
            );
        };
        // Now `inverse` is 1/(2 * 3 * ... * n), and each step up removes one more index from it.
        out[2].mod_mul_precomputed_assign(&inverse, m, &data);
        if n >= 3 {
            let doubled = inverse.clone();
            inverse.mod_add_assign(doubled, m);
            out[3].mod_mul_precomputed_assign(&inverse, m, &data);
            for (k, c) in out.iter_mut().enumerate().skip(4) {
                inverse.mod_mul_precomputed_assign(index_mod(k - 1, m), m, &data);
                c.mod_mul_precomputed_assign(&inverse, m, &data);
            }
        }
    }
    out
}

fn mod_integral_owned(p: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert_reduced(&p, m);
    if p.coefficients.is_empty() {
        return p;
    }
    let mut q = NaturalPolynomial {
        coefficients: mod_integral_coefficients(p.coefficients, m),
    };
    q.trim();
    q
}

fn mod_integral_ref(p: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert_reduced(p, m);
    if p.coefficients.is_empty() {
        return NaturalPolynomial::ZERO;
    }
    let mut q = NaturalPolynomial {
        coefficients: mod_integral_coefficients(p.coefficients.clone(), m),
    };
    q.trim();
    q
}

impl ModIntegral<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the integral modulo $m$ of a [`NaturalPolynomial`] whose constant term is zero,
    /// taking the polynomial by value and the modulus by value. Its coefficients must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ from 1 to
    /// the degree plus 1 must be a unit modulo $m$; equivalently, the smallest prime factor of $m$
    /// must exceed the degree plus 1. The divisions share a single modular inversion. The integral
    /// of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if the polynomial
    /// is nonzero and some $k$ from 2 to its degree plus 1 is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegral;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("3*x^2+4*x+5")
    ///         .unwrap()
    ///         .mod_integral(Natural::from(7u32))
    ///         .to_string(),
    ///     "x^3+2*x^2+5*x"
    /// );
    /// // Dividing by 3 is multiplying by 5 modulo 7.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2")
    ///         .unwrap()
    ///         .mod_integral(Natural::from(7u32))
    ///         .to_string(),
    ///     "5*x^3"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly` integral; this is `nmod_poly_integral` from
    /// `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any size.
    #[inline]
    fn mod_integral(self, m: Natural) -> Self {
        mod_integral_owned(self, &m)
    }
}

impl ModIntegral<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the integral modulo $m$ of a [`NaturalPolynomial`] whose constant term is zero,
    /// taking the polynomial by value and the modulus by reference. Its coefficients must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ from 1 to
    /// the degree plus 1 must be a unit modulo $m$; equivalently, the smallest prime factor of $m$
    /// must exceed the degree plus 1. The divisions share a single modular inversion. The integral
    /// of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if the polynomial
    /// is nonzero and some $k$ from 2 to its degree plus 1 is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegral;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("3*x^2+4*x+5")
    ///         .unwrap()
    ///         .mod_integral(&Natural::from(7u32))
    ///         .to_string(),
    ///     "x^3+2*x^2+5*x"
    /// );
    /// // Dividing by 3 is multiplying by 5 modulo 7.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2")
    ///         .unwrap()
    ///         .mod_integral(&Natural::from(7u32))
    ///         .to_string(),
    ///     "5*x^3"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly` integral; this is `nmod_poly_integral` from
    /// `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any size.
    #[inline]
    fn mod_integral(self, m: &Natural) -> Self {
        mod_integral_owned(self, m)
    }
}

impl ModIntegral<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the integral modulo $m$ of a [`NaturalPolynomial`] whose constant term is zero,
    /// taking the polynomial by reference and the modulus by value. Its coefficients must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ from 1 to
    /// the degree plus 1 must be a unit modulo $m$; equivalently, the smallest prime factor of $m$
    /// must exceed the degree plus 1. The divisions share a single modular inversion. The integral
    /// of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if the polynomial
    /// is nonzero and some $k$ from 2 to its degree plus 1 is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegral;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x^2+4*x+5").unwrap())
    ///         .mod_integral(Natural::from(7u32))
    ///         .to_string(),
    ///     "x^3+2*x^2+5*x"
    /// );
    /// // Dividing by 3 is multiplying by 5 modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2").unwrap())
    ///         .mod_integral(Natural::from(7u32))
    ///         .to_string(),
    ///     "5*x^3"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly` integral; this is `nmod_poly_integral` from
    /// `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any size.
    #[inline]
    fn mod_integral(self, m: Natural) -> NaturalPolynomial {
        mod_integral_ref(self, &m)
    }
}

impl ModIntegral<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the integral modulo $m$ of a [`NaturalPolynomial`] whose constant term is zero,
    /// taking the polynomial by reference and the modulus by reference. Its coefficients must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ from 1 to
    /// the degree plus 1 must be a unit modulo $m$; equivalently, the smallest prime factor of $m$
    /// must exceed the degree plus 1. The divisions share a single modular inversion. The integral
    /// of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if the polynomial
    /// is nonzero and some $k$ from 2 to its degree plus 1 is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegral;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x^2+4*x+5").unwrap())
    ///         .mod_integral(&Natural::from(7u32))
    ///         .to_string(),
    ///     "x^3+2*x^2+5*x"
    /// );
    /// // Dividing by 3 is multiplying by 5 modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2").unwrap())
    ///         .mod_integral(&Natural::from(7u32))
    ///         .to_string(),
    ///     "5*x^3"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly` integral; this is `nmod_poly_integral` from
    /// `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any size.
    #[inline]
    fn mod_integral(self, m: &Natural) -> NaturalPolynomial {
        mod_integral_ref(self, m)
    }
}

impl ModIntegralAssign<Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its integral modulo $m$ whose constant term is zero,
    /// taking the modulus by value. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ from 1 to
    /// the degree plus 1 must be a unit modulo $m$; equivalently, the smallest prime factor of $m$
    /// must exceed the degree plus 1. The divisions share a single modular inversion. The integral
    /// of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if the polynomial
    /// is nonzero and some $k$ from 2 to its degree plus 1 is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegralAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("3*x^2+4*x+5").unwrap();
    /// p.mod_integral_assign(Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^3+2*x^2+5*x");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly` integral; this is `nmod_poly_integral` from
    /// `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any size.
    #[inline]
    fn mod_integral_assign(&mut self, m: Natural) {
        *self = mod_integral_owned(core::mem::take(self), &m);
    }
}

impl ModIntegralAssign<&Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its integral modulo $m$ whose constant term is zero,
    /// taking the modulus by reference. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ from 1 to
    /// the degree plus 1 must be a unit modulo $m$; equivalently, the smallest prime factor of $m$
    /// must exceed the degree plus 1. The divisions share a single modular inversion. The integral
    /// of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if the polynomial
    /// is nonzero and some $k$ from 2 to its degree plus 1 is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegralAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("3*x^2+4*x+5").unwrap();
    /// p.mod_integral_assign(&Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^3+2*x^2+5*x");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly` integral; this is `nmod_poly_integral` from
    /// `nmod_poly/integral.c`, FLINT 3.6.0, with a modulus of any size.
    #[inline]
    fn mod_integral_assign(&mut self, m: &Natural) {
        *self = mod_integral_owned(core::mem::take(self), m);
    }
}
