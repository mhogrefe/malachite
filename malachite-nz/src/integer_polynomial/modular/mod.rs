// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`BalancedMod`](malachite_base::num::modular::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::modular::traits::BalancedModAssign), which reduce
/// every coefficient of a polynomial to the representative closest to zero.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::modular::traits::BalancedModPowerOf2) and
/// [`BalancedModPowerOf2Assign`]( malachite_base::num::modular::traits::BalancedModPowerOf2Assign),
/// which reduce every coefficient of a polynomial to the representative closest to zero modulo a
/// power of 2.
pub mod balanced_mod_power_of_2;
/// An implementation of [`ModEvaluate`](malachite_base::polynomial::modular::traits::ModEvaluate),
/// which evaluates a polynomial at a value modulo a [`u64`].
pub mod mod_evaluate;
/// Implementations of [`Mod`](malachite_base::num::modular::traits::Mod), which reduces every
/// coefficient of a polynomial into $[0, m)$, producing a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial) or an
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial), and of
/// [`Rem`](core::ops::Rem) and [`RemAssign`](core::ops::RemAssign), which keep each remainder's
/// sign.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::modular::traits::ModPowerOf2), which
/// reduces every coefficient of a polynomial into $[0, 2^k)$, producing a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial), and of
/// [`RemPowerOf2`](malachite_base::num::modular::traits::RemPowerOf2) and
/// [`RemPowerOf2Assign`](malachite_base::num::modular::traits::RemPowerOf2Assign), which keep each
/// remainder's sign.
pub mod mod_power_of_2;
