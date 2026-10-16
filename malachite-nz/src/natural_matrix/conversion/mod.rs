// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`From`] for converting an
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix) to a
/// [`NaturalMatrix`](super::NaturalMatrix).
pub mod from_unsigned_matrix;
#[cfg(feature = "serde")]
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Conversions between [`NaturalMatrix`](super::NaturalMatrix)s and strings.
pub mod string;
