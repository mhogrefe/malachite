// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, Parity};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModTaylorShift, ModTaylorShiftAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_mul::mod_mul_to_out;
use crate::unsigned_polynomial::arithmetic::mod_square::mod_square_to_out;
use alloc::vec;
use alloc::vec::Vec;

// The length from which splitting beats Horner's rule, as measured on this machine. It depends on
// the word size and on the size of the modulus, which decides how many words the split's
// multiplications accumulate in. For c = 1, Horner's rule only adds modulo m, and stays ahead to
// lengths in the thousands unless the products in the split fit easily in a word; for any other c,
// it multiplies modulo m at every step, and the split wins from lengths of a few dozen.
fn split_cutoff<T: PrimitiveUnsigned>(c: T, m: T) -> usize {
    let bits = m.significant_bits();
    if c == T::ONE {
        match T::WIDTH {
            8 | 16 => 1536,
            32 => {
                if bits <= 8 {
                    192
                } else if bits <= 16 {
                    3072
                } else {
                    2048
                }
            }
            64 => {
                if bits <= 16 {
                    256
                } else if bits <= 50 {
                    768
                } else {
                    2048
                }
            }
            _ => {
                if bits <= 16 {
                    96
                } else if bits <= 50 {
                    192
                } else if bits <= 100 {
                    1536
                } else {
                    4096
                }
            }
        }
    } else {
        match T::WIDTH {
            8 => 80,
            16 => {
                if bits <= 8 {
                    48
                } else {
                    80
                }
            }
            32 => {
                if bits <= 8 {
                    24
                } else if bits <= 16 {
                    32
                } else if bits < 32 {
                    48
                } else {
                    80
                }
            }
            64 => {
                if bits <= 16 {
                    32
                } else if bits <= 50 {
                    48
                } else if bits < 64 {
                    80
                } else {
                    128
                }
            }
            _ => {
                if bits <= 50 {
                    16
                } else if bits <= 100 {
                    24
                } else if bits < 128 {
                    32
                } else {
                    48
                }
            }
        }
    }
}

// Shifts the polynomial with coefficients `xs`, all reduced modulo `m`, by `c`, reduced modulo `m`,
// with Horner's rule, keeping the coefficients reduced.
crate_test_fn! {mod_taylor_shift_horner<T: PrimitiveUnsigned>(xs: &mut [T], c: T, m: T) {
    let n = xs.len();
    if n <= 1 || c == T::ZERO {
        return;
    }
    if c == T::ONE {
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let y = xs[j + 1];
                xs[j].mod_add_assign(y, m);
            }
        }
    } else {
        // Every step multiplies by the same c modulo the same m, so the data for that
        // multiplication is computed once.
        let data = T::precompute_mod_mul_data(&m);
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let product = c.mod_mul_precomputed(xs[j + 1], m, &data);
                xs[j].mod_add_assign(product, m);
            }
        }
    }
}}

// The coefficients of $(x + c)^k$ modulo `m`, which must be at least 2: the square of $(x +
// c)^{\lfloor k/2 \rfloor}$, times $x + c$ when $k$ is odd. The leading coefficient is 1, so the
// result has exactly $k + 1$ coefficients.
fn linear_power<T: PrimitiveUnsigned>(k: usize, c: T, m: T) -> Vec<T> {
    if k == 0 {
        return vec![T::ONE];
    }
    let half = linear_power(k >> 1, c, m);
    let mut out = vec![T::ZERO; k + 1];
    let square_len = ((k >> 1) << 1) + 1;
    mod_square_to_out(&mut out[..square_len], &half, m);
    if k.odd() {
        // Multiplies by x + c in place, from the top down.
        let data = T::precompute_mod_mul_data(&m);
        for i in (1..=k).rev() {
            out[i] = out[i - 1].mod_add(c.mod_mul_precomputed(out[i], m, &data), m);
        }
        out[0] = c.mod_mul_precomputed(out[0], m, &data);
    }
    out
}

// Shifts the polynomial with coefficients `xs`, all reduced modulo `m`, by `c`, reduced modulo `m`,
// splitting it when that is faster than Horner's rule.
crate_test_fn! {mod_taylor_shift_divide_and_conquer<T: PrimitiveUnsigned>(
    xs: &mut [T],
    c: T,
    m: T,
) {
    if c == T::ZERO || xs.len() < split_cutoff(c, m) {
        mod_taylor_shift_horner(xs, c, m);
    } else {
        mod_taylor_shift_divide_and_conquer_split(xs, c, m);
    }
}}

