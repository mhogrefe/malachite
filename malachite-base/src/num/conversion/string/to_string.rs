// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::basic::traits::Zero;
use crate::num::conversion::traits::{Digits, ToStringBase, WrappingFrom};
use crate::vecs::vec_pad_left;
use alloc::string::{String, ToString};
use core::fmt::{Debug, Display, Formatter, Result, Write};

/// A `struct` that allows for formatting a numeric type and rendering its digits in a specified
/// base.
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
    /// Panics if `base` is less than 2 or greater than 62.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::to_string::BaseFmtWrapper;
    ///
    /// let x = BaseFmtWrapper::new(1000000000u32, 36);
    /// assert_eq!(format!("{}", x), "gjdgxs");
    /// assert_eq!(format!("{:#}", x), "GJDGXS");
    /// ```
    pub fn new(x: T, base: u8) -> Self {
        assert!((2..=62).contains(&base), "base out of range");
        Self { x, base }
    }

    /// Recovers the value from a `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::to_string::BaseFmtWrapper;
    ///
    /// assert_eq!(BaseFmtWrapper::new(1000000000u32, 36).unwrap(), 1000000000);
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn unwrap(self) -> T {
        self.x
    }
}

/// Converts a digit to a byte corresponding to a numeric or lowercase alphabetic [`char`] that
/// represents the digit.
///
/// Digits from 0 to 9 become bytes corresponding to [`char`]s from '0' to '9'. Digits from 10 to 35
/// become bytes representing the lowercase [`char`]s 'a' to 'z'. Passing a digit greater than 35
/// gives a `None`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::to_string::digit_to_display_byte_lower;
///
/// assert_eq!(digit_to_display_byte_lower(0), Some(b'0'));
/// assert_eq!(digit_to_display_byte_lower(9), Some(b'9'));
/// assert_eq!(digit_to_display_byte_lower(10), Some(b'a'));
/// assert_eq!(digit_to_display_byte_lower(35), Some(b'z'));
/// assert_eq!(digit_to_display_byte_lower(100), None);
/// ```
pub const fn digit_to_display_byte_lower(b: u8) -> Option<u8> {
    match b {
        0..=9 => Some(b + b'0'),
        10..=35 => Some(b + b'a' - 10),
        _ => None,
    }
}

/// Converts a digit to a byte corresponding to a numeric or uppercase alphabetic [`char`] that
/// represents the digit.
///
/// Digits from 0 to 9 become bytes corresponding to [`char`]s from '0' to '9'. Digits from 10 to 35
/// become bytes representing the lowercase [`char`]s 'A' to 'Z'. Passing a digit greater than 35
/// gives a `None`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::to_string::digit_to_display_byte_upper;
///
/// assert_eq!(digit_to_display_byte_upper(0), Some(b'0'));
/// assert_eq!(digit_to_display_byte_upper(9), Some(b'9'));
/// assert_eq!(digit_to_display_byte_upper(10), Some(b'A'));
/// assert_eq!(digit_to_display_byte_upper(35), Some(b'Z'));
/// assert_eq!(digit_to_display_byte_upper(100), None);
/// ```
pub const fn digit_to_display_byte_upper(b: u8) -> Option<u8> {
    match b {
        0..=9 => Some(b + b'0'),
        10..=35 => Some(b + b'A' - 10),
        _ => None,
    }
}

/// Converts a digit to a byte corresponding to a numeric or alphabetic [`char`] in the large-base
/// alphabet used for bases from 37 through 62, in which the uppercase and lowercase letters are
/// distinct digits: `b'0'` through `b'9'` represent 0 through 9, `b'A'` through `b'Z'` represent 10
/// through 35, and `b'a'` through `b'z'` represent 36 through 61, as in GMP.
///
/// Digits from 62 on are not associated with any byte, so [`None`] is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::to_string::digit_to_display_byte_large;
///
/// assert_eq!(digit_to_display_byte_large(0), Some(b'0'));
/// assert_eq!(digit_to_display_byte_large(10), Some(b'A'));
/// assert_eq!(digit_to_display_byte_large(35), Some(b'Z'));
/// assert_eq!(digit_to_display_byte_large(36), Some(b'a'));
/// assert_eq!(digit_to_display_byte_large(61), Some(b'z'));
/// assert_eq!(digit_to_display_byte_large(62), None);
/// ```
pub const fn digit_to_display_byte_large(b: u8) -> Option<u8> {
    match b {
        0..=9 => Some(b + b'0'),
        10..=35 => Some(b + b'A' - 10),
        36..=61 => Some(b + b'a' - 36),
        _ => None,
    }
}

