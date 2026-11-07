// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::{
    FallingFactorialPolynomial, FallingFactorialPolynomialTruncated,
    RisingFactorialPolynomialTruncated,
};

impl FallingFactorialPolynomial for IntegerPolynomial {
    /// Computes the $n$th falling factorial polynomial, whose coefficients are the Stirling numbers
    /// of the first kind $s(n, k)$.
    ///
    /// $$
    /// x^{\underline{n}} = x(x - 1) \cdots (x - n + 1) = \sum_{k=0}^n s(n, k) x^k.
    /// $$
    ///
    /// $x^{\underline{n}}$ has degree $n$ and leading coefficient 1, and its value at an integer
    /// $m$ is the falling factorial of $m$. Its coefficients are those of the rising factorial
    /// polynomial, with the signs of those with $n + k$ odd reversed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::special::traits::FallingFactorialPolynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::falling_factorial_polynomial(0).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::falling_factorial_polynomial(1).to_string(),
    ///     "x"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::falling_factorial_polynomial(4).to_string(),
    ///     "x^4-6*x^3+11*x^2-6*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_1_vec` from `arith/stirling1.c`, FLINT 3.6.0,
    /// with a length of $n + 1$.
    #[inline]
    fn falling_factorial_polynomial(n: u64) -> Self {
        Self::falling_factorial_polynomial_truncated(n, n + 1)
    }
}

impl FallingFactorialPolynomialTruncated for IntegerPolynomial {
    /// Computes the $n$th falling factorial polynomial truncated to its first `len` coefficients,
    /// which are the Stirling numbers of the first kind $s(n, k)$ for $k < \text{len}$.
    ///
    /// $$
    /// f(n, l) = \sum_{k=0}^{\min(l, n + 1) - 1} s(n, k) x^k.
    /// $$
    ///
    /// A `len` greater than $n$ gives the whole polynomial. The coefficients are those of the
    /// truncated rising factorial polynomial, with the signs of those with $n + k$ odd reversed.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(nl (\log n)^3 \log\log n)$
    ///
    /// $M(n, l) = O(nl \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $l$ is `min(len, n + 1)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::special::traits::FallingFactorialPolynomialTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::falling_factorial_polynomial_truncated(5, 3).to_string(),
    ///     "-50*x^2+24*x"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::falling_factorial_polynomial_truncated(2, 10).to_string(),
    ///     "x^2-x"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::falling_factorial_polynomial_truncated(5, 0).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_1_vec` from `arith/stirling1.c`, FLINT 3.6.0.
    fn falling_factorial_polynomial_truncated(n: u64, len: u64) -> Self {
        let n_even = n.even();
        let coefficients: Vec<Integer> =
            NaturalPolynomial::rising_factorial_polynomial_truncated(n, len)
                .into_coefficients_asc()
                .into_iter()
                .enumerate()
                .map(|(k, c)| {
                    let c = Integer::from(c);
                    if k.even() == n_even { c } else { -c }
                })
                .collect();
        Self::from_coefficients_asc(coefficients)
    }
}
