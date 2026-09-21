// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for converting a value that a [`Integer`](crate::integer::Integer) can
/// be converted from into a constant [`IntegerPolynomial`](super::IntegerPolynomial).
pub mod from_integer;
/// Implementations of traits for converting a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial) to an
/// [`IntegerPolynomial`](super::IntegerPolynomial).
pub mod from_natural_polynomial;
/// Implementations of traits for converting a
/// [`U64Polynomial`](malachite_base::u64_polynomial::U64Polynomial) to an
/// [`IntegerPolynomial`](super::IntegerPolynomial).
pub mod from_u64_polynomial;
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Functions for converting an [`IntegerPolynomial`](super::IntegerPolynomial) to and from a
/// [`String`](alloc::string::String).
pub mod string;
