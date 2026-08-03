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

use crate::natural::arithmetic::div_mod::{limbs_div_limb_to_out_mod, limbs_div_mod_to_out};
use crate::natural::arithmetic::float::exp::limbs_float_exp;
use crate::natural::arithmetic::float::round::{round_helper_2, round_helper_raw};
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::conversion::digits::general_digits::limbs_from_digits_small_base;
use crate::natural::{bit_to_limb_count_ceiling, limb_to_bit_count};
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, DivMod, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

// `RED_INV_LOG_2[b - 2]`, as a `(numerator, denominator)` pair, is an upper approximation to
// `log(2) / log(b)`, no larger than 1. Both entries fit in 16 bits.
//
// This is `RedInvLog2Table` from `strtofr.c`, MPFR 4.3.0.
const RED_INV_LOG_2: [(u16, u16); 61] = [
    (1, 1),
    (53, 84),
    (1, 2),
    (4004, 9297),
    (53, 137),
    (2393, 6718),
    (1, 3),
    (665, 2108),
    (4004, 13301),
    (949, 3283),
    (53, 190),
    (5231, 19357),
    (2393, 9111),
    (247, 965),
    (1, 4),
    (4036, 16497),
    (665, 2773),
    (5187, 22034),
    (4004, 17305),
    (51, 224),
    (949, 4232),
    (3077, 13919),
    (53, 243),
    (73, 339),
    (5231, 24588),
    (665, 3162),
    (2393, 11504),
    (4943, 24013),
    (247, 1212),
    (3515, 17414),
    (1, 5),
    (4415, 22271),
    (4036, 20533),
    (263, 1349),
    (665, 3438),
    (1079, 5621),
    (5187, 27221),
    (2288, 12093),
    (4004, 21309),
    (179, 959),
    (51, 275),
    (495, 2686),
    (949, 5181),
    (3621, 19886),
    (3077, 16996),
    (229, 1272),
    (53, 296),
    (109, 612),
    (73, 412),
    (1505, 8537),
    (5231, 29819),
    (283, 1621),
    (665, 3827),
    (32, 185),
    (2393, 13897),
    (1879, 10960),
    (4943, 28956),
    (409, 2406),
    (247, 1459),
    (231, 1370),
    (3515, 20929),
];

// Converts `digits`, in base `base` and most significant first, to little-endian limbs written to
// `out`, returning the number of limbs written. `out` must have room for one limb beyond the
// result. The most significant limb written is nonzero as long as the first digit is.
//
// This is equivalent to `mpn_set_str` from `mpn/generic/set_str.c`, GMP 6.3.0.
fn limbs_set_str_helper(out: &mut [Limb], digits: &[u8], base: u64) -> usize {
    if let Some(bits) = base.checked_log_base_2() {
        // The base is a power of 2: read the digits from least to most significant, packing them
        // into limbs.
        let mut len = 0;
        let mut digit_out = 0;
        let mut next_bit_index = 0;
        for &digit in digits.iter().rev() {
            let digit = Limb::from(digit);
            digit_out |= digit << next_bit_index;
            next_bit_index += bits;
            if next_bit_index >= Limb::WIDTH {
                out[len] = digit_out;
                len += 1;
                next_bit_index -= Limb::WIDTH;
                digit_out = digit >> (bits - next_bit_index);
            }
        }
        if digit_out != 0 {
            out[len] = digit_out;
            len += 1;
        }
        len
    } else {
        limbs_from_digits_small_base(out, digits, base).unwrap()
    }
}

// The result of `limbs_set_str`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SetStrResult {
    // The significand limbs, the MPFR-style exponent (one more than the scientific exponent), and
    // the direction in which the magnitude was rounded.
    Finite(Vec<Limb>, i64, i8),
    // The exponent computation overflowed; the value is larger than any finite `Float`.
    Overflow,
    // The exponent computation underflowed; the value is smaller than any positive `Float`.
    Underflow,
}

// Adds two exponents, mirroring `MPFR_SADD_OVERFLOW`: `Err(true)` reports an overflow (the `goto
// overflow` branch) and `Err(false)` an underflow (`goto underflow`). Unlike the C macro, which
// performs a plain addition when the arguments have opposite signs, this is checked in every case.
fn add_exp(x: i64, y: i64) -> Result<i64, bool> {
    x.checked_add(y).ok_or(y > 0)
}

