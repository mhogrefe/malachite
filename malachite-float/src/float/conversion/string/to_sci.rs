// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// `get_str`-based scientific-string conversion, driven by `ToSciOptions`: the engine behind
// `Float`'s `Display` and power-of-2-base formatting traits (to_string.rs) and its `ToSci`
// implementation.
//
// The semantics mirror `Rational::fmt_sci` (malachite-q's to_sci.rs) — the same size options,
// negative-exponent threshold, trailing-zero handling, and digit rounding — with one addition,
// the `Float` `Display` convention: the output of a finite value always contains a point, so a
// string that would otherwise lack one gets `.0` appended to its mantissa (`255` becomes `255.0`,
// `8e-7` becomes `8.0e-7`).

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::conversion::string::format_float::strip_trailing_zeros;
use crate::float::conversion::string::get_str::get_str;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::fmt::{Formatter, Write};
use malachite_base::num::arithmetic::traits::{Abs, DivRound, DivisibleBy, Pow};
use malachite_base::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use malachite_base::num::conversion::traits::{ExactFrom, IntegerMantissaAndExponent, ToSci};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;
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

crate_test_fn! {
// Determines whether `x` can be converted to a string using `to_sci_string` and a particular set of
// options; this is the engine of `ToSci::fmt_sci_valid`. Mirrors `Rational::fmt_sci_valid`: with
// the `Complete` size option the expansion must terminate, and with the `Exact` rounding mode the
// value must be representable in the digits the size options allow.
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
            let s = i64::exact_from(precision - 1) - floor_log_base(x, base);
            if s >= 0 {
                min_scale <= s
            } else {
                // The last digit sits at position -s > 0, so the value must be divisible by
                // base^(-s): 2^(-s * v) must divide via the binary exponent, and the base's odd
                // part to the -s must divide the odd mantissa. (`min_scale` cannot see this: it
                // measures digits after the point, and gives no credit for trailing zeros before
                // it.)
                let t = s.unsigned_abs();
                let v = i64::from(base.trailing_zeros());
                if x.integer_exponent() < i64::exact_from(t) * v {
                    return false;
                }
                let odd_base = base >> v;
                if odd_base == 1 {
                    return true;
                }
                let mantissa = x.integer_mantissa();
                // odd_base >= 3, so odd_base^t > 2^t > mantissa: not divisible. This also keeps the
                // power below from being enormous.
                if t >= mantissa.significant_bits() {
                    return false;
                }
                mantissa.divisible_by(Natural::from(u64::exact_from(odd_base)).pow(t))
            }
        }
        SciSizeOptions::Complete => unreachable!(),
    }
}}

