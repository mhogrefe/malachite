// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::{FromStringBase, WrappingFrom};

/// Produces a digit from a byte corresponding to a numeric or alphabetic (lower- or uppercase)
/// [`char`] that represents the digit.
///
/// Bytes corresponding to `char`s from '0' to '9' become digits 0 to 9. Bytes corresponding to
/// `char`s from 'a' to 'z' become digits 10 to 35. Bytes corresponding to `char`s from 'A' to 'Z'
/// also become digits 10 to 35. Passing a byte that does not correspond to any of these `char`s
/// yields `None`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::from_string::digit_from_display_byte;
///
/// assert_eq!(digit_from_display_byte(b'0'), Some(0));
/// assert_eq!(digit_from_display_byte(b'9'), Some(9));
/// assert_eq!(digit_from_display_byte(b'a'), Some(10));
/// assert_eq!(digit_from_display_byte(b'z'), Some(35));
/// assert_eq!(digit_from_display_byte(b'A'), Some(10));
/// assert_eq!(digit_from_display_byte(b'Z'), Some(35));
/// assert_eq!(digit_from_display_byte(b' '), None);
/// assert_eq!(digit_from_display_byte(b'!'), None);
/// ```
pub const fn digit_from_display_byte(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'z' => Some(b - b'a' + 10),
        b'A'..=b'Z' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Converts a byte corresponding to a numeric or alphabetic [`char`] to a digit in the large-base
/// alphabet used for bases from 37 through 62, in which the uppercase and lowercase letters are
/// distinct digits: `b'0'` through `b'9'` represent 0 through 9, `b'A'` through `b'Z'` represent 10
/// through 35, and `b'a'` through `b'z'` represent 36 through 61, as in GMP.
///
/// Bytes that don't correspond to any digit are converted to [`None`].
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::from_string::digit_from_display_byte_large;
///
/// assert_eq!(digit_from_display_byte_large(b'0'), Some(0));
/// assert_eq!(digit_from_display_byte_large(b'A'), Some(10));
/// assert_eq!(digit_from_display_byte_large(b'Z'), Some(35));
/// assert_eq!(digit_from_display_byte_large(b'a'), Some(36));
/// assert_eq!(digit_from_display_byte_large(b'z'), Some(61));
/// assert_eq!(digit_from_display_byte_large(b'!'), None);
/// ```
pub const fn digit_from_display_byte_large(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'A'..=b'Z' => Some(b - b'A' + 10),
        b'a'..=b'z' => Some(b - b'a' + 36),
        _ => None,
    }
}

macro_rules! impl_from_string_base_unsigned {
    ($t:ident) => {
        impl FromStringBase for $t {
            /// For bases up to 36, this is a wrapper over the `from_str_radix` functions in the
            /// standard library, for example [this one](u32::from_str_radix). For bases from 37
            /// through 62, which `from_str_radix` does not support, the digits are read from the
            /// case-sensitive large-base alphabet (see [`digit_from_display_byte_large`]), as in
            /// GMP.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            fn from_string_base(base: u8, s: &str) -> Option<Self> {
                assert!((2..=62).contains(&base), "base out of range");
                if base <= 36 {
                    return $t::from_str_radix(s, u32::from(base)).ok();
                }
                // like `from_str_radix`, allow a single leading `+`
                let s = s.strip_prefix('+').unwrap_or(s);
                if s.is_empty() {
                    return None;
                }
                let t_base = $t::wrapping_from(base);
                let mut x: $t = 0;
                for b in s.bytes() {
                    let digit = digit_from_display_byte_large(b)?;
                    if digit >= base {
                        return None;
                    }
                    x = x
                        .checked_mul(t_base)?
                        .checked_add($t::wrapping_from(digit))?;
                }
                Some(x)
            }
        }
    };
}
apply_to_unsigneds!(impl_from_string_base_unsigned);

macro_rules! impl_from_string_base_signed {
    ($t:ident) => {
        impl FromStringBase for $t {
            /// For bases up to 36, this is a wrapper over the `from_str_radix` functions in the
            /// standard library, for example [this one](i32::from_str_radix). For bases from 37
            /// through 62, which `from_str_radix` does not support, the digits are read from the
            /// case-sensitive large-base alphabet (see [`digit_from_display_byte_large`]), as in
            /// GMP.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            fn from_string_base(base: u8, s: &str) -> Option<Self> {
                assert!((2..=62).contains(&base), "base out of range");
                if base <= 36 {
                    return $t::from_str_radix(s, u32::from(base)).ok();
                }
                // like `from_str_radix`, allow a single leading sign; accumulate negatively when it
                // is a `-`, so that `MIN` parses
                let (neg, s) = if let Some(r) = s.strip_prefix('-') {
                    (true, r)
                } else {
                    (false, s.strip_prefix('+').unwrap_or(s))
                };
                if s.is_empty() {
                    return None;
                }
                let t_base = $t::wrapping_from(base);
                let mut x: $t = 0;
                for b in s.bytes() {
                    let digit = digit_from_display_byte_large(b)?;
                    if digit >= base {
                        return None;
                    }
                    let t_digit = $t::wrapping_from(digit);
                    x = x.checked_mul(t_base)?;
                    x = if neg {
                        x.checked_sub(t_digit)?
                    } else {
                        x.checked_add(t_digit)?
                    };
                }
                Some(x)
            }
        }
    };
}
apply_to_signeds!(impl_from_string_base_signed);
