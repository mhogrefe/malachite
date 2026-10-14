// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2012, 2016, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::coefficient::{SMALL_FMPZ_BITCOUNT_MAX, small_values};
use crate::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::TWICE_WIDTH;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::platform::{Limb, SignedDoubleLimb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExact, DivExactAssign, NegAssign, XXXAddYYYToZZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{TaylorShift, TaylorShiftAssign};

// The side of the square blocks in which `sweep` does its additions when the elements are large, to
// improve cache locality.
const BLOCK_SIZE: usize = 32;

// The length from which `sweep` blocks its additions when the elements are large.
const MIN_BLOCKED_LEN: usize = BLOCK_SIZE << 1;

// Does the additions of Horner's rule for a shift by 1 on `n` elements: for `i` from `n - 2` down
// to 0, and `j` from `i` to `n - 2`, adds element `j + 1` into element `j`, which `add_next(xs, j)`
// does. With `blocked`, the same additions are done in square blocks of `BLOCK_SIZE`, an order that
// still adds into each element only after the element after it is final, so the result is the same.
//
// `n` must be at least 2.
fn sweep<T: ?Sized>(xs: &mut T, n: usize, blocked: bool, mut add_next: impl FnMut(&mut T, usize)) {
    if blocked {
        for ii in (0..n - 1).rev().step_by(BLOCK_SIZE) {
            for jj in (0..n - ii - 1).step_by(BLOCK_SIZE) {
                for i in (ii.saturating_sub(const { BLOCK_SIZE - 1 })..=ii).rev() {
                    for j in jj..(jj + BLOCK_SIZE).min(n - i - 1) {
                        add_next(xs, i + j);
                    }
                }
            }
        }
    } else {
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                add_next(xs, j);
            }
        }
    }
}

// Element `j` of `xs`, mutably, and element `j + 1`.
fn pair_mut<T>(xs: &mut [T], j: usize) -> (&mut T, &T) {
    let (x, y) = xs[j..=j + 1].split_at_mut(1);
    (&mut x[0], &y[0])
}

// Writes `x` to `out` in two's complement, sign-extended to fill all of `out`, which must be long
// enough.
//
// This is equivalent to `fmpz_get_signed_ui_array` from `fmpz.h`, FLINT 3.6.0.
fn integer_to_padded_twos_complement_limbs(x: &Integer, out: &mut [Limb]) {
    let limbs = x.to_twos_complement_limbs_asc();
    let (out_lo, out_hi) = out.split_at_mut(limbs.len());
    out_lo.copy_from_slice(&limbs);
    out_hi.fill(if *x < 0u32 { Limb::MAX } else { 0 });
}

