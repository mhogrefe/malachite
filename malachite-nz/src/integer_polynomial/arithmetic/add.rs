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
use core::ops::{Add, AddAssign};

// Adds `ys` into `xs`, cloning the coefficients of `ys` past the end of `xs`.
fn add_assign_ref(xs: &mut Vec<Integer>, ys: &[Integer]) {
    let common = min(xs.len(), ys.len());
    for (x, y) in xs.iter_mut().zip(&ys[..common]) {
        *x += y;
    }
    if ys.len() > common {
        xs.extend_from_slice(&ys[common..]);
    }
}

// Adds `ys` into `xs`, reusing whichever of the two is longer.
fn add_assign_val(xs: &mut Vec<Integer>, mut ys: Vec<Integer>) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        *x += y;
    }
}

impl Add<Self> for IntegerPolynomial {
    type Output = Self;

    /// Adds two [`IntegerPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the sum is lower.
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
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         + IntegerPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "x^2-x+7"
    /// );
    /// // The leading coefficients cancel, and so does the next.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("-x^2+3*x+1").unwrap()
    ///         + IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///     .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(mut self, other: Self) -> Self {
        add_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
        self
    }
}

impl Add<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Adds two [`IntegerPolynomial`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the sum is lower.
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
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         + &IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "x^2-x+7"
    /// );
    /// // The leading coefficients cancel, and so does the next.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("-x^2+3*x+1").unwrap()
    ///         + &IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(mut self, other: &Self) -> Self {
        add_assign_ref(&mut self.coefficients, &other.coefficients);
        self.trim();
        self
    }
}

impl Add<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Adds two [`IntegerPolynomial`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the sum is lower.
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
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         + IntegerPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "x^2-x+7"
    /// );
    /// // The leading coefficients cancel, and so does the next.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-x^2+3*x+1").unwrap()
    ///         + IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///     .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(self, other: IntegerPolynomial) -> IntegerPolynomial {
        let mut other = other;
        add_assign_ref(&mut other.coefficients, &self.coefficients);
        other.trim();
        other
    }
}

impl Add<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Adds two [`IntegerPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// When the two polynomials have the same degree, their leading coefficients can cancel, and
    /// then the degree of the sum is lower.
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
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         + &IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "x^2-x+7"
    /// );
    /// // The leading coefficients cancel, and so does the next.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-x^2+3*x+1").unwrap()
    ///         + &IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        let (longer, shorter) = if self.coefficients.len() >= other.coefficients.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut sum = IntegerPolynomial {
            coefficients: longer.coefficients.clone(),
        };
        add_assign_ref(&mut sum.coefficients, &shorter.coefficients);
        sum.trim();
        sum
    }
}

impl AddAssign<Self> for IntegerPolynomial {
    /// Adds another [`IntegerPolynomial`] to an [`IntegerPolynomial`] in place, taking the
    /// right-hand side by value.
    ///
    /// $$
    /// p \gets p + q.
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
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p += IntegerPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "x^2-x+7");
    /// ```
    fn add_assign(&mut self, other: Self) {
        add_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
    }
}

impl AddAssign<&Self> for IntegerPolynomial {
    /// Adds another [`IntegerPolynomial`] to an [`IntegerPolynomial`] in place, taking the
    /// right-hand side by reference.
    ///
    /// $$
    /// p \gets p + q.
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
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p += &IntegerPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "x^2-x+7");
    /// ```
    fn add_assign(&mut self, other: &Self) {
        add_assign_ref(&mut self.coefficients, &other.coefficients);
        self.trim();
    }
}
