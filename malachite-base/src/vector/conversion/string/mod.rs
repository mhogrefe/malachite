// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`FromStr`](core::str::FromStr) for [`Vector`](super::super::Vector).
pub mod from_string;
/// An implementation of [`ToLatex`](crate::strings::latex::ToLatex) for
/// [`Vector`](super::super::Vector).
pub mod latex;
/// Implementations of [`Display`](core::fmt::Display) and [`Debug`](core::fmt::Debug) for
/// [`Vector`](super::super::Vector).
pub mod to_string;
/// An implementation of [`ToTypst`](crate::strings::typst::ToTypst) for
/// [`Vector`](super::super::Vector).
pub mod typst;