// The divide-and-conquer step: splits `xs` as $p_0 + x^k p_1$, shifts both halves with
// `mod_taylor_shift_divide_and_conquer`, and returns $p_0(x + c) + (x + c)^k p_1(x + c)$ modulo
// `m`. `xs` must have length at least 2, and `c` must be nonzero, so that `m` is at least 2.
crate_test_fn! {mod_taylor_shift_divide_and_conquer_split<T: PrimitiveUnsigned>(
    xs: &mut [T],
    c: T,
    m: T,
) {
    let len = xs.len();
    let len_lo = len >> 1;
    let (xs_lo, xs_hi) = xs.split_at_mut(len_lo);
    mod_taylor_shift_divide_and_conquer(xs_lo, c, m);
    mod_taylor_shift_divide_and_conquer(xs_hi, c, m);
    let powers = linear_power(len_lo, c, m);
    // The product has exactly len coefficients, since the factors have len_lo + 1 and len - len_lo.
    let mut product = vec![T::ZERO; len];
    mod_mul_to_out(&mut product, &powers, xs_hi, m);
    let (product_lo, product_hi) = product.split_at(len_lo);
    for (x, &y) in xs_lo.iter_mut().zip(product_lo) {
        x.mod_add_assign(y, m);
    }
    xs_hi.copy_from_slice(product_hi);
}}

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, c: T, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
}

impl<T: PrimitiveUnsigned> ModTaylorShift<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Shifts an [`UnsignedPolynomial`] by a number $c$ modulo $m$, replacing $x$ with $x + c$,
    /// taking the polynomial by value. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// Long polynomials are split in two, both halves are shifted recursively, and the high half is
    /// multiplied by $(x + c)^k$ modulo $m$, which is computed by repeated squaring, so nothing is
    /// ever divided modulo $m$ and any modulus works, prime or not. Short polynomials are shifted
    /// with Horner's rule. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.clone().mod_taylor_shift(3, 7).to_string(), "x^2+2*x+6");
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = UnsignedPolynomial::<u64>::from_str("x^4").unwrap();
    /// assert_eq!(p.clone().mod_taylor_shift(1, 4).to_string(), "x^4+2*x^2+1");
    /// ```
    #[inline]
    fn mod_taylor_shift(mut self, c: T, m: T) -> Self {
        self.mod_taylor_shift_assign(c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModTaylorShift<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Shifts an [`UnsignedPolynomial`] by a number $c$ modulo $m$, replacing $x$ with $x + c$,
    /// taking the polynomial by reference. The coefficients and $c$ must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// Long polynomials are split in two, both halves are shifted recursively, and the high half is
    /// multiplied by $(x + c)^k$ modulo $m$, which is computed by repeated squaring, so nothing is
    /// ever divided modulo $m$ and any modulus works, prime or not. Short polynomials are shifted
    /// with Horner's rule. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).mod_taylor_shift(3, 7).to_string(), "x^2+2*x+6");
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = UnsignedPolynomial::<u64>::from_str("x^4").unwrap();
    /// assert_eq!((&p).mod_taylor_shift(1, 4).to_string(), "x^4+2*x^2+1");
    /// ```
    #[inline]
    fn mod_taylor_shift(self, c: T, m: T) -> UnsignedPolynomial<T> {
        self.clone().mod_taylor_shift(c, m)
    }
}

impl<T: PrimitiveUnsigned> ModTaylorShiftAssign<T> for UnsignedPolynomial<T> {
    /// Shifts an [`UnsignedPolynomial`] by a number $c$ modulo $m$ in place, replacing $x$ with $x
    /// + c$. The coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod m.
    /// $$
    ///
    /// Long polynomials are split in two, both halves are shifted recursively, and the high half is
    /// multiplied by $(x + c)^k$ modulo $m$, which is computed by repeated squaring, so nothing is
    /// ever divided modulo $m$ and any modulus works, prime or not. Short polynomials are shifted
    /// with Horner's rule. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModTaylorShiftAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_taylor_shift_assign(3, 7);
    /// assert_eq!(p.to_string(), "x^2+2*x+6");
    ///
    /// let mut p = UnsignedPolynomial::<u64>::from_str("x^4").unwrap();
    /// p.mod_taylor_shift_assign(1, 4);
    /// assert_eq!(p.to_string(), "x^4+2*x^2+1");
    /// ```
    fn mod_taylor_shift_assign(&mut self, c: T, m: T) {
        assert_reduced(self, c, m);
        mod_taylor_shift_divide_and_conquer(&mut self.coefficients, c, m);
    }
}
