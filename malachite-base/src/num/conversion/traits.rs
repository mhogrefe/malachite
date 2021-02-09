use rounding_modes::RoundingMode;
use std::num::ParseIntError;

/// This trait defines a conversion from another type. If the conversion fails, `None` is returned.
///
/// If `CheckedFrom` is implemented, it usually makes sense to implement `ConvertibleFrom` as well.
pub trait CheckedFrom<T>: Sized {
    fn checked_from(value: T) -> Option<Self>;
}

/// This trait defines a conversion to another type. If the conversion fails, `None` is returned.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `CheckedFrom` is implemented.
pub trait CheckedInto<T> {
    fn checked_into(self) -> Option<T>;
}

impl<T, U: CheckedFrom<T>> CheckedInto<U> for T {
    #[inline]
    fn checked_into(self) -> Option<U> {
        U::checked_from(self)
    }
}

/// This trait defines a conversion from another type. If the conversion fails, the function panics.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `CheckedFrom` is implemented.
pub trait ExactFrom<T>: Sized {
    fn exact_from(value: T) -> Self;
}

/// This trait defines a conversion to another type. If the conversion fails, the function panics.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `ExactFrom` is implemented.
pub trait ExactInto<T> {
    fn exact_into(self) -> T;
}

impl<T, U: CheckedFrom<T>> ExactFrom<T> for U {
    #[inline]
    fn exact_from(value: T) -> U {
        U::checked_from(value).unwrap()
    }
}

impl<T, U: ExactFrom<T>> ExactInto<U> for T {
    #[inline]
    fn exact_into(self) -> U {
        U::exact_from(self)
    }
}

/// This trait defines a conversion from another type, where if the conversion is not exact the
/// result will wrap around.
///
/// If `WrappingFrom` is implemented, it usually makes sense to implement `OverflowingFrom` as well.
pub trait WrappingFrom<T>: Sized {
    fn wrapping_from(value: T) -> Self;
}

/// This trait defines a conversion to another type, where if the conversion is not exact the result
/// will wrap around.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `WrappingFrom` is implemented.
pub trait WrappingInto<T>: Sized {
    fn wrapping_into(self) -> T;
}

impl<T, U: WrappingFrom<T>> WrappingInto<U> for T {
    #[inline]
    fn wrapping_into(self) -> U {
        U::wrapping_from(self)
    }
}

/// This trait defines a conversion from another type, where if the conversion is not exact the
/// result is set to the maximum or minimum value of the result type, whichever is closer.
pub trait SaturatingFrom<T>: Sized {
    fn saturating_from(value: T) -> Self;
}

/// This trait defines a conversion to another type, where if the conversion is not exact the result
/// is set to the maximum or minimum value of the result type, whichever is closer.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `SaturatingFrom` is implemented.
pub trait SaturatingInto<T>: Sized {
    fn saturating_into(self) -> T;
}

impl<T, U: SaturatingFrom<T>> SaturatingInto<U> for T {
    #[inline]
    fn saturating_into(self) -> U {
        U::saturating_from(self)
    }
}

/// This trait defines a conversion from another type, where if the conversion is not exact the
/// result will wrap around. The result is returned along with a `bool` that indicates whether
/// wrapping has occurred.
///
/// If `OverflowingFrom` is implemented, it usually makes sense to implement `WrappingFrom` as well.
pub trait OverflowingFrom<T>: Sized {
    fn overflowing_from(value: T) -> (Self, bool);
}

/// This trait defines a conversion to another type, where if the conversion is not exact the result
/// can wrap around. The result is returned along with a `bool` that indicates whether wrapping has
/// occurred.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `OverflowingFrom` is implemented.
pub trait OverflowingInto<T>: Sized {
    fn overflowing_into(self) -> (T, bool);
}

impl<T, U: OverflowingFrom<T>> OverflowingInto<U> for T {
    #[inline]
    fn overflowing_into(self) -> (U, bool) {
        U::overflowing_from(self)
    }
}

/// This trait defines a conversion from another type, where the conversion is made according to a
/// specified `RoundingMode`.
pub trait RoundingFrom<T>: Sized {
    fn rounding_from(value: T, rm: RoundingMode) -> Self;
}

/// This trait defines a conversion to another type, where the conversion is made according to a
/// specified `RoundingMode`.
///
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `RoundingFrom` is implemented.
pub trait RoundingInto<T>: Sized {
    fn rounding_into(self, rm: RoundingMode) -> T;
}

impl<T, U: RoundingFrom<T>> RoundingInto<U> for T {
    #[inline]
    fn rounding_into(self, rm: RoundingMode) -> U {
        U::rounding_from(self, rm)
    }
}

/// This trait provides a function that tests whether a value of type `T` is convertible into a
/// value of type `Self`.
///
/// If `ConvertibleFrom<T>` for `Self` is implemented, it usually makes sense to implement
/// `CheckedFrom` for `T` as well.
pub trait ConvertibleFrom<T> {
    fn convertible_from(value: T) -> bool;
}

//TODO
pub trait ToStringBase {
    fn to_string_base(&self, base: u64) -> String;

    fn to_string_base_upper(&self, base: u64) -> String;
}

