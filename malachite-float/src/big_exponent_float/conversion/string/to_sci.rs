// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Scientific-string conversion driven by `ToSciOptions`, the engine behind `BigExponentFloat`'s
// `Display`, its `ToStringBase` in bases that are not powers of 2, and its `ToSci`. This is
// `Float`'s to_sci.rs with `Integer` exponents and the digits computed by
// `big_exponent_float_get_str`; for a value within `Float`'s exponent range the output is that of
// `Float`'s `to_sci` for the `Float` with the same value and its minimum precision. A
// `BigExponentFloat` has no negative zero, so zero is written without a sign; a negative value that
// rounds to zero keeps its sign, as it does for `Float` and `Rational`.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::round_to_integer::finite_odd_part;
use crate::big_exponent_float::conversion::string::digits::{
    big_exponent_float_get_str, finite_shl_integer,
};
use crate::float::conversion::string::format_float::strip_trailing_zeros;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::fmt::{Formatter, Write};
use malachite_base::num::arithmetic::traits::{DivRound, Pow, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use malachite_base::num::conversion::traits::{ExactFrom, ToSci};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// The number of base-`base` digits after the point in the exact expansion of a finite nonzero value
// whose odd mantissa is scaled by 2^k, or `None` if the expansion is non-terminating: it terminates
// iff the value is an integer or the base is even, and when 2^v is the largest power of 2 dividing
// the base, clearing 2^-|k| takes ceil(|k| / v) digits.
fn length_after_point(k: &Integer, base: u64) -> Option<Natural> {
    if *k >= 0u32 {
        Some(Natural::ZERO)
    } else {
        match u64::from(base.trailing_zeros()) {
            0 => None,
            v => Some(k.unsigned_abs_ref().div_round(Natural::from(v), Ceiling).0),
        }
    }
}

// The exact floor of log_`base` of the absolute value of the finite nonzero `x`.
fn floor_log_base(x: &BigExponentFloat, base: u64) -> Integer {
    big_exponent_float_get_str(x, i64::exact_from(base), 1, Down).1 - Integer::ONE
}

// Writes the exponent part: the exponent character, the sign (an explicit `+` only when forced or
// when the base is 15 or greater, to distinguish the exponent character from the digit 'e'), and
// the exponent.
fn push_exponent(out: &mut String, options: ToSciOptions, exp: &Integer) {
    out.push(if options.get_e_lowercase() { 'e' } else { 'E' });
    if *exp > 0u32 && (options.get_force_exponent_plus_sign() || options.get_base() >= 15) {
        out.push('+');
    }
    write!(out, "{exp}").unwrap();
}

// The string for zero, or for a nonzero value that rounds to zero, which keeps its sign: as with
// `Rational`, which has no negative zero either, the sign is the value's.
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
// Whether `x` can be converted to a string using `to_sci_string` and a particular set of options:
// with the `Complete` size option the expansion must terminate, and with the `Exact` rounding mode
// the value must be representable in the digits the size options allow.
to_sci_valid(x: &BigExponentFloat, options: ToSciOptions) -> bool {
    if !matches!(x, BigExponentFloat(Finite { .. })) {
        // NaN, infinities, and zero have fixed representations
        return true;
    }
    let base = u64::from(options.get_base());
    let (mantissa, k) = finite_odd_part(x);
    let min_scale = length_after_point(&k, base);
    if let SciSizeOptions::Complete = options.get_size_options() {
        return min_scale.is_some();
    }
    if options.get_rounding_mode() != Exact {
        return true;
    }
    let Some(min_scale) = min_scale else {
        return false;
    };
    let min_scale = Integer::from(min_scale);
    match options.get_size_options() {
        SciSizeOptions::Scale(scale) => min_scale <= scale,
        SciSizeOptions::Precision(precision) => {
            let s = Integer::from(precision - 1) - floor_log_base(x, base);
            if s >= 0u32 {
                min_scale <= s
            } else {
                // The last digit sits at position -s > 0, so the value must be divisible by
                // base^(-s): 2^(-s * v) must divide via the binary exponent, and the base's odd
                // part to the -s must divide the odd mantissa.
                let t = s.unsigned_abs();
                let v = u64::from(base.trailing_zeros());
                if k < &t * Natural::from(v) {
                    return false;
                }
                let odd_base = base >> v;
                if odd_base == 1 {
                    return true;
                }
                // odd_base >= 3, so odd_base^t > 2^t > mantissa: not divisible. This also keeps the
                // power below from being enormous.
                if t >= mantissa.significant_bits() {
                    return false;
                }
                mantissa.divisible_by(Natural::from(odd_base).pow(u64::exact_from(&t)))
            }
        }
        SciSizeOptions::Complete => unreachable!(),
    }
}}

