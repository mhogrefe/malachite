// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
//
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
use crate::float::conversion::string::get_str::ceil_mul;
use crate::float::conversion::string::set_str::{overflow, set_str_helper};
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
    /// Converts a string, possibly in scientific notation, to a [`Float`], with a given precision
    /// and rounding mode.
    ///
    /// The string is read in base 10; use
    /// [`from_sci_string_with_options_prec`](Float::from_sci_string_with_options_prec) for another
    /// base. The result is the string's exact value rounded once to `prec` bits with `rm`, together
    /// with the [`Ordering`] of the result against that exact value. `None` means the string is not
    /// a number.
    ///
    /// See [`from_sci_string_with_options_prec`](Float::from_sci_string_with_options_prec) for the
    /// grammar and for the treatment of the special values and of zero.
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
    /// use malachite_float::Float;
    ///
    /// let s = |s, prec, rm| {
    ///     Float::from_sci_string_prec_round(s, prec, rm).map(|(x, o)| (x.to_string(), o))
    /// };
    ///
    /// assert_eq!(s("1.5", 10, Nearest), Some(("1.5000".to_string(), Equal)));
    ///
    /// // 0.1 is not representable in binary, so it is rounded and the `Ordering` gives the
    /// // direction.
    /// assert_eq!(s("0.1", 4, Floor), Some(("0.0938".to_string(), Less)));
    /// assert_eq!(s("0.1", 4, Ceiling), Some(("0.102".to_string(), Greater)));
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

    /// Converts a string, possibly in scientific notation, to a [`Float`], with a given precision,
    /// rounding to nearest.
    ///
    /// This is [`from_sci_string_prec_round`](Float::from_sci_string_prec_round) with `Nearest`.
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
    /// use malachite_float::Float;
    ///
    /// let s = |s, prec| Float::from_sci_string_prec(s, prec).map(|(x, o)| (x.to_string(), o));
    ///
    /// assert_eq!(s("1.5", 10), Some(("1.5000".to_string(), Equal)));
    /// assert_eq!(s("0.1", 4), Some(("0.102".to_string(), Greater)));
    /// assert_eq!(
    ///     s("1e10", 53),
    ///     Some(("10000000000.000000".to_string(), Equal))
    /// );
    /// assert_eq!(s("abc", 10), None);
    /// ```
    #[inline]
    pub fn from_sci_string_prec(s: &str, prec: u64) -> Option<(Self, Ordering)> {
        Self::from_sci_string_with_options_prec(s, FromSciStringOptions::default(), prec)
    }

    /// Converts a string, possibly in scientific notation, to a [`Float`], with a given precision,
    /// using the given options for the base and the rounding mode.
    ///
    /// The result is the string's exact value rounded once to `prec` bits, together with the
    /// [`Ordering`] of the result against that exact value. `None` means the string is not a
    /// number; it never means the value is out of range, since a value too large in magnitude gives
    /// an infinity (or, under a mode that rounds toward zero, the largest finite value) and one too
    /// small gives a zero.
    ///
    /// Use [`FromSciStringOptions`] to specify the base, from 2 to 36 inclusive, and the rounding
    /// mode. This is the grammar the rest of Malachite uses, so a [`Float`] reads a string the same
    /// way a [`Rational`](malachite_q::Rational) does, with three additions: the strings `NaN`,
    /// `Infinity`, and `-Infinity`, which are what [`Float`]'s [`Display`](std::fmt::Display)
    /// writes and are read in every base; a signed zero, so that `-0.0` is negative zero rather
    /// than zero; and the precision and rounding mode. Note that from base 24 up `NaN` is also a
    /// valid digit string, and from base 35 up so is `Infinity`; the special value wins.
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
    /// use malachite_float::Float;
    ///
    /// let s = |s, options, prec| {
    ///     Float::from_sci_string_with_options_prec(s, options, prec)
    ///         .map(|(x, o)| (x.to_string(), o))
    /// };
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(
    ///     s("ff", options, 53),
    ///     Some(("255.00000000000000".to_string(), Equal))
    /// );
    /// // From base 15 up, an exponent needs an explicit sign, since `e` is also a digit.
    /// assert_eq!(
    ///     s("1e5", options, 20),
    ///     Some(("485.00000".to_string(), Equal))
    /// );
    /// assert_eq!(
    ///     s("1e+5", options, 20),
    ///     Some(("1048576.0".to_string(), Equal))
    /// );
    ///
    /// // The rounding mode comes from the options.
    /// options.set_base(10);
    /// options.set_rounding_mode(Floor);
    /// assert_eq!(s("0.1", options, 4), Some(("0.0938".to_string(), Less)));
    /// options.set_rounding_mode(Ceiling);
    /// assert_eq!(s("0.1", options, 4), Some(("0.102".to_string(), Greater)));
    ///
    /// // Zero keeps its sign, and an exponent too large to represent gives an infinity.
    /// assert_eq!(s("-0.0", options, 53), Some(("-0.0".to_string(), Equal)));
    /// assert_eq!(
    ///     s("1e1000000000000000000", options, 53),
    ///     Some(("Infinity".to_string(), Greater))
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

crate_test_fn! {
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
    ///
    /// The grammar, the base, and the treatment of the special values and of zero are as in
    /// [`from_sci_string_with_options_prec`](Float::from_sci_string_with_options_prec), which this
    /// differs from only in where the precision comes from. The rounding mode option is ignored;
    /// the value is rounded to nearest.
    ///
    /// A string does not say how precise it is, so the precision has to be guessed, and the guess
    /// is that its $n$ significant digits are all meaningful: $\lceil n \log_2 b \rceil$ bits. If
    /// the value is exactly representable in fewer bits than that, it is stored in the fewest that
    /// represent it, which makes a literal agree with [`Float::from`]: `"1.5"` gives precision 2
    /// and `"255"` gives 8, matching `Float::from(1.5)` and `Float::from(255)`.
    ///
    /// This is worth dwelling on, because for short strings the guess is coarse in a way that may
    /// surprise, much as
    /// [`from_sci_string_simplest`](malachite_q::Rational::from_sci_string_simplest) does for
    /// [`Rational`](malachite_q::Rational). One decimal digit buys only four bits, so `"0.1"` gives
    /// a precision-4 [`Float`] whose value is $13/128$, or 0.1015625 — not the nearest `f64` to
    /// 0.1, and not close to it. Reading `"0.1000000000000000055511151231257827"` gives that
    /// instead, and asking for a precision outright with
    /// [`from_sci_string_prec`](Float::from_sci_string_prec) avoids the question altogether. Note
    /// also that a string with a huge exponent is not thereby precise: `"1e100000000"` still has
    /// one significant digit, so it gives four bits rather than its exact 332 million.
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
    /// use malachite_float::Float;
    ///
    /// // An exactly representable value is stored in the fewest bits that represent it, so these
    /// // agree with `Float::from`.
    /// assert_eq!(Float::from_sci_string("1.5").unwrap(), Float::from(1.5));
    /// assert_eq!(Float::from_sci_string("255").unwrap(), Float::from(255));
    ///
    /// // A value that is not exactly representable keeps the precision its digits imply, which
    /// // for short strings is coarse: one decimal digit buys only four bits.
    /// assert_eq!(Float::from_sci_string("0.1").unwrap().to_string(), "0.102");
    /// assert_eq!(
    ///     Float::from_sci_string("3.14159").unwrap().to_string(),
    ///     "3.1415901"
    /// );
    ///
    /// // A huge exponent does not make the digits more precise.
    /// assert_eq!(
    ///     Float::from_sci_string("1e100000000").unwrap().to_string(),
    ///     "9.80e99999999"
    /// );
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(
    ///     Float::from_sci_string_with_options("ff", options).unwrap(),
    ///     Float::from(255)
    /// );
    ///
    /// assert!(Float::from_sci_string("abc").is_none());
    /// ```
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
