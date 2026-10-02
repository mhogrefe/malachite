// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModIntegral, ModIntegralAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// The index `k` reduced modulo `m`, without converting `k` to `T`, which may be too narrow for it.
fn index_mod<T: PrimitiveUnsigned>(k: usize, m: T) -> T {
    match TryInto::<usize>::try_into(m) {
        Ok(m) => T::wrapping_from(k % m),
        // Here m is larger than every usize, so k is already reduced, and fits in T.
        Err(_) => T::wrapping_from(k),
    }
}

// The coefficients of the integral modulo `m` of the polynomial with coefficients `xs`, which is
// nonempty and reduced modulo `m`. The coefficient of $x^{k-1}$, divided by $k$, goes to $x^k$; a
// zero coefficient stays zero, so its $k$ need not be a unit. All the divisions share one
// inversion. Going down from the top, each nonzero coefficient is first multiplied by the product
// of the larger indices with nonzero coefficients, and then its own index joins the product. Once
// the product has every such index, it is inverted, and going up, each nonzero coefficient is
// multiplied by the inverse, and then its own index is multiplied back into the inverse, dividing
// each coefficient by its own index. The result is not trimmed.
//
// This is `_nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, except that FLINT inverts
// the product of every index, needing all of them to be units.
fn mod_integral_coefficients<T: PrimitiveUnsigned>(xs: &[T], m: T) -> Vec<T> {
    let n = xs.len();
    let mut out = vec![T::ZERO; n + 1];
    out[1..].copy_from_slice(xs);
    if n >= 2 {
        let data = T::precompute_mod_mul_data(&m);
        // The product of the indices above k whose coefficients are nonzero.
        let mut product = T::ONE % m;
        for (k, c) in out.iter_mut().enumerate().skip(2).rev() {
            if *c != T::ZERO {
                c.mod_mul_precomputed_assign(product, m, &data);
                product.mod_mul_precomputed_assign(index_mod(k, m), m, &data);
            }
        }
        let inverse = if product == T::ZERO {
            None
        } else {
            product.mod_inverse(m)
        };
        let Some(mut inverse) = inverse else {
            panic!(
                "The integral modulo m is only defined if every k for which the coefficient of \
                x^(k-1) is nonzero is a unit modulo m, but m is {m}"
            );
        };
        for (k, c) in out.iter_mut().enumerate().skip(2) {
            if *c != T::ZERO {
                c.mod_mul_precomputed_assign(inverse, m, &data);
                inverse.mod_mul_precomputed_assign(index_mod(k, m), m, &data);
            }
        }
    }
    out
}

fn mod_integral_ref<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, m);
    if p.coefficients.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut q = UnsignedPolynomial {
        coefficients: mod_integral_coefficients(&p.coefficients, m),
    };
    q.trim();
    q
}

impl<T: PrimitiveUnsigned> ModIntegral<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the integral modulo $m$ of an [`UnsignedPolynomial`] whose constant term is zero,
    /// taking the polynomial by value. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which
    /// that coefficient is nonzero must be a unit modulo $m$; a zero coefficient stays zero. The
    /// divisions share a single modular inversion. The integral of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if, for some $k$,
    /// the coefficient of $x^{k-1}$ is nonzero and $k$ is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegral;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+4*x+5")
    ///         .unwrap()
    ///         .mod_integral(7)
    ///         .to_string(),
    ///     "x^3+2*x^2+5*x"
    /// );
    /// // Dividing by 3 is multiplying by 5 modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2")
    ///         .unwrap()
    ///         .mod_integral(7)
    ///         .to_string(),
    ///     "5*x^3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, except
    /// that FLINT needs every $k$ from 1 to the degree plus 1 to be a unit modulo $m$, even when
    /// the coefficient of $x^{k-1}$ is zero, and aborts otherwise; so, for example, FLINT cannot
    /// integrate $x^2$ modulo 8, whose integral is $3x^3$.
    #[inline]
    fn mod_integral(self, m: T) -> Self {
        mod_integral_ref(&self, m)
    }
}

impl<T: PrimitiveUnsigned> ModIntegral<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the integral modulo $m$ of an [`UnsignedPolynomial`] whose constant term is zero,
    /// taking the polynomial by reference. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which
    /// that coefficient is nonzero must be a unit modulo $m$; a zero coefficient stays zero. The
    /// divisions share a single modular inversion. The integral of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if, for some $k$,
    /// the coefficient of $x^{k-1}$ is nonzero and $k$ is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegral;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x^2+4*x+5").unwrap())
    ///         .mod_integral(7)
    ///         .to_string(),
    ///     "x^3+2*x^2+5*x"
    /// );
    /// // Dividing by 3 is multiplying by 5 modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2").unwrap())
    ///         .mod_integral(7)
    ///         .to_string(),
    ///     "5*x^3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, except
    /// that FLINT needs every $k$ from 1 to the degree plus 1 to be a unit modulo $m$, even when
    /// the coefficient of $x^{k-1}$ is zero, and aborts otherwise; so, for example, FLINT cannot
    /// integrate $x^2$ modulo 8, whose integral is $3x^3$.
    #[inline]
    fn mod_integral(self, m: T) -> UnsignedPolynomial<T> {
        mod_integral_ref(self, m)
    }
}

impl<T: PrimitiveUnsigned> ModIntegralAssign<T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its integral modulo $m$ whose constant term is zero.
    /// Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod m = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which
    /// that coefficient is nonzero must be a unit modulo $m$; a zero coefficient stays zero. The
    /// divisions share a single modular inversion. The integral of zero is zero, for every $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if any coefficient is greater than or equal to `m`, or if, for some $k$,
    /// the coefficient of $x^{k-1}$ is nonzero and $k$ is not a unit modulo $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModIntegralAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+4*x+5").unwrap();
    /// p.mod_integral_assign(7);
    /// assert_eq!(p.to_string(), "x^3+2*x^2+5*x");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_integral` from `nmod_poly/integral.c`, FLINT 3.6.0, except
    /// that FLINT needs every $k$ from 1 to the degree plus 1 to be a unit modulo $m$, even when
    /// the coefficient of $x^{k-1}$ is zero, and aborts otherwise; so, for example, FLINT cannot
    /// integrate $x^2$ modulo 8, whose integral is $3x^3$.
    #[inline]
    fn mod_integral_assign(&mut self, m: T) {
        *self = mod_integral_ref(self, m);
    }
}
