// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for producing iterators over the digits of a [`Rational`](crate::Rational).
#[allow(clippy::module_inception)]
pub mod digits;
/// Functions for constructing a [`Rational`](crate::Rational) from digits.
pub mod from_digits;
/// Functions for constructing a [`Rational`](crate::Rational) from base-$2^k$ digits.
pub mod from_power_of_2_digits;
/// Functions for producing iterators over the base-$2^k$ digits of a [`Rational`](crate::Rational).
pub mod power_of_2_digits;
/// Functions for returning the digits of a [`Rational`](crate::Rational). The digits after the
/// point are returned as a
/// [`RationalSequence`](malachite_base::rational_sequences::RationalSequence).
pub mod to_digits;
/// Functions for returning the base-$2^k$ digits of a [`Rational`](crate::Rational). The digits
/// after the point are returned as a
/// [`RationalSequence`](malachite_base::rational_sequences::RationalSequence).
pub mod to_power_of_2_digits;
