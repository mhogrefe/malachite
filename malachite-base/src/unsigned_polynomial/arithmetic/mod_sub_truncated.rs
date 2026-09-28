// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModSubTruncated, ModSubTruncatedAssign, Polynomial};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_sub::{
    rsub_assign_ref, sub_assign_ref, sub_assign_val,
};

// The first `len` elements of `xs`, or all of them if there are fewer.
fn prefix<T: PrimitiveUnsigned>(xs: &[T], len: u64) -> &[T] {
    &xs[..usize::try_from(len).map_or(xs.len(), |len| len.min(xs.len()))]
}

impl<T: PrimitiveUnsigned> ModSubTruncated<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking both by value. The coefficients of both must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p - q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. Where the second polynomial
    /// has more of those, they are negated modulo `m`. The difference is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSubTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The quadratic and linear coefficients wrap around.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_sub_truncated(UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 3, 7)
    ///         .to_string(),
    ///     "5*x^2+2*x+3"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_sub_truncated(UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 1, 7)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub_series` from `nmod_poly/sub_series.c`, FLINT 3.6.0.
    fn mod_sub_truncated(mut self, mut other: Self, len: u64, m: T) -> Self {
        assert_reduced(&self, &other, m);
        self.truncate_assign(len);
        other.truncate_assign(len);
        sub_assign_val(&mut self.coefficients, other.coefficients, m);
        self.trim();
        self
    }
}

impl<T: PrimitiveUnsigned> ModSubTruncated<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the first by value and the second by reference. The
    /// coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p - q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. Where the second polynomial
    /// has more of those, they are negated modulo `m`. The difference is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSubTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The quadratic and linear coefficients wrap around.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_sub_truncated(&UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 3, 7)
    ///         .to_string(),
    ///     "5*x^2+2*x+3"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_sub_truncated(&UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 1, 7)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub_series` from `nmod_poly/sub_series.c`, FLINT 3.6.0.
    fn mod_sub_truncated(mut self, other: &Self, len: u64, m: T) -> Self {
        assert_reduced(&self, other, m);
        self.truncate_assign(len);
        sub_assign_ref(&mut self.coefficients, prefix(&other.coefficients, len), m);
        self.trim();
        self
    }
}

impl<T: PrimitiveUnsigned> ModSubTruncated<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the first by reference and the second by value. The
    /// coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p - q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. Where the second polynomial
    /// has more of those, they are negated modulo `m`. The difference is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSubTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The quadratic and linear coefficients wrap around.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_sub_truncated(UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 3, 7)
    ///         .to_string(),
    ///     "5*x^2+2*x+3"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_sub_truncated(UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 1, 7)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub_series` from `nmod_poly/sub_series.c`, FLINT 3.6.0.
    fn mod_sub_truncated(
        self,
        mut other: UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, m);
        other.truncate_assign(len);
        rsub_assign_ref(&mut other.coefficients, prefix(&self.coefficients, len), m);
        other.trim();
        other
    }
}

impl<T: PrimitiveUnsigned> ModSubTruncated<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking both by reference. The coefficients of both must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p - q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. Where the second polynomial
    /// has more of those, they are negated modulo `m`. The difference is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSubTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The quadratic and linear coefficients wrap around.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_sub_truncated(&UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 3, 7)
    ///         .to_string(),
    ///     "5*x^2+2*x+3"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_sub_truncated(&UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 1, 7)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub_series` from `nmod_poly/sub_series.c`, FLINT 3.6.0.
    fn mod_sub_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, m);
        let mut coefficients = prefix(&self.coefficients, len).to_vec();
        sub_assign_ref(&mut coefficients, prefix(&other.coefficients, len), m);
        let mut result = UnsignedPolynomial { coefficients };
        result.trim();
        result
    }
}

impl<T: PrimitiveUnsigned> ModSubTruncatedAssign<Self, T> for UnsignedPolynomial<T> {
    /// Subtracts an [`UnsignedPolynomial`] from an [`UnsignedPolynomial`] modulo `m` in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial
    /// by value. The coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets ((p - q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. Where the second polynomial
    /// has more of those, they are negated modulo `m`. The difference is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSubTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The quadratic and linear coefficients wrap around.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_sub_truncated_assign(UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 3, 7);
    /// assert_eq!(p.to_string(), "5*x^2+2*x+3");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_sub_truncated_assign(UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 1, 7);
    /// assert_eq!(p.to_string(), "3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub_series` from `nmod_poly/sub_series.c`, FLINT 3.6.0.
    fn mod_sub_truncated_assign(&mut self, mut other: Self, len: u64, m: T) {
        assert_reduced(self, &other, m);
        self.truncate_assign(len);
        other.truncate_assign(len);
        sub_assign_val(&mut self.coefficients, other.coefficients, m);
        self.trim();
    }
}

impl<T: PrimitiveUnsigned> ModSubTruncatedAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Subtracts an [`UnsignedPolynomial`] from an [`UnsignedPolynomial`] modulo `m` in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial
    /// by reference. The coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets ((p - q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. Where the second polynomial
    /// has more of those, they are negated modulo `m`. The difference is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSubTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The quadratic and linear coefficients wrap around.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_sub_truncated_assign(&UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 3, 7);
    /// assert_eq!(p.to_string(), "5*x^2+2*x+3");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_sub_truncated_assign(&UnsignedPolynomial::from_str("4*x^2+6*x+2").unwrap(), 1, 7);
    /// assert_eq!(p.to_string(), "3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub_series` from `nmod_poly/sub_series.c`, FLINT 3.6.0.
    fn mod_sub_truncated_assign(&mut self, other: &Self, len: u64, m: T) {
        assert_reduced(self, other, m);
        self.truncate_assign(len);
        sub_assign_ref(&mut self.coefficients, prefix(&other.coefficients, len), m);
        self.trim();
    }
}
