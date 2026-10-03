// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul::{mul_ref_ref, mul_val_ref, mul_val_val};
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::low_preferred;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{ModAssign, ModMul, ModMulAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::unsigned_polynomial::arithmetic::mod_mul::mod_mul_to_out;

// The polynomial whose coefficients are `xs`, each reduced modulo `m`, and trimmed.
pub(crate) fn mod_reduce_coefficients(mut xs: Vec<Natural>, m: &Natural) -> NaturalPolynomial {
    for x in &mut xs {
        x.mod_assign(m);
    }
    let mut p = NaturalPolynomial { coefficients: xs };
    p.trim();
    p
}

// # Multiplication modulo a word
//
// When $m$ fits in a limb, the coefficients are converted to limbs and multiplied by
// malachite-base's word kernels, which accumulate each coefficient of the product exactly in at
// most three limbs and reduce it once. Since the full product is computed by Kronecker
// substitution, whose cost grows almost linearly, the word kernels only win for short factors.

// For each operation, the lengths and moduli for which the word kernels beat the full product. Each
// pair `(max_bits, max_len)` covers the moduli with up to `max_bits` bits not covered by an earlier
// pair, and the word kernels are used when the shorter factor has at most `max_len` coefficients
// and $m$ fits in a limb. Measured on an Apple M-series machine, 2026-10, with 64-bit limbs, on
// factors of equal length. The word kernels gain the most for moduli of more than 32 bits, whose
// products no longer fit in a limb: two to eight times at short lengths. For smaller moduli the
// gain is at most about 40%, and for moduli of 32 bits, whose products just fit in a limb, the full
// product is up to 5% faster for lengths 3 to 12.
pub(crate) const MUL_WORD_WINDOWS: [(u64, usize); 5] =
    [(26, 80), (31, 96), (40, 192), (60, 320), (64, 160)];

// Whether the word kernels are used for a product, according to `windows`, when the length that
// decides it is `len`: when $m$ fits in a limb and `len` is in the window.
pub(crate) fn word_preferred(windows: &[(u64, usize)], len: usize, m: &Natural) -> bool {
    let bits = m.significant_bits();
    bits <= Limb::WIDTH && low_preferred(windows, len, bits)
}

// Whether the word kernels are used to multiply factors of lengths `len1` and `len2` modulo `m`:
// when neither is a constant and the shorter is in the window.
fn mul_word_preferred(len1: usize, len2: usize, m: &Natural) -> bool {
    len1 > 1 && len2 > 1 && word_preferred(&MUL_WORD_WINDOWS, min(len1, len2), m)
}

// The coefficients `xs`, which fit in limbs, as limbs.
pub(crate) fn naturals_to_limbs(xs: &[Natural]) -> Vec<Limb> {
    xs.iter().map(Limb::exact_from).collect()
}

// The polynomial whose coefficients are `xs`, trimmed.
pub(crate) fn limbs_to_polynomial(xs: Vec<Limb>) -> NaturalPolynomial {
    let mut p = NaturalPolynomial {
        coefficients: xs.into_iter().map(Natural::from).collect(),
    };
    p.trim();
    p
}

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, both nonempty
// and reduced modulo `m`, which must fit in a limb, modulo `m`, computed by the word kernels. The
// result is not trimmed.
crate_test_fn! {mod_mul_word(xs: &[Natural], ys: &[Natural], m: &Natural) -> Vec<Limb> {
    let mut out = vec![0; xs.len() + ys.len() - 1];
    mod_mul_to_out(
        &mut out,
        &naturals_to_limbs(xs),
        &naturals_to_limbs(ys),
        Limb::exact_from(m),
    );
    out
}}

// The product of the polynomials with coefficients `xs` and `ys`, reduced modulo `m`, as a
// polynomial. The word kernels are used in their window; otherwise, as in FLINT, the product is
// computed in full, in place when either factor is a constant, and reduced afterwards.
fn mod_mul_val_val(xs: Vec<Natural>, ys: Vec<Natural>, m: &Natural) -> NaturalPolynomial {
    if mul_word_preferred(xs.len(), ys.len(), m) {
        limbs_to_polynomial(mod_mul_word(&xs, &ys, m))
    } else {
        mod_reduce_coefficients(mul_val_val(xs, ys), m)
    }
}

// As `mod_mul_val_val`, taking the second factor by reference.
fn mod_mul_val_ref(xs: Vec<Natural>, ys: &[Natural], m: &Natural) -> NaturalPolynomial {
    if mul_word_preferred(xs.len(), ys.len(), m) {
        limbs_to_polynomial(mod_mul_word(&xs, ys, m))
    } else {
        mod_reduce_coefficients(mul_val_ref(xs, ys), m)
    }
}

// As `mod_mul_val_val`, taking both factors by reference.
pub(crate) fn mod_mul_ref_ref(xs: &[Natural], ys: &[Natural], m: &Natural) -> NaturalPolynomial {
    if mul_word_preferred(xs.len(), ys.len(), m) {
        limbs_to_polynomial(mod_mul_word(xs, ys, m))
    } else {
        mod_reduce_coefficients(mul_ref_ref(xs, ys), m)
    }
}

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, both nonempty
// and reduced modulo `m`, modulo `m`: the full product, computed as an integer product, with its
// coefficients reduced afterwards. The result is not trimmed.
crate_test_fn! {mod_mul_full(xs: &[Natural], ys: &[Natural], m: &Natural) -> Vec<Natural> {
    let mut out = mul_ref_ref(xs, ys);
    for x in &mut out {
        x.mod_assign(m);
    }
    out
}}

impl ModMul<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the
    /// second by value, and the modulus by value. The coefficients of both polynomials must already
    /// be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: Self, m: Natural) -> Self {
        assert_reduced(&self, &other, &m);
        mod_mul_val_val(self.coefficients, other.coefficients, &m)
    }
}

