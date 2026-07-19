// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// `get_str`-based scientific-string conversion, driven by `ToSciOptions`: the engine behind
// `Float`'s `Display` and power-of-2-base formatting traits (to_string.rs), and eventually behind
// a `ToSci` implementation.
//
// The semantics mirror `Rational::fmt_sci` (malachite-q's to_sci.rs) — the same size options,
// negative-exponent threshold, trailing-zero handling, and digit rounding — with one addition,
// the `Float` `Display` convention: the output of a finite value always contains a point, so a
// string that would otherwise lack one gets `.0` appended to its mantissa (`255` becomes `255.0`,
// `8e-7` becomes `8.0e-7`).

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::conversion::string::format_float::strip_trailing_zeros;
use crate::conversion::string::get_str::get_str;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::fmt::Write;
use malachite_base::num::arithmetic::traits::{Abs, DivRound, Pow};
use malachite_base::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use malachite_base::num::conversion::traits::{ExactFrom, IntegerMantissaAndExponent};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_q::Rational;

// The number of base-`base` digits after the point in the exact expansion of the finite nonzero
// `Float` with least binary exponent `k` (that is, whose odd mantissa is scaled by 2^k), or `None`
// if the expansion is non-terminating. A `Float` is a dyadic rational, so the expansion terminates
// iff the value is an integer or the base is even; when 2^v is the largest power of 2 dividing the
// base, clearing 2^-|k| takes ceil(|k| / v) digits. This is the `Float` analogue of
// `Rational::length_after_point_in_small_base`.
fn length_after_point(k: i64, base: i64) -> Option<u64> {
    if k >= 0 {
        Some(0)
    } else {
        match u64::from(base.trailing_zeros()) {
            0 => None,
            v => Some(k.unsigned_abs().div_round(v, Ceiling).0),
        }
    }
}

// The exact floor of log_`base` of `|x|`, for finite nonzero `x`. `get_str` returns the exponent
// `e` such that the rounded value is 0.ddd... * base^e; with one digit and truncating rounding no
// magnitude round-up can occur, so `e - 1` is exact.
fn floor_log_base(x: &Float, base: i64) -> i64 {
    get_str(x, base, 1, Down).unwrap().1 - 1
}

// Writes the exponent part: the exponent character, the sign (an explicit `+` only when forced or
// when the base is 15 or greater, to distinguish the exponent character from the digit 'e'), and
// the exponent. This is `write_exponent` from malachite-base's to_sci.rs, writing to a `String`.
fn push_exponent(out: &mut String, options: ToSciOptions, exp: i64) {
    out.push(if options.get_e_lowercase() { 'e' } else { 'E' });
    if exp > 0 && (options.get_force_exponent_plus_sign() || options.get_base() >= 15) {
        out.push('+');
    }
    write!(out, "{exp}").unwrap();
}

// The string for a zero `Float` with the given sign. This mirrors `fmt_zero` from malachite-q's
// to_sci.rs, plus the trailing-`.0` convention.
fn zero_to_string(neg: bool, options: ToSciOptions) -> String {
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    out.push('0');
    if options.get_include_trailing_zeros() {
        let zeros = match options.get_size_options() {
            SciSizeOptions::Complete => 0,
            SciSizeOptions::Scale(scale) => scale,
            SciSizeOptions::Precision(precision) => precision - 1,
        };
        if zeros != 0 {
            out.push('.');
            for _ in 0..zeros {
                out.push('0');
            }
        }
    }
    if !out.contains('.') {
        out.push_str(".0");
    }
    out
}

