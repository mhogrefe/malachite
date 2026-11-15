// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use crate::big_exponent_float::conversion::string::to_sci::to_sci_string;
use crate::big_exponent_float::{
    BigExponentFloat, ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use crate::float::conversion::string::get_str::get_str_digit_count;
use crate::{Float, InnerFloat};
use alloc::format;
use alloc::string::String;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, Mod};
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ExactFrom, ToStringBase};
use malachite_base::strings::{ToBinaryString, ToLowerHexString, ToOctalString, ToUpperHexString};
use malachite_nz::integer::Integer;

// The string for `x` in base 2^`log_base`, exactly, as `render` writes the `Float` of the same
// value and precision. A finite nonzero `x` with an exponent in `Float`'s range is written as that
// `Float`. Otherwise, the exponent is shifted into range by a multiple of `log_base`, which changes
// neither the digits nor where they start, and the shift, in digits, is added back to the exponent
// after the `E`.
pub(crate) fn power_of_2_base_string(
    x: &BigExponentFloat,
    log_base: u64,
    render: &dyn Fn(&Float) -> String,
) -> String {
    let (sign, exponent, significand) = match x {
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => (*sign, exponent, significand),
        _ => return render(&x.to_float_clamped()),
    };
    let float = |exponent: i32| {
        Float(InnerFloat::Finite {
            sign,
            exponent,
            precision: x.get_prec().unwrap(),
            significand: significand.clone(),
        })
    };
    if let Ok(e) = i32::try_from(exponent)
        && (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&e)
    {
        return render(&float(e));
    }
    // A target exponent near the end of `Float`'s range, where `Float` writes an `E`, and congruent
    // to `exponent` mod `log_base`.
    let k = Integer::from(log_base);
    let target = if *exponent > 0u32 {
        Float::MAX_EXPONENT
            - i32::exact_from(&(Integer::from(Float::MAX_EXPONENT) - exponent).mod_op(&k))
    } else {
        Float::MIN_EXPONENT
            + i32::exact_from(&(exponent - Integer::from(Float::MIN_EXPONENT)).mod_op(&k))
    };
    let shift = (exponent - Integer::from(target)).div_exact(k);
    let s = render(&float(target));
    let e_index = s.rfind('E').unwrap();
    let printed = &s[e_index + 1..];
    // `Float` writes the sign of a positive exponent only in bases where `E` is also a digit.
    let signed = printed.starts_with(['+', '-']);
    let new_exponent = Integer::from_str(printed.trim_start_matches('+')).unwrap() + shift;
    let plus = if signed && new_exponent >= 0u32 {
        "+"
    } else {
        ""
    };
    format!("{}E{plus}{new_exponent}", &s[..e_index])
}

impl LowerHex for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to a hexadecimal [`String`], using lowercase digits.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string, after any sign.
    ///
    /// Sixteen is a power of two, so every [`BigExponentFloat`] is exactly representable in this
    /// base: the output has exactly as many digits as are needed to write the value, and is never
    /// rounded. The exponent, when one is shown, is a decimal number following an `E`. The output
    /// is the same as that of [`Float`]'s [`LowerHex`] for a [`Float`] with the same value whose
    /// precision is the [`BigExponentFloat`]'s, and continues it to exponents outside [`Float`]'s
    /// range.
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
    /// use malachite_base::num::basic::traits::{NaN, One, OneHalf, Zero};
    /// use malachite_base::strings::ToLowerHexString;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_lower_hex_string(), "NaN");
    /// assert_eq!(BigExponentFloat::ZERO.to_lower_hex_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_lower_hex_string(), "1.0");
    /// assert_eq!(BigExponentFloat::ONE_HALF.to_lower_hex_string(), "0.8");
    ///
    /// assert_eq!(format!("{:#x}", BigExponentFloat::ONE_HALF), "0x0.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        let render: &dyn Fn(&Float) -> String = if f.alternate() {
            &|x| format!("{x:#x}")
        } else {
            &|x| x.to_lower_hex_string()
        };
        f.write_str(&power_of_2_base_string(self, 4, render))
    }
}

