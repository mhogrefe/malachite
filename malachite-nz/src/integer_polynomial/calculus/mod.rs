// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Derivative`](malachite_base::polynomial::calculus::traits::Derivative) and
/// [`DerivativeAssign`](malachite_base::polynomial::calculus::traits::DerivativeAssign), for
/// differentiating a polynomial.
pub mod derivative;
/// Implementations of
/// [`NthDerivative`](malachite_base::polynomial::calculus::traits::NthDerivative) and
/// [`NthDerivativeAssign`](malachite_base::polynomial::calculus::traits::NthDerivativeAssign), for
/// differentiating a polynomial any number of times.
pub mod nth_derivative;
