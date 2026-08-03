// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2022 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::float::exp::limbs_float_exp;
use crate::natural::arithmetic::float::round::{
    MPFR_ROUND_FAILED, NEG_MPFR_ROUND_FAILED, round_helper_2, round_helper_raw,
    round_helper_raw_aliased,
};
use crate::natural::arithmetic::div_mod::{limbs_div_limb_to_out_mod, limbs_div_mod_to_out};
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::shl::limbs_shl_to_out;
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::conversion::digits::general_digits::limbs_to_digits_small_base;
use crate::natural::{
    LIMB_HIGH_BIT, Natural, bit_to_limb_count_ceiling, bit_to_limb_count_floor,
    limb_to_bit_count,
};
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    DivMod, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

const NUM_TO_TEXT_36: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
// `num_to_text62[d]` is the character for digit `d`, using uppercase letters for `d` in 10..=35 and
// lowercase letters for `d` in 36..=61; for negative bases and for bases 37..=62.
const NUM_TO_TEXT_62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

// Input: an approximation `xs * 2 ^ -neg_f` to a real `Y`, with `|xs * 2 ^ -neg_f - Y| <= 2 ^ (e -
// neg_f)`.
//
// If rounding is possible, returns:
// - in `out`: the characters of the significand corresponding to the integer nearest to `Y`, in the
//   direction `rm`;
// - in `exp`: the exponent (the number of superfluous characters).
//
// `n` is the number of limbs of `xs` (that is, `xs.len()`). `e` represents the maximal error in the
// approximation to `Y` (`e < 0` means that the approximation is known to be exact, that is, `xs * 2
// ^ -neg_f = Y`). `base` is the wanted base (`2 <= base <= 62` or `-36 <= base <= -2`), with
// magnitude `b = base.unsigned_abs()`. `digit_len` is the number of wanted digits in the
// significand. `rm` is the rounding mode. It is assumed that `b ^ (digit_len - 1) <= Y < b ^
// (digit_len + 1)`, thus the returned value satisfies `b ^ (digit_len - 1) <= rm(Y) < b ^
// (digit_len + 1)`.
//
// Rounding may fail for two reasons:
// - the error is too large to determine the integer `N` nearest to `Y`;
// - either the number of digits of `N` in base `b` is too large (`digit_len + 1`), or
//   `N=2*N1+(b/2)` and the rounding mode is to nearest. This can only happen when `b` is even.
//
// The first returned value is the direction of rounding:
// - the direction of rounding (-1, 0, 1) if rounding is possible;
// - `-MPFR_ROUND_FAILED` if rounding is not possible because of `digit_len + 1` digits;
// - `MPFR_ROUND_FAILED` otherwise (too large error).
//
// This is `mpfr_get_str_aux` from `get_str.c`, MPFR 4.2.2.
private_test_fn! {limbs_get_str_aux(
    out: &mut [u8],
    xs: &mut [Limb],
    neg_f: u64,
    e: i64,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
) -> (i8, i64) {
    let n = xs.len();
    let n_width = limb_to_bit_count(n);
    assert!(neg_f < n_width);
    let b = base.unsigned_abs();
    let mut exp = 0;
    // check if it is possible to round xs with rounding mode rm, where |xs * 2 ^ -neg_f - Y| <= 2 ^
    // (e - neg_f). xs contains exactly neg_f bits after the integer point; to determine the nearest
    // integer, we thus need a precision of n * Limb::WIDTH - neg_f.
    let exact = e < 0;
    if exact
        || round_helper_2(
            xs,
            i32::exact_from(i64::exact_from(n_width) - e),
            n_width - neg_f + u64::from(rm == Nearest),
        )
    {
        // compute the nearest integer to xs
        //
        // bit of weight 0 in xs has position j0 in limb xs[i0]
        let mut i0 = bit_to_limb_count_floor(neg_f);
        let j0 = neg_f & Limb::WIDTH_MASK;
        // mpfr_round_raw writes the rounded high limbs of xs back into xs starting at index i0,
        // while reading the original xs. Malachite uses a special function to handle this aliasing.
        let (mut dir, carry) = round_helper_raw_aliased(i0, n_width - neg_f, xs, n_width, rm);
        assert_ne!(dir, MPFR_ROUND_FAILED);
        if carry {
            // Y is a power of 2
            xs[n - 1] = if j0 != 0 {
                LIMB_HIGH_BIT >> (j0 - 1)
            } else {
                // j0 == 0, necessarily i0 >= 1, otherwise neg_f = 0 and xs is exact
                i0 -= 1;
                xs[i0] = 0; // set to zero the new low limb
                Limb::from(carry)
            };
        } else if j0 != 0 {
            // shift xs to the right by neg_f bits (i0 already done)
            limbs_slice_shr_in_place(&mut xs[i0..], j0);
        }
        // now the rounded value Y is in {xs + i0, n - i0}
        //
        // convert xs + i0 into base b: we use base, which might be in -36..-2 one extra character
        // is needed for limbs_to_digits_small_base
        let mut str1 = vec![0; digit_len + 3];
        let size_s1 = limbs_to_digits_small_base(&mut str1, b, &mut xs[i0..], None);
        // round str1
        assert!(size_s1 >= digit_len);
        exp = i64::exact_from(size_s1 - digit_len); // number of superfluous characters

        // if size_s1 = digit_len + 2, necessarily we have b ^ (digit_len + 1) as result, and the
        // result will not change; so we have to double-round only when size_s1 = digit_len + 1 and
        // (i) the result is inexact (ii) or the last digit is nonzero
        let size_s1_m1 = size_s1 - 1;
        if size_s1 == digit_len + 1 && (dir != 0 || str1[size_s1_m1] != 0) {
            // rounding mode
            let rnd1 = if rm == Nearest {
                let twice_last = u64::from(str1[size_s1_m1]) << 1;
                match twice_last.cmp(&b) {
                    Equal => {
                        if dir == 0 && exact {
                            // exact: even rounding
                            if str1[size_s1 - 2].even() {
                                Floor
                            } else {
                                Ceiling
                            }
                        } else {
                            // otherwise we cannot round correctly: for example if b = 10, we might
                            // have a mantissa of xxxxxxx5.00000000 which can be rounded to nearest
                            // to 8 digits but not to 7
                            return (NEG_MPFR_ROUND_FAILED, exp);
                        }
                    }
                    Less => Floor,
                    Greater => Ceiling,
                }
            } else {
                rm
            };
            // now rnd1 is either Floor or Down -> truncate, or Ceiling or Up -> round toward
            // infinity
            if rnd1 == Ceiling || rnd1 == Up {
                // round away from zero
                if str1[size_s1_m1] != 0 {
                    // the carry cannot propagate to the whole string, since Y = x * b ^ (digit_len
                    // - g) < 2 * b ^ digit_len <= b ^ (digit_len + 1) - b, where x is the input
                    // float
                    assert!(size_s1 >= 2);
                    let mut i = size_s1 - 2;
                    let target = u8::exact_from(b - 1);
                    while str1[i] == target {
                        assert_ne!(i, 0);
                        str1[i] = 0;
                        i -= 1;
                    }
                    str1[i] += 1;
                }
                dir = 1;
            } else if str1[size_s1_m1] != 0 {
                // Round toward zero (truncate). When the dropped digit is nonzero the digit
                // rounding dominates the earlier integer rounding (|V - N| >= 1 > |N - Y|), so the
                // overall direction is toward zero.
                dir = -1;
            }
            // Otherwise the dropped digit is zero, so the truncation is exact (V == N) and the
            // overall direction is the integer rounding's `dir`, which we leave unchanged.
            //
            // MPFR's `mpfr_get_str_aux` sets `dir = -1` unconditionally here, since it uses only
            // `dir != 0` (an inexact flag) and the sign is incidental; Malachite returns the
            // direction as an `Ordering`, so it must be correct.
        }
        // copy str1 into out and convert to characters (digits and letters from the source
        // character set)
        let num_to_text = if (2..=36).contains(&base) {
            NUM_TO_TEXT_36
        } else {
            NUM_TO_TEXT_62
        };
        for i in 0..digit_len {
            out[i] = num_to_text[usize::from(str1[i])];
        }
        (dir, exp)
    } else {
        // round_helper_2 failed: rounding is not possible
        (MPFR_ROUND_FAILED, exp)
    }
}}