impl UpperHex for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to a hexadecimal [`String`], using uppercase digits.
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
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_upper_hex_string(), "NaN");
    /// assert_eq!(BigExponentFloat::ZERO.to_upper_hex_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_upper_hex_string(), "1.0");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        let render: &dyn Fn(&Float) -> String = if f.alternate() {
            &|x| format!("{x:#X}")
        } else {
            &|x| x.to_upper_hex_string()
        };
        f.write_str(&power_of_2_base_string(self, 4, render))
    }
}

impl Binary for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to a binary [`String`].
    ///
    /// Using the `#` format flag prepends `"0b"` to the string, after any sign.
    ///
    /// Every [`BigExponentFloat`] is exactly representable in binary: the output has exactly as
    /// many digits as are needed to write the value. The exponent, when one is shown, is a decimal
    /// number following an `E`. The output is that of [`Float`]'s [`Binary`] for a [`Float`] with
    /// the same value and precision, continued to exponents outside [`Float`]'s range.
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
    /// use malachite_base::strings::ToBinaryString;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_binary_string(), "NaN");
    /// assert_eq!(BigExponentFloat::ZERO.to_binary_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_binary_string(), "1.0");
    /// assert_eq!(
    ///     BigExponentFloat::from(Float::from(1.5)).to_binary_string(),
    ///     "1.1"
    /// );
    /// assert_eq!(
    ///     format!("{:#b}", BigExponentFloat::from(Float::from(-1.5))),
    ///     "-0b1.1"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        let render: &dyn Fn(&Float) -> String = if f.alternate() {
            &|x| format!("{x:#b}")
        } else {
            &|x| x.to_binary_string()
        };
        f.write_str(&power_of_2_base_string(self, 1, render))
    }
}

impl Octal for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to an octal [`String`].
    ///
    /// Using the `#` format flag prepends `"0o"` to the string, after any sign.
    ///
    /// Every [`BigExponentFloat`] is exactly representable in octal: the output has exactly as many
    /// digits as are needed to write the value. The exponent, when one is shown, is a decimal
    /// number following an `E`. The output is that of [`Float`]'s [`Octal`] for a [`Float`] with
    /// the same value and precision, continued to exponents outside [`Float`]'s range.
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
    /// use malachite_base::strings::ToOctalString;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_octal_string(), "NaN");
    /// assert_eq!(BigExponentFloat::ZERO.to_octal_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_octal_string(), "1.0");
    /// assert_eq!(
    ///     BigExponentFloat::from(Float::from(1.5)).to_octal_string(),
    ///     "1.4"
    /// );
    /// assert_eq!(
    ///     format!("{:#o}", BigExponentFloat::from(Float::from(-1.5))),
    ///     "-0o1.4"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        let render: &dyn Fn(&Float) -> String = if f.alternate() {
            &|x| format!("{x:#o}")
        } else {
            &|x| x.to_octal_string()
        };
        f.write_str(&power_of_2_base_string(self, 3, render))
    }
}

// The base-2 logarithm of `base`, which must be a power of 2 from 2 to 32.
pub(crate) fn power_of_2_base_log(base: u8) -> u64 {
    assert!(
        base.is_power_of_two() && (2..=32).contains(&base),
        "{base} is not a power of 2 from 2 to 32"
    );
    u64::from(base.trailing_zeros())
}

// The options for writing a finite nonzero `x` in a base that is not a power of 2 with the digit
// count that round-trips a `Float` of `x`'s precision, as `Float`'s `Display` and `ToStringBase`
// do.
fn round_trip_options(x: &BigExponentFloat, base: u8, uppercase: bool) -> ToSciOptions {
    let mut options = ToSciOptions::default();
    options.set_base(base);
    if uppercase {
        options.set_uppercase();
        // so that the whole string, exponent marker included, is the uppercase of the lowercase
        // form; from base 15 up the mandatory sign on the exponent keeps `E` the digit distinct
        // from `E` the marker
        options.set_e_uppercase();
    }
    if let Some(prec) = x.get_prec() {
        options.set_precision(u64::exact_from(get_str_digit_count(u64::from(base), prec)));
        options.set_include_trailing_zeros(true);
    }
    options
}

