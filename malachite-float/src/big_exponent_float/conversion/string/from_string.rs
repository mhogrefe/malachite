// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::conversion::string::to_string::power_of_2_base_log;
use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
use malachite_base::num::conversion::traits::{FromStringBase, IntegerMantissaAndExponent};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl FromStringBase for BigExponentFloat {
    /// Converts a string, in a specified base, to a [`BigExponentFloat`]. The base must for now be
    /// a power of 2: 2, 4, 8, 16, or 32.
    ///
    /// The string is in the format that
    /// [`ToStringBase`](malachite_base::num::conversion::traits::ToStringBase) writes: an optional
    /// `-`, digits with an optional `.`, and an optional exponent, written as `E` (or `e`) followed
    /// by a decimal number, giving the power of the base by which to multiply. The number's sign is
    /// optional in bases 2, 4, and 8, and mandatory in bases 16 and 32, where `E` is also a digit.
    /// A `0b`, `0o`, or `0x` prefix after the sign is accepted in bases 2, 8, and 16 respectively,
    /// so the `#`-flag output of [`Binary`](core::fmt::Binary), [`Octal`](core::fmt::Octal), and
    /// [`LowerHex`](core::fmt::LowerHex) is accepted too. `NaN`, `Infinity`, and `-Infinity` are
    /// also accepted. Digits may be uppercase or lowercase.
    ///
    /// The value of the string is always exactly representable, so no rounding takes place. If the
    /// string is not valid, `None` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Panics
    /// Panics if `base` is not 2, 4, 8, 16, or 32.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::FromStringBase;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let x = BigExponentFloat::from_string_base(16, "-0x1.8").unwrap();
    /// assert_eq!(format!("{x:?}"), "-0x1.8");
    /// let x = BigExponentFloat::from_string_base(2, "1.1E+100000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.get_exponent().unwrap().to_string(),
    ///     "100000000000000000001"
    /// );
    /// assert!(BigExponentFloat::from_string_base(16, "1.g").is_none());
    /// ```
    fn from_string_base(base: u8, s: &str) -> Option<Self> {
        let log_base = power_of_2_base_log(base);
        match s {
            "NaN" => return Some(Self::NAN),
            "Infinity" => return Some(Self::INFINITY),
            "-Infinity" => return Some(Self::NEGATIVE_INFINITY),
            _ => {}
        }
        let (negative, s) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };
        let prefix = match base {
            2 => "0b",
            8 => "0o",
            16 => "0x",
            _ => "",
        };
        let s = if prefix.is_empty() {
            s
        } else {
            s.strip_prefix(prefix).unwrap_or(s)
        };
        // The exponent marker is an `E` or `e`. In bases from 15 up, where `E` is also a digit, it
        // must be followed by a sign, which no digit is; in smaller bases the sign is optional.
        let marker = |i: usize, c: char| {
            (c == 'E' || c == 'e') && (base < 15 || s[i + 1..].starts_with(['+', '-']))
        };
        let (body, exponent) = match s.char_indices().find(|&(i, c)| marker(i, c)) {
            Some((i, _)) => (
                &s[..i],
                Integer::from_str(s[i + 1..].trim_start_matches('+')).ok()?,
            ),
            None => (s, Integer::ZERO),
        };
        let (int_digits, frac_digits) = body.split_once('.').unwrap_or((body, ""));
        if int_digits.is_empty() && frac_digits.is_empty() {
            return None;
        }
        let mut digits = alloc::string::String::with_capacity(body.len());
        digits.push_str(int_digits);
        digits.push_str(frac_digits);
        let mantissa = Natural::from_string_base(base, &digits)?;
        // The value is mantissa * base^(exponent - fraction digits).
        let power = (exponent - Integer::from(frac_digits.len())) * Integer::from(log_base);
        let x = Self::from_integer_mantissa_and_exponent(mantissa, power).unwrap();
        Some(if negative && !x.is_zero() {
            x.negate()
        } else {
            x
        })
    }
}
