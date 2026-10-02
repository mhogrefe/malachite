// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square::square_ref;
use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mod_power_of_2_square::limbs_square_low;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::{
    low_preferred, naturals_to_slots, reduce_coefficients, slot_len, slots_add_assign,
    slots_sub_assign, slots_to_naturals, square_slots_karatsuba_threshold,
};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_square::*;

// Doubles each slot of `xs`, wrapping within each slot.
fn slots_double_assign(xs: &mut [Limb], slot_len: usize) {
    for x in xs.chunks_exact_mut(slot_len) {
        limbs_slice_shl_in_place(x, 1);
    }
}

// Sets `out` to the square of the polynomial whose coefficients are in the slots of `xs`, wrapping
// within each slot, by schoolbook multiplication: each product of two different coefficients is
// computed once and doubled. `out` must hold `2 * n - 1` slots, where `n` is the number of slots in
// `xs`.
pub(crate) fn square_slots_classical(out: &mut [Limb], xs: &[Limb], slot_len: usize) {
    out.fill(0);
    let mut product = vec![0; slot_len];
    for (i, x) in xs.chunks_exact(slot_len).enumerate() {
        if slice_test_zero(x) {
            continue;
        }
        for (o, y) in out[((i << 1) + 1) * slot_len..]
            .chunks_exact_mut(slot_len)
            .zip(xs[(i + 1) * slot_len..].chunks_exact(slot_len))
        {
            limbs_mul_low_same_length(&mut product, x, y);
            limbs_slice_add_same_length_in_place_left(o, &product);
        }
    }
    slots_double_assign(out, slot_len);
    for (o, x) in out
        .chunks_exact_mut(slot_len)
        .step_by(2)
        .zip(xs.chunks_exact(slot_len))
    {
        limbs_square_low(&mut product, x);
        limbs_slice_add_same_length_in_place_left(o, &product);
    }
}

// Sets `out` to the square of the polynomial whose coefficients are in the slots of `xs`, which is
// nonempty, wrapping within each slot, by Karatsuba multiplication, falling back to schoolbook
// multiplication for short polynomials.
pub(crate) fn square_slots_karatsuba(out: &mut [Limb], xs: &[Limb], slot_len: usize) {
    let n = xs.len() / slot_len;
    if n < square_slots_karatsuba_threshold(slot_len) {
        square_slots_classical(out, xs, slot_len);
        return;
    }
    // Write x = x_0 + x^h x_1. Then x^2 = x_0^2 + x^h ((x_0 + x_1)^2 - x_0^2 - x_1^2) + x^{2h}
    // x_1^2.
    let h = n >> 1;
    let split = h * slot_len;
    let low_len = ((h << 1) - 1) * slot_len;
    let (x0, x1) = xs.split_at(split);
    let (low, high) = out.split_at_mut(split << 1);
    square_slots_karatsuba(&mut low[..low_len], x0, slot_len);
    low[low_len..].fill(0);
    square_slots_karatsuba(high, x1, slot_len);
    let mut sum = x1.to_vec();
    slots_add_assign(&mut sum, x0, slot_len);
    let mut middle = vec![0; (((n - h) << 1) - 1) * slot_len];
    square_slots_karatsuba(&mut middle, &sum, slot_len);
    slots_sub_assign(&mut middle, &out[..low_len], slot_len);
    slots_sub_assign(&mut middle, &out[split << 1..], slot_len);
    slots_add_assign(&mut out[split..], &middle, slot_len);
}

// The coefficients of the square of the polynomial with coefficients `xs`, nonempty and reduced
// modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by schoolbook multiplication with low halves of
// products. The result is not trimmed.
crate_test_fn! {mod_power_of_2_square_low_classical(xs: &[Natural], pow: u64) -> Vec<Natural> {
    let len = (xs.len() << 1) - 1;
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    square_slots_classical(&mut out, &naturals_to_slots(xs, slot_len), slot_len);
    slots_to_naturals(&out, slot_len, pow)
}}

