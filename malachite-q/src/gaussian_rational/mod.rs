// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, OneHalf, Two, Zero};

/// Functions for converting a [`GaussianRational`] to and from other types and strings.
pub mod conversion;
/// Iterators that generate [`GaussianRational`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`GaussianRational`]s randomly.
pub mod random;

/// A Gaussian rational: a complex number whose real and imaginary parts are both rational.
///
/// The fields are public, since every combination of real and imaginary parts is a valid Gaussian
/// rational.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GaussianRational {
    pub real: Rational,
    pub imaginary: Rational,
}

/// The constant 0.
impl Zero for GaussianRational {
    const ZERO: Self = Self {
        real: Rational::ZERO,
        imaginary: Rational::ZERO,
    };
}

/// The constant 1.
impl One for GaussianRational {
    const ONE: Self = Self {
        real: Rational::ONE,
        imaginary: Rational::ZERO,
    };
}

/// The constant 2.
impl Two for GaussianRational {
    const TWO: Self = Self {
        real: Rational::TWO,
        imaginary: Rational::ZERO,
    };
}

/// The constant 1/2.
impl OneHalf for GaussianRational {
    const ONE_HALF: Self = Self {
        real: Rational::ONE_HALF,
        imaginary: Rational::ZERO,
    };
}

/// The constant -1.
impl NegativeOne for GaussianRational {
    const NEGATIVE_ONE: Self = Self {
        real: Rational::NEGATIVE_ONE,
        imaginary: Rational::ZERO,
    };
}

/// The constant i.
impl I for GaussianRational {
    const I: Self = Self {
        real: Rational::ZERO,
        imaginary: Rational::ONE,
    };
}

/// The constant -i.
impl NegativeI for GaussianRational {
    const NEGATIVE_I: Self = Self {
        real: Rational::ZERO,
        imaginary: Rational::NEGATIVE_ONE,
    };
}
