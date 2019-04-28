use round::RoundingMode;

pub trait CheckedFrom<T>: Sized {
    fn checked_from(value: T) -> Option<Self>;
}

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

pub trait WrappingFrom<T>: Sized {
    fn wrapping_from(value: T) -> Self;
}

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

pub trait BitwiseFrom<T>: Sized {
    fn bitwise_from(value: T) -> Self;
}

pub trait BitwiseInto<T>: Sized {
    fn bitwise_into(self) -> T;
}

impl<T, U> BitwiseInto<U> for T
where
    U: BitwiseFrom<T>,
{
    #[inline]
    fn bitwise_into(self) -> U {
        U::bitwise_from(self)
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
