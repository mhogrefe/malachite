// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_nz::natural::Natural;

impl FromStringBase for Rational {
    /// Converts a string, in a specified base, to a [`Rational`].
    ///
    /// The string may contain a single `'/'` separating a numerator and a denominator. The
    /// numerator and denominator do not need to be in lowest terms, but the denominator must be
    /// nonzero. A negative sign is only allowed at the 0th position of the string.
    ///
    /// If the string does not represent a valid [`Rational`], `None` is returned.
    ///
    /// For bases greater than 36, the case of a letter matters: `'A'` through `'Z'` represent the
    /// digit values 10 through 35 and `'a'` through `'z'` represent 36 through 61, as in GMP. For
    /// bases no greater than 36, letters of either case are accepted.
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
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_string_base(10, "22/7").unwrap().to_string(),
    ///     "22/7"
    /// );
    /// assert_eq!(
    ///     Rational::from_string_base(10, "-3/21").unwrap().to_string(),
    ///     "-1/7"
    /// );
    /// assert_eq!(
    ///     Rational::from_string_base(16, "-ff/7").unwrap().to_string(),
    ///     "-255/7"
    /// );
    /// // above base 36, the uppercase and lowercase letters are distinct digits
    /// assert_eq!(
    ///     Rational::from_string_base(62, "G8/z").unwrap().to_string(),
    ///     "1000/61"
    /// );
    /// assert_eq!(
    ///     Rational::from_string_base(62, "g8/z").unwrap().to_string(),
    ///     "2612/61"
    /// );
    ///
    /// assert!(Rational::from_string_base(10, "1/0").is_none());
    /// assert!(Rational::from_string_base(10, "1/-2").is_none());
    /// assert!(Rational::from_string_base(37, "b/2").is_none());
    /// ```
    fn from_string_base(base: u8, s: &str) -> Option<Self> {
        let (abs_string, sign) = if let Some(abs_string) = s.strip_prefix('-') {
            if abs_string.starts_with('+') {
                return None;
            }
            (abs_string, false)
        } else {
            (s, true)
        };
        let numerator;
        let denominator;
        if let Some(slash_index) = abs_string.find('/') {
            numerator = Natural::from_string_base(base, &abs_string[..slash_index])?;
            denominator = Natural::from_string_base(base, &abs_string[slash_index + 1..])?;
            if denominator == 0u32 {
                return None;
            }
        } else {
            numerator = Natural::from_string_base(base, abs_string)?;
            denominator = Natural::ONE;
        }
        Some(Self::from_sign_and_naturals(sign, numerator, denominator))
    }
}

impl FromStr for Rational {
    type Err = ();

    /// Converts an string to a [`Rational`].
    ///
    /// If the string does not represent a valid [`Rational`], an `Err` is returned. The numerator
    /// and denominator do not need to be in lowest terms, but the denominator must be nonzero. An
    /// optional single leading `'-'` or `'+'` is allowed before the numerator. The denominator may
    /// also have a single leading `'+'`, but not a `'-'`.
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
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("123456").unwrap(), 123456);
    /// assert_eq!(Rational::from_str("00123456").unwrap(), 123456);
    /// assert_eq!(Rational::from_str("0").unwrap(), 0);
    /// assert_eq!(Rational::from_str("-123456").unwrap(), -123456);
    /// assert_eq!(Rational::from_str("-00123456").unwrap(), -123456);
    /// assert_eq!(Rational::from_str("-0").unwrap(), 0);
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_string(), "22/7");
    /// assert_eq!(Rational::from_str("01/02").unwrap().to_string(), "1/2");
    /// assert_eq!(Rational::from_str("3/21").unwrap().to_string(), "1/7");
    /// assert_eq!(Rational::from_str("-22/7").unwrap().to_string(), "-22/7");
    /// assert_eq!(Rational::from_str("-01/02").unwrap().to_string(), "-1/2");
    /// assert_eq!(Rational::from_str("-3/21").unwrap().to_string(), "-1/7");
    /// assert_eq!(Rational::from_str("+22/7").unwrap().to_string(), "22/7");
    /// assert_eq!(Rational::from_str("22/+7").unwrap().to_string(), "22/7");
    ///
    /// assert!(Rational::from_str("").is_err());
    /// assert!(Rational::from_str("a").is_err());
    /// assert!(Rational::from_str("1/0").is_err());
    /// assert!(Rational::from_str("1/-2").is_err());
    /// assert!(Rational::from_str("/1").is_err());
    /// assert!(Rational::from_str("1/").is_err());
    /// assert!(Rational::from_str("--1").is_err());
    /// assert!(Rational::from_str("1/-2").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        let (abs_string, sign) = if let Some(abs_string) = s.strip_prefix('-') {
            if abs_string.starts_with('+') {
                return Err(());
            }
            (abs_string, false)
        } else {
            (s, true)
        };
        let numerator;
        let denominator;
        if let Some(slash_index) = abs_string.find('/') {
            numerator = Natural::from_str(&abs_string[..slash_index])?;
            denominator = Natural::from_str(&abs_string[slash_index + 1..])?;
            if denominator == 0u32 {
                return Err(());
            }
        } else {
            numerator = Natural::from_str(abs_string)?;
            denominator = Natural::ONE;
        }
        Ok(Self::from_sign_and_naturals(sign, numerator, denominator))
    }
}
