// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul_truncated::{
    mul_truncated_ref_ref, mul_truncated_val_ref, mul_truncated_val_val,
};
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_mul::mod_reduce_coefficients;
use core::mem::take;
use malachite_base::polynomial::{ModMulTruncated, ModMulTruncatedAssign};

impl ModMulTruncated<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by value, the second by value, and the
    /// modulus by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(self, other: Self, len: u64, m: Natural) -> Self {
        assert_reduced(&self, &other, &m);
        mod_reduce_coefficients(
            mul_truncated_val_val(self.coefficients, other.coefficients, len),
            &m,
        )
    }
}

impl ModMulTruncated<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by value, the second by value, and the
    /// modulus by reference. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(self, other: Self, len: u64, m: &Natural) -> Self {
        assert_reduced(&self, &other, m);
        mod_reduce_coefficients(
            mul_truncated_val_val(self.coefficients, other.coefficients, len),
            m,
        )
    }
}

impl ModMulTruncated<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by value, the second by reference, and the
    /// modulus by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(self, other: &Self, len: u64, m: Natural) -> Self {
        assert_reduced(&self, other, &m);
        mod_reduce_coefficients(
            mul_truncated_val_ref(self.coefficients, &other.coefficients, len),
            &m,
        )
    }
}

impl ModMulTruncated<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by value, the second by reference, and the
    /// modulus by reference. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(self, other: &Self, len: u64, m: &Natural) -> Self {
        assert_reduced(&self, other, m);
        mod_reduce_coefficients(
            mul_truncated_val_ref(self.coefficients, &other.coefficients, len),
            m,
        )
    }
}

impl ModMulTruncated<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by reference, the second by value, and the
    /// modulus by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        assert_reduced(self, &other, &m);
        mod_reduce_coefficients(
            mul_truncated_val_ref(other.coefficients, &self.coefficients, len),
            &m,
        )
    }
}

impl ModMulTruncated<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by reference, the second by value, and the
    /// modulus by reference. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        assert_reduced(self, &other, m);
        mod_reduce_coefficients(
            mul_truncated_val_ref(other.coefficients, &self.coefficients, len),
            m,
        )
    }
}

impl ModMulTruncated<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by reference, the second by reference, and
    /// the modulus by value. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, &m);
        mod_reduce_coefficients(
            mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len),
            &m,
        )
    }
}

impl ModMulTruncated<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first polynomial by reference, the second by reference, and
    /// the modulus by reference. The coefficients of both polynomials must already be reduced
    /// modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(
    ///             &NaturalPolynomial::from_str("x+1").unwrap(),
    ///             2,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, m);
        mod_reduce_coefficients(
            mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len),
            m,
        )
    }
}

impl ModMulTruncatedAssign<Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by value and the modulus by value. The coefficients of both polynomials must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_truncated_assign(
    ///     NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     2,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated_assign(&mut self, other: Self, len: u64, m: Natural) {
        assert_reduced(self, &other, &m);
        *self = mod_reduce_coefficients(
            mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len),
            &m,
        );
    }
}

impl ModMulTruncatedAssign<Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by value and the modulus by reference. The coefficients of both polynomials must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_truncated_assign(
    ///     NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     2,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated_assign(&mut self, other: Self, len: u64, m: &Natural) {
        assert_reduced(self, &other, m);
        *self = mod_reduce_coefficients(
            mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len),
            m,
        );
    }
}

impl ModMulTruncatedAssign<&Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by reference and the modulus by value. The coefficients of both polynomials must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_truncated_assign(
    ///     &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     2,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated_assign(&mut self, other: &Self, len: u64, m: Natural) {
        assert_reduced(self, other, &m);
        *self = mod_reduce_coefficients(
            mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len),
            &m,
        );
    }
}

impl ModMulTruncatedAssign<&Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by reference and the modulus by reference. The coefficients of both polynomials must already
    /// be reduced modulo `m`.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_truncated_assign(
    ///     &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     2,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated_assign(&mut self, other: &Self, len: u64, m: &Natural) {
        assert_reduced(self, other, m);
        *self = mod_reduce_coefficients(
            mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len),
            m,
        );
    }
}
