// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `getbits`, `MPN_REDC_1`, `win_size`, `redcify`, `mpn_powm`, and `mpz_powm` contributed to
//      the GNU project by Torbjörn Granlund.
//
//      Copyright (C) 2000-2002, 2004, 2007–2012 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_add_to_out_aliased,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::div_exact::{
    limbs_modular_invert, limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use crate::natural::arithmetic::div_mod::limbs_div_limb_to_out_mod;
use crate::natural::arithmetic::mod_op::limbs_mod_to_out;
use crate::natural::arithmetic::mod_power_of_2_pow::limbs_pow_low;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::arithmetic::mul::mul_mod::{
    limbs_mul_mod_base_pow_n_minus_1, limbs_mul_mod_base_pow_n_minus_1_next_size,
    limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out_basecase, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::arithmetic::square::{
    limbs_square_to_out, limbs_square_to_out_basecase, limbs_square_to_out_scratch_len,
};
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_to_out,
};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::logic::bit_access::limbs_get_bit;
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::{Limb, MUL_TOOM22_THRESHOLD, SQR_BASECASE_THRESHOLD, SQR_TOOM2_THRESHOLD};
use alloc::vec::Vec;
use core::cmp::{max, min, Ordering::*};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    ModPow, ModPowAssign, ModPowerOf2, ModPowerOf2Assign, Parity, PowerOf2, WrappingNegAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero};

// Equivalent to limbs_slice_get_bits(xs, end.saturating_sub(len), end)[0]
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `getbits` from `mpn/generic/powm.c` and `mpn/generic/powlo.c`, GMP 6.2.1.
// Investigate changes from 6.1.2?
pub(crate) fn get_bits(xs: &[Limb], mut end: u64, len: u64) -> usize {
    usize::exact_from(if end < len {
        xs[0].mod_power_of_2(end)
    } else {
        end -= len;
        let i = usize::exact_from(end >> Limb::LOG_WIDTH);
        end &= Limb::WIDTH_MASK;
        let mut bits = xs[i] >> end;
        let coend = Limb::WIDTH - end;
        if coend < len {
            bits += xs[i + 1] << coend;
        }
        bits.mod_power_of_2(len)
    })
}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ms.len()`.
//
// This is equivalent to `mpn_redc_1` from `mpn/generic/redc_1.c`, GMP 6.2.1.
#[allow(clippy::redundant_slicing)]
fn limbs_redc_limb_raw(out: &mut [Limb], xs: &mut [Limb], ms: &[Limb], m_inv: Limb) -> bool {
    let len = ms.len();
    assert_ne!(len, 0);
    let xs = &mut xs[..len << 1];
    let mut xs_tail = &mut xs[..]; // force borrow rather than move
    for _ in 0..len {
        let product = xs_tail[0].wrapping_mul(m_inv);
        let carry =
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut xs_tail[..len], ms, product);
        assert_eq!(xs_tail[0], 0);
        xs_tail[0] = carry;
        xs_tail = &mut xs_tail[1..];
    }
    let (xs_lo, xs_hi) = xs.split_at(len);
    limbs_add_same_length_to_out(out, xs_hi, xs_lo)
}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ms.len()`.
//
// This is equivalent to `MPN_REDC_1` from `mpn/generic/powm.c`, GMP 6.2.1. Investigate changes from
// 6.1.2?
fn limbs_redc_limb(out: &mut [Limb], xs: &mut [Limb], ms: &[Limb], m_inv: Limb) {
    if limbs_redc_limb_raw(out, xs, ms, m_inv) {
        limbs_sub_same_length_in_place_left(&mut out[..ms.len()], ms);
    }
}

