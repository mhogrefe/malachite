// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The numeric cores of the conversions in non-power-of-2 bases.
pub mod digits;
/// An implementation of [`FromSciString`](malachite_base::num::conversion::traits::FromSciString)
/// for [`BigExponentFloat`](crate::BigExponentFloat), and parsing with a given precision and
/// rounding mode.
pub mod from_sci_string;
/// Implementations of [`FromStringBase`](malachite_base::num::conversion::traits::FromStringBase)
/// and [`FromStr`](core::str::FromStr) for [`BigExponentFloat`](crate::BigExponentFloat).
pub mod from_string;
/// The implementation of [`ToLatex`](malachite_base::strings::latex::ToLatex) for a
/// [`BigExponentFloat`](crate::BigExponentFloat).
pub mod latex;
/// An implementation of [`ToSci`](malachite_base::num::conversion::traits::ToSci) for
/// [`BigExponentFloat`](crate::BigExponentFloat).
pub mod to_sci;
/// Implementations of [`Display`](core::fmt::Display), [`Binary`](core::fmt::Binary),
/// [`Octal`](core::fmt::Octal), [`LowerHex`](core::fmt::LowerHex),
/// [`UpperHex`](core::fmt::UpperHex), [`Debug`](core::fmt::Debug), and
/// [`ToStringBase`](malachite_base::num::conversion::traits::ToStringBase) for
/// [`BigExponentFloat`](crate::BigExponentFloat).
pub mod to_string;
/// The implementation of [`ToTypst`](malachite_base::strings::typst::ToTypst) for a
/// [`BigExponentFloat`](crate::BigExponentFloat).
pub mod typst;
