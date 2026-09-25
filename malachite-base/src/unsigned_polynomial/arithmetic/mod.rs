// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`EvaluateModPowerOf2`](crate::polynomial::EvaluateModPowerOf2), which
/// evaluates a polynomial at a value modulo a power of 2.
pub mod evaluate;
/// An implementation of [`Height`](crate::num::arithmetic::traits::Height), the largest of a
/// polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](crate::num::arithmetic::traits::IsUnit), a trait for determining
/// whether a number is a unit of its ring.
pub mod is_unit;
/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), which
/// checks whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`Mod`](crate::num::arithmetic::traits::Mod),
/// [`ModAssign`](crate::num::arithmetic::traits::ModAssign), [`Rem`](core::ops::Rem) and
/// [`RemAssign`](core::ops::RemAssign), which reduce every coefficient of a polynomial modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](crate::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::arithmetic::traits::ModPowerOf2Assign), which reduce every
/// coefficient of a polynomial modulo a power of 2.
pub mod mod_power_of_2;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), which checks
/// whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