crate_test_fn! {
// Converts a `BigExponentFloat` to a string using a specified base, possibly using scientific
// notation, following `ToSciOptions`. NaN and the infinities are rendered as `NaN`, `Infinity`, and
// `-Infinity`, and the output for any finite value (including zero) always contains a point, `.0`
// being appended if necessary.
//
// Panics if the rounding mode is `Exact` but the size options are such that the input must be
// rounded, or if the size option is `Complete` and the expansion is non-terminating (an odd base
// and a fractional value); `to_sci_valid` identifies both cases.
to_sci_string(x: &BigExponentFloat, options: ToSciOptions) -> String {
    let (neg, sign) = match x {
        BigExponentFloat(NaN) => return String::from("NaN"),
        BigExponentFloat(Infinity { sign: true }) => return String::from("Infinity"),
        BigExponentFloat(Infinity { sign: false }) => return String::from("-Infinity"),
        BigExponentFloat(Zero) => return zero_to_string(false, options),
        BigExponentFloat(Finite { sign, .. }) => (!*sign, *sign),
    };
    let base = u64::from(options.get_base());
    let rm = options.get_rounding_mode();
    let trim_zeros = !options.get_include_trailing_zeros()
        && options.get_size_options() != SciSizeOptions::Complete;
    let log = floor_log_base(x, base);
    // `scale` is the number of digits after the point and `precision` the total number of digits. A
    // nonpositive `precision` means the value rounds to 0 or to 1 unit at the requested scale.
    let (scale, precision) = match options.get_size_options() {
        SciSizeOptions::Complete => {
            let scale = length_after_point(&finite_odd_part(x).1, base).unwrap_or_else(|| {
                panic!("{x} has a non-terminating expansion in base {base}")
            });
            let scale = Integer::from(scale);
            let precision = &scale + &log + Integer::ONE;
            // the digits of the exact expansion begin at the first significant digit
            assert!(precision > 0u32);
            (scale, precision)
        }
        SciSizeOptions::Scale(scale) => {
            let scale = Integer::from(scale);
            let precision = &scale + &log + Integer::ONE;
            (scale, precision)
        }
        SciSizeOptions::Precision(precision) => (
            Integer::from(precision - 1) - &log,
            Integer::from(precision),
        ),
    };
    let (digits, log) = if precision <= 0u32 {
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
            // which requires log + 1 == -scale. Then 2|x| < 2 base^(log + 1) <= base^(log + 2), and
            // 2|x| > base^(log + 1) iff floor(log_base 2|x|) = log + 1 and 2|x| is not base^(log +
            // 1) itself. A tie rounds to the even option, 0.
            Nearest => {
                &log + Integer::ONE == -&scale && {
                    let two_x = finite_shl_integer(x, &Integer::ONE);
                    let (s, e, o) =
                        big_exponent_float_get_str(&two_x, i64::exact_from(base), 1, Down);
                    let s = if neg { &s[1..] } else { &s[..] };
                    e - Integer::ONE == &log + Integer::ONE && (o != Equal || s != b"1")
                }
            }
        };
        if round_up_to_one {
            (vec![b'1'], -&scale)
        } else {
            return zero_to_string(neg, options);
        }
    } else {
        let m = usize::exact_from(&precision);
        // a negative base produces uppercase digits
        let get_str_base = if options.get_lowercase() {
            i64::exact_from(base)
        } else {
            -i64::exact_from(base)
        };
        let (s, e, o) = big_exponent_float_get_str(x, get_str_base, m, rm);
        let mut digits = if neg { s[1..].to_vec() } else { s };
        debug_assert!(options.get_size_options() != SciSizeOptions::Complete || o == Equal);
        let new_log = e - Integer::ONE;
        // Rounding up to a power of the base adds an integral digit. With a requested scale the
        // number of digits after the point must not shrink, so widen the digit string. (With a
        // requested precision the digit count is fixed and the scale shrinks instead, which the
        // layout below derives from `new_log`; and a `Complete` conversion is exact, so no rounding
        // up can occur.)
        if new_log > log && matches!(options.get_size_options(), SciSizeOptions::Scale(_)) {
            digits.push(b'0');
        }
        (digits, new_log)
    };
    // the number of digits after the point, for the padding assertions below
    let target_scale = match options.get_size_options() {
        SciSizeOptions::Precision(_) => Integer::from(digits.len()) - Integer::ONE - &log,
        _ => scale,
    };
    let mut mantissa: Vec<u8> = Vec::new();
    let mut exponent = None;
    if log <= options.get_neg_exp_threshold() || target_scale < 0u32 {
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
    } else if log < 0u32 {
        // no exponent; the value is less than 1, so all digits are fractional
        let ds = if trim_zeros {
            strip_trailing_zeros(&digits)
        } else {
            &digits
        };
        mantissa.extend_from_slice(b"0.");
        mantissa.resize(2 + usize::exact_from(&(-&log - Integer::ONE)), b'0');
        mantissa.extend_from_slice(ds);
        debug_assert!(
            trim_zeros
                || -&log - Integer::ONE + Integer::from(ds.len()) == target_scale,
            "fractional length mismatch"
        );
    } else {
        // no exponent; split the digits at the point
        let digits_before = usize::exact_from(&(&log + Integer::ONE));
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
            trim_zeros || target_scale == frac.len(),
            "fractional length mismatch"
        );
    }
    // a finite value always shows a point
    if !mantissa.contains(&b'.') {
        mantissa.extend_from_slice(b".0");
    }
    let mut out = String::new();
    if !sign {
        out.push('-');
    }
    out.push_str(core::str::from_utf8(&mantissa).unwrap());
    if let Some(exp) = exponent {
        push_exponent(&mut out, options, &exp);
    }
    out
}}

