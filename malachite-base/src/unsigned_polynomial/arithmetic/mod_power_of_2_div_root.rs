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

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2DivRoot, ModPowerOf2DivRootAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::mask_coefficients;
use alloc::vec;

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomial with coefficients
// `xs`, reduced, by $x - c$, by Horner's rule in wrapping arithmetic: the remainder is the value of
// the polynomial at `c`.
//
// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with the
// modulus $2^k$.
crate_test_fn! {mod_power_of_2_div_root_with<T: PrimitiveUnsigned>(
    xs: &[T],
    c: T,
    pow: u64,
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
    let mut qs = vec![T::ZERO; xs.len() - 1];
    let mut r = T::ZERO;
    for (q, &x) in qs.iter_mut().zip(&xs[1..]).rev() {
        r = r.wrapping_mul(c).wrapping_add(x);
        *q = r;
    }
    let r = r.wrapping_mul(c).wrapping_add(xs[0]).mod_power_of_2(pow);
    mask_coefficients(&mut qs, pow);
    (UnsignedPolynomial { coefficients: qs }, r)
}}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    pow: u64,
) -> (UnsignedPolynomial<T>, T) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
    mod_power_of_2_div_root_with(&p.coefficients, c, pow)
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRoot<T> for UnsignedPolynomial<T> {
    type Output = Self;
    type RemOutput = T;

    /// Divides an [`UnsignedPolynomial`] by $x - c$ modulo $2^k$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by value. The coefficients and
    /// $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, or `c`, is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRoot;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = p.mod_power_of_2_div_root(3, 3);
    /// assert_eq!(q.to_string(), "x^2+3*x+3");
    /// assert_eq!(r, 2);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root(self, c: T, pow: u64) -> (Self, T) {
        helper(&self, c, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRoot<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;
    type RemOutput = T;

    /// Divides an [`UnsignedPolynomial`] by $x - c$ modulo $2^k$, returning the quotient and the
    /// remainder, which is its value at $c$, taking the polynomial by reference. The coefficients
    /// and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, or `c`, is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRoot;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (q, r) = (&p).mod_power_of_2_div_root(3, 3);
    /// assert_eq!(q.to_string(), "x^2+3*x+3");
    /// assert_eq!(r, 2);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root(self, c: T, pow: u64) -> (UnsignedPolynomial<T>, T) {
        helper(self, c, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRootAssign<T> for UnsignedPolynomial<T> {
    type RemOutput = T;

    /// Divides an [`UnsignedPolynomial`] by $x - c$ modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, which is its value at $c$. The coefficients and $c$
    /// must already be reduced modulo $2^k$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{2^k},
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, or `c`, is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRootAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_div_root_assign(3, 3), 2);
    /// assert_eq!(p.to_string(), "x^2+3*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_root` from `nmod_poly/div_root.c`, FLINT 3.6.0, with
    /// the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_div_root_assign(&mut self, c: T, pow: u64) -> T {
        let (q, r) = helper(self, c, pow);
        *self = q;
        r
    }
}
