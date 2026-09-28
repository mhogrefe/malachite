// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_add::{
    add_assign_ref, add_assign_val, assert_reduced,
};
use malachite_base::polynomial::{ModAddTruncated, ModAddTruncatedAssign, Polynomial};

// The first `len` elements of `xs`, or all of them if there are fewer.
fn prefix(xs: &[Natural], len: u64) -> &[Natural] {
    &xs[..usize::try_from(len).map_or(xs.len(), |len| len.min(xs.len()))]
}

fn mod_add_truncated_owned_owned(
    mut p: NaturalPolynomial,
    mut q: NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(&p, &q, m);
    p.truncate_assign(len);
    q.truncate_assign(len);
    add_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
    p
}

fn mod_add_truncated_owned_ref(
    mut p: NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(&p, q, m);
    p.truncate_assign(len);
    add_assign_ref(&mut p.coefficients, prefix(&q.coefficients, len), m);
    p.trim();
    p
}

fn mod_add_truncated_ref_owned(
    p: &NaturalPolynomial,
    mut q: NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(p, &q, m);
    q.truncate_assign(len);
    add_assign_ref(&mut q.coefficients, prefix(&p.coefficients, len), m);
    q.trim();
    q
}

fn mod_add_truncated_ref_ref(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(p, q, m);
    let mut coefficients = prefix(&p.coefficients, len).to_vec();
    add_assign_ref(&mut coefficients, prefix(&q.coefficients, len), m);
    let mut result = NaturalPolynomial { coefficients };
    result.trim();
    result
}

fn mod_add_truncated_assign_owned(
    p: &mut NaturalPolynomial,
    mut q: NaturalPolynomial,
    len: u64,
    m: &Natural,
) {
    assert_reduced(p, &q, m);
    p.truncate_assign(len);
    q.truncate_assign(len);
    add_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
}

fn mod_add_truncated_assign_ref(
    p: &mut NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) {
    assert_reduced(p, q, m);
    p.truncate_assign(len);
    add_assign_ref(&mut p.coefficients, prefix(&q.coefficients, len), m);
    p.trim();
}

impl ModAddTruncated<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by value, the second by value, and the modulus
    /// by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(self, other: Self, len: u64, m: Natural) -> Self {
        mod_add_truncated_owned_owned(self, other, len, &m)
    }
}

impl ModAddTruncated<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by value, the second by value, and the modulus
    /// by reference. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(self, other: Self, len: u64, m: &Natural) -> Self {
        mod_add_truncated_owned_owned(self, other, len, m)
    }
}

impl ModAddTruncated<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by value, the second by reference, and the
    /// modulus by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(self, other: &Self, len: u64, m: Natural) -> Self {
        mod_add_truncated_owned_ref(self, other, len, &m)
    }
}

impl ModAddTruncated<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by value, the second by reference, and the
    /// modulus by reference. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(self, other: &Self, len: u64, m: &Natural) -> Self {
        mod_add_truncated_owned_ref(self, other, len, m)
    }
}

impl ModAddTruncated<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by reference, the second by value, and the
    /// modulus by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_add_truncated_ref_owned(self, other, len, &m)
    }
}

impl ModAddTruncated<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by reference, the second by value, and the
    /// modulus by reference. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_add_truncated_ref_owned(self, other, len, m)
    }
}

impl ModAddTruncated<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by reference, the second by reference, and the
    /// modulus by value. The coefficients of both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_add_truncated_ref_ref(self, other, len, &m)
    }
}

impl ModAddTruncated<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first polynomial by reference, the second by reference, and the
    /// modulus by reference. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             3,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "6*x^2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///             1,
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_add_truncated_ref_ref(self, other, len, m)
    }
}

impl ModAddTruncatedAssign<Self, Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m` in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value and
    /// the modulus by value. The coefficients of both polynomials must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     3,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     1,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated_assign(&mut self, other: Self, len: u64, m: Natural) {
        mod_add_truncated_assign_owned(self, other, len, &m);
    }
}

impl ModAddTruncatedAssign<Self, &Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m` in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value and
    /// the modulus by reference. The coefficients of both polynomials must already be reduced
    /// modulo `m`.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     3,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     1,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated_assign(&mut self, other: Self, len: u64, m: &Natural) {
        mod_add_truncated_assign_owned(self, other, len, m);
    }
}

impl ModAddTruncatedAssign<&Self, Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m` in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by reference
    /// and the modulus by value. The coefficients of both polynomials must already be reduced
    /// modulo `m`.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     3,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     1,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated_assign(&mut self, other: &Self, len: u64, m: Natural) {
        mod_add_truncated_assign_ref(self, other, len, &m);
    }
}

impl ModAddTruncatedAssign<&Self, &Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m` in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by reference
    /// and the modulus by reference. The coefficients of both polynomials must already be reduced
    /// modulo `m`.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. The sum is trimmed, so when
    /// coefficients cancel modulo `m` at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModAddTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear and constant coefficients wrap around to 0.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     3,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_add_truncated_assign(
    ///     &NaturalPolynomial::from_str("4*x^2+6*x+2").unwrap(),
    ///     1,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_add_truncated_assign(&mut self, other: &Self, len: u64, m: &Natural) {
        mod_add_truncated_assign_ref(self, other, len, m);
    }
}
