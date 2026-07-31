// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::Finite;
use crate::float::conversion::string::get_str::get_str_digit_count;
use crate::float::conversion::string::to_sci::to_sci_string;
use crate::{ComparableFloat, ComparableFloatRef, Float};
use alloc::string::String;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex, Write};
use malachite_base::num::arithmetic::traits::{DivRound, Mod, PowerOf2};
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ExactFrom, ToStringBase};
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

// The options that `to_string_base` and `to_string_base_upper` share, chosen so that each base
// agrees with the corresponding formatting impl: a power-of-2 base writes the exact digits, as
// `Binary`, `Octal`, and the hexadecimal impls do, and any other base writes the round-trip digit
// count, as `Display` does in base 10.
fn to_string_base_options(x: &Float, base: u8, uppercase: bool) -> ToSciOptions {
    let mut options = ToSciOptions::default();
    options.set_base(base);
    if uppercase {
        options.set_uppercase();
        // so that the whole string, exponent marker included, is the uppercase of the lowercase
        // form; from base 15 up the mandatory sign on the exponent keeps `E` the digit distinct
        // from `E` the marker
        options.set_e_uppercase();
    }
    if base.is_power_of_two() {
        options.set_e_uppercase();
        if let Float(Finite {
            exponent,
            precision,
            ..
        }) = x
        {
            options.set_precision(power_of_2_digit_count(
                *exponent,
                *precision,
                u64::from(base.trailing_zeros()),
            ));
            options.set_include_trailing_zeros(true);
        }
    } else if let Float(Finite { precision, .. }) = x {
        options.set_precision(u64::exact_from(get_str_digit_count(
            u64::from(base),
            *precision,
        )));
        options.set_include_trailing_zeros(true);
    }
    options
}

impl ToStringBase for Float {
    /// Converts a [`Float`] to a [`String`] using a specified base.
    ///
    /// Digits from 0 to 9 become [`char`]s from `'0'` to `'9'`, and digits from 10 to 35 become the
    /// lowercase [`char`]s `'a'` to `'z'`.
    ///
    /// The output agrees with the formatting impls: base 10 writes what
    /// [`Display`](std::fmt::Display) writes, and bases 2, 8, and 16 write what `{:b}`, `{:o}`, and
    /// `{:x}` write, without the prefix that the `#` flag would add. The number of digits follows
    /// from that. In a power-of-2 base the value is exactly representable, so exactly enough digits
    /// are written to reproduce it; in any other base the count is the one that round-trips a
    /// [`Float`] of this precision, with trailing zeros kept to reach it. The count therefore
    /// depends only on the precision, so a printed string does not by itself determine a [`Float`];
    /// see [`ComparableFloat`], whose output also records the precision.
    ///
    /// Values whose exponent is far from zero use scientific notation. From base 15 upward the
    /// exponent always carries an explicit sign, since `'e'` is a digit in those bases and the sign
    /// is what distinguishes the exponent from the digits.
    ///
    /// The special values are `NaN`, `Infinity`, and `-Infinity` in every base, and the zeros are
    /// `0.0` and `-0.0`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36. Unlike
    /// [`Natural`](malachite_nz::natural::Natural) and [`Integer`](malachite_nz::integer::Integer),
    /// whose strings reach base 62, a [`Float`] is limited to base 36 in both directions: see
    /// [`FromStringBase`](malachite_base::num::conversion::traits::FromStringBase), which this
    /// inverts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(255).to_string_base(10), "255.0");
    /// assert_eq!(Float::from(255).to_string_base(16), "ff.0");
    /// assert_eq!(Float::from(255).to_string_base(2), "11111111.0");
    /// assert_eq!(Float::from(1.5).to_string_base(10), "1.5");
    /// assert_eq!(Float::from(1.5).to_string_base(16), "1.8");
    ///
    /// // base 10 agrees with `Display`, and base 16 with `{:x}`
    /// let x = Float::from(core::f64::consts::PI);
    /// assert_eq!(x.to_string_base(10), x.to_string());
    /// assert_eq!(x.to_string_base(16), format!("{x:x}"));
    /// ```
    fn to_string_base(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        to_sci_string(self, to_string_base_options(self, base, false))
    }

