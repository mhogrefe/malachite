// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`From`] for converting an
/// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) to a
/// [`RationalMatrix`](super::RationalMatrix).
pub mod from_integer_matrix;
/// An implementation of [`From`] for converting a
/// [`NaturalMatrix`](malachite_nz::natural_matrix::NaturalMatrix) to a
/// [`RationalMatrix`](super::RationalMatrix).
pub mod from_natural_matrix;
/// An implementation of [`From`] for converting an
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix) to a
/// [`RationalMatrix`](super::RationalMatrix).
pub mod from_unsigned_matrix;
#[cfg(feature = "serde")]
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Conversions between [`RationalMatrix`](super::RationalMatrix)s and strings.
pub mod string;
