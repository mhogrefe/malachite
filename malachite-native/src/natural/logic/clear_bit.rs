use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 0.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// let mut x = Natural::from(0x7fu32);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x.to_string(), "100");
    /// ```
    pub fn clear_bit(&mut self, index: u64) {
        match *self {
            Small(ref mut small) => {
                if index < LIMB_BITS.into() {
                    *small &= !(1 << index);
                }
                return;
            }
            Large(ref mut limbs) => {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index < limbs.len() {
                    limbs[limb_index] &= !(1 << (index & LIMB_BITS_MASK as u64));
                } else {
                    return;
                }
            }
        }
        self.trim();
    }
}
