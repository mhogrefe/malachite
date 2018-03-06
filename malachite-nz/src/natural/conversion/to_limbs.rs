use natural::Natural::{self, Large, Small};

/// An iterator over the limbs of a `Natural` in ascending order (least-significant first).
pub struct LimbIteratorAsc<'a> {
    n: &'a Natural,
    limb_count: u64,
    i: u64,
}

impl<'a> Iterator for LimbIteratorAsc<'a> {
    type Item = u32;

    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn next(&mut self) -> Option<u32> {
        if self.i < self.limb_count {
            let limb = match *self.n {
                Small(small) => small,
                Large(ref limbs) => limbs[self.i as usize],
            };
            self.i += 1;
            Some(limb)
        } else {
            None
        }
    }
}

/// An iterator over the limbs of a `Natural` in descending order (most-significant first).
pub struct LimbIteratorDesc<'a> {
    n: &'a Natural,
    some_remaining: bool,
    i: u64,
}

impl<'a> Iterator for LimbIteratorDesc<'a> {
    type Item = u32;

    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn next(&mut self) -> Option<u32> {
        if self.some_remaining {
            let limb = match *self.n {
                Small(small) => small,
                Large(ref limbs) => limbs[self.i as usize],
            };
            if self.i == 0 {
                self.some_remaining = false;
            } else {
                self.i -= 1;
            }
            Some(limb)
        } else {
            None
        }
    }
}

impl Natural {
    /// Returns the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in ascending order, so
    /// that less significant limbs have lower indices in the output vector. There are no trailing
    /// zero limbs.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible, `into_limbs_asc` is
    /// more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is more efficient than `Natural::to_limbs_desc`.
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
    /// This function borrows `self`. If taking ownership of `self` is possible, `into_limbs_desc`
    /// is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is less efficient than `Natural::to_limbs_asc`.
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
        match *self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(ref limbs) => limbs.iter().cloned().rev().collect(),
        }
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in ascending order, so
    /// that less significant limbs have lower indices in the output vector. There are no trailing
    /// zero limbs.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_limbs_asc`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This method is more efficient than `Natural::into_limbs_desc`.
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
    ///     assert!(Natural::ZERO.into_limbs_asc().is_empty());
    ///     assert_eq!(Natural::from(123u32).into_limbs_asc(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().into_limbs_asc(), vec![3567587328, 232]);
    /// }
    /// ```
    pub fn into_limbs_asc(self) -> Vec<u32> {
        match self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(limbs) => limbs,
        }
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. There are no leading
    /// zero limbs.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is less efficient than `Natural::into_limbs_asc`.
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
    ///     assert!(Natural::ZERO.into_limbs_desc().is_empty());
    ///     assert_eq!(Natural::from(123u32).into_limbs_desc(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().into_limbs_desc(), vec![232, 3567587328]);
    /// }
    /// ```
    pub fn into_limbs_desc(self) -> Vec<u32> {
        match self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(mut limbs) => {
                limbs.reverse();
                limbs
            }
        }
    }

    /// Returns an iterator the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in ascending
    /// order, so that less significant limbs appear first. There are no trailing zero limbs.
    ///
    /// If it's necessary to get a `Vec` of all the limbs, consider using `to_limbs_asc` or
    /// `into_limbs_asc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
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
    ///     assert!(Natural::ZERO.limbs_asc().next().is_none());
    ///     assert_eq!(Natural::from(123u32).limbs_asc().collect::<Vec<u32>>(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().limbs_asc().collect::<Vec<u32>>(),
    ///         vec![3567587328, 232]);
    /// }
    /// ```
    pub fn limbs_asc(&self) -> LimbIteratorAsc {
        LimbIteratorAsc {
            n: self,
            limb_count: self.limb_count(),
            i: 0,
        }
    }

    /// Returns an iterator the limbs, or base-2<sup>32</sup> digits, of a `Natural`, in descending
    /// order, so that more significant limbs appear first. There are no leading zero limbs.
    ///
    /// If it's necessary to get a `Vec` of all the limbs, consider using `to_limbs_desc` or
    /// `into_limbs_desc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
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
    ///     assert!(Natural::ZERO.limbs_desc().next().is_none());
    ///     assert_eq!(Natural::from(123u32).limbs_desc().collect::<Vec<u32>>(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().limbs_desc().collect::<Vec<u32>>(),
    ///         vec![232, 3567587328]);
    /// }
    /// ```
    pub fn limbs_desc(&self) -> LimbIteratorDesc {
        let limb_count = self.limb_count();
        LimbIteratorDesc {
            n: self,
            some_remaining: limb_count != 0,
            i: limb_count - 1,
        }
    }
}
