// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Scientific-string parsing driven by `FromSciStringOptions`, the counterpart of to_sci.rs. This is
// `Float`'s from_sci_string.rs with `Integer` exponents: the grammar is the one the rest of
// Malachite uses (`preprocess_sci_string` from malachite-base), except that the exponent may be
// arbitrarily large, and the digits are handed to `big_exponent_float_set_str_helper`. For a string
// whose value is within `Float`'s exponent range, the result has the value of `Float`'s.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::conversion::string::digits::big_exponent_float_set_str_helper;
use crate::float::conversion::string::get_str::ceil_mul;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
use malachite_base::num::conversion::string::from_string::digit_from_display_byte;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{ExactFrom, FromSciString};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;

// The outcome of reading a string's digits, before any rounding.
enum Parsed {
    // A NaN or an infinity.
    Special(BigExponentFloat),
    // Zero.
    Zero,
    // The sign, the digit values (most significant first, with leading and trailing zeros
    // stripped), the number of digits before the point plus any exponent, and how many significant
    // digits the string had before the trailing zeros were stripped.
    Finite(bool, Vec<u8>, Integer, usize),
}

// An exponent: an optionally signed decimal integer, as `i64::from_str` reads it, but of any size.
fn parse_exponent(s: &[u8]) -> Option<Integer> {
    let s = core::str::from_utf8(s).ok()?;
    let (negative, digits) = match s.as_bytes().first()? {
        b'+' => (false, &s[1..]),
        b'-' => (true, &s[1..]),
        _ => (false, s),
    };
    if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let e = Integer::from_str(digits).ok()?;
    Some(if negative { -e } else { e })
}

// `preprocess_sci_string` from malachite-base with an `Integer` exponent: removes the point and the
// exponent, returning the digit characters and an exponent `e` with the value being those digits,
// read as an integer, times base ^ e.
fn preprocess_sci_string(s: &str, options: FromSciStringOptions) -> Option<(Vec<u8>, Integer)> {
    let mut s = s.as_bytes().to_vec();
    let mut exponent = Integer::ZERO;
    if options.get_base() < 15 {
        for (i, &c) in s.iter().enumerate().rev() {
            if c == b'e' || c == b'E' {
                if i == 0 || i == s.len() - 1 {
                    return None;
                }
                exponent = parse_exponent(&s[i + 1..])?;
                s.truncate(i);
                break;
            }
        }
    } else {
        for (i, &c) in s.iter().enumerate().rev() {
            if c == b'+' || c == b'-' {
                if i == 0 {
                    break;
                }
                if i == 1 || i == s.len() - 1 {
                    return None;
                }
                let exp_indicator = s[i - 1];
                if exp_indicator != b'e' && exp_indicator != b'E' {
                    return None;
                }
                exponent = parse_exponent(&s[i..])?;
                s.truncate(i - 1);
                break;
            }
        }
    }
    if let Some(point_index) = s.iter().position(|&c| c == b'.') {
        let len = s.len();
        if point_index != len - 1 {
            let next_char = s[point_index + 1];
            if next_char == b'+' || next_char == b'-' {
                return None;
            }
            exponent -= Integer::from(len - point_index - 1);
            s.copy_within(point_index + 1..len, point_index);
        }
        s.pop();
    }
    Some((s, exponent))
}

// Reads `s` in the base given by `options`, returning `None` if it is not a valid number.
fn parse(s: &str, options: FromSciStringOptions) -> Option<Parsed> {
    // The special values, spelled as `Display` writes them.
    match s {
        "NaN" => return Some(Parsed::Special(BigExponentFloat::NAN)),
        "Infinity" => return Some(Parsed::Special(BigExponentFloat::INFINITY)),
        "-Infinity" => return Some(Parsed::Special(BigExponentFloat::NEGATIVE_INFINITY)),
        _ => {}
    }
    let base = options.get_base();
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
    let mut exp_base = Integer::from(digits.len()) + exponent;
    // Leading zeros are not significant and each one lowers the exponent.
    let leading = digits.iter().take_while(|&&d| d == 0).count();
    digits.drain(..leading);
    exp_base -= Integer::from(leading);
    // Every digit that is left counts towards the precision the string implies, including the
    // trailing zeros the core does not want.
    let significant = digits.len();
    while digits.last() == Some(&0) {
        digits.pop();
    }
    Some(if digits.is_empty() {
        Parsed::Zero
    } else {
        Parsed::Finite(sign, digits, exp_base, significant)
    })
}

