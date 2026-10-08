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

// Dot products of vectors whose elements are words reduced modulo a word $m$. As in FLINT's
// `nmod_vec` dot products, the sum of products is accumulated exactly, in one, two, or three words,
// depending on how large it can get, and reduced once.

// What summing products of vector elements and reducing the sums modulo $m$ needs: $m$, data for
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
