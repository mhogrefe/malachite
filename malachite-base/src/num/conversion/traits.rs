// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::string::options::{FromSciStringOptions, ToSciOptions};
use crate::num::conversion::string::to_sci::SciWrapper;
use crate::rounding_modes::RoundingMode;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::{Formatter, Result};

/// Expresses a value as a [`Vec`] of digits, or reads a value from an iterator of digits.
///
/// The trait is parameterized by `T`, which is both the digit type and the base type.
pub trait Digits<T>: Sized {
    /// Returns a [`Vec`] containing the digits of a value in ascending order: least- to
    /// most-significant.
    fn to_digits_asc(&self, base: &T) -> Vec<T>;

    /// Returns a [`Vec`] containing the digits of a value in descending order: most- to
    /// least-significant.
    fn to_digits_desc(&self, base: &T) -> Vec<T>;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in ascending order: least- to most-significant.
    fn from_digits_asc<I: Iterator<Item = T>>(base: &T, digits: I) -> Option<Self>;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in descending order: most- to least-significant.
    fn from_digits_desc<I: Iterator<Item = T>>(base: &T, digits: I) -> Option<Self>;
}

/// An iterator over a value's base-$2^k$ digits.
pub trait PowerOf2DigitIterator<T>: Iterator<Item = T> + DoubleEndedIterator<Item = T> {
    fn get(&self, index: u64) -> T;
}

/// Creates an iterator over a value's base-$2^k$ digits.
pub trait PowerOf2DigitIterable<T> {
    type PowerOf2DigitIterator: PowerOf2DigitIterator<T>;

    /// Returns a double-ended iterator over a value's digits in base $2^l$, where $k$ is
    /// `log_base`.
    ///
    /// The iterator ends after the value's most-significant digit.
    fn power_of_2_digits(self, log_base: u64) -> Self::PowerOf2DigitIterator;
}

/// Expresses a value as a [`Vec`] of base-$2^k$ digits, or reads a value from an iterator of
/// base-$2^k$ digits.
///
/// The trait is parameterized by the digit type.
pub trait PowerOf2Digits<T>: Sized {
    /// Returns a [`Vec`] containing the digits of a value in ascending order: least- to
    /// most-significant.
    ///
    /// The base is $2^k$, where $k$ is `log_base`.
    fn to_power_of_2_digits_asc(&self, log_base: u64) -> Vec<T>;

    /// Returns a [`Vec`] containing the digits of a value in descending order: most- to
    /// least-significant.
    ///
    /// The base is $2^k$, where $k$ is `log_base`.
    fn to_power_of_2_digits_desc(&self, log_base: u64) -> Vec<T>;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in ascending order: least- to most-significant. The base is $2^k$,
    /// where $k$ is `log_base`.
    fn from_power_of_2_digits_asc<I: Iterator<Item = T>>(log_base: u64, digits: I) -> Option<Self>;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in descending order: most- to least-significant. The base is $2^k$,
    /// where $k$ is `log_base`.
    fn from_power_of_2_digits_desc<I: Iterator<Item = T>>(log_base: u64, digits: I)
        -> Option<Self>;
}

/// Converts a string slice in a given base to a value.
pub trait FromStringBase: Sized {
    fn from_string_base(base: u8, s: &str) -> Option<Self>;
}

/// Converts a number to a string using a specified base.
pub trait ToStringBase {
    /// Converts a signed number to a lowercase string using a specified base.
    fn to_string_base(&self, base: u8) -> String;

    /// Converts a signed number to an uppercase string using a specified base.
    fn to_string_base_upper(&self, base: u8) -> String;
}

/// Converts a number to a string, possibly in scientific notation.
pub trait ToSci: Sized {
    /// Formats a number, possibly in scientific notation.
    fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> Result;

    /// Determines whether some formatting options can be applied to a number.
    fn fmt_sci_valid(&self, options: ToSciOptions) -> bool;

    /// Converts a number to a string, possibly in scientific notation.
    fn to_sci_with_options(&self, options: ToSciOptions) -> SciWrapper<Self> {
        SciWrapper { x: self, options }
    }

    /// Converts a number to a string, possibly in scientific notation, using the default
    /// [`ToSciOptions`].
    #[inline]
    fn to_sci(&self) -> SciWrapper<Self> {
        SciWrapper {
            x: self,
            options: ToSciOptions::default(),
        }
    }
}

/// Converts a `&str`, possibly in scientific notation, to a number.
pub trait FromSciString: Sized {
    /// Converts a `&str`, possibly in scientific notation, to a number.
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Self>;

