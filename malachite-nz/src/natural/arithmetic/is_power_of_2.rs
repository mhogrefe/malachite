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
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::slices::slice_test_zero;

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
// whether that `Natural` is an integer power of 2.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty.
pub_crate_test! {limbs_is_power_of_2(xs: &[Limb]) -> bool {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    slice_test_zero(xs_init) && xs_last.is_power_of_2()
}}

impl IsPowerOf2 for Natural {
    /// Determines whether a [`Natural`] is an integer power of 2.
    ///
    /// $f(x) = (\exists n \in \Z : 2^n = x)$.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::{IsPowerOf2, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.is_power_of_2(), false);
    /// assert_eq!(Natural::from(123u32).is_power_of_2(), false);
    /// assert_eq!(Natural::from(0x80u32).is_power_of_2(), true);
    /// assert_eq!(Natural::from(10u32).pow(12).is_power_of_2(), false);
    /// assert_eq!(
    ///     Natural::from_str("1099511627776").unwrap().is_power_of_2(),
    ///     true
    /// );
    /// ```
    fn is_power_of_2(&self) -> bool {
        match *self {
            Natural(Small(small)) => small.is_power_of_2(),
            Natural(Large(ref limbs)) => limbs_is_power_of_2(limbs),
        }
    }
}
