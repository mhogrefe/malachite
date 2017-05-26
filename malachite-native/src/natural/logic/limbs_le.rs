use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the limbs, or base-2^(32) digits, of `self`, in little-endian order, so that less
    /// significant limbs have lower indices in the output vector.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert!(Natural::from(0u32).limbs_le().is_empty());
    /// assert_eq!(Natural::from(123u32).limbs_le(), vec![123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().limbs_le(), vec![3567587328, 232]);
    /// ```
    pub fn limbs_le(&self) -> Vec<u32> {
        match *self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(ref limbs) => limbs.clone(),
        }
    }
}
