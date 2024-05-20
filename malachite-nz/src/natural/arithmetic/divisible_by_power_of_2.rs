// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2001, 2002 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
// whether that `Natural` is divisible by 2 raised to a given power.
//
// This function assumes that `xs` is nonempty and does not only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(pow, xs.len())`.
//
// This is equivalent to `mpz_divisible_2exp_p` from `mpz/divis_2exp.c`, GMP 6.2.1, where `a` is
// non-negative.
pub_crate_test! {limbs_divisible_by_power_of_2(xs: &[Limb], pow: u64) -> bool {
    assert!(!xs.is_empty());
    let zeros = usize::exact_from(pow >> Limb::LOG_WIDTH);
    zeros < xs.len()
        && slice_test_zero(&xs[..zeros])
        && xs[zeros].divisible_by_power_of_2(pow & Limb::WIDTH_MASK)
}}

impl<'a> DivisibleByPowerOf2 for &'a Natural {
    /// Returns whether a [`Natural`] is divisible by $2^k$.
    ///
    /// $f(x, k) = (2^k|x)$.
    ///
    /// $f(x, k) = (\exists n \in \N : \ x = n2^k)$.
    ///
    /// If `self` is 0, the result is always true; otherwise, it is equivalent to
    /// `self.trailing_zeros().unwrap() <= pow`, but more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(pow, self.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by_power_of_2(100), true);
    /// assert_eq!(Natural::from(100u32).divisible_by_power_of_2(2), true);
    /// assert_eq!(Natural::from(100u32).divisible_by_power_of_2(3), false);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).divisible_by_power_of_2(12),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).divisible_by_power_of_2(13),
    ///     false
    /// );
    /// ```
    fn divisible_by_power_of_2(self, pow: u64) -> bool {
        match (self, pow) {
            (_, 0) => true,
            (&Natural(Small(small)), pow) => small.divisible_by_power_of_2(pow),
            (&Natural(Large(ref limbs)), pow) => limbs_divisible_by_power_of_2(limbs, pow),
        }
    }
}
