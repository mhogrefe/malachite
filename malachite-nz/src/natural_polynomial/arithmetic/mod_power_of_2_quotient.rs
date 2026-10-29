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

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_div_rem::{from_words, to_words, trimmed};
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_rem::{
    assert_reduced, mod_power_of_2_quotient_with,
};
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    CheckedModPowerOf2Quotient, ModPowerOf2Quotient, ModPowerOf2QuotientAssign,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_quotient::*;

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, or `None` if `ys` is zero or its leading coefficient is even. Modulo 1, every
// polynomial is 0, and so is the result.
//
// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that `None` is returned where FLINT aborts.
fn mod_power_of_2_quotient_natural(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> Option<NaturalPolynomial> {
    if pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let inverse = ys.last()?.mod_power_of_2_inverse(pow)?;
    if xs.len() < ys.len() {
        return Some(NaturalPolynomial::ZERO);
    }
    Some(trimmed(mod_power_of_2_quotient_with(
        xs,
        ys,
        pow,
        &inverse,
        mod_power_of_2_div_truncated_dispatch,
    )))
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, computed in `T`, which $2^k$ must fit in.
fn mod_power_of_2_quotient_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    Some(from_words(mod_power_of_2_quotient_vec(
        &to_words::<T>(xs),
        &to_words::<T>(ys),
        pow,
    )?))
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, or `None` if the divisor is zero or its leading coefficient is even. When $2^k$
// fits in one or two limbs, this is done in wrapping word arithmetic.
fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    assert_reduced(p, q, pow);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if pow <= Limb::WIDTH {
        mod_power_of_2_quotient_word::<Limb>(xs, ys, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_quotient_word::<DoubleLimb>(xs, ys, pow)
    } else {
        mod_power_of_2_quotient_natural(xs, ys, pow)
    }
}

fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    assert!(
        pow == 0 || *q != NaturalPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, pow).expect("the leading coefficient of the divisor must be odd")
}

impl ModPowerOf2Quotient<Self> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, taking the dividend by value
    /// and the divisor by value. The coefficients of both must already be reduced modulo $2^k$, and
    /// the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_quotient(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_quotient(self, other: Self, pow: u64) -> Self {
        helper(&self, &other, pow)
    }
}

impl ModPowerOf2Quotient<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, taking the dividend by value
    /// and the divisor by reference. The coefficients of both must already be reduced modulo $2^k$,
    /// and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_quotient(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_quotient(self, other: &Self, pow: u64) -> Self {
        helper(&self, other, pow)
    }
}

impl ModPowerOf2Quotient<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, taking the dividend by
    /// reference and the divisor by value. The coefficients of both must already be reduced modulo
    /// $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_quotient(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_quotient(self, other: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        helper(self, &other, pow)
    }
}

impl ModPowerOf2Quotient<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, taking the dividend by
    /// reference and the divisor by reference. The coefficients of both must already be reduced
    /// modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_quotient(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_quotient(self, other: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        helper(self, other, pow)
    }
}

impl ModPowerOf2QuotientAssign<Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its quotient by another modulo $2^k$, taking the
    /// divisor by value. The coefficients of both must already be reduced modulo $2^k$, and the
    /// leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2QuotientAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_quotient_assign(q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_quotient_assign(&mut self, other: Self, pow: u64) {
        *self = helper(self, &other, pow);
    }
}

impl ModPowerOf2QuotientAssign<&Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its quotient by another modulo $2^k$, taking the
    /// divisor by reference. The coefficients of both must already be reduced modulo $2^k$, and the
    /// leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2QuotientAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_quotient_assign(&q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_quotient_assign(&mut self, other: &Self, pow: u64) {
        *self = helper(self, other, pow);
    }
}

impl CheckedModPowerOf2Quotient<Self> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, or `None` if the divisor's
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
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_quotient(q, 3).unwrap().to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_quotient(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_quotient(self, other: Self, pow: u64) -> Option<Self> {
        checked_helper(&self, &other, pow)
    }
}

impl CheckedModPowerOf2Quotient<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, or `None` if the divisor's
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
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_quotient(&q, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_quotient(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_quotient(self, other: &Self, pow: u64) -> Option<Self> {
        checked_helper(&self, other, pow)
    }
}

impl CheckedModPowerOf2Quotient<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, or `None` if the divisor's
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
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_quotient(q, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_quotient(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_quotient(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, pow)
    }
}

impl CheckedModPowerOf2Quotient<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $2^k$, or `None` if the divisor's
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
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2Quotient;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_quotient(&q, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^2+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_quotient(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_quotient(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, pow)
    }
}
