// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
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
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{ModPowerOf2DivRoot, ModPowerOf2DivRootAssign};

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of `p`, reduced, by $x - c$, by
// Horner's rule: the remainder is the value of `p` at `c`.
//
// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with the
// modulus $2^k$.
fn helper(p: &NaturalPolynomial, c: &Natural, pow: u64) -> (NaturalPolynomial, Natural) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
    let xs = &p.coefficients;
    match xs.as_slice() {
        [] => return (NaturalPolynomial::ZERO, Natural::ZERO),
        [x] => return (NaturalPolynomial::ZERO, x.clone()),
        _ => {}
    }
    if *c == 0u32 {
        return (
            NaturalPolynomial {
                coefficients: xs[1..].to_vec(),
            },
            xs[0].clone(),
        );
    }
    let mut qs = Vec::with_capacity(xs.len() - 1);
    let mut r = Natural::ZERO;
    for x in xs[1..].iter().rev() {
        r.mod_power_of_2_mul_assign(c, pow);
        r.mod_power_of_2_add_assign(x, pow);
        qs.push(r.clone());
    }
    qs.reverse();
    r.mod_power_of_2_mul_assign(c, pow);
    r.mod_power_of_2_add_assign(&xs[0], pow);
    (NaturalPolynomial { coefficients: qs }, r)
}

impl ModPowerOf2DivRoot<Natural> for NaturalPolynomial {
    type Output = Self;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $2^k$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value and $c$ by value. The
    /// coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_power_of_2_div_root(Natural::from(3u32), 3);
    /// assert_eq!(q.to_string(), "x^2+3*x+3");
    /// assert_eq!(r, 2u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root(self, c: Natural, pow: u64) -> (Self, Natural) {
        helper(&self, &c, pow)
    }
}

impl ModPowerOf2DivRoot<&Natural> for NaturalPolynomial {
    type Output = Self;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $2^k$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value and $c$ by reference.
    /// The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_power_of_2_div_root(&Natural::from(3u32), 3);
    /// assert_eq!(q.to_string(), "x^2+3*x+3");
    /// assert_eq!(r, 2u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root(self, c: &Natural, pow: u64) -> (Self, Natural) {
        helper(&self, c, pow)
    }
}

impl ModPowerOf2DivRoot<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $2^k$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference and $c$ by value.
    /// The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_power_of_2_div_root(Natural::from(3u32), 3);
    /// assert_eq!(q.to_string(), "x^2+3*x+3");
    /// assert_eq!(r, 2u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root(self, c: Natural, pow: u64) -> (NaturalPolynomial, Natural) {
        helper(self, &c, pow)
    }
}

impl ModPowerOf2DivRoot<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $2^k$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference and $c$ by
    /// reference. The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_power_of_2_div_root(&Natural::from(3u32), 3);
    /// assert_eq!(q.to_string(), "x^2+3*x+3");
    /// assert_eq!(r, 2u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root(self, c: &Natural, pow: u64) -> (NaturalPolynomial, Natural) {
        helper(self, c, pow)
    }
}

impl ModPowerOf2DivRootAssign<Natural> for NaturalPolynomial {
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$, taking $c$ by value. The
    /// coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{2^k},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRootAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_div_root_assign(Natural::from(3u32), 3),
    ///     2u32
    /// );
    /// assert_eq!(p.to_string(), "x^2+3*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root_assign(&mut self, c: Natural, pow: u64) -> Natural {
        let (q, r) = helper(self, &c, pow);
        *self = q;
        r
    }
}

impl ModPowerOf2DivRootAssign<&Natural> for NaturalPolynomial {
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$, taking $c$ by reference.
    /// The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{2^k},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRootAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_div_root_assign(&Natural::from(3u32), 3),
    ///     2u32
    /// );
    /// assert_eq!(p.to_string(), "x^2+3*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root_assign(&mut self, c: &Natural, pow: u64) -> Natural {
        let (q, r) = helper(self, c, pow);
        *self = q;
        r
    }
}