impl ModMul<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the
    /// second by value, and the modulus by reference. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             &Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: Self, m: &Natural) -> Self {
        assert_reduced(&self, &other, m);
        mod_mul_val_val(self.coefficients, other.coefficients, m)
    }
}

impl ModMul<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the
    /// second by reference, and the modulus by value. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: &Self, m: Natural) -> Self {
        assert_reduced(&self, other, &m);
        mod_mul_val_ref(self.coefficients, &other.coefficients, &m)
    }
}

impl ModMul<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the
    /// second by reference, and the modulus by reference. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             &Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: &Self, m: &Natural) -> Self {
        assert_reduced(&self, other, m);
        mod_mul_val_ref(self.coefficients, &other.coefficients, m)
    }
}

impl ModMul<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference,
    /// the second by value, and the modulus by value. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, &other, &m);
        mod_mul_val_ref(other.coefficients, &self.coefficients, &m)
    }
}

impl ModMul<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference,
    /// the second by value, and the modulus by reference. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             &Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, &other, m);
        mod_mul_val_ref(other.coefficients, &self.coefficients, m)
    }
}

impl ModMul<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference,
    /// the second by reference, and the modulus by value. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, other, &m);
        mod_mul_ref_ref(&self.coefficients, &other.coefficients, &m)
    }
}

impl ModMul<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference,
    /// the second by reference, and the modulus by reference. The coefficients of both polynomials
    /// must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the product can vanish modulo `m`, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 2, 11, 19, and 10 are reduced modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // Modulo 6, which has zero divisors, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_mul(
    ///             &NaturalPolynomial::from_str("3*x+1").unwrap(),
    ///             &Natural::from(6u32)
    ///         )
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, other, m);
        mod_mul_ref_ref(&self.coefficients, &other.coefficients, m)
    }
}

impl ModMulAssign<Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// taking the right-hand side by value and the modulus by value. The coefficients of both
    /// polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets pq \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_assign(
    ///     NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "2*x^3+4*x^2+5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul_assign(&mut self, other: Self, m: Natural) {
        assert_reduced(self, &other, &m);
        *self = mod_mul_val_val(take(&mut self.coefficients), other.coefficients, &m);
    }
}

impl ModMulAssign<Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// taking the right-hand side by value and the modulus by reference. The coefficients of both
    /// polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets pq \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_assign(
    ///     NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "2*x^3+4*x^2+5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul_assign(&mut self, other: Self, m: &Natural) {
        assert_reduced(self, &other, m);
        *self = mod_mul_val_val(take(&mut self.coefficients), other.coefficients, m);
    }
}

impl ModMulAssign<&Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// taking the right-hand side by reference and the modulus by value. The coefficients of both
    /// polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets pq \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_assign(
    ///     &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "2*x^3+4*x^2+5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul_assign(&mut self, other: &Self, m: Natural) {
        assert_reduced(self, other, &m);
        *self = mod_mul_val_ref(take(&mut self.coefficients), &other.coefficients, &m);
    }
}

impl ModMulAssign<&Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo `m` in place,
    /// taking the right-hand side by reference and the modulus by reference. The coefficients of
    /// both polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets pq \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_assign(
    ///     &NaturalPolynomial::from_str("2*x+5").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "2*x^3+4*x^2+5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul_assign(&mut self, other: &Self, m: &Natural) {
        assert_reduced(self, other, m);
        *self = mod_mul_val_ref(take(&mut self.coefficients), &other.coefficients, m);
    }
}
