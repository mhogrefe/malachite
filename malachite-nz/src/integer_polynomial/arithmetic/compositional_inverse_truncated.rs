// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::compose_truncated::compose_truncated_ref;
use crate::integer_polynomial::arithmetic::div_truncated::div_truncated_dispatch;
use crate::integer_polynomial::arithmetic::inverse_truncated::inverse_truncated_dispatch;
use crate::integer_polynomial::arithmetic::mul_truncated::mul_truncated_ref_ref;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMulAssign, DivExact};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_compositional_inverse_truncated::*;

fn padded(mut xs: Vec<Integer>, len: usize) -> Vec<Integer> {
    xs.resize(len, Integer::ZERO);
    xs
}

// The ring of integers. Only 1 and -1 are invertible.
pub(crate) struct IntegerRing;

impl SeriesReversionRing for IntegerRing {
    type C = Integer;

    #[inline]
    fn zero(&self) -> Integer {
        Integer::ZERO
    }

    #[inline]
    fn is_zero(&self, x: &Integer) -> bool {
        *x == 0u32
    }

    #[inline]
    fn add(&self, x: &Integer, y: &Integer) -> Integer {
        x + y
    }

    #[inline]
    fn sub(&self, x: &Integer, y: &Integer) -> Integer {
        x - y
    }

    #[inline]
    fn mul(&self, x: &Integer, y: &Integer) -> Integer {
        x * y
    }

    fn inverse(&self, x: &Integer) -> Option<Integer> {
        if x.unsigned_abs_ref() == &1u32 {
            Some(x.clone())
        } else {
            None
        }
    }

    // Since the coefficient of $x$ is 1 or -1, the inverse has integer coefficients, so Lagrange
    // inversion's divisions are always exact.
    #[inline]
    fn div_by_index(&self, x: &Integer, i: u64) -> Option<Integer> {
        Some(x.div_exact(Integer::from(i)))
    }

    #[inline]
    fn mul_by_index(&self, x: &Integer, i: u64) -> Integer {
        x * Integer::from(i)
    }

    #[inline]
    fn mul_truncated(&self, xs: &[Integer], ys: &[Integer], len: usize) -> Vec<Integer> {
        padded(mul_truncated_ref_ref(xs, ys, u64::exact_from(len)), len)
    }

    #[inline]
    fn inverse_truncated(&self, xs: &[Integer], len: usize) -> Vec<Integer> {
        padded(inverse_truncated_dispatch(xs, len), len)
    }

    #[inline]
    fn div_truncated(&self, xs: &[Integer], ys: &[Integer], len: usize) -> Vec<Integer> {
        padded(div_truncated_dispatch(xs, ys, len).unwrap(), len)
    }

    #[inline]
    fn compose_truncated(&self, xs: &[Integer], ys: &[Integer], len: usize) -> Vec<Integer> {
        padded(
            compose_truncated_ref(xs, ys, u64::exact_from(len)).coefficients,
            len,
        )
    }

    // The products are accumulated in place.
    fn dot_rev(&self, xs: &[Integer], ys: &[Integer], n: usize) -> Integer {
        let mut sum = Integer::ZERO;
        for (x, y) in xs[..n].iter().zip(ys[..n].iter().rev()) {
            sum.add_mul_assign(x, y);
        }
        sum
    }
}

// The compositional inverse of `p` to `len` terms, or `None` if its constant coefficient is not 0
// or its coefficient of $x$ is not 1 or -1.
//
// This is equivalent to `fmpz_poly_revert_series` from `fmpz_poly/revert_series.c`, FLINT 3.6.0,
// except that `None` is returned where FLINT throws.
pub(crate) fn compositional_inverse_truncated_integer_helper(
    p: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    compositional_inverse_truncated_helper(
        &IntegerRing,
        p.coefficients_asc(),
        usize::exact_from(len),
    )
    .map(IntegerPolynomial::from_coefficients_asc)
}

impl CompositionalInverseTruncated for IntegerPolynomial {
    type Output = Self;

    /// Finds the compositional inverse of an [`IntegerPolynomial`] as a power series, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking it by value.
    ///
    /// $$
    /// f(p, n) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is 1 or $-1$,
    /// which is when the inverse has integer coefficients; otherwise the inverse exists and is
    /// unique. When the inverse exists and `len` is less than 2, the result is the zero polynomial.
    ///
    /// The inverse is computed by the fast Lagrange algorithm, in which the coefficient of $x^i$ is
    /// that of $x^{i-1}$ in $(x/p)^i$, divided by $i$, a division that is always exact; when `len`
    /// is at least 2500, Newton's iteration is used instead, in which each step doubles the number
    /// of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^{5/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is the number of bits of
    /// the largest coefficient of `self` plus the number of bits of `len`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CompositionalInverseTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .compositional_inverse_truncated(6)
    ///         .unwrap()
    ///         .to_string(),
    ///     "14*x^5-5*x^4+2*x^3-x^2+x"
    /// );
    /// // The coefficient of x is not 1 or -1.
    /// assert!(
    ///     (IntegerPolynomial::from_str("x^2+2*x").unwrap())
    ///         .compositional_inverse_truncated(5)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_revert_series` from `fmpz_poly/revert_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT throws, and that the inverse modulo $x^0$
    /// or $x^1$ is 0.
    #[inline]
    fn compositional_inverse_truncated(self, len: u64) -> Option<Self> {
        compositional_inverse_truncated_integer_helper(&self, len)
    }
}

impl CompositionalInverseTruncated for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the compositional inverse of an [`IntegerPolynomial`] as a power series, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking it by reference.
    ///
    /// $$
    /// f(p, n) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is 1 or $-1$,
    /// which is when the inverse has integer coefficients; otherwise the inverse exists and is
    /// unique. When the inverse exists and `len` is less than 2, the result is the zero polynomial.
    ///
    /// The inverse is computed by the fast Lagrange algorithm, in which the coefficient of $x^i$ is
    /// that of $x^{i-1}$ in $(x/p)^i$, divided by $i$, a division that is always exact; when `len`
    /// is at least 2500, Newton's iteration is used instead, in which each step doubles the number
    /// of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^{5/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is the number of bits of
    /// the largest coefficient of `self` plus the number of bits of `len`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CompositionalInverseTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .compositional_inverse_truncated(6)
    ///         .unwrap()
    ///         .to_string(),
    ///     "14*x^5-5*x^4+2*x^3-x^2+x"
    /// );
    /// // The coefficient of x is not 1 or -1.
    /// assert!(
    ///     (&IntegerPolynomial::from_str("x^2+2*x").unwrap())
    ///         .compositional_inverse_truncated(5)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_revert_series` from `fmpz_poly/revert_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT throws, and that the inverse modulo $x^0$
    /// or $x^1$ is 0.
    #[inline]
    fn compositional_inverse_truncated(self, len: u64) -> Option<IntegerPolynomial> {
        compositional_inverse_truncated_integer_helper(self, len)
    }
}
