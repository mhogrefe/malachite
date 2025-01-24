// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::conversion::string::to_string::BaseFmtWrapper;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex, Write};
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use malachite_base::num::conversion::traits::{Digits, ToStringBase};
use malachite_base::vecs::vec_pad_left;

impl Display for BaseFmtWrapper<&Integer> {
    /// Writes a wrapped [`Integer`] to a string using a specified base.
    ///
    /// If the base is greater than 10, lowercase alphabetic letters are used by default. Using the
    /// `#` flag switches to uppercase letters. Padding with zeros works as usual.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
    ///
    /// let n = Integer::from(-1000000000);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "-gjdgxs");
    /// assert_eq!(format!("{:#}", x), "-GJDGXS");
    /// assert_eq!(format!("{:010}", x), "-000gjdgxs");
    /// assert_eq!(format!("{:#010}", x), "-000GJDGXS");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.x.sign {
            f.write_char('-')?;
            if let Some(width) = f.width() {
                return if f.alternate() {
                    write!(
                        f,
                        "{:#0width$}",
                        &BaseFmtWrapper::new(self.x.unsigned_abs_ref(), self.base),
                        width = width.saturating_sub(1)
                    )
                } else {
                    write!(
                        f,
                        "{:0width$}",
                        &BaseFmtWrapper::new(self.x.unsigned_abs_ref(), self.base),
                        width = width.saturating_sub(1)
                    )
                };
            }
        }
        Display::fmt(
            &BaseFmtWrapper::new(self.x.unsigned_abs_ref(), self.base),
            f,
        )
    }
}

impl Debug for BaseFmtWrapper<&Integer> {
    /// Writes a wrapped [`Integer`] to a string using a specified base.
    ///
    /// If the base is greater than 10, lowercase alphabetic letters are used by default. Using the
    /// `#` flag switches to uppercase letters. Padding with zeros works as usual.
    ///
    /// This is the same as the [`Display::fmt`] implementation.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
    ///
    /// let n = Integer::from(-1000000000);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{:?}", x), "-gjdgxs");
    /// assert_eq!(format!("{:#?}", x), "-GJDGXS");
    /// assert_eq!(format!("{:010?}", x), "-000gjdgxs");
    /// assert_eq!(format!("{:#010?}", x), "-000GJDGXS");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl ToStringBase for Integer {
    /// Converts an [`Integer`] to a [`String`] using a specified base.
    ///
    /// Digits from 0 to 9 become [`char`]s from `'0'` to `'9'`. Digits from 10 to 35 become the
    /// lowercase [`char`]s `'a'` to `'z'`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(1000).to_string_base(2), "1111101000");
    /// assert_eq!(Integer::from(1000).to_string_base(10), "1000");
    /// assert_eq!(Integer::from(1000).to_string_base(36), "rs");
    ///
    /// assert_eq!(Integer::from(-1000).to_string_base(2), "-1111101000");
    /// assert_eq!(Integer::from(-1000).to_string_base(10), "-1000");
    /// assert_eq!(Integer::from(-1000).to_string_base(36), "-rs");
    /// ```
    fn to_string_base(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if *self == 0 {
            "0".to_string()
        } else {
            let mut digits = self.unsigned_abs_ref().to_digits_desc(&base);
            for digit in &mut digits {
                *digit = digit_to_display_byte_lower(*digit).unwrap();
            }
            if *self < 0 {
                vec_pad_left(&mut digits, 1, b'-');
            }
            String::from_utf8(digits).unwrap()
        }
    }

    /// Converts an [`Integer`] to a [`String`] using a specified base.
    ///
    /// Digits from 0 to 9 become [`char`]s from `'0'` to `'9'`. Digits from 10 to 35 become the
    /// uppercase [`char`]s `'A'` to `'Z'`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(1000).to_string_base_upper(2), "1111101000");
    /// assert_eq!(Integer::from(1000).to_string_base_upper(10), "1000");
    /// assert_eq!(Integer::from(1000).to_string_base_upper(36), "RS");
    ///
    /// assert_eq!(Integer::from(-1000).to_string_base_upper(2), "-1111101000");
    /// assert_eq!(Integer::from(-1000).to_string_base_upper(10), "-1000");
    /// assert_eq!(Integer::from(-1000).to_string_base_upper(36), "-RS");
    /// ```
    fn to_string_base_upper(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if *self == 0 {
            "0".to_string()
        } else {
            let mut digits = self.unsigned_abs_ref().to_digits_desc(&base);
            for digit in &mut digits {
                *digit = digit_to_display_byte_upper(*digit).unwrap();
            }
            if *self < 0 {
                vec_pad_left(&mut digits, 1, b'-');
            }
            String::from_utf8(digits).unwrap()
        }
    }
}

