use crate::integer::Integer;
use malachite_base::num::conversion::traits::FromStringBase;
use crate::natural::Natural;
use std::ops::Neg;
use std::str::FromStr;

impl FromStr for Integer {
    type Err = ();

    /// Converts an string to an [`Integer`].
    ///
    /// If the string does not represent a valid [`Integer`], an `Err` is returned. To be valid,
    /// the string must be nonempty and only contain the [`char`]s `'0'` through `'9'`, with an
    /// optional leading `'-'`. Leading zeros are allowed, as is the string `"-0"`. The string
    /// `"-"` is not.
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
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
    /// If the string does not represent a valid [`Integer`], an `Err` is returned. To be valid,
    /// the string must be nonempty and only contain the [`char`]s `'0'` through `'9'`, `'a'`
    /// through `'z'`, and `'A'` through `'Z'`, with an optional leading `'-'`; and only characters
    /// that represent digits smaller than the base are allowed. Leading zeros are allowed, as is
    /// the string `"-0"`. The string `"-"` is not.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::conversion::traits::{Digits, FromStringBase};
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
            Natural::from_string_base(base, abs_string).map(Neg::neg)
        } else {
            Natural::from_string_base(base, s).map(Integer::from)
        }
    }
}
