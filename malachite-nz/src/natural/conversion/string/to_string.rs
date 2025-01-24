// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_to_digits_small_base_no_alg_specified,
};
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::string::String;
#[cfg(feature = "test_build")]
use core::fmt::Write;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex};
#[cfg(feature = "test_build")]
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{DivRound, Parity, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper, BaseFmtWrapper as BaseBaseFmtWrapper,
};
#[cfg(feature = "test_build")]
use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, ToStringBase, WrappingFrom};
#[cfg(feature = "test_build")]
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::*;

/// A `struct` that allows for formatting a [`Natural`] or [`Integer`](crate::integer::Integer) and
/// rendering its digits in a specified base.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BaseFmtWrapper<T> {
    pub(crate) x: T,
    pub(crate) base: u8,
}

impl<T> BaseFmtWrapper<T> {
    /// Creates a new `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::from(1000000000u32);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "gjdgxs");
    /// assert_eq!(format!("{:#}", x), "GJDGXS");
    ///
    /// let n = Integer::from(-1000000000);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "-gjdgxs");
    /// assert_eq!(format!("{:#}", x), "-GJDGXS");
    /// ```
    pub fn new(x: T, base: u8) -> Self {
        assert!((2..=36).contains(&base), "base out of range");
        BaseFmtWrapper { x, base }
    }

    /// Recovers the value from a `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     BaseFmtWrapper::new(Natural::from(1000000000u32), 36).unwrap(),
    ///     1000000000
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn unwrap(self) -> T {
        self.x
    }
}

impl Display for BaseFmtWrapper<&Natural> {
    /// Writes a wrapped [`Natural`] to a string using a specified base.
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
    /// use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::from(1000000000u32);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "gjdgxs");
    /// assert_eq!(format!("{:#}", x), "GJDGXS");
    /// assert_eq!(format!("{:010}", x), "0000gjdgxs");
    /// assert_eq!(format!("{:#010}", x), "0000GJDGXS");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        assert!((2..=36).contains(&self.base), "base out of range");
        if let Natural(Small(x)) = self.x {
            Display::fmt(&BaseBaseFmtWrapper::new(*x, self.base), f)
        } else {
            let mut digits = self.x.to_digits_desc(&u8::wrapping_from(self.base));
            if f.alternate() {
                for digit in &mut digits {
                    *digit = digit_to_display_byte_upper(*digit).unwrap();
                }
            } else {
                for digit in &mut digits {
                    *digit = digit_to_display_byte_lower(*digit).unwrap();
                }
            }
            f.pad_integral(true, "", core::str::from_utf8(&digits).unwrap())
        }
    }
}

impl Debug for BaseFmtWrapper<&Natural> {
    /// Writes a wrapped [`Natural`] to a string using a specified base.
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
    /// use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::from(1000000000u32);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{:?}", x), "gjdgxs");
    /// assert_eq!(format!("{:#?}", x), "GJDGXS");
    /// assert_eq!(format!("{:010?}", x), "0000gjdgxs");
    /// assert_eq!(format!("{:#010?}", x), "0000GJDGXS");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl ToStringBase for Natural {
    /// Converts a [`Natural`] to a [`String`] using a specified base.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(1000u32).to_string_base(2), "1111101000");
    /// assert_eq!(Natural::from(1000u32).to_string_base(10), "1000");
    /// assert_eq!(Natural::from(1000u32).to_string_base(36), "rs");
    /// ```
    fn to_string_base(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if let Natural(Small(x)) = self {
            x.to_string_base(base)
        } else {
            let mut digits = self.to_digits_desc(&base);
            for digit in &mut digits {
                *digit = digit_to_display_byte_lower(*digit).unwrap();
            }
            String::from_utf8(digits).unwrap()
        }
    }

    /// Converts a [`Natural`] to a [`String`] using a specified base.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(1000u32).to_string_base_upper(2), "1111101000");
    /// assert_eq!(Natural::from(1000u32).to_string_base_upper(10), "1000");
    /// assert_eq!(Natural::from(1000u32).to_string_base_upper(36), "RS");
    /// ```
    fn to_string_base_upper(&self, base: u8) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if let Natural(Small(x)) = self {
            x.to_string_base_upper(base)
        } else {
            let mut digits = self.to_digits_desc(&base);
            for digit in &mut digits {
                *digit = digit_to_display_byte_upper(*digit).unwrap();
            }
            String::from_utf8(digits).unwrap()
        }
    }
}

