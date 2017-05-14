use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Set the `index`th bit of `self`, or the coefficient of 2^(`index`) in the binary expansion
    /// of `self`, to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// let mut x = Natural::new();
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x.to_string(), "100");
    /// ```
    pub fn set_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(self,
                                        small,
                                        limbs,
                                        {
                                            if index < LIMB_BITS as u64 {
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
                                            limbs[limb_index] |= 1 <<
                                                                 (index & LIMB_BITS_MASK as u64);
                                        });
    }
}
