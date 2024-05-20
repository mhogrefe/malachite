// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993-1995, 1997, 1999, 2000, 2001, 2002, 2012 Free Software Foundation,
//      Inc.
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
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, gets a bit of
// the `Natural` at a specified index. Sufficiently high indices will return `false`.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpz_tstbit` from `mpz/tstbit.c`, GMP 6.2.1, where the input is
// non-negative.
pub_crate_test! {limbs_get_bit(xs: &[Limb], index: u64) -> bool {
    xs.get(usize::exact_from(index >> Limb::LOG_WIDTH))
        .map_or(false, |x| x.get_bit(index & Limb::WIDTH_MASK))
}}

fn limbs_set_bit_helper(xs: &mut [Limb], index: u64, limb_index: usize) {
    xs[limb_index].set_bit(index & Limb::WIDTH_MASK);
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, sets a bit of
// the `Natural` at a specified index to `true`. Indices that are outside the bounds of the slice
// will cause a panic.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `index >= xs.len() * Limb::WIDTH`.
//
// This is equivalent to `mpz_setbit` from `mpz/setbit.c`, GMP 6.2.1, where `d` is non-negative and
// `bit_idx` small enough that no additional memory needs to be given to `d`.
pub_crate_test! {limbs_slice_set_bit(xs: &mut [Limb], index: u64) {
    limbs_set_bit_helper(xs, index, usize::exact_from(index >> Limb::LOG_WIDTH));
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, sets a bit of
// the `Natural` at a specified index to `true`. Sufficiently high indices will increase the length
// of the limbs vector.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
//
// This is equivalent to `mpz_setbit` from `mpz/setbit.c`, GMP 6.2.1, where `d` is non-negative.
pub_test! {limbs_vec_set_bit(xs: &mut Vec<Limb>, index: u64) {
    let small_index = usize::exact_from(index >> Limb::LOG_WIDTH);
    if small_index >= xs.len() {
        xs.resize(small_index + 1, 0);
    }
    limbs_set_bit_helper(xs, index, small_index);
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, sets a bit of
// the `Natural` at a specified index to `false`. Indices that are outside the bounds of the slice
// will result in no action being taken, since there are infinitely many leading zeros.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpz_clrbit` from `mpz/clrbit.c`, GMP 6.2.1, where `d` is non-negative.
pub_crate_test! {limbs_clear_bit(xs: &mut [Limb], index: u64) {
    let small_index = usize::exact_from(index >> Limb::LOG_WIDTH);
    if small_index < xs.len() {
        xs[small_index].clear_bit(index & Limb::WIDTH_MASK);
    }
}}

/// Provides functions for accessing and modifying the $i$th bit of a [`Natural`], or the
/// coefficient of $2^i$ in its binary expansion.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::logic::traits::BitAccess;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::ZERO;
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x, 100);
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x, 0);
///
/// let mut x = Natural::ZERO;
/// x.flip_bit(10);
/// assert_eq!(x, 1024);
/// x.flip_bit(10);
/// assert_eq!(x, 0);
/// ```
impl BitAccess for Natural {
    /// Determines whether the $i$th bit of a [`Natural`], or the coefficient of $2^i$ in its binary
    /// expansion, is 0 or 1.
    ///
    /// `false` means 0 and `true` means 1. Getting bits beyond the [`Natural`]'s width is allowed;
    /// those bits are `false`.
    ///
    /// Let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the rest are
    /// 0. Then $f(n, j) = (b_j = 1)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).get_bit(2), false);
    /// assert_eq!(Natural::from(123u32).get_bit(3), true);
    /// assert_eq!(Natural::from(123u32).get_bit(100), false);
    /// assert_eq!(Natural::from(10u32).pow(12).get_bit(12), true);
    /// assert_eq!(Natural::from(10u32).pow(12).get_bit(100), false);
    /// ```
    fn get_bit(&self, index: u64) -> bool {
        match *self {
            Natural(Small(small)) => small.get_bit(index),
            Natural(Large(ref limbs)) => limbs_get_bit(limbs, index),
        }
    }

    /// Sets the $i$th bit of a [`Natural`], or the coefficient of $2^i$ in its binary expansion, to
    /// 1.
    ///
    /// Let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the rest are
    /// 0. Then
    /// $$
    /// n \gets \\begin{cases}
    ///     n + 2^j & \text{if} \\quad b_j = 0, \\\\
    ///     n & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x, 100);
    /// ```
    fn set_bit(&mut self, index: u64) {
        match self {
            Natural(Small(ref mut small)) => {
                if index < Limb::WIDTH {
                    let mut modified = *small;
                    modified.set_bit(index);
                    *small = modified;
                } else {
                    let mut limbs = vec![*small];
                    limbs_vec_set_bit(&mut limbs, index);
                    *self = Natural(Large(limbs));
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_set_bit(limbs, index);
            }
        }
    }

    /// Sets the $i$th bit of a [`Natural`], or the coefficient of $2^i$ in its binary expansion, to
    /// 0.
    ///
    /// Clearing bits beyond the [`Natural`]'s width is allowed; since those bits are already
    /// `false`, clearing them does nothing.
    ///
    /// Let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the rest are
    /// 0. Then
    /// $$
    /// n \gets \\begin{cases}
    ///     n - 2^j & \text{if} \\quad b_j = 1, \\\\
    ///     n & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(0x7fu32);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x, 100);
    /// ```
    fn clear_bit(&mut self, index: u64) {
        match self {
            Natural(Small(ref mut small)) => small.clear_bit(index),
            Natural(Large(ref mut limbs)) => {
                limbs_clear_bit(limbs, index);
                self.trim();
            }
        }
    }
}