impl ToSci for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] can be converted to a string using
    /// [`to_sci_with_options`](ToSci::to_sci_with_options) and a particular set of options.
    ///
    /// With the [`Complete`](SciSizeOptions::Complete) size option the expansion must terminate, so
    /// a fractional value cannot be written completely in an odd base. With the `Exact` rounding
    /// mode the value must be representable with the digits that the size options allow. Every
    /// other combination is valid.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut options = ToSciOptions::default();
    /// assert!(BigExponentFloat::from(1.5).fmt_sci_valid(options));
    ///
    /// options.set_rounding_mode(Exact);
    /// options.set_precision(1);
    /// assert!(!BigExponentFloat::from(1.5).fmt_sci_valid(options));
    /// options.set_precision(2);
    /// assert!(BigExponentFloat::from(1.5).fmt_sci_valid(options));
    ///
    /// let mut options = ToSciOptions::default();
    /// options.set_base(3);
    /// options.set_size_complete();
    /// assert!(!BigExponentFloat::from(1.5).fmt_sci_valid(options));
    /// ```
    #[inline]
    fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
        to_sci_valid(self, options)
    }

    /// Converts a [`BigExponentFloat`] to a string using a specified base, possibly formatting the
    /// number using scientific notation.
    ///
    /// See [`ToSciOptions`] for details on the available options. The value is rounded to the
    /// digits the size options allow, using the options' rounding mode. `NaN` and the infinities
    /// are written as `NaN`, `Infinity`, and `-Infinity`, and the output for a finite value
    /// (including zero) always contains a point, `.0` being appended if necessary. The exponent may
    /// be arbitrarily large.
    ///
    /// For a value within [`Float`](crate::Float)'s exponent range, the output is that of
    /// [`Float`](crate::Float)'s [`ToSci`] for the [`Float`](crate::Float) with the same value and
    /// the smallest precision that holds it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the larger of
    /// `self.significant_bits()` and the number of digits written.
    ///
    /// # Panics
    /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the input
    /// must be rounded, or if the size option is `Complete` but `self` has a non-terminating
    /// expansion in the chosen base (a fractional value in an odd base).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::{FromStringBase, ToSci};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_sci().to_string(), "NaN");
    /// // a finite value always shows a point
    /// assert_eq!(BigExponentFloat::from(255.0).to_sci().to_string(), "255.0");
    ///
    /// let x = BigExponentFloat::from(1234.5);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1234.5");
    /// options.set_precision(2);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1.2e3");
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// let mut options = ToSciOptions::default();
    /// options.set_scale(0);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "2.0");
    /// options.set_rounding_mode(Down);
    /// assert_eq!(x.to_sci_with_options(options).to_string(), "1.0");
    ///
    /// // The exponent is unbounded: this is 2^(10^20).
    /// let x = BigExponentFloat::from_string_base(2, "1.0E+100000000000000000000").unwrap();
    /// let mut options = ToSciOptions::default();
    /// options.set_precision(10);
    /// assert_eq!(
    ///     x.to_sci_with_options(options).to_string(),
    ///     "2.36531765e30102999566398119521"
    /// );
    /// ```
    #[inline]
    fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
        f.write_str(&to_sci_string(self, options))
    }
}
