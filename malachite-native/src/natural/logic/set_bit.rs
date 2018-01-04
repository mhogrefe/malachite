use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 1.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::ZERO;
    ///     x.set_bit(2);
    ///     x.set_bit(5);
    ///     x.set_bit(6);
    ///     assert_eq!(x.to_string(), "100");
    /// }
    /// ```
    pub fn set_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if index < LIMB_BITS.into() {
                    Some(*small | (1 << index))
                } else {
                    None
                }
            },
            {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index >= limbs.len() {
                    limbs.resize(limb_index + 1, 0);
                }
                limbs[limb_index] |= 1 << (index & LIMB_BITS_MASK as u64);
            }
        );
    }
}
