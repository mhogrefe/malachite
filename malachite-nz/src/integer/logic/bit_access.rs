// Copyright © 2025 Mikhail Hogrefe
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

use crate::integer::Integer;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{PowerOf2, WrappingAddAssign, WrappingNegAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::slices::{slice_leading_zeros, slice_test_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, performs an
// action equivalent to taking the two's complement of the limbs and getting the bit at the
// specified index. Sufficiently high indices will return `true`. The slice cannot be empty or
// contain only zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpz_tstbit` from `mpz/tstbit.c`, GMP 6.2.1, where `d` is negative.
pub_test! {limbs_get_bit_neg(xs: &[Limb], index: u64) -> bool {
    let x_i = usize::exact_from(index >> Limb::LOG_WIDTH);
    if x_i >= xs.len() {
        // We're indexing into the infinite suffix of 1s
        true
    } else {
        let x = if slice_test_zero(&xs[..x_i]) {
            xs[x_i].wrapping_neg()
        } else {
            !xs[x_i]
        };
        x.get_bit(index & Limb::WIDTH_MASK)
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, performs an
// action equivalent to taking the two's complement of the limbs, setting a bit at the specified
// index to `true`, and taking the two's complement again. Indices that are outside the bounds of
// the slice will result in no action being taken, since negative numbers in two's complement have
// infinitely many leading 1s. The slice cannot be empty or contain only zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
//
// # Panics
// If the slice contains only zeros a panic may occur.
//
// This is equivalent to `mpz_setbit` from `mpz/setbit.c`, GMP 6.2.1, where `d` is negative.
pub_test! {limbs_set_bit_neg(xs: &mut [Limb], index: u64) {
    let x_i = usize::exact_from(index >> Limb::LOG_WIDTH);
    if x_i >= xs.len() {
        return;
    }
    let reduced_index = index & Limb::WIDTH_MASK;
    let zero_bound = slice_leading_zeros(xs);
    match x_i.cmp(&zero_bound) {
        Equal => {
            let boundary = &mut xs[x_i];
            // boundary != 0 here
            *boundary -= 1;
            boundary.clear_bit(reduced_index);
            // boundary != Limb::MAX here
            *boundary += 1;
        }
        Less => {
            assert!(!limbs_sub_limb_in_place(
                &mut xs[x_i..],
                Limb::power_of_2(reduced_index),
            ));
        }
        Greater => {
            xs[x_i].clear_bit(reduced_index);
        }
    }
}}

fn limbs_clear_bit_neg_helper(xs: &mut [Limb], x_i: usize, reduced_index: u64) -> bool {
    let zero_bound = slice_leading_zeros(xs);
    match x_i.cmp(&zero_bound) {
        Equal => {
            // xs[x_i] != 0 here
            let mut boundary = xs[x_i] - 1;
            boundary.set_bit(reduced_index);
            boundary.wrapping_add_assign(1);
            xs[x_i] = boundary;
            boundary == 0 && limbs_slice_add_limb_in_place(&mut xs[x_i + 1..], 1)
        }
        Greater => {
            xs[x_i].set_bit(reduced_index);
            false
        }
        _ => false,
    }
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, performs an
// action equivalent to taking the two's complement of the limbs, setting a bit at the specified
// index to `false`, and taking the two's complement again. Inputs that would result in new `true`
// bits outside of the slice will cause a panic. The slice cannot be empty or contain only zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
//
// # Panics
// Panics if evaluation would require new `true` bits outside of the slice. If the slice contains
// only zeros a panic may occur.
//
// This is equivalent to `mpz_clrbit` from `mpz/clrbit.c`, GMP 6.2.1, where `d` is negative and
// `bit_idx` small enough that no additional memory needs to be given to `d`.
pub fn limbs_slice_clear_bit_neg(xs: &mut [Limb], index: u64) {
    let x_i = usize::exact_from(index >> Limb::LOG_WIDTH);
    let reduced_index = index & Limb::WIDTH_MASK;
    assert!(
        x_i < xs.len() && !limbs_clear_bit_neg_helper(xs, x_i, reduced_index),
        "Setting bit cannot be done within existing slice"
    );
}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, performs an
// action equivalent to taking the two's complement of the limbs, setting a bit at the specified
// index to `false`, and taking the two's complement again. Sufficiently high indices will increase
// the length of the limbs vector. The slice cannot be empty or contain only zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
//
// # Panics
// If the slice contains only zeros a panic may occur.
//
// This is equivalent to `mpz_clrbit` from `mpz/clrbit.c`, GMP 6.2.1, where `d` is negative.
pub_test! {limbs_vec_clear_bit_neg(xs: &mut Vec<Limb>, index: u64) {
    let x_i = usize::exact_from(index >> Limb::LOG_WIDTH);
    let reduced_index = index & Limb::WIDTH_MASK;
    if x_i < xs.len() {
        if limbs_clear_bit_neg_helper(xs, x_i, reduced_index) {
            xs.push(1);
        }
    } else {
        xs.resize(x_i, 0);
        xs.push(Limb::power_of_2(reduced_index));
    }
}}

impl Natural {
    // self cannot be zero
    pub(crate) fn get_bit_neg(&self, index: u64) -> bool {
        match *self {
            Natural(Small(small)) => index >= Limb::WIDTH || small.wrapping_neg().get_bit(index),
            Natural(Large(ref limbs)) => limbs_get_bit_neg(limbs, index),
        }
    }

    // self cannot be zero
    fn set_bit_neg(&mut self, index: u64) {
        match *self {
            Natural(Small(ref mut small)) => {
                if index < Limb::WIDTH {
                    small.wrapping_neg_assign();
                    small.set_bit(index);
                    small.wrapping_neg_assign();
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_set_bit_neg(limbs, index);
                self.trim();
            }
        }
    }

    // self cannot be zero
    fn clear_bit_neg(&mut self, index: u64) {
        match *self {
            Natural(Small(ref mut small)) if index < Limb::WIDTH => {
                let mut cleared_small = small.wrapping_neg();
                cleared_small.clear_bit(index);
                if cleared_small == 0 {
                    *self = Natural(Large(vec![0, 1]));
                } else {
                    *small = cleared_small.wrapping_neg();
                }
            }
            Natural(Small(_)) => {
                let limbs = self.promote_in_place();
                limbs_vec_clear_bit_neg(limbs, index);
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_clear_bit_neg(limbs, index);
            }
        }
    }
}

/// Provides functions for accessing and modifying the $i$th bit of a [`Integer`], or the
/// coefficient of $2^i$ in its two's complement binary expansion.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::logic::traits::BitAccess;
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::ZERO;
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x, 100);
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x, 0);
///
/// let mut x = Integer::from(-0x100);
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x, -156);
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x, -256);
///
/// let mut x = Integer::ZERO;
/// x.flip_bit(10);
/// assert_eq!(x, 1024);
/// x.flip_bit(10);
/// assert_eq!(x, 0);
///
/// let mut x = Integer::NEGATIVE_ONE;
/// x.flip_bit(10);
/// assert_eq!(x, -1025);
/// x.flip_bit(10);
/// assert_eq!(x, -1);
/// ```
impl BitAccess for Integer {
    /// Determines whether the $i$th bit of an [`Integer`], or the coefficient of $2^i$ in its two's
    /// complement binary expansion, is 0 or 1.
    ///
    /// `false` means 0 and `true` means 1. Getting bits beyond the [`Integer`]'s width is allowed;
    /// those bits are `false` if the [`Integer`] is non-negative and `true` if it is negative.
    ///
    /// If $n \geq 0$, let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i};
    /// $$
    /// but if $n < 0$, let
    /// $$
    /// -n - 1 = \sum_{i=0}^\infty 2^{1 - b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$.
    ///
    /// $f(n, i) = (b_i = 1)$.
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
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).get_bit(2), false);
    /// assert_eq!(Integer::from(123).get_bit(3), true);
    /// assert_eq!(Integer::from(123).get_bit(100), false);
    /// assert_eq!(Integer::from(-123).get_bit(0), true);
    /// assert_eq!(Integer::from(-123).get_bit(1), false);
    /// assert_eq!(Integer::from(-123).get_bit(100), true);
    /// assert_eq!(Integer::from(10u32).pow(12).get_bit(12), true);
    /// assert_eq!(Integer::from(10u32).pow(12).get_bit(100), false);
    /// assert_eq!((-Integer::from(10u32).pow(12)).get_bit(12), true);
    /// assert_eq!((-Integer::from(10u32).pow(12)).get_bit(100), true);
    /// ```
    fn get_bit(&self, index: u64) -> bool {
        match *self {
            Integer {
                sign: true,
                ref abs,
            } => abs.get_bit(index),
            Integer {
                sign: false,
                ref abs,
            } => abs.get_bit_neg(index),
        }
    }

    /// Sets the $i$th bit of an [`Integer`], or the coefficient of $2^i$ in its two's complement
    /// binary expansion, to 1.
    ///
    /// If $n \geq 0$, let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i};
    /// $$
    /// but if $n < 0$, let
    /// $$
    /// -n - 1 = \sum_{i=0}^\infty 2^{1 - b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x, 100);
    ///
    /// let mut x = Integer::from(-0x100);
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x, -156);
    /// ```
    fn set_bit(&mut self, index: u64) {
        match *self {
            Integer {
                sign: true,
                ref mut abs,
            } => abs.set_bit(index),
            Integer {
                sign: false,
                ref mut abs,
            } => abs.set_bit_neg(index),
        }
    }

    /// Sets the $i$th bit of an [`Integer`], or the coefficient of $2^i$ in its binary expansion,
    /// to 0.
    ///
    /// If $n \geq 0$, let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i};
    /// $$
    /// but if $n < 0$, let
    /// $$
    /// -n - 1 = \sum_{i=0}^\infty 2^{1 - b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$.
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
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(0x7f);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x, 100);
    ///
    /// let mut x = Integer::from(-156);
    /// x.clear_bit(2);
    /// x.clear_bit(5);
    /// x.clear_bit(6);
    /// assert_eq!(x, -256);
    /// ```
    fn clear_bit(&mut self, index: u64) {
        match *self {
            Integer {
                sign: true,
                ref mut abs,
            } => abs.clear_bit(index),
            Integer {
                sign: false,
                ref mut abs,
            } => abs.clear_bit_neg(index),
        }
    }
}
