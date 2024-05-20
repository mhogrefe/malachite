// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mod_power_of_2::limbs_vec_mod_power_of_2_in_place;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::logic::not::limbs_not_in_place;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_set_zero;
use malachite_base::vecs::vec_delete_left;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs obtained by taking a slice of bits beginning at index `start` of the input slice and ending
// at index `end - 1`. `start` must be less than or equal to `end`, but apart from that there are no
// restrictions on the index values. If they index beyond the physical size of the input limbs, the
// function interprets them as pointing to `false` bits.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `start > end`.
pub_crate_test! {limbs_slice_get_bits(xs: &[Limb], start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let small_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = xs.len();
    if small_start >= len {
        return Vec::new();
    }
    let small_end = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    let mut out = (if small_end >= len {
        &xs[small_start..]
    } else {
        &xs[small_start..small_end]
    })
    .to_vec();
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut out, offset);
    }
    limbs_vec_mod_power_of_2_in_place(&mut out, end - start);
    out
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs obtained by taking a slice of bits beginning at index `start` of the input slice and ending
// at index `end - 1`. `start` must be less than or equal to `end`, but apart from that there are no
// restrictions on the index values. If they index beyond the physical size of the input limbs, the
// function interprets them as pointing to `false` bits.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `start > end`.
pub_test! {limbs_vec_get_bits(mut xs: Vec<Limb>, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let small_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    if small_start >= xs.len() {
        return Vec::new();
    }
    limbs_vec_mod_power_of_2_in_place(&mut xs, end);
    vec_delete_left(&mut xs, small_start);
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut xs, offset);
    }
    xs
}}

// Copy values from `ys` into `xs`.
//
// - If `ys` has the same length as `xs`, the usual copy is performed.
// - If `ys` is longer than `xs`, the first `xs.len()` limbs of `ys` are copied.
// - If `ys` is shorter than `xs`, `ys` is copied and the remaining bits of `xs` are filled with
//   zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn copy_from_diff_len_slice(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len <= ys_len {
        xs.copy_from_slice(&ys[..xs_len]);
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
        xs_lo.copy_from_slice(ys);
        slice_set_zero(xs_hi);
    }
}

