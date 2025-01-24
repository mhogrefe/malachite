// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::conversion::string::from_string::digit_from_display_byte;
use crate::num::conversion::string::options::FromSciStringOptions;
use crate::num::conversion::traits::FromSciString;
use crate::rounding_modes::RoundingMode::*;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::str::FromStr;

#[doc(hidden)]
pub fn parse_exponent(s: &[u8]) -> Option<i64> {
    i64::from_str(core::str::from_utf8(s).ok()?).ok()
}

#[doc(hidden)]
pub fn validate_helper(s: &[u8], base: u8) -> Option<()> {
    for &c in s {
        if digit_from_display_byte(c)? >= base {
            return None;
        }
    }
    Some(())
}

#[doc(hidden)]
pub fn is_zero_helper(s: &[u8], base: u8) -> Option<bool> {
    let mut all_zeros = true;
    for &c in s {
        let d = digit_from_display_byte(c)?;
        if d >= base {
            return None;
        }
        if d != 0 {
            all_zeros = false;
        }
    }
    Some(all_zeros)
}

#[doc(hidden)]
pub fn cmp_half_helper(s: &[u8], base: u8) -> Option<Ordering> {
    if s.is_empty() {
        return Some(Less);
    }
    let h = base >> 1;
    let mut done = false;
    let mut result;
    if base.even() {
        // 1/2 is 0.h
        result = Equal;
        let mut first = true;
        for &c in s {
            let d = digit_from_display_byte(c)?;
            if d >= base {
                return None;
            }
            if done {
                continue;
            }
            if first {
                let half_c = d.cmp(&h);
                if half_c != Equal {
                    result = half_c;
                    done = true;
                }
                first = false;
            } else if d != 0 {
                result = Greater;
                done = true;
            }
        }
    } else {
        // 1/2 is 0.hhh...
        result = Less;
        for &c in s {
            let d = digit_from_display_byte(c)?;
            if done {
                continue;
            }
            let half_c = d.cmp(&h);
            if half_c != Equal {
                result = half_c;
                done = true;
            }
        }
    }
    Some(result)
}

fn parse_int<T: PrimitiveInt>(cs: &[u8], base: u8) -> Option<T> {
    // if T is unsigned, from_string_base won't handle -0
    let mut test_neg_zero = false;
    if T::MIN == T::ZERO {
        if let Some(&b'-') = cs.first() {
            test_neg_zero = true;
        }
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
        Some(T::ZERO)
    } else {
        T::from_string_base(base, core::str::from_utf8(cs).ok()?)
    }
}

fn up_1<T: PrimitiveInt>(x: T, neg: bool) -> Option<T> {
    if neg {
        x.checked_sub(T::ONE)
    } else {
        x.checked_add(T::ONE)
    }
}

#[doc(hidden)]
pub fn preprocess_sci_string(s: &str, options: FromSciStringOptions) -> Option<(Vec<u8>, i64)> {
    let mut s = s.as_bytes().to_vec();
    let mut exponent = 0;
    if options.base < 15 {
        for (i, &c) in s.iter().enumerate().rev() {
            if c == b'e' || c == b'E' {
                if i == 0 || i == s.len() - 1 {
                    return None;
                }
                exponent = parse_exponent(&s[i + 1..])?;
                s.truncate(i);
                break;
            }
        }
    } else {
        for (i, &c) in s.iter().enumerate().rev() {
            if c == b'+' || c == b'-' {
                if i == 0 {
                    break;
                }
                if i == 1 || i == s.len() - 1 {
                    return None;
                }
                let exp_indicator = s[i - 1];
                if exp_indicator != b'e' && exp_indicator != b'E' {
                    return None;
                }
                exponent = parse_exponent(&s[i..])?;
                s.truncate(i - 1);
                break;
            }
        }
    }
    let mut point_index = None;
    for (i, &c) in s.iter().enumerate() {
        if c == b'.' {
            point_index = Some(i);
            break;
        }
    }
    if let Some(point_index) = point_index {
        let len = s.len();
        if point_index != len - 1 {
            let next_char = s[point_index + 1];
            if next_char == b'+' || next_char == b'-' {
                return None;
            }
            exponent = exponent.checked_sub(i64::try_from(len - point_index - 1).ok()?)?;
            s.copy_within(point_index + 1..len, point_index);
        }
        s.pop();
    }
    Some((s, exponent))
}

