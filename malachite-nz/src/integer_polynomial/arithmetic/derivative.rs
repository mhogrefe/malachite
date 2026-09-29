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
use malachite_base::polynomial::{Derivative, DerivativeAssign};

// The coefficients of the derivative of the polynomial whose coefficients `xs` holds in ascending
// order: the coefficient of x^i, for i at least 1, times i. The leading coefficient of a
// nonconstant polynomial, times its degree, is nonzero, so the result is normalized.
fn derivative_ref(xs: &[Integer]) -> Vec<Integer> {
    xs.iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| c * Integer::from(i))
        .collect()
}

fn derivative_in_place(xs: &mut Vec<Integer>) {
    if xs.is_empty() {
        return;
    }
    xs.remove(0);
    for (i, c) in xs.iter_mut().enumerate() {
        *c *= Integer::from(i + 1);
    }
}

impl Derivative for IntegerPolynomial {
    type Output = Self;

    /// Computes the derivative of an [`IntegerPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = p' = \sum_{i=1}^n ia_ix^{i-1}.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$ and moves to $x^{i-1}$. A constant polynomial,
    /// including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Derivative;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^3-3*x^2+2*x-5").unwrap();
    /// assert_eq!(p.derivative().to_string(), "3*x^2-6*x+2");
    ///
    /// let p = IntegerPolynomial::from_str("7").unwrap();
    /// assert_eq!(p.derivative(), IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_derivative` from `fmpz_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn derivative(mut self) -> Self {
        self.derivative_assign();
        self
    }
}

impl Derivative for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the derivative of an [`IntegerPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = p' = \sum_{i=1}^n ia_ix^{i-1}.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$ and moves to $x^{i-1}$. A constant polynomial,
    /// including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Derivative;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^3-3*x^2+2*x-5").unwrap();
    /// assert_eq!((&p).derivative().to_string(), "3*x^2-6*x+2");
    ///
    /// let p = IntegerPolynomial::from_str("7").unwrap();
    /// assert_eq!((&p).derivative(), IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_derivative` from `fmpz_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn derivative(self) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: derivative_ref(&self.coefficients),
        }
    }
}

impl DerivativeAssign for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its derivative.
    ///
    /// $$
    /// p \gets p' = \sum_{i=1}^n ia_ix^{i-1}.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$ and moves to $x^{i-1}$. A constant polynomial,
    /// including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DerivativeAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3-3*x^2+2*x-5").unwrap();
    /// p.derivative_assign();
    /// assert_eq!(p.to_string(), "3*x^2-6*x+2");
    ///
    /// let mut p = IntegerPolynomial::from_str("7").unwrap();
    /// p.derivative_assign();
    /// assert_eq!(p, IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_derivative` from `fmpz_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn derivative_assign(&mut self) {
        derivative_in_place(&mut self.coefficients);
    }
}