const WIDTH_LIMITS: [u64; 10] = [7, 25, 81, 241, 673, 1793, 4609, 11521, 28161, u64::MAX];

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `win_size` from `mpn/generic/powm.c`, 6.2.1. Investigate changes from
// 6.1.2?
pub(crate) fn get_window_size(width: u64) -> u64 {
    u64::wrapping_from(
        WIDTH_LIMITS
            .iter()
            .position(|&limit| width <= limit)
            .unwrap()
            + 1,
    )
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ms.len()`.
//
// This is equivalent to `mpn_redc_n` from `mpn/generic/redc_n.c`, GMP 6.2.1.
fn limbs_redc(out: &mut [Limb], xs: &[Limb], ms: &[Limb], is: &[Limb]) {
    let ms_len = ms.len();
    assert!(ms_len > 8);
    let n = limbs_mul_mod_base_pow_n_minus_1_next_size(ms_len);
    let mut scratch =
        vec![0; limbs_mul_mod_base_pow_n_minus_1_scratch_len(n, ms_len, ms_len) + ms_len + n];
    let (scratch_0, scratch) = scratch.split_at_mut(ms_len);
    limbs_mul_low_same_length(scratch_0, &xs[..ms_len], &is[..ms_len]);
    let (scratch_1, scratch_2) = scratch.split_at_mut(n);
    limbs_mul_mod_base_pow_n_minus_1(scratch_1, n, scratch_0, ms, scratch_2);
    let two_ms_len = ms_len << 1;
    assert!(two_ms_len > n);
    let m = two_ms_len - n;
    let carry = limbs_sub_same_length_to_out(scratch_2, &scratch_1[..m], &xs[..m]);
    let scratch = &mut scratch[..two_ms_len];
    if carry {
        assert!(!limbs_sub_limb_in_place(&mut scratch[m..], 1));
    }
    if limbs_sub_same_length_to_out(out, &xs[ms_len..two_ms_len], &scratch[ms_len..]) {
        limbs_slice_add_same_length_in_place_left(&mut out[..ms_len], ms);
    }
}

// Convert U to REDC form, U_r = B^n * U mod M
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ms.len()`.
//
// This is equivalent to `redcify` from `mpn/generic/powm.c`, 6.2.1. Investigate changes from 6.1.2?
fn to_redc(out: &mut [Limb], xs: &[Limb], ms: &[Limb]) {
    let xs_len = xs.len();
    let ms_len = ms.len();
    if ms_len == 1 {
        let mut scratch = vec![0; (xs_len << 1) + ms_len + 1];
        let (scratch, qs) = scratch.split_at_mut(xs_len + ms_len);
        scratch[ms_len..].copy_from_slice(xs);
        out[0] = limbs_div_limb_to_out_mod(qs, scratch, ms[0]);
    } else {
        let mut scratch = vec![0; xs_len + ms_len];
        scratch[ms_len..].copy_from_slice(xs);
        limbs_mod_to_out(out, &scratch, ms);
    }
}

// TODO tune
const REDC_1_TO_REDC_N_THRESHOLD: usize = 100;

// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_mod_pow_odd_scratch_len(n: usize) -> usize {
    max(limbs_modular_invert_scratch_len(n), n << 1)
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn square_using_basecase_mul(out: &mut [Limb], xs: &[Limb]) {
    limbs_mul_greater_to_out_basecase(out, xs, xs);
}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ms.len()`.
fn limbs_redc_limb_helper(out: &mut [Limb], xs: &mut [Limb], ms: &[Limb], is: &[Limb]) {
    limbs_redc_limb(out, xs, ms, is[0]);
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ms.len()`.
fn limbs_redc_helper(out: &mut [Limb], xs: &mut [Limb], ms: &[Limb], is: &[Limb]) {
    limbs_redc(out, xs, ms, is);
}

// # Worst-case complexity
// Constant time and additional memory.
#[allow(clippy::absurd_extreme_comparisons, clippy::type_complexity)]
fn select_fns(
    ms_len: usize,
) -> (
    &'static dyn Fn(&mut [Limb], &[Limb], &[Limb]),
    &'static dyn Fn(&mut [Limb], &[Limb]),
    &'static dyn Fn(&mut [Limb], &mut [Limb], &[Limb], &[Limb]),
) {
    if REDC_1_TO_REDC_N_THRESHOLD < MUL_TOOM22_THRESHOLD {
        if ms_len < REDC_1_TO_REDC_N_THRESHOLD {
            (
                &limbs_mul_greater_to_out_basecase,
                if REDC_1_TO_REDC_N_THRESHOLD < SQR_BASECASE_THRESHOLD
                    || !(SQR_BASECASE_THRESHOLD..=SQR_TOOM2_THRESHOLD).contains(&ms_len)
                {
                    &square_using_basecase_mul
                } else {
                    &limbs_square_to_out_basecase
                },
                &limbs_redc_limb_helper,
            )
        } else if ms_len < MUL_TOOM22_THRESHOLD {
            (
                &limbs_mul_greater_to_out_basecase,
                if MUL_TOOM22_THRESHOLD < SQR_BASECASE_THRESHOLD
                    || !(SQR_BASECASE_THRESHOLD..=SQR_TOOM2_THRESHOLD).contains(&ms_len)
                {
                    &square_using_basecase_mul
                } else {
                    &limbs_square_to_out_basecase
                },
                &limbs_redc_helper,
            )
        } else {
            (
                &|out, xs, ys| {
                    let mut mul_scratch =
                        vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
                    limbs_mul_same_length_to_out(out, xs, ys, &mut mul_scratch);
                },
                &|out, xs| {
                    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(xs.len())];
                    limbs_square_to_out(out, xs, &mut square_scratch);
                },
                &limbs_redc_helper,
            )
        }
    } else if ms_len < MUL_TOOM22_THRESHOLD {
        (
            &limbs_mul_greater_to_out_basecase,
            if MUL_TOOM22_THRESHOLD < SQR_BASECASE_THRESHOLD
                || !(SQR_BASECASE_THRESHOLD..=SQR_TOOM2_THRESHOLD).contains(&ms_len)
            {
                &square_using_basecase_mul
            } else {
                &limbs_square_to_out_basecase
            },
            &limbs_redc_limb_helper,
        )
    } else {
        (
            &|out, xs, ys| {
                let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
                limbs_mul_same_length_to_out(out, xs, ys, &mut mul_scratch);
            },
            &|out, xs| {
                let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(xs.len())];
                limbs_square_to_out(out, xs, &mut square_scratch);
            },
            if ms_len < REDC_1_TO_REDC_N_THRESHOLD {
                &limbs_redc_limb_helper
            } else {
                &limbs_redc_helper
            },
        )
    }
}

