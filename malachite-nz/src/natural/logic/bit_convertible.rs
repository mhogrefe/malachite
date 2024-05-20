// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, BitConvertible};

impl BitConvertible for Natural {
    /// Returns a [`Vec`] containing the bits of a [`Natural`] in ascending order: least- to
    /// most-significant.
    ///
    /// If the number is 0, the [`Vec`] is empty; otherwise, it ends with `true`.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_bits_asc().is_empty());
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from(105u32).to_bits_asc(),
    ///     &[true, false, false, true, false, true, true]
    /// );
    /// ```
    fn to_bits_asc(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        if *self == 0 {
            return bits;
        }
        let limbs = self.limbs();
        let last_index = usize::exact_from(self.limb_count()) - 1;
        let mut last = limbs[last_index];
        for limb in limbs.take(last_index) {
            for i in 0..Limb::WIDTH {
                bits.push(limb.get_bit(i));
            }
        }
        while last != 0 {
            bits.push(last.odd());
            last >>= 1;
        }
        bits
    }

    /// Returns a [`Vec`] containing the bits of a [`Natural`] in descending order: most- to
    /// least-significant.
    ///
    /// If the number is 0, the [`Vec`] is empty; otherwise, it begins with `true`.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_bits_desc().is_empty());
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from(105u32).to_bits_desc(),
    ///     &[true, true, false, true, false, false, true]
    /// );
    /// ```
    fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_bits_asc();
        bits.reverse();
        bits
    }

    /// Converts an iterator of bits into a [`Natural`]. The bits should be in ascending order
    /// (least- to most-significant).
    ///
    /// $$
    /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^i \[b_i\],
    /// $$
    /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_bits_asc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_asc(
    ///         [true, false, false, true, false, true, true]
    ///             .iter()
    ///             .cloned()
    ///     ),
    ///     105
    /// );
    /// ```
    fn from_bits_asc<I: Iterator<Item = bool>>(xs: I) -> Natural {
        Natural::from_owned_limbs_asc(
            xs.chunks(usize::wrapping_from(Limb::WIDTH))
                .into_iter()
                .map(Limb::from_bits_asc)
                .collect(),
        )
    }

    /// Converts an iterator of bits into a [`Natural`]. The bits should be in descending order
    /// (most- to least-significant).
    ///
    /// $$
    /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^{k-i-1} \[b_i\],
    /// $$
    /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_bits_desc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_desc(
    ///         [true, true, false, true, false, false, true]
    ///             .iter()
    ///             .cloned()
    ///     ),
    ///     105
    /// );
    /// ```
    fn from_bits_desc<I: Iterator<Item = bool>>(xs: I) -> Natural {
        let mut out = Vec::new();
        let mut last_width = 0;
        for chunk in &xs.chunks(usize::exact_from(Limb::WIDTH)) {
            let mut x = 0;
            let mut i = 0;
            for bit in chunk {
                x <<= 1;
                if bit {
                    x |= 1;
                }
                i += 1;
            }
            last_width = i;
            out.push(x);
        }
        match out.len() {
            0 => Natural::ZERO,
            1 => Natural::from(out[0]),
            _ => {
                out.reverse();
                if last_width != Limb::WIDTH {
                    let out_0 = out[0];
                    out[0] = 0;
                    limbs_slice_shr_in_place(&mut out, Limb::WIDTH - last_width);
                    out[0] |= out_0;
                }
                Natural::from_owned_limbs_asc(out)
            }
        }
    }
}
