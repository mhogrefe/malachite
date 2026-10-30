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
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModAddAssign, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{ModDivRoot, ModDivRootAssign};

// The quotient and remainder modulo `m` of `p`, reduced, by $x - c$, by Horner's rule: the
// remainder is the value of `p` at `c`.
//
// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
// coefficients modulo `m`.
fn helper(p: &NaturalPolynomial, c: &Natural, m: &Natural) -> (NaturalPolynomial, Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
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
    let data: ModMulData =
        <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    let mut qs = Vec::with_capacity(xs.len() - 1);
    let mut r = Natural::ZERO;
    for x in xs[1..].iter().rev() {
        r.mod_mul_precomputed_assign(c, m, &data);
        r.mod_add_assign(x, m);
        qs.push(r.clone());
    }
    qs.reverse();
    r.mod_mul_precomputed_assign(c, m, &data);
    r.mod_add_assign(&xs[0], m);
    (NaturalPolynomial { coefficients: qs }, r)
}

impl ModDivRoot<Natural, Natural> for NaturalPolynomial {
    type Output = Self;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value, $c$ by value, and the
    /// modulus by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_div_root(Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: Natural, m: Natural) -> (Self, Natural) {
        helper(&self, &c, &m)
    }
}

impl ModDivRoot<Natural, &Natural> for NaturalPolynomial {
    type Output = Self;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value, $c$ by value, and the
    /// modulus by reference. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_div_root(Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: Natural, m: &Natural) -> (Self, Natural) {
        helper(&self, &c, m)
    }
}

impl ModDivRoot<&Natural, Natural> for NaturalPolynomial {
    type Output = Self;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value, $c$ by reference, and
    /// the modulus by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_div_root(&Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: &Natural, m: Natural) -> (Self, Natural) {
        helper(&self, c, &m)
    }
}

impl ModDivRoot<&Natural, &Natural> for NaturalPolynomial {
    type Output = Self;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value, $c$ by reference, and
    /// the modulus by reference. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_div_root(&Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: &Natural, m: &Natural) -> (Self, Natural) {
        helper(&self, c, m)
    }
}

impl ModDivRoot<Natural, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference, $c$ by value, and
    /// the modulus by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_div_root(Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: Natural, m: Natural) -> (NaturalPolynomial, Natural) {
        helper(self, &c, &m)
    }
}

impl ModDivRoot<Natural, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference, $c$ by value, and
    /// the modulus by reference. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_div_root(Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: Natural, m: &Natural) -> (NaturalPolynomial, Natural) {
        helper(self, &c, m)
    }
}

impl ModDivRoot<&Natural, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference, $c$ by reference,
    /// and the modulus by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_div_root(&Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: &Natural, m: Natural) -> (NaturalPolynomial, Natural) {
        helper(self, c, &m)
    }
}

impl ModDivRoot<&Natural, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference, $c$ by reference,
    /// and the modulus by reference. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_div_root(&Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6u32);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root(self, c: &Natural, m: &Natural) -> (NaturalPolynomial, Natural) {
        helper(self, c, m)
    }
}

impl ModDivRootAssign<Natural, Natural> for NaturalPolynomial {
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$, taking $c$ by value and the
    /// modulus by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{m},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRootAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_root_assign(Natural::from(3u32), Natural::from(7u32)),
    ///     6u32
    /// );
    /// assert_eq!(p.to_string(), "x^2+3*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root_assign(&mut self, c: Natural, m: Natural) -> Natural {
        let (q, r) = helper(self, &c, &m);
        *self = q;
        r
    }
}

impl ModDivRootAssign<Natural, &Natural> for NaturalPolynomial {
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$, taking $c$ by value and the
    /// modulus by reference. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{m},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRootAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_root_assign(Natural::from(3u32), &Natural::from(7u32)),
    ///     6u32
    /// );
    /// assert_eq!(p.to_string(), "x^2+3*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root_assign(&mut self, c: Natural, m: &Natural) -> Natural {
        let (q, r) = helper(self, &c, m);
        *self = q;
        r
    }
}

impl ModDivRootAssign<&Natural, Natural> for NaturalPolynomial {
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$, taking $c$ by reference and
    /// the modulus by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{m},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRootAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_root_assign(&Natural::from(3u32), Natural::from(7u32)),
    ///     6u32
    /// );
    /// assert_eq!(p.to_string(), "x^2+3*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root_assign(&mut self, c: &Natural, m: Natural) -> Natural {
        let (q, r) = helper(self, c, &m);
        *self = q;
        r
    }
}

impl ModDivRootAssign<&Natural, &Natural> for NaturalPolynomial {
    type RemOutput = Natural;

    /// Divides a [`NaturalPolynomial`] by $x - c$ modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$, taking $c$ by reference and
    /// the modulus by reference. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{m},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRootAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_root_assign(&Natural::from(3u32), &Natural::from(7u32)),
    ///     6u32
    /// );
    /// assert_eq!(p.to_string(), "x^2+3*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// coefficients modulo $m$.
    #[inline]
    fn mod_div_root_assign(&mut self, c: &Natural, m: &Natural) -> Natural {
        let (q, r) = helper(self, c, m);
        *self = q;
        r
    }
}