// Computes the mantissa digits and exponent of a nonzero finite `Float` whose normalized
// little-endian significand is `xs` and whose MPFR-style exponent (one more than the scientific
// exponent) is `x_exp`, in base `abs_base` (the absolute value of the wanted base `base`), with
// `digit_len` digits, rounding with `rm`. Returns the `digit_len` digit characters and the
// exponent.
//
// `g`, `prec`, and `exp` are the initial values computed by the caller (see `mpfr_get_str`): `g =
// ceil_mul(x_exp - 1, abs_base, 1)`, the radix-2 working precision, and `|digit_len - g|`.
//
// This is the non-power-of-two, non-special branch of `mpfr_get_str` from `get_str.c`, MPFR 4.2.2.
#[doc(hidden)]
pub fn limbs_get_str(
    xs: &[Limb],
    x_exp: i64,
    abs_base: u64,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
    mut g: i64,
    mut prec: u64,
    mut exp: i64,
) -> (Vec<u8>, i64, i8) {
    let xs_len = xs.len();
    let digit_len_i = i64::exact_from(digit_len);
    // MPFR_ZIV_INIT: the initial precision increment.
    let mut ziv_step = Limb::WIDTH;
    loop {
        let mut exact = true;
        // number of limbs for the working precision
        let n = bit_to_limb_count_ceiling(prec);
        let mut a = vec![0; n];
        let mut exp_a: i64;
        let mut err: i64;
        match digit_len_i.cmp(&g) {
            Equal => {
                // final exponent is 0: no multiplication or division to perform
                err = if n < xs_len {
                    let (xs_lo, xs_hi) = xs.split_at(xs_len - n);
                    exact = slice_test_zero(xs_lo);
                    a.copy_from_slice(xs_hi);
                    i64::from(!exact)
                } else {
                    a[n - xs_len..].copy_from_slice(xs);
                    0
                };
                exp_a = x_exp - i64::exact_from(limb_to_bit_count(n));
            }
            Greater => {
                // multiply x by abs_base ^ exp; the error on a is at most 2 ^ err ulps
                let err_e;
                (exp_a, err_e) = limbs_float_exp(&mut a, abs_base, exp);
                exact = err_e == -1;
                // x = x1 * 2 ^ (n * Limb::WIDTH): the top min(n, xs_len) limbs of x
                let (x1, nx1) = if n < xs_len {
                    let (xs_lo, xs_hi) = xs.split_at(xs_len - n);
                    if exact {
                        exact = slice_test_zero(xs_lo);
                    }
                    (xs_hi, n)
                } else {
                    (xs, xs_len)
                };
                // we lose one more bit in the multiplication, except when err = 0 (two bits)
                err = if err_e <= 0 { 2 } else { i64::from(err_e) + 1 };
                let result = limbs_mul(&a, x1);
                let (result_lo, result_hi) = result.split_at(nx1);
                let result_hi = &result_hi[..n];
                if !slice_test_zero(result_lo) {
                    exact = false;
                }
                exp_a += x_exp;
                // normalize a and truncate
                if result_hi.last().unwrap().get_highest_bit() {
                    a.copy_from_slice(result_hi);
                } else {
                    limbs_shl_to_out(&mut a, result_hi, 1);
                    a[0] |= Limb::from(result_lo.last().unwrap().get_highest_bit());
                    exp_a -= 1;
                }
            }
            Less => {
                // digit_len < g: divide x by abs_base ^ exp
                let err_e;
                (exp_a, err_e) = limbs_float_exp(&mut a, abs_base, exp);
                exact = err_e == -1;
                let two_n = n << 1;
                let mut scratch;
                let rem;
                let result;
                let x1 = if two_n <= xs_len {
                    scratch = vec![0; two_n + 1];
                    (rem, result) = scratch.split_at_mut(n);
                    let (xs_lo, xs_hi) = xs.split_at(xs_len - two_n);
                    // we ignore the low xs_len - 2 * n limbs of x
                    if exact && !slice_test_zero(xs_lo) {
                        exact = false;
                    }
                    xs_hi
                } else {
                    scratch = vec![0; (two_n << 1) + 1];
                    let scratch_2;
                    (rem, scratch_2) = scratch.split_at_mut(n);
                    let x1_mut;
                    (x1_mut, result) = scratch_2.split_at_mut(two_n);
                    // copy the xs_len most significant limbs of x into the top of x1
                    x1_mut[two_n - xs_len..].copy_from_slice(xs);
                    &*x1_mut
                };
                // result = x / a
                if n == 1 {
                    rem[0] = limbs_div_limb_to_out_mod(result, x1, a[0]);
                } else {
                    limbs_div_mod_to_out(result, rem, x1, &a);
                }
                exp_a = x_exp - exp_a - i64::exact_from(limb_to_bit_count(two_n));
                // test if the division was exact
                if exact {
                    exact = slice_test_zero(rem);
                }
                // normalize the result and copy into a
                let (result_last, result_init) = result.split_last().unwrap();
                if *result_last == 1 {
                    limbs_shr_to_out(&mut a, result_init, 1);
                    a[n - 1] |= LIMB_HIGH_BIT;
                    exp_a += 1;
                } else {
                    a.copy_from_slice(result_init);
                }
                err = if err_e == -1 { 2 } else { i64::from(err_e) + 2 };
            }
        }
        if exact {
            err = -1;
        }
        let mut s = vec![0; digit_len];
        assert!(exp_a < 0);
        let (ret, e) = limbs_get_str_aux(
            &mut s,
            &mut a,
            exp_a.unsigned_abs(),
            err,
            base,
            digit_len,
            rm,
        );
        match ret {
            MPFR_ROUND_FAILED => {
                // error too large: increase the working precision (MPFR_ZIV_NEXT)
                prec += ziv_step;
                ziv_step = prec >> 1;
            }
            NEG_MPFR_ROUND_FAILED => {
                // too many digits in the mantissa: adjust the final exponent g and exp = |digit_len
                // - g|
                if digit_len_i > g {
                    exp -= 1;
                } else {
                    exp += 1;
                }
                g += 1;
            }
            _ => {
                // the exponent of s is its own exponent plus g; ret is the rounding direction
                return (s, e + g, ret);
            }
        }
    }
}