// Shifts the polynomial with coefficients `xs` by `c` with Horner's rule, which takes $O(n^2)$
// additions. A shift by 1 is done with the smallest representation that holds every partial sum: a
// signed word, a signed double word, a fixed number of limbs in two's complement, or an `Integer`.
// Any other shift is reduced to a shift by 1: a shift by -1 by negating the odd-index coefficients
// before and after, and a shift by any other `c` by scaling coefficient `i` by $c^i$ before and
// dividing it by $c^i$ after, except for at most 10 coefficients, which are shifted by `c`
// directly.
//
// This is equivalent to `_fmpz_poly_taylor_shift_horner` from `fmpz_poly/taylor_shift_horner.c`,
// FLINT 3.6.0.
crate_test_fn! {taylor_shift_horner(xs: &mut [Integer], c: &Integer) {
    let n = xs.len();
    if n <= 1 || *c == 0u32 {
        return;
    }
    if *c != 1u32 {
        if n <= 4 || (*c != -1i32 && n <= 10) {
            if *c == -1i32 {
                sweep(xs, n, false, |xs, j| {
                    let (x, y) = pair_mut(xs, j);
                    *x -= y;
                });
            } else {
                sweep(xs, n, false, |xs, j| {
                    let (x, y) = pair_mut(xs, j);
                    x.add_mul_assign(y, c);
                });
            }
        } else if *c == -1i32 {
            for x in xs.iter_mut().skip(1).step_by(2) {
                x.neg_assign();
            }
            taylor_shift_horner(xs, &Integer::ONE);
            for x in xs.iter_mut().skip(1).step_by(2) {
                x.neg_assign();
            }
        } else {
            // power is c^i when it multiplies coefficient i.
            let mut power = c.clone();
            xs[1] *= &power;
            for x in &mut xs[2..] {
                power *= c;
                *x *= &power;
            }
            taylor_shift_horner(xs, &Integer::ONE);
            for i in (1..n).rev() {
                xs[i].div_exact_assign(&power);
                if i > 2 {
                    power.div_exact_assign(c);
                } else if i == 2 {
                    power.clone_from(c);
                }
            }
        }
        return;
    }
    let bits = vec_max_bits(xs).0;
    // Every partial sum is a sum of at most 2^(n - 1) coefficients, so it has fewer than `out_bits`
    // bits.
    let out_bits = bits + u64::wrapping_from(n) + 1;
    if out_bits <= SMALL_FMPZ_BITCOUNT_MAX {
        let mut sums: Vec<SignedLimb> = small_values(xs);
        sweep(&mut sums[..], n, false, |sums, j| sums[j] += sums[j + 1]);
        for (x, sum) in xs.iter_mut().zip(sums) {
            *x = Integer::from(sum);
        }
    } else if n >= 5 && out_bits < TWICE_WIDTH {
        let mut sums: Vec<SignedDoubleLimb> =
            xs.iter().map(SignedDoubleLimb::wrapping_from).collect();
        sweep(&mut sums[..], n, false, |sums, j| sums[j] += sums[j + 1]);
        for (x, sum) in xs.iter_mut().zip(sums) {
            *x = Integer::from(sum);
        }
    } else if n >= 3 + usize::wrapping_from(bits.significant_bits())
        && bits <= const { 100 * Limb::WIDTH }
    {
        // Each partial sum is held in `d` limbs, in two's complement.
        let d = usize::wrapping_from(out_bits.div_ceil(Limb::WIDTH));
        let mut limbs = vec![0; d * n];
        for (x, out) in xs.iter().zip(limbs.chunks_exact_mut(d)) {
            integer_to_padded_twos_complement_limbs(x, out);
        }
        if d == 3 {
            sweep(&mut limbs[..], n, false, |limbs, j| {
                let (x, y) = limbs[3 * j..3 * j + 6].split_at_mut(3);
                let (z_2, z_1, z_0) = Limb::xxx_add_yyy_to_zzz(x[2], x[1], x[0], y[2], y[1], y[0]);
                x[2] = z_2;
                x[1] = z_1;
                x[0] = z_0;
            });
        } else {
            sweep(&mut limbs[..], n, n >= MIN_BLOCKED_LEN, |limbs, j| {
                let (x, y) = limbs[d * j..d * (j + 2)].split_at_mut(d);
                limbs_slice_add_same_length_in_place_left(x, y);
            });
        }
        for (x, sum) in xs.iter_mut().zip(limbs.chunks_exact(d)) {
            // This is equivalent to `fmpz_set_signed_ui_array` from `fmpz.h`, FLINT 3.6.0.
            *x = Integer::from_twos_complement_limbs_asc(sum);
        }
    } else {
        sweep(xs, n, n >= MIN_BLOCKED_LEN, |xs, j| {
            let (x, y) = pair_mut(xs, j);
            *x += y;
        });
    }
}}

