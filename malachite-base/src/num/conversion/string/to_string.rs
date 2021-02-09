use num::arithmetic::traits::{CheckedLogTwo, UnsignedAbs};
use num::basic::traits::Zero;
use num::conversion::string::BaseFmtWrapper;
use num::conversion::traits::{Digits, PowerOfTwoDigitIterable, ToStringBase, WrappingFrom};
use std::fmt::{self, Debug, Display, Formatter, Write};
use vecs::vec_pad_left;

/// Converts a digit to a byte corresponding to a numeric or lowercase alphabetic `char` that
/// represents the digit.
///
/// Digits from 0 to 9 become bytes corresponding to `char`s from '0' to '9'. Digits from 10 to 35
/// become bytes representing the lowercase `char`s 'a' to 'z'.
///
/// # Worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `b` is greater than 35.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::to_string::digit_to_display_byte_lower;
///
/// assert_eq!(digit_to_display_byte_lower(0), b'0');
/// assert_eq!(digit_to_display_byte_lower(9), b'9');
/// assert_eq!(digit_to_display_byte_lower(10), b'a');
/// assert_eq!(digit_to_display_byte_lower(35), b'z');
/// ```
pub fn digit_to_display_byte_lower(b: u8) -> u8 {
    match b {
        0..=9 => b + b'0',
        10..=35 => b + b'a' - 10,
        _ => panic!("Invalid byte: {}", b),
    }
}

/// Converts a digit to a byte corresponding to a numeric or uppercase alphabetic `char` that
/// represents the digit.
///
/// Digits from 0 to 9 become bytes corresponding to `char`s from '0' to '9'. Digits from 10 to 35
/// become bytes representing the lowercase `char`s 'A' to 'Z'.
///
/// # Worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `b` is greater than 35.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::to_string::digit_to_display_byte_upper;
///
/// assert_eq!(digit_to_display_byte_upper(0), b'0');
/// assert_eq!(digit_to_display_byte_upper(9), b'9');
/// assert_eq!(digit_to_display_byte_upper(10), b'A');
/// assert_eq!(digit_to_display_byte_upper(35), b'Z');
/// ```
pub fn digit_to_display_byte_upper(b: u8) -> u8 {
    match b {
        0..=9 => b + b'0',
        10..=35 => b + b'A' - 10,
        _ => panic!("Invalid byte: {}", b),
    }
}

fn _fmt_unsigned<T: Copy + Digits<u8> + Eq + PowerOfTwoDigitIterable<u8> + Zero>(
    w: &BaseFmtWrapper<T>,
    f: &mut Formatter,
) -> fmt::Result {
    let upper = f.alternate();
    if w.x == T::ZERO {
        f.write_char('0')
    } else if let Some(log_base) = w.base.checked_log_two() {
        if upper {
            for digit in PowerOfTwoDigitIterable::<u8>::power_of_two_digits(w.x, log_base).rev() {
                f.write_char(char::from(digit_to_display_byte_upper(digit)))?;
            }
        } else {
            for digit in PowerOfTwoDigitIterable::<u8>::power_of_two_digits(w.x, log_base).rev() {
                f.write_char(char::from(digit_to_display_byte_lower(digit)))?;
            }
        }
        Ok(())
    } else {
        let mut digits = w.x.to_digits_desc(&u8::wrapping_from(w.base));
        if upper {
            for digit in &mut digits {
                *digit = digit_to_display_byte_upper(*digit);
            }
        } else {
            for digit in &mut digits {
                *digit = digit_to_display_byte_lower(*digit);
            }
        }
        write!(f, "{}", std::str::from_utf8(&digits).unwrap())
    }
}

fn _to_string_base_unsigned<T: Copy + Digits<u8> + Eq + Zero>(x: &T, base: u64) -> String {
    assert!((2..=36).contains(&base), "base out of range");
    if *x == T::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.to_digits_desc(&u8::wrapping_from(base));
        for digit in &mut digits {
            *digit = digit_to_display_byte_lower(*digit);
        }
        String::from_utf8(digits).unwrap()
    }
}