// Given the limbs of $x$, $E$, and odd $m$, writes the limbs of $x^E \mod m$ to an output slice.
//
// `xs`, `es`, and `ms` must be nonempty and their last elements must be nonzero. $m$ must be odd,
// $E$ must be greater than 1, and `out` must be at least as long as `ms`. It is not required than
// `xs` be less than `ms`.
//
// # Worst-case complexity
// $T(n, m) = O(mn \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ms.len()`, and $m$ is `es.len()`.
//
// # Panics
// Panics if `xs`, `es`, or `ms` are empty, if `xs` is longer than `ms`, if the first element of
// `ms` is even, or if $E$ less than 2.
//
// This is equivalent to `mpn_powm` from `mpn/generic/powm.c`, GMP 6.2.1.
pub_test! {limbs_mod_pow_odd(
    out: &mut [Limb],
    xs: &[Limb],
    es: &[Limb],
    ms: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let es_len = es.len();
    let ms_len = ms.len();
    assert_ne!(xs_len, 0);
    assert_ne!(es_len, 0);
    if es_len == 1 {
        assert!(es[0] > 1);
    }
    assert!(ms[0].odd());
    let out = &mut out[..ms_len];
    let width = limbs_significant_bits(es);
    let window_size = get_window_size(width);
    let mut small_is = [0; 2];
    let mut is_vec;
    let is: &mut [Limb];
    let redc_fn: &dyn Fn(&mut [Limb], &mut [Limb], &[Limb], &[Limb]);
    if ms_len < REDC_1_TO_REDC_N_THRESHOLD {
        is = &mut small_is;
        is[0] = limbs_modular_invert_limb(ms[0]);
        is[0].wrapping_neg_assign();
        redc_fn = &limbs_redc_limb_helper;
    } else {
        is_vec = vec![0; ms_len];
        is = &mut is_vec;
        limbs_modular_invert(is, ms, scratch);
        redc_fn = &limbs_redc_helper;
    }
    let mut powers = vec![0; ms_len << (window_size - 1)];
    let mut powers: Vec<&mut [Limb]> = powers.chunks_mut(ms_len).collect();
    to_redc(powers[0], xs, ms);
    // Store x ^ 2 at `out`.
    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(powers[0].len())];
    limbs_square_to_out(scratch, powers[0], &mut square_scratch);
    redc_fn(out, scratch, ms, is);
    // Precompute odd powers of x and put them in `powers`.
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(out.len())];
    for i in 1..usize::power_of_2(window_size - 1) {
        let (powers_lo, powers_hi) = powers.split_at_mut(i);
        limbs_mul_same_length_to_out(scratch, powers_lo[i - 1], out, &mut mul_scratch);
        redc_fn(powers_hi[0], scratch, ms, is);
    }
    let exp_bits = get_bits(es, width, window_size);
    let mut bit_index = if width < window_size {
        fail_on_untested_path("limbs_mod_pow_odd, width < window_size");
        0
    } else {
        width - window_size
    };
    let trailing_zeros = TrailingZeros::trailing_zeros(Limb::exact_from(exp_bits));
    bit_index += trailing_zeros;
    out.copy_from_slice(powers[exp_bits >> trailing_zeros >> 1]);
    let (mul_fn, square_fn, reduce_fn) = select_fns(ms_len);
    'outer: while bit_index != 0 {
        while !limbs_get_bit(es, bit_index - 1) {
            square_fn(scratch, out);
            reduce_fn(out, scratch, ms, is);
            bit_index -= 1;
            if bit_index == 0 {
                break 'outer;
            }
        }
        // The next bit of the exponent is 1. Now extract the largest block of bits <= window_size,
        // and such that the least significant bit is 1.
        let exp_bits = get_bits(es, bit_index, window_size);
        let mut this_window_size = window_size;
        if bit_index < window_size {
            this_window_size -= window_size - bit_index;
            bit_index = 0;
        } else {
            bit_index -= window_size;
        }
        let trailing_zeros = TrailingZeros::trailing_zeros(Limb::exact_from(exp_bits));
        bit_index += trailing_zeros;
        for _ in 0..this_window_size - trailing_zeros {
            square_fn(scratch, out);
            reduce_fn(out, scratch, ms, is);
        }
        mul_fn(scratch, out, powers[exp_bits >> trailing_zeros >> 1]);
        reduce_fn(out, scratch, ms, is);
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(ms_len);
    scratch_lo.copy_from_slice(out);
    slice_set_zero(&mut scratch_hi[..ms_len]);
    redc_fn(out, scratch, ms, is);
    if limbs_cmp_same_length(out, ms) != Less {
        limbs_sub_same_length_in_place_left(out, ms);
    }
}}

