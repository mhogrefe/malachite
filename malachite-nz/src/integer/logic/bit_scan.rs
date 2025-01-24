// Copyright © 2025 Mikhail Hogrefe
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

use crate::integer::Integer;
use crate::natural::logic::bit_scan::{
    limbs_index_of_next_false_bit, limbs_index_of_next_true_bit,
};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::*;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitScan, LowMask, TrailingZeros};
use malachite_base::slices::slice_leading_zeros;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, finds the lowest index greater than or equal to `starting_index` at which the
// `Integer` has a `false` bit. If the starting index is too large and there are no more `false`
// bits above it, `None` is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpz_scan0` from `mpz/scan0.c`, GMP 6.2.1.
pub_test! {limbs_index_of_next_false_bit_neg(xs: &[Limb], mut starting_index: u64) -> Option<u64> {
    let n = xs.len();
    let i = slice_leading_zeros(xs);
    assert!(i < n);
    let starting_limb_index = usize::exact_from(starting_index >> Limb::LOG_WIDTH);
    if starting_limb_index >= n {
        return None;
    }
    let after_boundary_offset = (u64::wrapping_from(i) + 1) << Limb::LOG_WIDTH;
    match starting_limb_index.cmp(&i) {
        Equal => {
            let within_limb_index = starting_index & Limb::WIDTH_MASK;
            if let Some(result) = xs[i]
                .wrapping_neg()
                .index_of_next_false_bit(within_limb_index)
            {
                if result < Limb::WIDTH {
                    return Some((u64::wrapping_from(i) << Limb::LOG_WIDTH) + result);
                }
                starting_index = 0;
            }
        }
        Less => {
            return Some(starting_index);
        }
        Greater => {
            starting_index -= after_boundary_offset;
        }
    }
    limbs_index_of_next_true_bit(&xs[i + 1..], starting_index)
        .map(|result| result + after_boundary_offset)
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, finds the lowest index greater than or equal to `starting_index` at which the
// `Integer` has a `true` bit.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpz_scan1` from `mpz/scan1.c`, GMP 6.2.1.
pub_test! {limbs_index_of_next_true_bit_neg(xs: &[Limb], mut starting_index: u64) -> u64 {
    let n = xs.len();
    let i = slice_leading_zeros(xs);
    assert!(i < n);
    let mut starting_limb_index = usize::exact_from(starting_index >> Limb::LOG_WIDTH);
    if starting_limb_index >= n {
        return starting_index;
    }
    let after_boundary_offset = (u64::wrapping_from(i) + 1) << Limb::LOG_WIDTH;
    if starting_limb_index < i {
        starting_index = u64::wrapping_from(i) << Limb::LOG_WIDTH;
        starting_limb_index = i;
    }
    if starting_limb_index == i {
        let within_limb_index = starting_index & Limb::WIDTH_MASK;
        if let Some(result) = xs[i]
            .wrapping_neg()
            .index_of_next_true_bit(within_limb_index)
        {
            return (u64::wrapping_from(i) << Limb::LOG_WIDTH) + result;
        }
        starting_index = 0;
    } else {
        starting_index -= after_boundary_offset;
    }
    limbs_index_of_next_false_bit(&xs[i + 1..], starting_index) + after_boundary_offset
}}

impl Natural {
    // self != 0
    fn index_of_next_false_bit_neg(&self, starting_index: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => {
                if starting_index >= Limb::WIDTH {
                    None
                } else {
                    let index = TrailingZeros::trailing_zeros(
                        (small - 1) & !Limb::low_mask(starting_index),
                    );
                    if index == Limb::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
            Natural(Large(ref limbs)) => limbs_index_of_next_false_bit_neg(limbs, starting_index),
        }
    }

    // self != 0
    fn index_of_next_true_bit_neg(&self, starting_index: u64) -> u64 {
        match *self {
            Natural(Small(small)) => {
                if starting_index >= Limb::WIDTH {
                    starting_index
                } else {
                    TrailingZeros::trailing_zeros(!((small - 1) | Limb::low_mask(starting_index)))
                }
            }
            Natural(Large(ref limbs)) => limbs_index_of_next_true_bit_neg(limbs, starting_index),
        }
    }
}

impl BitScan for &Integer {
    /// Given an [`Integer`] and a starting index, searches the [`Integer`] for the smallest index
    /// of a `false` bit that is greater than or equal to the starting index.
    ///
    /// If the [`Integer]` is negative, and the starting index is too large and there are no more
    /// `false` bits above it, `None` is returned.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(0),
    ///     Some(0)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(20),
    ///     Some(20)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(31),
    ///     Some(31)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(32),
    ///     Some(34)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(33),
    ///     Some(34)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(34),
    ///     Some(34)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(35),
    ///     None
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_false_bit(100),
    ///     None
    /// );
    /// ```
    fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
        if self.sign {
            self.abs.index_of_next_false_bit(starting_index)
        } else {
            self.abs.index_of_next_false_bit_neg(starting_index)
        }
    }

    /// Given an [`Integer`] and a starting index, searches the [`Integer`] for the smallest index
    /// of a `true` bit that is greater than or equal to the starting index.
    ///
    /// If the [`Integer`] is non-negative, and the starting index is too large and there are no
    /// more `true` bits above it, `None` is returned.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(0),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(20),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(31),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(32),
    ///     Some(32)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(33),
    ///     Some(33)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(34),
    ///     Some(35)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(35),
    ///     Some(35)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(36),
    ///     Some(36)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(0x500000000u64)).index_of_next_true_bit(100),
    ///     Some(100)
    /// );
    /// ```
    fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
        if self.sign {
            self.abs.index_of_next_true_bit(starting_index)
        } else {
            Some(self.abs.index_of_next_true_bit_neg(starting_index))
        }
    }
}
