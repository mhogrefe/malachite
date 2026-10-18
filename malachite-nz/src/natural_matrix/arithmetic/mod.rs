// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// matrices of the same shape.
pub mod add;
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod), which
/// reduce every element of a [`NaturalMatrix`](super::NaturalMatrix) to the representative closest
/// to zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::arithmetic::traits::BalancedModPowerOf2), which
/// reduce every element of a [`NaturalMatrix`](super::NaturalMatrix) to the representative closest
/// to zero modulo a power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`ModAdd`](malachite_base::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::arithmetic::traits::ModAddAssign), for adding
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced), a
/// trait for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a number.
pub mod mod_is_reduced;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign), for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), for negating every
/// element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](malachite_base::num::arithmetic::traits::Mod), and
/// [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign), for reducing every element of
/// a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), traits for
/// reducing every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::arithmetic::traits::ModPowerOf2Add)
/// and [`ModPowerOf2AddAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign), for
/// adding [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_add;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), a trait
/// for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign), for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign), for
/// negating every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;

/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign), for
/// subtracting [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;

/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_sub;

/// Implementations of [`Mul`](core::ops::Mul) for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right.
pub mod mul_vector;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a scalar on either side.
pub mod scalar_mul;

/// Implementations of [`Mul`](core::ops::Mul) for multiplying a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) by a
/// [`NaturalMatrix`](super::NaturalMatrix) on the right.
pub mod vector_mul;
