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
use crate::natural_polynomial::arithmetic::mod_mul::{
    limbs_to_polynomial, mod_reduce_coefficients, naturals_to_limbs, word_preferred,
};
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::truncated_len;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::take;
use malachite_base::num::arithmetic::traits::ModAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModMulTruncated, ModMulTruncatedAssign};
use malachite_base::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;

// For each modulus size, the lengths for which the word kernels beat the full truncated product, as
// with `MUL_WORD_WINDOWS`.
pub(crate) const MUL_TRUNCATED_WORD_WINDOWS: [(u64, usize); 4] =
    [(24, 64), (40, 192), (60, 320), (64, 192)];

// Whether the word kernels are used for a truncated product of factors of lengths `len1` and `len2`
// modulo `m`: when neither is a constant and the shorter, after truncation to `len` coefficients,
// is in the window.
fn mul_truncated_word_preferred(len1: usize, len2: usize, len: u64, m: &Natural) -> bool {
    len != 0
        && len1 > 1
        && len2 > 1
        && word_preferred(
            &MUL_TRUNCATED_WORD_WINDOWS,
            min(min(len1, len2), usize::try_from(len).unwrap_or(usize::MAX)),
            m,
        )
}

// The first `len` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty and reduced modulo `m`, which must fit in a limb, modulo `m`, computed by the word
// kernels. `len` must be positive. The result is not trimmed.
crate_test_fn! {mod_mul_truncated_word(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    m: &Natural,
) -> Vec<Limb> {
    let mut out = vec![0; len];
    mod_mul_truncated_to_out(
        &mut out,
        &naturals_to_limbs(xs),
        &naturals_to_limbs(ys),
        Limb::exact_from(m),
    );
    out
}}

// The product of the polynomials with coefficients `xs` and `ys`, truncated to `len` coefficients
// and reduced modulo `m`, as a polynomial. The word kernels are used in their window; otherwise the
// full truncated product is computed, in place when either factor is a constant, and reduced
// afterwards.
fn mod_mul_truncated_val_val(
    xs: Vec<Natural>,
    ys: Vec<Natural>,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    if mul_truncated_word_preferred(xs.len(), ys.len(), len, m) {
        let len = truncated_len(xs.len(), ys.len(), len);
        limbs_to_polynomial(mod_mul_truncated_word(&xs, &ys, len, m))
    } else {
        mod_reduce_coefficients(mul_truncated_val_val(xs, ys, len), m)
    }
}

// As `mod_mul_truncated_val_val`, taking the second factor by reference.
fn mod_mul_truncated_val_ref(
    xs: Vec<Natural>,
    ys: &[Natural],
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    if mul_truncated_word_preferred(xs.len(), ys.len(), len, m) {
        let len = truncated_len(xs.len(), ys.len(), len);
        limbs_to_polynomial(mod_mul_truncated_word(&xs, ys, len, m))
    } else {
        mod_reduce_coefficients(mul_truncated_val_ref(xs, ys, len), m)
    }
}

// As `mod_mul_truncated_val_val`, taking both factors by reference.
fn mod_mul_truncated_ref_ref(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    if mul_truncated_word_preferred(xs.len(), ys.len(), len, m) {
        let len = truncated_len(xs.len(), ys.len(), len);
        limbs_to_polynomial(mod_mul_truncated_word(xs, ys, len, m))
    } else {
        mod_reduce_coefficients(mul_truncated_ref_ref(xs, ys, len), m)
    }
}

// The first `len` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// reduced modulo `m`, modulo `m`: the truncated integer product, with its coefficients reduced
// afterwards. The result is not trimmed.
crate_test_fn! {mod_mul_truncated_full(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    m: &Natural,
) -> Vec<Natural> {
    let mut out = mul_truncated_ref_ref(xs, ys, u64::exact_from(len));
    for x in &mut out {
        x.mod_assign(m);
    }
    out
}}

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
        mod_mul_truncated_val_val(self.coefficients, other.coefficients, len, &m)
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
        mod_mul_truncated_val_val(self.coefficients, other.coefficients, len, m)
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
        mod_mul_truncated_val_ref(self.coefficients, &other.coefficients, len, &m)
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
        mod_mul_truncated_val_ref(self.coefficients, &other.coefficients, len, m)
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
        mod_mul_truncated_val_ref(other.coefficients, &self.coefficients, len, &m)
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
        mod_mul_truncated_val_ref(other.coefficients, &self.coefficients, len, m)
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
        mod_mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len, &m)
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
        mod_mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len, m)
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
        *self =
            mod_mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len, &m);
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
        *self = mod_mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len, m);
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
        *self =
            mod_mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len, &m);
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
        *self =
            mod_mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len, m);
    }
}
