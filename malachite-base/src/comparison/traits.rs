// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Defines the minimum value of a type.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Min {
    /// The minimum value of `Self`.
    const MIN: Self;
}

/// Defines the maximum value of a type.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Max {
    /// The maximum value of `Self`.
    const MAX: Self;
}
