// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Convergents`](super::traits::Convergents), a trait for generating the
/// convergents of a number.
pub mod convergents;
/// Functions for constructing a [`Rational`](crate::Rational) from a continued fraction.
pub mod from_continued_fraction;
/// Implementations of [`ContinuedFraction`](super::traits::ContinuedFraction), a trait for
/// generating the continued fraction of a number.
pub mod to_continued_fraction;
