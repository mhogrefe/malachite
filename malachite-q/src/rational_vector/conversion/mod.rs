// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for converting a [`Vec`](alloc::vec::Vec) or slice of [`Rational`](crate::Rational)s
/// to a [`RationalVector`](super::RationalVector).
pub mod from_elements;
/// Implementations of traits for converting an
/// [`IntegerVector`](malachite_nz::integer_vector::IntegerVector) to a
/// [`RationalVector`](super::RationalVector).
pub mod from_integer_vector;
/// Implementations of traits for converting a
/// [`NaturalVector`](malachite_nz::natural_vector::NaturalVector) to a
/// [`RationalVector`](super::RationalVector).
pub mod from_natural_vector;
/// A function for building a [`RationalVector`](super::RationalVector) from a vector of numerators
/// and a single denominator.
pub mod from_numerators_and_denominator;
/// Implementations of traits for converting an
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector) to a
/// [`RationalVector`](super::RationalVector).
pub mod from_unsigned_vector;
/// Functions for converting a [`RationalVector`](super::RationalVector) to and from a [`String`].
pub mod string;
/// Functions for converting a [`RationalVector`](super::RationalVector) to a
/// [`Vec`](alloc::vec::Vec) or slice of [`Rational`](crate::Rational)s.
pub mod to_elements;
/// A function for clearing the denominators of a [`RationalVector`](super::RationalVector), giving
/// a vector of numerators and a single denominator.
pub mod to_numerators_and_denominator;
