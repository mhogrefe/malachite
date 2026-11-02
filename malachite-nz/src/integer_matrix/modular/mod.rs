// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`BalancedMod`](malachite_base::num::modular::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::modular::traits::BalancedModAssign), which reduce
/// every element of an [`IntegerMatrix`](super::IntegerMatrix) to the representative closest to
/// zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::modular::traits::BalancedModPowerOf2) and
/// [`BalancedModPowerOf2Assign`]( malachite_base::num::modular::traits::BalancedModPowerOf2Assign),
/// which reduce every element of an [`IntegerMatrix`](super::IntegerMatrix) to the representative
/// closest to zero modulo a power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`BalancedCrt`](malachite_base::num::arithmetic::traits::BalancedCrt), which
/// combine a matrix of residues modulo any number with one modulo a word-sized number into balanced
/// representatives by the Chinese remainder theorem.
pub mod crt;
/// Implementations of [`Mod`](malachite_base::num::modular::traits::Mod), [`Rem`](core::ops::Rem),
/// and [`RemAssign`](core::ops::RemAssign), for reducing every element of an
/// [`IntegerMatrix`](super::IntegerMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::modular::traits::ModPowerOf2),
/// [`RemPowerOf2`](malachite_base::num::modular::traits::RemPowerOf2), and
/// [`RemPowerOf2Assign`](malachite_base::num::modular::traits::RemPowerOf2Assign), for reducing
/// every element of an [`IntegerMatrix`](super::IntegerMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// [`IntegerMatrix::multi_balanced_crt`](super::IntegerMatrix::multi_balanced_crt), which combines
/// matrices of residues modulo many word-sized moduli into balanced representatives by the Chinese
/// remainder theorem.
pub mod multi_crt;
/// [`IntegerMatrix::multi_mod`](super::IntegerMatrix::multi_mod), which reduces a matrix modulo
/// many word-sized moduli.
pub mod multi_mod;