impl Display for Natural {
    /// Converts a [`Natural`] to a [`String`].
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.to_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_string(), "123");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000").unwrap().to_string(),
    ///     "1000000000000"
    /// );
    /// assert_eq!(format!("{:05}", Natural::from(123u32)), "00123");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => Display::fmt(x, f),
            Natural(Large(xs)) => {
                let mut digits = vec![0; usize::exact_from(limbs_digit_count(xs, 10))];
                let mut xs = xs.clone();
                let len = limbs_to_digits_small_base_no_alg_specified(&mut digits, 10, &mut xs);
                digits.truncate(len);
                for digit in &mut digits {
                    *digit = digit_to_display_byte_lower(*digit).unwrap();
                }
                f.pad_integral(true, "", core::str::from_utf8(&digits).unwrap())
            }
        }
    }
}

impl Debug for Natural {
    /// Converts a [`Natural`] to a [`String`].
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.to_debug_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_debug_string(), "123");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_debug_string(),
    ///     "1000000000000"
    /// );
    /// assert_eq!(format!("{:05?}", Natural::from(123u32)), "00123");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

#[cfg(feature = "test_build")]
pub struct NaturalAlt(pub Natural);

#[cfg(feature = "test_build")]
pub struct NaturalAlt2(pub Natural);

#[cfg(feature = "test_build")]
impl Binary for NaturalAlt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Natural(Small(x)) = self.0 {
            Binary::fmt(&x, f)
        } else {
            if f.alternate() {
                f.write_str("0b")?;
            }
            if let Some(width) = f.width() {
                let mut len = usize::exact_from(self.0.significant_bits());
                if f.alternate() {
                    len += 2;
                }
                for _ in 0..width.saturating_sub(len) {
                    f.write_char('0')?;
                }
            }
            for bit in self.0.bits().rev() {
                f.write_char(if bit { '1' } else { '0' })?;
            }
            Ok(())
        }
    }
}

#[cfg(feature = "test_build")]
impl Binary for NaturalAlt2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.0 {
            Natural(Small(x)) => Binary::fmt(x, f),
            Natural(Large(ref xs)) => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let width = if let Some(width) = f.width() {
                    width.saturating_sub(xs_init.len() << Limb::LOG_WIDTH)
                } else {
                    0
                };
                let mut result = if f.alternate() {
                    write!(f, "{xs_last:#0width$b}")
                } else {
                    write!(f, "{xs_last:0width$b}")
                };
                for x in xs_init.iter().rev() {
                    #[cfg(feature = "32_bit_limbs")]
                    {
                        result = write!(f, "{x:032b}");
                    }
                    #[cfg(not(feature = "32_bit_limbs"))]
                    {
                        result = write!(f, "{x:064b}");
                    }
                }
                result
            }
        }
    }
}

impl Binary for Natural {
    /// Converts a [`Natural`] to a binary [`String`].
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.to_binary_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_binary_string(), "1111011");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_binary_string(),
    ///     "1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:011b}", Natural::from(123u32)), "00001111011");
    ///
    /// assert_eq!(format!("{:#b}", Natural::ZERO), "0b0");
    /// assert_eq!(format!("{:#b}", Natural::from(123u32)), "0b1111011");
    /// assert_eq!(
    ///     format!("{:#b}", Natural::from_str("1000000000000").unwrap()),
    ///     "0b1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:#011b}", Natural::from(123u32)), "0b001111011");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => Binary::fmt(x, f),
            Natural(Large(xs)) => {
                let mut bits = vec![0; usize::exact_from(limbs_significant_bits(xs))];
                let mut limbs = xs.iter();
                let mut limb = *limbs.next().unwrap();
                let mut remaining_bits = Limb::WIDTH;
                for bit in bits.iter_mut().rev() {
                    if remaining_bits == 0 {
                        remaining_bits = Limb::WIDTH;
                        limb = *limbs.next().unwrap();
                    }
                    *bit = if limb.even() { b'0' } else { b'1' };
                    limb >>= 1;
                    remaining_bits -= 1;
                }
                f.pad_integral(true, "0b", core::str::from_utf8(&bits).unwrap())
            }
        }
    }
}