    /// Converts a [`Float`] to a [`String`] using a specified base, with digits being uppercase.
    ///
    /// Digits from 0 to 9 become [`char`]s from `'0'` to `'9'`, and digits from 10 to 35 become the
    /// uppercase [`char`]s `'A'` to `'Z'`.
    ///
    /// This is [`to_string_base`](ToStringBase::to_string_base) with the whole string uppercased,
    /// the exponent marker included; in base 16 it writes what `{:X}` writes, without the prefix
    /// that the `#` flag would add. The special values `NaN`, `Infinity`, and `-Infinity` keep
    /// their spelling, as they do in every base.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(255).to_string_base_upper(16), "FF.0");
    /// assert_eq!(Float::from(1.5).to_string_base_upper(16), "1.8");
    ///
    /// let x = Float::from(core::f64::consts::PI);
    /// assert_eq!(x.to_string_base_upper(16), format!("{x:X}"));
    /// ```
    fn to_string_base_upper(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        to_sci_string(self, to_string_base_options(self, base, true))
    }
}

impl Display for Float {
    /// Converts a [`Float`] to a [`String`].
    ///
    /// The output has enough digits to round-trip: a [`Float`] of precision $p$ is written with
    /// $1+\lceil p \log_{10} 2 \rceil$ significant digits, correctly rounded to nearest. That count
    /// depends only on the precision, so it is the same for every value of a given precision, and
    /// trailing zeros are kept to reach it; a value of precision 1 prints as `"1.0"` where the same
    /// value at precision 100 prints as `"1.0000000000000000000000000000000"`. A printed string
    /// therefore does not by itself determine a [`Float`]; see [`ComparableFloat`], whose output
    /// also records the precision.
    ///
    /// The output of a finite value always contains a point. Values whose exponent is far from zero
    /// use scientific notation, zeros are `0.0` and `-0.0`, and the special values are `NaN`,
    /// `Infinity`, and `-Infinity`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_string(), "NaN");
    /// assert_eq!(Float::INFINITY.to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_INFINITY.to_string(), "-Infinity");
    /// assert_eq!(Float::ZERO.to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.to_string(), "-0.0");
    ///
    /// assert_eq!(Float::ONE.to_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_string(), "1.5");
    /// assert_eq!(Float::from(255).to_string(), "255.0");
    /// assert_eq!(
    ///     Float::from(core::f64::consts::PI).to_string(),
    ///     "3.1415926535897931"
    /// );
    ///
    /// // The digit count is determined by the precision, not by the value.
    /// assert_eq!(
    ///     Float::one_prec(100).to_string(),
    ///     "1.0000000000000000000000000000000"
    /// );
    ///
    /// // Values far from 1 use scientific notation.
    /// assert_eq!(Float::power_of_2(100u64).to_string(), "1.3e30");
    /// assert_eq!(Float::power_of_2(-100i64).to_string(), "7.9e-31");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut options = ToSciOptions::default();
        if let Self(Finite { precision, .. }) = self {
            options.set_precision(u64::exact_from(get_str_digit_count(10, *precision)));
            options.set_include_trailing_zeros(true);
        }
        f.write_str(&to_sci_string(self, options))
    }
}

impl Debug for Float {
    /// Converts a [`Float`] to a [`String`].
    ///
    /// This is the same implementation as for [`Display`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_debug_string(), "NaN");
    /// assert_eq!(Float::ZERO.to_debug_string(), "0.0");
    /// assert_eq!(Float::ONE.to_debug_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_debug_string(), "1.5");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl Binary for Float {
    /// Converts a [`Float`] to a binary [`String`].
    ///
    /// Using the `#` format flag prepends `"0b"` to the string, after any sign.
    ///
    /// Two is a power of two, so every [`Float`] is exactly representable in this base: the output
    /// has exactly as many digits as are needed to write the value, one per bit of precision, and
    /// is never rounded. The exponent, when one is shown, is a decimal number following an `E`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NaN, One, Zero};
    /// use malachite_base::strings::ToBinaryString;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_binary_string(), "NaN");
    /// assert_eq!(Float::ZERO.to_binary_string(), "0.0");
    /// assert_eq!(Float::ONE.to_binary_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_binary_string(), "1.1");
    /// assert_eq!(Float::from(255).to_binary_string(), "11111111.0");
    /// assert_eq!(Float::power_of_2(100u64).to_binary_string(), "1.0E100");
    ///
    /// assert_eq!(format!("{:#b}", Float::ZERO), "0b0.0");
    /// assert_eq!(format!("{:#b}", Float::from(1.5)), "0b1.1");
    /// assert_eq!(format!("{:#b}", Float::from(-1.5)), "-0b1.1");
    /// // The specials are never prefixed.
    /// assert_eq!(format!("{:#b}", Float::NAN), "NaN");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 1, false, "0b")
    }
}

