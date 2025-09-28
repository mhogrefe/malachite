// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#![cfg_attr(not(any(feature = "random", feature = "std")), no_std)]

/// This module contains various functions that support the other crates. This includes many
/// numeric traits and their implementation for primitive numeric types, as well as many functions
/// for exhaustively and randomly generating values of many types.
pub mod base {
    pub use malachite_base::*;
}

#[cfg(feature = "naturals_and_integers")]
#[cfg(feature = "rationals")]
#[cfg(feature = "floats")]
pub use malachite_float::Float;
#[cfg(feature = "naturals_and_integers")]
pub use malachite_nz::integer::Integer;
#[cfg(feature = "naturals_and_integers")]
pub use malachite_nz::natural::Natural;
#[cfg(feature = "naturals_and_integers")]
pub use malachite_nz::*;
#[cfg(feature = "naturals_and_integers")]
#[cfg(feature = "rationals")]
/// [`Rational`], a type representing rational numbers with arbitrarily large numerators and
/// denominators.
pub mod rational {
    pub use malachite_q::*;
}
#[cfg(feature = "naturals_and_integers")]
#[cfg(feature = "rationals")]
pub use malachite_q::Rational;

/// Various types and constants dependent on whether Malachite is built using 32-bit limbs or
/// 64-bit limbs. `Limb` is the type such that `Vec`s of limbs are used to represent the bits of a
/// [`Natural`].
pub mod platform {
    #[cfg(feature = "32_bit_limbs")]
    pub use malachite_nz::platform_32::*;
    #[cfg(not(feature = "32_bit_limbs"))]
    pub use malachite_nz::platform_64::*;
}
