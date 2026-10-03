// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_ref;
use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mod_power_of_2_square::limbs_square_low;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::{
    low_preferred, naturals_to_slots, reduce_coefficients, slot_len, slots_add_assign,
    slots_to_naturals, square_slots_karatsuba_threshold,
};
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::natural_polynomial::arithmetic::mod_power_of_2_square::{
    assert_reduced, square_slots_karatsuba,
};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Assign, ModPowerOf2SquareAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModPowerOf2SquareTruncated, ModPowerOf2SquareTruncatedAssign};
use malachite_base::slices::slice_test_zero;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_square_truncated::*;

// Sets `out` to the first slots of the square of the polynomial whose coefficients are in the slots
// of `xs`, wrapping within each slot, by schoolbook multiplication: each product of two different
// coefficients is computed once and doubled.
pub(crate) fn square_truncated_slots_classical(out: &mut [Limb], xs: &[Limb], slot_len: usize) {
    let len = out.len() / slot_len;
    out.fill(0);
    let mut product = vec![0; slot_len];
    for (i, x) in xs.chunks_exact(slot_len).enumerate() {
        if (i << 1) + 1 >= len {
            break;
        }
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
    for o in out.chunks_exact_mut(slot_len) {
        limbs_slice_shl_in_place(o, 1);
    }
    for (o, x) in out
        .chunks_exact_mut(slot_len)
        .step_by(2)
        .zip(xs.chunks_exact(slot_len))
    {
        limbs_square_low(&mut product, x);
        limbs_slice_add_same_length_in_place_left(o, &product);
    }
}

// Sets `out` to the first slots of the square of the polynomial whose coefficients are in the slots
// of `xs`, which is nonempty, wrapping within each slot. With $n$ the number of slots in `out` and
// $h = \lceil n/2 \rceil$, write x = x_0 + x^h x_1; since $2h \geq n$, x^2 mod x^n is x_0^2 + 2 x^h
// x_0 x_1 mod x^n. The square is a full square of a half-length polynomial, computed by Karatsuba
// multiplication, and the cross product is a truncated product of half the length.
pub(crate) fn square_truncated_slots_karatsuba(out: &mut [Limb], xs: &[Limb], slot_len: usize) {
    let len = out.len() / slot_len;
    let xs = &xs[..min(xs.len(), len * slot_len)];
    let n = xs.len() / slot_len;
    let full_len = (n << 1) - 1;
    if full_len <= len {
        square_slots_karatsuba(&mut out[..full_len * slot_len], xs, slot_len);
        out[full_len * slot_len..].fill(0);
        return;
    }
    if n < square_slots_karatsuba_threshold(slot_len) {
        square_truncated_slots_classical(out, xs, slot_len);
        return;
    }
    let h = len.div_ceil(2);
    let split = h * slot_len;
    let x0 = &xs[..min(split, xs.len())];
    // The square of x_0 has at most 2h - 1 <= `len` coefficients, so this is a full square.
    square_truncated_slots_karatsuba(out, x0, slot_len);
    if n > h {
        let mut cross = vec![0; (len - h) * slot_len];
        mul_truncated_slots_karatsuba(&mut cross, x0, &xs[split..], slot_len);
        slots_add_assign(&mut out[split..], &cross, slot_len);
        slots_add_assign(&mut out[split..], &cross, slot_len);
    }
}

// The first `len` coefficients of the square of the polynomial with coefficients `xs`, both
// nonempty and the input reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by schoolbook
// multiplication with low halves of products. The result is not trimmed.
crate_test_fn! {mod_power_of_2_square_truncated_low_classical(
    xs: &[Natural],
    len: usize,
    pow: u64,
) -> Vec<Natural> {
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    square_truncated_slots_classical(&mut out, &naturals_to_slots(xs, slot_len), slot_len);
    slots_to_naturals(&out, slot_len, pow)
}}

// The first `len` coefficients of the square of the polynomial with coefficients `xs`, both
// nonempty and the input reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$, by Karatsuba
// multiplication with low halves of products: with word arithmetic when $k \leq \text{W}$, and on
// slots otherwise. The result is not trimmed.
crate_test_fn! {mod_power_of_2_square_truncated_low_karatsuba(
    xs: &[Natural],
    len: usize,
    pow: u64,
) -> Vec<Natural> {
    if pow == 0 {
        return vec![Natural::ZERO; len];
    }
    if pow <= Limb::WIDTH {
        let xs: Vec<Limb> = xs.iter().map(Limb::exact_from).collect();
        let mut out = vec![0; len];
        mod_power_of_2_square_truncated_to_out(&mut out, &xs, pow);
        return out.into_iter().map(Natural::from).collect();
    }
    let slot_len = slot_len(pow);
    let mut out = vec![0; len * slot_len];
    square_truncated_slots_karatsuba(&mut out, &naturals_to_slots(xs, slot_len), slot_len);
    slots_to_naturals(&out, slot_len, pow)
}}

// The windows in which the low-half truncated squaring kernels beat the full truncated square; see
// `MUL_LOW_WINDOWS`. Measured as for it, truncating to the length of the polynomial.
pub(crate) const SQUARE_TRUNCATED_LOW_WINDOWS: [(u64, usize); 13] = [
    (16, 200),
    (32, 1000),
    (64, 2000),
    (96, 64),
    (128, 100),
    (200, 150),
    (300, 200),
    (500, 100),
    (2000, 64),
    (5000, 16),
    (10000, 8),
    (20000, 4),
    (u64::MAX, 0),
];

// The square of the polynomial with coefficients `xs`, truncated to `len` coefficients and reduced
// modulo $2^k$, where $k$ is `pow`, as a polynomial. The low-half kernels are used in their window
// when the polynomial is not a constant, and the full truncated square otherwise.
pub(crate) fn mod_power_of_2_square_truncated_ref(
    xs: &[Natural],
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    let n = xs.len();
    let len_usize = usize::try_from(len).unwrap_or(usize::MAX);
    if len != 0 && n > 1 && low_preferred(&SQUARE_TRUNCATED_LOW_WINDOWS, min(n, len_usize), pow) {
        let len = min(len_usize, (n << 1) - 1);
        reduce_coefficients(
            mod_power_of_2_square_truncated_low_karatsuba(xs, len, pow),
            pow,
        )
    } else {
        reduce_coefficients(square_truncated_ref(xs, len), pow)
    }
}

// The first `len` coefficients of the square of the polynomial with coefficients `xs`, nonempty and
// reduced modulo $2^k$, where $k$ is `pow`, modulo $2^k$: the truncated integer square, with its
// coefficients reduced afterwards. The result is trimmed.
crate_test_fn! {mod_power_of_2_square_truncated_full(
    xs: &[Natural],
    len: usize,
    pow: u64,
) -> Vec<Natural> {
    let mut out = square_truncated_ref(xs, u64::exact_from(len));
    for x in &mut out {
        x.mod_power_of_2_assign(pow);
    }
    out
}}

impl ModPowerOf2SquareTruncated for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by value. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 3)
    ///         .to_string(),
    ///     "5*x^2+4*x+4"
    /// );
    /// // The square is 16*x^2+8*x+1; truncation and reduction leave 8*x+1.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same and the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_square_truncated(mut self, len: u64, pow: u64) -> Self {
        self.mod_power_of_2_square_truncated_assign(len, pow);
        self
    }
}

impl ModPowerOf2SquareTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by reference. The coefficients must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 3)
    ///         .to_string(),
    ///     "5*x^2+4*x+4"
    /// );
    /// // The square is 16*x^2+8*x+1; truncation and reduction leave 8*x+1.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same and the modulus $2^k$.
    fn mod_power_of_2_square_truncated(self, len: u64, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        mod_power_of_2_square_truncated_ref(&self.coefficients, len, pow)
    }
}

impl ModPowerOf2SquareTruncatedAssign for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo $2^k$ in place, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (p^2 \\bmod x^n) \\bmod 2^k.
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
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_square_truncated_assign(3, 3);
    /// assert_eq!(p.to_string(), "5*x^2+4*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same and the modulus $2^k$.
    fn mod_power_of_2_square_truncated_assign(&mut self, len: u64, pow: u64) {
        assert_reduced(self, pow);
        // The square of a constant is computed in place.
        if len != 0
            && let [c] = self.coefficients.as_mut_slice()
        {
            c.mod_power_of_2_square_assign(pow);
            self.trim();
        } else {
            *self = mod_power_of_2_square_truncated_ref(&self.coefficients, len, pow);
        }
    }
}
