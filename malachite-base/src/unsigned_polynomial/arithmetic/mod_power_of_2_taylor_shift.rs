// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    add_wrapping_assign, mask_coefficients, mod_power_of_2_mul_to_out,
};
use alloc::vec;
use alloc::vec::Vec;

// The length from which splitting beats Horner's rule, as measured on this machine. Horner's rule
// is a wrapping multiply-add per step, which the compiler vectorizes for narrow words, so the
// narrower the word, the longer Horner's rule stays ahead; for 128-bit words, the multiplications
// in the split are expensive, so it stays ahead longer when c = 1 and it only adds.
fn split_cutoff<T: PrimitiveUnsigned>(c: T, pow: u64) -> usize {
    match (T::WIDTH, c == T::ONE) {
        (8, _) => 3072,
        (16, _) => {
            if pow <= 8 {
                2048
            } else {
                6144
            }
        }
        (32, _) => 1536,
        (64, true) => 768,
        (64, false) => 192,
        (_, true) => 2048,
        (_, false) => 160,
    }
}

// Shifts the polynomial with coefficients `xs` by `c` modulo 2^pow, with Horner's rule in wrapping
// arithmetic, reducing at the end.
crate_test_fn! {mod_power_of_2_taylor_shift_horner<T: PrimitiveUnsigned>(
    xs: &mut [T],
    c: T,
    pow: u64,
) {
    let n = xs.len();
    if n <= 1 || c == T::ZERO {
        return;
    }
    if c == T::ONE {
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let y = xs[j + 1];
                xs[j].wrapping_add_assign(y);
            }
        }
    } else {
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let y = c.wrapping_mul(xs[j + 1]);
                xs[j].wrapping_add_assign(y);
            }
        }
    }
    mask_coefficients(xs, pow);
}}

// The coefficients of $(x + c)^m$ modulo 2^pow, $\binom{m}{j} c^{m - j}$, without big integers.
// Each binomial coefficient is held as an odd part, modulo 2^W, times a power of 2: going from
// $\binom{m}{j - 1}$ to $\binom{m}{j}$ multiplies by $m + 1 - j$ and divides by $j$, so the odd
// part is multiplied by the odd part of $m + 1 - j$ and by the inverse of the odd part of $j$,
// which exists modulo 2^W because it is odd, and the exponent changes by the difference of their
// 2-adic valuations.
fn binomial_row<T: PrimitiveUnsigned>(m: usize, c: T, pow: u64) -> Vec<T> {
    let mut out = Vec::with_capacity(m + 1);
    out.push(T::ONE);
    let mut odd = T::ONE;
    let mut exponent = 0u64;
    for j in 1..=m {
        let a = m + 1 - j;
        let a_zeros = u64::from(a.trailing_zeros());
        let j_zeros = u64::from(j.trailing_zeros());
        odd.wrapping_mul_assign(T::wrapping_from(a >> a_zeros));
        let j_odd = T::wrapping_from(j >> j_zeros);
        // An odd number is invertible modulo 2^W.
        odd.wrapping_mul_assign(j_odd.mod_power_of_2_inverse(T::WIDTH).unwrap());
        // A binomial coefficient is an integer, so its 2-adic valuation is never negative.
        exponent = exponent + a_zeros - j_zeros;
        out.push(if exponent >= pow {
            T::ZERO
        } else {
            odd << exponent
        });
    }
    if c != T::ONE {
        // power is c^(m - j) when it multiplies binomial j.
        let mut power = c;
        for b in out[..m].iter_mut().rev() {
            b.wrapping_mul_assign(power);
            power.wrapping_mul_assign(c);
        }
    }
    mask_coefficients(&mut out, pow);
    out
}

// Shifts the polynomial with coefficients `xs` by `c` modulo 2^pow, splitting it when that is
// faster than Horner's rule.
crate_test_fn! {mod_power_of_2_taylor_shift_divide_and_conquer<T: PrimitiveUnsigned>(
    xs: &mut [T],
    c: T,
    pow: u64,
) {
    if c == T::ZERO || xs.len() < split_cutoff(c, pow) {
        mod_power_of_2_taylor_shift_horner(xs, c, pow);
    } else {
        mod_power_of_2_taylor_shift_divide_and_conquer_split(xs, c, pow);
    }
}}

