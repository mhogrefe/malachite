use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in ascending order, so
    /// that less significant limbs have lower indices in the output vector. There are no trailing
    /// zero limbs.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is more efficient than `Natural::limbs_desc`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert!(Natural::ZERO.to_limbs_asc().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_asc(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().to_limbs_asc(), vec![3567587328, 232]);
    /// }
    /// ```
    pub fn to_limbs_asc(&self) -> Vec<u32> {
        match *self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(ref limbs) => limbs.clone(),
        }
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. There are no leading
    /// zero limbs.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is less efficient than `Natural::limbs_asc`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert!(Natural::ZERO.to_limbs_desc().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_desc(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().to_limbs_desc(), vec![232, 3567587328]);
    /// }
    /// ```
    pub fn to_limbs_desc(&self) -> Vec<u32> {
        self.to_limbs_asc().into_iter().rev().collect()
    }

    //TODO doc and test
    pub fn into_limbs_asc(self) -> Vec<u32> {
        match self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(limbs) => limbs,
        }
    }

    //TODO doc and test
    pub fn into_limbs_desc(self) -> Vec<u32> {
        let mut limbs = self.into_limbs_asc();
        limbs.reverse();
        limbs
    }
}
