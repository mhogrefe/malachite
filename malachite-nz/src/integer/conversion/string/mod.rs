// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`FromSciString`](malachite_base::num::conversion::traits::FromSciString).
/// This is a trait for converting strings, possibly using scientific notation, to numbers.
pub mod from_sci_string;
/// Implementations of [`FromStr`](core::str::FromStr) and of
/// [`FromStringBase`](malachite_base::num::conversion::traits::FromStringBase), a trait for
/// converting strings in a specified base to numbers.
pub mod from_string;
/// Implementations of [`ToSci`](malachite_base::num::conversion::traits::ToSci), a trait for
/// converting a number to string, possibly using scientific notation.
pub mod to_sci;
/// Implementations of [`Display`](core::fmt::Display), [`Debug`], [`Binary`](core::fmt::Binary),
/// [`Octal`](core::fmt::Octal), [`LowerHex`](core::fmt::LowerHex), and
/// [`UpperHex`](core::fmt::UpperHex), and of the
/// [`ToStringBase`](malachite_base::num::conversion::traits::ToStringBase) trait, used for
/// converting numbers to strings.
pub mod to_string;
