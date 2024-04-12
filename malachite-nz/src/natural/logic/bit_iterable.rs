// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::conversion::to_limbs::LimbIterator;
use crate::natural::Natural;
use crate::platform::Limb;
use core::ops::Index;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitIterable, SignificantBits};

/// A double-ended iterator over the bits of a [`Natural`].
///
/// The forward order is ascending (least-significant first).
///
/// This `struct` is created by [`BitIterable::bits`]; see its documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NaturalBitIterator<'a> {
    pub(crate) significant_bits: u64,
    pub(crate) limbs: LimbIterator<'a>,
    remaining: usize,
    indices_are_in_same_limb: bool,
    current_limb_forward: Limb,
    current_limb_back: Limb,
    // If `n` is nonzero, this mask initially points to the least-significant bit, and is left-
    // shifted by next().
    i_mask: Limb,
    // If `n` is nonzero, this mask initially points to the most-significant nonzero bit, and is
    // right-shifted by next_back().
    j_mask: Limb,
}

impl<'a> Iterator for NaturalBitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.remaining != 0 {
            let bit = self.current_limb_forward & self.i_mask != 0;
            self.i_mask <<= 1;
            if self.i_mask == 0 {
                self.i_mask = 1;
                if let Some(next) = self.limbs.next() {
                    self.current_limb_forward = next;
                } else {
                    self.current_limb_forward = self.current_limb_back;
                    self.indices_are_in_same_limb = true;
                }
            }
            self.remaining -= 1;
            Some(bit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> DoubleEndedIterator for NaturalBitIterator<'a> {
    fn next_back(&mut self) -> Option<bool> {
        if self.remaining != 0 {
            let bit = self.current_limb_back & self.j_mask != 0;
            self.j_mask >>= 1;
            if self.j_mask == 0 {
                self.j_mask = Limb::power_of_2(Limb::WIDTH - 1);
                if let Some(next_back) = self.limbs.next_back() {
                    self.current_limb_back = next_back;
                } else {
                    self.current_limb_back = self.current_limb_forward;
                    self.indices_are_in_same_limb = true;
                }
            }
            self.remaining -= 1;
            Some(bit)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for NaturalBitIterator<'a> {}

impl<'a> Index<u64> for NaturalBitIterator<'a> {
    type Output = bool;

    /// A function to retrieve a [`Natural`]'s bits by index.
    ///
    /// The index is the power of 2 of which the bit is a coefficient. Indexing at or above the
    /// significant bit count returns `false` bits.
    ///
    /// This is equivalent to [`get_bit`](malachite_base::num::logic::traits::BitAccess::get_bit).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.bits()[0], false);
    ///
    /// // 105 = 1101001b
    /// let n = Natural::from(105u32);
    /// let bits = n.bits();
    /// assert_eq!(bits[0], true);
    /// assert_eq!(bits[1], false);
    /// assert_eq!(bits[2], false);
    /// assert_eq!(bits[3], true);
    /// assert_eq!(bits[4], false);
    /// assert_eq!(bits[5], true);
    /// assert_eq!(bits[6], true);
    /// assert_eq!(bits[7], false);
    /// assert_eq!(bits[100], false);
    /// ```
    fn index(&self, index: u64) -> &bool {
        if self.limbs.n.get_bit(index) {
            &true
        } else {
            &false
        }
    }
}

impl<'a> BitIterable for &'a Natural {
    type BitIterator = NaturalBitIterator<'a>;

    /// Returns a double-ended iterator over the bits of a [`Natural`].
    ///
    /// The forward order is ascending, so that less significant bits appear first. There are no
    /// trailing false bits going forward, or leading falses going backward.
    ///
    /// If it's necessary to get a [`Vec`] of all the bits, consider using
    /// [`to_bits_asc`](malachite_base::num::logic::traits::BitConvertible::to_bits_asc) or
    /// [`to_bits_desc`](malachite_base::num::logic::traits::BitConvertible::to_bits_desc) instead.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.bits().next().is_none());
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from(105u32).bits().collect::<Vec<bool>>(),
    ///     &[true, false, false, true, false, true, true]
    /// );
    ///
    /// assert!(Natural::ZERO.bits().next_back().is_none());
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from(105u32).bits().rev().collect::<Vec<bool>>(),
    ///     &[true, true, false, true, false, false, true]
    /// );
    /// ```
    fn bits(self) -> NaturalBitIterator<'a> {
        let significant_bits = self.significant_bits();
        let remainder = significant_bits & Limb::WIDTH_MASK;
        let mut bits = NaturalBitIterator {
            significant_bits,
            limbs: self.limbs(),
            remaining: usize::exact_from(significant_bits),
            indices_are_in_same_limb: significant_bits <= Limb::WIDTH,
            current_limb_forward: 0,
            current_limb_back: 0,
            i_mask: 1,
            j_mask: if remainder != 0 {
                Limb::power_of_2(remainder - 1)
            } else {
                Limb::power_of_2(Limb::WIDTH - 1)
            },
        };
        if let Some(next) = bits.limbs.next() {
            bits.current_limb_forward = next;
        }
        if let Some(next_back) = bits.limbs.next_back() {
            bits.current_limb_back = next_back;
        } else {
            bits.current_limb_back = bits.current_limb_forward;
        }
        bits
    }
}
