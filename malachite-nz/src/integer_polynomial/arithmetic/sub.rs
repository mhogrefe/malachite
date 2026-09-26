// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::swap;
use core::ops::{Sub, SubAssign};
use malachite_base::num::arithmetic::traits::NegAssign;

// Subtracts `ys` from `xs`, negating the coefficients of `ys` past the end of `xs`.
fn sub_assign_ref(xs: &mut Vec<Integer>, ys: &[Integer]) {
    let common = min(xs.len(), ys.len());
    for (x, y) in xs.iter_mut().zip(&ys[..common]) {
        *x -= y;
    }
    if ys.len() > common {
        xs.extend(ys[common..].iter().map(|y| -y));
    }
}

// Subtracts `ys` from `xs`, reusing whichever of the two is longer: when `ys` is, it is negated in
// place and `xs` is added to it.
fn sub_assign_val(xs: &mut Vec<Integer>, mut ys: Vec<Integer>) {
    if ys.len() > xs.len() {
        for y in &mut ys {
            y.neg_assign();
        }
        swap(xs, &mut ys);
        for (x, y) in xs.iter_mut().zip(ys) {
            *x += y;
        }
    } else {
        for (x, y) in xs.iter_mut().zip(ys) {
            *x -= y;
        }
    }
}

// Replaces `ys` with `xs - ys`, reusing the storage of `ys`.
fn rsub_assign_ref(ys: &mut Vec<Integer>, xs: &[Integer]) {
    for y in ys.iter_mut() {
        y.neg_assign();
    }
    let common = min(xs.len(), ys.len());
    for (y, x) in ys.iter_mut().zip(&xs[..common]) {
        *y += x;
    }
    if xs.len() > common {
        ys.extend_from_slice(&xs[common..]);
    }
}

impl Sub<Self> for IntegerPolynomial {
    type Output = Self;

    /// Subtracts two [`IntegerPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+x").unwrap()
    ///         - IntegerPolynomial::from_str("x^2-1").unwrap())
    ///     .to_string(),
    ///     "x+1"
    /// );
    /// // A longer subtrahend is negated.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("3*x+1").unwrap()
    ///         - IntegerPolynomial::from_str("x^2").unwrap())
    ///     .to_string(),
    ///     "-x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub` from `fmpz_poly/sub.c`, FLINT 3.6.0.
    fn sub(mut self, other: Self) -> Self {
        sub_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
        self
    }
}

impl Sub<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Subtracts two [`IntegerPolynomial`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+x").unwrap()
    ///         - &IntegerPolynomial::from_str("x^2-1").unwrap())
    ///         .to_string(),
    ///     "x+1"
    /// );
    /// // A longer subtrahend is negated.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("3*x+1").unwrap()
    ///         - &IntegerPolynomial::from_str("x^2").unwrap())
    ///         .to_string(),
    ///     "-x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub` from `fmpz_poly/sub.c`, FLINT 3.6.0.
    fn sub(mut self, other: &Self) -> Self {
        sub_assign_ref(&mut self.coefficients, &other.coefficients);
        self.trim();
        self
    }
}

impl Sub<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Subtracts two [`IntegerPolynomial`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+x").unwrap()
    ///         - IntegerPolynomial::from_str("x^2-1").unwrap())
    ///     .to_string(),
    ///     "x+1"
    /// );
    /// // A longer subtrahend is negated.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("3*x+1").unwrap()
    ///         - IntegerPolynomial::from_str("x^2").unwrap())
    ///     .to_string(),
    ///     "-x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub` from `fmpz_poly/sub.c`, FLINT 3.6.0.
    fn sub(self, other: IntegerPolynomial) -> IntegerPolynomial {
        let mut other = other;
        rsub_assign_ref(&mut other.coefficients, &self.coefficients);
        other.trim();
        other
    }
}

impl Sub<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Subtracts two [`IntegerPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+x").unwrap()
    ///         - &IntegerPolynomial::from_str("x^2-1").unwrap())
    ///         .to_string(),
    ///     "x+1"
    /// );
    /// // A longer subtrahend is negated.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("3*x+1").unwrap()
    ///         - &IntegerPolynomial::from_str("x^2").unwrap())
    ///         .to_string(),
    ///     "-x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub` from `fmpz_poly/sub.c`, FLINT 3.6.0.
    fn sub(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        let mut difference = IntegerPolynomial {
            coefficients: self.coefficients.clone(),
        };
        sub_assign_ref(&mut difference.coefficients, &other.coefficients);
        difference.trim();
        difference
    }
}

impl SubAssign<Self> for IntegerPolynomial {
    /// Subtracts another [`IntegerPolynomial`] from an [`IntegerPolynomial`] in place, taking the
    /// right-hand side by value.
    ///
    /// $$
    /// p \gets p - q.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+x").unwrap();
    /// p -= IntegerPolynomial::from_str("x^2-1").unwrap();
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    fn sub_assign(&mut self, other: Self) {
        sub_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
    }
}

impl SubAssign<&Self> for IntegerPolynomial {
    /// Subtracts another [`IntegerPolynomial`] from an [`IntegerPolynomial`] in place, taking the
    /// right-hand side by reference.
    ///
    /// $$
    /// p \gets p - q.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+x").unwrap();
    /// p -= &IntegerPolynomial::from_str("x^2-1").unwrap();
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    fn sub_assign(&mut self, other: &Self) {
        sub_assign_ref(&mut self.coefficients, &other.coefficients);
        self.trim();
    }
}
