// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_bc_mulmod_bnm1`, `mpn_bc_mulmod_bnp1`, `mpn_mulmod_bnm1`, and
//      `mpn_mulmod_bnm1_next_size` contributed to the GNU project by Niels Möller, Torbjörn
//      Granlund and Marco Bodrato.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len,
};
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_greater_to_out, limbs_sub_limb_in_place,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_same_length_with_borrow_in_in_place_right,
};
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{Parity, RoundToMultipleOfPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_test_zero;

// TODO tune
pub(crate) const MULMOD_BNM1_THRESHOLD: usize = 13;

// # Worst-case complexity
// Constant time and additional memory.
pub(crate) fn limbs_mul_mod_base_pow_n_minus_1_next_size_helper(
    n: usize,
    low_threshold: usize,
) -> usize {
    if n < low_threshold {
        n
    } else if n <= (low_threshold - 1) << 2 {
        n.round_to_multiple_of_power_of_2(1, Ceiling).0
    } else if n <= (low_threshold - 1) << 3 {
        n.round_to_multiple_of_power_of_2(2, Ceiling).0
    } else {
        n.round_to_multiple_of_power_of_2(3, Ceiling).0
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// The result is $O(n)$.
//
// This is equivalent to `mpn_mulmod_bnm1_next_size` from `mpn/generic/mulmod_bnm1.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_mod_base_pow_n_minus_1_next_size(n: usize) -> usize {
    limbs_mul_mod_base_pow_n_minus_1_next_size_helper(
        n,
        MULMOD_BNM1_THRESHOLD,
    )
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// The result is $O(n)$.
//
// This is equivalent to `mpn_mulmod_bnm1_itch` from `gmp-impl.h`, GMP 6.2.1.
pub(crate) const fn limbs_mul_mod_base_pow_n_minus_1_scratch_len(
    n: usize,
    xs_len: usize,
    ys_len: usize,
) -> usize {
    let half_n = n >> 1;
    if xs_len > half_n {
        if ys_len > half_n {
            (n + 2) << 1
        } else {
            n + 4 + half_n
        }
    } else {
        n + 4
    }
}

// Interpreting two equal-length, nonempty slices of `Limb`s as the limbs (in ascending order) of
// two `Natural`s, multiplies the `Natural`s mod `2 ^ (Limb::WIDTH * n) - 1`, where n is the length
// of either slice. The result is semi-normalized: zero is represented as either 0 or `Limb::WIDTH ^
// n - 1`. The limbs of the result are written to `out`. `out` should have length at least n, and
// `scratch` at least 2 * n. This is the basecase algorithm.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths, if `out` or `scratch` are too short, or if the
// input slices are empty.
//
// This is equivalent to `mpn_bc_mulmod_bnm1` from `mpn/generic/mulmod_bnm1.c`, GMP 6.2.1.
fn limbs_mul_mod_base_pow_n_minus_1_basecase(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_ne!(n, 0);
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(n)];
    limbs_mul_same_length_to_out(scratch, xs, ys, &mut mul_scratch);
    split_into_chunks_mut!(scratch, n, [scratch_lo, scratch_hi], _unused);
    if limbs_add_same_length_to_out(out, scratch_lo, scratch_hi) {
        // If carry == 1, then the value of out is at most B ^ n - 2, so there can be no overflow
        // when adding in the carry.
        limbs_slice_add_limb_in_place(&mut out[..n], 1);
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
pub(crate) fn limbs_mul_mod_base_pow_n_plus_1_basecase_helper(out: &mut [Limb], n: usize) {
    split_into_chunks_mut!(out, n, [out_0, out_1], out_2);
    assert_eq!(out_2[1], 0);
    let mut carry = out_2[0];
    assert_ne!(carry, Limb::MAX);
    if limbs_sub_same_length_in_place_left(out_0, out_1) {
        carry += 1;
    }
    out_1[0] = 0;
    assert!(!limbs_slice_add_limb_in_place(&mut out[..=n], carry));
}

// Interpreting the first n + 1 limbs of two slices of `Limb`s as the limbs (in ascending order) of
// two `Natural`s, multiplies the `Natural`s mod `2 ^ (Limb::WIDTH * n) + 1`. The limbs of the
// result are written to `out`, which should have length at least 2 * n + 2.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// # Panics
// Panics if `xs`, `ys`, or `out` are too short, or if n is zero.
//
// This is equivalent to `mpn_bc_mulmod_bnp1` from `mpn/generic/mulmod_bnm1.c`, GMP 6.2.1, where `rp
// == tp`.
fn limbs_mul_mod_base_pow_n_plus_1_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb], n: usize) {
    assert_ne!(0, n);
    let m = n + 1;
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(m)];
    limbs_mul_same_length_to_out(out, &xs[..m], &ys[..m], &mut mul_scratch);
    limbs_mul_mod_base_pow_n_plus_1_basecase_helper(out, n);
}

// Interpreting two nonempty slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
// multiplies the `Natural`s mod `2 ^ (Limb::WIDTH * n) - 1`. The limbs of the result are written to
// `out`.
//
// The result is expected to be 0 if and only if one of the operands already is. Otherwise the class
// 0 mod `(Limb::WIDTH ^ n - 1)` is represented by `2 ^ (n * Limb::WIDTH) - 1`. This should not be a
// problem if `limbs_mul_mod_base_pow_n_minus_1` is used to combine results and obtain a natural
// number when one knows in advance that the final value is less than `2 ^ (n * Limb::WIDTH) - 1`.
// Moreover it should not be a problem if `limbs_mul_mod_base_pow_n_minus_1` is used to compute the
// full product with `xs.len() + ys.len() <= n`, because this condition implies `(2 ^ (Limb::WIDTH *
// xs.len()) - 1)(2 ^ (Limb::WIDTH * ys.len()) - 1) < 2 ^ (Limb::WIDTH * n) - 1`.
//
// Requires 0 < `ys.len()` <= `xs.len()` <= n and an + `ys.len()` > n / 2. Scratch need: n + (need
// for recursive call OR n + 4). This gives S(n) <= n + MAX (n + 4, S(n / 2)) <= 2 * n + 4
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`, if `ys` is empty, is `xs` is longer than n, or if `out` or
// `scratch` are too short.
//
// This is equivalent to `mpn_mulmod_bnm1` from `mpn/generic/mulmod_bnm1.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_mod_base_pow_n_minus_1(
    out: &mut [Limb],
    n: usize,
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(0, ys_len);
    assert!(xs_len >= ys_len);
    assert!(xs_len <= n);
    let sum = xs_len + ys_len;
    if n < MULMOD_BNM1_THRESHOLD || n.odd() {
        if ys_len < n {
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)];
            if sum <= n {
                limbs_mul_greater_to_out(out, xs, ys, &mut mul_scratch);
            } else {
                limbs_mul_greater_to_out(scratch, xs, ys, &mut mul_scratch);
                if limbs_add_to_out(out, &scratch[..n], &scratch[n..sum]) {
                    assert!(!limbs_slice_add_limb_in_place(&mut out[..n], 1));
                }
            }
        } else {
            limbs_mul_mod_base_pow_n_minus_1_basecase(out, &xs[..n], ys, scratch);
        }
    } else {
        let half_n = n >> 1;
        // We need at least xs_len + ys_len >= half_n, to be able to fit one of the recursive
        // products at out. Requiring strict inequality makes the code slightly simpler. If desired,
        // we could avoid this restriction by initially halving n as long as n is even and xs_len +
        // ys_len <= n / 2.
        assert!(sum > half_n);
        // Compute xm = a * b mod (2 ^ (Limb::WIDTH * half_n) - 1), xp = a * b mod (2 ^ (Limb::WIDTH
        // * half_n) + 1), and Chinese-Remainder-Theorem together as x = -xp * 2 ^ (Limb::WIDTH *
        // half_n) + (2 ^ (Limb::WIDTH * half_n) + 1) * ((xp + xm) / 2 mod (2 ^ (Limb::WIDTH *
        // half_n) - 1))
        let m = half_n + 1;
        if xs_len <= half_n {
            limbs_mul_mod_base_pow_n_minus_1(out, half_n, xs, ys, scratch);
                assert!(sum <= (half_n << 1) | 1);
                let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)];
                limbs_mul_greater_to_out(scratch, xs, ys, &mut mul_scratch);
                let mut limit = sum - half_n;
                assert!(limit <= half_n || scratch[half_n << 1] == 0);
                if limit > half_n {
                    limit -= 1;
                }
                let carry = {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(half_n);
                    limbs_sub_greater_in_place_left(scratch_lo, &scratch_hi[..limit])
                };
                scratch[half_n] = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(&mut scratch[..m], 1));
                }
        } else {
            let (xs_0, xs_1) = xs.split_at(half_n);
            let carry = limbs_add_to_out(scratch, xs_0, xs_1);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(half_n);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(scratch_lo, 1));
            }
            if ys_len <= half_n {
                limbs_mul_mod_base_pow_n_minus_1(out, half_n, scratch_lo, ys, scratch_hi);
                let scratch_2 = &mut scratch[m << 1..3 * m];
                let carry = limbs_sub_greater_to_out(scratch_2, xs_0, xs_1);
                *scratch_2.last_mut().unwrap() = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_2, 1));
                }
                let a = half_n + usize::exact_from(*scratch_2.last_mut().unwrap());
                    let sum_2 = a + ys_len;
                    assert!(sum_2 <= (half_n << 1) + 1);
                    assert!(sum_2 > half_n);
                    assert!(a >= ys_len);
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(m << 1);
                    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(a, ys_len)];
                    limbs_mul_greater_to_out(scratch_lo, &scratch_hi[..a], ys, &mut mul_scratch);
                    let mut a = sum_2 - half_n;
                    assert!(a <= half_n || scratch[half_n << 1] == 0);
                    if a > half_n {
                        a -= 1;
                    }
                    let carry = {
                        let (scratch_lo, scratch_hi) = scratch.split_at_mut(half_n);
                        limbs_sub_greater_in_place_left(scratch_lo, &scratch_hi[..a])
                    };
                    scratch[half_n] = 0;
                    if carry {
                        assert!(!limbs_slice_add_limb_in_place(&mut scratch[..m], 1));
                    }
            } else {
                let (ys_0, ys_1) = ys.split_at(half_n);
                let carry = limbs_add_to_out(scratch_hi, ys_0, ys_1);
                let (scratch_1, scratch_2) = scratch_hi.split_at_mut(half_n);
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_1, 1));
                }
                limbs_mul_mod_base_pow_n_minus_1(out, half_n, scratch_lo, scratch_1, scratch_2);
                let (scratch_2, scratch_3) = scratch[m << 1..].split_at_mut(m);
                let carry = limbs_sub_greater_to_out(scratch_2, xs_0, xs_1);
                *scratch_2.last_mut().unwrap() = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_2, 1));
                }
                let scratch_3 = &mut scratch_3[..m];
                let (ys_0, ys_1) = ys.split_at(half_n);
                let carry = limbs_sub_greater_to_out(scratch_3, ys_0, ys_1);
                *scratch_3.last_mut().unwrap() = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_3, 1));
                }
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(m << 1);
                    limbs_mul_mod_base_pow_n_plus_1_basecase(
                        scratch_lo,
                        scratch_hi,
                        &scratch_hi[m..],
                        half_n,
                    );
            }
        }
        // Here the Chinese Remainder Theorem recomposition begins.
        //
        // let xm = (scratch + xm) / 2 = (scratch + xm) * 2 ^ (Limb::WIDTH * half_n) / 2 mod (2 ^
        // (Limb::WIDTH * half_n) - 1). Division by 2 is a bitwise rotation.
        //
        // Assumes scratch normalised mod (2 ^ (Limb::WIDTH * half_n) + 1).
        //
        // The residue class 0 is represented by [2 ^ (Limb::WIDTH * half_n) - 1]; except when both
        // inputs are zero.
        //
        // scratch[half_n] == 1 implies slice_test_zero(scratch[..half_n]).
        let mut carry = scratch[half_n];
        let (out_lo, out_hi) = out.split_at_mut(half_n);
        if limbs_slice_add_same_length_in_place_left(out_lo, &scratch[..half_n]) {
            carry += 1;
        }
        if out_lo[0].odd() {
            carry += 1;
        }
        limbs_slice_shr_in_place(out_lo, 1);
        let out_lo_last = out_lo.last_mut().unwrap();
        assert!(!out_lo_last.get_highest_bit());
        match carry {
            1 => out_lo_last.set_bit(Limb::WIDTH - 1),
            2 => {
                assert!(!out_lo_last.get_highest_bit());
                assert!(!limbs_slice_add_limb_in_place(out_lo, 1));
            }
            _ => assert_eq!(carry, 0),
        }
        // Compute the highest half: ([(scratch + xm) / 2 mod (2 ^ (Limb::WIDTH * half_n) - 1)] -
        // scratch) * 2 ^ (Limb::WIDTH * half_n)
        if sum < n {
            let a = sum - half_n;
            // Note that in this case, the only way the result can equal zero mod 2 ^ (Limb::WIDTH
            // * n) - 1 is if one of the inputs is zero, and then the output of both the recursive
            // calls and this CRT reconstruction is zero, not 2 ^ (Limb::WIDTH * n) - 1. Which is
            // good, since the latter representation doesn't fit in the output area.
            let borrow = limbs_sub_same_length_to_out(out_hi, &out_lo[..a], &scratch[..a]);
            let mut carry = scratch[half_n];
            let scratch = &mut scratch[..n - half_n];
            if limbs_sub_same_length_with_borrow_in_in_place_right(
                &out[a..n - half_n],
                &mut scratch[a..],
                borrow,
            ) {
                carry += 1;
            }
            assert!(sum == n - 1 || slice_test_zero(&scratch[a + 1..]));
            assert_eq!(
                scratch[a],
                Limb::from(limbs_sub_limb_in_place(&mut out[..sum], carry))
            );
        } else {
            let mut carry = scratch[half_n];
            if limbs_sub_same_length_to_out(out_hi, out_lo, &scratch[..half_n]) {
                carry += 1;
            }
            // carry == 1 only if &scratch[..half_n + 1] is not zero, i.e. out[..half_n] is not
            // zero. The decrement will affect _at most_ the lowest half_n limbs.
            assert!(!limbs_sub_limb_in_place(&mut out[..half_n << 1], carry));
        }
    }
}}