fn fmt_unsigned<T: Copy + Digits<u8> + Eq + Zero>(
    w: &BaseFmtWrapper<T>,
    f: &mut Formatter,
) -> Result {
    let mut digits = w.x.to_digits_desc(&u8::wrapping_from(w.base));
    // Above base 36 the uppercase and lowercase letters are distinct digits, so there is only one
    // alphabet and the `#` flag has no effect.
    if w.base > 36 {
        for digit in &mut digits {
            *digit = digit_to_display_byte_large(*digit).unwrap();
        }
    } else if f.alternate() {
        for digit in &mut digits {
            *digit = digit_to_display_byte_upper(*digit).unwrap();
        }
    } else {
        for digit in &mut digits {
            *digit = digit_to_display_byte_lower(*digit).unwrap();
        }
    }
    if w.x == T::ZERO {
        digits.push(b'0');
    }
    f.pad_integral(true, "", core::str::from_utf8(&digits).unwrap())
}

fn to_string_base_unsigned<T: Copy + Digits<u8> + Eq + Zero>(x: &T, base: u8) -> String {
    assert!((2..=62).contains(&base), "base out of range");
    if *x == T::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.to_digits_desc(&base);
        let map = if base > 36 {
            // above base 36 there is only one alphabet, with both cases as distinct digits
            digit_to_display_byte_large
        } else {
            digit_to_display_byte_lower
        };
        for digit in &mut digits {
            *digit = map(*digit).unwrap();
        }
        String::from_utf8(digits).unwrap()
    }
}

fn to_string_base_upper_unsigned<T: Copy + Digits<u8> + Eq + Zero>(x: &T, base: u8) -> String {
    assert!((2..=62).contains(&base), "base out of range");
    if *x == T::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.to_digits_desc(&base);
        let map = if base > 36 {
            // above base 36 there is only one alphabet, with both cases as distinct digits
            digit_to_display_byte_large
        } else {
            digit_to_display_byte_upper
        };
        for digit in &mut digits {
            *digit = map(*digit).unwrap();
        }
        String::from_utf8(digits).unwrap()
    }
}

macro_rules! impl_to_string_base_unsigned {
    ($t:ident) => {
        impl Display for BaseFmtWrapper<$t> {
            /// Writes a wrapped unsigned number to a string using a specified base.
            ///
            /// If the base is greater than 10, lowercase alphabetic letters are used by default.
            /// Using the `#` flag switches to uppercase letters. Padding with zeros works as usual.
            /// If the base is greater than 36, the uppercase and lowercase letters are distinct
            /// digits (see
            /// [`digit_to_display_byte_large`](super::to_string::digit_to_display_byte_large)), and
            /// the `#` flag has no effect.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string).
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result {
                fmt_unsigned(self, f)
            }
        }

        impl Debug for BaseFmtWrapper<$t> {
            /// Writes a wrapped unsigned number to a string using a specified base.
            ///
            /// If the base is greater than 10, lowercase alphabetic letters are used by default.
            /// Using the `#` flag switches to uppercase letters. Padding with zeros works as usual.
            /// If the base is greater than 36, the uppercase and lowercase letters are distinct
            /// digits (see
            /// [`digit_to_display_byte_large`](super::to_string::digit_to_display_byte_large)), and
            /// the `#` flag has no effect.
            ///
            /// This is the same as the [`Display::fmt`] implementation.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string).
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result {
                Display::fmt(self, f)
            }
        }

        impl ToStringBase for $t {
            /// Converts an unsigned number to a string using a specified base.
            ///
            /// For bases up to 36, digits from 0 to 9 become [`char`]s from '0' to '9' and digits
            /// from 10 to 35 become the lowercase [`char`]s 'a' to 'z'. For bases from 37 through
            /// 62 the uppercase and lowercase letters are distinct digits, 'A' through 'Z'
            /// representing 10 through 35 and 'a' through 'z' representing 36 through 61, as in
            /// GMP.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string#to_string_base).
            #[inline]
            fn to_string_base(&self, base: u8) -> String {
                to_string_base_unsigned(self, base)
            }

            /// Converts an unsigned number to a string using a specified base.
            ///
            /// For bases up to 36, digits from 0 to 9 become [`char`]s from '0' to '9' and digits
            /// from 10 to 35 become the uppercase [`char`]s 'A' to 'Z'. For bases from 37 through
            /// 62 the uppercase and lowercase letters are distinct digits and there is only one
            /// alphabet, so the result is the same as
            /// [`to_string_base`](super::traits::ToStringBase::to_string_base)'s.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string#to_string_base_upper).
            #[inline]
            fn to_string_base_upper(&self, base: u8) -> String {
                to_string_base_upper_unsigned(self, base)
            }
        }
    };
}
apply_to_unsigneds!(impl_to_string_base_unsigned);

fn fmt_signed<T: Copy + Ord + UnsignedAbs + Zero>(
    w: &BaseFmtWrapper<T>,
    f: &mut Formatter,
) -> Result
where
    <T as UnsignedAbs>::Output: Copy + Digits<u8> + Eq + Zero,
    BaseFmtWrapper<<T as UnsignedAbs>::Output>: Display,
{
    if w.x < T::ZERO {
        if f.width().is_some() || f.sign_plus() {
            // Let `pad_integral` handle the interaction of the sign with the `+` flag, the
            // sign-aware zero flag, and fill and alignment.
            let s = if f.alternate() {
                to_string_base_upper_unsigned(&w.x.unsigned_abs(), w.base)
            } else {
                to_string_base_unsigned(&w.x.unsigned_abs(), w.base)
            };
            return f.pad_integral(false, "", &s);
        }
        f.write_char('-')?;
    }
    Display::fmt(&BaseFmtWrapper::new(w.x.unsigned_abs(), w.base), f)
}

