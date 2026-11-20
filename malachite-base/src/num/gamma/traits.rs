// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Computes the gamma function $\Gamma(x)$ of a number.
pub trait Gamma {
    type Output;

    fn gamma(self) -> Self::Output;
}

/// Replaces a number $x$ with $\Gamma(x)$, its gamma function.
pub trait GammaAssign {
    fn gamma_assign(&mut self);
}

/// Computes the reciprocal gamma function $1/\Gamma(x)$ of a number.
pub trait ReciprocalGamma {
    type Output;

    fn reciprocal_gamma(self) -> Self::Output;
}

/// Replaces a number $x$ with $1/\Gamma(x)$, its reciprocal gamma function.
pub trait ReciprocalGammaAssign {
    fn reciprocal_gamma_assign(&mut self);
}
