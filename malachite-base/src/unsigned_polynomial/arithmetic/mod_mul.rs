// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::mod_mul::{limbs_invert_limb_u64, mod_preinverted_double};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::LeadingZeros;
use alloc::vec;

// Multiplication of polynomials whose coefficients are words reduced modulo a word $m$. As in
// FLINT's `_nmod_poly_mul_classical`, each coefficient of a product is accumulated exactly, in one,
// two, or three words, depending on how large the sum can get, and reduced once. The reduction uses
// multiplication modulo $m$ with precomputed data, so that no two-word division is needed.

// The length of the shorter factor at which Karatsuba multiplication overtakes classical
// multiplication. Measured on an Apple M-series machine, 2026-10, with 64-bit words: about 80 for
// moduli of 16 to 64 bits.
pub(crate) const MOD_MUL_KARATSUBA_THRESHOLD: usize = 80;

// The length at which Karatsuba squaring overtakes classical squaring. Classical squaring computes
// only half the products, so it stays ahead longer: measured as above, about 256.
pub(crate) const MOD_SQUARE_KARATSUBA_THRESHOLD: usize = 256;

// What summing products of coefficients and reducing the sums modulo $m$ needs: $m$, data for
// reducing two-word values modulo $m$, and the number of words in which the sums are accumulated.
// As in FLINT's `_nmod_vec_dot_bound_limbs`, a sum of at most `terms` products of values less than
// $m$ is bounded by `terms` $(m - 1)^2$, and when the bound fits in one or two words, fewer words
// are used.
pub(crate) struct ModData<T: PrimitiveUnsigned> {
    pub(crate) m: T,
    // When `T` has 64 bits: the inverse of $m$, shifted left until its top bit is set, for
    // `mod_preinverted_double`.
    inv: u64,
    // When `T` has more than 64 bits: $2^\text{W} \bmod m$, where W is `T::WIDTH`.
    radix: T,
    // 1, 2, or 3.
    pub(crate) words: u8,
    // When three words are used: the number of products that can be added to a reduced value
    // without overflowing three words, with room to double the sum: $2^{\text{W} - 1}$, or
    // `usize::MAX` if that does not fit.
    pub(crate) max_terms: usize,
}

impl<T: PrimitiveUnsigned> ModData<T> {
    // Sums will have at most `terms` products, which must be positive.
    pub(crate) fn new(m: T, terms: usize) -> Self {
        assert_ne!(m, T::ZERO);
        let words = match T::try_from(terms) {
            Ok(terms) => {
                // (m - 1)^2 terms, in three words.
                let (hi, lo) = T::x_mul_y_to_zz(m - T::ONE, m - T::ONE);
                let carry = T::x_mul_y_to_zz(lo, terms).0;
                let (top, mid) = T::x_mul_y_to_zz(hi, terms);
                let (top, mid) = T::xx_add_yy_to_zz(top, mid, T::ZERO, carry);
                if top != T::ZERO {
                    3
                } else if mid != T::ZERO {
                    2
                } else {
                    1
                }
            }
            Err(_) => 3,
        };
        let inv = if T::WIDTH == u64::WIDTH {
            let m: u64 = m.wrapping_into();
            limbs_invert_limb_u64(m << LeadingZeros::leading_zeros(m))
        } else {
            0
        };
        Self {
            m,
            inv,
            // 2^W mod m = (2^W - m) mod m, computed without overflow.
            radix: if T::WIDTH > u64::WIDTH {
                m.wrapping_neg() % m
            } else {
                T::ZERO
            },
            words,
            max_terms: if T::WIDTH > usize::WIDTH {
                usize::MAX
            } else {
                1 << (T::WIDTH - 1)
            },
        }
    }

    // The value $x_1 2^\text{W} + x_0$ modulo $m$. Words of at most 32 bits are combined into a
    // `u64` and divided once, and 64-bit words are reduced with a precomputed inverse, as by
    // FLINT's `NMOD2_RED2`.
    #[inline]
    pub(crate) fn reduce_2(&self, x1: T, x0: T) -> T {
        let m = self.m;
        if T::WIDTH <= u32::WIDTH {
            let x: u64 = x1.wrapping_into();
            let x0: u64 = x0.wrapping_into();
            let m: u64 = m.wrapping_into();
            T::wrapping_from(((x << T::WIDTH) | x0) % m)
        } else if T::WIDTH == u64::WIDTH {
            T::wrapping_from(mod_preinverted_double::<u64, u128>(
                x1.wrapping_into(),
                x0.wrapping_into(),
                m.wrapping_into(),
                self.inv,
            ))
        } else {
            let data = T::precompute_mod_mul_data(&m);
            (x1 % m)
                .mod_mul_precomputed(self.radix, m, &data)
                .mod_add(x0 % m, m)
        }
    }

