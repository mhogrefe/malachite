use malachite_base::num::conversion::traits::ExactFrom;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::ops::Index;

/// A double-ended iterator over the limbs of a `Natural`. The forward order is ascending (least-
/// significant first). The iterator does not iterate over the implicit leading zero limbs.
///
/// This struct also supports retrieving limbs by index. This functionality is completely
/// independent of the iterator's state. Indexing the implicit leading zero limbs is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LimbIterator<'a> {
    pub(crate) n: &'a Natural,
    pub(crate) limb_count: usize,
    // This is true iff `n` is nonzero and `i` and `j` are not yet equal. The iterator returns
    // `Some` iff this is true.
    pub(crate) some_remaining: bool,
    // If `n` is nonzero, this index initially points to the least-significant limb, and is
    // incremented by next().
    pub(crate) i: u64,
    // If `n` is nonzero, this index initially points to the most-significant limb, and is
    // decremented by next_back().
    pub(crate) j: u64,
}

impl<'a> Iterator for LimbIterator<'a> {
    type Item = Limb;

    /// A function to iterate through the limbs of a `Natural` in ascending order (least-significant
    /// first).
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.limbs().next(), None);
    ///
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// let trillion = Natural::trillion();
    /// let mut limbs = trillion.limbs();
    /// assert_eq!(limbs.next(), Some(3567587328));
    /// assert_eq!(limbs.next(), Some(232));
    /// assert_eq!(limbs.next(), None);
    /// ```
    fn next(&mut self) -> Option<Limb> {
        if self.some_remaining {
            let limb = match *self.n {
                Natural(Small(small)) => small,
                Natural(Large(ref limbs)) => limbs[usize::exact_from(self.i)],
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

    /// A function that returns the length of the limbs iterator; that is, the `Natural`'s limb
    /// count. The format is (lower bound, Option<upper bound>), but in this case it's trivial to
    /// always have an exact bound.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.limbs().size_hint(), (0, Some(0)));
    /// assert_eq!(Natural::trillion().limbs().size_hint(), (2, Some(2)));
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.limb_count, Some(self.limb_count))
    }
}

impl<'a> DoubleEndedIterator for LimbIterator<'a> {
    /// A function to iterate through the limbs of a `Natural` in descending order (most-significant
    /// first).
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.limbs().next_back(), None);
    ///
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// let trillion = Natural::trillion();
    /// let mut limbs = trillion.limbs();
    /// assert_eq!(limbs.next_back(), Some(232));
    /// assert_eq!(limbs.next_back(), Some(3567587328));
    /// assert_eq!(limbs.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<Limb> {
        if self.some_remaining {
            let limb = match *self.n {
                Natural(Small(small)) => small,
                Natural(Large(ref limbs)) => limbs[usize::exact_from(self.j)],
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

/// This allows for some optimizations, e.g. when collecting into a `Vec`.
impl<'a> ExactSizeIterator for LimbIterator<'a> {}

impl<'a> Index<usize> for LimbIterator<'a> {
    type Output = Limb;

    /// A function to retrieve limbs by index. The index is the power of 2<sup>32</sub> of which the
    /// limbs is a coefficient. Indexing at or above the limb count returns zero limbs.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.limbs()[0], 0);
    ///
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// let trillion = Natural::trillion();
    /// let limbs = trillion.limbs();
    /// assert_eq!(limbs[0], 3567587328);
    /// assert_eq!(limbs[1], 232);
    /// assert_eq!(limbs[2], 0);
    /// assert_eq!(limbs[100], 0);
    /// ```
    fn index(&self, index: usize) -> &Limb {
        if index >= self.limb_count {
            &0
        } else {
            match *self.n {
                Natural(Small(ref small)) => small,
                Natural(Large(ref limbs)) => limbs.index(index),
            }
        }
    }
}

impl Natural {
    /// Returns the limbs of a `Natural`, in ascending order, so that less significant limbs have
    /// lower indices in the output vector. There are no trailing zero limbs.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_limbs_asc().is_empty());
    /// assert_eq!(Natural::from(123u32).to_limbs_asc(), &[123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::trillion().to_limbs_asc(), &[3567587328, 232]);
    /// ```
    pub fn to_limbs_asc(&self) -> Vec<Limb> {
        match *self {
            natural_zero!() => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(ref limbs)) => limbs.clone(),
        }
    }

    /// Returns the limbs of a `Natural`, in descending order, so that less significant limbs have
    /// higher indices in the output vector. There are no leading zero limbs.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_limbs_desc().is_empty());
    /// assert_eq!(Natural::from(123u32).to_limbs_desc(), &[123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::trillion().to_limbs_desc(), &[232, 3567587328]);
    /// ```
    pub fn to_limbs_desc(&self) -> Vec<Limb> {
        match *self {
            natural_zero!() => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(ref limbs)) => limbs.iter().cloned().rev().collect(),
        }
    }

    /// Returns the limbs of a `Natural`, in ascending order, so that less significant limbs have
    /// lower indices in the output vector. There are no trailing zero limbs.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.into_limbs_asc().is_empty());
    /// assert_eq!(Natural::from(123u32).into_limbs_asc(), &[123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::trillion().into_limbs_asc(), &[3567587328, 232]);
    /// ```
    pub fn into_limbs_asc(self) -> Vec<Limb> {
        match self {
            natural_zero!() => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(limbs)) => limbs,
        }
    }

    /// Returns the limbs of a `Natural`, in descending order, so that less significant limbs have
    /// higher indices in the output vector. There are no leading zero limbs.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.into_limbs_desc().is_empty());
    /// assert_eq!(Natural::from(123u32).into_limbs_desc(), &[123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::trillion().into_limbs_desc(), &[232, 3567587328]);
    /// ```
    pub fn into_limbs_desc(self) -> Vec<Limb> {
        match self {
            natural_zero!() => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(mut limbs)) => {
                limbs.reverse();
                limbs
            }
        }
    }

    /// Returns a double-ended iterator over the limbs of a `Natural`. The forward order is
    /// ascending, so that less significant limbs appear first. There are no trailing zero limbs
    /// going forward, or leading zeros going backward.
    ///
    /// If it's necessary to get a `Vec` of all the limbs, consider using `to_limbs_asc`,
    /// `to_limbs_desc`, `into_limbs_asc`, or `into_limbs_desc` instead.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.limbs().next().is_none());
    /// assert_eq!(Natural::from(123u32).limbs().collect::<Vec<u32>>(), &[123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::trillion().limbs().collect::<Vec<u32>>(), &[3567587328, 232]);
    ///
    /// assert!(Natural::ZERO.limbs().rev().next().is_none());
    /// assert_eq!(Natural::from(123u32).limbs().rev().collect::<Vec<u32>>(), &[123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::trillion().limbs().rev().collect::<Vec<u32>>(),
    ///     &[232, 3567587328]);
    /// ```
    pub fn limbs(&self) -> LimbIterator {
        let limb_count = self.limb_count();
        LimbIterator {
            n: self,
            limb_count: usize::exact_from(limb_count),
            some_remaining: limb_count != 0,
            i: 0,
            j: limb_count.saturating_sub(1),
        }
    }
}