crate_test_fn! {
// Converts a `Float` to a string using a specified base, possibly using scientific notation; this
// is the engine behind `Display`, the power-of-2-base formatting traits, and `ToSci`. See
// `ToSciOptions` for details on the available options. The `Float` `Display` conventions apply on
// top of them: NaN and the infinities are rendered as `NaN`, `Infinity`, and `-Infinity`, and the
// output for any finite value (including zeros) always contains a point, `.0` being appended if
// necessary.
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

impl ToSci for Float {
    /// Determines whether a [`Float`] can be converted to a string using
    /// [`to_sci`](malachite_base::num::conversion::traits::ToSci::to_sci) and a particular set of
    /// options.
    ///
    /// NaN, the infinities, and zeros have fixed representations and are always convertible. A
    /// finite nonzero [`Float`] is convertible unless the options request more digits than the
    /// value has: if the size option is `Complete`, the value's expansion in the chosen base must
    /// terminate (any [`Float`] is a dyadic rational, so this holds whenever the value is an
    /// integer or the base is even), and if the rounding mode is `Exact`, the value must be exactly
    /// representable in the digits the size options allow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut options = ToSciOptions::default();
    /// assert!(Float::NAN.fmt_sci_valid(options));
    /// // 1.5 has 2 significant bits
    /// assert!(Float::from(1.5).fmt_sci_valid(options));
    /// options.set_rounding_mode(Exact);
    /// options.set_precision(1);
    /// assert!(!Float::from(1.5).fmt_sci_valid(options));
    /// options.set_precision(2);
    /// assert!(Float::from(1.5).fmt_sci_valid(options));
    ///
    /// let mut options = ToSciOptions::default();
    /// options.set_size_complete();
    /// // 0.5 is non-terminating in base 3...
    /// options.set_base(3);
    /// assert!(!Float::from(0.5).fmt_sci_valid(options));
    /// // ...but is terminating in base 32
    /// options.set_base(32);
    /// assert!(Float::from(0.5).fmt_sci_valid(options));
    /// ```
    #[inline]
    fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
        to_sci_valid(self, options)
    }

    /// Converts a [`Float`] to a string using a specified base, possibly formatting the number
    /// using scientific notation.
    ///
    /// See [`ToSciOptions`] for details on the available options. The [`Float`] `Display`
    /// conventions apply on top of them: NaN and the infinities are rendered as `NaN`, `Infinity`,
    /// and `-Infinity`, and the output for any finite value (including zeros) always contains a
    /// point, `.0` being appended if necessary. Note that the digits are those of the value's
    /// actual expansion, rounded to the requested size; unlike `Display`, which shows the shortest
    /// string that rounds back to the value, no round-trip shortening occurs.
    ///
    /// The digits are computed by rounding the value directly, so the [`Float`] is never
    /// materialized as a [`Rational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), s)`,
    /// where `s` depends on the size type specified in `options`.
    /// - If `options` has `scale` specified, then `s` is `options.scale`.
    /// - If `options` has `precision` specified, then `s` is `options.precision`.
    /// - If `options` has `size_complete` specified, then `s` is
    ///   `self.get_exponent().unwrap().unsigned_abs()`. This reflects the fact that setting
    ///   `size_complete` might result in a very long string when the value's magnitude is very
    ///   large or very small.
    ///
    /// # Panics
    /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the input
    /// must be rounded, or if the size option is `Complete` but `self` has a non-terminating
    /// expansion in the chosen base (a fractional value in an odd base).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_sci().to_string(), "NaN");
    /// // a finite value always shows a point
    /// assert_eq!(Float::from(255.0).to_sci().to_string(), "255.0");
    ///
    /// let x = Float::from(1234.5);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1234.5");
    /// options.set_precision(4);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1234.0");
    /// options.set_precision(2);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1.2e3");
    ///
    /// let x = Float::from(1.5);
    /// let mut options = ToSciOptions::default();
    /// options.set_scale(0);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "2.0");
    /// options.set_rounding_mode(Down);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1.0");
    ///
    /// let mut options = ToSciOptions::default();
    /// options.set_base(20);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1.a");
    /// options.set_uppercase();
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1.A");
    ///
    /// // in bases 15 and up, a positive exponent always gets an explicit sign, to distinguish
    /// // the exponent indicator from the digit 'e'
    /// let mut options = ToSciOptions::default();
    /// options.set_base(16);
    /// options.set_precision(2);
    /// assert_eq!(
    ///     Float::from(1000000.0)
    ///         .to_sci_with_options(options)
    ///         .to_string(),
    ///     "f.4e+4"
    /// );
    ///
    /// // 2^-17, a 1-bit value, printed with its actual digits
    /// let x = Float::power_of_2(-17i64);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(
    ///     x.to_sci_with_options(options).to_string(),
    ///     "7.62939453125e-6"
    /// );
    /// options.set_e_uppercase();
    /// assert_eq!(
    ///     x.to_sci_with_options(options).to_string(),
    ///     "7.62939453125E-6"
    /// );
    /// options.set_neg_exp_threshold(-10);
    /// assert_eq!(
    ///     x.to_sci_with_options(options).to_string(),
    ///     "0.00000762939453125"
    /// );
    /// ```
    #[inline]
    fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
        f.write_str(&to_sci_string(self, options))
    }
}