impl Octal for Float {
    /// Converts a [`Float`] to an octal [`String`].
    ///
    /// Using the `#` format flag prepends `"0o"` to the string, after any sign.
    ///
    /// Eight is a power of two, so every [`Float`] is exactly representable in this base: the
    /// output has exactly as many digits as are needed to write the value, and is never rounded.
    /// The exponent, when one is shown, is a decimal number following an `E`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NaN, One, Zero};
    /// use malachite_base::strings::ToOctalString;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_octal_string(), "NaN");
    /// assert_eq!(Float::ZERO.to_octal_string(), "0.0");
    /// assert_eq!(Float::ONE.to_octal_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_octal_string(), "1.4");
    /// assert_eq!(Float::from(255).to_octal_string(), "377.0");
    /// assert_eq!(Float::power_of_2(100u64).to_octal_string(), "2.0E33");
    ///
    /// assert_eq!(format!("{:#o}", Float::ZERO), "0o0.0");
    /// assert_eq!(format!("{:#o}", Float::from(1.5)), "0o1.4");
    /// assert_eq!(format!("{:#o}", Float::from(-1.5)), "-0o1.4");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 3, false, "0o")
    }
}

impl LowerHex for Float {
    /// Converts a [`Float`] to a hexadecimal [`String`], using lowercase digits.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string, after any sign.
    ///
    /// Sixteen is a power of two, so every [`Float`] is exactly representable in this base: the
    /// output has exactly as many digits as are needed to write the value, and is never rounded.
    /// The exponent, when one is shown, is a decimal number following an `E`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NaN, One, Zero};
    /// use malachite_base::strings::ToLowerHexString;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_lower_hex_string(), "NaN");
    /// assert_eq!(Float::ZERO.to_lower_hex_string(), "0.0");
    /// assert_eq!(Float::ONE.to_lower_hex_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_lower_hex_string(), "1.8");
    /// assert_eq!(Float::from(255).to_lower_hex_string(), "ff.0");
    /// assert_eq!(Float::power_of_2(100u64).to_lower_hex_string(), "1.0E+25");
    ///
    /// assert_eq!(format!("{:#x}", Float::ZERO), "0x0.0");
    /// assert_eq!(format!("{:#x}", Float::from(1.5)), "0x1.8");
    /// assert_eq!(format!("{:#x}", Float::from(-1.5)), "-0x1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 4, false, "0x")
    }
}

impl UpperHex for Float {
    /// Converts a [`Float`] to a hexadecimal [`String`], using uppercase digits.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string, after any sign. As for the
    /// primitive integers, the prefix stays lowercase.
    ///
    /// This is the same as [`LowerHex`] apart from the case of the digits; see it for the
    /// properties of the base.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One, Zero};
    /// use malachite_base::strings::ToUpperHexString;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_upper_hex_string(), "NaN");
    /// assert_eq!(Float::ZERO.to_upper_hex_string(), "0.0");
    /// assert_eq!(Float::ONE.to_upper_hex_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_upper_hex_string(), "1.8");
    /// assert_eq!(Float::from(255).to_upper_hex_string(), "FF.0");
    ///
    /// assert_eq!(format!("{:#X}", Float::from(255)), "0xFF.0");
    /// assert_eq!(format!("{:#X}", Float::from(-1.5)), "-0x1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_power_of_2_base(self, f, 4, true, "0x")
    }
}

