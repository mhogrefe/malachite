// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`FromStr`](core::str::FromStr), and a function for reading a
/// [`RationalPolynomial`](super::super::RationalPolynomial) whose variable is named by any
/// [`VarScheme`](malachite_base::vars::VarScheme).
pub mod from_string;
/// Implementations of [`ToLatex`](malachite_base::strings::latex::ToLatex), and a function for
/// writing an [`RationalPolynomial`](super::super::RationalPolynomial) whose variable is named by
/// any [`VarScheme`](malachite_base::vars::VarScheme).
pub mod latex;
/// An implementation of [`Display`](core::fmt::Display), and a function for writing a
/// [`RationalPolynomial`](super::super::RationalPolynomial) whose variable is named by any
/// [`VarScheme`](malachite_base::vars::VarScheme).
pub mod to_string;
/// Implementations of [`ToTypst`](malachite_base::strings::typst::ToTypst), and a function for
/// writing an [`RationalPolynomial`](super::super::RationalPolynomial) whose variable is named by
/// any [`VarScheme`](malachite_base::vars::VarScheme).
pub mod typst;
