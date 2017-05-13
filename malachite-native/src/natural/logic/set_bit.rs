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
                                        x,
                                        xs,
                                        {
                                            if index < 32 {
                                                Some(*x | (1 << index))
                                            } else {
                                                None
                                            }
                                        },
                                        {
                                            let limb_index = (index >> 5) as usize;
                                            if limb_index >= xs.len() {
                                                xs.resize(limb_index + 1, 0);
                                            }
                                            xs[limb_index] |= 1 << (index & 0x1f);
                                        });
    }
}
