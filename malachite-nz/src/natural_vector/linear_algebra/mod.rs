// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`AreLinearlyIndependent`](
/// malachite_base::num::linear_algebra::traits::AreLinearlyIndependent), a trait for determining
/// whether vectors are linearly independent.
pub mod are_linearly_independent;
/// Implementations of [`ModAreLinearlyIndependent`](
/// malachite_base::num::linear_algebra::traits::ModAreLinearlyIndependent), a trait for determining
/// whether vectors are linearly independent modulo $m$.
pub mod mod_are_linearly_independent;
/// An implementation of [`ModPowerOf2AreLinearlyIndependent`](
/// malachite_base::num::linear_algebra::traits::ModPowerOf2AreLinearlyIndependent), a trait for
/// determining whether vectors are linearly independent modulo $2^k$.
pub mod mod_power_of_2_are_linearly_independent;