// Interpreting a `Vec<Limb>` and two `&[Limb]` as the limbs (in ascending order) of three
// `Natural`s, `x`, `exp`, and `m`, writes the limbs of `x ^ exp` mod `2 ^ m` to an output slice.
// Assumes the input is already reduced mod `m`. No input may be empty or have trailing zeros, the
// exponent must be greater than 1, and the output slice must be at least as long as `ms`.
//
// # Worst-case complexity
// $T(n, m) = O(mn \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `ms.len()`, and $m$ is `es.len()`.
//
// # Panics
// Panics if the exponent has trailing zeros or is 1.
//
// This is equivalent to `mpz_powm` from `mpn/generic/powm.c`, GMP 6.2.1, where `b`, `e`, and `m`
// are non-negative. Investigate changes from 6.1.2?
pub_test! {limbs_mod_pow(out: &mut [Limb], xs: &[Limb], es: &[Limb], ms: &[Limb]) {
    let ms_len = ms.len();
    let es_len = es.len();
    let xs_len = xs.len();
    let mut ms_zero_len = slice_leading_zeros(ms);
    let mut ms = &ms[ms_zero_len..];
    let mut ms_nonzero_len = ms_len - ms_zero_len;
    let mut ms_vec;
    let mut ms_twos = 0;
    if ms[0].even() {
        ms_vec = vec![0; ms_nonzero_len];
        ms_twos = TrailingZeros::trailing_zeros(ms[0]);
        limbs_shr_to_out(&mut ms_vec, &ms[..ms_nonzero_len], ms_twos);
        if ms_vec[ms_nonzero_len - 1] == 0 {
            ms_nonzero_len -= 1;
        }
        ms = &ms_vec;
        ms_zero_len += 1;
    }
    let scratch_len = if ms_zero_len != 0 {
        // We will call both `limbs_mod_pow_odd` and `limbs_pow_low`.
        let max_invert_len = max(ms_zero_len, ms_nonzero_len);
        let invert_scratch_len = limbs_modular_invert_scratch_len(max_invert_len);
        (ms_len << 1) + max(invert_scratch_len, ms_len << 1)
    } else {
        // We will call just `limbs_mod_pow_odd`.
        let invert_scratch_len = limbs_modular_invert_scratch_len(ms_nonzero_len);
        max(invert_scratch_len, ms_len << 1)
    };
    let mut scratch = vec![0; scratch_len];
    limbs_mod_pow_odd(out, xs, es, &ms[..ms_nonzero_len], &mut scratch);
    let mut xs_vec;
    let mut xs = xs;
    if ms_zero_len != 0 {
        if xs_len < ms_zero_len {
            xs_vec = vec![0; ms_zero_len];
            xs_vec[..xs_len].copy_from_slice(xs);
            xs = &xs_vec;
        }
        let mut do_pow_low = true;
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(ms_zero_len);
        if xs[0].even() {
            if es_len > 1 {
                slice_set_zero(scratch_lo);
                do_pow_low = false;
            } else {
                assert_eq!(es_len, 1);
                let t = if ms_twos == 0 {
                    ms_zero_len << Limb::LOG_WIDTH
                } else {
                    ((ms_zero_len - 1) << Limb::LOG_WIDTH) + usize::exact_from(ms_twos)
                };
                // Count number of low zero bits in `xs`, up to 3.
                let bits =
                    (Limb::exact_from(0x1213) >> (xs[0].mod_power_of_2(3) << 1)).mod_power_of_2(2);
                // Note that es[0] * bits might overflow, but that just results in a missed
                // optimization.
                if let Ok(t) = Limb::try_from(t) {
                    if es[0].wrapping_mul(bits) >= t {
                        slice_set_zero(scratch_lo);
                        do_pow_low = false;
                    }
                }
            }
        }
        if do_pow_low {
            scratch_lo.copy_from_slice(&xs[..ms_zero_len]);
            limbs_pow_low(scratch_lo, &es[..es_len], scratch_hi);
        }
        let mut ms_vec;
        if ms_nonzero_len < ms_zero_len {
            ms_vec = vec![0; ms_zero_len];
            ms_vec[..ms_nonzero_len].copy_from_slice(&ms[..ms_nonzero_len]);
            ms = &ms_vec;
        }
        let (scratch_0_1, scratch_2) = scratch.split_at_mut(ms_len << 1);
        let (scratch_0, scratch_1) = scratch_0_1.split_at_mut(ms_len);
        let scratch_0 = &mut scratch_0[..ms_zero_len];
        limbs_modular_invert(scratch_1, &ms[..ms_zero_len], scratch_2);
        limbs_sub_greater_in_place_left(scratch_0, &out[..min(ms_zero_len, ms_nonzero_len)]);
        limbs_mul_low_same_length(scratch_2, &scratch_1[..ms_zero_len], scratch_0);
        if ms_twos != 0 {
            scratch_2[ms_zero_len - 1].mod_power_of_2_assign(ms_twos);
        }
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(ms_zero_len, ms_nonzero_len)];
        limbs_mul_to_out(
            scratch_0_1,
            &scratch_2[..ms_zero_len],
            &ms[..ms_nonzero_len], &mut mul_scratch
        );
        limbs_add_to_out_aliased(out, ms_nonzero_len, &scratch_0_1[..ms_len]);
    }
}}

