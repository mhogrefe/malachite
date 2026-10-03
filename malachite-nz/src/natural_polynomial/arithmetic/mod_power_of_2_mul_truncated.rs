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
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::{
    low_preferred, mul_slots_karatsuba, mul_slots_karatsuba_threshold, naturals_to_slots,
    reduce_coefficients, slot_len, slots_add_assign, slots_to_naturals,
};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::take;
use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModPowerOf2MulTruncated, ModPowerOf2MulTruncatedAssign};
use malachite_base::slices::slice_test_zero;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;

// Sets `out` to the first slots of the product of the polynomials whose coefficients are in the
// slots of `xs` and `ys`, wrapping within each slot, by schoolbook multiplication.
pub(crate) fn mul_truncated_slots_classical(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    slot_len: usize,
) {
    let len = out.len() / slot_len;
    out.fill(0);
    let mut product = vec![0; slot_len];
    for (i, x) in xs.chunks_exact(slot_len).take(len).enumerate() {
        if slice_test_zero(x) {
            continue;
        }
        for (o, y) in out[i * slot_len..]
            .chunks_exact_mut(slot_len)
            .zip(ys.chunks_exact(slot_len))
        {
            limbs_mul_low_same_length(&mut product, x, y);
            limbs_slice_add_same_length_in_place_left(o, &product);
        }
    }
}

// Sets `out` to the first slots of the product of the polynomials whose coefficients are in the
// slots of `xs` and `ys`, both nonempty, wrapping within each slot. With $n$ the number of slots in
// `out` and $h = \lceil n/2 \rceil$, write x = x_0 + x^h x_1 and y = y_0 + x^h y_1; since $2h \geq
// n$, xy mod x^n is x_0 y_0 + x^h (x_1 y_0 + x_0 y_1) mod x^n. The first product is a full product
// of half-length factors, computed by Karatsuba multiplication, and the other two are truncated
// products of half the length, computed recursively.
pub(crate) fn mul_truncated_slots_karatsuba(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    slot_len: usize,
) {
    let len = out.len() / slot_len;
    let xs = &xs[..min(xs.len(), len * slot_len)];
    let ys = &ys[..min(ys.len(), len * slot_len)];
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let n = xs.len() / slot_len;
    let m = ys.len() / slot_len;
    let full_len = n + m - 1;
    if full_len <= len {
        mul_slots_karatsuba(&mut out[..full_len * slot_len], xs, ys, slot_len);
        out[full_len * slot_len..].fill(0);
        return;
    }
    if m < mul_slots_karatsuba_threshold(slot_len) {
        mul_truncated_slots_classical(out, xs, ys, slot_len);
        return;
    }
    let h = len.div_ceil(2);
    let split = h * slot_len;
    let x0 = &xs[..min(split, xs.len())];
    let y0 = &ys[..min(split, ys.len())];
    // The product of x_0 and y_0 has at most 2h - 1 <= `len` coefficients, so this is a full
    // product.
    mul_truncated_slots_karatsuba(out, x0, y0, slot_len);
    let mut cross = vec![0; (len - h) * slot_len];
    if n > h {
        mul_truncated_slots_karatsuba(&mut cross, &xs[split..], y0, slot_len);
        slots_add_assign(&mut out[split..], &cross, slot_len);
    }
    if m > h {
        mul_truncated_slots_karatsuba(&mut cross, x0, &ys[split..], slot_len);
        slots_add_assign(&mut out[split..], &cross, slot_len);
    }
}

// The first `len` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// all three nonempty and the inputs reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by
// schoolbook multiplication with low halves of products. The result is not trimmed.
crate_test_fn! {mod_power_of_2_mul_truncated_low_classical(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    pow: u64,
) -> Vec<Natural> {
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    mul_truncated_slots_classical(
        &mut out,
        &naturals_to_slots(xs, slot_len),
        &naturals_to_slots(ys, slot_len),
        slot_len,
    );
    slots_to_naturals(&out, slot_len, pow)
}}

