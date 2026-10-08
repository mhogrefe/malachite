// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for converting a [`Vec`](alloc::vec::Vec) or slice of
/// [`Integer`](crate::integer::Integer)s to an [`IntegerVector`](super::IntegerVector).
pub mod from_elements;
/// Implementations of traits for converting a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) to an
/// [`IntegerVector`](super::IntegerVector).
pub mod from_natural_vector;
/// Implementations of traits for converting an
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector) to an
/// [`IntegerVector`](super::IntegerVector).
pub mod from_unsigned_vector;
/// Functions for converting an [`IntegerVector`](super::IntegerVector) to and from a [`String`].
pub mod string;
/// Functions for converting an [`IntegerVector`](super::IntegerVector) to a
/// [`Vec`](alloc::vec::Vec) or slice of [`Integer`](crate::integer::Integer)s.
pub mod to_elements;
