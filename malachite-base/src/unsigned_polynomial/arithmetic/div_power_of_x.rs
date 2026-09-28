// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{DivPowerOfX, DivPowerOfXAssign, Polynomial};
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> DivPowerOfX for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides an [`UnsignedPolynomial`] by $x^n$, discarding the remainder, taking it by value.
    /// Every coefficient moves down by $n$ places, and the lowest $n$ are dropped.
    ///
    /// $$
    /// f(p, n) = \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    ///
    /// The result is zero when $n$ is at least the number of coefficients, and dividing by $x^0$
    /// changes nothing. Multiplying the result by $x^n$ and adding back the dropped low part, the
    /// truncation to $n$ coefficients, gives the polynomial back.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DivPowerOfX;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(p.div_power_of_x(2).to_string(), "x+3");
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x").unwrap();
    /// assert_eq!(p.div_power_of_x(1).to_string(), "5");
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(p.div_power_of_x(10), UnsignedPolynomial::<u8>::ZERO);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_shift_right` from `nmod_poly/shift_right.c`, FLINT 3.6.0.
    #[inline]
    fn div_power_of_x(mut self, n: u64) -> Self {
        self.div_power_of_x_assign(n);
        self
    }
}

impl<T: PrimitiveUnsigned> DivPowerOfX for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides an [`UnsignedPolynomial`] by $x^n$, discarding the remainder, taking it by
    /// reference. Every coefficient moves down by $n$ places, and the lowest $n$ are dropped.
    ///
    /// $$
    /// f(p, n) = \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    ///
    /// The result is zero when $n$ is at least the number of coefficients, and dividing by $x^0$
    /// changes nothing. Multiplying the result by $x^n$ and adding back the dropped low part, the
    /// truncation to $n$ coefficients, gives the polynomial back.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DivPowerOfX;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!((&p).div_power_of_x(2).to_string(), "x+3");
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x").unwrap();
    /// assert_eq!((&p).div_power_of_x(1).to_string(), "5");
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!((&p).div_power_of_x(10), UnsignedPolynomial::<u8>::ZERO);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_shift_right` from `nmod_poly/shift_right.c`, FLINT 3.6.0.
    fn div_power_of_x(self, n: u64) -> UnsignedPolynomial<T> {
        if n >= self.len() {
            return UnsignedPolynomial::ZERO;
        }
        // The leading coefficient is kept, so the result needs no trimming.
        UnsignedPolynomial {
            coefficients: self.coefficients[usize::exact_from(n)..].to_vec(),
        }
    }
}

impl<T: PrimitiveUnsigned> DivPowerOfXAssign for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by $x^n$ in place, discarding the remainder. Every
    /// coefficient moves down by $n$ places, and the lowest $n$ are dropped.
    ///
    /// $$
    /// p \gets \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    ///
    /// The result is zero when $n$ is at least the number of coefficients, and dividing by $x^0$
    /// changes nothing. Multiplying the result by $x^n$ and adding back the dropped low part, the
    /// truncation to $n$ coefficients, gives the polynomial back.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DivPowerOfXAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// p.div_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "x+3");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x").unwrap();
    /// p.div_power_of_x_assign(1);
    /// assert_eq!(p.to_string(), "5");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// p.div_power_of_x_assign(10);
    /// assert_eq!(p, UnsignedPolynomial::<u8>::ZERO);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_shift_right` from `nmod_poly/shift_right.c`, FLINT 3.6.0.
    fn div_power_of_x_assign(&mut self, n: u64) {
        if n >= self.len() {
            *self = Self::ZERO;
        } else if n != 0 {
            self.coefficients.drain(..usize::exact_from(n));
        }
    }
}