// The `goto overflow` and `goto underflow` targets of `parsed_string_to_mpfr`.
const fn out_of_range(overflow: bool) -> SetStrResult {
    if overflow {
        SetStrResult::Overflow
    } else {
        SetStrResult::Underflow
    }
}

// Converts a parsed digit string to a `Float` significand and MPFR-style exponent, correctly
// rounded to `prec_x` bits.
//
// `digits` holds digit values (not characters), most significant first, with leading and trailing
// zeros already stripped; it must be nonempty. `exp_base` is the number of digits before the point
// plus any base-`base` exponent, and `exp_bin` an additional binary exponent (the `p` form of the
// input), zero when there is none. `rm` must already have been inverted if the value is negative,
// since the returned direction refers to the magnitude.
//
// `base` must be between 2 and 62, inclusive, and `prec_x` must be positive.
//
// This is `parsed_string_to_mpfr` from `strtofr.c`, MPFR 4.3.0.
#[doc(hidden)]
pub fn limbs_set_str(
    digits: &[u8],
    base: u64,
    exp_base: i64,
    exp_bin: i64,
    prec_x: u64,
    rm: RoundingMode,
) -> SetStrResult {
    let digits_len = digits.len();
    assert_ne!(digits_len, 0);
    assert!(const { 2..=62 }.contains(&base));
    assert_ne!(prec_x, 0);
    // the initial working precision
    let mut prec = prec_x + prec_x.ceiling_log_base_2();
    // MPFR_ZIV_INIT: the house increment schedule, not MPFR's
    let mut ziv_step = Limb::WIDTH;
    // Compute the value of the leading digits as long as rounding is not possible.
    let (result, ysize_bits, mut exp) = loop {
        // y is regarded as a number of precision prec, occupying ysize limbs.
        let ysize = bit_to_limb_count_ceiling(prec);
        let ysize_bits = limb_to_bit_count(ysize);
        // pstr_size is the number of digits to read to fill at least ysize full limbs: we need base
        // ^ (pstr_size - 1) >= 2 ^ ysize_bits, so pstr_size = 1 + ceil(ysize_bits * Num / Den) with
        // Num / Den an upper approximation to 1 / log2(base). Writing ysize_bits = a * Den + b
        // keeps the products from overflowing.
        let (num, den) = RED_INV_LOG_2[usize::exact_from(base) - 2];
        let (num, den) = (u64::from(num), u64::from(den));
        let (a, b) = ysize_bits.div_mod(den);
        let mut pstr_size = usize::exact_from(a * num + (b * num).div_ceil(den) + 1);
        // Since pstr_size corresponds to at least ysize_bits bits, and ysize_bits >= prec, the
        // weight of the neglected part of the digits (if any) is less than ulp(y) < ulp(x).
        if pstr_size > digits_len {
            pstr_size = digits_len;
        }
        // The digits' value is less than base ^ pstr_size, which bounds the limbs the conversion
        // writes; one more is added because `limbs_set_str_helper` may touch the limb past the
        // result. MPFR instead assumes a fixed couple of limbs beyond ysize, which holds only when
        // pstr_size is the exact ceiling above. `Num / Den` overshoots it by a number of digits
        // proportional to ysize_bits, so at high precision the value really does need more, and no
        // fixed allowance is enough.
        let y_len =
            bit_to_limb_count_ceiling(u64::exact_from(pstr_size) * base.ceiling_log_base_2()) + 1;
        // y starts at offset ysize; the low ysize limbs are the scratch that the two exponentiation
        // cases below use.
        let mut y0 = vec![0; ysize + max(ysize, y_len)];
        // Convert the (possibly truncated) digits to binary; they are big-endian, so no offset is
        // needed.
        let real_ysize = limbs_set_str_helper(&mut y0[ysize..], &digits[..pstr_size], base);
        // `exact` tracks whether the result is known to be exact, which lets the loop terminate
        // even when the rounding test fails. It starts by accounting for the part of the input that
        // was ignored: trailing zeros were stripped in parsing, so anything ignored is nonzero.
        let mut exact = pstr_size == digits_len;
        // Normalize y and set the initial value of its exponent, which is 0 when y is not shifted.
        // The digits were normalized, so limbs_set_str_helper leaves a nonzero top limb.
        let y = &mut y0[ysize..];
        assert_ne!(y[real_ysize - 1], 0);
        let count = u64::from(y[real_ysize - 1].leading_zeros());
        let mut exp;
        if let Some(diff_ysize) = ysize.checked_sub(real_ysize) {
            // There is room to store {y, real_ysize} exactly in {y, ysize}, so the left shift loses
            // nothing and `exact` does not change.
            if count != 0 {
                limbs_slice_shl_in_place(&mut y[..real_ysize], count);
            }
            if diff_ysize != 0 {
                y.copy_within(0..real_ysize, diff_ysize);
                y[..diff_ysize].fill(0);
            }
            // the negation of the total shift count
            exp = -(i64::exact_from(limb_to_bit_count(diff_ysize)) + i64::exact_from(count));
        } else {
            // {y, real_ysize} does not fit in ysize limbs. Drop the low limbs that cannot be kept,
            // then shift the rest right by Limb::WIDTH - count bits, leaving the value's top bit at
            // the top of the ysize-th limb. MPFR only ever drops a limb when its limbs are narrower
            // than 12 bits; here the slack in `Num / Den` makes it happen at high precision too.
            let dropped = real_ysize - ysize - 1;
            if dropped != 0 {
                exact = exact && slice_test_zero(&y[..dropped]);
                y.copy_within(dropped..real_ysize, 0);
            }
            let kept = ysize + 1;
            if count != 0 {
                if limbs_slice_shr_in_place(&mut y[..kept], Limb::WIDTH - count) != 0 {
                    // some nonzero bits were shifted out
                    exact = false;
                }
            } else {
                exact = exact && y[0] == 0;
                y.copy_within(1..kept, 0);
            }
            exp = i64::exact_from(limb_to_bit_count(dropped + 1)) - i64::exact_from(count);
        }
        // Compute base ^ (exp_base - pstr_size) on ysize limbs, multiplying or dividing y by it.
        let pstr_size_i = i64::exact_from(pstr_size);
        let ysize_bits_i = i64::exact_from(ysize_bits);
        let mut err;
        // The rounded-toward-zero approximation, and the offset within it of the ysize significant
        // limbs.
        let mut product;
        let result_offset;
        if let Some(pow2) = base.checked_log_base_2() {
            // Case 1: the base is a power of two, so the scaling is exact.
            let pow2 = i64::exact_from(pow2);
            let mut tmp = match add_exp(exp_base, -pstr_size_i) {
                Ok(tmp) => tmp,
                Err(over) => return out_of_range(over),
            };
            tmp = match tmp.checked_mul(pow2) {
                Some(tmp) => tmp,
                None => return out_of_range(tmp > 0),
            };
            tmp = match add_exp(tmp, exp_bin) {
                Ok(tmp) => tmp,
                Err(over) => return out_of_range(over),
            };
            exp = match add_exp(exp, tmp) {
                Ok(exp) => exp,
                Err(over) => return out_of_range(over),
            };
            product = y0;
            result_offset = ysize;
            err = 0;
        } else if exp_base > pstr_size_i {
            // Case 2: multiply y by base ^ (exp_base - pstr_size).
            let (y0_lo, y_hi) = y0.split_at_mut(ysize);
            // z = base ^ (exp_base - pstr_size), rounded toward zero, in the scratch below y
            let (mut exp_z, err_z) = limbs_float_exp(y0_lo, base, exp_base - pstr_size_i);
            if err_z == -2 {
                return SetStrResult::Overflow;
            }
            exact = exact && err_z == -1;
            // Both y and z are rounded toward zero, so the product is too.
            product = limbs_mul(&y_hi[..ysize], y0_lo);
            // one more bit is lost in the multiplication, except when err_z is 0 (two bits)
            err = if err_z == -1 { 0 } else { i64::from(err_z) } + 1;
            exp_z = match add_exp(exp_z, ysize_bits_i) {
                Ok(exp_z) => exp_z,
                Err(over) => return out_of_range(over),
            };
            exp = match add_exp(exp, exp_z) {
                Ok(exp) => exp,
                Err(over) => return out_of_range(over),
            };
            // normalize the product
            if !product[(ysize << 1) - 1].get_highest_bit() {
                limbs_slice_shl_in_place(&mut product[ysize - 1..], 1);
                exp -= 1;
            }
            // if the low ysize limbs are all zero the result is still exact, if it was before
            exact = exact && slice_test_zero(&product[..ysize]);
            result_offset = ysize;
        } else if exp_base < pstr_size_i {
            // Case 3: divide y by base ^ (pstr_size - exp_base).
            //
            // y0 = y * 2 ^ ysize_bits
            y0[..ysize].fill(0);
            // avoid negating the extreme value
            let neg_exp_base = if exp_base == i64::MIN {
                i64::MAX
            } else {
                -exp_base
            };
            // The two overflow branches are swapped here: a larger divisor means a smaller result.
            let mut exp_z = match add_exp(pstr_size_i, neg_exp_base) {
                Ok(exp_z) => exp_z,
                Err(over) => return out_of_range(!over),
            };
            let mut z = vec![0; ysize];
            let err_z;
            (exp_z, err_z) = limbs_float_exp(&mut z, base, exp_z);
            // {z, ysize} * 2 ^ (exp_z - ysize_bits) approximates base ^ exp_z from below, with the
            // error bounded by 2 ^ err_z ulps (or exact when err_z is -1). The truncation errors of
            // the division and of the ignored digits have the opposite sign to the error on z, so
            // they partly compensate; the bound below takes the maximum rather than the sum.
            if err_z == -2 {
                return SetStrResult::Underflow;
            } else if err_z == -1 {
                err = 0;
            } else {
                err = i64::from(err_z);
                exact = false;
            }
            exp_z = match add_exp(exp_z, ysize_bits_i) {
                Ok(exp_z) => exp_z,
                Err(over) => return out_of_range(!over),
            };
            exp = match add_exp(exp, -exp_z) {
                Ok(exp) => exp,
                Err(over) => return out_of_range(over),
            };
            // Divide, rounding toward zero: the quotient has ysize + 1 limbs and the remainder
            // ysize. Both operands are normalized.
            assert!(y0[(ysize << 1) - 1].get_highest_bit());
            assert!(z[ysize - 1].get_highest_bit());
            // The quotient must end up owned (it becomes `product`), so it is the parent's prefix,
            // with the remainder scratch as the tail; the parent is truncated once the remainder
            // has been tested.
            let mut quotient = vec![0; (ysize << 1) + 1];
            let (qs, remainder) = quotient.split_at_mut(ysize + 1);
            if ysize == 1 {
                remainder[0] = limbs_div_limb_to_out_mod(qs, &y0[..2], z[0]);
            } else {
                limbs_div_mod_to_out(qs, remainder, &y0[..ysize << 1], &z);
            }
            assert!(qs[ysize] <= 1);
            // see the note above on the compensating errors
            err += 1;
            // if the remainder is zero the result is still exact, if it was before
            exact = exact && slice_test_zero(remainder);
            quotient.truncate(ysize + 1);
            if quotient[ysize] == 1 {
                exact = exact && quotient[0].even();
                limbs_slice_shr_in_place(&mut quotient, 1);
                exp += 1;
            }
            product = quotient;
            result_offset = 0;
        } else {
            // Case 4: exp_base == pstr_size, so base ^ (exp_base - pstr_size) is 1 and there is
            // nothing to compute.
            product = y0;
            result_offset = ysize;
            err = 0;
        }
        // `product[result_offset..]` is an approximation, rounded toward zero, of the pstr_size
        // most significant digits, with equality when `exact`.
        let result = &product[result_offset..result_offset + ysize];
        // Test whether rounding is possible. The precx + (rnd == RNDN) trick is needed because the
        // ternary value must be determined too: for xxx...xxx111...111 under Nearest the correct
        // rounding is known but the ternary value is not.
        if exact
            || round_helper_2(
                result,
                i32::exact_from(ysize_bits_i - err - 1),
                prec_x + u64::from(rm == Nearest),
            )
        {
            break (result.to_vec(), ysize_bits, exp);
        }
        // MPFR_ZIV_NEXT
        prec += ziv_step;
        ziv_step = prec >> 1;
    };
    // round the result to prec_x bits
    let mut out = vec![0; bit_to_limb_count_ceiling(prec_x)];
    let (dir, increment) = round_helper_raw(&mut out, prec_x, &result, ysize_bits, rm);
    if increment {
        // round_helper_raw has already renormalized the top limb
        exp += 1;
    }
    // If the approximation was exact then no double rounding can occur, so `dir` is the correct
    // direction. The exponent may be out of range; the caller checks it. `add_exp` reports a
    // downward overflow only when its second argument is negative, and `ysize_bits` is always
    // positive, so the only failure possible here is an upward one.
    match add_exp(exp, i64::exact_from(ysize_bits)) {
        Ok(exp) => SetStrResult::Finite(out, exp, dir),
        Err(_) => SetStrResult::Overflow,
    }
}
