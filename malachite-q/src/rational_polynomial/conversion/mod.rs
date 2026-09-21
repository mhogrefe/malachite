// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for converting a value that a [`Rational`](crate::Rational) can be
/// converted from into a constant [`RationalPolynomial`](super::RationalPolynomial).
pub mod from_rational;
/// Functions for converting an [`RationalPolynomial`](super::RationalPolynomial) to and from a
/// [`String`](alloc::string::String).
pub mod string;