impl Display for ComparableFloat {
    /// Converts a [`ComparableFloat`] to a [`String`].
    ///
    /// This is the same implementation as for [`ComparableFloatRef`]: the wrapped [`Float`]'s
    /// [`Display`] output, followed by `#` and the precision.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(ComparableFloat(Float::ONE).to_string(), "1.0#1");
    /// assert_eq!(ComparableFloat(Float::one_prec(100)).to_string().len(), 37);
    /// assert_eq!(ComparableFloat(Float::from(1.5)).to_string(), "1.5#2");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Debug for ComparableFloat {
    /// Converts a [`ComparableFloat`] to a [`String`].
    ///
    /// This is the same implementation as for [`Display`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(ComparableFloat(Float::ONE).to_debug_string(), "1.0#1");
    /// assert_eq!(ComparableFloat(Float::from(1.5)).to_debug_string(), "1.5#2");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl LowerHex for ComparableFloat {
    /// Converts a [`ComparableFloat`] to a hexadecimal [`String`].
    ///
    /// This is the same implementation as for [`ComparableFloatRef`]: the wrapped [`Float`]'s
    /// [`LowerHex`] output, followed by `#` and the precision. Using the `#` format flag prepends
    /// `"0x"` to the value, after any sign.
    ///
    /// This is the form that identifies a [`Float`] exactly, and the one the tests use as their
    /// canonical label: the digits are exact because the base is a power of two, and the suffix
    /// records the precision, which the digits alone may not determine.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(format!("{:x}", ComparableFloat(Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#x}", ComparableFloat(Float::ONE)), "0x1.0#1");
    /// assert_eq!(
    ///     format!("{:#x}", ComparableFloat(Float::from(1.5))),
    ///     "0x1.8#2"
    /// );
    /// assert_eq!(
    ///     format!("{:#x}", ComparableFloat(Float::from(-1.5))),
    ///     "-0x1.8#2"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        LowerHex::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Binary for ComparableFloat {
    /// Converts a [`ComparableFloat`] to a binary [`String`].
    ///
    /// This is the same implementation as for [`ComparableFloatRef`]: the wrapped [`Float`]'s
    /// [`Binary`] output, followed by `#` and the precision. Using the `#` format flag prepends
    /// `"0b"` to the value, after any sign.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(format!("{:b}", ComparableFloat(Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#b}", ComparableFloat(Float::ONE)), "0b1.0#1");
    /// assert_eq!(
    ///     format!("{:#b}", ComparableFloat(Float::from(-1.5))),
    ///     "-0b1.1#2"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Binary::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Octal for ComparableFloat {
    /// Converts a [`ComparableFloat`] to an octal [`String`].
    ///
    /// This is the same implementation as for [`ComparableFloatRef`]: the wrapped [`Float`]'s
    /// [`Octal`] output, followed by `#` and the precision. Using the `#` format flag prepends
    /// `"0o"` to the value, after any sign.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(format!("{:o}", ComparableFloat(Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#o}", ComparableFloat(Float::ONE)), "0o1.0#1");
    /// assert_eq!(
    ///     format!("{:#o}", ComparableFloat(Float::from(-1.5))),
    ///     "-0o1.4#2"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Octal::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl UpperHex for ComparableFloat {
    /// Converts a [`ComparableFloat`] to a hexadecimal [`String`].
    ///
    /// This is the same implementation as for [`ComparableFloatRef`]: the wrapped [`Float`]'s
    /// [`UpperHex`] output, followed by `#` and the precision. Using the `#` format flag prepends
    /// `"0x"` to the value, after any sign.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(format!("{:X}", ComparableFloat(Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#X}", ComparableFloat(Float::ONE)), "0x1.0#1");
    /// assert_eq!(
    ///     format!("{:#X}", ComparableFloat(Float::from(255))),
    ///     "0xFF.0#8"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        UpperHex::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Display for ComparableFloatRef<'_> {
    /// Converts a [`ComparableFloatRef`] to a [`String`].
    ///
    /// The output is the wrapped [`Float`]'s [`Display`] output, followed by `#` and the precision,
    /// as in `"1.5#2"`. Because a [`Float`]'s decimal digits do not determine its precision, the
    /// suffix is what makes the output identify the value that [`ComparableFloatRef`]'s [`Eq`]
    /// compares. The special values and the zeros have no precision, so they are written exactly as
    /// [`Float`] writes them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One, Zero};
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(ComparableFloatRef(&Float::ONE).to_string(), "1.0#1");
    /// assert_eq!(ComparableFloatRef(&Float::from(1.5)).to_string(), "1.5#2");
    /// assert_eq!(ComparableFloatRef(&Float::from(255)).to_string(), "255.0#8");
    ///
    /// // The specials and the zeros carry no precision.
    /// assert_eq!(ComparableFloatRef(&Float::NAN).to_string(), "NaN");
    /// assert_eq!(ComparableFloatRef(&Float::ZERO).to_string(), "0.0");
    /// ```
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
    /// Converts a [`ComparableFloatRef`] to a hexadecimal [`String`].
    ///
    /// The output is the wrapped [`Float`]'s [`LowerHex`] output, followed by `#` and the
    /// precision, as in `"1.8#2"`. Using the `#` format flag prepends `"0x"` to the value, after
    /// any sign, giving `"0x1.8#2"`.
    ///
    /// This is the form that identifies a [`Float`] exactly: the digits are exact because the base
    /// is a power of two, and the suffix supplies the precision. It is also what a base-16
    /// [`FromStringBase`](malachite_base::num::conversion::traits::FromStringBase) parse accepts,
    /// so the two round-trip, which is why the tests use it as their canonical label.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(format!("{:x}", ComparableFloatRef(&Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#x}", ComparableFloatRef(&Float::ONE)), "0x1.0#1");
    /// assert_eq!(
    ///     format!("{:#x}", ComparableFloatRef(&Float::from(1.5))),
    ///     "0x1.8#2"
    /// );
    /// assert_eq!(
    ///     format!("{:#x}", ComparableFloatRef(&Float::from(255))),
    ///     "0xff.0#8"
    /// );
    /// assert_eq!(format!("{:#x}", ComparableFloatRef(&Float::NAN)), "NaN");
    /// ```
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

impl Binary for ComparableFloatRef<'_> {
    /// Converts a [`ComparableFloatRef`] to a binary [`String`].
    ///
    /// The output is the wrapped [`Float`]'s [`Binary`] output, followed by `#` and the precision.
    /// Using the `#` format flag prepends `"0b"` to the value, after any sign.
    ///
    /// Like the hexadecimal form, this identifies a [`Float`] exactly: the digits are exact because
    /// the base is a power of two, and the suffix supplies the precision, which the digits alone
    /// may not determine.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(format!("{:b}", ComparableFloatRef(&Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#b}", ComparableFloatRef(&Float::ONE)), "0b1.0#1");
    /// assert_eq!(
    ///     format!("{:#b}", ComparableFloatRef(&Float::from(1.5))),
    ///     "0b1.1#2"
    /// );
    /// assert_eq!(
    ///     format!("{:#b}", ComparableFloatRef(&Float::from(255))),
    ///     "0b11111111.0#8"
    /// );
    /// assert_eq!(format!("{:#b}", ComparableFloatRef(&Float::NAN)), "NaN");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            if f.alternate() {
                write!(f, "{x:#b}")?;
            } else {
                write!(f, "{x:b}")?;
            }
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            Binary::fmt(&self.0, f)
        }
    }
}

