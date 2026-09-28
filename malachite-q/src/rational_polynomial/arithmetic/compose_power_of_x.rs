// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use malachite_base::polynomial::{ComposePowerOfX, ComposePowerOfXAssign, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// The constant p(1), the sum of the numerator's coefficients over the denominator, in lowest terms.
fn value_at_one(p: &RationalPolynomial) -> RationalPolynomial {
    RationalPolynomial::from_numerator_and_denominator(
        IntegerPolynomial::from_coefficients_asc(vec![
            p.numerator.coefficients_asc().iter().sum::<Integer>(),
        ]),
        p.denominator.clone(),
    )
}

impl ComposePowerOfX for RationalPolynomial {
    type Output = Self;

    /// Composes a [`RationalPolynomial`] with $x^k$, giving $p(x^k)$, taking it by value. The
    /// coefficient of $x^i$ moves to $x^{ik}$, and zeros fill the places in between.
    ///
    /// $$
    /// f(p, k) = p(x^k).
    /// $$
    ///
    /// Only the numerator changes when $k$ is positive, so the result stays in lowest terms. When
    /// $k$ is 0 the result is the constant $p(1)$, the sum of the coefficients; when $k$ is 1, or
    /// the polynomial is constant, nothing changes.
    ///
    /// # Worst-case complexity
    /// $T(m, k) = O(mk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is the total number of bits
    /// of the numerator's coefficients and the denominator.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfX;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/3").unwrap();
    /// assert_eq!(p.compose_power_of_x(2).to_string(), "1/2*x^4-1/3");
    /// // With k = 0, this is p(1).
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/6").unwrap();
    /// assert_eq!(p.compose_power_of_x(0).to_string(), "1");
    /// ```
    ///
    /// FLINT has no `fmpq_poly_inflate`; this corresponds to `fmpz_poly_inflate` from
    /// `fmpz_poly/inflate.c`, FLINT 3.6.0, applied to the numerator.
    #[inline]
    fn compose_power_of_x(mut self, k: u64) -> Self {
        self.compose_power_of_x_assign(k);
        self
    }
}

impl ComposePowerOfX for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Composes a [`RationalPolynomial`] with $x^k$, giving $p(x^k)$, taking it by reference. The
    /// coefficient of $x^i$ moves to $x^{ik}$, and zeros fill the places in between.
    ///
    /// $$
    /// f(p, k) = p(x^k).
    /// $$
    ///
    /// Only the numerator changes when $k$ is positive, so the result stays in lowest terms. When
    /// $k$ is 0 the result is the constant $p(1)$, the sum of the coefficients; when $k$ is 1, or
    /// the polynomial is constant, nothing changes.
    ///
    /// # Worst-case complexity
    /// $T(m, k) = O(mk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is the total number of bits
    /// of the numerator's coefficients and the denominator.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfX;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/3").unwrap();
    /// assert_eq!((&p).compose_power_of_x(2).to_string(), "1/2*x^4-1/3");
    /// // With k = 0, this is p(1).
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/6").unwrap();
    /// assert_eq!((&p).compose_power_of_x(0).to_string(), "1");
    /// ```
    ///
    /// FLINT has no `fmpq_poly_inflate`; this corresponds to `fmpz_poly_inflate` from
    /// `fmpz_poly/inflate.c`, FLINT 3.6.0, applied to the numerator.
    fn compose_power_of_x(self, k: u64) -> RationalPolynomial {
        if k == 0 {
            return value_at_one(self);
        }
        RationalPolynomial {
            numerator: (&self.numerator).compose_power_of_x(k),
            denominator: self.denominator.clone(),
        }
    }
}

impl ComposePowerOfXAssign for RationalPolynomial {
    /// Composes a [`RationalPolynomial`] with $x^k$ in place, replacing $p$ with $p(x^k)$. The
    /// coefficient of $x^i$ moves to $x^{ik}$, and zeros fill the places in between.
    ///
    /// $$
    /// p \gets p(x^k).
    /// $$
    ///
    /// Only the numerator changes when $k$ is positive, so the result stays in lowest terms. When
    /// $k$ is 0 the result is the constant $p(1)$, the sum of the coefficients; when $k$ is 1, or
    /// the polynomial is constant, nothing changes.
    ///
    /// # Worst-case complexity
    /// $T(m, k) = O(mk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is the total number of bits
    /// of the numerator's coefficients and the denominator.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfXAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2-1/3").unwrap();
    /// p.compose_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "1/2*x^4-1/3");
    ///
    /// // With k = 0, this is p(1).
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/6").unwrap();
    /// p.compose_power_of_x_assign(0);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// FLINT has no `fmpq_poly_inflate`; this corresponds to `fmpz_poly_inflate` from
    /// `fmpz_poly/inflate.c`, FLINT 3.6.0, applied to the numerator.
    fn compose_power_of_x_assign(&mut self, k: u64) {
        if k == 0 {
            *self = value_at_one(self);
        } else {
            self.numerator.compose_power_of_x_assign(k);
        }
    }
}
