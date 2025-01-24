// Copyright © 2025 Mikhail Hogrefe
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
/// Iterators that generate numbers without repetition.
pub mod exhaustive;
/// Traits for generating primes, primality testing, and factorization (TODO!)
pub mod factorization;
/// [`NiceFloat`](float::NiceFloat), a wrapper around primitive floats.
pub mod float;
/// Iterators related to numbers.
pub mod iterators;
/// Traits for logic and bit manipulation.
pub mod logic;
#[cfg(feature = "random")]
/// Iterators that generate numbers randomly.
pub mod random;
