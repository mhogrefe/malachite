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
use crate::natural_polynomial::arithmetic::mod_div_rem::checked_helper as div_rem_checked_helper;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{CheckedModRem, ModRem, ModRemAssign};

fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    div_rem_checked_helper(p, q, m).map(|(_, r)| r)
}

fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert!(
        *m == 1u32 || *q != NaturalPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, m)
        .expect("the leading coefficient of the divisor must be invertible modulo m")
}

impl ModRem<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by value, and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_rem(q, Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: Self, m: Natural) -> Self {
        helper(&self, &other, &m)
    }
}

impl ModRem<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by value, and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_rem(q, &Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: Self, m: &Natural) -> Self {
        helper(&self, &other, m)
    }
}

impl ModRem<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by reference, and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_rem(&q, Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: &Self, m: Natural) -> Self {
        helper(&self, other, &m)
    }
}

impl ModRem<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by value,
    /// the divisor by reference, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_rem(&q, &Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: &Self, m: &Natural) -> Self {
        helper(&self, other, m)
    }
}

impl ModRem<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by value, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_rem(q, Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        helper(self, &other, &m)
    }
}

impl ModRem<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by value, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_rem(q, &Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        helper(self, &other, m)
    }
}

impl ModRem<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by reference, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_rem(&q, Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        helper(self, other, &m)
    }
}

impl ModRem<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, taking the dividend by
    /// reference, the divisor by reference, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_rem(&q, &Natural::from(7u32)).to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        helper(self, other, m)
    }
}

impl ModRemAssign<Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its rem by another modulo $m$, taking the divisor by
    /// value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRemAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_rem_assign(q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem_assign(&mut self, other: Self, m: Natural) {
        *self = helper(self, &other, &m);
    }
}

impl ModRemAssign<Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its rem by another modulo $m$, taking the divisor by
    /// value, and the modulus by reference. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRemAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_rem_assign(q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem_assign(&mut self, other: Self, m: &Natural) {
        *self = helper(self, &other, m);
    }
}

impl ModRemAssign<&Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its rem by another modulo $m$, taking the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRemAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_rem_assign(&q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem_assign(&mut self, other: &Self, m: Natural) {
        *self = helper(self, other, &m);
    }
}

impl ModRemAssign<&Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its rem by another modulo $m$, taking the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModRemAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_rem_assign(&q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_rem_assign(&mut self, other: &Self, m: &Natural) {
        *self = helper(self, other, m);
    }
}

impl CheckedModRem<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_rem(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_rem(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: Self, m: Natural) -> Option<Self> {
        checked_helper(&self, &other, &m)
    }
}

impl CheckedModRem<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by value, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_rem(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_rem(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: Self, m: &Natural) -> Option<Self> {
        checked_helper(&self, &other, m)
    }
}

impl CheckedModRem<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by reference, and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_rem(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_rem(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: &Self, m: Natural) -> Option<Self> {
        checked_helper(&self, other, &m)
    }
}

impl CheckedModRem<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by value, the divisor
    /// by reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_rem(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_rem(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: &Self, m: &Natural) -> Option<Self> {
        checked_helper(&self, other, m)
    }
}

impl CheckedModRem<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by value, and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_rem(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_rem(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: NaturalPolynomial, m: Natural) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, &m)
    }
}

impl CheckedModRem<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by value, and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_rem(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_rem(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, m)
    }
}

impl CheckedModRem<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by reference, and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_rem(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_rem(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: &NaturalPolynomial, m: Natural) -> Option<NaturalPolynomial> {
        checked_helper(self, other, &m)
    }
}

impl CheckedModRem<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the remainder of two [`NaturalPolynomial`]s modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$, taking the dividend by reference, the
    /// divisor by reference, and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the remainder is unique. When $m$ is 1, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::CheckedModRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_rem(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "3*x+6"
    /// );
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_rem(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_rem` from `fmpz_mod_poly/rem.c`, FLINT 3.6.0, except
    /// that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_rem(self, other: &NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
        checked_helper(self, other, m)
    }
}