// Shifts the polynomial with coefficients `xs` by `c` by splitting it as $p_0 + x^m p_1$, shifting
// both halves recursively, and returning $p_0(x + c) + (x + c)^m p_1(x + c)$, with $(x + c)^m$
// expanded by the binomial theorem and the product computed by fast polynomial multiplication.
// Short polynomials, and polynomials whose coefficients are so large that Horner's rule is faster,
// are shifted with `taylor_shift_horner`.
//
// This is equivalent to `_fmpz_poly_taylor_shift_divconquer` from
// `fmpz_poly/taylor_shift_divconquer.c`, FLINT 3.6.0, with one thread.
crate_test_fn! {taylor_shift_divide_and_conquer(xs: &mut [Integer], c: &Integer) {
    let len = xs.len();
    if len < 64 || *c == 0u32 {
        taylor_shift_horner(xs, c);
        return;
    }
    // The length from which splitting beats Horner's rule, as measured on this machine (FLINT's
    // cutoff grows with the square root of the coefficient size, which is too slow here). A shift
    // by 1 or -1 with word-sized coefficients splits as soon as Horner's partial sums no longer fit
    // in two words, where Horner's rule becomes several times slower.
    let bits = vec_max_bits(xs).0;
    let cutoff = if bits <= Limb::WIDTH && (*c == 1u32 || *c == -1i32) {
        max(64, TWICE_WIDTH - bits)
    } else if bits <= Limb::WIDTH || c.significant_bits() >= Limb::WIDTH {
        160
    } else {
        256
    };
    if u64::wrapping_from(len) < cutoff {
        taylor_shift_horner(xs, c);
        return;
    }
    taylor_shift_divide_and_conquer_split(xs, c);
}}

// The divide-and-conquer step of `taylor_shift_divide_and_conquer`, applied to `xs` whatever its
// length, with the halves shifted by `taylor_shift_divide_and_conquer`. `xs` must have length at
// least 2.
crate_test_fn! {taylor_shift_divide_and_conquer_split(xs: &mut [Integer], c: &Integer) {
    let len = xs.len();
    let len_lo = len >> 1;
    let (xs_lo, xs_hi) = xs.split_at_mut(len_lo);
    taylor_shift_divide_and_conquer(xs_lo, c);
    taylor_shift_divide_and_conquer(xs_hi, c);
    // The coefficients of (x + c)^len_lo, by the binomial theorem. Recomputing them at every node
    // costs only O(len) multiplications, little next to the product.
    let mut binomials = vec![Integer::ZERO; len_lo + 1];
    binomials[0] = Integer::ONE;
    for k in 1..=len_lo {
        binomials[k] = if k > len_lo - k {
            binomials[len_lo - k].clone()
        } else {
            (&binomials[k - 1] * Integer::from(len_lo + 1 - k)).div_exact(Integer::from(k))
        };
    }
    if *c != 1u32 {
        if *c == -1i32 {
            for b in binomials[..len_lo].iter_mut().rev().step_by(2) {
                b.neg_assign();
            }
        } else {
            // power is c^(len_lo - k) when it multiplies binomial k.
            let mut power = c.clone();
            for b in binomials[..len_lo].iter_mut().rev() {
                *b *= &power;
                power *= c;
            }
        }
    }
    let mut product = vec![Integer::ZERO; len];
    mul_greater_to_out(&mut product, &binomials, xs_hi);
    let (product_lo, product_hi) = product.split_at_mut(len_lo);
    for (x, y) in xs_lo.iter_mut().zip(product_lo) {
        *x += core::mem::take(y);
    }
    xs_hi.swap_with_slice(product_hi);
}}

// Shifts the polynomial with coefficients `xs` by `c`.
//
// This is equivalent to `_fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
crate_test_fn! {taylor_shift_in_place(xs: &mut [Integer], c: &Integer) {
    if xs.len() < 64 {
        taylor_shift_horner(xs, c);
    } else {
        taylor_shift_divide_and_conquer(xs, c);
    }
}}

