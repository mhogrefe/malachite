// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Malachite's own scientific-string parsing, driven by `FromSciStringOptions`: the counterpart of
// `to_sci.rs`, and the reverse-direction sibling of `strtofr.rs`.
//
// The grammar is the one the rest of Malachite uses, `preprocess_sci_string` from malachite-base,
// so a `Float` reads a string the same way a `Rational` does. That differs from MPFR's, which
// `strtofr.rs` implements instead; the two never disagree about a string's value, but each accepts
// some the other rejects (see PORTING.md). What `Float` adds over `Rational` is the three special
// values, a signed zero, and a precision and rounding mode, with the ternary value they imply.
//
// The digits are handed to the same `set_str_helper` core that `strtofr.rs` uses, so the arithmetic
// is shared and only the grammar differs.

use crate::Float;
use crate::conversion::string::get_str::ceil_mul;
use crate::conversion::string::set_str::{overflow, set_str_helper};
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::string::from_sci_string::preprocess_sci_string;
use malachite_base::num::conversion::string::from_string::digit_from_display_byte;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{
    ExactFrom, FromSciString, IntegerMantissaAndExponent,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};

// The outcome of reading a string's digits, before any rounding.
enum Parsed {
    // A NaN or an infinity, which carry no precision.
    Special(Float),
    // A zero with the given sign.
    Zero(bool),
    // The sign, the digit values (most significant first, with leading and trailing zeros
    // stripped), the number of digits before the point plus any exponent, and how many significant
    // digits the string had before the trailing zeros were stripped.
    Finite(bool, Vec<u8>, i64, usize),
    // The exponent is too large for the value to be finite.
    Overflow(bool),
}

// Reads `s` in the base given by `options`, returning `None` if it is not a valid number.
fn parse(s: &str, options: FromSciStringOptions) -> Option<Parsed> {
    // The special values, spelled as `Float`'s `Display` writes them.
    match s {
        "NaN" => return Some(Parsed::Special(Float::NAN)),
        "Infinity" => return Some(Parsed::Special(Float::INFINITY)),
        "-Infinity" => return Some(Parsed::Special(Float::NEGATIVE_INFINITY)),
        _ => {}
    }
    let base = options.get_base();
    // `preprocess_sci_string` removes the point and folds it, and any exponent, into a power of the
    // base: the result is the digit characters and an exponent `e` with the value being those
    // digits, read as an integer, times base ^ e.
    let (chars, exponent) = preprocess_sci_string(s, options)?;
    let (sign, chars) = match chars.split_first() {
        Some((&b'-', rest)) => (false, rest),
        Some((&b'+', rest)) => (true, rest),
        _ => (true, &chars[..]),
    };
    if chars.is_empty() {
        return None;
    }
    let mut digits = Vec::with_capacity(chars.len());
    for &c in chars {
        let digit = digit_from_display_byte(c)?;
        if digit >= base {
            return None;
        }
        digits.push(digit);
    }
    // Restate the value as 0.d1 d2 ... times base ^ exp_base, the form the core takes.
    let Some(mut exp_base) = i64::exact_from(digits.len()).checked_add(exponent) else {
        // Only an exponent near the top of its range can do this, and the digit count is positive,
        // so the sum can only have overflowed upwards.
        return Some(Parsed::Overflow(sign));
    };
    // Leading zeros are not significant and each one lowers the exponent.
    let leading = digits.iter().take_while(|&&d| d == 0).count();
    digits.drain(..leading);
    exp_base -= i64::exact_from(leading);
    // Every digit that is left counts towards the precision the string implies, including the
    // trailing zeros the core does not want.
    let significant = digits.len();
    while digits.last() == Some(&0) {
        digits.pop();
    }
    Some(if digits.is_empty() {
        Parsed::Zero(sign)
    } else {
        Parsed::Finite(sign, digits, exp_base, significant)
    })
}

// The precision a string implies when the caller does not give one: as many bits as its significant
// digits can carry.
fn implied_prec(significant: usize, base: u8) -> u64 {
    u64::exact_from(ceil_mul(i64::exact_from(significant), u64::from(base), 0))
}

impl Float {
    /// Converts a string, possibly in scientific notation, to a [`Float`].
    pub fn from_sci_string_prec_round(
        s: &str,
        prec: u64,
        rm: RoundingMode,
    ) -> Option<(Self, Ordering)> {
        let mut options = FromSciStringOptions::default();
        options.set_rounding_mode(rm);
        Self::from_sci_string_with_options_prec(s, options, prec)
    }

