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

/// Provides the constant 1/2.
#[allow(clippy::declare_interior_mutable_const)]
pub trait OneHalf {
    const ONE_HALF: Self;
}

/// The [Iverson bracket](https://en.wikipedia.org/wiki/Iverson_bracket): converts a [`bool`] to 0
/// or 1.
pub trait Iverson {
    fn iverson(b: bool) -> Self;
}

/// Converts a [`bool`] to 0 or 1.
///
/// This function is known as the [Iverson bracket](https://en.wikipedia.org/wiki/Iverson_bracket).
///
/// $$
/// f(P) = \[P\] = \\begin{cases}
///     1 & \text{if} \\quad P, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Iverson;
///
/// assert_eq!(u32::iverson(false), 0);
/// assert_eq!(i8::iverson(true), 1);
/// ```
impl<T: One + Sized + Zero> Iverson for T {
    #[inline]
    fn iverson(b: bool) -> T {
        if b {
            T::ONE
        } else {
            T::ZERO
        }
    }
}
