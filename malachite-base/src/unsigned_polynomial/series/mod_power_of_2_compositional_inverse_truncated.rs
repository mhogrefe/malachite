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

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::polynomial::Polynomial;
use crate::polynomial::series::traits::ModPowerOf2CompositionalInverseTruncated;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::series::mod_compositional_inverse_truncated::*;
use crate::unsigned_polynomial::series::mod_power_of_2_compose_truncated::*;
use crate::unsigned_polynomial::series::mod_power_of_2_div_truncated::*;
use crate::unsigned_polynomial::series::mod_power_of_2_inverse_truncated::*;
use crate::unsigned_polynomial::series::mod_power_of_2_mul_truncated::*;
use alloc::vec::Vec;
use core::marker::PhantomData;

fn padded<T: PrimitiveUnsigned>(mut xs: Vec<T>, len: usize) -> Vec<T> {
    xs.resize(len, T::ZERO);
    xs
}

// The ring of coefficients modulo $2^k$.
pub(crate) struct ModPowerOf2Ring<T: PrimitiveUnsigned> {
    pub(crate) pow: u64,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: PrimitiveUnsigned> SeriesReversionRing for ModPowerOf2Ring<T> {
    type C = T;

    #[inline]
    fn zero(&self) -> T {
        T::ZERO
    }

    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_add(*y, self.pow)
    }

    #[inline]
    fn sub(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_sub(*y, self.pow)
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_mul(*y, self.pow)
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        if x.even() {
            None
        } else {
            x.mod_power_of_2_inverse(self.pow)
        }
    }

    fn div_by_index(&self, x: &T, i: u64) -> Option<T> {
        let inverse = self.inverse(&T::wrapping_from(i).mod_power_of_2(self.pow))?;
        Some(x.mod_power_of_2_mul(inverse, self.pow))
    }

    #[inline]
    fn mul_by_index(&self, x: &T, i: u64) -> T {
        x.mod_power_of_2_mul(T::wrapping_from(i).mod_power_of_2(self.pow), self.pow)
    }

    fn mul_truncated(&self, xs: &[T], ys: &[T], len: usize) -> Vec<T> {
        let mut out = vec![T::ZERO; len];
        if len != 0 && !xs.is_empty() && !ys.is_empty() {
            mod_power_of_2_mul_truncated_to_out(&mut out, xs, ys, self.pow);
        }
        out
    }

    #[inline]
    fn inverse_truncated(&self, xs: &[T], len: usize) -> Vec<T> {
        padded(
            mod_power_of_2_inverse_truncated_vec(xs, u64::exact_from(len), self.pow).unwrap(),
            len,
        )
    }

    #[inline]
    fn div_truncated(&self, xs: &[T], ys: &[T], len: usize) -> Vec<T> {
        padded(
            mod_power_of_2_div_truncated_vec(xs, ys, u64::exact_from(len), self.pow).unwrap(),
            len,
        )
    }

    #[inline]
    fn compose_truncated(&self, xs: &[T], ys: &[T], len: usize) -> Vec<T> {
        padded(
            mod_power_of_2_compose_truncated_vec(xs, ys, u64::exact_from(len), self.pow),
            len,
        )
    }

    // The products and sums wrap, and only the result is reduced, since $2^k$ divides
    // $2^{\mathrm{W}}$.
    #[inline]
    fn dot_rev(&self, xs: &[T], ys: &[T], n: usize) -> T {
        xs[..n]
            .iter()
            .zip(ys[..n].iter().rev())
            .fold(T::ZERO, |sum, (&x, &y)| sum.wrapping_add_mul(x, y))
            .mod_power_of_2(self.pow)
    }

    // 2 is not invertible, so a division by an index is possible only when there is none.
    #[inline]
    fn lagrange_possible(&self, n: usize) -> bool {
        n <= 2
    }
}

// The compositional inverse of `p` modulo $2^k$ to `len` terms.
//
// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT 3.6.0,
// with the modulus $2^k$, except that `None` is returned where FLINT throws.
pub(crate) fn mod_power_of_2_compositional_inverse_truncated_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        // Every polynomial is 0 modulo 1, and so is its reversion.
        return Some(UnsignedPolynomial::ZERO);
    }
    compositional_inverse_truncated_helper(
        &ModPowerOf2Ring {
            pow,
            phantom: PhantomData,
        },
        p.coefficients_asc(),
        usize::exact_from(len),
    )
    .map(UnsignedPolynomial::from_coefficients_asc)
}

impl<T: PrimitiveUnsigned> ModPowerOf2CompositionalInverseTruncated for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the compositional inverse of an [`UnsignedPolynomial`] as a power series modulo $2^k$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking it by value. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is odd;
    /// otherwise the inverse exists and is unique. When the inverse exists and `len` is less than
    /// 2, the result is the zero polynomial, and when $k$ is 0 the result is always `Some` of the
    /// zero polynomial.
    ///
    /// The Lagrange algorithms divide by every index less than `len`, which is not possible modulo
    /// $2^k$ once 2 is among them, so the inverse is computed by Newton's iteration, in which each
    /// step doubles the number of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModPowerOf2CompositionalInverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(6, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^5+3*x^4+2*x^3+7*x^2+x"
    /// );
    /// // The coefficient of x is even.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(5, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT throws or fails,
    /// that the inverse modulo $x^0$ or $x^1$, or modulo 1, is 0, and that Newton's iteration is
    /// used at once rather than after the fast Lagrange algorithm fails.
    #[inline]
    fn mod_power_of_2_compositional_inverse_truncated(self, len: u64, pow: u64) -> Option<Self> {
        mod_power_of_2_compositional_inverse_truncated_helper(&self, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CompositionalInverseTruncated for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the compositional inverse of an [`UnsignedPolynomial`] as a power series modulo $2^k$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking it by reference. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is odd;
    /// otherwise the inverse exists and is unique. When the inverse exists and `len` is less than
    /// 2, the result is the zero polynomial, and when $k$ is 0 the result is always `Some` of the
    /// zero polynomial.
    ///
    /// The Lagrange algorithms divide by every index less than `len`, which is not possible modulo
    /// $2^k$ once 2 is among them, so the inverse is computed by Newton's iteration, in which each
    /// step doubles the number of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModPowerOf2CompositionalInverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(6, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^5+3*x^4+2*x^3+7*x^2+x"
    /// );
    /// // The coefficient of x is even.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(5, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT throws or fails,
    /// that the inverse modulo $x^0$ or $x^1$, or modulo 1, is 0, and that Newton's iteration is
    /// used at once rather than after the fast Lagrange algorithm fails.
    #[inline]
    fn mod_power_of_2_compositional_inverse_truncated(
        self,
        len: u64,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        mod_power_of_2_compositional_inverse_truncated_helper(self, len, pow)
    }
}
