use round::RoundingMode;

/// This trait defines a conversion from another type. If the conversion fails, `None` is returned.
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

pub trait RoundingFrom<T>: Sized {
    fn rounding_from(value: T, rm: RoundingMode) -> Self;
}

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
