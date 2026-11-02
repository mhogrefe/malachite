// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use crate::polynomial::calculus::traits::{
    ModDerivative, ModDerivativeAssign, ModIntegral, ModIntegralAssign, ModNthDerivative,
    ModNthDerivativeAssign, ModPowerOf2Derivative, ModPowerOf2DerivativeAssign,
    ModPowerOf2Integral, ModPowerOf2IntegralAssign, ModPowerOf2NthDerivative,
    ModPowerOf2NthDerivativeAssign,
};

/// Implementations of [`ModDerivative`] and [`ModDerivativeAssign`], for differentiating a
/// polynomial modulo a number.
pub mod mod_derivative;
/// Implementations of [`ModIntegral`] and [`ModIntegralAssign`], for integrating a polynomial
/// modulo a value.
pub mod mod_integral;
/// Implementations of [`ModNthDerivative`] and [`ModNthDerivativeAssign`], for differentiating a
/// polynomial any number of times modulo a number.
pub mod mod_nth_derivative;
/// Implementations of [`ModPowerOf2Derivative`] and [`ModPowerOf2DerivativeAssign`], for
/// differentiating a polynomial modulo a power of 2.
pub mod mod_power_of_2_derivative;
/// Implementations of [`ModPowerOf2Integral`] and [`ModPowerOf2IntegralAssign`], for integrating a
/// polynomial modulo $2^k$.
pub mod mod_power_of_2_integral;
/// Implementations of [`ModPowerOf2NthDerivative`] and [`ModPowerOf2NthDerivativeAssign`], for
/// differentiating a polynomial any number of times modulo a power of 2.
pub mod mod_power_of_2_nth_derivative;