// The precision a string implies when the caller does not give one: as many bits as its significant
// digits can carry.
pub(crate) fn implied_prec(significant: usize, base: u8) -> u64 {
    u64::exact_from(ceil_mul(i64::exact_from(significant), u64::from(base), 0))
}

impl BigExponentFloat {
    /// Converts a string, possibly in scientific notation, to a [`BigExponentFloat`], with a given
    /// precision and rounding mode.
    ///
    /// The string is read in base 10; use
    /// [`from_sci_string_with_options_prec`](BigExponentFloat::from_sci_string_with_options_prec)
    /// for another base. The result is the string's exact value rounded once to `prec` bits with
    /// `rm`, together with the [`Ordering`] of the result against that exact value. `None` means
    /// the string is not a number.
    ///
    /// See
    /// [`from_sci_string_with_options_prec`](BigExponentFloat::from_sci_string_with_options_prec)
    /// for the grammar and for the treatment of the special values and of zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(s.len(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the string's value is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let s = |s, prec, rm| {
    ///     BigExponentFloat::from_sci_string_prec_round(s, prec, rm)
    ///         .map(|(x, o)| (x.to_string(), o))
    /// };
    ///
    /// assert_eq!(s("1.5", 10, Nearest), Some(("1.5".to_string(), Equal)));
    ///
    /// // 0.1 is not representable in binary, so it is rounded and the `Ordering` gives the
    /// // direction.
    /// assert_eq!(s("0.1", 4, Floor), Some(("0.094".to_string(), Less)));
    /// assert_eq!(s("0.1", 4, Ceiling), Some(("0.102".to_string(), Greater)));
    ///
    /// // The exponent may be arbitrarily large.
    /// assert_eq!(
    ///     s("1.0e100000000000000000000", 10, Nearest),
    ///     Some(("9.9912e99999999999999999999".to_string(), Less))
    /// );
    ///
    /// assert_eq!(s("abc", 10, Nearest), None);
    /// ```
    pub fn from_sci_string_prec_round(
        s: &str,
        prec: u64,
        rm: RoundingMode,
    ) -> Option<(Self, Ordering)> {
        let mut options = FromSciStringOptions::default();
        options.set_rounding_mode(rm);
        Self::from_sci_string_with_options_prec(s, options, prec)
    }

    /// Converts a string, possibly in scientific notation, to a [`BigExponentFloat`], with a given
    /// precision, rounding to the nearest value of that precision.
    ///
    /// The string is read in base 10. This is
    /// [`from_sci_string_prec_round`](BigExponentFloat::from_sci_string_prec_round) with `Nearest`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(s.len(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let s = |s, prec| {
    ///     BigExponentFloat::from_sci_string_prec(s, prec).map(|(x, o)| (x.to_string(), o))
    /// };
    ///
    /// assert_eq!(s("1.5", 10), Some(("1.5".to_string(), Equal)));
    /// assert_eq!(s("0.1", 10), Some(("0.099976".to_string(), Less)));
    /// assert_eq!(
    ///     s("-1.0e-100000000000000000000", 10),
    ///     Some(("-1.000e-100000000000000000000".to_string(), Less))
    /// );
    /// ```
    #[inline]
    pub fn from_sci_string_prec(s: &str, prec: u64) -> Option<(Self, Ordering)> {
        Self::from_sci_string_with_options_prec(s, FromSciStringOptions::default(), prec)
    }

