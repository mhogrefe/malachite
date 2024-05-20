// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use crate::integer::Integer;
use crate::natural::arithmetic::add::limbs_vec_add_limb_in_place;
use crate::natural::arithmetic::mod_power_of_2::limbs_vec_mod_power_of_2_in_place;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::logic::bit_block_access::limbs_assign_bits_helper;
use crate::natural::logic::not::limbs_not_in_place;
use crate::natural::logic::trailing_zeros::limbs_trailing_zeros;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::vecs::vec_delete_left;

// Returns the limbs obtained by taking a slice of bits beginning at index `start` of the negative
// of `limb` and ending at index `end - 1`. `start` must be less than or equal to `end`, but apart
// from that there are no restrictions on the index values. If they index beyond the physical size
// of the input limbs, the function interprets them as pointing to `true` bits. `x` must be
// positive.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `end`.
//
// # Panics
// Panics if `start > end`.
pub_test! {limbs_neg_limb_get_bits(x: Limb, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = TrailingZeros::trailing_zeros(x);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let bit_len = end - start;
    let mut out = if start >= Limb::WIDTH {
        vec![
            Limb::MAX;
            usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, Ceiling).0)
        ]
    } else {
        let mut out = vec![x >> start];
        out.resize(usize::exact_from(end >> Limb::LOG_WIDTH) + 1, 0);
        if trailing_zeros >= start {
            limbs_twos_complement_in_place(&mut out);
        } else {
            limbs_not_in_place(&mut out);
        }
        out
    };
    limbs_vec_mod_power_of_2_in_place(&mut out, bit_len);
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs obtained by taking a slice of bits beginning at index `start` of the negative of the
// `Natural` and ending at index `end - 1`. `start` must be less than or equal to `end`, but apart
// from that there are no restrictions on the index values. If they index beyond the physical size
// of the input limbs, the function interprets them as pointing to `true` bits. The input slice
// cannot only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(xs.len(), end * Limb::WIDTH)`.
//
// # Panics
// Panics if `start > end`.
pub_test! {limbs_slice_neg_get_bits(xs: &[Limb], start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = limbs_trailing_zeros(xs);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let start_i = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = xs.len();
    let bit_len = end - start;
    if start_i >= len {
        let mut out =
            vec![
                Limb::MAX;
                usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, Ceiling).0)
            ];
        limbs_vec_mod_power_of_2_in_place(&mut out, bit_len);
        return out;
    }
    let end_i = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    let mut out = (if end_i >= len {
        &xs[start_i..]
    } else {
        &xs[start_i..end_i]
    })
    .to_vec();
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut out, offset);
    }
    out.resize(end_i - start_i, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut out);
    } else {
        limbs_not_in_place(&mut out);
    }
    limbs_vec_mod_power_of_2_in_place(&mut out, bit_len);
    out
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs obtained by taking a slice of bits beginning at index `start` of the negative of the
// `Natural` and ending at index `end - 1`. `start` must be less than or equal to `end`, but apart
// from that there are no restrictions on the index values. If they index beyond the physical size
// of the input limbs, the function interprets them as pointing to `true` bits. The input slice
// cannot only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(xs.len(), end * Limb::WIDTH)`.
//
// # Panics
// Panics if `start > end`.
pub_test! {limbs_vec_neg_get_bits(mut xs: Vec<Limb>, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = limbs_trailing_zeros(&xs);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let start_i = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = xs.len();
    let bit_len = end - start;
    if start_i >= len {
        xs = vec![
            Limb::MAX;
            usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, Ceiling).0)
        ];
        limbs_vec_mod_power_of_2_in_place(&mut xs, bit_len);
        return xs;
    }
    let end_i = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    xs.truncate(end_i);
    vec_delete_left(&mut xs, start_i);
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut xs, offset);
    }
    xs.resize(end_i - start_i, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut xs);
    } else {
        limbs_not_in_place(&mut xs);
    }
    limbs_vec_mod_power_of_2_in_place(&mut xs, bit_len);
    xs
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural` n, writes the
// limbs of `bits` into the limbs of -n, starting at bit `start` of -n (inclusive) and ending at bit
// `end` of -n (exclusive). The bit indices do not need to be aligned with any limb boundaries. If
// `bits` has more than `end` - `start` bits, only the first `end` - `start` bits are written. If
// `bits` has fewer than `end` - `start` bits, the remaining written bits are one. `xs` may be
// extended to accommodate the new bits. `start` must be smaller than `end`, and `xs` cannot only
// contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(n / 2 ^ Limb::WIDTH, m)`, and $m$ is
// `end`.
//
// # Panics
// Panics if `start >= end` or `xs` only contains zeros.
pub_test! {limbs_neg_assign_bits(xs: &mut Vec<Limb>, start: u64, end: u64, bits: &[Limb]) {
    assert!(start < end);
    assert!(!limbs_sub_limb_in_place(xs, 1));
    limbs_assign_bits_helper(xs, start, end, bits, true);
    limbs_vec_add_limb_in_place(xs, 1);
}}

impl Natural {
    fn neg_get_bits(&self, start: u64, end: u64) -> Natural {
        Natural::from_owned_limbs_asc(match *self {
            Natural(Small(small)) => limbs_neg_limb_get_bits(small, start, end),
            Natural(Large(ref limbs)) => limbs_slice_neg_get_bits(limbs, start, end),
        })
    }

