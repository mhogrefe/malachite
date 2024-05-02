// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::string::String;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};

/// An integer.
///
/// Any `Integer` whose absolute value is small enough to fit into a [`Limb`](crate#limbs) is
/// represented inline. Only integers outside this range incur the costs of heap-allocation.
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeInteger", into = "SerdeInteger")
)]
pub struct Integer {
    // whether the `Integer` is non-negative
    pub(crate) sign: bool,
    pub(crate) abs: Natural,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct SerdeInteger(String);

impl Integer {
    // Returns true iff `self` is valid.
    //
    // To be valid, its absolute value must be valid, and if the absolute value is zero, the sign
    // must be `true`. All `Integer`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.abs.is_valid() && (self.sign || self.abs != 0)
    }
}

macro_rules! integer_zero {
    () => {
        Integer {
            sign: true,
            abs: Natural::ZERO,
        }
    };
}

macro_rules! integer_one {
    () => {
        Integer {
            sign: true,
            abs: Natural::ONE,
        }
    };
}

macro_rules! integer_two {
    () => {
        Integer {
            sign: true,
            abs: Natural::TWO,
        }
    };
}

macro_rules! integer_negative_one {
    () => {
        Integer {
            sign: false,
            abs: Natural::ONE,
        }
    };
}

/// The constant 0.
impl Zero for Integer {
    const ZERO: Integer = integer_zero!();
}

/// The constant 1.
impl One for Integer {
    const ONE: Integer = integer_one!();
}

/// The constant 2.
impl Two for Integer {
    const TWO: Integer = integer_two!();
}

/// The constant -1.
impl NegativeOne for Integer {
    const NEGATIVE_ONE: Integer = integer_negative_one!();
}

impl Default for Integer {
    /// The default value of an [`Integer`], 0.
    fn default() -> Integer {
        Integer::ZERO
    }
}

// Implements `Named` for `Integer`.
impl_named!(Integer);

/// Traits for arithmetic.
pub mod arithmetic;
/// Traits for comparing [`Integer`]s for equality or order.
pub mod comparison;
/// Traits for converting to and from [`Integer`]s, converting to and from strings, and extracting
/// digits.
pub mod conversion;
/// Iterators that generate [`Integer`]s without repetition.
pub mod exhaustive;
/// Traits for logic and bit manipulation.
pub mod logic;
#[cfg(feature = "random")]
/// Iterators that generate [`Integer`]s randomly.
pub mod random;
