use natural::{LIMB_BITS, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the smallest number of bits necessary to represent `self`. 0 has zero significant
    /// bits.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// assert_eq!(Natural::from(0u32).significant_bits(), 0);
    /// assert_eq!(Natural::from(100u32).significant_bits(), 7);
    /// ```
    pub fn significant_bits(&self) -> u64 {
        match *self {
            Small(small) => (LIMB_BITS - small.leading_zeros()) as u64,
            Large(ref limbs) => {
                ((limbs.len() as u64) << LOG_LIMB_BITS as u64) -
                limbs.last().unwrap().leading_zeros() as u64
            }
        }
    }
}
