// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::{Natural, WIDTH_MINUS_1, WIDTH_MINUS_2, WIDTH_MINUS_3};
use crate::platform::{LIMB_WIDTH_USIZE, Limb, MAX_DIGITS_PER_LIMB};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::string::from_string::{
    digit_from_display_byte, digit_from_display_byte_large,
};
use malachite_base::num::conversion::traits::{Digits, ExactFrom, FromStringBase, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::*;

const LOG_WIDTH_MINUS_2: u64 = Limb::LOG_WIDTH - 2;

impl FromStr for Natural {
    type Err = ();

    /// Converts an string to a [`Natural`].
    ///
    /// If the string does not represent a valid [`Natural`], an `Err` is returned. To be valid, the
    /// string must be nonempty and only contain the [`char`]s `'0'` through `'9'`. Leading zeros
    /// are allowed.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_str("123456").unwrap(), 123456);
    /// assert_eq!(Natural::from_str("00123456").unwrap(), 123456);
    /// assert_eq!(Natural::from_str("0").unwrap(), 0);
    ///
    /// assert!(Natural::from_str("").is_err());
    /// assert!(Natural::from_str("a").is_err());
    /// assert!(Natural::from_str("-5").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_string_base(10, s).ok_or(())
    }
}

fn from_binary_str(s: &str) -> Option<Natural> {
    let len = s.len();
    if len <= LIMB_WIDTH_USIZE {
        Limb::from_str_radix(s, 2).ok().map(Natural::from)
    } else {
        let mut xs = vec![0; len.shr_round(Limb::LOG_WIDTH, Ceiling).0];
        let mut remaining = u64::wrapping_from(len & const { Limb::WIDTH_MASK as usize });
        let mut i = xs.len();
        let mut x = xs.last_mut().unwrap();
        if remaining != 0 {
            i -= 1;
        }
        for b in s.bytes() {
            if remaining == 0 {
                i -= 1;
                x = &mut xs[i];
                remaining = Limb::WIDTH;
            }
            *x <<= 1;
            match b {
                b'1' => *x |= 1,
                b'0' => {}
                _ => return None,
            }
            remaining -= 1;
        }
        Some(Natural::from_owned_limbs_asc(xs))
    }
}

fn from_oct_str(s: &str) -> Option<Natural> {
    let len = s.len();
    if len <= const { (Limb::WIDTH / 3) as usize } {
        Limb::from_str_radix(s, 8).ok().map(Natural::from)
    } else {
        let bit_len = len.checked_mul(3).unwrap();
        let mut xs = vec![0; bit_len.shr_round(Limb::LOG_WIDTH, Ceiling).0];
        let mut remaining = u64::exact_from(bit_len) & Limb::WIDTH_MASK;
        let mut i = xs.len();
        let mut x = xs.last_mut().unwrap();
        if remaining != 0 {
            i -= 1;
        }
        for b in s.bytes() {
            let digit = digit_from_display_byte(b)?;
            if digit >= 8 {
                return None;
            }
            let digit = Limb::wrapping_from(digit);
            match remaining {
                0 => {
                    i -= 1;
                    x = &mut xs[i];
                    *x = digit;
                    remaining = WIDTH_MINUS_3;
                }
                1 => {
                    *x <<= 1;
                    *x |= digit >> 2;
                    i -= 1;
                    x = &mut xs[i];
                    *x = digit & 3;
                    remaining = WIDTH_MINUS_2;
                }
                2 => {
                    *x <<= 2;
                    *x |= digit >> 1;
                    i -= 1;
                    x = &mut xs[i];
                    *x = digit & 1;
                    remaining = WIDTH_MINUS_1;
                }
                _ => {
                    *x <<= 3;
                    *x |= digit;
                    remaining -= 3;
                }
            }
        }
        Some(Natural::from_owned_limbs_asc(xs))
    }
}

