// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::CoprimeWith;
use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

/// A rational number.
///
/// `Rational`s whose numerator and denominator have 64 significant bits or fewer can be represented
/// without any memory allocation. (Unless Malachite is compiled with `32_bit_limbs`, in which case
/// the limit is 32).
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Rational {
    // whether the `Rational` is non-negative
    #[cfg_attr(feature = "serde", serde(rename = "s"))]
    pub(crate) sign: bool,
    #[cfg_attr(feature = "serde", serde(rename = "n"))]
    pub(crate) numerator: Natural,
    #[cfg_attr(feature = "serde", serde(rename = "d"))]
    pub(crate) denominator: Natural,
}

impl Rational {
    // Returns true iff `self` is valid.
    //
    // To be valid, its denominator must be nonzero, its numerator and denominator must be
    // relatively prime, and if its numerator is zero, then `sign` must be `true`. All `Rational`s
    // must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.denominator != 0
            && (self.sign || self.numerator != 0)
            && (&self.numerator).coprime_with(&self.denominator)
    }
}

impl SignificantBits for &Rational {
    /// Returns the sum of the bits needed to represent the numerator and denominator.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.significant_bits(), 1);
    /// assert_eq!(
    ///     Rational::from_str("-100/101").unwrap().significant_bits(),
    ///     14
    /// );
    /// ```
    fn significant_bits(self) -> u64 {
        self.numerator.significant_bits() + self.denominator.significant_bits()
    }
}

/// The constant 0.
impl Zero for Rational {
    const ZERO: Self = Self {
        sign: true,
        numerator: Natural::ZERO,
        denominator: Natural::ONE,
    };
}

/// The constant 1.
impl One for Rational {
    const ONE: Self = Self {
        sign: true,
        numerator: Natural::ONE,
        denominator: Natural::ONE,
    };
}

/// The constant 2.
impl Two for Rational {
    const TWO: Self = Self {
        sign: true,
        numerator: Natural::TWO,
        denominator: Natural::ONE,
    };
}

/// The constant -1.
impl NegativeOne for Rational {
    const NEGATIVE_ONE: Self = Self {
        sign: false,
        numerator: Natural::ONE,
        denominator: Natural::ONE,
    };
}

/// The constant 1/2.
impl OneHalf for Rational {
    const ONE_HALF: Self = Self {
        sign: true,
        numerator: Natural::ONE,
        denominator: Natural::TWO,
    };
}

impl Default for Rational {
    /// The default value of a [`Rational`], 0.
    fn default() -> Self {
        Self::ZERO
    }
}

// Implements `Named` for `Rational`.
impl_named!(Rational);

/// Traits for arithmetic.
pub mod arithmetic;
/// Traits for comparing [`Rational`]s for equality or order.
pub mod comparison;
/// Traits for converting to and from [`Rational`]s, converting to and from strings, and extracting
/// digits and continued fractions.
pub mod conversion;
/// Iterators that generate [`Rational`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Rational`]s randomly.
pub mod random;
