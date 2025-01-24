// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use crate::integer::Integer;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::Natural;
use crate::platform::{Limb, SignedLimb};
use alloc::vec::Vec;
use itertools::Itertools;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitConvertible, LowMask, NotAssign};

// Given the bits of a non-negative `Integer`, in ascending order, checks whether the most
// significant bit is `false`; if it isn't, appends an extra `false` bit. This way the `Integer`'s
// non-negativity is preserved in its bits.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {bits_to_twos_complement_bits_non_negative(bits: &mut Vec<bool>) {
    if !bits.is_empty() && *bits.last().unwrap() {
        // Sign-extend with an extra false bit to indicate a positive Integer
        bits.push(false);
    }
}}

// Given the bits of the absolute value of a negative `Integer`, in ascending order, converts the
// bits to two's complement. Returns whether there is a carry left over from the two's complement
// conversion process.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits.len()`.
pub_test! {bits_slice_to_twos_complement_bits_negative(bits: &mut [bool]) -> bool {
    let mut true_seen = false;
    for bit in &mut *bits {
        if true_seen {
            bit.not_assign();
        } else if *bit {
            true_seen = true;
        }
    }
    !true_seen
}}

// Given the bits of the absolute value of a negative `Integer`, in ascending order, converts the
// bits to two's complement and checks whether the most significant bit is `true`; if it isn't,
// appends an extra `true` bit. This way the `Integer`'s negativity is preserved in its bits. The
// bits cannot be empty or contain only `false`s.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits.len()`.
//
// # Panics
// Panics if `bits` contains only `false`s.
pub_test! {bits_vec_to_twos_complement_bits_negative(bits: &mut Vec<bool>) {
    assert!(!bits_slice_to_twos_complement_bits_negative(bits));
    if bits.last() == Some(&false) {
        // Sign-extend with an extra true bit to indicate a negative Integer
        bits.push(true);
    }
}}

fn from_bits_helper(mut limbs: Vec<Limb>, sign_bit: bool, last_width: u64) -> Integer {
    if sign_bit {
        if last_width != Limb::WIDTH {
            *limbs.last_mut().unwrap() |= !Limb::low_mask(last_width);
        }
        assert!(!limbs_twos_complement_in_place(&mut limbs));
    }
    Integer::from_sign_and_abs(!sign_bit, Natural::from_owned_limbs_asc(limbs))
}

