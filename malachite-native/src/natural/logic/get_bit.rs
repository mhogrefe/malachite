use natural::Natural::{self, Large, Small};

impl Natural {
    /// Determines whether the `index`th bit of `self`, or the coefficient of 2^(`index`) in the
    /// binary expansion of `self`, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123).get_bit(2), false);
    /// assert_eq!(Natural::from(123).get_bit(3), true);
    /// assert_eq!(Natural::from(123).get_bit(100), false);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().get_bit(12), true);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().get_bit(100), false);
    /// ```
    pub fn get_bit(&self, index: u64) -> bool {
        match *self {
            Small(x) => index < 32 && x & (1 << index) != 0,
            Large(ref xs) => {
                let limb_index = (index >> 5) as usize;
                xs.get(limb_index).map_or(false, |limb| limb & (1 << (index & 0x1f)) != 0)
            }
        }
    }
}
