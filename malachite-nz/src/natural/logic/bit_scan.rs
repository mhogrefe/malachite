// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2000-2002, 2004, 2012, 2015 Free Software Foundation, Inc.
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
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitScan, TrailingZeros};
use malachite_base::slices::slice_leading_zeros;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, finds the
// lowest index greater than or equal to `start` at which the `Natural` has a `false` bit.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_scan0` from `mpn/generic/scan0.c`, GMP 6.2.1.
pub_crate_test! {limbs_index_of_next_false_bit(xs: &[Limb], start: u64) -> u64 {
    let starting_index = usize::exact_from(start >> Limb::LOG_WIDTH);
    if starting_index >= xs.len() {
        return start;
    }
    if let Some(result) = xs[starting_index].index_of_next_false_bit(start & Limb::WIDTH_MASK) {
        if result != Limb::WIDTH {
            return (u64::wrapping_from(starting_index) << Limb::LOG_WIDTH) + result;
        }
    }
    if starting_index == xs.len() - 1 {
        return u64::wrapping_from(xs.len()) << Limb::LOG_WIDTH;
    }
    let false_index = starting_index
        + 1
        + xs[starting_index + 1..]
            .iter()
            .take_while(|&&y| y == Limb::MAX)
            .count();
    let mut result_offset = u64::exact_from(false_index) << Limb::LOG_WIDTH;
    if false_index != xs.len() {
        result_offset += TrailingZeros::trailing_zeros(!xs[false_index]);
    }
    result_offset
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, finds the
// lowest index greater than or equal to `start` at which the `Natural` has a `true` bit. If the
// starting index is too large and there are no more `true` bits above it, `None` is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_scan1` from `mpn/generic/scan1.c`, GMP 6.2.1.
pub_crate_test! {limbs_index_of_next_true_bit(xs: &[Limb], start: u64) -> Option<u64> {
    let starting_index = usize::exact_from(start >> Limb::LOG_WIDTH);
    if starting_index >= xs.len() {
        None
    } else if let Some(result) = xs[starting_index].index_of_next_true_bit(start & Limb::WIDTH_MASK)
    {
        Some((u64::wrapping_from(starting_index) << Limb::LOG_WIDTH) + result)
    } else if starting_index == xs.len() - 1 {
        None
    } else {
        let true_index = starting_index + 1 + slice_leading_zeros(&xs[starting_index + 1..]);
        if true_index == xs.len() {
            None
        } else {
            let result_offset = u64::wrapping_from(true_index) << Limb::LOG_WIDTH;
            Some(
                result_offset
                    .checked_add(TrailingZeros::trailing_zeros(xs[true_index]))
                    .unwrap(),
            )
        }
    }
}}

impl<'a> BitScan for &'a Natural {
    /// Given a [`Natural`] and a starting index, searches the [`Natural`] for the smallest index of
    /// a `false` bit that is greater than or equal to the starting index.
    ///
    /// Since every [`Natural`] has an implicit prefix of infinitely-many zeros, this function
    /// always returns a value.
    ///
    /// Starting beyond the [`Natural`]'s width is allowed; the result is the starting index.
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
    /// use malachite_base::num::logic::traits::BitScan;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(0),
    ///     Some(0)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(20),
    ///     Some(20)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(31),
    ///     Some(31)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(32),
    ///     Some(34)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(33),
    ///     Some(34)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(34),
    ///     Some(34)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(35),
    ///     Some(36)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_false_bit(100),
    ///     Some(100)
    /// );
    /// ```
    fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.index_of_next_false_bit(start),
            Natural(Large(ref limbs)) => Some(limbs_index_of_next_false_bit(limbs, start)),
        }
    }

    /// Given a [`Natural`] and a starting index, searches the [`Natural`] for the smallest index of
    /// a `true` bit that is greater than or equal to the starting index.
    ///
    /// If the starting index is greater than or equal to the [`Natural`]'s width, the result is
    /// `None` since there are no `true` bits past that point.
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
    /// use malachite_base::num::logic::traits::BitScan;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(0),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(20),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(31),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(32),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(33),
    ///     Some(33)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(34),
    ///     Some(35)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(35),
    ///     Some(35)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(36),
    ///     None
    /// );
    /// assert_eq!(
    ///     Natural::from(0xb00000000u64).index_of_next_true_bit(100),
    ///     None
    /// );
    /// ```
    fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.index_of_next_true_bit(start),
            Natural(Large(ref limbs)) => limbs_index_of_next_true_bit(limbs, start),
        }
    }
}