#[cfg(feature = "test_build")]
impl Octal for NaturalAlt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Natural(Small(x)) = self.0 {
            Octal::fmt(&x, f)
        } else {
            if f.alternate() {
                f.write_str("0o")?;
            }
            if let Some(width) = f.width() {
                let mut len = usize::exact_from(self.0.significant_bits().div_round(3, Ceiling).0);
                if f.alternate() {
                    len += 2;
                }
                for _ in 0..width.saturating_sub(len) {
                    f.write_char('0')?;
                }
            }
            for digit in PowerOf2DigitIterable::<u8>::power_of_2_digits(&self.0, 3).rev() {
                f.write_char(char::from(digit_to_display_byte_lower(digit).unwrap()))?;
            }
            Ok(())
        }
    }
}

#[cfg(feature = "test_build")]
#[cfg(feature = "32_bit_limbs")]
fn oz_fmt(f: &mut Formatter, x: Limb) -> Result {
    write!(f, "{x:08o}")
}
#[cfg(feature = "test_build")]
#[cfg(not(feature = "32_bit_limbs"))]
fn oz_fmt(f: &mut Formatter, x: Limb) -> Result {
    write!(f, "{x:016o}")
}

#[cfg(feature = "test_build")]
impl Octal for NaturalAlt2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.0 {
            Natural(Small(x)) => Octal::fmt(x, f),
            Natural(Large(xs)) => {
                if f.alternate() {
                    f.write_str("0o")?;
                }
                if let Some(width) = f.width() {
                    let mut len =
                        usize::exact_from(limbs_significant_bits(xs).div_round(3, Ceiling).0);
                    if f.alternate() {
                        len += 2;
                    }
                    for _ in 0..width.saturating_sub(len) {
                        f.write_char('0')?;
                    }
                }
                let mut triple_r = xs.len() % 3;
                if triple_r == 0 {
                    triple_r = 3;
                }
                let mut result;
                let last_i = xs.len() - 1;
                const W_1_2: u64 = Limb::WIDTH >> 1;
                const W_1_4: u64 = Limb::WIDTH >> 2;
                const W_3_4: u64 = W_1_4 * 3;
                const MASK: Limb = (1 << W_3_4) - 1;
                match triple_r {
                    1 => {
                        let x_2 = xs[last_i];
                        let y = x_2 >> W_3_4;
                        if y == 0 {
                            result = write!(f, "{:o}", x_2 & MASK);
                        } else {
                            write!(f, "{y:o}").unwrap();
                            result = oz_fmt(f, x_2 & MASK);
                        }
                    }
                    2 => {
                        let x_1 = xs[last_i];
                        let x_2 = xs[last_i - 1];
                        let y = x_1 >> W_1_2;
                        if y == 0 {
                            write!(f, "{:o}", ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                        } else {
                            write!(f, "{y:o}").unwrap();
                            oz_fmt(f, ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                        }
                        result = oz_fmt(f, x_2 & MASK);
                    }
                    _ => {
                        let x_0 = xs[last_i];
                        let x_1 = xs[last_i - 1];
                        let x_2 = xs[last_i - 2];
                        let y = x_0 >> W_1_4;
                        if y == 0 {
                            write!(f, "{:o}", ((x_0 << W_1_2) & MASK) | (x_1 >> W_1_2)).unwrap();
                        } else {
                            write!(f, "{y:o}").unwrap();
                            oz_fmt(f, ((x_0 << W_1_2) & MASK) | (x_1 >> W_1_2)).unwrap();
                        }
                        oz_fmt(f, ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                        result = oz_fmt(f, x_2 & MASK);
                    }
                }
                for mut chunk in &xs.iter().rev().skip(triple_r).chunks(3) {
                    let x_0 = chunk.next().unwrap();
                    let x_1 = chunk.next().unwrap();
                    let x_2 = chunk.next().unwrap();
                    oz_fmt(f, x_0 >> W_1_4).unwrap();
                    oz_fmt(f, ((x_0 << W_1_2) & MASK) | (x_1 >> W_1_2)).unwrap();
                    oz_fmt(f, ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                    result = oz_fmt(f, x_2 & MASK);
                }
                result
            }
        }
    }
}

impl Octal for Natural {
    /// Converts a [`Natural`] to an octal [`String`].
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.to_octal_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_octal_string(), "173");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_octal_string(),
    ///     "16432451210000"
    /// );
    /// assert_eq!(format!("{:07o}", Natural::from(123u32)), "0000173");
    ///
    /// assert_eq!(format!("{:#o}", Natural::ZERO), "0o0");
    /// assert_eq!(format!("{:#o}", Natural::from(123u32)), "0o173");
    /// assert_eq!(
    ///     format!("{:#o}", Natural::from_str("1000000000000").unwrap()),
    ///     "0o16432451210000"
    /// );
    /// assert_eq!(format!("{:#07o}", Natural::from(123u32)), "0o00173");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => Octal::fmt(x, f),
            Natural(Large(xs)) => {
                let mut digits =
                    vec![0; usize::exact_from(limbs_significant_bits(xs).div_round(3, Ceiling).0)];
                let mut limbs = xs.iter();
                let mut remaining_bits = Limb::WIDTH;
                let mut limb = *limbs.next().unwrap();
                for digit in digits.iter_mut().rev() {
                    if remaining_bits >= 3 {
                        *digit = digit_to_display_byte_lower(u8::wrapping_from(limb & 7)).unwrap();
                        remaining_bits -= 3;
                        limb >>= 3;
                    } else {
                        match remaining_bits {
                            0 => {
                                limb = *limbs.next().unwrap();
                                *digit = digit_to_display_byte_lower(u8::wrapping_from(limb & 7))
                                    .unwrap();
                                remaining_bits = Limb::WIDTH - 3;
                                limb >>= 3;
                            }
                            1 => {
                                let previous_limb = limb;
                                limb = *limbs.next().unwrap_or(&0);
                                *digit = digit_to_display_byte_lower(u8::wrapping_from(
                                    ((limb & 3) << 1) | previous_limb,
                                ))
                                .unwrap();
                                remaining_bits = Limb::WIDTH - 2;
                                limb >>= 2;
                            }
                            _ => {
                                let previous_limb = limb;
                                limb = *limbs.next().unwrap_or(&0);
                                *digit = digit_to_display_byte_lower(u8::wrapping_from(
                                    ((limb & 1) << 2) | previous_limb,
                                ))
                                .unwrap();
                                remaining_bits = Limb::WIDTH - 1;
                                limb >>= 1;
                            }
                        }
                    }
                }
                f.pad_integral(true, "0o", core::str::from_utf8(&digits).unwrap())
            }
        }
    }
}

