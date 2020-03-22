use malachite_base::num::arithmetic::traits::{ModNeg, ModNegAssign};
use malachite_base::num::basic::traits::Zero;

use natural::Natural;

impl ModNeg<Natural> for Natural {
    type Output = Natural;

    /// Computes `-self` mod `modulus`, taking `self` and `modulus` by value. Assumes the input is
    /// already reduced mod `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `modulus.significant_bits()`
    ///
    /// # Example
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
    fn mod_neg(mut self, modulus: Natural) -> Natural {
        self.mod_neg_assign(&modulus);
        self
    }
}

impl<'a> ModNeg<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes `-self` mod `modulus`, taking `self` by value and `modulus` by reference. Assumes
    /// the input is already reduced mod `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `modulus.significant_bits()`
    ///
    /// # Example
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
    fn mod_neg(mut self, modulus: &'a Natural) -> Natural {
        self.mod_neg_assign(modulus);
        self
    }
}

impl<'a> ModNeg<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes `-self` mod `modulus`, taking `self` by reference and `modulus` by value. Assumes
    /// the input is already reduced mod `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `modulus.significant_bits()`
    ///
    /// # Example
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
    fn mod_neg(self, modulus: Natural) -> Natural {
        if *self == 0 {
            Natural::ZERO
        } else {
            modulus - self
        }
    }
}

impl<'a, 'b> ModNeg<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Computes `-self` mod `modulus`, taking `self` by reference and `modulus` by value. Assumes
    /// the input is already reduced mod `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `modulus.significant_bits()`
    ///
    /// # Example
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
    fn mod_neg(self, modulus: &'b Natural) -> Natural {
        if *self == 0 {
            Natural::ZERO
        } else {
            modulus - self
        }
    }
}

impl ModNegAssign<Natural> for Natural {
    /// Replaces `self` with `-self` mod `modulus`, taking `modulus` by value. Assumes the input is
    /// already reduced mod `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `modulus.significant_bits()`
    ///
    /// # Example
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
    fn mod_neg_assign(&mut self, modulus: Natural) {
        self.mod_neg_assign(&modulus);
    }
}

impl<'a> ModNegAssign<&'a Natural> for Natural {
    /// Replaces `self` with `-self` mod `modulus`, taking `modulus` by reference. Assumes the input
    /// is already reduced mod `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `modulus.significant_bits()`
    ///
    /// # Example
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
    fn mod_neg_assign(&mut self, modulus: &'a Natural) {
        if *self != 0 {
            assert!(!self.sub_right_assign_no_panic(modulus));
        }
    }
}
