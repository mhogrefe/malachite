// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for computing the cosine of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod cos;
/// Functions for computing the cotangent of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod cot;
/// Functions for computing the cosecant of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod csc;
/// Functions for computing the secant of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod sec;
/// Functions for computing the sine of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod sin;
/// Functions for computing the sine and cosine of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s together, and the implementation shared by the forward
/// trigonometric functions.
pub mod sin_cos;
/// Functions for computing the tangent of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod tan;
