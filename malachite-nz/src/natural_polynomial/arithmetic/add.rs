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
use core::cmp::min;
use core::mem::swap;
use core::ops::{Add, AddAssign};

// Adds `ys` into `xs`, cloning the coefficients of `ys` past the end of `xs`. Natural coefficients
// cannot cancel, so the result needs no trimming.
fn add_assign_ref(xs: &mut Vec<Natural>, ys: &[Natural]) {
    let common = min(xs.len(), ys.len());
    for (x, y) in xs.iter_mut().zip(&ys[..common]) {
        *x += y;
    }
    if ys.len() > common {
        xs.extend_from_slice(&ys[common..]);
    }
}

// Adds `ys` into `xs`, reusing whichever of the two is longer.
fn add_assign_val(xs: &mut Vec<Natural>, mut ys: Vec<Natural>) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        *x += y;
    }
}

impl Add<Self> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// Natural coefficients cannot cancel, so the degree of the sum is the larger of the two
    /// degrees.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         + NaturalPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "x^2+5*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(mut self, other: Self) -> Self {
        add_assign_val(&mut self.coefficients, other.coefficients);
        self
    }
}

impl Add<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// Natural coefficients cannot cancel, so the degree of the sum is the larger of the two
    /// degrees.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         + &NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "x^2+5*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(mut self, other: &Self) -> Self {
        add_assign_ref(&mut self.coefficients, &other.coefficients);
        self
    }
}

impl Add<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// Natural coefficients cannot cancel, so the degree of the sum is the larger of the two
    /// degrees.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         + NaturalPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "x^2+5*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(self, other: NaturalPolynomial) -> NaturalPolynomial {
        let mut other = other;
        add_assign_ref(&mut other.coefficients, &self.coefficients);
        other
    }
}

impl Add<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// Natural coefficients cannot cancel, so the degree of the sum is the larger of the two
    /// degrees.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         + &NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "x^2+5*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add` from `fmpz_poly/add.c`, FLINT 3.6.0.
    fn add(self, other: &NaturalPolynomial) -> NaturalPolynomial {
        let (longer, shorter) = if self.coefficients.len() >= other.coefficients.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut coefficients = longer.coefficients.clone();
        add_assign_ref(&mut coefficients, &shorter.coefficients);
        NaturalPolynomial { coefficients }
    }
}

impl AddAssign<Self> for NaturalPolynomial {
    /// Adds another [`NaturalPolynomial`] to a [`NaturalPolynomial`] in place, taking the
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p += NaturalPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "x^2+5*x+7");
    /// ```
    fn add_assign(&mut self, other: Self) {
        add_assign_val(&mut self.coefficients, other.coefficients);
    }
}

impl AddAssign<&Self> for NaturalPolynomial {
    /// Adds another [`NaturalPolynomial`] to a [`NaturalPolynomial`] in place, taking the
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p += &NaturalPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "x^2+5*x+7");
    /// ```
    fn add_assign(&mut self, other: &Self) {
        add_assign_ref(&mut self.coefficients, &other.coefficients);
    }
}
