// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul::{mul_ref_ref, mul_val_ref, mul_val_val};
use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul::mod_power_of_2_mul_to_out;

// # Multiplication with low halves of products
//
// When only the coefficients of a product modulo $2^k$ are wanted, each product of two coefficients
// is only needed modulo $2^k$ too, and so are the sums. With $L = \lceil k/\text{W} \rceil$, where
// W is `Limb::WIDTH`, every coefficient is stored in a slot of $L$ limbs, products are computed
// with low-half multiplication, keeping $L$ limbs, and sums wrap around within a slot. Since $2^k$
// divides $2^{L\text{W}}$, reducing each slot modulo $2^k$ at the end gives the coefficients modulo
// $2^k$. When $k \leq \text{W}$ the slots are single limbs, and malachite-base's word kernels do
// the work.

// The length of the shorter factor at which Karatsuba multiplication overtakes classical
// multiplication, for coefficients in slots of `slot_len` limbs, which is at least 2. The larger
// the slots, the more a product of two coefficients costs compared with the additions Karatsuba
// trades it for, and the earlier Karatsuba wins. Measured on an Apple M-series machine, 2026-10:
// about 48 for 2-limb slots, about 20 for 4, and at most 8 for 10 or more.
pub(crate) fn mul_slots_karatsuba_threshold(slot_len: usize) -> usize {
    max(4, 96 / slot_len)
}

// The length at which Karatsuba squaring overtakes classical squaring, for coefficients in slots of
// `slot_len` limbs, which is at least 2. Classical squaring computes only half the products, so it
// stays ahead much longer: measured as above, past 512 for 2-limb slots, about 192 for 4, about 20
// for 10, and at most 8 for 32.
pub(crate) fn square_slots_karatsuba_threshold(slot_len: usize) -> usize {
    max(4, 2048 / (slot_len * slot_len))
}

// The number of limbs in a slot holding values modulo $2^k$, where $k$ is `pow`.
pub(crate) fn slot_len(pow: u64) -> usize {
    usize::exact_from(pow.div_ceil(Limb::WIDTH))
}

// The coefficients `xs`, each less than $2^{k \text{W}}$, where $k$ is `slot_len`, in consecutive
// slots of `slot_len` limbs.
pub(crate) fn naturals_to_slots(xs: &[Natural], slot_len: usize) -> Vec<Limb> {
    let mut slots = vec![0; xs.len() * slot_len];
    for (slot, x) in slots.chunks_exact_mut(slot_len).zip(xs) {
        let limbs = x.as_limbs_asc();
        slot[..limbs.len()].copy_from_slice(limbs);
    }
    slots
}

// The values in consecutive slots of `slot_len` limbs, each reduced modulo $2^k$, where $k$ is
// `pow`.
pub(crate) fn slots_to_naturals(slots: &[Limb], slot_len: usize, pow: u64) -> Vec<Natural> {
    let mut slots = slots.to_vec();
    let rem = pow & Limb::WIDTH_MASK;
    slots
        .chunks_exact_mut(slot_len)
        .map(|slot| {
            if rem != 0 {
                slot[slot_len - 1].mod_power_of_2_assign(rem);
            }
            Natural::from_limbs_asc(slot)
        })
        .collect()
}

// Adds each slot of `ys` to the slot of `xs` at the same index, wrapping within each slot.
pub(crate) fn slots_add_assign(xs: &mut [Limb], ys: &[Limb], slot_len: usize) {
    for (x, y) in xs.chunks_exact_mut(slot_len).zip(ys.chunks_exact(slot_len)) {
        limbs_slice_add_same_length_in_place_left(x, y);
    }
}

// Subtracts each slot of `ys` from the slot of `xs` at the same index, wrapping within each slot.
pub(crate) fn slots_sub_assign(xs: &mut [Limb], ys: &[Limb], slot_len: usize) {
    for (x, y) in xs.chunks_exact_mut(slot_len).zip(ys.chunks_exact(slot_len)) {
        limbs_sub_same_length_in_place_left(x, y);
    }
}

