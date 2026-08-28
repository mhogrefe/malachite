// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`From`] for converting values to a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger).
pub mod from;
/// Implementations of [`ImaginaryFrom`](malachite_base::num::conversion::traits::ImaginaryFrom) for
/// converting values to purely imaginary
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod imaginary_from;
/// Conversions to and from strings.
pub mod string;
