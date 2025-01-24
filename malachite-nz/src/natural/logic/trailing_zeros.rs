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
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::slice_leading_zeros;

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
// number of trailing zeros in the binary expansion of a `Natural` (equivalently, the multiplicity
// of 2 in its prime factorization). The limbs cannot be empty or all zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` only contains zeros.
pub_crate_test! {limbs_trailing_zeros(xs: &[Limb]) -> u64 {
    let zeros = slice_leading_zeros(xs);
    let remaining_zeros = TrailingZeros::trailing_zeros(xs[zeros]);
    (u64::wrapping_from(zeros) << Limb::LOG_WIDTH) + remaining_zeros
}}

impl Natural {
    /// Returns the number of trailing zeros in the binary expansion of a [`Natural`] (equivalently,
    /// the multiplicity of 2 in its prime factorization), or `None` is the [`Natural`] is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.trailing_zeros(), None);
    /// assert_eq!(Natural::from(3u32).trailing_zeros(), Some(0));
    /// assert_eq!(Natural::from(72u32).trailing_zeros(), Some(3));
    /// assert_eq!(Natural::from(100u32).trailing_zeros(), Some(2));
    /// assert_eq!(Natural::from(10u32).pow(12).trailing_zeros(), Some(12));
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            Natural::ZERO => None,
            Natural(Small(small)) => Some(TrailingZeros::trailing_zeros(small)),
            Natural(Large(ref limbs)) => Some(limbs_trailing_zeros(limbs)),
        }
    }
}
