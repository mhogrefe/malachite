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
    /// extern crate malachite_base;
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert!(Natural::zero().to_limbs_le().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_le(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().to_limbs_le(),
    ///             vec![3567587328, 232]);
    /// }
    /// ```
    pub fn to_limbs_le(&self) -> Vec<u32> {
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
    /// extern crate malachite_base;
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert!(Natural::zero().to_limbs_be().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_be(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().to_limbs_be(),
    ///             vec![232, 3567587328]);
    /// }
    /// ```
    pub fn to_limbs_be(&self) -> Vec<u32> {
        self.to_limbs_le().into_iter().rev().collect()
    }

    //TODO doc and test
    pub fn into_limbs_le(self) -> Vec<u32> {
        match self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(limbs) => limbs,
        }
    }

    //TODO doc and test
    pub fn into_limbs_be(self) -> Vec<u32> {
        let mut limbs = self.into_limbs_le();
        limbs.reverse();
        limbs
    }
}
