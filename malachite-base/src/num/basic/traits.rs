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

/// The Iverson bracket: converts a `bool` to 0 or 1.
pub trait Iverson {
    fn iverson(b: bool) -> Self;
}

impl<T: One + Sized + Zero> Iverson for T {
    /// Converts a `bool` to 0 or 1.
    ///
    /// This function is known as the Iverson bracket.
    ///
    /// $$
    /// f(P) = \[P\] = \\begin{cases}
    ///     1 & P \\\\
    ///     0 & \\text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::basic::traits` module.
    #[inline]
    fn iverson(b: bool) -> T {
        if b {
            T::ONE
        } else {
            T::ZERO
        }
    }
}
