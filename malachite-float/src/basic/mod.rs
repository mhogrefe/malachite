// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Classification of a [`Float`](super::Float) into several kinds.
pub mod classification;
/// Measuring the complexity to a [`Float`](super::Float).
pub mod complexity;
/// Various [`Float`](super::Float) constants. This module contains actual Rust constants like
/// [`One`](super::Float#impl-One-for-Float), and functions like [`one`](super::Float::one_prec)
/// which accept a precision.
#[macro_use]
pub mod constants;
/// Getting and setting the components of a [`Float`](super::Float).
pub mod get_and_set;
/// Getting [`Float`](super::Float)'s ulp (unit in the last place).
pub mod ulp;