pub(crate) fn limbs_assign_bits_helper(
    xs: &mut Vec<Limb>,
    start: u64,
    end: u64,
    mut bits: &[Limb],
    invert: bool,
) {
    let small_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    let small_end = usize::exact_from((end - 1) >> Limb::LOG_WIDTH) + 1;
    let width = usize::exact_from((end - start).shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if width < bits.len() {
        bits = &bits[..width];
    }
    let start_remainder = start & Limb::WIDTH_MASK;
    let end_remainder = end & Limb::WIDTH_MASK;
    if small_end > xs.len() {
        // Possible inefficiency here: we might write many zeros only to delete them later.
        xs.resize(small_end, 0);
    }
    let out = &mut xs[small_start..small_end];
    assert!(!out.is_empty());
    let original_first = out[0];
    let original_last = *out.last().unwrap();
    copy_from_diff_len_slice(out, bits);
    if invert {
        limbs_not_in_place(out);
    }
    if start_remainder != 0 {
        limbs_slice_shl_in_place(out, start_remainder);
        out[0] |= original_first.mod_power_of_2(start_remainder);
    }
    if end_remainder != 0 {
        out.last_mut().unwrap().assign_bits(
            end_remainder,
            Limb::WIDTH,
            &(original_last >> end_remainder),
        );
    }
}

// Writes the limbs of `bits` into the limbs of `xs`, starting at bit `start` of `xs` (inclusive)
// and ending at bit `end` of `xs` (exclusive). The bit indices do not need to be aligned with any
// limb boundaries. If `bits` has more than `end` - `start` bits, only the first `end` - `start`
// bits are written. If `bits` has fewer than `end` - `start` bits, the remaining written bits are
// zero. `xs` may be extended to accommodate the new bits. `start` must be smaller than `end`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `end`.
//
// # Panics
// Panics if `start >= end`.
pub_test! {limbs_assign_bits(xs: &mut Vec<Limb>, start: u64, end: u64, bits: &[Limb]) {
    assert!(start < end);
    limbs_assign_bits_helper(xs, start, end, bits, false);
}}

impl BitBlockAccess for Natural {
    type Bits = Natural;

    /// Extracts a block of adjacent bits from a [`Natural`], taking the [`Natural`] by reference.
    ///
    /// The first index is `start` and last index is `end - 1`.
    ///
    /// Let $n$ be `self`, and let $p$ and $q$ be `start` and `end`, respectively.
    ///
    /// Let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the rest are
    /// 0. Then
    /// $$
    /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits(16, 48),
    ///     0xef011234u32
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits(4, 16),
    ///     0x567u32
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits(0, 100),
    ///     0xabcdef0112345678u64
    /// );
    /// assert_eq!(Natural::from(0xabcdef0112345678u64).get_bits(10, 10), 0);
    /// ```
    fn get_bits(&self, start: u64, end: u64) -> Natural {
        match *self {
            Natural(Small(small)) => Natural(Small(small.get_bits(start, end))),
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_slice_get_bits(limbs, start, end))
            }
        }
    }

    /// Extracts a block of adjacent bits from a [`Natural`], taking the [`Natural`] by value.
    ///
    /// The first index is `start` and last index is `end - 1`.
    ///
    /// Let $n$ be `self`, and let $p$ and $q$ be `start` and `end`, respectively.
    ///
    /// Let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the rest are
    /// 0. Then
    /// $$
    /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
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
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits_owned(16, 48),
    ///     0xef011234u32
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits_owned(4, 16),
    ///     0x567u32
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits_owned(0, 100),
    ///     0xabcdef0112345678u64
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcdef0112345678u64).get_bits_owned(10, 10),
    ///     0
    /// );
    /// ```
    fn get_bits_owned(self, start: u64, end: u64) -> Natural {
        match self {
            Natural(Small(small)) => Natural(Small(small.get_bits(start, end))),
            Natural(Large(limbs)) => {
                Natural::from_owned_limbs_asc(limbs_vec_get_bits(limbs, start, end))
            }
        }
    }

    /// Replaces a block of adjacent bits in a [`Natural`] with other bits.
    ///
    /// The least-significant `end - start` bits of `bits` are assigned to bits `start` through `end
    /// - 1`, inclusive, of `self`.
    ///
    /// Let $n$ be `self` and let $m$ be `bits`, and let $p$ and $q$ be `start` and `end`,
    /// respectively.
    ///
    /// If `bits` has fewer bits than `end - start`, the high bits are interpreted as 0. Let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the rest are
    /// 0. Let
    /// $$
    /// m = \sum_{i=0}^k 2^{d_i},
    /// $$
    /// where for all $i$, $d_i\in \\{0, 1\\}$. Also, let $p, q \in \mathbb{N}$, and let $W$ be
    /// `max(self.significant_bits(), end + 1)`.
    ///
    /// Then
    /// $$
    /// n \gets \sum_{i=0}^{W-1} 2^{c_i},
    /// $$
    /// where
    /// $$
    /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
    /// \\{b_0, b_1, b_2, \ldots, b_{p-1}, d_0, d_1, \ldots, d_{p-q-1}, b_q, \ldots,
    /// b_ {W-1}\\}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `end`.
    ///
    /// # Panics
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(123u32);
    /// n.assign_bits(5, 7, &Natural::from(456u32));
    /// assert_eq!(n, 27);
    ///
    /// let mut n = Natural::from(123u32);
    /// n.assign_bits(64, 128, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "8411715297611555537019");
    ///
    /// let mut n = Natural::from(123u32);
    /// n.assign_bits(80, 100, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "551270173744270903666016379");
    /// ```
    fn assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if start == end {
            return;
        }
        if let Natural(Small(ref mut small_self)) = self {
            if let Natural(Small(small_bits)) = bits {
                let bits_width = end - start;
                let small_bits = small_bits.mod_power_of_2(bits_width);
                if small_bits == 0 || LeadingZeros::leading_zeros(small_bits) >= start {
                    small_self.assign_bits(start, end, &small_bits);
                    return;
                }
            }
        }
        let limbs = self.promote_in_place();
        match *bits {
            Natural(Small(small_bits)) => limbs_assign_bits(limbs, start, end, &[small_bits]),
            Natural(Large(ref bits_limbs)) => limbs_assign_bits(limbs, start, end, bits_limbs),
        }
        self.trim();
    }
}
