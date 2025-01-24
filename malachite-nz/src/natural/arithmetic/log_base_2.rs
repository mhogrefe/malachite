// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;

// Given the limbs of a `Natural`, returns the floor of its base-2 logarithm.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// $f((d_i)_ {i=0}^k) = \lfloor\log_2 x\rfloor$, where $x = \sum_{i=0}^kB^id_i$ and $B$ is one more
// than `Limb::MAX`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_floor_log_base_2(xs: &[Limb]) -> u64 {
    limbs_significant_bits(xs) - 1
}}

// Given the limbs of a `Natural`, returns the ceiling of its base-2 logarithm.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// $f((d_i)_ {i=0}^k) = \lceil\log_2 x\rceil$, where $x = \sum_{i=0}^kB^id_i$ and $B$ is one more
// than `Limb::MAX`.
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
pub_test! {limbs_ceiling_log_base_2(xs: &[Limb]) -> u64 {
    let floor_log_base_2 = limbs_floor_log_base_2(xs);
    if limbs_is_power_of_2(xs) {
        floor_log_base_2
    } else {
        floor_log_base_2 + 1
    }
}}

// Given the limbs of a `Natural`, returns the its base-2 logarithm. If the `Natural` is not a power
// of 2, returns `None`.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// $$
// f((d_i)_ {i=0}^k) = \\begin{cases}
//     \operatorname{Some}(\log_2 x) & \text{if} \\quad \log_2 x \in \Z, \\\\
//     \operatorname{None} & \textrm{otherwise}.
// \\end{cases}
// $$
// where $x = \sum_{i=0}^kB^id_i$ and $B$ is one more than `Limb::MAX`.
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
pub_test! {limbs_checked_log_base_2(xs: &[Limb]) -> Option<u64> {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    if slice_test_zero(xs_init) {
        xs_last
            .checked_log_base_2()
            .map(|log| log + (u64::exact_from(xs_init.len()) << Limb::LOG_WIDTH))
    } else {
        None
    }
}}

impl FloorLogBase2 for &Natural {
    type Output = u64;

    /// Returns the floor of the base-2 logarithm of a positive [`Natural`].
    ///
    /// $f(x) = \lfloor\log_2 x\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).floor_log_base_2(), 1);
    /// assert_eq!(Natural::from(100u32).floor_log_base_2(), 6);
    /// ```
    fn floor_log_base_2(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.floor_log_base_2(),
            Natural(Large(ref limbs)) => limbs_floor_log_base_2(limbs),
        }
    }
}

impl CeilingLogBase2 for &Natural {
    type Output = u64;

    /// Returns the ceiling of the base-2 logarithm of a positive [`Natural`].
    ///
    /// $f(x) = \lceil\log_2 x\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).ceiling_log_base_2(), 2);
    /// assert_eq!(Natural::from(100u32).ceiling_log_base_2(), 7);
    /// ```
    fn ceiling_log_base_2(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.ceiling_log_base_2(),
            Natural(Large(ref limbs)) => limbs_ceiling_log_base_2(limbs),
        }
    }
}

impl CheckedLogBase2 for &Natural {
    type Output = u64;

    /// Returns the base-2 logarithm of a positive [`Natural`]. If the [`Natural`] is not a power of
    /// 2, then `None` is returned.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(\log_2 x) & \text{if} \\quad \log_2 x \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).checked_log_base_2(), None);
    /// assert_eq!(Natural::from(4u32).checked_log_base_2(), Some(2));
    /// assert_eq!(
    ///     Natural::from_str("1267650600228229401496703205376")
    ///         .unwrap()
    ///         .checked_log_base_2(),
    ///     Some(100)
    /// );
    /// ```
    fn checked_log_base_2(self) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.checked_log_base_2(),
            Natural(Large(ref limbs)) => limbs_checked_log_base_2(limbs),
        }
    }
}
