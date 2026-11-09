// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[doc(hidden)]
#[macro_use]
pub mod macros;

/// Traits for arithmetic.
pub mod arithmetic;
/// Traits for primitive integers or floats and some of their basic functionality.
pub mod basic;
/// Traits for comparing the absolute values of numbers for equality or order.
pub mod comparison;
/// Traits for converting to and from numbers, converting to and from strings, and extracting
/// digits.
pub mod conversion;
/// Elliptic integrals and related functions, such as the arithmetic-geometric mean.
pub mod elliptic;
/// Iterators that generate numbers without repetition.
pub mod exhaustive;
/// Exponential and logarithmic functions, and integer logarithms.
pub mod exponential;
/// Traits for generating primes, primality testing, and factorization.
pub mod factorization;
/// [`NiceFloat`](float::NiceFloat), a wrapper around primitive floats.
pub mod float;
/// Hyperbolic functions and their inverses.
pub mod hyperbolic;
/// Iterators related to numbers.
pub mod iterators;
/// Linear algebra: determinants, ranks, nullspaces, solving, decompositions, and normal forms.
pub mod linear_algebra;
/// Traits for logic and bit manipulation.
pub mod logic;
/// Modular arithmetic: operations on residues modulo $m$ and $2^k$, reduction, divisibility, and
/// the Chinese remainder theorem.
pub mod modular;
/// Number theory: GCDs, Kronecker symbols, and combinatorial sequences.
pub mod number_theory;
#[cfg(feature = "random")]
/// Iterators that generate numbers randomly.
pub mod random;
/// Trigonometric functions and their inverses.
pub mod trigonometric;
