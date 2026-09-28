// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{ComposePowerOfX, ComposePowerOfXAssign, Polynomial};
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;
use alloc::vec::Vec;

// The value at 1, the sum of the coefficients.
fn sum_of_coefficients<T: PrimitiveUnsigned>(coefficients: &[T]) -> T {
    let mut sum = T::ZERO;
    for &c in coefficients {
        sum = sum
            .checked_add(c)
            .expect("p(1), the sum of the coefficients, overflows the coefficient type");
    }
    sum
}

// Moves the coefficient of x^i to x^(ik), in place, for k at least 2. The vector is first extended
// with zeros to the final length; then the coefficients are moved from the top down, so each lands
// on a place that is already zero. The leading coefficient stays nonzero.
fn spread<T: PrimitiveUnsigned>(coefficients: &mut Vec<T>, k: u64) {
    let len = coefficients.len();
    if len <= 1 {
        return;
    }
    let k = usize::exact_from(k);
    let new_len = (len - 1)
        .checked_mul(k)
        .and_then(|n| n.checked_add(1))
        .unwrap();
    coefficients.resize(new_len, T::ZERO);
    for i in (1..len).rev() {
        coefficients.swap(i, i * k);
    }
}

impl<T: PrimitiveUnsigned> ComposePowerOfX for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes an [`UnsignedPolynomial`] with $x^k$, giving $p(x^k)$, taking it by value. The
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
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`, or if `k` is 0 and the sum
    /// of the coefficients overflows `T`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfX;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.compose_power_of_x(2).to_string(), "x^4+3*x^2+2");
    /// // With k = 0, this is p(1).
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.compose_power_of_x(0).to_string(), "6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inflate` from `nmod_poly/inflate.c`, FLINT 3.6.0.
    #[inline]
    fn compose_power_of_x(mut self, k: u64) -> Self {
        self.compose_power_of_x_assign(k);
        self
    }
}

impl<T: PrimitiveUnsigned> ComposePowerOfX for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Composes an [`UnsignedPolynomial`] with $x^k$, giving $p(x^k)$, taking it by reference. The
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
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`, or if `k` is 0 and the sum
    /// of the coefficients overflows `T`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfX;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).compose_power_of_x(2).to_string(), "x^4+3*x^2+2");
    /// // With k = 0, this is p(1).
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).compose_power_of_x(0).to_string(), "6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inflate` from `nmod_poly/inflate.c`, FLINT 3.6.0.
    fn compose_power_of_x(self, k: u64) -> UnsignedPolynomial<T> {
        if k == 0 {
            return UnsignedPolynomial::from_coefficients_asc(vec![sum_of_coefficients(
                &self.coefficients,
            )]);
        }
        let mut coefficients = self.coefficients.clone();
        if k != 1 {
            spread(&mut coefficients, k);
        }
        UnsignedPolynomial { coefficients }
    }
}

impl<T: PrimitiveUnsigned> ComposePowerOfXAssign for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with $x^k$ in place, replacing $p$ with $p(x^k)$. The
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
    /// where $T$ is time, $M$ is additional memory, $k$ is `k`, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if the degree of the result is greater than `usize::MAX`, or if `k` is 0 and the sum
    /// of the coefficients overflows `T`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposePowerOfXAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.compose_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "x^4+3*x^2+2");
    ///
    /// // With k = 0, this is p(1).
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.compose_power_of_x_assign(0);
    /// assert_eq!(p.to_string(), "6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inflate` from `nmod_poly/inflate.c`, FLINT 3.6.0.
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