impl ToStringBase for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to a [`String`] using a specified base.
    ///
    /// Digits from 0 to 9 become [`char`]s from `'0'` to `'9'`, and digits from 10 to 35 become the
    /// lowercase [`char`]s `'a'` to `'z'`.
    ///
    /// In a power-of-2 base every [`BigExponentFloat`] is exactly representable, so the output has
    /// exactly as many digits as are needed to write the value; base 10 writes what [`Display`]
    /// writes. In any other base the value is rounded to nearest, to the number of digits that
    /// round-trips a [`Float`] whose precision is the number of significant bits of `self`, with
    /// trailing zeros kept to reach it. For a value within [`Float`]'s exponent range, the output
    /// is that of [`Float`]'s [`ToStringBase`] for the [`Float`] with the same value and that
    /// precision; the exponent may be arbitrarily large.
    ///
    /// Values whose exponent is far from zero use scientific notation. From base 15 upward the
    /// exponent always carries an explicit sign, since `'e'` is a digit in those bases and the sign
    /// is what distinguishes the exponent from the digits.
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
    /// use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// let x = BigExponentFloat::from(Float::from(1.5));
    /// assert_eq!(x.to_string_base(2), "1.1");
    /// assert_eq!(x.to_string_base(3), "1.11");
    /// assert_eq!(x.to_string_base(32), "1.g");
    ///
    /// // 2^(10^20), whose decimal exponent is far beyond `Float`'s range.
    /// let x = BigExponentFloat::from_string_base(2, "1.0E+100000000000000000000").unwrap();
    /// assert_eq!(x.to_string_base(10), "2.4e30102999566398119521");
    /// ```
    fn to_string_base(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if base.is_power_of_two() {
            let log_base = power_of_2_base_log(base);
            power_of_2_base_string(self, log_base, &|x| x.to_string_base(base))
        } else {
            to_sci_string(self, round_trip_options(self, base, false))
        }
    }

    /// Converts a [`BigExponentFloat`] to a [`String`] using a specified base, with digits being
    /// uppercase.
    ///
    /// Digits from 0 to 9 become [`char`]s from `'0'` to `'9'`, and digits from 10 to 35 become the
    /// uppercase [`char`]s `'A'` to `'Z'`.
    ///
    /// This is [`to_string_base`](ToStringBase::to_string_base) with the whole string uppercased,
    /// the exponent marker included. The special values `NaN`, `Infinity`, and `-Infinity` keep
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
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// let x = BigExponentFloat::from(Float::from(1.5));
    /// assert_eq!(x.to_string_base_upper(32), "1.G");
    /// assert_eq!(x.to_string_base_upper(20), "1.A");
    /// ```
    fn to_string_base_upper(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if base.is_power_of_two() {
            let log_base = power_of_2_base_log(base);
            power_of_2_base_string(self, log_base, &|x| x.to_string_base_upper(base))
        } else {
            to_sci_string(self, round_trip_options(self, base, true))
        }
    }
}

impl Display for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to a [`String`], in base 10.
    ///
    /// The value is rounded to nearest, to the number of significant digits that round-trips a
    /// [`Float`] whose precision is the number of significant bits of `self`, with trailing zeros
    /// kept to reach it. For a value within [`Float`]'s exponent range, the output is what
    /// [`Float`]'s [`Display`] writes for the [`Float`] with the same value and that precision; the
    /// exponent may be arbitrarily large. Values whose exponent is far from zero use scientific
    /// notation, and a finite value always shows a point. `NaN` and the infinities are written as
    /// `NaN`, `Infinity`, and `-Infinity`.
    ///
    /// Since the output is rounded, it does not in general determine the value; [`Debug`], which
    /// writes the value in hexadecimal, is exact.
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_base::num::conversion::traits::FromStringBase;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_string(), "NaN");
    /// assert_eq!(BigExponentFloat::INFINITY.to_string(), "Infinity");
    /// assert_eq!(BigExponentFloat::ZERO.to_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_string(), "1.0");
    /// assert_eq!(
    ///     BigExponentFloat::from(0.1).to_string(),
    ///     "0.10000000000000001"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from(core::f64::consts::PI).to_string(),
    ///     "3.1415926535897931"
    /// );
    ///
    /// // 2^(-10^20)
    /// let x = BigExponentFloat::from_string_base(2, "1.0E-100000000000000000000").unwrap();
    /// assert_eq!(x.to_string(), "4.2e-30102999566398119522");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(&to_sci_string(self, round_trip_options(self, 10, false)))
    }
}

