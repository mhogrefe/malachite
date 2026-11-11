// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// A function for determining whether an approximation can be correctly rounded.
pub mod can_round;
/// Functions for determining whether a [`BigExponentFloat`](super::BigExponentFloat) is NaN,
/// infinite, zero, and so on.
pub mod classification;
/// Functions for measuring the size of a [`BigExponentFloat`](super::BigExponentFloat).
pub mod complexity;
/// Constants that a [`BigExponentFloat`](super::BigExponentFloat) can take, such as 0, 1, and NaN.
pub mod constants;
/// Functions for getting the parts of a [`BigExponentFloat`](super::BigExponentFloat), and for
/// rounding it to a given precision.
pub mod get_and_set;
