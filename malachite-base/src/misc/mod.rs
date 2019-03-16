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

/// This trait defines the minimum value of a type.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Min {
    /// The minimum value of `Self`.
    const MIN: Self;
}

/// This trait defines the minimum value of a type.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Max {
    /// The maximum value of `Self`.
    const MAX: Self;
}

/// This trait defines the name of a type. This is useful when constructing error messages in a
/// generic function.
pub trait Named {
    /// The name of `Self`.
    const NAME: &'static str;
}

/// Implements `Named` for a type.
#[macro_export]
macro_rules! impl_named {
    ($t:ident) => {
        impl Named for $t {
            /// Returns the name of a type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// #[macro_use]
            /// extern crate malachite_base;
            ///
            /// use malachite_base::misc::Named;
            ///
            /// fn main() {
            ///     assert_eq!(u8::NAME, "u8");
            ///     assert_eq!(i64::NAME, "i64");
            /// }
            /// ```
            const NAME: &'static str = stringify!($t);
        }
    };
}

/// This trait defines two functions, `increment` and `decrement`, that undo each other's effect,
/// and have the property that if a < b, b is reachable from a by a finite number of increments and
/// a is reachable from b by a finite number of decrements. If the type has a maximum value,
/// incrementing it should panic; if it has a minimum value, decrementing it should panic.
pub trait Walkable: Eq + Ord {
    /// Changes `self` to the smallest value greater than its old value. Panics if no greater value
    /// exists.
    fn increment(&mut self);

    /// Changes `self` to the greatest value smaller than its old value. Panics if no smaller value
    /// exists.
    fn decrement(&mut self);
}

#[macro_export]
macro_rules! min {
    ($first: expr $(,$next: expr)*) => {
        {
            let mut min = $first;
            $(
                let next = $next;
                if next < min {
                    min = next;
                }
            )*
            min
        }
    };
}

#[macro_export]
macro_rules! max {
    ($first: expr $(,$next: expr)*) => {
        {
            let mut max = $first;
            $(
                let next = $next;
                if next > max {
                    max = next;
                }
            )*
            max
        }
    };
}
