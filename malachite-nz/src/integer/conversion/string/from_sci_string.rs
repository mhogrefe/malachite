// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::conversion::string::from_sci_string::{
    from_sci_string_with_options_helper, FromSciStringHelper,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, FromStringBase};

impl FromSciStringHelper for Integer {
    fn parse_int(mut cs: &[u8], base: u8) -> Option<Integer> {
        if let Some(b'+') = cs.first() {
            cs = &cs[1..];
            // If the string begins with a '+', the second character cannot be '+' or '-'
            match cs {
                [] | [b'+' | b'-', ..] => return None,
                _ => {}
            }
        }
        Integer::from_string_base(base, core::str::from_utf8(cs).ok()?)
    }

    fn up_1(self, neg: bool) -> Option<Integer> {
        Some(if neg {
            self - Integer::ONE
        } else {
            self + Integer::ONE
        })
    }
}

impl FromSciString for Integer {
    /// Converts a string, possibly in scientfic notation, to an [`Integer`].
    ///
    /// Use [`FromSciStringOptions`] to specify the base (from 2 to 36, inclusive) and the rounding
    /// mode, in case rounding is necessary because the string represents a non-integer.
    ///
    /// If the base is greater than 10, the higher digits are represented by the letters `'a'`
    /// through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need to be
    /// consistent.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the base is
    /// 15 or greater, an ambiguity arises where it may not be clear whether `'e'` is a digit or an
    /// exponent indicator. To resolve this ambiguity, always use a `'+'` or `'-'` sign after the
    /// exponent indicator when the base is 15 or greater.
    ///
    /// The exponent itself is always parsed using base 10.
    ///
    /// Decimal (or other-base) points are allowed. These are most useful in conjunction with
    /// exponents, but they may be used on their own. If the string represents a non-integer, the
    /// rounding mode specified in `options` is used to round to an integer.
    ///
    /// If the string is unparseable, `None` is returned. `None` is also returned if the rounding
    /// mode in options is `Exact`, but rounding is necessary.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m^n n \log m (\log n + \log\log m))$
    ///
    /// $M(n, m) = O(m^n n \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `s.len()`, and $m$ is `options.base`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_base::num::conversion::traits::FromSciString;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_sci_string("123").unwrap(), 123);
    /// assert_eq!(Integer::from_sci_string("123.5").unwrap(), 124);
    /// assert_eq!(Integer::from_sci_string("-123.5").unwrap(), -124);
    /// assert_eq!(Integer::from_sci_string("1.23e10").unwrap(), 12300000000i64);
    ///
    /// let mut options = FromSciStringOptions::default();
    /// assert_eq!(
    ///     Integer::from_sci_string_with_options("123.5", options).unwrap(),
    ///     124
    /// );
    ///
    /// options.set_rounding_mode(Floor);
    /// assert_eq!(
    ///     Integer::from_sci_string_with_options("123.5", options).unwrap(),
    ///     123
    /// );
    ///
    /// options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(
    ///     Integer::from_sci_string_with_options("ff", options).unwrap(),
    ///     255
    /// );
    /// ```
    #[inline]
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Integer> {
        from_sci_string_with_options_helper(s, options)
    }
}
