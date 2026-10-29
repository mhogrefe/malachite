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
use crate::polynomial::{CheckedModQuotient, ModQuotient, ModQuotientAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, leading_inverse, mod_quotient_with,
};
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, m);
    mod_quotient_with_dispatch(&p.coefficients, &q.coefficients, m)
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, or `None` if
// `ys` is zero or its leading coefficient is not invertible. Modulo 1, every polynomial is 0, and
// so is the result.
//
// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that `None` is
// returned where FLINT aborts or assumes the leading coefficient invertible.
fn mod_quotient_with_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    if m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let inverse = leading_inverse(ys, m)?;
    if xs.len() < ys.len() {
        return Some(UnsignedPolynomial::ZERO);
    }
    Some(from_coefficients_trimmed(mod_quotient_with(
        xs,
        ys,
        m,
        inverse,
        mod_div_truncated_dispatch,
    )))
}

/// The coefficients of the quotient modulo `m` of the polynomials with coefficients `xs` and `ys`,
/// without zeros at the end, or `None` if `ys` is empty or its last coefficient is not invertible.
/// The coefficients must be reduced modulo `m`, `m` must be positive, and neither `xs` nor `ys` may
/// end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
#[inline]
pub fn mod_quotient_vec<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Option<Vec<T>> {
    mod_quotient_with_dispatch(xs, ys, m).map(|q| q.coefficients)
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

impl<T: PrimitiveUnsigned> ModQuotient<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by value
    /// and the divisor by value. The coefficients of both must already be reduced modulo $m$, and
    /// the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::ModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_quotient(q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_quotient(self, other: Self, m: T) -> Self {
        helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModQuotient<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by value
    /// and the divisor by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::ModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_quotient(&q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_quotient(self, other: &Self, m: T) -> Self {
        helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModQuotient<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by
    /// reference and the divisor by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::ModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_quotient(q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_quotient(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModQuotient<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, taking the dividend by
    /// reference and the divisor by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::ModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_quotient(&q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_quotient(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModQuotientAssign<Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its quotient by another modulo $m$, taking the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$, and the
    /// leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::ModQuotientAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// p.mod_quotient_assign(q, 7);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_quotient_assign(&mut self, other: Self, m: T) {
        *self = helper(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModQuotientAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its quotient by another modulo $m$, taking the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$, and the
    /// leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::ModQuotientAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// p.mod_quotient_assign(&q, 7);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_quotient_assign(&mut self, other: &Self, m: T) {
        *self = helper(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> CheckedModQuotient<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value and the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_quotient(q, 7).unwrap().to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_quotient(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_quotient(self, other: Self, m: T) -> Option<Self> {
        checked_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModQuotient<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_quotient(&q, 7).unwrap().to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_quotient(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_quotient(self, other: &Self, m: T) -> Option<Self> {
        checked_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModQuotient<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference and the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_quotient(q, 7).unwrap().to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_quotient(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_quotient(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModQuotient<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
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
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_quotient(&q, 7).unwrap().to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_quotient(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_quotient(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, m)
    }
}