    fn neg_get_bits_owned(self, start: u64, end: u64) -> Natural {
        Natural::from_owned_limbs_asc(match self {
            Natural(Small(small)) => limbs_neg_limb_get_bits(small, start, end),
            Natural(Large(limbs)) => limbs_vec_neg_get_bits(limbs, start, end),
        })
    }

    fn neg_assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if start == end {
            return;
        }
        let bits_width = end - start;
        if bits_width <= Limb::WIDTH {
            if let (&mut Natural(Small(ref mut small_self)), &Natural(Small(small_bits))) =
                (&mut *self, bits)
            {
                let small_bits = (!small_bits).mod_power_of_2(bits_width);
                if small_bits == 0 || LeadingZeros::leading_zeros(small_bits) >= start {
                    let mut new_small_self = *small_self - 1;
                    new_small_self.assign_bits(start, end, &small_bits);
                    let (sum, overflow) = new_small_self.overflowing_add(1);
                    if !overflow {
                        *small_self = sum;
                        return;
                    }
                }
            }
        }
        let limbs = self.promote_in_place();
        match *bits {
            Natural(Small(small_bits)) => limbs_neg_assign_bits(limbs, start, end, &[small_bits]),
            Natural(Large(ref bits_limbs)) => limbs_neg_assign_bits(limbs, start, end, bits_limbs),
        }
        self.trim();
    }
}

impl BitBlockAccess for Integer {
    type Bits = Natural;

    /// Extracts a block of adjacent two's complement bits from an [`Integer`], taking the
    /// [`Integer`] by reference.
    ///
    /// The first index is `start` and last index is `end - 1`.
    ///
    /// Let $n$ be `self`, and let $p$ and $q$ be `start` and `end`, respectively.
    ///
    /// If $n \geq 0$, let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i};
    /// $$
    /// but if $n < 0$, let
    /// $$
    /// -n - 1 = \sum_{i=0}^\infty 2^{1 - b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$. Then
    /// $$
    /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), end)`.
    ///
    /// # Panics
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits(16, 48),
    ///     Natural::from(0x10feedcbu32)
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcdef0112345678u64).get_bits(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits(0, 100),
    ///     Natural::from_str("1267650600215849587758112418184").unwrap()
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcdef0112345678u64).get_bits(10, 10),
    ///     Natural::ZERO
    /// );
    /// ```
    fn get_bits(&self, start: u64, end: u64) -> Natural {
        if self.sign {
            self.abs.get_bits(start, end)
        } else {
            self.abs.neg_get_bits(start, end)
        }
    }

    /// Extracts a block of adjacent two's complement bits from an [`Integer`], taking the
    /// [`Integer`] by value.
    ///
    /// The first index is `start` and last index is `end - 1`.
    ///
    /// Let $n$ be `self`, and let $p$ and $q$ be `start` and `end`, respectively.
    ///
    /// If $n \geq 0$, let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i};
    /// $$
    /// but if $n < 0$, let
    /// $$
    /// -n - 1 = \sum_{i=0}^\infty 2^{1 - b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$. Then
    /// $$
    /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), end)`.
    ///
    /// # Panics
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits_owned(16, 48),
    ///     Natural::from(0x10feedcbu32)
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcdef0112345678u64).get_bits_owned(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits_owned(0, 100),
    ///     Natural::from_str("1267650600215849587758112418184").unwrap()
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcdef0112345678u64).get_bits_owned(10, 10),
    ///     Natural::ZERO
    /// );
    /// ```
    fn get_bits_owned(self, start: u64, end: u64) -> Natural {
        if self.sign {
            self.abs.get_bits_owned(start, end)
        } else {
            self.abs.neg_get_bits_owned(start, end)
        }
    }

    /// Replaces a block of adjacent two's complement bits in an [`Integer`] with other bits.
    ///
    /// The least-significant `end - start` bits of `bits` are assigned to bits `start` through `end
    /// - 1`, inclusive, of `self`.
    ///
    /// Let $n$ be `self` and let $m$ be `bits`, and let $p$ and $q$ be `start` and `end`,
    /// respectively.
    ///
    /// Let
    /// $$
    /// m = \sum_{i=0}^k 2^{d_i},
    /// $$
    /// where for all $i$, $d_i\in \\{0, 1\\}$.
    ///
    /// If $n \geq 0$, let
    /// $$
    /// n = \sum_{i=0}^\infty 2^{b_i};
    /// $$
    /// but if $n < 0$, let
    /// $$
    /// -n - 1 = \sum_{i=0}^\infty 2^{1 - b_i},
    /// $$
    /// where for all $i$, $b_i\in \\{0, 1\\}$. Then
    /// $$
    /// n \gets \sum_{i=0}^\infty 2^{c_i},
    /// $$
    /// where
    /// $$
    /// \\{c_0, c_1, c_2, \ldots \\} =
    /// \\{b_0, b_1, b_2, \ldots, b_{p-1}, d_0, d_1, \ldots, d_{p-q-1}, b_q, b_{q+1}, \ldots \\}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), end)`, and
    /// $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Integer::from(123);
    /// n.assign_bits(5, 7, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "27");
    ///
    /// let mut n = Integer::from(-123);
    /// n.assign_bits(64, 128, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "-340282366920938455033212565746503123067");
    ///
    /// let mut n = Integer::from(-123);
    /// n.assign_bits(80, 100, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "-1267098121128665515963862483067");
    /// ```
    fn assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if self.sign {
            self.abs.assign_bits(start, end, bits);
        } else {
            self.abs.neg_assign_bits(start, end, bits);
        }
    }
}