impl ModPow<Natural, Natural> for Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. All three [`Natural`]s are taken by value.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_pow(Natural::from(13u32), Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_pow(Natural::from(1000u32), Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mod_pow(mut self, exp: Natural, m: Natural) -> Natural {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl<'a> ModPow<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. The first two [`Natural`]s are taken by value and the third by
    /// reference.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_pow(Natural::from(13u32), &Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_pow(Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mod_pow(mut self, exp: Natural, m: &'a Natural) -> Natural {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl<'a> ModPow<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. The first and third [`Natural`]s are taken by value and the
    /// second by reference.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_pow(&Natural::from(13u32), Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_pow(&Natural::from(1000u32), Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mod_pow(mut self, exp: &'a Natural, m: Natural) -> Natural {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl<'a, 'b> ModPow<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. The first [`Natural`] is taken by value and the second and third
    /// by reference.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_pow(&Natural::from(13u32), &Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_pow(&Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mod_pow(mut self, exp: &'a Natural, m: &'b Natural) -> Natural {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl<'a> ModPow<Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`]$m$. The base must be
    /// already reduced modulo $m$. The first [`Natural`] is taken by reference and the second and
    /// third by value.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(Natural::from(13u32), Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(Natural::from(1000u32), Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow(self, mut exp: Natural, mut m: Natural) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, &exp, &m) {
            (_, _, &Natural::ONE) => Natural::ZERO,
            (_, &Natural::ZERO, _) => Natural::ONE,
            (&Natural::ZERO, _, _) => Natural::ZERO,
            (x, &Natural::ONE, _) => x.clone(),
            (&Natural::ONE, _, _) => Natural::ONE,
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                Natural::from(x.mod_pow(u64::wrapping_from(*e), *m))
            }
            _ => {
                let ms = m.promote_in_place();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(&mut out, &self.to_limbs_asc(), exp.promote_in_place(), ms);
                Natural::from_owned_limbs_asc(out)
            }
        }
    }
}

