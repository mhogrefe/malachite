// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::mem::take;
use malachite_base::num::arithmetic::traits::DivExactAssign;
use malachite_base::polynomial::{DivPowerOfX, DivPowerOfXAssign};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::content_chained::integers_content_chained;
use malachite_nz::natural::Natural;

// Divides out the common factor of a numerator and denominator after low coefficients were dropped
// from a canonical pair. A zero numerator reduces to 0/1, since the content of zero is taken to be
// 0.
//
// This is equivalent to the `fmpq_poly_canonicalise` step of `fmpq_poly_shift_right` from
// `fmpq_poly/shift_right.c`, FLINT 3.6.0, where the denominator is already positive.
fn reduce(mut numerator: IntegerPolynomial, mut denominator: Natural) -> RationalPolynomial {
    let g = integers_content_chained(numerator.coefficients_asc(), &denominator);
    if g != 1u32 {
        denominator.div_exact_assign(&g);
        numerator.div_exact_assign(Integer::from(g));
    }
    RationalPolynomial {
        numerator,
        denominator,
    }
}

impl DivPowerOfX for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by $x^n$, discarding the remainder, taking it by value.
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
    /// The result is kept in lowest terms: dropping the low coefficients can remove the ones that
    /// kept the numerator coprime to the denominator, so the common factor of the new numerator and
    /// denominator is divided out, as in $(10x^2 + 5x + 3)/5$ divided by $x$, which is $2x + 1$.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m (\log m)^2 \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DivPowerOfX;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("2*x^2+x+3/5").unwrap();
    /// assert_eq!(p.div_power_of_x(1).to_string(), "2*x+1");
    /// let p = RationalPolynomial::from_str("1/2*x^3-1/3*x^2").unwrap();
    /// assert_eq!(p.div_power_of_x(2).to_string(), "1/2*x-1/3");
    /// let p = RationalPolynomial::from_str("2*x^2+x+3/5").unwrap();
    /// assert_eq!(p.div_power_of_x(10), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_shift_right` from `fmpq_poly/shift_right.c`, FLINT 3.6.0.
    #[inline]
    fn div_power_of_x(mut self, n: u64) -> Self {
        self.div_power_of_x_assign(n);
        self
    }
}

impl DivPowerOfX for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by $x^n$, discarding the remainder, taking it by reference.
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
    /// The result is kept in lowest terms: dropping the low coefficients can remove the ones that
    /// kept the numerator coprime to the denominator, so the common factor of the new numerator and
    /// denominator is divided out, as in $(10x^2 + 5x + 3)/5$ divided by $x$, which is $2x + 1$.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m (\log m)^2 \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DivPowerOfX;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("2*x^2+x+3/5").unwrap();
    /// assert_eq!((&p).div_power_of_x(1).to_string(), "2*x+1");
    /// let p = RationalPolynomial::from_str("1/2*x^3-1/3*x^2").unwrap();
    /// assert_eq!((&p).div_power_of_x(2).to_string(), "1/2*x-1/3");
    /// let p = RationalPolynomial::from_str("2*x^2+x+3/5").unwrap();
    /// assert_eq!((&p).div_power_of_x(10), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_shift_right` from `fmpq_poly/shift_right.c`, FLINT 3.6.0.
    fn div_power_of_x(self, n: u64) -> RationalPolynomial {
        if n == 0 {
            return self.clone();
        }
        reduce(
            (&self.numerator).div_power_of_x(n),
            self.denominator.clone(),
        )
    }
}

impl DivPowerOfXAssign for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] by $x^n$ in place, discarding the remainder. Every
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
    /// The result is kept in lowest terms: dropping the low coefficients can remove the ones that
    /// kept the numerator coprime to the denominator, so the common factor of the new numerator and
    /// denominator is divided out, as in $(10x^2 + 5x + 3)/5$ divided by $x$, which is $2x + 1$.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m (\log m)^2 \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DivPowerOfXAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("2*x^2+x+3/5").unwrap();
    /// p.div_power_of_x_assign(1);
    /// assert_eq!(p.to_string(), "2*x+1");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^3-1/3*x^2").unwrap();
    /// p.div_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "1/2*x-1/3");
    ///
    /// let mut p = RationalPolynomial::from_str("2*x^2+x+3/5").unwrap();
    /// p.div_power_of_x_assign(10);
    /// assert_eq!(p, RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_shift_right` from `fmpq_poly/shift_right.c`, FLINT 3.6.0.
    fn div_power_of_x_assign(&mut self, n: u64) {
        if n != 0 {
            let p = take(self);
            *self = reduce(p.numerator.div_power_of_x(n), p.denominator);
        }
    }
}
