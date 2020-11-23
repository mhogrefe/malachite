use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, BitConvertible};

use natural::arithmetic::shr::limbs_slice_shr_in_place;
use natural::Natural;
use platform::Limb;

impl BitConvertible for Natural {
    /// Returns the bits of a `Natural` in ascending order, so that less significant bits have lower
    /// indices in the output vector. There are no trailing false bits.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_bits_asc().is_empty());
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).to_bits_asc(),
    ///     vec![true, false, false, true, false, true, true]);
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

    /// Returns the bits of a `Natural` in ascending order, so that less significant bits have lower
    /// indices in the output vector. There are no leading false bits.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_bits_desc().is_empty());
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).to_bits_desc(),
    ///     vec![true, true, false, true, false, false, true]);
    /// ```
    fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_bits_asc();
        bits.reverse();
        bits
    }

    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::natural::Natural;
    /// use std::iter::empty;
    ///
    /// assert_eq!(Natural::from_bits_asc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_asc([true, false, false, true, false, true, true].iter().cloned()),
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

    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::natural::Natural;
    /// use std::iter::empty;
    ///
    /// assert_eq!(Natural::from_bits_desc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_desc([true, true, false, true, false, false, true].iter().cloned()),
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
