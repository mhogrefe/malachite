// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::Finite;
use crate::conversion::string::get_str::get_str_ndigits;
use crate::conversion::string::to_sci::to_sci_string;
use crate::{ComparableFloat, ComparableFloatRef, Float};
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex, Write};
use malachite_base::num::arithmetic::traits::{DivRound, Mod, PowerOf2};
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::Ceiling;

// The number of base-2^`digit_bits` digits that exactly cover a `Float` with binary exponent
// `exponent` and precision `precision`, with the digits aligned to the base-2^`digit_bits` point:
// the first digit holds `exponent mod digit_bits` significant bits (all `digit_bits` of them when
// the exponent is a multiple), and the rest of the precision fills subsequent digits.
fn power_of_2_digit_count(exponent: i32, precision: u64, digit_bits: u64) -> u64 {
    let m = u64::exact_from(exponent.mod_op(i32::exact_from(digit_bits)));
    let mut count = precision.saturating_sub(m).div_round(digit_bits, Ceiling).0;
    if m != 0 {
        count += 1;
    }
    count
}

// Writes `x` in the base 2^`digit_bits`, with exactly enough digits to represent it. When the
// formatter's alternate flag is set, `prefix` follows the sign for zero and finite values (but not
// NaN or the infinities).
fn fmt_power_of_2_base(
    x: &Float,
    f: &mut Formatter,
    digit_bits: u64,
    uppercase: bool,
    prefix: &str,
) -> Result {
    let mut options = ToSciOptions::default();
    options.set_base(u8::power_of_2(digit_bits));
    options.set_e_uppercase();
    if uppercase {
        options.set_uppercase();
    }
    if let Float(Finite {
        exponent,
        precision,
        ..
    }) = x
    {
        options.set_precision(power_of_2_digit_count(*exponent, *precision, digit_bits));
        options.set_include_trailing_zeros(true);
    }
    let s = to_sci_string(x, options);
    if !x.is_nan() && !x.is_infinite() {
        let (sign, body) = match s.strip_prefix('-') {
            Some(body) => ("-", body),
            None => ("", s.as_str()),
        };
        f.write_str(sign)?;
        if f.alternate() {
            f.write_str(prefix)?;
        }
        f.write_str(body)
    } else {
        f.write_str(&s)
    }
}

impl Display for Float {
    /// Converts a [`Float`] to a [`String`](alloc::string::String).
    ///
    /// The output has enough base-10 digits to round-trip: `1 + ceil(p log10(2))` significant
    /// digits for a [`Float`] of precision `p` (the same count for every value of a given
    /// precision), correctly rounded to nearest. It always contains a decimal point, small and
    /// large values use scientific notation, zeros are `0.0` and `-0.0`, and the special values are
    /// `NaN`, `Infinity`, and `-Infinity`.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut options = ToSciOptions::default();
        if let Self(Finite { precision, .. }) = self {
            options.set_precision(u64::exact_from(get_str_ndigits(10, *precision)));
            options.set_include_trailing_zeros(true);
        }
        f.write_str(&to_sci_string(self, options))
    }
}

impl Debug for Float {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl Binary for Float {
    /// Converts a [`Float`] to a binary [`String`](alloc::string::String); the alternate form
    /// prefixes it with `0b`.
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 1, false, "0b")
    }
}

impl Octal for Float {
    /// Converts a [`Float`] to an octal [`String`](alloc::string::String); the alternate form
    /// prefixes it with `0o`.
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 3, false, "0o")
    }
}

impl LowerHex for Float {
    /// Converts a [`Float`] to a hexadecimal [`String`](alloc::string::String); the alternate form
    /// prefixes it with `0x`.
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 4, false, "0x")
    }
}

impl UpperHex for Float {
    /// Converts a [`Float`] to a hexadecimal [`String`](alloc::string::String) with uppercase
    /// digits; the alternate form prefixes it with `0x`.
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 4, true, "0x")
    }
}

impl Display for ComparableFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Debug for ComparableFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl LowerHex for ComparableFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        LowerHex::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Display for ComparableFloatRef<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            write!(f, "{x}")?;
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            Display::fmt(&self.0, f)
        }
    }
}

impl LowerHex for ComparableFloatRef<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            if f.alternate() {
                write!(f, "{x:#x}")?;
            } else {
                write!(f, "{x:x}")?;
            }
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            LowerHex::fmt(&self.0, f)
        }
    }
}

impl Debug for ComparableFloatRef<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
