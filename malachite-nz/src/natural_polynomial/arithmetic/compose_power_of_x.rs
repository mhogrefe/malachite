// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ComposePowerOfX, ComposePowerOfXAssign, Polynomial};

// The value at 1, the sum of the coefficients.
fn sum_of_coefficients(coefficients: &[Natural]) -> Natural {
    coefficients.iter().sum()
}

// Moves the coefficient of x^i to x^(ik), in place, for k at least 2. The vector is first extended
// with zeros to the final length; then the coefficients are moved from the top down, so each lands
// on a place that is already zero. The leading coefficient stays nonzero.
fn spread(coefficients: &mut Vec<Natural>, k: u64) {
    let len = coefficients.len();
    if len <= 1 {
        return;
    }
    let k = usize::exact_from(k);
    let new_len = (len - 1)
        .checked_mul(k)
        .and_then(|n| n.checked_add(1))
        .unwrap();
    coefficients.resize(new_len, Natural::ZERO);
    for i in (1..len).rev() {
        coefficients.swap(i, i * k);
    }
}

impl ComposePowerOfX for NaturalPolynomial {
    type Output = Self;

    /// Composes a [`NaturalPolynomial`] with $x^k$, giving $p(x^k)$, taking it by value. The
    /// coefficient of $x^i$ moves to $x^{ik}$, and zeros fill the places in between.
    ///
    /// $$
    /// f(p, k) = p(x^k).
    /// $$
    ///
    /// When $k$ is 0 the result is the constant $p(1)$, the sum of the coefficients; when $k$ is 1,
    /// or the polynomial is constant, nothing changes.
    ///
    /// # Worst-case complexity
    /// $T(m, k) = O(mk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is the total number of bits
    /// of the coefficients.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfX;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.compose_power_of_x(2).to_string(), "x^4+3*x^2+2");
    /// // With k = 0, this is p(1).
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.compose_power_of_x(0).to_string(), "6");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_inflate` from `fmpz_poly/inflate.c`, FLINT 3.6.0.
    #[inline]
    fn compose_power_of_x(mut self, k: u64) -> Self {
        self.compose_power_of_x_assign(k);
        self
    }
}

impl ComposePowerOfX for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes a [`NaturalPolynomial`] with $x^k$, giving $p(x^k)$, taking it by reference. The
    /// coefficient of $x^i$ moves to $x^{ik}$, and zeros fill the places in between.
    ///
    /// $$
    /// f(p, k) = p(x^k).
    /// $$
    ///
    /// When $k$ is 0 the result is the constant $p(1)$, the sum of the coefficients; when $k$ is 1,
    /// or the polynomial is constant, nothing changes.
    ///
    /// # Worst-case complexity
    /// $T(m, k) = O(mk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is the total number of bits
    /// of the coefficients.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfX;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).compose_power_of_x(2).to_string(), "x^4+3*x^2+2");
    /// // With k = 0, this is p(1).
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).compose_power_of_x(0).to_string(), "6");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_inflate` from `fmpz_poly/inflate.c`, FLINT 3.6.0.
    fn compose_power_of_x(self, k: u64) -> NaturalPolynomial {
        if k == 0 {
            return NaturalPolynomial::from_coefficients_asc(vec![sum_of_coefficients(
                &self.coefficients,
            )]);
        }
        let mut coefficients = self.coefficients.clone();
        if k != 1 {
            spread(&mut coefficients, k);
        }
        NaturalPolynomial { coefficients }
    }
}

impl ComposePowerOfXAssign for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with $x^k$ in place, replacing $p$ with $p(x^k)$. The
    /// coefficient of $x^i$ moves to $x^{ik}$, and zeros fill the places in between.
    ///
    /// $$
    /// p \gets p(x^k).
    /// $$
    ///
    /// When $k$ is 0 the result is the constant $p(1)$, the sum of the coefficients; when $k$ is 1,
    /// or the polynomial is constant, nothing changes.
    ///
    /// # Worst-case complexity
    /// $T(m, k) = O(mk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is the total number of bits
    /// of the coefficients.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfXAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.compose_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "x^4+3*x^2+2");
    ///
    /// // With k = 0, this is p(1).
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.compose_power_of_x_assign(0);
    /// assert_eq!(p.to_string(), "6");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_inflate` from `fmpz_poly/inflate.c`, FLINT 3.6.0.
    fn compose_power_of_x_assign(&mut self, k: u64) {
        match k {
            0 => {
                *self = Self::from_coefficients_asc(vec![sum_of_coefficients(&self.coefficients)]);
            }
            1 => {}
            _ => spread(&mut self.coefficients, k),
        }
    }
}