//TODO
/// Converts a string slice in a given base to a value.
///
/// The string is expected to be an optional `+` sign followed by digits. Leading and trailing
/// whitespace represent an error. Digits are a subset of these characters, depending on `radix`:
///
/// * `0-9`
/// * `a-z`
/// * `A-Z`
pub trait FromStrRadix: Sized {
    fn from_str_radix(src: &str, radix: u64) -> Result<Self, ParseIntError>;
}

/// Associates with `Self` a type that's half `Self`'s size.
pub trait HasHalf {
    /// The type that's half the size of `Self`.
    type Half;
}

/// Provides a function to join two pieces into a value. For example, two `u32`s may be joined to
/// form a `u64`.
pub trait JoinHalves: HasHalf {
    /// Joins two values into a single value; the upper, or most significant half, comes first.
    fn join_halves(upper: Self::Half, lower: Self::Half) -> Self;
}

/// Provides functions to split a value into two pieces. For example, a `u64` may be split into two
/// `u32`s.
pub trait SplitInHalf: HasHalf {
    /// Extracts the lower, or least significant half, of `self`.
    fn lower_half(&self) -> Self::Half;

    /// Extracts the upper, or most significant half, of `self`.
    fn upper_half(&self) -> Self::Half;

    /// Extracts both halves of `self`; the upper, or most significant half, comes first.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(\max(T_U(n), T_L(n)))$
    ///
    /// $M(n) = O(\max(M_U(n), M_L(n)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $T_U$ and $T_L$ are the time complexities of
    /// the `upper_half` and `lower_half` functions, respectively, and $M_U$ and $M_L$ are the
    /// memory complexities of the `upper_half` and `lower_half` functions, respectively.
    #[inline]
    fn split_in_half(&self) -> (Self::Half, Self::Half) {
        (self.upper_half(), self.lower_half())
    }
}

/// Converts a slice of one type of value to a single value of another type.
pub trait FromOtherTypeSlice<T: Sized> {
    fn from_other_type_slice(slice: &[T]) -> Self;
}

/// Converts a slice of one type of value to a `Vec` of another type.
pub trait VecFromOtherTypeSlice<T: Sized>: Sized {
    fn vec_from_other_type_slice(slice: &[T]) -> Vec<Self>;
}

/// Converts a slice of one type of value to a `Vec` of another type.
pub trait VecFromOtherType<T>: Sized {
    fn vec_from_other_type(value: T) -> Vec<Self>;
}

/// This trait defines functions that express a value as a `Vec` of digits and read a value from an
/// iterator of digits, where the base is a power of two.
///
/// The base-2 logarithm of the base is specified, and the trait is parameterized by the digit type.
pub trait PowerOfTwoDigits<T> {
    /// Returns a `Vec` containing the digits of a value in ascending order: least- to most-
    /// significant.
    ///
    /// The base is $2^\ell$, where $\ell$ is `log_base`.
    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<T>;

    /// Returns a `Vec` containing the digits of a value in descending order: most- to least-
    /// significant.
    ///
    /// The base is $2^\ell$, where $\ell$ is `log_base`.
    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<T>;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in ascending order: least- to most-significant. The base is $2^\ell$,
    /// where $\ell$ is `log_base`.
    fn from_power_of_two_digits_asc<I: Iterator<Item = T>>(log_base: u64, digits: I) -> Self;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in descending order: most- to least-significant. The base is $2^\ell$,
    /// where $b$ is `log_base`.
    fn from_power_of_two_digits_desc<I: Iterator<Item = T>>(log_base: u64, digits: I) -> Self;
}

/// An iterator over a value's base-power-of-two digits.
pub trait PowerOfTwoDigitIterator<T>: Iterator<Item = T> + DoubleEndedIterator<Item = T> {
    fn get(&self, index: u64) -> T;
}

/// This trait defines an iterator over a value's base-power-of-two digits.
pub trait PowerOfTwoDigitIterable<T> {
    type PowerOfTwoDigitIterator: PowerOfTwoDigitIterator<T>;

    /// Returns a double-ended iterator over a value's digits in base $2^\ell$, where $\ell$ is
    /// `log_base`.
    ///
    /// The iterator ends after the value's most-significant digit.
    fn power_of_two_digits(self, log_base: u64) -> Self::PowerOfTwoDigitIterator;
}

/// This trait defines functions that express a value as a `Vec` of digits and read a value from an
/// iterator of digits.
///
/// The trait is parameterized by `T`, which is both the digit type and the base type.
pub trait Digits<T> {
    /// Returns a `Vec` containing the digits of a value in ascending order: least- to most-
    /// significant.
    fn to_digits_asc(&self, base: &T) -> Vec<T>;

    /// Returns a `Vec` containing the digits of a value in descending order: most- to least-
    /// significant.
    fn to_digits_desc(&self, base: &T) -> Vec<T>;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in ascending order: least- to most-significant.
    fn from_digits_asc<I: Iterator<Item = T>>(base: &T, digits: I) -> Self;

    /// Converts an iterator of digits into a value.
    ///
    /// The input digits are in descending order: most- to least-significant.
    fn from_digits_desc<I: Iterator<Item = T>>(base: &T, digits: I) -> Self;
}