    /// Converts a string, possibly in scientific notation, to a [`BigExponentFloat`], with a given
    /// precision, using the given options for the base and the rounding mode.
    ///
    /// The result is the string's exact value rounded once to `prec` bits, together with the
    /// [`Ordering`] of the result against that exact value. `None` means the string is not a
    /// number. Since the exponent is unbounded, no string overflows or underflows. For a string
    /// whose value is within [`Float`](crate::Float)'s exponent range, the result has the value of
    /// [its `Float` counterpart](crate::Float::from_sci_string_with_options_prec).
    ///
    /// Use [`FromSciStringOptions`] to specify the base, from 2 to 36 inclusive, and the rounding
    /// mode. This is the grammar the rest of Malachite uses, so a [`BigExponentFloat`] reads a
    /// string the same way a [`Rational`](malachite_q::Rational) does, with two additions: the
    /// strings `NaN`, `Infinity`, and `-Infinity`, which are what [`BigExponentFloat`]'s
    /// [`Display`](std::fmt::Display) writes and are read in every base; and the precision and
    /// rounding mode. The exponent may be arbitrarily large. Note that from base 24 up `NaN` is
    /// also a valid digit string, and from base 35 up so is `Infinity`; the special value wins. A
    /// [`BigExponentFloat`] has only one zero, so `-0.0` is zero.
    ///
    /// If the base is greater than 10, the higher digits are represented by the letters `'a'`
    /// through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need to be
    /// consistent.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the base is
    /// 15 or greater, an ambiguity arises where it may not be clear whether `'e'` is a digit or an
    /// exponent indicator. To resolve this ambiguity, always use a `'+'` or `'-'` sign after the
    /// exponent indicator when the base is 15 or greater. The exponent itself is always parsed
    /// using base 10.
    ///
    /// Points are allowed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(s.len(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if the rounding mode is `Exact` but the string's value is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let s = |s, options, prec| {
    ///     BigExponentFloat::from_sci_string_with_options_prec(s, options, prec)
    ///         .map(|(x, o)| (x.to_string(), o))
    /// };
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(s("ff", options, 53), Some(("255.0".to_string(), Equal)));
    /// // From base 15 up, an exponent needs an explicit sign, since `e` is also a digit.
    /// assert_eq!(s("1e5", options, 20), Some(("485.0".to_string(), Equal)));
    /// assert_eq!(s("1e+5", options, 20), Some(("1.0e6".to_string(), Equal)));
    ///
    /// // The rounding mode comes from the options.
    /// options.set_base(10);
    /// options.set_rounding_mode(Floor);
    /// assert_eq!(s("0.1", options, 4), Some(("0.094".to_string(), Less)));
    /// options.set_rounding_mode(Ceiling);
    /// assert_eq!(s("0.1", options, 4), Some(("0.102".to_string(), Greater)));
    ///
    /// // There is one zero, and no exponent is too large.
    /// assert_eq!(s("-0.0", options, 53), Some(("0.0".to_string(), Equal)));
    /// assert_eq!(
    ///     s("1e1000000000000000000000", options, 53),
    ///     Some((
    ///         "1.0000000000000001e1000000000000000000000".to_string(),
    ///         Greater
    ///     ))
    /// );
    /// ```
    pub fn from_sci_string_with_options_prec(
        s: &str,
        options: FromSciStringOptions,
        prec: u64,
    ) -> Option<(Self, Ordering)> {
        assert_ne!(prec, 0);
        let rm = options.get_rounding_mode();
        Some(match parse(s, options)? {
            Parsed::Special(x) => (x, Equal),
            Parsed::Zero => (Self::ZERO, Equal),
            Parsed::Finite(sign, digits, exp_base, _) => big_exponent_float_set_str_helper(
                sign,
                &digits,
                options.get_base(),
                &exp_base,
                prec,
                rm,
            ),
        })
    }
}

impl FromSciString for BigExponentFloat {
    /// Converts a string, possibly in scientific notation, to a [`BigExponentFloat`].
    ///
    /// The grammar is that of
    /// [`from_sci_string_with_options_prec`](BigExponentFloat::from_sci_string_with_options_prec).
    /// Since no precision is given, the string's own digits imply one: as many bits as its
    /// significant digits (trailing zeros included) can carry, in the base given by the options.
    /// The value is rounded to nearest at that precision; the options' rounding mode is not used.
    /// `None` means the string is not a number. For a string whose value is within
    /// [`Float`](crate::Float)'s exponent range, the result has the value of
    /// [`Float`](crate::Float)'s [`FromSciString`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_base::num::conversion::traits::FromSciString;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let s = |s| BigExponentFloat::from_sci_string(s).map(|x| x.to_string());
    /// assert_eq!(s("1.5"), Some("1.5".to_string()));
    /// assert_eq!(s("0.1"), Some("0.102".to_string()));
    /// assert_eq!(
    ///     s("1e100000000000000000000"),
    ///     Some("9.8e99999999999999999999".to_string())
    /// );
    /// assert_eq!(s("NaN"), Some("NaN".to_string()));
    /// assert_eq!(s("abc"), None);
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(3);
    /// assert_eq!(
    ///     BigExponentFloat::from_sci_string_with_options("0.1", options).map(|x| x.to_string()),
    ///     Some("0.38".to_string())
    /// );
    /// ```
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Self> {
        let base = options.get_base();
        Some(match parse(s, options)? {
            Parsed::Special(x) => x,
            Parsed::Zero => Self::ZERO,
            Parsed::Finite(sign, digits, exp_base, significant) => {
                big_exponent_float_set_str_helper(
                    sign,
                    &digits,
                    base,
                    &exp_base,
                    implied_prec(significant, base),
                    Nearest,
                )
                .0
            }
        })
    }
}
