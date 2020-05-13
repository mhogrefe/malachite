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
