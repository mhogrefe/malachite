use natural::Natural::{self, Large, Small};

/// A double-ended iterator over the limbs of a `Natural`. The forward order is ascending (least-
/// significant first).
pub struct LimbIterator<'a> {
    n: &'a Natural,
    some_remaining: bool,
    i: u64,
    j: u64,
}

impl<'a> Iterator for LimbIterator<'a> {
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
            if self.i == self.j {
                self.some_remaining = false;
            } else {
                self.i += 1;
            }
            Some(limb)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for LimbIterator<'a> {
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn next_back(&mut self) -> Option<u32> {
        if self.some_remaining {
            let limb = match *self.n {
                Small(small) => small,
                Large(ref limbs) => limbs[self.j as usize],
            };
            if self.j == self.i {
                self.some_remaining = false;
            } else {
                self.j -= 1;
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

    /// Returns a double-ended iterator over the limbs, or base-2<sup>32</sup> digits, of a
    /// `Natural`. The forward order is ascending, so that less significant limbs appear first.
    /// There are no trailing zero limbs going forward, or leading zeros going backward.
    ///
    /// If it's necessary to get a `Vec` of all the limbs, consider using `to_limbs_asc`,
    /// `to_limbs_desc`, `into_limbs_asc`, or `into_limbs_asc` instead.
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
    ///     assert!(Natural::ZERO.limbs().next().is_none());
    ///     assert_eq!(Natural::from(123u32).limbs().collect::<Vec<u32>>(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().limbs().collect::<Vec<u32>>(),
    ///         vec![3567587328, 232]);
    ///
    ///     assert!(Natural::ZERO.limbs().rev().next().is_none());
    ///     assert_eq!(Natural::from(123u32).limbs().rev().collect::<Vec<u32>>(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::trillion().limbs().rev().collect::<Vec<u32>>(),
    ///         vec![232, 3567587328]);
    /// }
    /// ```
    pub fn limbs(&self) -> LimbIterator {
        let limb_count = self.limb_count();
        LimbIterator {
            n: self,
            some_remaining: limb_count != 0,
            i: 0,
            j: limb_count - 1,
        }
    }
}
