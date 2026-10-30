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

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::AddMul;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{DivRoot, DivRootAssign};

// The quotient and remainder of `p` by $x - c$, by Horner's rule: the remainder is the value of `p`
// at `c`.
//
// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT 3.6.0,
// except that the remainder is returned too.
fn div_root_helper(p: &IntegerPolynomial, c: &Integer) -> (IntegerPolynomial, Integer) {
    let xs = &p.coefficients;
    match xs.as_slice() {
        [] => return (IntegerPolynomial::ZERO, Integer::ZERO),
        [x] => return (IntegerPolynomial::ZERO, x.clone()),
        _ => {}
    }
    if *c == 0u32 {
        return (
            IntegerPolynomial {
                coefficients: xs[1..].to_vec(),
            },
            xs[0].clone(),
        );
    }
    let mut qs: Vec<Integer> = Vec::with_capacity(xs.len() - 1);
    let mut r = Integer::ZERO;
    for x in xs[1..].iter().rev() {
        r = x.add_mul(&r, c);
        qs.push(r.clone());
    }
    qs.reverse();
    (
        IntegerPolynomial { coefficients: qs },
        (&xs[0]).add_mul(&r, c),
    )
}

impl DivRoot<Integer> for IntegerPolynomial {
    type Output = Self;
    type RemOutput = Integer;

    /// Divides an [`IntegerPolynomial`] by $x - c$, returning the quotient and the remainder, which
    /// is its value at $c$, taking the polynomial by value and $c$ by value.
    ///
    /// $$
    /// f(a, c) = (q, r), \quad a = (x - c)q + r, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm(n + m))$
    ///
    /// $M(n, m) = O(n(n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the larger
    /// number of bits in a coefficient of `self` and in `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivRoot;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let c = Integer::from(3);
    /// let (q, r) = p.div_root(c);
    /// assert_eq!(q.to_string(), "x^2+3*x+11");
    /// assert_eq!(r, 34);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT
    /// 3.6.0, except that the remainder is returned too.
    #[inline]
    fn div_root(self, c: Integer) -> (Self, Integer) {
        div_root_helper(&self, &c)
    }
}

impl DivRoot<&Integer> for IntegerPolynomial {
    type Output = Self;
    type RemOutput = Integer;

    /// Divides an [`IntegerPolynomial`] by $x - c$, returning the quotient and the remainder, which
    /// is its value at $c$, taking the polynomial by value and $c$ by reference.
    ///
    /// $$
    /// f(a, c) = (q, r), \quad a = (x - c)q + r, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm(n + m))$
    ///
    /// $M(n, m) = O(n(n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the larger
    /// number of bits in a coefficient of `self` and in `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivRoot;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let c = Integer::from(3);
    /// let (q, r) = p.div_root(&c);
    /// assert_eq!(q.to_string(), "x^2+3*x+11");
    /// assert_eq!(r, 34);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT
    /// 3.6.0, except that the remainder is returned too.
    #[inline]
    fn div_root(self, c: &Integer) -> (Self, Integer) {
        div_root_helper(&self, c)
    }
}

impl DivRoot<Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;
    type RemOutput = Integer;

    /// Divides an [`IntegerPolynomial`] by $x - c$, returning the quotient and the remainder, which
    /// is its value at $c$, taking the polynomial by reference and $c$ by value.
    ///
    /// $$
    /// f(a, c) = (q, r), \quad a = (x - c)q + r, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm(n + m))$
    ///
    /// $M(n, m) = O(n(n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the larger
    /// number of bits in a coefficient of `self` and in `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivRoot;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let c = Integer::from(3);
    /// let (q, r) = (&p).div_root(c);
    /// assert_eq!(q.to_string(), "x^2+3*x+11");
    /// assert_eq!(r, 34);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT
    /// 3.6.0, except that the remainder is returned too.
    #[inline]
    fn div_root(self, c: Integer) -> (IntegerPolynomial, Integer) {
        div_root_helper(self, &c)
    }
}

impl DivRoot<&Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;
    type RemOutput = Integer;

    /// Divides an [`IntegerPolynomial`] by $x - c$, returning the quotient and the remainder, which
    /// is its value at $c$, taking the polynomial by reference and $c$ by reference.
    ///
    /// $$
    /// f(a, c) = (q, r), \quad a = (x - c)q + r, \quad r = a(c).
    /// $$
    ///
    /// The quotient and remainder are found by Horner's rule.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm(n + m))$
    ///
    /// $M(n, m) = O(n(n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the larger
    /// number of bits in a coefficient of `self` and in `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivRoot;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let c = Integer::from(3);
    /// let (q, r) = (&p).div_root(&c);
    /// assert_eq!(q.to_string(), "x^2+3*x+11");
    /// assert_eq!(r, 34);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT
    /// 3.6.0, except that the remainder is returned too.
    #[inline]
    fn div_root(self, c: &Integer) -> (IntegerPolynomial, Integer) {
        div_root_helper(self, c)
    }
}

impl DivRootAssign<Integer> for IntegerPolynomial {
    type RemOutput = Integer;

    /// Divides an [`IntegerPolynomial`] by $x - c$ in place, replacing it with the quotient and
    /// returning the remainder, which is its value at $c$, taking $c$ by value.
    ///
    /// $$
    /// a \gets q, \quad a = (x - c)q + r,
    /// $$
    /// returning $r = a(c)$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm(n + m))$
    ///
    /// $M(n, m) = O(n(n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the larger
    /// number of bits in a coefficient of `self` and in `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivRootAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let c = Integer::from(3);
    /// assert_eq!(p.div_root_assign(c), 34);
    /// assert_eq!(p.to_string(), "x^2+3*x+11");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT
    /// 3.6.0, except that the remainder is returned too.
    #[inline]
    fn div_root_assign(&mut self, c: Integer) -> Integer {
        let (q, r) = div_root_helper(self, &c);
        *self = q;
        r
    }
}

impl DivRootAssign<&Integer> for IntegerPolynomial {
    type RemOutput = Integer;

    /// Divides an [`IntegerPolynomial`] by $x - c$ in place, replacing it with the quotient and
    /// returning the remainder, which is its value at $c$, taking $c$ by reference.
    ///
    /// $$
    /// a \gets q, \quad a = (x - c)q + r,
    /// $$
    /// returning $r = a(c)$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm(n + m))$
    ///
    /// $M(n, m) = O(n(n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the larger
    /// number of bits in a coefficient of `self` and in `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivRootAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let c = Integer::from(3);
    /// assert_eq!(p.div_root_assign(&c), 34);
    /// assert_eq!(p.to_string(), "x^2+3*x+11");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_root_fmpz` from `fmpz_poly/div_root_fmpz.c`, FLINT
    /// 3.6.0, except that the remainder is returned too.
    #[inline]
    fn div_root_assign(&mut self, c: &Integer) -> Integer {
        let (q, r) = div_root_helper(self, c);
        *self = q;
        r
    }
}
