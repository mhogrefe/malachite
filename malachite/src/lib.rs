// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! [Malachite](https://www.malachite.rs/) is an arbitrary-precision arithmetic library for Rust,
//! with efficient algorithms partially derived from [GMP](https://gmplib.org/),
//! [FLINT](https://www.flintlib.org/), and [MPFR](https://www.mpfr.org/).
//!
//! This is the main Malachite crate, and the recommended way to use the library. It re-exports the
//! contents of the more specialized crates that make up Malachite, so that depending on `malachite`
//! alone is enough:
//!
//! - [`Natural`] and [`Integer`], arbitrary-precision unsigned and signed integers, are re-exported
//!   from [`malachite-nz`](https://docs.rs/malachite-nz).
//! - [`Rational`], arbitrary-precision rational numbers, is re-exported from
//!   [`malachite-q`](https://docs.rs/malachite-q). (Requires the `rationals` feature, which is
//!   enabled by default.)
//! - `Float`, arbitrary-precision floating-point numbers, is re-exported from
//!   [`malachite-float`](https://docs.rs/malachite-float). (Requires the `floats` feature; floats
//!   are currently experimental.)
//! - The [`base`] module re-exports [`malachite-base`](https://docs.rs/malachite-base), which
//!   provides the numeric traits (and their implementations for primitive types) used throughout
//!   Malachite, along with tools for generating values for tests and benchmarks.
//!
//! Most conversion and arithmetic operations are expressed as traits that live in [`base`], so a
//! common pattern is to import them from there, for example
//! `use malachite::base::num::arithmetic::traits::Pow;`.
//!
//! # Examples
//! ```
//! use malachite::base::num::arithmetic::traits::Pow;
//! use malachite::{Integer, Natural, Rational};
//!
//! // Arbitrary-precision unsigned integers, far larger than any primitive type:
//! assert_eq!(Natural::from(2u32).pow(128), Natural::from(1u32) << 128u64);
//!
//! // Signed integers:
//! assert_eq!(-Integer::from(5) * Integer::from(5), Integer::from(-25));
//!
//! // Exact rational arithmetic: 1/3 + 1/6 = 1/2.
//! assert_eq!(
//!     Rational::from_unsigneds(1u32, 3u32) + Rational::from_unsigneds(1u32, 6u32),
//!     Rational::from_unsigneds(1u32, 2u32)
//! );
//! ```
//!
//! For documentation of individual items, follow the re-export links below.

#![cfg_attr(not(any(feature = "random", feature = "std")), no_std)]
#![forbid(unsafe_code)]

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
