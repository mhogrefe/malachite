// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2004-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::conversion::string::set_str::{overflow, set_str_helper};
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::rounding_modes::RoundingMode;

// The largest base `strtofr` accepts.
//
// This is `MPFR_MAX_BASE` from `strtofr.c`, MPFR 4.3.0.
const MAX_BASE: u8 = 62;

// C's `isspace` in the "C" locale. Rust's `is_ascii_whitespace` is not the same: it omits the
// vertical tab.
const fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

// The value of the digit character `c` in base `base`, or `None` if `c` is not a digit of that
// base. For a base of 36 or less the letter case does not matter; above that, lowercase letters
// continue the sequence after the uppercase ones.
//
// This is `digit_value_in_base` from `strtofr.c`, MPFR 4.3.0.
const fn digit_value_in_base(c: u8, base: u8) -> Option<u8> {
    let digit = if c.is_ascii_digit() {
        c - b'0'
    } else if c.is_ascii_lowercase() {
        if base >= 37 {
            c - b'a' + 36
        } else {
            c - b'a' + 10
        }
    } else if c.is_ascii_uppercase() {
        c - b'A' + 10
    } else {
        return None;
    };
    if digit < base { Some(digit) } else { None }
}

// Whether `s` begins with `prefix`, which must be lowercase, ignoring ASCII case.
//
// This is `fast_casecmp` from `strtofr.c`, MPFR 4.3.0, returning whether the comparison succeeded.
fn starts_with_ignore_case(s: &[u8], prefix: &[u8]) -> bool {
    s.len() >= prefix.len()
        && s[..prefix.len()]
            .iter()
            .zip(prefix)
            .all(|(&c, &p)| c.to_ascii_lowercase() == p)
}

// Reads an optional sign followed by decimal digits, saturating at the bounds of `i64`. Returns the
// value and the number of bytes read, which is zero when there are no digits (in which case the
// value is zero too).
//
// This is the `strtol` call in `parse_string` from `strtofr.c`, MPFR 4.3.0, together with the
// clamping to `MPFR_EXP_MIN` and `MPFR_EXP_MAX` that follows it. Leading whitespace is not skipped:
// the caller has already checked that the first character is not a space.
fn read_exponent(s: &[u8]) -> (i64, usize) {
    let mut i = 0;
    let negative = s.first() == Some(&b'-');
    if negative || s.first() == Some(&b'+') {
        i = 1;
    }
    let start = i;
    let mut exp = 0i64;
    while let Some(&c) = s.get(i)
        && c.is_ascii_digit()
    {
        exp = exp
            .saturating_mul(10)
            .saturating_add(i64::from(c - b'0') * if negative { -1 } else { 1 });
        i += 1;
    }
    if i == start { (0, 0) } else { (exp, i) }
}

// The outcome of `parse_string`, corresponding to its return values: `Invalid` is -1, the special
// values and `Zero` are 0, `Finite` is 1, and `Overflow` is 2. The `bool` fields are signs, `true`
// meaning positive, the opposite of MPFR's `negative` field.
#[derive(Clone, Debug, Eq, PartialEq)]
enum ParsedString {
    Invalid,
    NaN,
    Infinity(bool),
    Zero(bool),
    // The sign, the resolved base, the digit values (most significant first, with leading and
    // trailing zeros stripped), the number of digits before the point plus any base exponent, and
    // any binary exponent.
    Finite(bool, u8, Vec<u8>, i64, i64),
    Overflow(bool),
}

