// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Assign, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::*;

// Returns the limbs of a `Natural`, where the lowest `bits` bits are set.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
pub_crate_test! {limbs_low_mask(bits: u64) -> Vec<Limb> {
    let len = bits.shr_round(Limb::LOG_WIDTH, Ceiling).0;
    let remaining_bits = bits & Limb::WIDTH_MASK;
    let mut xs = vec![Limb::MAX; usize::exact_from(len)];
    if remaining_bits != 0 {
        xs.last_mut().unwrap().mod_power_of_2_assign(remaining_bits);
    }
    xs
}}

impl LowMask for Natural {
    /// Returns a [`Natural`] whose least significant $b$ bits are `true` and whose other bits are
    /// `false`.
    ///
    /// $f(b) = 2^b - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::LowMask;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::low_mask(0), 0);
    /// assert_eq!(Natural::low_mask(3), 7);
    /// assert_eq!(
    ///     Natural::low_mask(100).to_string(),
    ///     "1267650600228229401496703205375"
    /// );
    /// ```
    fn low_mask(bits: u64) -> Natural {
        if bits <= Limb::WIDTH {
            Natural(Small(Limb::low_mask(bits)))
        } else {
            Natural(Large(limbs_low_mask(bits)))
        }
    }
}