    // The value $x_2 2^{2\text{W}} + x_1 2^\text{W} + x_0$ modulo $m$, by Horner's rule, as by
    // FLINT's `NMOD_RED3`.
    #[inline]
    pub(crate) fn reduce(&self, x2: T, x1: T, x0: T) -> T {
        self.reduce_2(self.reduce_2(x2, x1), x0)
    }

    // A sum from `column_sum`, modulo $m$.
    #[inline]
    pub(crate) fn reduce_sum(&self, (x2, x1, x0): (T, T, T)) -> T {
        match self.words {
            1 => x0 % self.m,
            2 => self.reduce_2(x1, x0),
            _ => self.reduce(x2, x1, x0),
        }
    }
}

// Adds the product of `x` and `y` to the three-word accumulator `(a2, a1, a0)`.
#[inline]
pub(crate) fn accumulate<T: PrimitiveUnsigned>(acc: &mut (T, T, T), x: T, y: T) {
    let (hi, lo) = T::x_mul_y_to_zz(x, y);
    let (a2, a1, a0) = *acc;
    *acc = T::xxx_add_yyy_to_zzz(a2, a1, a0, T::ZERO, hi, lo);
}

// The sum of the products of `xs[i]` and `ys[ys.len() - 1 - i]`, for `i` less than `xs.len()`, as a
// three-word value, accumulated in `d.words` words; `xs` and `ys` must have the same length, at
// most the number of terms `d` was made for. When three words are used, whenever the number of
// products would exceed `d.max_terms` the sum so far is reduced modulo $m$, so the result is
// congruent to the true sum and, with room to spare, can be doubled without overflowing.
#[inline]
pub(crate) fn column_sum<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], d: &ModData<T>) -> (T, T, T) {
    match d.words {
        1 => {
            let mut sum = T::ZERO;
            for (&x, &y) in xs.iter().zip(ys.iter().rev()) {
                sum.wrapping_add_assign(x.wrapping_mul(y));
            }
            (T::ZERO, T::ZERO, sum)
        }
        2 => {
            let (mut hi, mut lo) = (T::ZERO, T::ZERO);
            for (&x, &y) in xs.iter().zip(ys.iter().rev()) {
                let (p_hi, p_lo) = T::x_mul_y_to_zz(x, y);
                (hi, lo) = T::xx_add_yy_to_zz(hi, lo, p_hi, p_lo);
            }
            (T::ZERO, hi, lo)
        }
        _ => {
            let mut acc = (T::ZERO, T::ZERO, T::ZERO);
            for (i, (x_chunk, y_chunk)) in xs
                .chunks(d.max_terms)
                .zip(ys.rchunks(d.max_terms))
                .enumerate()
            {
                if i != 0 {
                    acc = (T::ZERO, T::ZERO, d.reduce(acc.0, acc.1, acc.2));
                }
                for (&x, &y) in x_chunk.iter().zip(y_chunk.iter().rev()) {
                    accumulate(&mut acc, x, y);
                }
            }
            acc
        }
    }
}

// Adds each element of `ys` to the element of `xs` at the same index, modulo `m`.
pub(crate) fn mod_add_assign_slice<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        *x = x.mod_add(y, m);
    }
}

// Subtracts each element of `ys` from the element of `xs` at the same index, modulo `m`.
pub(crate) fn mod_sub_assign_slice<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        *x = x.mod_sub(y, m);
    }
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, reduced modulo $m$,
// by schoolbook multiplication, one coefficient at a time. `out` must have length `xs.len() +
// ys.len() - 1`.
pub(crate) fn mod_mul_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    let m = ys.len();
    for (k, o) in out.iter_mut().enumerate() {
        let start = k.saturating_sub(m - 1);
        let stop = core::cmp::min(k, n - 1);
        let acc = column_sum(&xs[start..=stop], &ys[k - stop..=k - start], d);
        *o = d.reduce_sum(acc);
    }
}