impl BitConvertible for Integer {
    /// Returns a [`Vec`] containing the twos-complement bits of an [`Integer`] in ascending order:
    /// least- to most-significant.
    ///
    /// The most significant bit indicates the sign; if the bit is `false`, the [`Integer`] is
    /// positive, and if the bit is `true` it is negative. There are no trailing `false` bits if the
    /// [`Integer`] is positive or trailing `true` bits if the [`Integer`] is negative, except as
    /// necessary to include the correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This function is more efficient than [`to_bits_desc`](`Self::to_bits_desc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::ZERO.to_bits_asc().is_empty());
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).to_bits_asc(),
    ///     &[true, false, false, true, false, true, true, false]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).to_bits_asc(),
    ///     &[true, true, true, false, true, false, false, true]
    /// );
    /// ```
    fn to_bits_asc(&self) -> Vec<bool> {
        let mut bits = self.abs.to_bits_asc();
        if self.sign {
            bits_to_twos_complement_bits_non_negative(&mut bits);
        } else {
            bits_vec_to_twos_complement_bits_negative(&mut bits);
        }
        bits
    }

    /// Returns a [`Vec`] containing the twos-complement bits of an [`Integer`] in descending order:
    /// most- to least-significant.
    ///
    /// The most significant bit indicates the sign; if the bit is `false`, the [`Integer`] is
    /// positive, and if the bit is `true` it is negative. There are no leading `false` bits if the
    /// [`Integer`] is positive or leading `true` bits if the [`Integer`] is negative, except as
    /// necessary to include the correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This is similar to how `BigInteger`s in Java are represented.
    ///
    /// This function is less efficient than [`to_bits_asc`](`Self::to_bits_asc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::ZERO.to_bits_desc().is_empty());
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).to_bits_desc(),
    ///     &[false, true, true, false, true, false, false, true]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).to_bits_desc(),
    ///     &[true, false, false, true, false, true, true, true]
    /// );
    /// ```
    fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_bits_asc();
        bits.reverse();
        bits
    }

    /// Converts an iterator of twos-complement bits into an [`Integer`]. The bits should be in
    /// ascending order (least- to most-significant).
    ///
    /// Let $k$ be `bits.count()`. If $k = 0$ or $b_{k-1}$ is `false`, then
    /// $$
    /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^i \[b_i\],
    /// $$
    /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
    ///
    /// If $b_{k-1}$ is `true`, then
    /// $$
    /// f((b_i)_ {i=0}^{k-1}) = \left ( \sum_{i=0}^{k-1}2^i \[b_i\] \right ) - 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::empty;
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_bits_asc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Integer::from_bits_asc(
    ///         [true, false, false, true, false, true, true, false]
    ///             .iter()
    ///             .cloned()
    ///     ),
    ///     105
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_asc(
    ///         [true, true, true, false, true, false, false, true]
    ///             .iter()
    ///             .cloned()
    ///     ),
    ///     -105
    /// );
    /// ```
    fn from_bits_asc<I: Iterator<Item = bool>>(xs: I) -> Integer {
        let mut limbs = Vec::new();
        let mut last_width = 0;
        let mut last_bit = false;
        for chunk in &xs.chunks(usize::exact_from(Limb::WIDTH)) {
            let mut limb = 0;
            let mut i = 0;
            let mut mask = 1;
            for bit in chunk {
                if bit {
                    limb |= mask;
                }
                mask <<= 1;
                i += 1;
                last_bit = bit;
            }
            last_width = i;
            limbs.push(limb);
        }
        from_bits_helper(limbs, last_bit, last_width)
    }

    /// Converts an iterator of twos-complement bits into an [`Integer`]. The bits should be in
    /// descending order (most- to least-significant).
    ///
    /// If `bits` is empty or $b_0$ is `false`, then
    /// $$
    /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^{k-i-1} \[b_i\],
    /// $$
    /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
    ///
    /// If $b_0$ is `true`, then
    /// $$
    /// f((b_i)_ {i=0}^{k-1}) = \left ( \sum_{i=0}^{k-1}2^{k-i-1} \[b_i\] \right ) - 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::empty;
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_bits_desc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Integer::from_bits_desc(
    ///         [false, true, true, false, true, false, false, true]
    ///             .iter()
    ///             .cloned()
    ///     ),
    ///     105
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_desc(
    ///         [true, false, false, true, false, true, true, true]
    ///             .iter()
    ///             .cloned()
    ///     ),
    ///     -105
    /// );
    /// ```
    fn from_bits_desc<I: Iterator<Item = bool>>(xs: I) -> Integer {
        let mut limbs = Vec::new();
        let mut last_width = 0;
        let mut first_bit = false;
        let mut first = true;
        for chunk in &xs.chunks(usize::exact_from(Limb::WIDTH)) {
            let mut limb = 0;
            let mut i = 0;
            for bit in chunk {
                if first {
                    first_bit = bit;
                    first = false;
                }
                limb <<= 1;
                if bit {
                    limb |= 1;
                }
                i += 1;
            }
            last_width = i;
            limbs.push(limb);
        }
        match limbs.len() {
            0 => Integer::ZERO,
            1 => {
                if first_bit {
                    if last_width != Limb::WIDTH {
                        limbs[0] |= !Limb::low_mask(last_width);
                    }
                    Integer::from(SignedLimb::wrapping_from(limbs[0]))
                } else {
                    Integer::from(limbs[0])
                }
            }
            _ => {
                limbs.reverse();
                if last_width != Limb::WIDTH {
                    let smallest_limb = limbs[0];
                    limbs[0] = 0;
                    limbs_slice_shr_in_place(&mut limbs, Limb::WIDTH - last_width);
                    limbs[0] |= smallest_limb;
                }
                from_bits_helper(limbs, first_bit, last_width)
            }
        }
    }
}
