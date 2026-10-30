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

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModDivRoot, ModDivRootAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;

// The quotient and remainder modulo `m` of the polynomial with coefficients `xs`, reduced, by $x -
// c$, by Horner's rule: the remainder is the value of the polynomial at `c`.
//
// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0.
crate_test_fn! {mod_div_root_with<T: PrimitiveUnsigned>(
    xs: &[T],
    c: T,
    m: T,
) -> (UnsignedPolynomial<T>, T) {
    match xs {
        [] => return (UnsignedPolynomial::ZERO, T::ZERO),
        &[x] => return (UnsignedPolynomial::ZERO, x),
        _ => {}
    }
    if c == T::ZERO {
        return (
            UnsignedPolynomial {
                coefficients: xs[1..].to_vec(),
            },
            xs[0],
        );
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut qs = vec![T::ZERO; xs.len() - 1];
    let mut r = T::ZERO;
    for (q, &x) in qs.iter_mut().zip(&xs[1..]).rev() {
        r = r.mod_mul_precomputed(c, m, &data).mod_add(x, m);
        *q = r;
    }
    let r = r.mod_mul_precomputed(c, m, &data).mod_add(xs[0], m);
    (UnsignedPolynomial { coefficients: qs }, r)
}}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    m: T,
) -> (UnsignedPolynomial<T>, T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
    mod_div_root_with(&p.coefficients, c, m)
}

impl<T: PrimitiveUnsigned> ModDivRoot<T, T> for UnsignedPolynomial<T> {
    type Output = Self;
    type RemOutput = T;

    /// Divides an [`UnsignedPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value. The coefficients and
    /// $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_div_root(3, 7);
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0.
    #[inline]
    fn mod_div_root(self, c: T, m: T) -> (Self, T) {
        helper(&self, c, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivRoot<T, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;
    type RemOutput = T;

    /// Divides an [`UnsignedPolynomial`] by $x - c$ modulo $m$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference. The coefficients
    /// and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRoot;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_div_root(3, 7);
    /// assert_eq!(q.to_string(), "x^2+3*x+4");
    /// assert_eq!(r, 6);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0.
    #[inline]
    fn mod_div_root(self, c: T, m: T) -> (UnsignedPolynomial<T>, T) {
        helper(self, c, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivRootAssign<T, T> for UnsignedPolynomial<T> {
    type RemOutput = T;

    /// Divides an [`UnsignedPolynomial`] by $x - c$ modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$. The coefficients and $c$
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{m},
    /// $$
    /// returning $r = a(c)$.
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, or `c`, is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivRootAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_div_root_assign(3, 7), 6);
    /// assert_eq!(p.to_string(), "x^2+3*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0.
    #[inline]
    fn mod_div_root_assign(&mut self, c: T, m: T) -> T {
        let (q, r) = helper(self, c, m);
        *self = q;
        r
    }
}
