// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;

/// Functions for converting a [`GaussianInteger`] to and from other types and strings.
pub mod conversion;
/// Iterators that generate [`GaussianInteger`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`GaussianInteger`]s randomly.
pub mod random;

use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, Two, Zero};

/// A Gaussian integer: a complex number whose real and imaginary parts are both integers.
///
/// The fields are public, since every combination of real and imaginary parts is a valid Gaussian
/// integer.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GaussianInteger {
    pub real: Integer,
    pub imaginary: Integer,
}

/// The constant 0.
impl Zero for GaussianInteger {
    const ZERO: Self = Self {
        real: Integer::ZERO,
        imaginary: Integer::ZERO,
    };
}

/// The constant 1.
impl One for GaussianInteger {
    const ONE: Self = Self {
        real: Integer::ONE,
        imaginary: Integer::ZERO,
    };
}

/// The constant 2.
impl Two for GaussianInteger {
    const TWO: Self = Self {
        real: Integer::TWO,
        imaginary: Integer::ZERO,
    };
}

/// The constant -1.
impl NegativeOne for GaussianInteger {
    const NEGATIVE_ONE: Self = Self {
        real: Integer::NEGATIVE_ONE,
        imaginary: Integer::ZERO,
    };
}

/// The constant i.
impl I for GaussianInteger {
    const I: Self = Self {
        real: Integer::ZERO,
        imaginary: Integer::ONE,
    };
}

/// The constant -i.
impl NegativeI for GaussianInteger {
    const NEGATIVE_I: Self = Self {
        real: Integer::ZERO,
        imaginary: Integer::NEGATIVE_ONE,
    };
}