impl TaylorShift<&Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Shifts an [`IntegerPolynomial`] by an [`Integer`] $c$, replacing $x$ with $x + c$, taking
    /// both by reference.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$. Short polynomials, and polynomials with very large
    /// coefficients, are shifted with Horner's rule, using words, double words, or fixed-length
    /// limb arrays instead of [`Integer`]s when a shift by 1 allows it; otherwise the polynomial is
    /// split in two, both halves are shifted recursively, and the high half is multiplied by $(x +
    /// c)^m$, expanded with the binomial theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result: Horner's rule only runs on pieces whose
    /// length is bounded by a constant, so the cost is that of the polynomial multiplications.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((&p).taylor_shift(&Integer::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     (&p).taylor_shift(&Integer::NEGATIVE_ONE).to_string(),
    ///     "x^2-5*x+6"
    /// );
    ///
    /// let p = IntegerPolynomial::from_str("x^3+3*x^2+3*x+1").unwrap();
    /// assert_eq!((&p).taylor_shift(&Integer::NEGATIVE_ONE).to_string(), "x^3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
    fn taylor_shift(self, c: &Integer) -> IntegerPolynomial {
        let mut coefficients = self.coefficients.clone();
        taylor_shift_in_place(&mut coefficients, c);
        IntegerPolynomial { coefficients }
    }
}

impl TaylorShift<Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Shifts an [`IntegerPolynomial`] by an [`Integer`] $c$, replacing $x$ with $x + c$, taking
    /// the polynomial by reference and $c$ by value.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$. Short polynomials, and polynomials with very large
    /// coefficients, are shifted with Horner's rule, using words, double words, or fixed-length
    /// limb arrays instead of [`Integer`]s when a shift by 1 allows it; otherwise the polynomial is
    /// split in two, both halves are shifted recursively, and the high half is multiplied by $(x +
    /// c)^m$, expanded with the binomial theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result: Horner's rule only runs on pieces whose
    /// length is bounded by a constant, so the cost is that of the polynomial multiplications.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((&p).taylor_shift(Integer::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     (&p).taylor_shift(Integer::NEGATIVE_ONE).to_string(),
    ///     "x^2-5*x+6"
    /// );
    ///
    /// let p = IntegerPolynomial::from_str("x^3+3*x^2+3*x+1").unwrap();
    /// assert_eq!((&p).taylor_shift(Integer::NEGATIVE_ONE).to_string(), "x^3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
    #[inline]
    fn taylor_shift(self, c: Integer) -> IntegerPolynomial {
        self.taylor_shift(&c)
    }
}

impl TaylorShift<&Integer> for IntegerPolynomial {
    type Output = Self;

    /// Shifts an [`IntegerPolynomial`] by an [`Integer`] $c$, replacing $x$ with $x + c$, taking
    /// the polynomial by value and $c$ by reference.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$. Short polynomials, and polynomials with very large
    /// coefficients, are shifted with Horner's rule, using words, double words, or fixed-length
    /// limb arrays instead of [`Integer`]s when a shift by 1 allows it; otherwise the polynomial is
    /// split in two, both halves are shifted recursively, and the high half is multiplied by $(x +
    /// c)^m$, expanded with the binomial theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result: Horner's rule only runs on pieces whose
    /// length is bounded by a constant, so the cost is that of the polynomial multiplications.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!(p.clone().taylor_shift(&Integer::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     p.clone().taylor_shift(&Integer::NEGATIVE_ONE).to_string(),
    ///     "x^2-5*x+6"
    /// );
    ///
    /// let p = IntegerPolynomial::from_str("x^3+3*x^2+3*x+1").unwrap();
    /// assert_eq!(
    ///     p.clone().taylor_shift(&Integer::NEGATIVE_ONE).to_string(),
    ///     "x^3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
    #[inline]
    fn taylor_shift(mut self, c: &Integer) -> Self {
        self.taylor_shift_assign(c);
        self
    }
}

impl TaylorShift<Integer> for IntegerPolynomial {
    type Output = Self;

