use rounding_modes::RoundingMode;
use std::num::ParseIntError;

/// This trait defines a conversion from another type. If the conversion fails, `None` is returned.
/// If `CheckedFrom` is implemented, it usually makes sense to implement `ConvertibleFrom` as well.
pub trait CheckedFrom<T>: Sized {
    fn checked_from(value: T) -> Option<Self>;
}

/// This trait defines a conversion to another type. If the conversion fails, `None` is returned. It
/// is recommended that this trait is not implemented directly; it is automatically implemented when
/// `CheckedFrom` is implemented.
pub trait CheckedInto<T> {
    fn checked_into(self) -> Option<T>;
}

impl<T, U> CheckedInto<U> for T
where
    U: CheckedFrom<T>,
{
    #[inline]
    fn checked_into(self) -> Option<U> {
        U::checked_from(self)
    }
}

/// This trait defines a conversion from another type. If the conversion fails, the function panics.
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `CheckedFrom` is implemented.
pub trait ExactFrom<T>: Sized {
    fn exact_from(value: T) -> Self;
}

/// This trait defines a conversion to another type. If the conversion fails, the function panics.
/// It is recommended that this trait is not implemented directly; it is automatically implemented
/// when `ExactFrom` is implemented.
pub trait ExactInto<T> {
    fn exact_into(self) -> T;
}

impl<T, U> ExactFrom<T> for U
where
    U: CheckedFrom<T>,
{
    #[inline]
    fn exact_from(value: T) -> U {
        U::checked_from(value).unwrap()
    }
}

impl<T, U> ExactInto<U> for T
where
    U: ExactFrom<T>,
{
    #[inline]
    fn exact_into(self) -> U {
        U::exact_from(self)
    }
}

/// This trait defines a conversion from another type, where if the conversion is not exact the
/// result can wrap around. If `WrappingFrom` is implemented, it usually makes sense to implement
/// `OverflowingFrom` as well.
pub trait WrappingFrom<T>: Sized {
    fn wrapping_from(value: T) -> Self;
}

/// This trait defines a conversion to another type, where if the conversion is not exact the result
/// can wrap around. It is recommended that this trait is not implemented directly; it is
/// automatically implemented when `WrappingFrom` is implemented.
pub trait WrappingInto<T>: Sized {
    fn wrapping_into(self) -> T;
}

impl<T, U> WrappingInto<U> for T
where
    U: WrappingFrom<T>,
{
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
/// is set to the maximum or minimum value of the result type, whichever is closer. It is
/// recommended that this trait is not implemented directly; it is automatically implemented when
/// `SaturatingFrom` is implemented.
pub trait SaturatingInto<T>: Sized {
    fn saturating_into(self) -> T;
}

impl<T, U> SaturatingInto<U> for T
where
    U: SaturatingFrom<T>,
{
    #[inline]
    fn saturating_into(self) -> U {
        U::saturating_from(self)
    }
}

/// This trait defines a conversion from another type, where if the conversion is not exact the
/// result can wrap around. The result is returned along with a `bool` that indicates whether
/// wrapping has occurred. If `OverflowingFrom` is implemented, it usually makes sense to implement
/// `WrappingFrom` as well.
pub trait OverflowingFrom<T>: Sized {
    fn overflowing_from(value: T) -> (Self, bool);
}

/// This trait defines a conversion to another type, where if the conversion is not exact the result
/// can wrap around. The result is returned along with a `bool` that indicates whether wrapping has
/// occurred. It is recommended that this trait is not implemented directly; it is automatically
/// implemented when `OverflowingFrom` is implemented.
pub trait OverflowingInto<T>: Sized {
    fn overflowing_into(self) -> (T, bool);
}

impl<T, U> OverflowingInto<U> for T
where
    U: OverflowingFrom<T>,
{
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
/// specified `RoundingMode`. It is recommended that this trait is not implemented directly; it is
/// automatically implemented when `OverflowingFrom` is implemented.
pub trait RoundingInto<T>: Sized {
    fn rounding_into(self, rm: RoundingMode) -> T;
}

impl<T, U> RoundingInto<U> for T
where
    U: RoundingFrom<T>,
{
    #[inline]
    fn rounding_into(self, rm: RoundingMode) -> U {
        U::rounding_from(self, rm)
    }
}

/// This trait provides a function that tests whether a value of type `T` is convertible into a
/// value of type `Self`. If `ConvertibleFrom<T>` for `Self` is implemented, it usually makes sense
/// to implement `CheckedFrom` for `T` as well.
pub trait ConvertibleFrom<T> {
    fn convertible_from(value: T) -> bool;
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
    ///
    fn lower_half(&self) -> Self::Half;

    /// Extracts the upper, or most significant half, of `self`.
    fn upper_half(&self) -> Self::Half;

    /// Extracts both halves of `self`; the upper, or most significant half, comes first.
    ///
    /// Time: worst case O(max(f(n), g(n))), where f(n) is the worst-case time complexity of
    ///     `Self::lower_half` and g(n) is the worst-case time complexity of `Self::upper_half`.
    ///
    /// Additional memory: worst case O(max(f(n), g(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::lower_half` and g(n) is the worst-case
    ///     additional-memory complexity of `Self::upper_half.
    ///
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
