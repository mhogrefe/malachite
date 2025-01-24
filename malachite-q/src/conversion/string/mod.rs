// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`FromSciString`](malachite_base::num::conversion::traits::FromSciString).
/// This is a trait for converting strings, possibly using scientific notation, to numbers.
pub mod from_sci_string;
/// An implementation of [`FromStr`](std::str::FromStr).
pub mod from_string;
/// Implementations of [`ToSci`](malachite_base::num::conversion::traits::ToSci), a trait for
/// converting a number to string, possibly using scientific notation.
pub mod to_sci;
/// Implementations of [`Display`](std::fmt::Display) and [`Debug`].
pub mod to_string;
