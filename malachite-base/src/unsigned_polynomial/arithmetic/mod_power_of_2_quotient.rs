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
use crate::polynomial::{
    CheckedModPowerOf2Quotient, ModPowerOf2Quotient, ModPowerOf2QuotientAssign,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::{
    assert_reduced, leading_inverse, mod_power_of_2_quotient_with, reduced,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, pow);
    mod_power_of_2_quotient_with_dispatch(&p.coefficients, &q.coefficients, pow)
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, or `None` if `ys` is zero or its leading coefficient is even. Modulo 1, every
// polynomial is 0, and so is the result.
//
// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
// $2^k$, except that `None` is returned where FLINT aborts or assumes the leading coefficient
// invertible.
fn mod_power_of_2_quotient_with_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    if pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    let inverse = leading_inverse(ys, pow)?;
    if xs.len() < ys.len() {
        return Some(UnsignedPolynomial::ZERO);
    }
    Some(reduced(
        mod_power_of_2_quotient_with(xs, ys, pow, inverse, mod_power_of_2_div_truncated_dispatch),
        pow,
    ))
}

/// The coefficients of the quotient modulo $2^k$ of the polynomials with coefficients `xs` and
/// `ys`, without zeros at the end, or `None` if `ys` is empty or its last coefficient is even. The
/// coefficients must be reduced modulo $2^k$, where $k$ is `pow`, which must be no greater than
/// `T::WIDTH`, and neither `xs` nor `ys` may end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_quotient_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> Option<Vec<T>> {
    mod_power_of_2_quotient_with_dispatch(xs, ys, pow).map(|q| q.coefficients)
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert!(
        pow == 0 || *q != UnsignedPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, pow).expect("the leading coefficient of the divisor must be odd")
}

impl<T: PrimitiveUnsigned> ModPowerOf2Quotient<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, taking the dividend by value
    /// and the divisor by value. The coefficients of both must already be reduced modulo $2^k$, and
    /// the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_quotient(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_power_of_2_quotient(self, other: Self, pow: u64) -> Self {
        helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Quotient<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, taking the dividend by value
    /// and the divisor by reference. The coefficients of both must already be reduced modulo $2^k$,
    /// and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_quotient(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_power_of_2_quotient(self, other: &Self, pow: u64) -> Self {
        helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Quotient<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, taking the dividend by
    /// reference and the divisor by value. The coefficients of both must already be reduced modulo
    /// $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_quotient(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_power_of_2_quotient(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Quotient<&UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, taking the dividend by
    /// reference and the divisor by reference. The coefficients of both must already be reduced
    /// modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_quotient(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_power_of_2_quotient(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        helper(self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2QuotientAssign<Self> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its quotient by another modulo $2^k$, taking the
    /// divisor by value. The coefficients of both must already be reduced modulo $2^k$, and the
    /// leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2QuotientAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_quotient_assign(q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_power_of_2_quotient_assign(&mut self, other: Self, pow: u64) {
        *self = helper(self, &other, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2QuotientAssign<&Self> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its quotient by another modulo $2^k$, taking the
    /// divisor by reference. The coefficients of both must already be reduced modulo $2^k$, and the
    /// leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2QuotientAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_quotient_assign(&q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_power_of_2_quotient_assign(&mut self, other: &Self, pow: u64) {
        *self = helper(self, other, pow);
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Quotient<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, or `None` if the divisor's
    /// leading coefficient is not odd, taking the dividend by value and the divisor by value. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient is unique. When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_quotient(q, 3).unwrap().to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_quotient(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that `None` is returned where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_power_of_2_quotient(self, other: Self, pow: u64) -> Option<Self> {
        checked_helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Quotient<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, or `None` if the divisor's
    /// leading coefficient is not odd, taking the dividend by value and the divisor by reference.
    /// The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient is unique. When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_quotient(&q, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_quotient(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that `None` is returned where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_power_of_2_quotient(self, other: &Self, pow: u64) -> Option<Self> {
        checked_helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Quotient<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, or `None` if the divisor's
    /// leading coefficient is not odd, taking the dividend by reference and the divisor by value.
    /// The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient is unique. When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_quotient(q, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_quotient(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that `None` is returned where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_power_of_2_quotient(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Quotient<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Finds the quotient of two [`UnsignedPolynomial`]s modulo $2^k$, or `None` if the divisor's
    /// leading coefficient is not odd, taking the dividend by reference and the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient is unique. When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_quotient(&q, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_quotient(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div` from `nmod_poly/div.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that `None` is returned where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_power_of_2_quotient(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, pow)
    }
}