pub_crate_test! {
// Determines whether `x` can be converted to a string using `to_sci_string` and a particular set of
// options; this is the future `ToSci::fmt_sci_valid`. Mirrors `Rational::fmt_sci_valid`: with the
// `Complete` size option the expansion must terminate, and with the `Exact` rounding mode the value
// must be representable in the digits the size options allow.
to_sci_valid(x: &Float, options: ToSciOptions) -> bool {
    if !matches!(x, Float(Finite { .. })) {
        // NaN, infinities, and zeros have fixed representations
        return true;
    }
    let base = i64::from(options.get_base());
    let min_scale = length_after_point(x.integer_exponent(), base);
    if let SciSizeOptions::Complete = options.get_size_options() {
        return min_scale.is_some();
    }
    if options.get_rounding_mode() != Exact {
        return true;
    }
    let Some(min_scale) = min_scale else {
        return false;
    };
    let min_scale = i64::exact_from(min_scale);
    match options.get_size_options() {
        SciSizeOptions::Scale(scale) => min_scale <= i64::exact_from(scale),
        SciSizeOptions::Precision(precision) => {
            min_scale <= i64::exact_from(precision - 1) - floor_log_base(x, base)
        }
        SciSizeOptions::Complete => unreachable!(),
    }
}}

pub_crate_test! {
// Converts a `Float` to a string using a specified base, possibly using scientific notation; this
// is the engine behind `Display` and the power-of-2-base formatting traits (and eventually
// `ToSci`). See `ToSciOptions` for
// details on the available options. The `Float` `Display` conventions apply on top of them: NaN and
// the infinities are rendered as `NaN`, `Infinity`, and `-Infinity`, and the output for any finite
// value (including zeros) always contains a point, `.0` being appended if necessary.
//
// The digits are computed by `get_str`, which rounds the value directly, so this function never
// materializes the `Float` as a `Rational` (except in one corner case: deciding a `Nearest` tie
// when the value's magnitude lies within one base-power of a `Scale` boundary).
//
// Panics if the rounding mode is `Exact` but the size options are such that the input must be
// rounded, or if the size option is `Complete` and the expansion is non-terminating (an odd base
// and a fractional value); `to_sci_valid` identifies both cases.
to_sci_string(x: &Float, options: ToSciOptions) -> String {
    let (neg, sign) = match x {
        Float(NaN) => return String::from("NaN"),
        Float(Infinity { sign: true }) => return String::from("Infinity"),
        Float(Infinity { sign: false }) => return String::from("-Infinity"),
        Float(Zero { sign }) => return zero_to_string(!*sign, options),
        Float(Finite { sign, .. }) => (!*sign, *sign),
    };
    let base = i64::from(options.get_base());
    let rm = options.get_rounding_mode();
    let trim_zeros = !options.get_include_trailing_zeros()
        && options.get_size_options() != SciSizeOptions::Complete;
    let log = floor_log_base(x, base);
    // `scale` is the number of digits after the point and `precision` the total number of digits,
    // as in `Rational::fmt_sci`. A nonpositive `precision` means the value rounds to 0 or to 1 unit
    // at the requested scale.
    let (scale, precision) = match options.get_size_options() {
        SciSizeOptions::Complete => {
            let scale = length_after_point(x.integer_exponent(), base).unwrap_or_else(|| {
                panic!("{x} has a non-terminating expansion in base {base}")
            });
            let precision = i64::exact_from(scale) + log + 1;
            // the digits of the exact expansion begin at the first significant digit
            assert!(precision > 0);
            (i64::exact_from(scale), precision)
        }
        SciSizeOptions::Scale(scale) => {
            (i64::exact_from(scale), i64::exact_from(scale) + log + 1)
        }
        SciSizeOptions::Precision(precision) => (
            i64::exact_from(precision - 1) - log,
            i64::exact_from(precision),
        ),
    };
    let (digits, log) = if precision <= 0 {
        // 0 < |x| * base^scale < 1: the value rounds to 0 or to 1 in the last place.
        let round_up_to_one = match rm {
            Up => true,
            Down => false,
            Floor => neg,
            Ceiling => !neg,
            Exact => panic!(
                "Exact rounding was requested, but {x} is not exactly representable with {scale} \
                digits after the point",
            ),
            // |x| < base^(log + 1) <= base^(-scale); it rounds up iff it exceeds base^-scale / 2,
            // which requires log + 1 == -scale (one base-power below the boundary and it is already
            // at most half). A tie rounds to the even option, 0.
            Nearest => {
                log + 1 == -scale && {
                    let two_x = Rational::exact_from(x).abs() << 1u32;
                    two_x > Rational::from(base).pow(-scale)
                }
            }
        };
        if round_up_to_one {
            (vec![b'1'], -scale)
        } else {
            return zero_to_string(neg, options);
        }
    } else {
        let m = usize::exact_from(precision);
        // a negative base makes `get_str` produce uppercase digits
        let get_str_base = if options.get_lowercase() { base } else { -base };
        let (s, e, o) = get_str(x, get_str_base, m, rm).unwrap();
        let mut digits = if neg { s[1..].to_vec() } else { s };
        debug_assert!(options.get_size_options() != SciSizeOptions::Complete || o == Equal);
        let new_log = e - 1;
        // Rounding up to a power of the base adds an integral digit. With a requested scale the
        // number of digits after the point must not shrink, so widen the digit string; this mirrors
        // `Rational::fmt_sci`, which widens its precision. (With a requested precision the digit
        // count is fixed and the scale shrinks instead, which the layout below derives from
        // `new_log`; and a `Complete` conversion is exact, so no rounding up can occur.)
        if new_log > log && matches!(options.get_size_options(), SciSizeOptions::Scale(_)) {
            digits.push(b'0');
        }
        (digits, new_log)
    };
    // the number of digits after the point, for the padding assertions below
    let target_scale = match options.get_size_options() {
        SciSizeOptions::Precision(_) => i64::exact_from(digits.len()) - 1 - log,
        _ => scale,
    };
    let mut mantissa: Vec<u8> = Vec::new();
    let mut exponent = None;
    if log <= options.get_neg_exp_threshold() || target_scale < 0 {
        // scientific notation: one digit, the rest after a point, and an exponent
        let ds = if trim_zeros {
            strip_trailing_zeros(&digits)
        } else {
            &digits
        };
        mantissa.push(ds[0]);
        if ds.len() > 1 {
            mantissa.push(b'.');
            mantissa.extend_from_slice(&ds[1..]);
        }
        exponent = Some(log);
    } else if log < 0 {
        // no exponent; the value is less than 1, so all digits are fractional
        let ds = if trim_zeros {
            strip_trailing_zeros(&digits)
        } else {
            &digits
        };
        mantissa.extend_from_slice(b"0.");
        mantissa.resize(2 + usize::exact_from(-log - 1), b'0');
        mantissa.extend_from_slice(ds);
        debug_assert!(
            trim_zeros || -log - 1 + i64::exact_from(ds.len()) == target_scale,
            "fractional length mismatch"
        );
    } else {
        // no exponent; split the digits at the point
        let digits_before = usize::exact_from(log + 1);
        mantissa.extend_from_slice(&digits[..digits_before]);
        let frac = if trim_zeros {
            strip_trailing_zeros(&digits[digits_before..])
        } else {
            &digits[digits_before..]
        };
        if !frac.is_empty() {
            mantissa.push(b'.');
            mantissa.extend_from_slice(frac);
        }
        debug_assert!(
            trim_zeros || i64::exact_from(frac.len()) == target_scale,
            "fractional length mismatch"
        );
    }
    // the `Float` `Display` convention: a finite value always shows a point
    if !mantissa.contains(&b'.') {
        mantissa.extend_from_slice(b".0");
    }
    let mut out = String::new();
    if !sign {
        out.push('-');
    }
    out.push_str(core::str::from_utf8(&mantissa).unwrap());
    if let Some(exp) = exponent {
        push_exponent(&mut out, options, exp);
    }
    out
}}
