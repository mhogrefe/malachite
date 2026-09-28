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
use core::iter::repeat_with;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{MulPowerOfX, MulPowerOfXAssign};

impl MulPowerOfX for IntegerPolynomial {
    type Output = Self;

    /// Multiplies an [`IntegerPolynomial`] by $x^n$, taking it by value. Every coefficient moves up
    /// by $n$ places, and $n$ zeros fill the places below them.
    ///
    /// $$
    /// f(p, n) = x^np.
    /// $$
    ///
    /// The zero polynomial stays zero, and multiplying by $x^0$ changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $m$ is the total number of bits
    /// of the coefficients.
    ///
    /// # Panics
    /// Panics if the polynomial is nonzero and `n` is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MulPowerOfX;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-3*x+2")
    ///         .unwrap()
    ///         .mul_power_of_x(2)
    ///         .to_string(),
    ///     "x^4-3*x^3+2*x^2"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-5")
    ///         .unwrap()
    ///         .mul_power_of_x(1)
    ///         .to_string(),
    ///     "-5*x"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.mul_power_of_x(3),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_shift_left` from `fmpz_poly/shift_left.c`, FLINT 3.6.0.
    #[inline]
    fn mul_power_of_x(mut self, n: u64) -> Self {
        self.mul_power_of_x_assign(n);
        self
    }
}

impl MulPowerOfX for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies an [`IntegerPolynomial`] by $x^n$, taking it by reference. Every coefficient
    /// moves up by $n$ places, and $n$ zeros fill the places below them.
    ///
    /// $$
    /// f(p, n) = x^np.
    /// $$
    ///
    /// The zero polynomial stays zero, and multiplying by $x^0$ changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $m$ is the total number of bits
    /// of the coefficients.
    ///
    /// # Panics
    /// Panics if the polynomial is nonzero and `n` is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MulPowerOfX;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_power_of_x(2)
    ///         .to_string(),
    ///     "x^4-3*x^3+2*x^2"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-5").unwrap())
    ///         .mul_power_of_x(1)
    ///         .to_string(),
    ///     "-5*x"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::ZERO).mul_power_of_x(3),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_shift_left` from `fmpz_poly/shift_left.c`, FLINT 3.6.0.
    fn mul_power_of_x(self, n: u64) -> IntegerPolynomial {
        if self.coefficients.is_empty() {
            return IntegerPolynomial::ZERO;
        }
        let n = usize::exact_from(n);
        let mut coefficients = Vec::with_capacity(n + self.coefficients.len());
        coefficients.extend(repeat_with(|| Integer::ZERO).take(n));
        coefficients.extend_from_slice(&self.coefficients);
        IntegerPolynomial { coefficients }
    }
}

impl MulPowerOfXAssign for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by $x^n$ in place. Every coefficient moves up by $n$
    /// places, and $n$ zeros fill the places below them.
    ///
    /// $$
    /// p \gets x^np.
    /// $$
    ///
    /// The zero polynomial stays zero, and multiplying by $x^0$ changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $m$ is the total number of bits
    /// of the coefficients.
    ///
    /// # Panics
    /// Panics if the polynomial is nonzero and `n` is greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MulPowerOfXAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.mul_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "x^4-3*x^3+2*x^2");
    ///
    /// let mut p = IntegerPolynomial::from_str("-5").unwrap();
    /// p.mul_power_of_x_assign(1);
    /// assert_eq!(p.to_string(), "-5*x");
    ///
    /// let mut p = IntegerPolynomial::ZERO;
    /// p.mul_power_of_x_assign(3);
    /// assert_eq!(p, IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_shift_left` from `fmpz_poly/shift_left.c`, FLINT 3.6.0.
    fn mul_power_of_x_assign(&mut self, n: u64) {
        if n == 0 || self.coefficients.is_empty() {
            return;
        }
        let n = usize::exact_from(n);
        self.coefficients
            .splice(0..0, repeat_with(|| Integer::ZERO).take(n));
    }
}
