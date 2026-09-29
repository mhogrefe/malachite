// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2014 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use crate::integer_polynomial::arithmetic::mul::tiny::{mul_to_out_tiny_1, mul_to_out_tiny_2};
use crate::integer_polynomial::arithmetic::scalar_mul::integers_mul_scalar_to_out;
use crate::integer_polynomial::arithmetic::square::square_to_out;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::integer_polynomial::arithmetic::vec::{TinyKernel, tiny_kernel};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Mul, MulAssign};
use core::ptr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

pub mod classical;
pub mod tiny;

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// where `xs.len() >= ys.len() >= 1`. `out` must have length `xs.len() + ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^2 m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0. Only the tiny and
// classical algorithms have been ported so far, so classical multiplication stands in for the
// others.
crate_test_fn! {mul_greater_to_out(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    let len1 = xs.len();
    let len2 = ys.len();
    if len2 == 1 {
        integers_mul_scalar_to_out(out, xs, &ys[0]);
        return;
    }
    if ptr::eq(xs, ys) {
        square_to_out(out, xs);
        return;
    }
    let bits1 = vec_max_bits(xs).0;
    let bits2 = vec_max_bits(ys).0;
    let len1 = u64::exact_from(len1);
    let len2 = u64::exact_from(len2);
    let half_bits = (bits1 + bits2) >> 1;
    match tiny_kernel(bits1, bits2, len2, len2 < 40 + half_bits || len1 < 70 + half_bits) {
        Some(TinyKernel::OneWord) => mul_to_out_tiny_1(out, xs, ys),
        Some(TinyKernel::TwoWord) => mul_to_out_tiny_2(out, xs, ys),
        None => mul_to_out_classical(out, xs, ys),
    }
}}

// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
fn mul_ref_ref(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    if xs.is_empty() || ys.is_empty() {
        return Vec::new();
    }
    let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
    if xs.len() >= ys.len() {
        mul_greater_to_out(&mut out, xs, ys);
    } else {
        mul_greater_to_out(&mut out, ys, xs);
    }
    out
}

impl Mul<Self> for IntegerPolynomial {
    type Output = Self;

    /// Multiplies two [`IntegerPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The integers have no zero divisors, so the degree of a product of nonzero polynomials is the
    /// sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         * IntegerPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "2*x^3-x^2-11*x+10"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap()
    ///         * IntegerPolynomial::from_str("x-1").unwrap())
    ///     .to_string(),
    ///     "x^2-1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap() * IntegerPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: Self) -> Self {
        &self * &other
    }
}

impl Mul<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Multiplies two [`IntegerPolynomial`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The integers have no zero divisors, so the degree of a product of nonzero polynomials is the
    /// sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         * &IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "2*x^3-x^2-11*x+10"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap()
    ///         * &IntegerPolynomial::from_str("x-1").unwrap())
    ///         .to_string(),
    ///     "x^2-1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap() * &IntegerPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: &Self) -> Self {
        &self * other
    }
}

impl Mul<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies two [`IntegerPolynomial`]s, taking the first by reference and the second by
    /// value.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The integers have no zero divisors, so the degree of a product of nonzero polynomials is the
    /// sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         * IntegerPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "2*x^3-x^2-11*x+10"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap()
    ///         * IntegerPolynomial::from_str("x-1").unwrap())
    ///     .to_string(),
    ///     "x^2-1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap() * IntegerPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: IntegerPolynomial) -> IntegerPolynomial {
        self * &other
    }
}

impl Mul<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies two [`IntegerPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The integers have no zero divisors, so the degree of a product of nonzero polynomials is the
    /// sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap()
    ///         * &IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "2*x^3-x^2-11*x+10"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap()
    ///         * &IntegerPolynomial::from_str("x-1").unwrap())
    ///         .to_string(),
    ///     "x^2-1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap() * &IntegerPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    fn mul(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: mul_ref_ref(&self.coefficients, &other.coefficients),
        }
    }
}

impl MulAssign<Self> for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by another [`IntegerPolynomial`] in place, taking the
    /// right-hand side by value.
    ///
    /// $$
    /// p \gets pq.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p *= IntegerPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "2*x^3-x^2-11*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Mul::mul(&*self, &other);
    }
}

impl MulAssign<&Self> for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by another [`IntegerPolynomial`] in place, taking the
    /// right-hand side by reference.
    ///
    /// $$
    /// p \gets pq.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p *= &IntegerPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "2*x^3-x^2-11*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        *self = Mul::mul(&*self, other);
    }
}
