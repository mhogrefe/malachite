// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::modular::traits::{CeilingMod, CeilingModAssign, Mod, ModAssign};

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
/// Rational reconstruction: recovering a [`Rational`](crate::Rational) from its residue modulo a
/// [`Natural`](malachite_nz::natural::Natural).
pub mod reconstruct;