// Computes the mantissa digit characters and exponent of a nonzero finite `Float` whose normalized
// little-endian significand is `xs`, whose precision is `x_prec`, and whose MPFR-style exponent
// (one more than the scientific exponent) is `x_exp`, in the power-of-two base `abs_base` (the
// absolute value of the wanted base `base`), with `digit_len` digits, rounding the magnitude with
// `rm`.
//
// This is the power-of-two-base branch of `mpfr_get_str` from `get_str.c`, MPFR 4.2.2.
#[doc(hidden)]
pub fn limbs_get_str_power_of_2(
    xs: &[Limb],
    x_exp: i64,
    x_prec: u64,
    abs_base: u64,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
) -> (Vec<u8>, i64, i8) {
    let pow2 = abs_base.significant_bits() - 1; // base = 2 ^ pow2
    // x_exp = f * pow2 + r, with 1 <= r <= pow2 (a 1-indexed remainder, so split x_exp - 1)
    let (mut f, r) = (x_exp - 1).div_mod(i64::exact_from(pow2));
    f += 1;
    let r = u64::exact_from(r) + 1;
    // the first digit holds only r bits; prec is the total number of bits
    let prec = (u64::exact_from(digit_len) - 1) * pow2 + r;
    let len = bit_to_limb_count_ceiling(prec);
    let bit_len = limb_to_bit_count(len) - prec;
    let mut scratch = vec![0; len + 1];
    // round xs to prec bits into scratch, with the carry going into scratch[len]; the conversion to
    // base 2 ^ pow2 is then exact, so this rounding's direction is the overall direction
    let (dir, carry) = round_helper_raw(&mut scratch[..len], prec, xs, x_prec, rm);
    if carry {
        // mpfr_round_raw returns the wrapped value [0, ..., 0] and the carry; round_helper_raw
        // renormalizes the top limb to the high bit instead, so clear it to recover scratch = 2 ^
        // prec.
        scratch[len - 1] = 0;
        scratch[len] = 1;
        if r == pow2 {
            // prec = digit_len * pow2: 2 ^ prec needs digit_len + 1 digits in base 2 ^ pow2, so
            // divide by 2 ^ pow2
            limbs_slice_shr_in_place(&mut scratch, pow2);
            f += 1;
        }
    }
    // shift scratch right by bit_len bits, so the digit conversion sees a right-normalized number
    if bit_len != 0 {
        limbs_slice_shr_in_place(&mut scratch, bit_len);
        // the most significant limb may have become zero
        if *scratch.last().unwrap() == 0 {
            scratch.pop();
        }
    }
    // convert scratch to base abs_base = 2 ^ pow2, most significant digit first, and map to
    // characters
    let digits: Vec<u8> = Natural::from_owned_limbs_asc(scratch).to_power_of_2_digits_desc(pow2);
    let num_to_text = if (2..=36).contains(&base) {
        NUM_TO_TEXT_36
    } else {
        NUM_TO_TEXT_62
    };
    let s = digits[..digit_len]
        .iter()
        .map(|&d| num_to_text[usize::from(d)])
        .collect();
    (s, f, dir)
}
