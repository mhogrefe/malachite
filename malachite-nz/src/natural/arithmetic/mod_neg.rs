use malachite_base::num::arithmetic::traits::{ModNeg, ModNegAssign};
use malachite_base::num::basic::traits::Zero;

use natural::Natural;

impl ModNeg<Natural> for Natural {
    type Output = Natural;

    /// Computes `-self` mod `m`, taking `self` and `m` by value. Assumes the input is already
    /// reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_neg(Natural::from(5u32)).to_string(), "0");
    /// assert_eq!(Natural::from(7u32).mod_neg(Natural::from(10u32)).to_string(), "3");
    /// assert_eq!(Natural::from(7u32).mod_neg(Natural::trillion()).to_string(), "999999999993");
    /// ```
    #[inline]
    fn mod_neg(mut self, m: Natural) -> Natural {
        self.mod_neg_assign(&m);
        self
    }
}

impl<'a> ModNeg<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes `-self` mod `m`, taking `self` by value and `m` by reference. Assumes the input is
    /// already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_neg(&Natural::from(5u32)).to_string(), "0");
    /// assert_eq!(Natural::from(7u32).mod_neg(&Natural::from(10u32)).to_string(), "3");
    /// assert_eq!(Natural::from(7u32).mod_neg(&Natural::trillion()).to_string(), "999999999993");
    /// ```
    #[inline]
    fn mod_neg(mut self, m: &'a Natural) -> Natural {
        self.mod_neg_assign(m);
        self
    }
}

impl<'a> ModNeg<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes `-self` mod `m`, taking `self` by reference and `m` by value. Assumes the input is
    /// already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_neg(Natural::from(5u32)).to_string(), "0");
    /// assert_eq!((&Natural::from(7u32)).mod_neg(Natural::from(10u32)).to_string(), "3");
    /// assert_eq!((&Natural::from(7u32)).mod_neg(Natural::trillion()).to_string(), "999999999993");
    /// ```
    fn mod_neg(self, m: Natural) -> Natural {
        if *self == 0 {
            Natural::ZERO
        } else {
            m - self
        }
    }
}

impl<'a, 'b> ModNeg<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Computes `-self` mod `m`, taking `self` by reference and `m` by value. Assumes the input is
    /// already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_neg(&Natural::from(5u32)).to_string(), "0");
    /// assert_eq!((&Natural::from(7u32)).mod_neg(&Natural::from(10u32)).to_string(), "3");
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_neg(&Natural::trillion()).to_string(),
    ///     "999999999993"
    /// );
    /// ```
    fn mod_neg(self, m: &'b Natural) -> Natural {
        if *self == 0 {
            Natural::ZERO
        } else {
            m - self
        }
    }
}

impl ModNegAssign<Natural> for Natural {
    /// Replaces `self` with `-self` mod `m`, taking `m` by value. Assumes the input is already
    /// reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_neg_assign(Natural::from(5u32));
    /// assert_eq!(n.to_string(), "0");
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(Natural::from(10u32));
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(Natural::trillion());
    /// assert_eq!(n.to_string(), "999999999993");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: Natural) {
        self.mod_neg_assign(&m);
    }
}

impl<'a> ModNegAssign<&'a Natural> for Natural {
    /// Replaces `self` with `-self` mod `m`, taking `m` by reference. Assumes the input is already
    /// reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_neg_assign(&Natural::from(5u32));
    /// assert_eq!(n.to_string(), "0");
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(&Natural::from(10u32));
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(&Natural::trillion());
    /// assert_eq!(n.to_string(), "999999999993");
    /// ```
    fn mod_neg_assign(&mut self, m: &'a Natural) {
        if *self != 0 {
            assert!(!self.sub_right_assign_no_panic(m));
        }
    }
}
