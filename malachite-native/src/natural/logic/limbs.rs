use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the limbs, or base-2^(32) digits, of a `Natural`, in little-endian order, so that
    /// less significant limbs have lower indices in the output vector. There are no trailing zero
    /// limbs.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is more efficient than `Natural::limbs_be`.
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

    /// Returns the limbs, or base-2^(32) digits, of a `Natural`, in big-endian order, so that less
    /// significant limbs have higher indices in the output vector. There are no leading zero limbs.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is less efficient than `Natural::limbs_le`.
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
    pub fn limbs_be(&self) -> Vec<u32> {
        self.limbs_le().into_iter().rev().collect()
    }
}