impl Debug for BigExponentFloat {
    /// Converts a [`BigExponentFloat`] to a [`String`], as [`LowerHex`] with the `#` flag does.
    ///
    /// The output is exact, which makes it suitable for tests and debugging.
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(format!("{:?}", BigExponentFloat::ZERO), "0x0.0");
    /// assert_eq!(format!("{:?}", BigExponentFloat::ONE), "0x1.0");
    /// assert_eq!(format!("{:?}", BigExponentFloat::NEGATIVE_ONE), "-0x1.0");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{self:#x}")
    }
}

impl Debug for ComparableBigExponentFloat {
    /// Formats a [`ComparableBigExponentFloat`] as [`Debug`] does for the wrapped
    /// [`BigExponentFloat`]: exactly, in hexadecimal.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:?}", ComparableBigExponentFloat(x)), "0x1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl LowerHex for ComparableBigExponentFloat {
    /// Formats a [`ComparableBigExponentFloat`] as [`LowerHex`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:x}", ComparableBigExponentFloat(x)), "1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for ComparableBigExponentFloat {
    /// Formats a [`ComparableBigExponentFloat`] as [`UpperHex`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:X}", ComparableBigExponentFloat(x)), "1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl Binary for ComparableBigExponentFloat {
    /// Formats a [`ComparableBigExponentFloat`] as [`Binary`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:b}", ComparableBigExponentFloat(x)), "1.1");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Binary::fmt(&self.0, f)
    }
}

impl Octal for ComparableBigExponentFloat {
    /// Formats a [`ComparableBigExponentFloat`] as [`Octal`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:o}", ComparableBigExponentFloat(x)), "1.4");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Octal::fmt(&self.0, f)
    }
}

impl Debug for ComparableBigExponentFloatRef<'_> {
    /// Formats a [`ComparableBigExponentFloatRef`] as [`Debug`] does for the wrapped
    /// [`BigExponentFloat`]: exactly, in hexadecimal.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:?}", ComparableBigExponentFloatRef(&x)), "0x1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self.0, f)
    }
}

impl LowerHex for ComparableBigExponentFloatRef<'_> {
    /// Formats a [`ComparableBigExponentFloatRef`] as [`LowerHex`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:x}", ComparableBigExponentFloatRef(&x)), "1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        LowerHex::fmt(self.0, f)
    }
}

impl UpperHex for ComparableBigExponentFloatRef<'_> {
    /// Formats a [`ComparableBigExponentFloatRef`] as [`UpperHex`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:X}", ComparableBigExponentFloatRef(&x)), "1.8");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        UpperHex::fmt(self.0, f)
    }
}

impl Binary for ComparableBigExponentFloatRef<'_> {
    /// Formats a [`ComparableBigExponentFloatRef`] as [`Binary`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:b}", ComparableBigExponentFloatRef(&x)), "1.1");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Binary::fmt(self.0, f)
    }
}

impl Octal for ComparableBigExponentFloatRef<'_> {
    /// Formats a [`ComparableBigExponentFloatRef`] as [`Octal`] does for the wrapped
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// let x = BigExponentFloat::from(1.5);
    /// assert_eq!(format!("{:o}", ComparableBigExponentFloatRef(&x)), "1.4");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Octal::fmt(self.0, f)
    }
}
