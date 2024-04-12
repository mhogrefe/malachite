// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::logic::bit_iterable::NaturalBitIterator;
use crate::natural::Natural;
use core::ops::Index;
use malachite_base::num::logic::traits::{BitAccess, BitIterable};

/// A double-ended iterator over the two's complement bits of the negative of an [`Integer`].
///
/// The forward order is ascending (least-significant first). There may be at most one implicit
/// most-significant `true` bit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NegativeBitIterator<'a> {
    pub(crate) bits: NaturalBitIterator<'a>,
    i: u64,
    j: u64,
    first_true_index: Option<u64>,
}

impl<'a> Iterator for NegativeBitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let previous_i = self.i;
        self.bits.next().map(|bit| {
            self.i += 1;
            if let Some(first_true_index) = self.first_true_index {
                if previous_i <= first_true_index {
                    bit
                } else {
                    !bit
                }
            } else {
                if bit {
                    self.first_true_index = Some(previous_i);
                }
                bit
            }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.bits.size_hint()
    }
}

impl<'a> DoubleEndedIterator for NegativeBitIterator<'a> {
    fn next_back(&mut self) -> Option<bool> {
        let previous_j = self.j;
        self.bits.next_back().map(|bit| {
            if self.j != 0 {
                self.j -= 1;
            }
            if self.first_true_index.is_none() {
                let mut i = 0;
                while !self.bits[i] {
                    i += 1;
                }
                self.first_true_index = Some(i);
            }
            let first_true_index = self.first_true_index.unwrap();
            if previous_j <= first_true_index {
                bit
            } else {
                !bit
            }
        })
    }
}

impl<'a> ExactSizeIterator for NegativeBitIterator<'a> {}

trait SignExtendedBitIterator: DoubleEndedIterator<Item = bool> {
    const EXTENSION: bool;

    fn needs_sign_extension(&self) -> bool;

    fn iterate_forward(&mut self, extension_checked: &mut bool) -> Option<bool> {
        let next = self.next();
        if next.is_none() {
            if *extension_checked {
                None
            } else {
                *extension_checked = true;
                if self.needs_sign_extension() {
                    Some(Self::EXTENSION)
                } else {
                    None
                }
            }
        } else {
            next
        }
    }

    fn iterate_backward(&mut self, extension_checked: &mut bool) -> Option<bool> {
        if !*extension_checked {
            *extension_checked = true;
            if self.needs_sign_extension() {
                return Some(Self::EXTENSION);
            }
        }
        self.next_back()
    }
}

impl<'a> SignExtendedBitIterator for NaturalBitIterator<'a> {
    const EXTENSION: bool = false;

    fn needs_sign_extension(&self) -> bool {
        self[self.significant_bits - 1]
    }
}

impl<'a> SignExtendedBitIterator for NegativeBitIterator<'a> {
    const EXTENSION: bool = true;

    fn needs_sign_extension(&self) -> bool {
        let mut i = 0;
        while !self.bits[i] {
            i += 1;
        }
        let last_bit_index = self.bits.significant_bits - 1;
        if i == last_bit_index {
            !self.bits[last_bit_index]
        } else {
            self.bits[last_bit_index]
        }
    }
}

