// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of
/// [`FromStringBase`](malachite_base::num::conversion::traits::FromStringBase), for power-of-2
/// bases, for [`BigExponentFloat`](crate::BigExponentFloat).
pub mod from_string;
/// Implementations of [`Binary`](core::fmt::Binary), [`Octal`](core::fmt::Octal),
/// [`LowerHex`](core::fmt::LowerHex), [`UpperHex`](core::fmt::UpperHex),
/// [`Debug`](core::fmt::Debug), and
/// [`ToStringBase`](malachite_base::num::conversion::traits::ToStringBase), for power-of-2 bases,
/// for [`BigExponentFloat`](crate::BigExponentFloat).
pub mod to_string;
