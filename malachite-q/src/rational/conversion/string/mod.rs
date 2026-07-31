// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`format_rational_str`](format_rational::format_rational_str), a function for formatting a
/// [`Rational`](crate::rational::Rational) according to a GMP-style `printf` format string.
pub mod format_rational;
/// Implementations of [`FromSciString`](malachite_base::num::conversion::traits::FromSciString).
/// This is a trait for converting strings, possibly using scientific notation, to numbers.
pub mod from_sci_string;
/// Implementations of [`FromStr`](std::str::FromStr) and
/// [`FromStringBase`](malachite_base::num::conversion::traits::FromStringBase), traits for parsing
/// strings in base 10 or in other bases.
pub mod from_string;
/// Implementations of [`ToSci`](malachite_base::num::conversion::traits::ToSci), a trait for
/// converting a number to string, possibly using scientific notation.
pub mod to_sci;
/// Implementations of [`Display`](std::fmt::Display), [`Debug`], [`Binary`](std::fmt::Binary),
/// [`Octal`](std::fmt::Octal), [`LowerHex`](std::fmt::LowerHex), [`UpperHex`](std::fmt::UpperHex),
/// and [`ToStringBase`](malachite_base::num::conversion::traits::ToStringBase), traits for
/// converting a number to a string in base 10 or in other bases.
pub mod to_string;