fn from_sci_string_with_options_primitive_int<T: PrimitiveInt>(
    s: &str,
    options: FromSciStringOptions,
) -> Option<T> {
    let (s, exponent) = preprocess_sci_string(s, options)?;
    if exponent >= 0 {
        let x = parse_int::<T>(&s, options.base)?;
        x.checked_mul(T::wrapping_from(options.base).checked_pow(exponent.unsigned_abs())?)
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
            -options.rounding_mode
        } else {
            options.rounding_mode
        };
        let sig_len = if sign { len - 1 } else { len };
        if sig_len == 0 {
            return None;
        }
        if neg_exponent > sig_len {
            let s = if sign { &s[1..] } else { &s[..] };
            return match rm {
                Down | Floor | Nearest => {
                    validate_helper(s, options.base)?;
                    Some(T::ZERO)
                }
                Up | Ceiling => {
                    if is_zero_helper(s, options.base)? {
                        Some(T::ZERO)
                    } else {
                        up_1(T::ZERO, neg)
                    }
                }
                Exact => None,
            };
        }
        let (before_e, after_e) = s.split_at(len - neg_exponent);
        let x = match before_e {
            &[] | &[b'-'] | &[b'+'] => T::ZERO,
            before_e => parse_int(before_e, options.base)?,
        };
        if after_e.is_empty() {
            return Some(x);
        }
        match rm {
            Down | Floor => {
                validate_helper(after_e, options.base)?;
                Some(x)
            }
            Up | Ceiling => {
                if is_zero_helper(after_e, options.base)? {
                    Some(x)
                } else {
                    up_1(x, neg)
                }
            }
            Exact => {
                if is_zero_helper(after_e, options.base)? {
                    Some(x)
                } else {
                    None
                }
            }
            Nearest => match cmp_half_helper(after_e, options.base)? {
                Less => Some(x),
                Greater => up_1(x, neg),
                Equal => {
                    if x.even() {
                        Some(x)
                    } else {
                        up_1(x, neg)
                    }
                }
            },
        }
    }
}

macro_rules! impl_from_sci_string {
    ($t:ident) => {
        impl FromSciString for $t {
            /// Converts a [`String`], possibly in scientfic notation, to a primitive integer.
            ///
            /// Use [`FromSciStringOptions`] to specify the base (from 2 to 36, inclusive) and the
            /// rounding mode, in case rounding is necessary because the string represents a
            /// non-integer.
            ///
            /// If the base is greater than 10, the higher digits are represented by the letters
            /// `'a'` through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need
            /// to be consistent.
            ///
            /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the
            /// base is 15 or greater, an ambiguity arises where it may not be clear whether `'e'`
            /// is a digit or an exponent indicator. To resolve this ambiguity, always use a `'+'`
            /// or `'-'` sign after the exponent indicator when the base is 15 or greater.
            ///
            /// The exponent itself is always parsed using base 10.
            ///
            /// Decimal (or other-base) points are allowed. These are most useful in conjunction
            /// with exponents, but they may be used on their own. If the string represents a
            /// non-integer, the rounding mode specified in `options` is used to round to an
            /// integer.
            ///
            /// If the string is unparseable or parses to an out-of-range integer, `None` is
            /// returned. `None` is also returned if the rounding mode in options is `Exact`, but
            /// rounding is necessary.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
            ///
            /// # Examples
            /// See [here](super::from_sci_string).
            #[inline]
            fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<$t> {
                from_sci_string_with_options_primitive_int(s, options)
            }
        }
    };
}
apply_to_primitive_ints!(impl_from_sci_string);