    /// Converts a string, possibly in scientific notation, to a [`Float`], rounding to nearest.
    #[inline]
    pub fn from_sci_string_prec(s: &str, prec: u64) -> Option<(Self, Ordering)> {
        Self::from_sci_string_with_options_prec(s, FromSciStringOptions::default(), prec)
    }

    /// Converts a string, possibly in scientific notation, to a [`Float`], using the given options
    /// for the base and the rounding mode.
    pub fn from_sci_string_with_options_prec(
        s: &str,
        options: FromSciStringOptions,
        prec: u64,
    ) -> Option<(Self, Ordering)> {
        assert_ne!(prec, 0);
        let rm = options.get_rounding_mode();
        Some(match parse(s, options)? {
            Parsed::Special(x) => (x, Equal),
            Parsed::Zero(sign) => (
                if sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            Parsed::Overflow(sign) => overflow(sign, prec, rm),
            Parsed::Finite(sign, digits, exp_base, _) => {
                set_str_helper(sign, &digits, options.get_base(), exp_base, 0, prec, rm)
            }
        })
    }
}

// The base prefix `to_string.rs` writes for a power-of-two base, if any.
const fn base_prefix(base: u8) -> &'static str {
    match base {
        2 => "0b",
        8 => "0o",
        16 => "0x",
        _ => "",
    }
}

pub_crate_test! {
// Reads what `ComparableFloat` writes in the given base: an optional sign, an optional base prefix,
// the digits, and an optional `#` and precision. Without the suffix the precision is inferred from
// the digits.
float_from_string_base(base: u8, s: &str) -> Option<Float> {
    // Built first, so that an invalid base is rejected whatever the string is; otherwise the
    // strings that fail below would quietly return `None` for a base that cannot exist.
    let mut options = FromSciStringOptions::default();
    options.set_base(base);
    let (body, prec) = match s.rfind('#') {
        Some(i) => (&s[..i], Some(u64::from_str(&s[i + 1..]).ok()?)),
        None => (s, None),
    };
    // A precision of zero is not a `Float` precision, and the specials and zeros never carry one.
    if prec == Some(0) {
        return None;
    }
    let (sign, after_sign) = match body.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", body),
    };
    // The prefix is optional: `{:x}` omits it and `{:#x}` writes it.
    let prefix = base_prefix(base);
    let digits = if prefix.is_empty() {
        after_sign
    } else {
        after_sign.strip_prefix(prefix).unwrap_or(after_sign)
    };
    // `from_sci_string` reads the sign itself, and the specials only in their unprefixed spelling.
    let rebuilt = if sign.is_empty() && prefix.is_empty() {
        Cow::Borrowed(body)
    } else {
        Cow::Owned(format!("{sign}{digits}"))
    };
    match prec {
        Some(prec) => {
            let x = Float::from_sci_string_with_options_prec(&rebuilt, options, prec)?.0;
            // A suffix that a special or a zero cannot carry is not something they wrote.
            x.get_prec().map(|_| x)
        }
        None => Float::from_sci_string_with_options(&rebuilt, options),
    }
}}

impl FromSciString for Float {
    /// Converts a string, possibly in scientific notation, to a [`Float`], inferring a precision
    /// from the number of digits.
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Self> {
        let base = options.get_base();
        Some(match parse(s, options)? {
            Parsed::Special(x) => x,
            Parsed::Zero(sign) => {
                if sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                }
            }
            // This is `overflow(sign, _, Nearest)`, which is an infinity whatever the precision.
            // That is as well, since the digits cannot pin one down: an overflowing exponent says
            // nothing about how many of them there were.
            Parsed::Overflow(sign) => {
                if sign {
                    Self::INFINITY
                } else {
                    Self::NEGATIVE_INFINITY
                }
            }
            Parsed::Finite(sign, digits, exp_base, significant) => {
                let prec = implied_prec(significant, base);
                let (x, o) = set_str_helper(sign, &digits, base, exp_base, 0, prec, Nearest);
                // The implied precision is an upper bound on what the digits can say; when the
                // value needs fewer bits than that, it is stored with the fewest that represent it,
                // matching `Float::from` for a primitive float. A rounded value is not shrunk: its
                // low bits are not the string's.
                //
                // An exact result here is never zero, so there is no need to guard the shrink
                // against one: the digits are nonempty with their trailing zeros stripped, so they
                // name a value of at least one, and the only way `set_str_helper` reaches zero is
                // by underflowing, which is never exact.
                if o == Equal {
                    let min_prec = (&x).integer_mantissa().significant_bits();
                    Self::from_float_prec_round(x, min_prec, Exact).0
                } else {
                    x
                }
            }
        })
    }
}
