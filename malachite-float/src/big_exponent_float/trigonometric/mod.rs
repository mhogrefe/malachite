// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for computing the arccosine of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod acos;
/// Functions for computing the arccotangent of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod acot;
/// Functions for computing the arccosecant of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod acsc;
/// Functions for computing the arcsecant of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod asec;
/// Functions for computing the arcsine of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns.
pub mod asin;
/// Functions for computing the arctangent of [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s, in radians, in $u$ths of a turn, and in half-turns, and
/// the implementation shared by the inverse trigonometric functions.
pub mod atan;
/// Functions for computing $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$, for
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s, in
/// radians, in $u$ths of a turn, and in half-turns.
pub mod atan2;
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
