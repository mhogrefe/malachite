// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::platform::Limb;
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "doc-images")]
use embed_doc_image::embed_doc_image;
use malachite_base::comparison::traits::Min;
use malachite_base::named::Named;
#[cfg(feature = "float_helpers")]
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::slices::slice_trailing_zeros;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into a [`Limb`](crate#limbs) is represented inline. Only
/// `Natural`s outside this range incur the costs of heap-allocation.
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image("natural-mem-layout", "images/natural-mem-layout.svg")
)]
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeNatural", into = "SerdeNatural")
)]
pub struct Natural(pub(crate) InnerNatural);

// We want to limit the visibility of the `Small` and `Large` constructors to within this crate. To
// do this, we wrap the `InnerNatural` enum in a struct that gets compiled away.
#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) enum InnerNatural {
    Small(Limb),
    Large(Vec<Limb>),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct SerdeNatural(String);

impl Natural {
    // If a `Natural` is `Large` but is small enough to be `Small`, make it `Small`.
    fn demote_if_small(&mut self) {
        if let Self(Large(limbs)) = self {
            match limbs.len() {
                0 => *self = Self::ZERO,
                1 => *self = Self(Small(limbs[0])),
                _ => {}
            }
        }
    }

    // If a `Natural` is `Small`, make it `Large`. Return a reference to the `Limb` vector.
    pub(crate) fn promote_in_place(&mut self) -> &mut Vec<Limb> {
        if let Self(Small(x)) = self {
            *self = Self(Large(vec![*x]));
        }
        if let Self(Large(xs)) = self {
            xs
        } else {
            unreachable!();
        }
    }

    pub(crate) fn trim(&mut self) {
        if let Self(Large(limbs)) = self {
            let trailing_zero_count = slice_trailing_zeros(limbs);
            if trailing_zero_count != 0 {
                let len = limbs.len();
                limbs.truncate(len - trailing_zero_count);
            }
        }
        self.demote_if_small();
    }

    // Returns true iff `self` is valid. To be valid,
    //
    // `self` can only be `Large` when it is at least $2^W$, and cannot have leading zero limbs. All
    // `Natural`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        match self {
            Natural(Small(_)) => true,
            Natural(Large(xs)) => xs.len() > 1 && *xs.last().unwrap() != 0,
        }
    }
}

/// The constant 0.
impl Zero for Natural {
    const ZERO: Self = Self(Small(0));
}

/// The constant 1.
impl One for Natural {
    const ONE: Self = Self(Small(1));
}

/// The constant 2.
impl Two for Natural {
    const TWO: Self = Self(Small(2));
}

/// The minimum value of a [`Natural`], 0.
impl Min for Natural {
    const MIN: Self = Self::ZERO;
}

#[cfg(feature = "float_helpers")]
impl Natural {
    pub const HIGH_BIT: Self = Self(Small(1 << (Limb::WIDTH - 1)));
}

impl Default for Natural {
    /// The default value of a [`Natural`], 0.
    fn default() -> Self {
        Self::ZERO
    }
}

// Implements `Named` for `Natural`.
impl_named!(Natural);

/// Traits for arithmetic.
pub mod arithmetic;
/// Traits for comparing [`Natural`]s for equality or order.
pub mod comparison;
/// Traits for converting to and from [`Natural`]s, converting to and from strings, and extracting
/// digits.
pub mod conversion;
/// Iterators that generate [`Natural`]s without repetition.
pub mod exhaustive;
/// Traits for generating primes, primality testing, and factorization (TODO!)
pub mod factorization;
/// Traits for logic and bit manipulation.
pub mod logic;
#[cfg(feature = "random")]
/// Iterators that generate [`Natural`]s randomly.
pub mod random;
