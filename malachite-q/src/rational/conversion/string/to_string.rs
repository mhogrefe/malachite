// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::string::String;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex, Write};
use malachite_base::num::conversion::traits::ToStringBase;

impl ToStringBase for Rational {
    /// Converts a [`Rational`] to a [`String`] using a specified base.
    ///
    /// The numerator is written first, followed by a `'/'` and the denominator, unless the
    /// denominator is 1, in which case only the numerator is written. For bases up to 36, digits
    /// from 0 to 9 become [`char`]s from `'0'` to `'9'` and digits from 10 to 35 become the
    /// lowercase [`char`]s `'a'` to `'z'`. For bases from 37 through 62 the uppercase and lowercase
    /// letters are distinct digits, `'A'` through `'Z'` representing 10 through 35 and `'a'`
    /// through `'z'` representing 36 through 61, as in GMP.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 62.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(1000).to_string_base(10), "1000");
    /// assert_eq!(Rational::from_signeds(22, 7).to_string_base(10), "22/7");
    /// assert_eq!(Rational::from_signeds(-255, 7).to_string_base(16), "-ff/7");
    /// assert_eq!(Rational::from_signeds(1000, 7).to_string_base(36), "rs/7");
    /// // above base 36, the uppercase and lowercase letters are distinct digits
    /// assert_eq!(
    ///     Rational::from_signeds(-1000, 61).to_string_base(62),
    ///     "-G8/z"
    /// );
    /// ```
    fn to_string_base(&self, base: u8) -> String {
        let mut s = String::new();
        if !self.sign {
            s.push('-');
        }
        s.push_str(&self.numerator.to_string_base(base));
        if self.denominator != 1u32 {
            s.push('/');
            s.push_str(&self.denominator.to_string_base(base));
        }
        s
    }

    /// Converts a [`Rational`] to a [`String`] using a specified base, with the digits of the
    /// numerator and denominator being uppercase.
    ///
    /// The numerator is written first, followed by a `'/'` and the denominator, unless the
    /// denominator is 1, in which case only the numerator is written. For bases up to 36, digits
    /// from 0 to 9 become [`char`]s from `'0'` to `'9'` and digits from 10 to 35 become the
    /// uppercase [`char`]s `'A'` to `'Z'`. For bases from 37 through 62 there is only one alphabet,
    /// with the uppercase and lowercase letters as distinct digits, so the result is the same as
    /// [`to_string_base`](ToStringBase::to_string_base)'s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 62.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(1000).to_string_base_upper(10), "1000");
    /// assert_eq!(
    ///     Rational::from_signeds(-255, 7).to_string_base_upper(16),
    ///     "-FF/7"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(1000, 7).to_string_base_upper(36),
    ///     "RS/7"
    /// );
    /// // above base 36, the uppercase and lowercase letters are distinct digits
    /// assert_eq!(
    ///     Rational::from_signeds(-1000, 61).to_string_base_upper(62),
    ///     "-G8/z"
    /// );
    /// ```
    fn to_string_base_upper(&self, base: u8) -> String {
        let mut s = String::new();
        if !self.sign {
            s.push('-');
        }
        s.push_str(&self.numerator.to_string_base_upper(base));
        if self.denominator != 1u32 {
            s.push('/');
            s.push_str(&self.denominator.to_string_base_upper(base));
        }
        s
    }
}

impl Binary for Rational {
    /// Converts a [`Rational`] to a binary [`String`].
    ///
    /// The numerator and denominator are each written in binary, separated by a `'/'`, unless the
    /// denominator is 1, in which case only the numerator is written. Using the `#` format flag
    /// prepends `"0b"` to each of them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToBinaryString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.to_binary_string(), "0");
    /// assert_eq!(Rational::from(123).to_binary_string(), "1111011");
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).to_binary_string(),
    ///     "10110/111"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).to_binary_string(),
    ///     "-10110/111"
    /// );
    ///
    /// assert_eq!(format!("{:#b}", Rational::ZERO), "0b0");
    /// assert_eq!(
    ///     format!("{:#b}", Rational::from_signeds(22, 7)),
    ///     "0b10110/0b111"
    /// );
    /// assert_eq!(
    ///     format!("{:#b}", Rational::from_signeds(-22, 7)),
    ///     "-0b10110/0b111"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = Binary::fmt(&self.numerator, f);
        if self.denominator == 1u32 {
            result
        } else {
            f.write_char('/')?;
            Binary::fmt(&self.denominator, f)
        }
    }
}