#[cfg(feature = "test_build")]
impl LowerHex for NaturalAlt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Natural(Small(x)) = self.0 {
            LowerHex::fmt(&x, f)
        } else {
            if f.alternate() {
                f.write_str("0x")?;
            }
            if let Some(width) = f.width() {
                let mut len = usize::exact_from(self.0.significant_bits().shr_round(2, Ceiling).0);
                if f.alternate() {
                    len += 2;
                }
                for _ in 0..width.saturating_sub(len) {
                    f.write_char('0')?;
                }
            }
            for digit in PowerOf2DigitIterable::<u8>::power_of_2_digits(&self.0, 4).rev() {
                f.write_char(char::from(digit_to_display_byte_lower(digit).unwrap()))?;
            }
            Ok(())
        }
    }
}

#[cfg(feature = "test_build")]
impl LowerHex for NaturalAlt2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.0 {
            Natural(Small(x)) => LowerHex::fmt(x, f),
            Natural(Large(ref xs)) => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let width = if let Some(width) = f.width() {
                    width.saturating_sub(xs_init.len() << Limb::LOG_WIDTH >> 2)
                } else {
                    0
                };
                let mut result = if f.alternate() {
                    write!(f, "{xs_last:#0width$x}")
                } else {
                    write!(f, "{xs_last:0width$x}")
                };
                for x in xs_init.iter().rev() {
                    #[cfg(feature = "32_bit_limbs")]
                    {
                        result = write!(f, "{x:08x}");
                    }
                    #[cfg(not(feature = "32_bit_limbs"))]
                    {
                        result = write!(f, "{x:016x}");
                    }
                }
                result
            }
        }
    }
}