impl Octal for ComparableFloatRef<'_> {
    /// Converts a [`ComparableFloatRef`] to an octal [`String`].
    ///
    /// The output is the wrapped [`Float`]'s [`Octal`] output, followed by `#` and the precision.
    /// Using the `#` format flag prepends `"0o"` to the value, after any sign.
    ///
    /// Like the hexadecimal form, this identifies a [`Float`] exactly: the digits are exact because
    /// the base is a power of two, and the suffix supplies the precision, which the digits alone
    /// may not determine.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(format!("{:o}", ComparableFloatRef(&Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#o}", ComparableFloatRef(&Float::ONE)), "0o1.0#1");
    /// assert_eq!(
    ///     format!("{:#o}", ComparableFloatRef(&Float::from(1.5))),
    ///     "0o1.4#2"
    /// );
    /// assert_eq!(
    ///     format!("{:#o}", ComparableFloatRef(&Float::from(255))),
    ///     "0o377.0#8"
    /// );
    /// assert_eq!(format!("{:#o}", ComparableFloatRef(&Float::NAN)), "NaN");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            if f.alternate() {
                write!(f, "{x:#o}")?;
            } else {
                write!(f, "{x:o}")?;
            }
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            Octal::fmt(&self.0, f)
        }
    }
}

impl UpperHex for ComparableFloatRef<'_> {
    /// Converts a [`ComparableFloatRef`] to a hexadecimal [`String`].
    ///
    /// The output is the wrapped [`Float`]'s [`UpperHex`] output, followed by `#` and the
    /// precision. Using the `#` format flag prepends `"0x"` to the value, after any sign.
    ///
    /// Like the hexadecimal form, this identifies a [`Float`] exactly: the digits are exact because
    /// the base is a power of two, and the suffix supplies the precision, which the digits alone
    /// may not determine.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(format!("{:X}", ComparableFloatRef(&Float::ONE)), "1.0#1");
    /// assert_eq!(format!("{:#X}", ComparableFloatRef(&Float::ONE)), "0x1.0#1");
    /// assert_eq!(
    ///     format!("{:#X}", ComparableFloatRef(&Float::from(255))),
    ///     "0xFF.0#8"
    /// );
    /// // As for `Float`, the prefix stays lowercase, matching the primitive integers.
    /// assert_eq!(
    ///     format!("{:#X}", ComparableFloatRef(&Float::from(-1.5))),
    ///     "-0x1.8#2"
    /// );
    /// assert_eq!(format!("{:#X}", ComparableFloatRef(&Float::NAN)), "NaN");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            if f.alternate() {
                write!(f, "{x:#X}")?;
            } else {
                write!(f, "{x:X}")?;
            }
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            UpperHex::fmt(&self.0, f)
        }
    }
}

impl Debug for ComparableFloatRef<'_> {
    /// Converts a [`ComparableFloatRef`] to a [`String`].
    ///
    /// This is the same implementation as for [`Display`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(ComparableFloatRef(&Float::ONE).to_debug_string(), "1.0#1");
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::from(1.5)).to_debug_string(),
    ///     "1.5#2"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