impl<'a, 'b> ModPow<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. The first and third [`Natural`]s are taken by reference and the
    /// second by value.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(Natural::from(13u32), &Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow(self, mut exp: Natural, m: &'b Natural) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, &exp, m) {
            (_, _, &Natural::ONE) => Natural::ZERO,
            (_, &Natural::ZERO, _) => Natural::ONE,
            (&Natural::ZERO, _, _) => Natural::ZERO,
            (x, &Natural::ONE, _) => x.clone(),
            (&Natural::ONE, _, _) => Natural::ONE,
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                Natural::from(x.mod_pow(u64::wrapping_from(*e), *m))
            }
            _ => {
                let ms = m.to_limbs_asc();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(&mut out, &self.to_limbs_asc(), exp.promote_in_place(), &ms);
                Natural::from_owned_limbs_asc(out)
            }
        }
    }
}

impl<'a, 'b> ModPow<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. The first two [`Natural`]s are taken by reference and the third
    /// by value.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(&Natural::from(13u32), Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(&Natural::from(1000u32), Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow(self, exp: &'b Natural, mut m: Natural) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, exp, &m) {
            (_, _, &Natural::ONE) => Natural::ZERO,
            (_, &Natural::ZERO, _) => Natural::ONE,
            (&Natural::ZERO, _, _) => Natural::ZERO,
            (x, &Natural::ONE, _) => x.clone(),
            (&Natural::ONE, _, _) => Natural::ONE,
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                Natural::from(x.mod_pow(u64::wrapping_from(*e), *m))
            }
            _ => {
                let ms = m.promote_in_place();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(&mut out, &self.to_limbs_asc(), &exp.to_limbs_asc(), ms);
                Natural::from_owned_limbs_asc(out)
            }
        }
    }
}

impl<'a, 'b, 'c> ModPow<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$. The base must be
    /// already reduced modulo $m$. All three [`Natural`]s are taken by reference.
    ///
    /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(&Natural::from(13u32), &Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(&Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow(self, exp: &'b Natural, m: &'c Natural) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, exp, m) {
            (_, _, &Natural::ONE) => Natural::ZERO,
            (_, &Natural::ZERO, _) => Natural::ONE,
            (&Natural::ZERO, _, _) => Natural::ZERO,
            (x, &Natural::ONE, _) => x.clone(),
            (&Natural::ONE, _, _) => Natural::ONE,
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                Natural::from(x.mod_pow(u64::wrapping_from(*e), *m))
            }
            _ => {
                let ms = m.to_limbs_asc();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(&mut out, &self.to_limbs_asc(), &exp.to_limbs_asc(), &ms);
                Natural::from_owned_limbs_asc(out)
            }
        }
    }
}

