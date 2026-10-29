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
use crate::natural_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, from_words, mod_quotient_with, to_words, trimmed,
};
use crate::natural_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::ModInverse;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{CheckedModQuotient, ModQuotient, ModQuotientAssign};
use malachite_base::unsigned_polynomial::arithmetic::mod_quotient::mod_quotient_vec;

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, or `None` if
// `ys` is zero or its leading coefficient is not invertible. Modulo 1, every polynomial is 0, and
// so is the result.
//
// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except that
// `None` is returned where FLINT aborts.
fn mod_quotient_natural(xs: &[Natural], ys: &[Natural], m: &Natural) -> Option<NaturalPolynomial> {
    if *m == 1u32 {
        return Some(NaturalPolynomial::ZERO);
    }
    let inverse = ys.last()?.mod_inverse(m)?;
    if xs.len() < ys.len() {
        return Some(NaturalPolynomial::ZERO);
    }
    Some(trimmed(mod_quotient_with(
        xs,
        ys,
        m,
        &inverse,
        mod_div_truncated_dispatch,
    )))
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, computed in
// `T`, which `m` must fit in.
fn mod_quotient_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    Some(from_words(mod_quotient_vec(
        &to_words::<T>(xs),
        &to_words::<T>(ys),
        T::exact_from(m),
    )?))
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, or `None` if
// the divisor is zero or its leading coefficient is not invertible. When `m` fits in one or two
// limbs, this is done in word arithmetic.
fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    assert_reduced(p, q, m);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_quotient_word::<Limb>(xs, ys, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_quotient_word::<DoubleLimb>(xs, ys, m)
    } else {
        mod_quotient_natural(xs, ys, m)
    }
}

fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert!(
        *m == 1u32 || *q != NaturalPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, m)
        .expect("the leading coefficient of the divisor must be invertible modulo m")
}

impl ModQuotient<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by value, and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient(q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: Self, m: Natural) -> Self {
        helper(&self, &other, &m)
    }
}

impl ModQuotient<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by value, and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient(q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: Self, m: &Natural) -> Self {
        helper(&self, &other, m)
    }
}

impl ModQuotient<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by reference, and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient(&q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: &Self, m: Natural) -> Self {
        helper(&self, other, &m)
    }
}

impl ModQuotient<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by reference, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient(&q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: &Self, m: &Natural) -> Self {
        helper(&self, other, m)
    }
}

impl ModQuotient<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by value, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient(q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        helper(self, &other, &m)
    }
}

impl ModQuotient<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by value, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient(q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        helper(self, &other, m)
    }
}

impl ModQuotient<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by reference, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient(&q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        helper(self, other, &m)
    }
}

impl ModQuotient<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by reference, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient(&q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        helper(self, other, m)
    }
}

impl ModQuotientAssign<Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its quotient by another modulo $m$, taking the divisor
    /// by value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_quotient_assign(q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient_assign(&mut self, other: Self, m: Natural) {
        *self = helper(self, &other, &m);
    }
}

impl ModQuotientAssign<Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its quotient by another modulo $m$, taking the divisor
    /// by value, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_quotient_assign(q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient_assign(&mut self, other: Self, m: &Natural) {
        *self = helper(self, &other, m);
    }
}

impl ModQuotientAssign<&Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its quotient by another modulo $m$, taking the divisor
    /// by reference, and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_quotient_assign(&q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient_assign(&mut self, other: &Self, m: Natural) {
        *self = helper(self, other, &m);
    }
}

impl ModQuotientAssign<&Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its quotient by another modulo $m$, taking the divisor
    /// by reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_quotient_assign(&q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_quotient_assign(&mut self, other: &Self, m: &Natural) {
        *self = helper(self, other, m);
    }
}

impl CheckedModQuotient<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_quotient(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_quotient(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(self, other: Self, m: Natural) -> Option<Self> {
        checked_helper(&self, &other, &m)
    }
}

impl CheckedModQuotient<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by value, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_quotient(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_quotient(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(self, other: Self, m: &Natural) -> Option<Self> {
        checked_helper(&self, &other, m)
    }
}

impl CheckedModQuotient<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by reference, and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_quotient(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_quotient(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(self, other: &Self, m: Natural) -> Option<Self> {
        checked_helper(&self, other, &m)
    }
}

impl CheckedModQuotient<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_quotient(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_quotient(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(self, other: &Self, m: &Natural) -> Option<Self> {
        checked_helper(&self, other, m)
    }
}

impl CheckedModQuotient<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by value, and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_quotient(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_quotient(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(
        self,
        other: NaturalPolynomial,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, &m)
    }
}

impl CheckedModQuotient<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by value, and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_quotient(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_quotient(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(
        self,
        other: NaturalPolynomial,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, m)
    }
}

impl CheckedModQuotient<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by reference, and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_quotient(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_quotient(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(
        self,
        other: &NaturalPolynomial,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, &m)
    }
}

impl CheckedModQuotient<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the quotient of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by reference, and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModQuotient;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_quotient(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     (&p).checked_mod_quotient(&q, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div` from `fmpz_mod_poly/div.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_quotient(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, m)
    }
}