// Sets `out` to the product of the polynomials whose coefficients are in the slots of `xs` and
// `ys`, wrapping within each slot, by schoolbook multiplication. `out` must hold `xs.len() +
// ys.len() - 1` slots, counting slots rather than limbs.
pub(crate) fn mul_slots_classical(out: &mut [Limb], xs: &[Limb], ys: &[Limb], slot_len: usize) {
    out.fill(0);
    let mut product = vec![0; slot_len];
    for (i, x) in xs.chunks_exact(slot_len).enumerate() {
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

// Sets `out` to the product of the polynomials whose coefficients are in the slots of `xs` and
// `ys`, both nonempty, wrapping within each slot, by Karatsuba multiplication, falling back to
// schoolbook multiplication for short factors. When the factors' lengths differ, the longer is cut
// into pieces as long as the shorter, and the products of the pieces are added together.
pub(crate) fn mul_slots_karatsuba(out: &mut [Limb], xs: &[Limb], ys: &[Limb], slot_len: usize) {
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let n = xs.len() / slot_len;
    let m = ys.len() / slot_len;
    if m < mul_slots_karatsuba_threshold(slot_len) {
        mul_slots_classical(out, xs, ys, slot_len);
        return;
    }
    if n > m {
        out.fill(0);
        let mut product = vec![0; ((m << 1) - 1) * slot_len];
        for (k, piece) in xs.chunks(m * slot_len).enumerate() {
            let product = &mut product[..piece.len() + (m - 1) * slot_len];
            mul_slots_karatsuba(product, piece, ys, slot_len);
            slots_add_assign(&mut out[k * m * slot_len..], product, slot_len);
        }
        return;
    }
    // Write x = x_0 + x^h x_1 and y = y_0 + x^h y_1. Then xy = x_0 y_0 + x^h ((x_0 + x_1)(y_0 +
    // y_1) - x_0 y_0 - x_1 y_1) + x^{2h} x_1 y_1.
    let h = n >> 1;
    let split = h * slot_len;
    let low_len = ((h << 1) - 1) * slot_len;
    let (x0, x1) = xs.split_at(split);
    let (y0, y1) = ys.split_at(split);
    {
        let (low, high) = out.split_at_mut(split << 1);
        mul_slots_karatsuba(&mut low[..low_len], x0, y0, slot_len);
        low[low_len..].fill(0);
        mul_slots_karatsuba(high, x1, y1, slot_len);
    }
    let mut x_sum = x1.to_vec();
    slots_add_assign(&mut x_sum, x0, slot_len);
    let mut y_sum = y1.to_vec();
    slots_add_assign(&mut y_sum, y0, slot_len);
    let mut middle = vec![0; (((n - h) << 1) - 1) * slot_len];
    mul_slots_karatsuba(&mut middle, &x_sum, &y_sum, slot_len);
    slots_sub_assign(&mut middle, &out[..low_len], slot_len);
    slots_sub_assign(&mut middle, &out[split << 1..], slot_len);
    slots_add_assign(&mut out[split..], &middle, slot_len);
}

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, both nonempty
// and reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by schoolbook multiplication with low
// halves of products. The result is not trimmed.
crate_test_fn! {mod_power_of_2_mul_low_classical(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> Vec<Natural> {
    let len = xs.len() + ys.len() - 1;
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    mul_slots_classical(
        &mut out,
        &naturals_to_slots(xs, slot_len),
        &naturals_to_slots(ys, slot_len),
        slot_len,
    );
    slots_to_naturals(&out, slot_len, pow)
}}

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, both nonempty
// and reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by Karatsuba multiplication with low
// halves of products: with word arithmetic when $k \leq \text{W}$, and on slots otherwise. The
// result is not trimmed.
crate_test_fn! {mod_power_of_2_mul_low_karatsuba(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> Vec<Natural> {
    let len = xs.len() + ys.len() - 1;
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    if pow <= Limb::WIDTH {
        let xs: Vec<Limb> = xs.iter().map(Limb::exact_from).collect();
        let ys: Vec<Limb> = ys.iter().map(Limb::exact_from).collect();
        let mut out = vec![0; len];
        mod_power_of_2_mul_to_out(&mut out, &xs, &ys, pow);
        return out.into_iter().map(Natural::from).collect();
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    mul_slots_karatsuba(
        &mut out,
        &naturals_to_slots(xs, slot_len),
        &naturals_to_slots(ys, slot_len),
        slot_len,
    );
    slots_to_naturals(&out, slot_len, pow)
}}

// The polynomial whose coefficients are `xs`, each reduced modulo $2^k$, where $k$ is `pow`, and
// trimmed.
pub(crate) fn reduce_coefficients(mut xs: Vec<Natural>, pow: u64) -> NaturalPolynomial {
    for x in &mut xs {
        x.mod_power_of_2_assign(pow);
    }
    let mut p = NaturalPolynomial { coefficients: xs };
    p.trim();
    p
}

// For each operation, the lengths and coefficient sizes for which its low-half kernels beat the
// full product. Each pair `(max_pow, max_len)` covers the powers $k$ up to `max_pow` not covered by
// an earlier pair, and the low-half kernels are used when the shorter factor has at most `max_len`
// coefficients. Measured on an Apple M-series machine, 2026-10, with 64-bit limbs, on factors of
// equal length; beyond the last measured power the full product is used. The windows are not
// monotone in $k$: 96-bit coefficients fill only three quarters of their 2-limb slots, while the
// full product's cost follows the actual number of bits.
pub(crate) const MUL_LOW_WINDOWS: [(u64, usize); 11] = [
    (16, 150),
    (32, 500),
    (48, 1000),
    (64, 2000),
    (96, 24),
    (128, 64),
    (300, 100),
    (500, 48),
    (2000, 32),
    (5000, 8),
    (u64::MAX, 0),
];

// Whether the low-half kernels beat the full product for factors the shorter of which has `len2`
// coefficients, at power `pow`, according to `windows`.
pub(crate) fn low_preferred(windows: &[(u64, usize)], len2: usize, pow: u64) -> bool {
    windows
        .iter()
        .find(|&&(max_pow, _)| pow <= max_pow)
        .is_some_and(|&(_, max_len)| len2 <= max_len)
}

// Whether the low-half kernels are used to multiply factors of lengths `len1` and `len2`: when
// neither is a constant and the shorter is in the window.
fn mul_low_preferred(len1: usize, len2: usize, pow: u64) -> bool {
    len1 > 1 && len2 > 1 && low_preferred(&MUL_LOW_WINDOWS, min(len1, len2), pow)
}

// The product of the polynomials with coefficients `xs` and `ys`, reduced modulo $2^k$, where $k$
// is `pow`, as a polynomial. The low-half kernels are used in their window; otherwise the full
// product is computed, in place when either factor is a constant.
pub(crate) fn mod_power_of_2_mul_val_val(
    xs: Vec<Natural>,
    ys: Vec<Natural>,
    pow: u64,
) -> NaturalPolynomial {
    if mul_low_preferred(xs.len(), ys.len(), pow) {
        reduce_coefficients(mod_power_of_2_mul_low_karatsuba(&xs, &ys, pow), pow)
    } else {
        reduce_coefficients(mul_val_val(xs, ys), pow)
    }
}

// As `mod_power_of_2_mul_val_val`, taking the second factor by reference.
pub(crate) fn mod_power_of_2_mul_val_ref(
    xs: Vec<Natural>,
    ys: &[Natural],
    pow: u64,
) -> NaturalPolynomial {
    if mul_low_preferred(xs.len(), ys.len(), pow) {
        reduce_coefficients(mod_power_of_2_mul_low_karatsuba(&xs, ys, pow), pow)
    } else {
        reduce_coefficients(mul_val_ref(xs, ys), pow)
    }
}

// As `mod_power_of_2_mul_val_val`, taking both factors by reference.
pub(crate) fn mod_power_of_2_mul_ref_ref(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> NaturalPolynomial {
    if mul_low_preferred(xs.len(), ys.len(), pow) {
        reduce_coefficients(mod_power_of_2_mul_low_karatsuba(xs, ys, pow), pow)
    } else {
        reduce_coefficients(mul_ref_ref(xs, ys), pow)
    }
}

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, both nonempty
// and reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$: the full product, computed as an
// integer product, with its coefficients reduced afterwards. The result is not trimmed.
crate_test_fn! {mod_power_of_2_mul_full(xs: &[Natural], ys: &[Natural], pow: u64) -> Vec<Natural> {
    let mut out = mul_ref_ref(xs, ys);
    for x in &mut out {
        x.mod_power_of_2_assign(pow);
    }
    out
}}

impl ModPowerOf2Mul<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking both by value. The coefficients
    /// of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        mod_power_of_2_mul_val_val(self.coefficients, other.coefficients, pow)
    }
}

impl ModPowerOf2Mul<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking the first by value and the second
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        mod_power_of_2_mul_val_ref(self.coefficients, &other.coefficients, pow)
    }
}

impl ModPowerOf2Mul<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking the first by reference and the
    /// second by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        mod_power_of_2_mul_val_ref(other.coefficients, &self.coefficients, pow)
    }
}

impl ModPowerOf2Mul<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking both by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        mod_power_of_2_mul_ref_ref(&self.coefficients, &other.coefficients, pow)
    }
}

impl ModPowerOf2MulAssign<Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// taking the right-hand side by value. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// p \\gets pq \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        *self = mod_power_of_2_mul_val_val(take(&mut self.coefficients), other.coefficients, pow);
    }
}

impl ModPowerOf2MulAssign<&Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// taking the right-hand side by reference. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// p \\gets pq \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        *self = mod_power_of_2_mul_val_ref(take(&mut self.coefficients), &other.coefficients, pow);
    }
}