impl ModPowAssign<Natural, Natural> for Natural {
    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$, in place. The
    /// base must be already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken
    /// by value.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(Natural::from(13u32), Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(Natural::from(1000u32), Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow_assign(&mut self, mut exp: Natural, mut m: Natural) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (&mut *self, &exp, &m) {
            (_, _, &Natural::ONE) => *self = Natural::ZERO,
            (_, &Natural::ZERO, _) => *self = Natural::ONE,
            (&mut (Natural::ZERO | Natural::ONE), _, _) | (_, &Natural::ONE, _) => {}
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                x.mod_pow_assign(u64::wrapping_from(*e), *m);
            }
            _ => {
                let ms = m.promote_in_place();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(
                    &mut out,
                    self.promote_in_place(),
                    exp.promote_in_place(),
                    ms,
                );
                *self = Natural::from_owned_limbs_asc(out);
            }
        }
    }
}

impl<'a> ModPowAssign<Natural, &'a Natural> for Natural {
    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$, in place. The
    /// base must be already reduced modulo $m$. The first [`Natural`] on the right-hand side is
    /// taken by value and the second by reference.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(Natural::from(13u32), &Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(Natural::from(1000u32), &Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow_assign(&mut self, mut exp: Natural, m: &'a Natural) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (&mut *self, &exp, m) {
            (_, _, &Natural::ONE) => *self = Natural::ZERO,
            (_, &Natural::ZERO, _) => *self = Natural::ONE,
            (&mut (Natural::ZERO | Natural::ONE), _, _) | (_, &Natural::ONE, _) => {}
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                x.mod_pow_assign(u64::wrapping_from(*e), *m);
            }
            _ => {
                let ms = m.to_limbs_asc();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(
                    &mut out,
                    self.promote_in_place(),
                    exp.promote_in_place(),
                    &ms,
                );
                *self = Natural::from_owned_limbs_asc(out);
            }
        }
    }
}

impl<'a> ModPowAssign<&'a Natural, Natural> for Natural {
    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$, in place. The
    /// base must be already reduced modulo $m$. The first [`Natural`] on the right-hand side is
    /// taken by reference and the second by value.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(&Natural::from(13u32), Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(&Natural::from(1000u32), Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow_assign(&mut self, exp: &'a Natural, mut m: Natural) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (&mut *self, exp, &m) {
            (_, _, &Natural::ONE) => *self = Natural::ZERO,
            (_, &Natural::ZERO, _) => *self = Natural::ONE,
            (&mut (Natural::ZERO | Natural::ONE), _, _) | (_, &Natural::ONE, _) => {}
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                x.mod_pow_assign(u64::wrapping_from(*e), *m);
            }
            _ => {
                let ms = m.promote_in_place();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(&mut out, self.promote_in_place(), &exp.to_limbs_asc(), ms);
                *self = Natural::from_owned_limbs_asc(out);
            }
        }
    }
}

impl<'a, 'b> ModPowAssign<&'a Natural, &'b Natural> for Natural {
    /// Raises a [`Natural`] to a [`Natural`] power modulo a third [`Natural`] $m$, in place. The
    /// base must be already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken
    /// by reference.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^n \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(&Natural::from(13u32), &Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(&Natural::from(1000u32), &Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    #[allow(clippy::match_same_arms)] // matches are order-dependent
    fn mod_pow_assign(&mut self, exp: &'a Natural, m: &'b Natural) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (&mut *self, exp, m) {
            (_, _, &Natural::ONE) => *self = Natural::ZERO,
            (_, &Natural::ZERO, _) => *self = Natural::ONE,
            (&mut (Natural::ZERO | Natural::ONE), _, _) | (_, &Natural::ONE, _) => {}
            (Natural(Small(x)), Natural(Small(e)), Natural(Small(m)))
                if u64::convertible_from(*e) =>
            {
                x.mod_pow_assign(u64::wrapping_from(*e), *m);
            }
            _ => {
                let ms = m.to_limbs_asc();
                let mut out = vec![0; ms.len()];
                limbs_mod_pow(&mut out, self.promote_in_place(), &exp.to_limbs_asc(), &ms);
                *self = Natural::from_owned_limbs_asc(out);
            }
        }
    }
}
