// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`ModPowerOf2`](crate::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::arithmetic::traits::ModPowerOf2Assign), which reduce every
/// element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), which checks
/// whether every element of a vector is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