fn _to_string_base_upper_unsigned<T: Copy + Digits<u8> + Eq + Zero>(x: &T, base: u64) -> String {
    assert!((2..=36).contains(&base), "base out of range");
    if *x == T::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.to_digits_desc(&u8::wrapping_from(base));
        for digit in &mut digits {
            *digit = digit_to_display_byte_upper(*digit);
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
            /// Using the `#` flag switches to uppercase letters.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                _fmt_unsigned(self, f)
            }
        }

        impl Debug for BaseFmtWrapper<$t> {
            /// Writes a wrapped unsigned number to a string using a specified base.
            ///
            /// If the base is greater than 10, lowercase alphabetic letters are used by default.
            /// Using the `#` flag switches to uppercase letters.
            ///
            /// This is the same as the `Display::fmt` implementation.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(self, f)
            }
        }

        impl ToStringBase for $t {
            /// Converts an unsigned number to a string using a specified base.
            ///
            /// Digits from 0 to 9 become `char`s from '0' to '9'. Digits from 10 to 35 become the
            /// lowercase `char`s 'a' to 'z'.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn to_string_base(&self, base: u64) -> String {
                _to_string_base_unsigned(self, base)
            }

            /// Converts an unsigned number to a string using a specified base.
            ///
            /// Digits from 0 to 9 become `char`s from '0' to '9'. Digits from 10 to 35 become the
            /// uppercase `char`s 'A' to 'Z'.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn to_string_base_upper(&self, base: u64) -> String {
                _to_string_base_upper_unsigned(self, base)
            }
        }
    };
}
apply_to_unsigneds!(impl_to_string_base_unsigned);

fn _fmt_signed<T: Copy + Ord + UnsignedAbs + Zero>(
    w: &BaseFmtWrapper<T>,
    f: &mut Formatter,
) -> fmt::Result
where
    BaseFmtWrapper<<T as UnsignedAbs>::Output>: Display,
{
    if w.x < T::ZERO {
        f.write_char('-')?;
    }
    Display::fmt(&BaseFmtWrapper::new(w.x.unsigned_abs(), w.base), f)
}

fn _to_string_base_signed<U: Digits<u8>, S: Copy + Eq + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &S,
    base: u64,
) -> String {
    assert!((2..=36).contains(&base), "base out of range");
    if *x == S::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.unsigned_abs().to_digits_desc(&u8::wrapping_from(base));
        for digit in &mut digits {
            *digit = digit_to_display_byte_lower(*digit);
        }
        if *x < S::ZERO {
            vec_pad_left(&mut digits, 1, b'-');
        }
        String::from_utf8(digits).unwrap()
    }
}

fn _to_string_base_upper_signed<
    U: Digits<u8>,
    S: Copy + Eq + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: &S,
    base: u64,
) -> String {
    assert!((2..=36).contains(&base), "base out of range");
    if *x == S::ZERO {
        "0".to_string()
    } else {
        let mut digits = x.unsigned_abs().to_digits_desc(&u8::wrapping_from(base));
        for digit in &mut digits {
            *digit = digit_to_display_byte_upper(*digit);
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
            /// Using the `#` flag switches to uppercase letters.
            ///
            /// Unlike with the default implementations of `Binary`, `Octal`, `LowerHex`, and
            /// `UpperHex`, negative numbers are represented using a negative sign, not two's
            /// complement.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                _fmt_signed(self, f)
            }
        }

        impl Debug for BaseFmtWrapper<$s> {
            /// Writes a wrapped signed number to a string using a specified base.
            ///
            /// If the base is greater than 10, lowercase alphabetic letters are used by default.
            /// Using the `#` flag switches to uppercase letters.
            ///
            /// Unlike with the default implementations of `Binary`, `Octal`, `LowerHex`, and
            /// `UpperHex`, negative numbers are represented using a negative sign, not two's
            /// complement.
            ///
            /// This is the same as the `Display::fmt` implementation.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(self, f)
            }
        }

        impl ToStringBase for $s {
            /// Converts a signed number to a string using a specified base.
            ///
            /// Digits from 0 to 9 become `char`s from '0' to '9'. Digits from 10 to 35 become the
            /// lowercase `char`s 'a' to 'z'.
            ///
            /// Unlike with the default implementations of `Binary`, `Octal`, `LowerHex`, and
            /// `UpperHex`, negative numbers are represented using a negative sign, not two's
            /// complement.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn to_string_base(&self, base: u64) -> String {
                _to_string_base_signed::<$u, $s>(self, base)
            }

            /// Converts a signed number to a string using a specified base.
            ///
            /// Digits from 0 to 9 become `char`s from '0' to '9'. Digits from 10 to 35 become the
            /// uppercase `char`s 'A' to 'Z'.
            ///
            /// Unlike with the default implementations of `Binary`, `Octal`, `LowerHex`, and
            /// `UpperHex`, negative numbers are represented using a negative sign, not two's
            /// complement.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `base` is less than 2 or greater than 36.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::conversion::string::to_string` module.
            #[inline]
            fn to_string_base_upper(&self, base: u64) -> String {
                _to_string_base_upper_signed::<$u, $s>(self, base)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_to_string_base_signed);
