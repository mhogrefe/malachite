// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{CheckedModRem, ModRem, ModRemAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::{assert_reduced, mod_div_rem_with};
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, m);
    mod_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        m,
        mod_div_truncated_dispatch,
    )
    .map(|(_, r)| r)
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert!(
        m == T::ONE || *q != UnsignedPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, m)
        .expect("the leading coefficient of the divisor must be invertible modulo m")
}

impl<T: PrimitiveUnsigned> ModRem<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by value
    /// and the divisor by value. The coefficients of both must already be reduced modulo $m$, and
    /// the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_rem(q, 7).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_rem(self, other: Self, m: T) -> Self {
        helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRem<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by value
    /// and the divisor by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_rem(&q, 7).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_rem(self, other: &Self, m: T) -> Self {
        helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRem<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by
    /// reference and the divisor by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_rem(q, 7).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_rem(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRem<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by
    /// reference and the divisor by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_rem(&q, 7).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_rem(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRemAssign<Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its rem by another modulo $m$, taking the divisor by
    /// value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// p.mod_rem_assign(q, 7);
    /// assert_eq!(p.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_rem_assign(&mut self, other: Self, m: T) {
        *self = helper(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModRemAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its rem by another modulo $m$, taking the divisor by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// p.mod_rem_assign(&q, 7);
    /// assert_eq!(p.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_rem_assign(&mut self, other: &Self, m: T) {
        *self = helper(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> CheckedModRem<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value and the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.checked_mod_rem(q, 7).unwrap().to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_rem(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_rem(self, other: Self, m: T) -> Option<Self> {
        checked_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModRem<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.checked_mod_rem(&q, 7).unwrap().to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_rem(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_rem(self, other: &Self, m: T) -> Option<Self> {
        checked_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModRem<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference and the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).checked_mod_rem(q, 7).unwrap().to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_rem(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_rem(self, other: UnsignedPolynomial<T>, m: T) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModRem<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the remainder of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).checked_mod_rem(&q, 7).unwrap().to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_rem(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_rem` from `nmod_poly/rem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_rem(self, other: &UnsignedPolynomial<T>, m: T) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, m)
    }
}
