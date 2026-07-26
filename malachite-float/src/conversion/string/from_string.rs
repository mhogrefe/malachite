// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
//
// The inverse of `to_string.rs`: reads back what `Float` and `ComparableFloat` write.
//
// A `ComparableFloat` writes a precision after a `#`, since the digits alone do not determine one
// (`0xff.0#8` has three hex digits, or twelve bits, but a precision of 8). That suffix is what
// makes the round trip exact, in every base. Without it the precision is inferred from the digits,
// as `from_sci_string.rs` describes.

use crate::conversion::string::from_sci_string::float_from_string_base;
use crate::{ComparableFloat, Float};
use core::str::FromStr;
use malachite_base::num::conversion::traits::FromStringBase;

impl FromStringBase for Float {
    /// Converts a string, in a specified base, to a [`Float`].
    ///
    /// This reads back what [`Float`] and [`ComparableFloat`] write, so it is the inverse of
    /// [`to_string_base`](malachite_base::num::conversion::traits::ToStringBase::to_string_base)
    /// and of the formatting impls: an optional `-`, an optional base prefix (`0b`, `0o`, or `0x`,
    /// which the `{:#b}`-style formats write and the plain ones omit), the digits, and optionally
    /// `#` and a precision.
    ///
    /// The `#` suffix is what makes the round trip exact, because the digits alone do not determine
    /// a precision: `0xff.0#8` has three hexadecimal digits, or twelve bits, but a precision of 8.
    /// With the suffix the precision is exactly what it says; without it the precision is inferred
    /// from the digits, which is coarse for short strings — see [`Float`]'s
    /// [`FromSciString`](malachite_base::num::conversion::traits::FromSciString) impl for why.
    /// Since the special values and zeros carry no precision, a `#` suffix on one of them is
    /// rejected.
    ///
    /// Apart from the sign, the prefix, and the suffix, the grammar is the one described by
    /// [`from_sci_string_with_options_prec`](Float::from_sci_string_with_options_prec). The value
    /// is rounded to nearest.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36. This is checked before the string, so an
    /// unparseable string does not mask an impossible base.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::FromStringBase;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// let s = |base, s| Float::from_string_base(base, s).map(|x| x.to_string());
    ///
    /// // With a `#` suffix the precision is exact, so this round-trips whatever the base.
    /// assert_eq!(s(10, "1.5#10"), Some("1.5000".to_string()));
    /// assert_eq!(s(16, "0xff.0#8"), Some("255.0".to_string()));
    /// assert_eq!(s(8, "0o7.7#6"), Some("7.88".to_string()));
    /// // The base prefix is optional, since `{:x}` omits it and `{:#x}` writes it.
    /// assert_eq!(s(2, "0b1.01#3"), Some("1.2".to_string()));
    /// assert_eq!(s(2, "1.01#3"), Some("1.2".to_string()));
    ///
    /// // Without the suffix the precision is inferred from the digits.
    /// assert_eq!(s(10, "1.0"), Some("1.0".to_string()));
    ///
    /// // The specials and the zeros are spelled the same in every base and carry no precision, so
    /// // a suffix on one is not something `ComparableFloat` wrote.
    /// assert_eq!(s(16, "NaN"), Some("NaN".to_string()));
    /// assert_eq!(s(16, "-0x0.0"), Some("-0.0".to_string()));
    /// assert_eq!(s(10, "NaN#5"), None);
    ///
    /// // A precision of zero is not a `Float` precision.
    /// assert_eq!(s(10, "1.0#0"), None);
    /// assert_eq!(s(10, "abc"), None);
    ///
    /// // The round trip that the suffix exists for.
    /// let x = Float::from(1.5);
    /// let written = format!("{:#x}", ComparableFloat(x.clone()));
    /// assert_eq!(written, "0x1.8#2");
    /// assert_eq!(Float::from_string_base(16, &written).unwrap(), x);
    /// ```
    #[inline]
    fn from_string_base(base: u8, s: &str) -> Option<Self> {
        float_from_string_base(base, s)
    }
}

impl FromStr for Float {
    type Err = ();

    /// Converts a string to a [`Float`].
    ///
    /// This is [`from_string_base`](Float::from_string_base) in base 10, so it reads what
    /// [`Display`](std::fmt::Display) writes, and also accepts the `#` precision suffix that
    /// [`ComparableFloat`]'s [`Display`](std::fmt::Display) adds. Without that suffix the precision
    /// is inferred from the digits, so `Float::from_str(&x.to_string())` recovers the value of `x`
    /// but not necessarily its precision; go through [`ComparableFloat`] for that.
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
    /// use malachite_float::Float;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Float::from_str("1.5#10").unwrap().to_string(), "1.5000");
    /// assert_eq!(Float::from_str("NaN").unwrap().to_string(), "NaN");
    /// assert_eq!(
    ///     Float::from_str("-Infinity").unwrap().to_string(),
    ///     "-Infinity"
    /// );
    /// // Zero keeps its sign.
    /// assert_eq!(Float::from_str("-0.0").unwrap().to_string(), "-0.0");
    ///
    /// assert!(Float::from_str("abc").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        float_from_string_base(10, s).ok_or(())
    }
}

impl FromStr for ComparableFloat {
    type Err = ();

    /// Converts a string to a [`ComparableFloat`].
    ///
    /// This is [`Float`]'s [`FromStr`] with the result wrapped, so it accepts exactly the same
    /// strings. It is the inverse of [`ComparableFloat`]'s [`Display`](std::fmt::Display), which
    /// writes the `#` precision suffix: with that suffix the round trip preserves the precision as
    /// well as the value, which is what makes two [`ComparableFloat`]s that were written and read
    /// back compare equal.
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
    /// use malachite_float::{ComparableFloat, Float};
    /// use std::str::FromStr;
    ///
    /// // The round trip preserves the precision, which comparing as `ComparableFloat`s requires.
    /// let x = ComparableFloat(Float::from(1.5));
    /// assert_eq!(x.to_string(), "1.5#2");
    /// assert_eq!(ComparableFloat::from_str(&x.to_string()).unwrap(), x);
    ///
    /// // Without the suffix the precision is inferred, which here happens to agree.
    /// assert_eq!(ComparableFloat::from_str("1.5").unwrap(), x);
    ///
    /// assert!(ComparableFloat::from_str("abc").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        float_from_string_base(10, s).map(ComparableFloat).ok_or(())
    }
}