// The first `len` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// all three nonempty and the inputs reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by
// Karatsuba multiplication with low halves of products: with word arithmetic when $k \leq
// \text{W}$, and on slots otherwise. The result is not trimmed.
crate_test_fn! {mod_power_of_2_mul_truncated_low_karatsuba(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    pow: u64,
) -> Vec<Natural> {
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    if pow <= Limb::WIDTH {
        let xs: Vec<Limb> = xs.iter().map(Limb::exact_from).collect();
        let ys: Vec<Limb> = ys.iter().map(Limb::exact_from).collect();
        let mut out = vec![0; len];
        mod_power_of_2_mul_truncated_to_out(&mut out, &xs, &ys, pow);
        return out.into_iter().map(Natural::from).collect();
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    mul_truncated_slots_karatsuba(
        &mut out,
        &naturals_to_slots(xs, slot_len),
        &naturals_to_slots(ys, slot_len),
        slot_len,
    );
    slots_to_naturals(&out, slot_len, pow)
}}

// The windows in which the low-half truncated multiplication kernels beat the full truncated
// product; see `MUL_LOW_WINDOWS`. Measured as for it, truncating to the length of the factors.
pub(crate) const MUL_TRUNCATED_LOW_WINDOWS: [(u64, usize); 13] = [
    (16, 150),
    (32, 500),
    (48, 1000),
    (64, 1000),
    (128, 64),
    (300, 150),
    (500, 64),
    (1000, 48),
    (2000, 32),
    (5000, 12),
    (10000, 8),
    (20000, 4),
    (u64::MAX, 0),
];

// The number of coefficients of a truncated product worth computing: `len`, but no more than the
// whole product of factors of lengths `len1` and `len2`.
pub(crate) fn truncated_len(len1: usize, len2: usize, len: u64) -> usize {
    min(usize::try_from(len).unwrap_or(usize::MAX), len1 + len2 - 1)
}

// Whether the low-half kernels are used for a truncated product of factors of lengths `len1` and
// `len2`: when neither is a constant and the shorter, after truncation to `len` coefficients, is in
// the window.
fn mul_truncated_low_preferred(len1: usize, len2: usize, len: u64, pow: u64) -> bool {
    len != 0
        && len1 > 1
        && len2 > 1
        && low_preferred(
            &MUL_TRUNCATED_LOW_WINDOWS,
            min(min(len1, len2), usize::try_from(len).unwrap_or(usize::MAX)),
            pow,
        )
}

// The product of the polynomials with coefficients `xs` and `ys`, truncated to `len` coefficients
// and reduced modulo $2^k$, where $k$ is `pow`, as a polynomial. The low-half kernels are used in
// their window; otherwise the full truncated product is computed, in place when either factor is a
// constant.
fn mod_power_of_2_mul_truncated_val_val(
    xs: Vec<Natural>,
    ys: Vec<Natural>,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    if mul_truncated_low_preferred(xs.len(), ys.len(), len, pow) {
        let len = truncated_len(xs.len(), ys.len(), len);
        reduce_coefficients(
            mod_power_of_2_mul_truncated_low_karatsuba(&xs, &ys, len, pow),
            pow,
        )
    } else {
        reduce_coefficients(mul_truncated_val_val(xs, ys, len), pow)
    }
}

// As `mod_power_of_2_mul_truncated_val_val`, taking the second factor by reference.
fn mod_power_of_2_mul_truncated_val_ref(
    xs: Vec<Natural>,
    ys: &[Natural],
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    if mul_truncated_low_preferred(xs.len(), ys.len(), len, pow) {
        let len = truncated_len(xs.len(), ys.len(), len);
        reduce_coefficients(
            mod_power_of_2_mul_truncated_low_karatsuba(&xs, ys, len, pow),
            pow,
        )
    } else {
        reduce_coefficients(mul_truncated_val_ref(xs, ys, len), pow)
    }
}

// As `mod_power_of_2_mul_truncated_val_val`, taking both factors by reference.
pub(crate) fn mod_power_of_2_mul_truncated_ref_ref(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    if mul_truncated_low_preferred(xs.len(), ys.len(), len, pow) {
        let len = truncated_len(xs.len(), ys.len(), len);
        reduce_coefficients(
            mod_power_of_2_mul_truncated_low_karatsuba(xs, ys, len, pow),
            pow,
        )
    } else {
        reduce_coefficients(mul_truncated_ref_ref(xs, ys, len), pow)
    }
}

// The first `len` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty and reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$: the truncated integer
// product, with its coefficients reduced afterwards. The result is trimmed.
crate_test_fn! {mod_power_of_2_mul_truncated_full(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    pow: u64,
) -> Vec<Natural> {
    let mut out = mul_truncated_ref_ref(xs, ys, u64::exact_from(len));
    for x in &mut out {
        x.mod_power_of_2_assign(pow);
    }
    out
}}

impl ModPowerOf2MulTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(self, other: Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        mod_power_of_2_mul_truncated_val_val(self.coefficients, other.coefficients, len, pow)
    }
}

impl ModPowerOf2MulTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(self, other: &Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        mod_power_of_2_mul_truncated_val_ref(self.coefficients, &other.coefficients, len, pow)
    }
}

impl ModPowerOf2MulTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        mod_power_of_2_mul_truncated_val_ref(other.coefficients, &self.coefficients, len, pow)
    }
}

impl ModPowerOf2MulTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        mod_power_of_2_mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len, pow)
    }
}

impl ModPowerOf2MulTruncatedAssign<Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_truncated_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4);
    /// assert_eq!(p.to_string(), "3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: Self, len: u64, pow: u64) {
        assert_reduced(self, &other, pow);
        *self = mod_power_of_2_mul_truncated_val_val(
            take(&mut self.coefficients),
            other.coefficients,
            len,
            pow,
        );
    }
}

impl ModPowerOf2MulTruncatedAssign<&Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_truncated_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4);
    /// assert_eq!(p.to_string(), "3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        assert_reduced(self, other, pow);
        *self = mod_power_of_2_mul_truncated_val_ref(
            take(&mut self.coefficients),
            &other.coefficients,
            len,
            pow,
        );
    }
}
