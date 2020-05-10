/// Provides the constant 0.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Zero {
    const ZERO: Self;
}

/// Provides the constant 1.
#[allow(clippy::declare_interior_mutable_const)]
pub trait One {
    const ONE: Self;
}

/// Provides the constant 2.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Two {
    const TWO: Self;
}

/// Provides the constant -1.
#[allow(clippy::declare_interior_mutable_const)]
pub trait NegativeOne {
    const NEGATIVE_ONE: Self;
}

/// The Iverson bracket: converts a `bool` to 0 or 1. It should be used sparingly, but sometimes it
/// is the cleanest option.
pub trait Iverson {
    fn iverson(b: bool) -> Self;
}

impl<T: One + Sized + Zero> Iverson for T {
    /// Converts a `bool` to 0 or 1.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::basic::traits::Iverson;
    ///
    /// assert_eq!(u32::iverson(false), 0);
    /// assert_eq!(i8::iverson(true), 1);
    /// ```
    #[inline]
    fn iverson(b: bool) -> T {
        if b {
            T::ONE
        } else {
            T::ZERO
        }
    }
}