    /// Shifts an [`IntegerPolynomial`] by an [`Integer`] $c$, replacing $x$ with $x + c$, taking
    /// both by value.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$. Short polynomials, and polynomials with very large
    /// coefficients, are shifted with Horner's rule, using words, double words, or fixed-length
    /// limb arrays instead of [`Integer`]s when a shift by 1 allows it; otherwise the polynomial is
    /// split in two, both halves are shifted recursively, and the high half is multiplied by $(x +
    /// c)^m$, expanded with the binomial theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result: Horner's rule only runs on pieces whose
    /// length is bounded by a constant, so the cost is that of the polynomial multiplications.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!(p.clone().taylor_shift(Integer::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     p.clone().taylor_shift(Integer::NEGATIVE_ONE).to_string(),
    ///     "x^2-5*x+6"
    /// );
    ///
    /// let p = IntegerPolynomial::from_str("x^3+3*x^2+3*x+1").unwrap();
    /// assert_eq!(
    ///     p.clone().taylor_shift(Integer::NEGATIVE_ONE).to_string(),
    ///     "x^3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
    #[inline]
    fn taylor_shift(mut self, c: Integer) -> Self {
        self.taylor_shift_assign(&c);
        self
    }
}

impl TaylorShiftAssign<&Integer> for IntegerPolynomial {
    /// Shifts an [`IntegerPolynomial`] by an [`Integer`] $c$ in place, replacing $x$ with $x + c$,
    /// taking $c$ by reference.
    ///
    /// $$
    /// p \\gets p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$. Short polynomials, and polynomials with very large
    /// coefficients, are shifted with Horner's rule, using words, double words, or fixed-length
    /// limb arrays instead of [`Integer`]s when a shift by 1 allows it; otherwise the polynomial is
    /// split in two, both halves are shifted recursively, and the high half is multiplied by $(x +
    /// c)^m$, expanded with the binomial theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result: Horner's rule only runs on pieces whose
    /// length is bounded by a constant, so the cost is that of the polynomial multiplications.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::polynomial::TaylorShiftAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.taylor_shift_assign(&Integer::ONE);
    /// assert_eq!(p.to_string(), "x^2-x");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+3*x^2+3*x+1").unwrap();
    /// p.taylor_shift_assign(&Integer::NEGATIVE_ONE);
    /// assert_eq!(p.to_string(), "x^3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
    #[inline]
    fn taylor_shift_assign(&mut self, c: &Integer) {
        taylor_shift_in_place(&mut self.coefficients, c);
    }
}

impl TaylorShiftAssign<Integer> for IntegerPolynomial {
    /// Shifts an [`IntegerPolynomial`] by an [`Integer`] $c$ in place, replacing $x$ with $x + c$,
    /// taking $c$ by value.
    ///
    /// $$
    /// p \\gets p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$. Short polynomials, and polynomials with very large
    /// coefficients, are shifted with Horner's rule, using words, double words, or fixed-length
    /// limb arrays instead of [`Integer`]s when a shift by 1 allows it; otherwise the polynomial is
    /// split in two, both halves are shifted recursively, and the high half is multiplied by $(x +
    /// c)^m$, expanded with the binomial theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result: Horner's rule only runs on pieces whose
    /// length is bounded by a constant, so the cost is that of the polynomial multiplications.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::polynomial::TaylorShiftAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.taylor_shift_assign(Integer::ONE);
    /// assert_eq!(p.to_string(), "x^2-x");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+3*x^2+3*x+1").unwrap();
    /// p.taylor_shift_assign(Integer::NEGATIVE_ONE);
    /// assert_eq!(p.to_string(), "x^3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_taylor_shift` from `fmpz_poly/taylor_shift.c`, FLINT 3.6.0.
    #[inline]
    fn taylor_shift_assign(&mut self, c: Integer) {
        self.taylor_shift_assign(&c);
    }
}
