// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::Neg;
use core::str::FromStr;
use malachite_base::num::conversion::traits::FromStringBase;

impl FromStr for Integer {
    type Err = ();

    /// Converts an string to an [`Integer`].
    ///
    /// If the string does not represent a valid [`Integer`], an `Err` is returned. To be valid, the
    /// string must be nonempty and only contain the [`char`]s `'0'` through `'9'`, with an optional
    /// leading `'-'`. Leading zeros are allowed, as is the string `"-0"`. The string `"-"` is not.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_str("123456").unwrap(), 123456);
    /// assert_eq!(Integer::from_str("00123456").unwrap(), 123456);
    /// assert_eq!(Integer::from_str("0").unwrap(), 0);
    /// assert_eq!(Integer::from_str("-123456").unwrap(), -123456);
    /// assert_eq!(Integer::from_str("-00123456").unwrap(), -123456);
    /// assert_eq!(Integer::from_str("-0").unwrap(), 0);
    ///
    /// assert!(Integer::from_str("").is_err());
    /// assert!(Integer::from_str("a").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Integer, ()> {
        Integer::from_string_base(10, s).ok_or(())
    }
}

impl FromStringBase for Integer {
    /// Converts an string, in a specified base, to an [`Integer`].
    ///
    /// If the string does not represent a valid [`Integer`], an `Err` is returned. To be valid, the
    /// string must be nonempty and only contain the [`char`]s `'0'` through `'9'`, `'a'` through
    /// `'z'`, and `'A'` through `'Z'`, with an optional leading `'-'`; and only characters that
    /// represent digits smaller than the base are allowed. Leading zeros are allowed, as is the
    /// string `"-0"`. The string `"-"` is not.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_string_base(10, "123456").unwrap(), 123456);
    /// assert_eq!(Integer::from_string_base(10, "00123456").unwrap(), 123456);
    /// assert_eq!(Integer::from_string_base(16, "0").unwrap(), 0);
    /// assert_eq!(
    ///     Integer::from_string_base(16, "deadbeef").unwrap(),
    ///     3735928559i64
    /// );
    /// assert_eq!(
    ///     Integer::from_string_base(16, "deAdBeEf").unwrap(),
    ///     3735928559i64
    /// );
    /// assert_eq!(Integer::from_string_base(10, "-123456").unwrap(), -123456);
    /// assert_eq!(Integer::from_string_base(10, "-00123456").unwrap(), -123456);
    /// assert_eq!(Integer::from_string_base(16, "-0").unwrap(), 0);
    /// assert_eq!(
    ///     Integer::from_string_base(16, "-deadbeef").unwrap(),
    ///     -3735928559i64
    /// );
    /// assert_eq!(
    ///     Integer::from_string_base(16, "-deAdBeEf").unwrap(),
    ///     -3735928559i64
    /// );
    ///
    /// assert!(Integer::from_string_base(10, "").is_none());
    /// assert!(Integer::from_string_base(10, "a").is_none());
    /// assert!(Integer::from_string_base(2, "2").is_none());
    /// assert!(Integer::from_string_base(2, "-2").is_none());
    /// ```
    #[inline]
    fn from_string_base(base: u8, s: &str) -> Option<Integer> {
        if let Some(abs_string) = s.strip_prefix('-') {
            if abs_string.starts_with('+') {
                None
            } else {
                Natural::from_string_base(base, abs_string).map(Neg::neg)
            }
        } else {
            Natural::from_string_base(base, s).map(Integer::from)
        }
    }
}