    /// Converts a `&str`, possibly in scientific notation, to a number, using the default
    /// [`FromSciStringOptions`].
    #[inline]
    fn from_sci_string(s: &str) -> Option<Self> {
        Self::from_sci_string_with_options(s, FromSciStringOptions::default())
    }
}

/// Converts a value from one type to another. If the conversion fails, the function panics.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when [`TryFrom`] is implemented.
pub trait ExactFrom<T>: Sized {
    fn exact_from(value: T) -> Self;
}

/// Converts a value from one type to another. If the conversion fails, the function panics.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when [`ExactFrom`] is implemented.
pub trait ExactInto<T> {
    fn exact_into(self) -> T;
}

impl<T, U: TryFrom<T>> ExactFrom<T> for U {
    #[inline]
    fn exact_from(value: T) -> U {
        U::try_from(value).ok().unwrap()
    }
}

impl<T, U: ExactFrom<T>> ExactInto<U> for T {
    #[inline]
    fn exact_into(self) -> U {
        U::exact_from(self)
    }
}

/// Converts a value from one type to another. where if the conversion is not exact the result will
/// wrap around.
///
/// If `WrappingFrom` is implemented, it usually makes sense to implement [`OverflowingFrom`] as
/// well.
pub trait WrappingFrom<T>: Sized {
    fn wrapping_from(value: T) -> Self;
}

/// Converts a value from one type to another, where if the conversion is not exact the result will
/// wrap around.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when [`WrappingFrom`] is implemented.
pub trait WrappingInto<T>: Sized {
    fn wrapping_into(self) -> T;
}

impl<T, U: WrappingFrom<T>> WrappingInto<U> for T {
    #[inline]
    fn wrapping_into(self) -> U {
        U::wrapping_from(self)
    }
}

/// Converts a value from one type to another, where if the conversion is not exact the result is
/// set to the maximum or minimum value of the result type, whichever is closer.
pub trait SaturatingFrom<T>: Sized {
    fn saturating_from(value: T) -> Self;
}

/// Converts a value from one type to another, where if the conversion is not exact the result is
/// set to the maximum or minimum value of the result type, whichever is closer.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when [`SaturatingFrom`] is implemented.
pub trait SaturatingInto<T>: Sized {
    fn saturating_into(self) -> T;
}

impl<T, U: SaturatingFrom<T>> SaturatingInto<U> for T {
    #[inline]
    fn saturating_into(self) -> U {
        U::saturating_from(self)
    }
}

/// Converts a value from one type to another, where if the conversion is not exact the result will
/// wrap around. The result is returned along with a [`bool`] that indicates whether wrapping has
/// occurred.
///
/// If `OverflowingFrom` is implemented, it usually makes sense to implement [`WrappingFrom`] as
/// well.
pub trait OverflowingFrom<T>: Sized {
    fn overflowing_from(value: T) -> (Self, bool);
}

/// Converts a value from one type to another, where if the conversion is not exact the result will
/// wrap around. The result is returned along with a [`bool`] that indicates whether wrapping has
/// occurred.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when [`OverflowingFrom`] is implemented.
pub trait OverflowingInto<T>: Sized {
    fn overflowing_into(self) -> (T, bool);
}

impl<T, U: OverflowingFrom<T>> OverflowingInto<U> for T {
    #[inline]
    fn overflowing_into(self) -> (U, bool) {
        U::overflowing_from(self)
    }
}

/// Converts a value from one type to another, where the conversion is made according to a specified
/// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the returned value is
/// less than, equal to, or greater than the original value.
pub trait RoundingFrom<T>: Sized {
    fn rounding_from(value: T, rm: RoundingMode) -> (Self, Ordering);
}

/// Converts a value from one type to another, where the conversion is made according to a specified
/// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the returned value is
/// less than, equal to, or greater than the original value.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when [`RoundingFrom`] is implemented.
pub trait RoundingInto<T>: Sized {
    fn rounding_into(self, rm: RoundingMode) -> (T, Ordering);
}

impl<T, U: RoundingFrom<T>> RoundingInto<U> for T {
    #[inline]
    fn rounding_into(self, rm: RoundingMode) -> (U, Ordering) {
        U::rounding_from(self, rm)
    }
}

/// Tests whether a value of one type is convertible into a value of another.
///
/// If `ConvertibleFrom<T>` for `Self` is implemented, it usually makes sense to implement
/// [`TryFrom`] for `T` as well.
pub trait ConvertibleFrom<T> {
    fn convertible_from(value: T) -> bool;
}

/// Associates with `Self` a type that's half `Self`'s size.
pub trait HasHalf {
    /// The type that's half the size of `Self`.
    type Half;
}