fn to_string_base_signed<U: Digits<u8>, S: Copy + Eq + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &S,
    base: u8,
) -> String {
    assert!((2..=62).contains(&base), "base out of range");
    if *x == S::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.unsigned_abs().to_digits_desc(&u8::wrapping_from(base));
        let map = if base > 36 {
            // above base 36 there is only one alphabet, with both cases as distinct digits
            digit_to_display_byte_large
        } else {
            digit_to_display_byte_lower
        };
        for digit in &mut digits {
            *digit = map(*digit).unwrap();
        }
        if *x < S::ZERO {
            vec_pad_left(&mut digits, 1, b'-');
        }
        String::from_utf8(digits).unwrap()
    }
}

fn to_string_base_upper_signed<
    U: Digits<u8>,
    S: Copy + Eq + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: &S,
    base: u8,
) -> String {
    assert!((2..=62).contains(&base), "base out of range");
    if *x == S::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.unsigned_abs().to_digits_desc(&base);
        let map = if base > 36 {
            // above base 36 there is only one alphabet, with both cases as distinct digits
            digit_to_display_byte_large
        } else {
            digit_to_display_byte_upper
        };
        for digit in &mut digits {
            *digit = map(*digit).unwrap();
        }
        if *x < S::ZERO {
            vec_pad_left(&mut digits, 1, b'-');
        }
        String::from_utf8(digits).unwrap()
    }
}

macro_rules! impl_to_string_base_signed {
    ($u:ident, $s:ident) => {
        impl Display for BaseFmtWrapper<$s> {
            /// Writes a wrapped signed number to a string using a specified base.
            ///
            /// If the base is greater than 10, lowercase alphabetic letters are used by default.
            /// Using the `#` flag switches to uppercase letters. Padding with zeros works as usual.
            /// If the base is greater than 36, the uppercase and lowercase letters are distinct
            /// digits (see
            /// [`digit_to_display_byte_large`](super::to_string::digit_to_display_byte_large)), and
            /// the `#` flag has no effect.
            ///
            /// Unlike with the default implementations of [`Binary`](std::fmt::Binary),
            /// [`Octal`](std::fmt::Octal), [`LowerHex`](std::fmt::LowerHex), and
            /// [`UpperHex`](std::fmt::UpperHex), negative numbers are represented using a negative
            /// sign, not two's complement.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string).
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result {
                fmt_signed(self, f)
            }
        }

        impl Debug for BaseFmtWrapper<$s> {
            /// Writes a wrapped signed number to a string using a specified base.
            ///
            /// If the base is greater than 10, lowercase alphabetic letters are used by default.
            /// Using the `#` flag switches to uppercase letters. Padding with zeros works as usual.
            /// If the base is greater than 36, the uppercase and lowercase letters are distinct
            /// digits (see
            /// [`digit_to_display_byte_large`](super::to_string::digit_to_display_byte_large)), and
            /// the `#` flag has no effect.
            ///
            /// Unlike with the default implementations of [`Binary`](std::fmt::Binary),
            /// [`Octal`](std::fmt::Octal), [`LowerHex`](std::fmt::LowerHex), and
            /// [`UpperHex`](std::fmt::UpperHex), negative numbers are represented using a negative
            /// sign, not two's complement.
            ///
            /// This is the same as the [`Display::fmt`] implementation.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string).
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result {
                Display::fmt(self, f)
            }
        }

        impl ToStringBase for $s {
            /// Converts a signed number to a string using a specified base.
            ///
            /// For bases up to 36, digits from 0 to 9 become [`char`]s from '0' to '9' and digits
            /// from 10 to 35 become the lowercase [`char`]s 'a' to 'z'. For bases from 37 through
            /// 62 the uppercase and lowercase letters are distinct digits, 'A' through 'Z'
            /// representing 10 through 35 and 'a' through 'z' representing 36 through 61, as in
            /// GMP.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string#to_string_base).
            #[inline]
            fn to_string_base(&self, base: u8) -> String {
                to_string_base_signed::<$u, $s>(self, base)
            }

            /// Converts a signed number to a string using a specified base.
            ///
            /// For bases up to 36, digits from 0 to 9 become [`char`]s from '0' to '9' and digits
            /// from 10 to 35 become the uppercase [`char`]s 'A' to 'Z'. For bases from 37 through
            /// 62 the uppercase and lowercase letters are distinct digits and there is only one
            /// alphabet, so the result is the same as
            /// [`to_string_base`](super::traits::ToStringBase::to_string_base)'s.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 62.
            ///
            /// # Examples
            /// See [here](super::to_string#to_string_base_upper).
            #[inline]
            fn to_string_base_upper(&self, base: u8) -> String {
                to_string_base_upper_signed::<$u, $s>(self, base)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_to_string_base_signed);
