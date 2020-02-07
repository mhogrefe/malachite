use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitConvertible};

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
    /// # Example
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
            for i in 0..u64::from(Limb::WIDTH) {
                bits.push(limb.get_bit(i));
            }
        }
        while last != 0 {
            bits.push(last.get_bit(0));
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
    /// # Example
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
}

impl Natural {
    pub fn _to_bits_desc_alt(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        if *self == 0 {
            return bits;
        }
        let mut first = true;
        for limb in self.limbs().rev() {
            let mut i = u64::from(if first {
                first = false;
                Limb::WIDTH - limb.leading_zeros() - 1
            } else {
                Limb::WIDTH - 1
            });
            loop {
                bits.push(limb.get_bit(i));
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        bits
    }
}
