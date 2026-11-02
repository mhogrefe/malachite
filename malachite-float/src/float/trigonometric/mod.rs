// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Acos`](malachite_base::num::trigonometric::traits::Acos) and
/// [`AcosAssign`](malachite_base::num::trigonometric::traits::AcosAssign), traits for computing the
/// arccosine of [`Float`](super::Float)s.
pub mod acos;
pub mod acot;
pub mod acsc;
/// Implementations of [`Asec`](malachite_base::num::trigonometric::traits::Asec) and
/// [`AsecAssign`](malachite_base::num::trigonometric::traits::AsecAssign), traits for computing the
/// arcsecant of [`Float`](super::Float)s.
pub mod asec;
/// Implementations of [`Asin`](malachite_base::num::trigonometric::traits::Asin) and
/// [`AsinAssign`](malachite_base::num::trigonometric::traits::AsinAssign), traits for computing the
/// arcsine of [`Float`](super::Float)s.
pub mod asin;
/// Implementations of [`Atan`](malachite_base::num::trigonometric::traits::Atan) and
/// [`AtanAssign`](malachite_base::num::trigonometric::traits::AtanAssign), traits for computing the
/// arctangent of [`Float`](super::Float)s.
pub mod atan;
/// Implementations of [`Atan2`](malachite_base::num::trigonometric::traits::Atan2) and
/// [`Atan2Assign`](malachite_base::num::trigonometric::traits::Atan2Assign), traits for computing
/// the angle of a point given by two [`Float`](super::Float) coordinates.
pub mod atan2;
/// Implementations of [`Cos`](malachite_base::num::trigonometric::traits::Cos) and
/// [`CosAssign`](malachite_base::num::trigonometric::traits::CosAssign), traits for computing the
/// cosine of [`Float`](super::Float)s.
pub mod cos;
/// Implementations of [`Cot`](malachite_base::num::trigonometric::traits::Cot) and
/// [`CotAssign`](malachite_base::num::trigonometric::traits::CotAssign), traits for computing the
/// cotangent of [`Float`](super::Float)s.
pub mod cot;
/// Implementations of [`Csc`](malachite_base::num::trigonometric::traits::Csc) and
/// [`CscAssign`](malachite_base::num::trigonometric::traits::CscAssign), traits for computing the
/// cosecant of [`Float`](super::Float)s.
pub mod csc;
/// Implementations of [`Sec`](malachite_base::num::trigonometric::traits::Sec) and
/// [`SecAssign`](malachite_base::num::trigonometric::traits::SecAssign), traits for computing the
/// secant of [`Float`](super::Float)s.
pub mod sec;
/// Implementations of [`Sin`](malachite_base::num::trigonometric::traits::Sin) and
/// [`SinAssign`](malachite_base::num::trigonometric::traits::SinAssign), traits for computing the
/// sine of [`Float`](super::Float)s.
pub mod sin;
/// Implementations of [`SinCos`](malachite_base::num::trigonometric::traits::SinCos) and
/// [`SinCosAssign`](malachite_base::num::trigonometric::traits::SinCosAssign), traits for computing
/// the sine and cosine of [`Float`](super::Float)s together.
pub mod sin_cos;
pub mod tan;