// The scratch length needed by `mod_mul_karatsuba_balanced` for factors of length `n`.
pub(crate) const fn mod_karatsuba_scratch_len(mut n: usize, threshold: usize) -> usize {
    let mut len = 0;
    while n >= threshold {
        let c = n - (n >> 1);
        len += (c << 2) - 1;
        n = c;
    }
    len
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, which have the same
// nonzero length $n$, modulo $m$, by Karatsuba multiplication, falling back to schoolbook
// multiplication below the threshold. `out` must have length $2n - 1$.
fn mod_mul_karatsuba_balanced<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_MUL_KARATSUBA_THRESHOLD {
        mod_mul_classical(out, xs, ys, d);
        return;
    }
    // Write x = x_0 + x^h x_1 and y = y_0 + x^h y_1. Then xy = x_0 y_0 + x^h ((x_0 + x_1)(y_0 +
    // y_1) - x_0 y_0 - x_1 y_1) + x^{2h} x_1 y_1.
    let m = d.m;
    let h = n >> 1;
    let c = n - h;
    let two_h = h << 1;
    let (x0, x1) = xs.split_at(h);
    let (y0, y1) = ys.split_at(h);
    let (x_sum, scratch) = scratch.split_at_mut(c);
    let (y_sum, scratch) = scratch.split_at_mut(c);
    let (middle, scratch) = scratch.split_at_mut((c << 1) - 1);
    {
        let (low, high) = out.split_at_mut(two_h);
        mod_mul_karatsuba_balanced(&mut low[..two_h - 1], x0, y0, d, scratch);
        low[two_h - 1] = T::ZERO;
        mod_mul_karatsuba_balanced(high, x1, y1, d, scratch);
    }
    x_sum.copy_from_slice(x1);
    mod_add_assign_slice(x_sum, x0, m);
    y_sum.copy_from_slice(y1);
    mod_add_assign_slice(y_sum, y0, m);
    mod_mul_karatsuba_balanced(middle, x_sum, y_sum, d, scratch);
    mod_sub_assign_slice(middle, &out[..two_h - 1], m);
    mod_sub_assign_slice(middle, &out[two_h..], m);
    mod_add_assign_slice(&mut out[h..], middle, m);
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty,
// modulo $m$, by Karatsuba multiplication, falling back to schoolbook multiplication for short
// factors. When the factors' lengths differ, the longer is cut into pieces as long as the shorter,
// and the products of the pieces are added together.
pub(crate) fn mod_mul_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let n = xs.len();
    let m = ys.len();
    if m < MOD_MUL_KARATSUBA_THRESHOLD {
        mod_mul_classical(out, xs, ys, d);
        return;
    }
    let mut scratch = vec![T::ZERO; mod_karatsuba_scratch_len(m, MOD_MUL_KARATSUBA_THRESHOLD)];
    if n == m {
        mod_mul_karatsuba_balanced(out, xs, ys, d, &mut scratch);
        return;
    }
    out.fill(T::ZERO);
    let mut product = vec![T::ZERO; (m << 1) - 1];
    for (k, piece) in xs.chunks(m).enumerate() {
        let product = &mut product[..piece.len() + m - 1];
        if piece.len() == m {
            mod_mul_karatsuba_balanced(product, piece, ys, d, &mut scratch);
        } else {
            mod_mul_karatsuba(product, ys, piece, d);
        }
        mod_add_assign_slice(&mut out[k * m..], product, d.m);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
    assert_eq!(out.len(), xs.len() + ys.len() - 1);
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
// reduced modulo `m`, modulo `m`, by schoolbook multiplication. `out` must have length `xs.len() +
// ys.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_to_out_classical<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_classical(out, xs, ys, &ModData::new(m, xs.len().min(ys.len())));
}}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
// reduced modulo `m`, modulo `m`, by Karatsuba multiplication. `out` must have length `xs.len() +
// ys.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_to_out_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_karatsuba(out, xs, ys, &ModData::new(m, xs.len().min(ys.len())));
}}

/// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
/// reduced modulo `m`, modulo `m`. `out` must have length `xs.len() + ys.len() - 1`, and `m` must
/// be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_mul_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_karatsuba(out, xs, ys, &ModData::new(m, xs.len().min(ys.len())));
}
