// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::modular::traits::{
    BalancedMod, BalancedModAssign, BalancedModPowerOf2, CeilingMod, CeilingModAssign,
    CeilingModPowerOf2, CeilingModPowerOf2Assign, DivisibleBy, DivisibleByPowerOf2, EqMod,
    EqModPowerOf2, Mod, ModAssign, ModEuclidean, ModEuclideanAssign, ModPowerOf2,
    ModPowerOf2Assign, RemPowerOf2, RemPowerOf2Assign,
};

/// [`BalancedMod`] and [`BalancedModAssign`], traits for finding the representative of a number
/// modulo another number that is closest to zero.
pub mod balanced_mod;
/// [`BalancedModPowerOf2`] and [`BalancedModPowerOf2Assign`](
/// malachite_base::num::modular::traits::BalancedModPowerOf2Assign), traits for finding the
/// representative of a number modulo $2^k$ that is closest to zero.
pub mod balanced_mod_power_of_2;
/// Implementations of [`BalancedCrt`](malachite_base::num::arithmetic::traits::BalancedCrt), a
/// trait for combining two congruences by the Chinese remainder theorem and returning the
/// representative of smallest absolute value.
pub mod crt;
/// Implementations of [`DivisibleBy`], a trait for determining whether one number is divisible by
/// another.
pub mod divisible_by;
/// Implementations of [`DivisibleByPowerOf2`], a trait for determining whether a number is
/// divisible by $2^k$.
pub mod divisible_by_power_of_2;
/// Implementations of [`EqMod`], a trait for determining whether one number is equal by another
/// modulo a third.
pub mod eq_mod;
/// Implementations of [`EqModPowerOf2`], a trait for determining whether one number is equal to
/// another modulo $2^k$.
pub mod eq_mod_power_of_2;
/// Implementations of [`ModEuclidean`] and [`ModEuclideanAssign`], traits for finding the remainder
/// of two numbers, where the remainder is nonnegative.
pub mod mod_euclidean;
/// Implementations of traits for finding the remainder of two numbers, subject to various rounding
/// rules.
///
/// These are the traits:
///
/// | rounding          | by value or reference      | by mutable reference (assignment)      |
/// |-------------------|----------------------------|----------------------------------------|
/// | towards $-\infty$ | [`Mod`] | [`ModAssign`]       |
/// | towards $\infty$ | [`CeilingMod`] | [`CeilingModAssign`] |
///
/// The [`Rem`](core::ops::Rem) trait in the standard library rounds towards 0.
pub mod mod_op;
/// Implementations of traits for finding the remainder of a number divided by $2^k$, subject to
/// various rounding rules.
///
/// These are the traits:
///
/// | rounding | by value or reference | by mutable reference (assignment) |
/// |----------|-----------------------|-----------------------------------|
/// | towards $-\infty$ | [`ModPowerOf2`] | [`ModPowerOf2Assign`]       |
/// | towards 0 | [`RemPowerOf2`] | [`RemPowerOf2Assign`]       |
/// | towards $\infty$  | [`CeilingModPowerOf2`] | [`CeilingModPowerOf2Assign`] |
pub mod mod_power_of_2;
/// Negation of an [`Integer`](super::Integer). The one-shot balanced multi-modulus Chinese
/// remainder combination, [`Integer::multi_balanced_crt`](super::Integer::multi_balanced_crt).
pub mod multi_crt;
