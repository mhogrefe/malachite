// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Iterators that generate sets without repetition.
pub mod exhaustive;
/// The implementations of [`ToLatex`](crate::strings::latex::ToLatex) for sets.
pub mod latex;
#[cfg(feature = "random")]
/// Iterators that generate sets randomly.
pub mod random;
/// The implementations of [`ToTypst`](crate::strings::typst::ToTypst) for sets.
pub mod typst;
