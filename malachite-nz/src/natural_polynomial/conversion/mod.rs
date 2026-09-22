// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for converting a value that a [`Natural`](crate::natural::Natural) can
/// be converted from into a constant [`NaturalPolynomial`](super::NaturalPolynomial).
pub mod from_natural;
/// Implementations of traits for converting a
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial) to a
/// [`NaturalPolynomial`](super::NaturalPolynomial).
pub mod from_unsigned_polynomial;
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Functions for converting a [`NaturalPolynomial`](super::NaturalPolynomial) to and from a
/// [`String`](alloc::string::String).
pub mod string;