/// A double-ended iterator over the twos-complement bits of an [`Integer`].
///
/// The forward order is ascending (least-significant first). The most significant bit corresponds
/// to the sign of the [`Integer`]; `false` for non-negative and `true` for negative. This means
/// that there may be a single most-significant sign-extension bit.
///
/// This `struct` is created by [`BitIterable::bits`]; see its documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IntegerBitIterator<'a> {
    Zero,
    Positive(NaturalBitIterator<'a>, bool),
    Negative(NegativeBitIterator<'a>, bool),
}

impl<'a> Iterator for IntegerBitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        match *self {
            IntegerBitIterator::Zero => None,
            IntegerBitIterator::Positive(ref mut bits, ref mut extension_checked) => {
                bits.iterate_forward(extension_checked)
            }
            IntegerBitIterator::Negative(ref mut bits, ref mut extension_checked) => {
                bits.iterate_forward(extension_checked)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for IntegerBitIterator<'a> {
    fn next_back(&mut self) -> Option<bool> {
        match *self {
            IntegerBitIterator::Zero => None,
            IntegerBitIterator::Positive(ref mut bits, ref mut extension_checked) => {
                bits.iterate_backward(extension_checked)
            }
            IntegerBitIterator::Negative(ref mut bits, ref mut extension_checked) => {
                bits.iterate_backward(extension_checked)
            }
        }
    }
}

impl<'a> Index<u64> for IntegerBitIterator<'a> {
    type Output = bool;

    /// A function to retrieve an [`Integer`]'s two's complement bits by index. Indexing at or above
    /// the significant bit count returns `false` or `true` bits, depending on the [`Integer`]'s
    /// sign.
    ///
    /// This is equivalent to [`get_bit`](malachite_base::num::logic::traits::BitAccess::get_bit).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.bits()[0], false);
    ///
    /// // -105 = 10010111 in two's complement
    /// let n = Integer::from(-105);
    /// let bits = n.bits();
    /// assert_eq!(bits[0], true);
    /// assert_eq!(bits[1], true);
    /// assert_eq!(bits[2], true);
    /// assert_eq!(bits[3], false);
    /// assert_eq!(bits[4], true);
    /// assert_eq!(bits[5], false);
    /// assert_eq!(bits[6], false);
    /// assert_eq!(bits[7], true);
    /// assert_eq!(bits[100], true);
    /// ```
    fn index(&self, index: u64) -> &bool {
        let bit = match *self {
            IntegerBitIterator::Zero => false,
            IntegerBitIterator::Positive(ref bits, _) => bits.limbs.n.get_bit(index),
            IntegerBitIterator::Negative(ref bits, _) => bits.bits.limbs.n.get_bit_neg(index),
        };
        if bit {
            &true
        } else {
            &false
        }
    }
}

impl Natural {
    // Returns a double-ended iterator over the two's complement bits of the negative of a
    // `Natural`. The forward order is ascending, so that less significant bits appear first. There
    // may be at most one trailing `true` bit going forward, or leading `true` bit going backward.
    // The `Natural` cannot be zero.
    //
    // # Worst-case complexity
    // Constant time and additional memory.
    fn negative_bits(&self) -> NegativeBitIterator {
        assert_ne!(*self, 0, "Cannot get negative bits of 0.");
        let bits = self.bits();
        NegativeBitIterator {
            bits,
            first_true_index: None,
            i: 0,
            j: bits.significant_bits - 1,
        }
    }
}

impl<'a> BitIterable for &'a Integer {
    type BitIterator = IntegerBitIterator<'a>;

    /// Returns a double-ended iterator over the bits of an [`Integer`].
    ///
    /// The forward order is ascending, so that less significant bits appear first. There are no
    /// trailing false bits going forward, or leading falses going backward, except for possibly a
    /// most-significant sign-extension bit.
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
    /// use itertools::Itertools;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.bits().next(), None);
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).bits().collect_vec(),
    ///     &[true, false, false, true, false, true, true, false]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).bits().collect_vec(),
    ///     &[true, true, true, false, true, false, false, true]
    /// );
    ///
    /// assert_eq!(Integer::ZERO.bits().next_back(), None);
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).bits().rev().collect_vec(),
    ///     &[false, true, true, false, true, false, false, true]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).bits().rev().collect_vec(),
    ///     &[true, false, false, true, false, true, true, true]
    /// );
    /// ```
    fn bits(self) -> IntegerBitIterator<'a> {
        if *self == 0 {
            IntegerBitIterator::Zero
        } else if self.sign {
            IntegerBitIterator::Positive(self.abs.bits(), false)
        } else {
            IntegerBitIterator::Negative(self.abs.negative_bits(), false)
        }
    }
}