impl LowerHex for Natural {
    /// Converts a [`Natural`] to a hexadecimal [`String`] using lowercase characters.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.to_lower_hex_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_lower_hex_string(), "7b");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_lower_hex_string(),
    ///     "e8d4a51000"
    /// );
    /// assert_eq!(format!("{:07x}", Natural::from(123u32)), "000007b");
    ///
    /// assert_eq!(format!("{:#x}", Natural::ZERO), "0x0");
    /// assert_eq!(format!("{:#x}", Natural::from(123u32)), "0x7b");
    /// assert_eq!(
    ///     format!("{:#x}", Natural::from_str("1000000000000").unwrap()),
    ///     "0xe8d4a51000"
    /// );
    /// assert_eq!(format!("{:#07x}", Natural::from(123u32)), "0x0007b");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => LowerHex::fmt(x, f),
            Natural(Large(xs)) => {
                const DIGITS_PER_LIMB: u64 = Limb::WIDTH >> 2;
                let mut digits =
                    vec![0; usize::exact_from(limbs_significant_bits(xs).shr_round(2, Ceiling).0)];
                let mut limbs = xs.iter();
                let mut limb = *limbs.next().unwrap();
                let mut remaining_digits = DIGITS_PER_LIMB;
                for digit in digits.iter_mut().rev() {
                    if remaining_digits == 0 {
                        remaining_digits = DIGITS_PER_LIMB;
                        limb = *limbs.next().unwrap();
                    }
                    *digit = digit_to_display_byte_lower(u8::wrapping_from(limb & 15)).unwrap();
                    limb >>= 4;
                    remaining_digits -= 1;
                }
                f.pad_integral(true, "0x", core::str::from_utf8(&digits).unwrap())
            }
        }
    }
}

impl UpperHex for Natural {
    /// Converts a [`Natural`] to a hexadecimal [`String`] using uppercase characters.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.to_upper_hex_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_upper_hex_string(), "7B");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000")
    ///         .unwrap()
    ///         .to_upper_hex_string(),
    ///     "E8D4A51000"
    /// );
    /// assert_eq!(format!("{:07X}", Natural::from(123u32)), "000007B");
    ///
    /// assert_eq!(format!("{:#X}", Natural::ZERO), "0x0");
    /// assert_eq!(format!("{:#X}", Natural::from(123u32)), "0x7B");
    /// assert_eq!(
    ///     format!("{:#X}", Natural::from_str("1000000000000").unwrap()),
    ///     "0xE8D4A51000"
    /// );
    /// assert_eq!(format!("{:#07X}", Natural::from(123u32)), "0x0007B");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => UpperHex::fmt(x, f),
            Natural(Large(xs)) => {
                const DIGITS_PER_LIMB: u64 = Limb::WIDTH >> 2;
                let mut digits =
                    vec![0; usize::exact_from(limbs_significant_bits(xs).shr_round(2, Ceiling).0)];
                let mut limbs = xs.iter();
                let mut limb = *limbs.next().unwrap();
                let mut remaining_digits = DIGITS_PER_LIMB;
                for digit in digits.iter_mut().rev() {
                    if remaining_digits == 0 {
                        remaining_digits = DIGITS_PER_LIMB;
                        limb = *limbs.next().unwrap();
                    }
                    *digit = digit_to_display_byte_upper(u8::wrapping_from(limb & 15)).unwrap();
                    limb >>= 4;
                    remaining_digits -= 1;
                }
                f.pad_integral(true, "0x", core::str::from_utf8(&digits).unwrap())
            }
        }
    }
}
