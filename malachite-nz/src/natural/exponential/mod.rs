// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for taking the base-$b$ logarithm of a number.
///
/// The traits are [`FloorLogBase`](malachite_base::num::exponential::traits::FloorLogBase),
/// [`CeilingLogBase`](malachite_base::num::exponential::traits::CeilingLogBase), and
/// [`CheckedLogBase`](malachite_base::num::exponential::traits::CheckedLogBase).
pub mod log_base;
/// Implementations of traits for taking the base-2 logarithm of a number.
///
/// The traits are [`FloorLogBase2`](malachite_base::num::exponential::traits::FloorLogBase2),
/// [`CeilingLogBase2`](malachite_base::num::exponential::traits::CeilingLogBase2), and
/// [`CheckedLogBase2`](malachite_base::num::exponential::traits::CheckedLogBase2).
pub mod log_base_2;
/// Implementations of traits for taking the base-$2^k$ logarithm of a number.
///
/// The traits are
/// [`FloorLogBasePowerOf2`](malachite_base::num::exponential::traits::FloorLogBasePowerOf2),
/// [`CeilingLogBasePowerOf2`](malachite_base::num::exponential::traits::CeilingLogBasePowerOf2),
/// and
/// [`CheckedLogBasePowerOf2`](malachite_base::num::exponential::traits::CheckedLogBasePowerOf2).
pub mod log_base_power_of_2;