/// Provides a function to join two pieces into a number. For example, two [`u32`]s may be joined to
/// form a [`u64`].
pub trait JoinHalves: HasHalf {
    /// Joins two values into a single value; the upper, or most-significant, half comes first.
    fn join_halves(upper: Self::Half, lower: Self::Half) -> Self;
}

/// Provides functions to split a number into two pieces. For example, a [`u64`] may be split into
/// two [`u32`]s.
pub trait SplitInHalf: HasHalf {
    /// Extracts the lower, or least-significant, half of a number.
    fn lower_half(&self) -> Self::Half;

    /// Extracts the upper, or most-significant half of a number.
    fn upper_half(&self) -> Self::Half;

    /// Extracts both halves of a number; the upper, or most-significant, half comes first.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(\max(T_U(n), T_L(n)))$
    ///
    /// $M(n) = O(\max(M_U(n), M_L(n)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $T_U$ and $T_L$ are the time complexities of
    /// the [`upper_half`](Self::upper_half) and [`lower_half`](Self::lower_half) functions,
    /// respectively, and $M_U$ and $M_L$ are the memory complexities of the
    /// [`upper_half`](Self::upper_half) and [`lower_half`](Self::lower_half) functions,
    /// respectively.
    #[inline]
    fn split_in_half(&self) -> (Self::Half, Self::Half) {
        (self.upper_half(), self.lower_half())
    }
}

/// Determines whether a number is an integer.
pub trait IsInteger {
    #[allow(clippy::wrong_self_convention)]
    fn is_integer(self) -> bool;
}

/// Converts a number to or from a raw mantissa and exponent.
///
/// See [here](crate::num::basic::floats::PrimitiveFloat) for a definition of raw mantissa and
/// exponent.
pub trait RawMantissaAndExponent<M, E, T = Self>: Sized {
    /// Extracts the raw mantissa and exponent from a number.
    fn raw_mantissa_and_exponent(self) -> (M, E);

    /// Extracts the raw mantissa from a number.
    fn raw_mantissa(self) -> M {
        self.raw_mantissa_and_exponent().0
    }

    /// Extracts the raw exponent from a number.
    fn raw_exponent(self) -> E {
        self.raw_mantissa_and_exponent().1
    }

    /// Constructs a number from its raw mantissa and exponent.
    fn from_raw_mantissa_and_exponent(raw_mantissa: M, raw_exponent: E) -> T;
}

/// Converts a number to or from an integer mantissa and exponent.
///
/// See [here](crate::num::basic::floats::PrimitiveFloat) for a definition of integer mantissa and
/// exponent.
///
/// The mantissa is an odd integer, and the exponent is an integer, such that $x = 2^em$.
pub trait IntegerMantissaAndExponent<M, E, T = Self>: Sized {
    /// Extracts the integer mantissa and exponent from a number.
    fn integer_mantissa_and_exponent(self) -> (M, E);

    /// Extracts the integer mantissa from a number.
    fn integer_mantissa(self) -> M {
        self.integer_mantissa_and_exponent().0
    }

    /// Extracts the integer exponent from a number.
    fn integer_exponent(self) -> E {
        self.integer_mantissa_and_exponent().1
    }

    /// Constructs a number from its integer mantissa and exponent.
    fn from_integer_mantissa_and_exponent(integer_mantissa: M, integer_exponent: E) -> Option<T>;
}

/// Converts a number to or from a scientific mantissa and exponent.
///
/// See [here](crate::num::basic::floats::PrimitiveFloat) for a definition of scientific mantissa
/// and exponent.
pub trait SciMantissaAndExponent<M, E, T = Self>: Sized {
    /// Extracts the scientific mantissa and exponent from a number.
    fn sci_mantissa_and_exponent(self) -> (M, E);

    /// Extracts the scientific mantissa from a number.
    fn sci_mantissa(self) -> M {
        self.sci_mantissa_and_exponent().0
    }

    /// Extracts the scientific exponent from a number.
    fn sci_exponent(self) -> E {
        self.sci_mantissa_and_exponent().1
    }

    /// Constructs a number from its scientific mantissa and exponent.
    fn from_sci_mantissa_and_exponent(sci_mantissa: M, sci_exponent: E) -> Option<T>;
}

/// Converts a slice of one type of value to a single value of another type.
pub trait FromOtherTypeSlice<T: Sized> {
    fn from_other_type_slice(slice: &[T]) -> Self;
}

/// Converts a slice of one type of value to a [`Vec`] of another type.
pub trait VecFromOtherTypeSlice<T: Sized>: Sized {
    fn vec_from_other_type_slice(slice: &[T]) -> Vec<Self>;
}

/// Converts a slice of one type of value to a [`Vec`] of another type.
pub trait VecFromOtherType<T>: Sized {
    fn vec_from_other_type(value: T) -> Vec<Self>;
}
