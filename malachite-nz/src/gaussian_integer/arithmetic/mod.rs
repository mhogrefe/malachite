// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`AbsSquared`](malachite_base::num::arithmetic::traits::AbsSquared) and
/// [`AbsSquaredAssign`](malachite_base::num::arithmetic::traits::AbsSquaredAssign), traits for
/// computing the squared absolute value (norm) of a number.
pub mod abs_squared;
/// Addition of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod add;
/// Implementations of [`Conjugate`](malachite_base::num::arithmetic::traits::Conjugate) and
/// [`ConjugateAssign`](malachite_base::num::arithmetic::traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number: the sign of the imaginary part is flipped.
pub mod conjugate;
/// Multiplication of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod mul;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign) for
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger), negating both the real and
/// imaginary parts.
pub mod neg;
/// Subtraction of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod sub;
