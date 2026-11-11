// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::{Float, InnerFloat};
use alloc::format;
use alloc::string::String;
use core::fmt::{Debug, Formatter, LowerHex, Result, UpperHex};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::strings::{ToLowerHexString, ToUpperHexString};
use malachite_nz::integer::Integer;

// `Float`'s hexadecimal string for `x`, as `{:x}`, `{:#x}`, `{:X}`, or `{:#X}` would write it.
fn float_hex(x: &Float, alternate: bool, uppercase: bool) -> String {
    match (alternate, uppercase) {
        (false, false) => x.to_lower_hex_string(),
        (true, false) => format!("{x:#x}"),
        (false, true) => x.to_upper_hex_string(),
        (true, true) => format!("{x:#X}"),
    }
}

// Writes `x` in base 16, exactly, as `Float`'s `LowerHex` and `UpperHex` do. A finite nonzero `x`
// with an exponent in `Float`'s range is written as the `Float` of the same value. Otherwise, the
// exponent is shifted into range by a multiple of 4, which changes neither the hexadecimal digits
// nor where they start, and the shift is added back to the exponent after the `E`.
fn fmt_hex(x: &BigExponentFloat, f: &mut Formatter, uppercase: bool) -> Result {
    let alternate = f.alternate();
    let (sign, exponent, significand) = match x {
        BigExponentFloat(NaN) => return f.write_str("NaN"),
        BigExponentFloat(Infinity { sign: true }) => return f.write_str("Infinity"),
        BigExponentFloat(Infinity { sign: false }) => return f.write_str("-Infinity"),
        BigExponentFloat(Zero) => return f.write_str(if alternate { "0x0.0" } else { "0.0" }),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => (*sign, exponent, significand),
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
        return f.write_str(&float_hex(&float(e), alternate, uppercase));
    }
    // A target exponent near the end of `Float`'s range, where `Float` writes an `E`, and congruent
    // to `exponent` mod 4.
    let base = if *exponent > 0u32 {
        Float::MAX_EXPONENT - 3
    } else {
        Float::MIN_EXPONENT + 3
    };
    let target = base + i32::wrapping_from(&(exponent - Integer::from(base)).mod_power_of_2(2u64));
    let shift = (exponent - Integer::from(target)) >> 2u32;
    let s = float_hex(&float(target), alternate, uppercase);
    let e_index = s.rfind('E').unwrap();
    let printed = Integer::from_str(s[e_index + 1..].trim_start_matches('+')).unwrap();
    let new_exponent = printed + shift;
    let plus = if new_exponent >= 0u32 { "+" } else { "" };
    write!(f, "{}E{plus}{new_exponent}", &s[..e_index])
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
        fmt_hex(self, f, false)
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
        fmt_hex(self, f, true)
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