fn from_hex_str(s: &str) -> Option<Natural> {
    let len = s.len();
    if len <= const { (Limb::WIDTH >> 2) as usize } {
        Limb::from_str_radix(s, 16).ok().map(Natural::from)
    } else {
        let mut xs = vec![0; len.shr_round(LOG_WIDTH_MINUS_2, Ceiling).0];
        let mut remaining = u64::wrapping_from(len.mod_power_of_2(LOG_WIDTH_MINUS_2)) << 2;
        let mut i = xs.len();
        let mut x = xs.last_mut().unwrap();
        if remaining != 0 {
            i -= 1;
        }
        for b in s.bytes() {
            if remaining == 0 {
                i -= 1;
                x = &mut xs[i];
                remaining = Limb::WIDTH;
            }
            *x <<= 4;
            let digit = digit_from_display_byte(b)?;
            if digit >= 16 {
                return None;
            }
            *x |= Limb::wrapping_from(digit);
            remaining -= 4;
        }
        Some(Natural::from_owned_limbs_asc(xs))
    }
}

impl FromStringBase for Natural {
    /// Converts an string, in a specified base, to a [`Natural`].
    ///
    /// If the string does not represent a valid [`Natural`], an `Err` is returned. To be valid, the
    /// string must be nonempty and only contain the [`char`]s `'0'` through `'9'`, `'a'` through
    /// `'z'`, and `'A'` through `'Z'`, with an optional single leading `'+'`; and only characters
    /// that represent digits smaller than the base are allowed. Leading zeros are always allowed.
    /// For bases up to 36 the uppercase and lowercase letters are interchangeable; for bases from
    /// 37 through 62 they are distinct digits, `'A'` through `'Z'` representing 10 through 35 and
    /// `'a'` through `'z'` representing 36 through 61, as in GMP.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 62.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::FromStringBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_string_base(10, "123456").unwrap(), 123456);
    /// assert_eq!(Natural::from_string_base(10, "00123456").unwrap(), 123456);
    /// assert_eq!(Natural::from_string_base(16, "0").unwrap(), 0);
    /// assert_eq!(
    ///     Natural::from_string_base(16, "deadbeef").unwrap(),
    ///     3735928559u32
    /// );
    /// assert_eq!(
    ///     Natural::from_string_base(16, "deAdBeEf").unwrap(),
    ///     3735928559u32
    /// );
    ///
    /// assert!(Natural::from_string_base(10, "").is_none());
    /// assert!(Natural::from_string_base(10, "a").is_none());
    /// assert!(Natural::from_string_base(10, "-5").is_none());
    /// assert!(Natural::from_string_base(2, "2").is_none());
    ///
    /// // above base 36, the uppercase and lowercase letters are distinct digits
    /// assert_eq!(Natural::from_string_base(62, "G8").unwrap(), 1000);
    /// assert_eq!(Natural::from_string_base(62, "g8").unwrap(), 2612);
    /// ```
    #[inline]
    fn from_string_base(base: u8, mut s: &str) -> Option<Self> {
        assert!((2..=62).contains(&base), "base out of range");
        // A single leading `+` is allowed whatever the base. It is stripped here, before the
        // dispatch, so that every branch sees the bare digits; a second sign after it is an error
        // (the branches that delegate to `from_str_radix` would otherwise accept one of their own).
        if let Some(rest) = s.strip_prefix('+') {
            if rest.starts_with(['+', '-']) {
                return None;
            }
            s = rest;
        }
        if s.is_empty() {
            None
        } else {
            match base {
                2 => from_binary_str(s),
                8 => from_oct_str(s),
                16 => from_hex_str(s),
                10 => {
                    if s.len() < MAX_DIGITS_PER_LIMB {
                        Limb::from_str(s).ok().map(|x| Self(Small(x)))
                    } else {
                        if let Some(prefix_s) = s.strip_prefix('+') {
                            s = prefix_s;
                        }
                        Self::from_digits_desc(
                            &10,
                            s.bytes()
                                .map(|b| if b >= b'0' { b - b'0' } else { u8::MAX }),
                        )
                    }
                }
                _ => {
                    // above base 36, the uppercase and lowercase letters are distinct digits
                    let map = if base <= 36 {
                        digit_from_display_byte
                    } else {
                        digit_from_display_byte_large
                    };
                    for b in s.bytes() {
                        let digit = map(b)?;
                        if digit >= base {
                            return None;
                        }
                    }
                    Self::from_digits_desc(
                        &u8::wrapping_from(base),
                        s.bytes().map(|b| map(b).unwrap()),
                    )
                }
            }
        }
    }
}