// Parses `s` in base `base`, which is 0 (detect the base from the prefix, defaulting to 10) or
// between 2 and 62. Returns the parsed value and the number of bytes consumed, which is zero when
// the input is invalid.
//
// This is `parse_string` from `strtofr.c`, MPFR 4.3.0.
fn parse_string(s: &[u8], mut base: u8) -> (ParsedString, usize) {
    let at = |i: usize| s.get(i).copied().unwrap_or(0);
    let mut i = 0;
    // optional leading whitespace
    while at(i) != 0 && is_space(at(i)) {
        i += 1;
    }
    // an optional sign
    let sign = at(i) != b'-';
    if at(i) == b'-' || at(i) == b'+' {
        i += 1;
    }
    // a case-insensitive NaN
    let nan = if starts_with_ignore_case(&s[i..], b"@nan@") {
        i += 5;
        true
    } else if base <= 16 && starts_with_ignore_case(&s[i..], b"nan") {
        i += 3;
        true
    } else {
        false
    };
    if nan {
        // an optional "(dummychars)"
        if at(i) == b'(' {
            let mut j = i + 1;
            while at(j) != b')' {
                if !at(j).is_ascii_alphanumeric() && at(j) != b'_' {
                    break;
                }
                j += 1;
            }
            if at(j) == b')' {
                i = j + 1;
            }
        }
        return (ParsedString::NaN, i);
    }
    // a case-insensitive infinity
    let inf = if starts_with_ignore_case(&s[i..], b"@inf@") {
        i += 5;
        true
    } else if base <= 16 && starts_with_ignore_case(&s[i..], b"infinity") {
        i += 8;
        true
    } else if base <= 16 && starts_with_ignore_case(&s[i..], b"inf") {
        i += 3;
        true
    } else {
        false
    };
    if inf {
        return (ParsedString::Infinity(sign), i);
    }
    // For a base of 0 or 16 the string may carry a "0x" prefix, and for 0 or 2 a "0b" one.
    let mut prefix_index = None;
    if (base == 0 || base == 16) && at(i) == b'0' && (at(i + 1) | 0x20) == b'x' {
        prefix_index = Some(i);
        base = 16;
        i += 2;
    }
    if (base == 0 || base == 2) && at(i) == b'0' && (at(i + 1) | 0x20) == b'b' {
        prefix_index = Some(i);
        base = 2;
        i += 2;
    }
    if base == 0 {
        base = 10;
    }
    // Read the mantissa digits.
    let mut digits;
    let mut exp_base;
    let mut start = i;
    loop {
        digits = Vec::new();
        let mut point = false;
        exp_base = 0i64;
        i = start;
        // loop until an invalid character is read
        loop {
            let c = at(i);
            i += 1;
            if c == b'.' {
                if point {
                    // a second point stops the parse
                    break;
                }
                point = true;
                continue;
            }
            let Some(d) = digit_value_in_base(c, base) else {
                break;
            };
            digits.push(d);
            if !point {
                exp_base += 1;
            }
        }
        // the last character read was invalid
        i -= 1;
        if !digits.is_empty() {
            break;
        }
        // There are no digits. If a prefix was skipped, read the mantissa again without skipping
        // it, so that "0x" alone parses as the digit 0.
        let Some(p) = prefix_index else {
            return (ParsedString::Invalid, 0);
        };
        start = p;
        prefix_index = None;
    }
    // an optional exponent (e or E, p or P, @)
    let mut exp_bin = 0i64;
    let mut overflow = false;
    let c = at(i);
    if (c == b'@' || (base <= 10 && (c | 0x20) == b'e')) && !is_space(at(i + 1)) {
        let (read_exp, len) = read_exponent(&s[i + 1..]);
        if len != 0 {
            i += 1 + len;
        }
        match read_exp.checked_add(exp_base) {
            Some(sum) => exp_base = sum,
            // Since `exp_base` is nonnegative, the sum cannot overflow downwards. The overflow is
            // only recorded, not returned: a mantissa that turns out to be all zeros still parses
            // as an exact zero, which takes precedence.
            None => overflow = true,
        }
    } else if (base == 2 || base == 16) && (c | 0x20) == b'p' && !is_space(at(i + 1)) {
        let (read_exp, len) = read_exponent(&s[i + 1..]);
        if len != 0 {
            i += 1 + len;
        }
        exp_bin = read_exp;
    }
    // Remove the zeros at the beginning and the end of the mantissa.
    let mut leading = 0;
    while leading < digits.len() && digits[leading] == 0 {
        leading += 1;
        exp_base = exp_base.saturating_sub(1);
    }
    digits.drain(..leading);
    while digits.last() == Some(&0) {
        digits.pop();
    }
    if digits.is_empty() {
        return (ParsedString::Zero(sign), i);
    }
    if overflow {
        return (ParsedString::Overflow(sign), i);
    }
    (
        ParsedString::Finite(sign, base, digits, exp_base, exp_bin),
        i,
    )
}

/// Converts a string to a [`Float`], reading as much of it as forms a valid number.
///
/// Returns the value, the [`Ordering`] of the value against the exact value of the string, and the
/// number of bytes consumed, which is zero if the string does not begin with a valid number.
///
/// # Panics
/// Panics if `base` is 1 or greater than 62, or if `prec` is zero.
///
/// This is `mpfr_strtofr` from `strtofr.c`, MPFR 4.3.0.
pub fn strtofr(s: &str, base: u8, prec: u64, rm: RoundingMode) -> (Float, Ordering, usize) {
    assert!(base == 0 || (2..=MAX_BASE).contains(&base));
    assert_ne!(prec, 0);
    match parse_string(s.as_bytes(), base) {
        // An error occurred, so zero is returned; it is exact, so the ternary value is zero too.
        (ParsedString::Invalid, _) => (Float::ZERO, Equal, 0),
        (ParsedString::NaN, len) => (Float::NAN, Equal, len),
        (ParsedString::Infinity(sign), len) => (
            if sign {
                Float::INFINITY
            } else {
                Float::NEGATIVE_INFINITY
            },
            Equal,
            len,
        ),
        (ParsedString::Zero(sign), len) => (
            if sign {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            },
            Equal,
            len,
        ),
        (ParsedString::Overflow(sign), len) => {
            let (x, o) = overflow(sign, prec, rm);
            (x, o, len)
        }
        (ParsedString::Finite(sign, base, digits, exp_base, exp_bin), len) => {
            let (x, o) = set_str_helper(sign, &digits, base, exp_base, exp_bin, prec, rm);
            (x, o, len)
        }
    }
}

/// Converts a string to a [`Float`], requiring that the whole string be a valid number.
///
/// Returns the value and the [`Ordering`] of the value against the exact value of the string, or
/// `None` if the string is empty or is not entirely consumed by [`strtofr`].
///
/// # Panics
/// Panics if `base` is 1 or greater than 62, or if `prec` is zero.
///
/// This is `mpfr_set_str` from `set_str.c`, MPFR 4.3.0. MPFR's version reports only success or
/// failure, discarding the ternary value; this one returns it.
pub fn set_str(s: &str, base: u8, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    if s.is_empty() {
        return None;
    }
    let (x, o, len) = strtofr(s, base, prec, rm);
    if len == s.len() { Some((x, o)) } else { None }
}
