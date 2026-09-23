// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest of the magnitudes
/// of a polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced),
/// which checks whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), which reduce
/// every coefficient of a polynomial modulo a power of 2.
pub mod mod_power_of_2;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), which
/// checks whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
