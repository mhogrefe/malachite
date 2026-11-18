// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for computing the inverse hyperbolic cosine of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod acosh;
/// Functions for computing the inverse hyperbolic cotangent of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod acoth;
/// Functions for computing the inverse hyperbolic cosecant of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod acsch;
/// Functions for computing the inverse hyperbolic secant of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod asech;
/// Functions for computing the inverse hyperbolic sine of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s, and the
/// implementation shared by the inverse hyperbolic functions.
pub mod asinh;
/// Functions for computing the inverse hyperbolic tangent of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod atanh;
/// Functions for computing the hyperbolic cosine of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s.
pub mod cosh;
/// Functions for computing the hyperbolic cotangent of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod coth;
/// Functions for computing the hyperbolic cosecant of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod csch;
/// Functions for computing the Gudermannian function of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod gudermannian;
/// Functions for computing the inverse Gudermannian function of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod inverse_gudermannian;
/// Functions for computing the hyperbolic secant of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s.
pub mod sech;
/// Functions for computing the hyperbolic sine of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s.
pub mod sinh;
/// Functions for computing the hyperbolic sine and cosine of a
/// [`BigExponentFloat`](super::BigExponentFloat) together, and the implementation shared by the
/// forward hyperbolic functions.
pub mod sinh_cosh;
/// Functions for computing the hyperbolic tangent of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s.
pub mod tanh;