impl Octal for Rational {
    /// Converts a [`Rational`] to an octal [`String`].
    ///
    /// The numerator and denominator are each written in octal, separated by a `'/'`, unless the
    /// denominator is 1, in which case only the numerator is written. Using the `#` format flag
    /// prepends `"0o"` to each of them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToOctalString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.to_octal_string(), "0");
    /// assert_eq!(Rational::from(123).to_octal_string(), "173");
    /// assert_eq!(Rational::from_signeds(22, 7).to_octal_string(), "26/7");
    /// assert_eq!(Rational::from_signeds(-22, 7).to_octal_string(), "-26/7");
    ///
    /// assert_eq!(format!("{:#o}", Rational::ZERO), "0o0");
    /// assert_eq!(format!("{:#o}", Rational::from_signeds(22, 7)), "0o26/0o7");
    /// assert_eq!(
    ///     format!("{:#o}", Rational::from_signeds(-22, 7)),
    ///     "-0o26/0o7"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = Octal::fmt(&self.numerator, f);
        if self.denominator == 1u32 {
            result
        } else {
            f.write_char('/')?;
            Octal::fmt(&self.denominator, f)
        }
    }
}

impl LowerHex for Rational {
    /// Converts a [`Rational`] to a hexadecimal [`String`] using lowercase characters.
    ///
    /// The numerator and denominator are each written in hexadecimal, separated by a `'/'`, unless
    /// the denominator is 1, in which case only the numerator is written. Using the `#` format flag
    /// prepends `"0x"` to each of them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToLowerHexString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.to_lower_hex_string(), "0");
    /// assert_eq!(Rational::from(123).to_lower_hex_string(), "7b");
    /// assert_eq!(Rational::from_signeds(255, 7).to_lower_hex_string(), "ff/7");
    /// assert_eq!(
    ///     Rational::from_signeds(-255, 7).to_lower_hex_string(),
    ///     "-ff/7"
    /// );
    ///
    /// assert_eq!(format!("{:#x}", Rational::ZERO), "0x0");
    /// assert_eq!(format!("{:#x}", Rational::from_signeds(255, 7)), "0xff/0x7");
    /// assert_eq!(
    ///     format!("{:#x}", Rational::from_signeds(-255, 7)),
    ///     "-0xff/0x7"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = LowerHex::fmt(&self.numerator, f);
        if self.denominator == 1u32 {
            result
        } else {
            f.write_char('/')?;
            LowerHex::fmt(&self.denominator, f)
        }
    }
}

impl UpperHex for Rational {
    /// Converts a [`Rational`] to a hexadecimal [`String`] using uppercase characters.
    ///
    /// The numerator and denominator are each written in hexadecimal, separated by a `'/'`, unless
    /// the denominator is 1, in which case only the numerator is written. Using the `#` format flag
    /// prepends `"0x"` to each of them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToUpperHexString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.to_upper_hex_string(), "0");
    /// assert_eq!(Rational::from(123).to_upper_hex_string(), "7B");
    /// assert_eq!(Rational::from_signeds(255, 7).to_upper_hex_string(), "FF/7");
    /// assert_eq!(
    ///     Rational::from_signeds(-255, 7).to_upper_hex_string(),
    ///     "-FF/7"
    /// );
    ///
    /// assert_eq!(format!("{:#X}", Rational::ZERO), "0x0");
    /// assert_eq!(format!("{:#X}", Rational::from_signeds(255, 7)), "0xFF/0x7");
    /// assert_eq!(
    ///     format!("{:#X}", Rational::from_signeds(-255, 7)),
    ///     "-0xFF/0x7"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = UpperHex::fmt(&self.numerator, f);
        if self.denominator == 1u32 {
            result
        } else {
            f.write_char('/')?;
            UpperHex::fmt(&self.denominator, f)
        }
    }
}

impl Display for Rational {
    /// Converts a [`Rational`] to a [`String`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.to_string(), "0");
    /// assert_eq!(Rational::from(123).to_string(), "123");
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_string(), "22/7");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = Display::fmt(&self.numerator, f);
        if self.denominator == 1u32 {
            result
        } else {
            f.write_char('/')?;
            Display::fmt(&self.denominator, f)
        }
    }
}

impl Debug for Rational {
    /// Converts a [`Rational`] to a [`String`].
    ///
    /// This is the same implementation as for [`Display`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.to_debug_string(), "0");
    /// assert_eq!(Rational::from(123).to_debug_string(), "123");
    /// assert_eq!(Rational::from_signeds(22, 7).to_debug_string(), "22/7");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
