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
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{
    FromSciString, FromStringBase, IntegerMantissaAndExponent,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl FromStringBase for BigExponentFloat {
    /// Converts a string, in a specified base, to a [`BigExponentFloat`].
    ///
    /// In a power-of-2 base (2, 4, 8, 16, or 32) the string is in the format that
    /// [`ToStringBase`](malachite_base::num::conversion::traits::ToStringBase) writes: an optional
    /// `-`, digits with an optional `.`, and an optional exponent, written as `E` (or `e`) followed
    /// by a decimal number, giving the power of the base by which to multiply. The number's sign is
    /// optional in bases 2, 4, and 8, and mandatory in bases 16 and 32, where `E` is also a digit.
    /// A `0b`, `0o`, or `0x` prefix after the sign is accepted in bases 2, 8, and 16 respectively,
    /// so the `#`-flag output of [`Binary`](core::fmt::Binary), [`Octal`](core::fmt::Octal), and
    /// [`LowerHex`](core::fmt::LowerHex) is accepted too. The value of such a string is always
    /// exactly representable, so no rounding takes place.
    ///
    /// In any other base, from 3 to 36, this is
    /// [`from_sci_string_with_options`](malachite_base::num::conversion::traits::FromSciString::from_sci_string_with_options)
    /// in that base: the precision is inferred from the digits, and the value is rounded to
    /// nearest.
    ///
    /// In every base `NaN`, `Infinity`, and `-Infinity` are accepted, digits may be uppercase or
    /// lowercase, and the exponent may be arbitrarily large. If the string is not valid, `None` is
    /// returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::FromStringBase;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let x = BigExponentFloat::from_string_base(16, "-0x1.8").unwrap();
    /// assert_eq!(x.to_string(), "-1.5");
    /// let x = BigExponentFloat::from_string_base(2, "1.1E+100000000000000000000").unwrap();
    /// assert_eq!(
    ///     x.get_exponent().unwrap().to_string(),
    ///     "100000000000000000001"
    /// );
    /// assert!(BigExponentFloat::from_string_base(16, "1.g").is_none());
    ///
    /// let x = BigExponentFloat::from_string_base(10, "0.1").unwrap();
    /// assert_eq!(x.to_string(), "0.102");
    /// let x = BigExponentFloat::from_string_base(3, "1.1e100000000000000000000").unwrap();
    /// assert_eq!(x.to_string(), "4.3e47712125471966243729");
    /// ```
    fn from_string_base(base: u8, s: &str) -> Option<Self> {
        assert!((2..=36).contains(&base), "base out of range");
        if !base.is_power_of_two() {
            let mut options = FromSciStringOptions::default();
            options.set_base(base);
            return Self::from_sci_string_with_options(s, options);
        }
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

impl FromStr for BigExponentFloat {
    type Err = ();

    /// Converts a string to a [`BigExponentFloat`].
    ///
    /// This is [`from_string_base`](BigExponentFloat::from_string_base) in base 10, so it reads
    /// what [`Display`](core::fmt::Display) writes. The precision is inferred from the digits and
    /// the value is rounded to nearest, so `BigExponentFloat::from_str(&x.to_string())` is in
    /// general only close to `x`; [`Debug`](core::fmt::Debug)'s hexadecimal output, which
    /// [`from_string_base`](BigExponentFloat::from_string_base) reads exactly in base 16, is the
    /// exact round trip.
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
    /// use core::str::FromStr;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from_str("1.5").unwrap().to_string(),
    ///     "1.5"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from_str("NaN").unwrap().to_string(),
    ///     "NaN"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from_str("-Infinity").unwrap().to_string(),
    ///     "-Infinity"
    /// );
    /// // There is only one zero.
    /// assert_eq!(
    ///     BigExponentFloat::from_str("-0.0").unwrap().to_string(),
    ///     "0.0"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from_str("2.5e-100000000000000000000")
    ///         .unwrap()
    ///         .to_string(),
    ///     "2.49e-100000000000000000000"
    /// );
    ///
    /// assert!(BigExponentFloat::from_str("abc").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_string_base(10, s).ok_or(())
    }
}
