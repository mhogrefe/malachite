// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::*;
use core::ops::Mul;
use malachite_base::num::arithmetic::traits::{CheckedSub, Parity, Pow};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::string::from_sci_string::{
    cmp_half_helper, is_zero_helper, preprocess_sci_string, validate_helper,
};
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::*;

#[doc(hidden)]
pub trait FromSciStringHelper: Sized {
    fn parse_int(cs: &[u8], base: u8) -> Option<Self>;

    fn up_1(self, neg: bool) -> Option<Self>;
}

impl FromSciStringHelper for Natural {
    fn parse_int(mut cs: &[u8], base: u8) -> Option<Natural> {
        // if T is unsigned, from_string_base won't handle -0
        let mut test_neg_zero = false;
        if let Some(&b'-') = cs.first() {
            test_neg_zero = true;
        }
        if test_neg_zero {
            if cs.len() == 1 {
                return None;
            }
            for &c in &cs[1..] {
                if c != b'0' {
                    return None;
                }
            }
            Some(Natural::ZERO)
        } else {
            if let Some(b'+') = cs.first() {
                cs = &cs[1..];
                // If the string begins with a '+', the second character cannot be '+' or '-'
                match cs {
                    [] | [b'+' | b'-', ..] => return None,
                    _ => {}
                }
            }
            Natural::from_string_base(base, core::str::from_utf8(cs).ok()?)
        }
    }

    fn up_1(self, neg: bool) -> Option<Natural> {
        if neg {
            self.checked_sub(Natural::ONE)
        } else {
            Some(self + Natural::ONE)
        }
    }
}

pub(crate) fn from_sci_string_with_options_helper<
    T: From<u8> + FromSciStringHelper + Mul<T, Output = T> + Pow<u64, Output = T> + Zero,
>(
    s: &str,
    options: FromSciStringOptions,
) -> Option<T>
where
    for<'a> &'a T: Parity,
{
    let (s, exponent) = preprocess_sci_string(s, options)?;
    if exponent >= 0 {
        let x = T::parse_int(&s, options.get_base())?;
        Some(x * T::from(options.get_base()).pow(exponent.unsigned_abs()))
    } else {
        let neg_exponent = usize::try_from(exponent.unsigned_abs()).ok()?;
        let len = s.len();
        if len == 0 {
            return None;
        }
        let first = s[0];
        let neg = first == b'-';
        let sign = neg || first == b'+';
        let rm = if neg {
            -options.get_rounding_mode()
        } else {
            options.get_rounding_mode()
        };
        let sig_len = if sign { len - 1 } else { len };
        if sig_len == 0 {
            return None;
        }
        if neg_exponent > sig_len {
            let s = if sign { &s[1..] } else { &s[..] };
            return match rm {
                Down | Floor | Nearest => {
                    validate_helper(s, options.get_base())?;
                    Some(T::ZERO)
                }
                Up | Ceiling => {
                    if is_zero_helper(s, options.get_base())? {
                        Some(T::ZERO)
                    } else {
                        T::ZERO.up_1(neg)
                    }
                }
                Exact => None,
            };
        }
        let (before_e, after_e) = s.split_at(len - neg_exponent);
        let x = match before_e {
            &[] | &[b'-'] | &[b'+'] => T::ZERO,
            before_e => T::parse_int(before_e, options.get_base())?,
        };
        if after_e.is_empty() {
            return Some(x);
        }
        match rm {
            Down | Floor => {
                validate_helper(after_e, options.get_base())?;
                Some(x)
            }
            Up | Ceiling => {
                if is_zero_helper(after_e, options.get_base())? {
                    Some(x)
                } else {
                    x.up_1(neg)
                }
            }
            Exact => {
                if is_zero_helper(after_e, options.get_base())? {
                    Some(x)
                } else {
                    None
                }
            }
            Nearest => match cmp_half_helper(after_e, options.get_base())? {
                Less => Some(x),
                Greater => x.up_1(neg),
                Equal => {
                    if x.even() {
                        Some(x)
                    } else {
                        x.up_1(neg)
                    }
                }
            },
        }
    }
}

impl FromSciString for Natural {
    /// Converts a string, possibly in scientfic notation, to a [`Natural`].
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_sci_string("123").unwrap(), 123);
    /// assert_eq!(Natural::from_sci_string("123.5").unwrap(), 124);
    /// assert_eq!(Natural::from_sci_string("-123.5"), None);
    /// assert_eq!(Natural::from_sci_string("1.23e10").unwrap(), 12300000000u64);
    ///
    /// let mut options = FromSciStringOptions::default();
    /// assert_eq!(
    ///     Natural::from_sci_string_with_options("123.5", options).unwrap(),
    ///     124
    /// );
    ///
    /// options.set_rounding_mode(Floor);
    /// assert_eq!(
    ///     Natural::from_sci_string_with_options("123.5", options).unwrap(),
    ///     123
    /// );
    ///
    /// options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(
    ///     Natural::from_sci_string_with_options("ff", options).unwrap(),
    ///     255
    /// );
    ///
    /// options = FromSciStringOptions::default();
    /// options.set_base(36);
    /// assert_eq!(
    ///     Natural::from_sci_string_with_options("1e5", options).unwrap(),
    ///     1805
    /// );
    /// assert_eq!(
    ///     Natural::from_sci_string_with_options("1e+5", options).unwrap(),
    ///     60466176
    /// );
    /// assert_eq!(
    ///     Natural::from_sci_string_with_options("1e-5", options).unwrap(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Natural> {
        from_sci_string_with_options_helper(s, options)
    }
}