impl Display for Integer {
    /// Converts an [`Integer`] to a [`String`].
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.to_string(), "0");
    ///
    /// assert_eq!(Integer::from(123).to_string(), "123");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000").unwrap().to_string(),
    ///     "1000000000000"
    /// );
    /// assert_eq!(format!("{:05}", Integer::from(123)), "00123");
    ///
    /// assert_eq!(Integer::from(-123).to_string(), "-123");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000").unwrap().to_string(),
    ///     "-1000000000000"
    /// );
    /// assert_eq!(format!("{:05}", Integer::from(-123)), "-0123");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self < 0 {
            f.write_char('-')?;
            if let Some(width) = f.width() {
                return write!(
                    f,
                    "{:0width$}",
                    self.unsigned_abs_ref(),
                    width = width.saturating_sub(1)
                );
            }
        }
        Display::fmt(self.unsigned_abs_ref(), f)
    }
}

impl Debug for Integer {
    /// Converts an [`Integer`] to a [`String`].
    ///
    /// This is the same as the [`Display::fmt`] implementation.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.to_debug_string(), "0");
    ///
    /// assert_eq!(Integer::from(123).to_debug_string(), "123");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_debug_string(),
    ///     "1000000000000"
    /// );
    /// assert_eq!(format!("{:05?}", Integer::from(123)), "00123");
    ///
    /// assert_eq!(Integer::from(-123).to_debug_string(), "-123");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000")
    ///         .unwrap()
    ///         .to_debug_string(),
    ///     "-1000000000000"
    /// );
    /// assert_eq!(format!("{:05?}", Integer::from(-123)), "-0123");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl Binary for Integer {
    /// Converts an [`Integer`] to a binary [`String`].
    ///
    /// Using the `#` format flag prepends `"0b"` to the string.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToBinaryString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.to_binary_string(), "0");
    /// assert_eq!(Integer::from(123).to_binary_string(), "1111011");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_binary_string(),
    ///     "1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:011b}", Integer::from(123)), "00001111011");
    /// assert_eq!(Integer::from(-123).to_binary_string(), "-1111011");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000")
    ///         .unwrap()
    ///         .to_binary_string(),
    ///     "-1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:011b}", Integer::from(-123)), "-0001111011");
    ///
    /// assert_eq!(format!("{:#b}", Integer::ZERO), "0b0");
    /// assert_eq!(format!("{:#b}", Integer::from(123)), "0b1111011");
    /// assert_eq!(
    ///     format!("{:#b}", Integer::from_str("1000000000000").unwrap()),
    ///     "0b1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:#011b}", Integer::from(123)), "0b001111011");
    /// assert_eq!(format!("{:#b}", Integer::from(-123)), "-0b1111011");
    /// assert_eq!(
    ///     format!("{:#b}", Integer::from_str("-1000000000000").unwrap()),
    ///     "-0b1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:#011b}", Integer::from(-123)), "-0b01111011");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self < 0 {
            f.write_char('-')?;
            if let Some(width) = f.width() {
                return if f.alternate() {
                    write!(
                        f,
                        "{:#0width$b}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                } else {
                    write!(
                        f,
                        "{:0width$b}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                };
            }
        }
        Binary::fmt(self.unsigned_abs_ref(), f)
    }
}

impl Octal for Integer {
    /// Converts an [`Integer`] to an octal [`String`].
    ///
    /// Using the `#` format flag prepends `"0o"` to the string.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToOctalString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.to_octal_string(), "0");
    /// assert_eq!(Integer::from(123).to_octal_string(), "173");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_octal_string(),
    ///     "16432451210000"
    /// );
    /// assert_eq!(format!("{:07o}", Integer::from(123)), "0000173");
    /// assert_eq!(Integer::from(-123).to_octal_string(), "-173");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000")
    ///         .unwrap()
    ///         .to_octal_string(),
    ///     "-16432451210000"
    /// );
    /// assert_eq!(format!("{:07o}", Integer::from(-123)), "-000173");
    ///
    /// assert_eq!(format!("{:#o}", Integer::ZERO), "0o0");
    /// assert_eq!(format!("{:#o}", Integer::from(123)), "0o173");
    /// assert_eq!(
    ///     format!("{:#o}", Integer::from_str("1000000000000").unwrap()),
    ///     "0o16432451210000"
    /// );
    /// assert_eq!(format!("{:#07o}", Integer::from(123)), "0o00173");
    /// assert_eq!(format!("{:#o}", Integer::from(-123)), "-0o173");
    /// assert_eq!(
    ///     format!("{:#o}", Integer::from_str("-1000000000000").unwrap()),
    ///     "-0o16432451210000"
    /// );
    /// assert_eq!(format!("{:#07o}", Integer::from(-123)), "-0o0173");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self < 0 {
            f.write_char('-')?;
            if let Some(width) = f.width() {
                return if f.alternate() {
                    write!(
                        f,
                        "{:#0width$o}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                } else {
                    write!(
                        f,
                        "{:0width$o}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                };
            }
        }
        Octal::fmt(self.unsigned_abs_ref(), f)
    }
}

impl LowerHex for Integer {
    /// Converts an [`Integer`] to a hexadecimal [`String`] using lowercase characters.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToLowerHexString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.to_lower_hex_string(), "0");
    /// assert_eq!(Integer::from(123).to_lower_hex_string(), "7b");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_lower_hex_string(),
    ///     "e8d4a51000"
    /// );
    /// assert_eq!(format!("{:07x}", Integer::from(123)), "000007b");
    /// assert_eq!(Integer::from(-123).to_lower_hex_string(), "-7b");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000")
    ///         .unwrap()
    ///         .to_lower_hex_string(),
    ///     "-e8d4a51000"
    /// );
    /// assert_eq!(format!("{:07x}", Integer::from(-123)), "-00007b");
    ///
    /// assert_eq!(format!("{:#x}", Integer::ZERO), "0x0");
    /// assert_eq!(format!("{:#x}", Integer::from(123)), "0x7b");
    /// assert_eq!(
    ///     format!("{:#x}", Integer::from_str("1000000000000").unwrap()),
    ///     "0xe8d4a51000"
    /// );
    /// assert_eq!(format!("{:#07x}", Integer::from(123)), "0x0007b");
    /// assert_eq!(format!("{:#x}", Integer::from(-123)), "-0x7b");
    /// assert_eq!(
    ///     format!("{:#x}", Integer::from_str("-1000000000000").unwrap()),
    ///     "-0xe8d4a51000"
    /// );
    /// assert_eq!(format!("{:#07x}", Integer::from(-123)), "-0x007b");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self < 0 {
            f.write_char('-')?;
            if let Some(width) = f.width() {
                return if f.alternate() {
                    write!(
                        f,
                        "{:#0width$x}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                } else {
                    write!(
                        f,
                        "{:0width$x}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                };
            }
        }
        LowerHex::fmt(self.unsigned_abs_ref(), f)
    }
}

impl UpperHex for Integer {
    /// Converts an [`Integer`] to a hexadecimal [`String`] using uppercase characters.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToUpperHexString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.to_upper_hex_string(), "0");
    /// assert_eq!(Integer::from(123).to_upper_hex_string(), "7B");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_upper_hex_string(),
    ///     "E8D4A51000"
    /// );
    /// assert_eq!(format!("{:07X}", Integer::from(123)), "000007B");
    /// assert_eq!(Integer::from(-123).to_upper_hex_string(), "-7B");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000")
    ///         .unwrap()
    ///         .to_upper_hex_string(),
    ///     "-E8D4A51000"
    /// );
    /// assert_eq!(format!("{:07X}", Integer::from(-123)), "-00007B");
    ///
    /// assert_eq!(format!("{:#X}", Integer::ZERO), "0x0");
    /// assert_eq!(format!("{:#X}", Integer::from(123)), "0x7B");
    /// assert_eq!(
    ///     format!("{:#X}", Integer::from_str("1000000000000").unwrap()),
    ///     "0xE8D4A51000"
    /// );
    /// assert_eq!(format!("{:#07X}", Integer::from(123)), "0x0007B");
    /// assert_eq!(format!("{:#X}", Integer::from(-123)), "-0x7B");
    /// assert_eq!(
    ///     format!("{:#X}", Integer::from_str("-1000000000000").unwrap()),
    ///     "-0xE8D4A51000"
    /// );
    /// assert_eq!(format!("{:#07X}", Integer::from(-123)), "-0x007B");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self < 0 {
            f.write_char('-')?;
            if let Some(width) = f.width() {
                return if f.alternate() {
                    write!(
                        f,
                        "{:#0width$X}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                } else {
                    write!(
                        f,
                        "{:0width$X}",
                        self.unsigned_abs_ref(),
                        width = width.saturating_sub(1)
                    )
                };
            }
        }
        UpperHex::fmt(self.unsigned_abs_ref(), f)
    }
}