// The coefficients of the square of the polynomial with coefficients `xs`, nonempty and reduced
// modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by Karatsuba multiplication with low halves of
// products: with word arithmetic when $k \leq \text{W}$, and on slots otherwise. The result is not
// trimmed.
crate_test_fn! {mod_power_of_2_square_low_karatsuba(xs: &[Natural], pow: u64) -> Vec<Natural> {
    let len = (xs.len() << 1) - 1;
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    if pow <= Limb::WIDTH {
        let xs: Vec<Limb> = xs.iter().map(Limb::exact_from).collect();
        let mut out = vec![0; len];
        mod_power_of_2_square_to_out(&mut out, &xs, pow);
        return out.into_iter().map(Natural::from).collect();
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    square_slots_karatsuba(&mut out, &naturals_to_slots(xs, slot_len), slot_len);
    slots_to_naturals(&out, slot_len, pow)
}}

pub(crate) fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The windows in which the low-half squaring kernels beat the full square; see `MUL_LOW_WINDOWS`.
// Measured as for it.
pub(crate) const SQUARE_LOW_WINDOWS: [(u64, usize); 11] = [
    (16, 150),
    (32, 1000),
    (64, 2000),
    (96, 48),
    (200, 64),
    (300, 100),
    (2000, 32),
    (5000, 16),
    (10000, 8),
    (20000, 4),
    (u64::MAX, 0),
];

// The square of the polynomial with coefficients `xs`, which has more than one coefficient, reduced
// modulo $2^k$, where $k$ is `pow`, as a polynomial. The low-half kernels are used in their window,
// and the full square otherwise.
fn mod_power_of_2_square_ref(xs: &[Natural], pow: u64) -> NaturalPolynomial {
    if xs.len() > 1 && low_preferred(&SQUARE_LOW_WINDOWS, xs.len(), pow) {
        reduce_coefficients(mod_power_of_2_square_low_karatsuba(xs, pow), pow)
    } else {
        reduce_coefficients(square_ref(xs), pow)
    }
}

// The coefficients of the square of the polynomial with coefficients `xs`, nonempty and reduced
// modulo $2^k$, where $k$ is `pow`, modulo $2^k$: the full square, computed as an integer square,
// with its coefficients reduced afterwards. The result is not trimmed.
crate_test_fn! {mod_power_of_2_square_full(xs: &[Natural], pow: u64) -> Vec<Natural> {
    let mut out = square_ref(xs);
    for x in &mut out {
        x.mod_power_of_2_assign(pow);
    }
    out
}}

impl ModPowerOf2Square for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, taking it by value. The coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p^2 \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the square can vanish modulo $2^k$, and then the degree of the
    /// square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 8.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "x^4+6*x^3+5*x^2+4*x+4"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square(4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    #[inline]
    fn mod_power_of_2_square(mut self, pow: u64) -> Self {
        self.mod_power_of_2_square_assign(pow);
        self
    }
}

impl ModPowerOf2Square for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, taking it by reference. The coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p^2 \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the square can vanish modulo $2^k$, and then the degree of the
    /// square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 8.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "x^4+6*x^3+5*x^2+4*x+4"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square(4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_square(self, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        mod_power_of_2_square_ref(&self.coefficients, pow)
    }
}

impl ModPowerOf2SquareAssign for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo $2^k$ in place. The coefficients must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p^2 \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_square_assign(3);
    /// assert_eq!(p.to_string(), "x^4+6*x^3+5*x^2+4*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_square_assign(&mut self, pow: u64) {
        assert_reduced(self, pow);
        // The square of a constant is computed in place.
        if let [c] = self.coefficients.as_mut_slice() {
            c.mod_power_of_2_square_assign(pow);
            self.trim();
        } else {
            *self = mod_power_of_2_square_ref(&self.coefficients, pow);
        }
    }
}