// The divide-and-conquer step: splits `xs` as $p_0 + x^m p_1$, shifts both halves with
// `mod_power_of_2_taylor_shift_divide_and_conquer`, and returns $p_0(x + c) + (x + c)^m p_1(x + c)$
// modulo 2^pow. `xs` must have length at least 2.
crate_test_fn! {mod_power_of_2_taylor_shift_divide_and_conquer_split<T: PrimitiveUnsigned>(
    xs: &mut [T],
    c: T,
    pow: u64,
) {
    let len = xs.len();
    let len_lo = len >> 1;
    let (xs_lo, xs_hi) = xs.split_at_mut(len_lo);
    mod_power_of_2_taylor_shift_divide_and_conquer(xs_lo, c, pow);
    mod_power_of_2_taylor_shift_divide_and_conquer(xs_hi, c, pow);
    let binomials = binomial_row(len_lo, c, pow);
    let mut product = vec![T::ZERO; len];
    mod_power_of_2_mul_to_out(&mut product, &binomials, xs_hi, pow);
    let (product_lo, product_hi) = product.split_at(len_lo);
    add_wrapping_assign(xs_lo, product_lo);
    mask_coefficients(xs_lo, pow);
    xs_hi.copy_from_slice(product_hi);
}}

fn assert_valid<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, c: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2TaylorShift<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Shifts an [`UnsignedPolynomial`] by a number $c$ modulo $2^k$, replacing $x$ with $x + c$,
    /// taking the polynomial by value. The coefficients and $c$ must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// All the arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. Long polynomials are split in two,
    /// both halves are shifted recursively, and the high half is multiplied by $(x + c)^m$; short
    /// ones are shifted with Horner's rule. The leading coefficient does not change, so neither
    /// does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if `c` or any coefficient of `self` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShift;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_power_of_2_taylor_shift(10, 4).to_string(),
    ///     "x^2+7*x+4"
    /// );
    ///
    /// // Modulo 2, the binomial coefficients reduce to the parity of Pascal's triangle.
    /// let p = UnsignedPolynomial::<u64>::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_power_of_2_taylor_shift(1, 1).to_string(),
    ///     "x^3+x^2+x+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_taylor_shift(mut self, c: T, pow: u64) -> Self {
        self.mod_power_of_2_taylor_shift_assign(c, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2TaylorShift<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Shifts an [`UnsignedPolynomial`] by a number $c$ modulo $2^k$, replacing $x$ with $x + c$,
    /// taking the polynomial by reference. The coefficients and $c$ must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// All the arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. Long polynomials are split in two,
    /// both halves are shifted recursively, and the high half is multiplied by $(x + c)^m$; short
    /// ones are shifted with Horner's rule. The leading coefficient does not change, so neither
    /// does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if `c` or any coefficient of `self` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShift;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_taylor_shift(10, 4).to_string(),
    ///     "x^2+7*x+4"
    /// );
    ///
    /// // Modulo 2, the binomial coefficients reduce to the parity of Pascal's triangle.
    /// let p = UnsignedPolynomial::<u64>::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_taylor_shift(1, 1).to_string(),
    ///     "x^3+x^2+x+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_taylor_shift(self, c: T, pow: u64) -> UnsignedPolynomial<T> {
        self.clone().mod_power_of_2_taylor_shift(c, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2TaylorShiftAssign<T> for UnsignedPolynomial<T> {
    /// Shifts an [`UnsignedPolynomial`] by a number $c$ modulo $2^k$ in place, replacing $x$ with
    /// $x + c$. The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// All the arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. Long polynomials are split in two,
    /// both halves are shifted recursively, and the high half is multiplied by $(x + c)^m$; short
    /// ones are shifted with Horner's rule. The leading coefficient does not change, so neither
    /// does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if `c` or any coefficient of `self` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShiftAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_taylor_shift_assign(10, 4);
    /// assert_eq!(p.to_string(), "x^2+7*x+4");
    ///
    /// let mut p = UnsignedPolynomial::<u64>::from_str("x^3").unwrap();
    /// p.mod_power_of_2_taylor_shift_assign(1, 1);
    /// assert_eq!(p.to_string(), "x^3+x^2+x+1");
    /// ```
    fn mod_power_of_2_taylor_shift_assign(&mut self, c: T, pow: u64) {
        assert_valid(self, c, pow);
        mod_power_of_2_taylor_shift_divide_and_conquer(&mut self.coefficients, c, pow);
    }
}
