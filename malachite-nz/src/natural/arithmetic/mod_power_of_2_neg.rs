use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Neg, ModPowerOf2NegAssign, NegModPowerOf2, NegModPowerOf2Assign,
};
use natural::Natural;

impl ModPowerOf2Neg for Natural {
    type Output = Natural;

    /// Computes `-self` mod 2<sup>`pow`</sup>, taking `self` by value. Assumes the input is already
    /// reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_neg(5).to_string(), "0");
    /// assert_eq!(Natural::ZERO.mod_power_of_2_neg(100).to_string(), "0");
    /// assert_eq!(Natural::from(100u32).mod_power_of_2_neg(8).to_string(), "156");
    /// assert_eq!(
    ///     Natural::from(100u32).mod_power_of_2_neg(100).to_string(),
    ///     "1267650600228229401496703205276"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_neg(mut self, pow: u64) -> Natural {
        self.neg_mod_power_of_2_assign(pow);
        self
    }
}

impl<'a> ModPowerOf2Neg for &'a Natural {
    type Output = Natural;

    /// Computes `-self` mod 2<sup>`pow`</sup>, taking `self` by reference. Assumes the input is
    /// already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_neg(5).to_string(), "0");
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_neg(100).to_string(), "0");
    /// assert_eq!((&Natural::from(100u32)).mod_power_of_2_neg(8).to_string(), "156");
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_power_of_2_neg(100).to_string(),
    ///     "1267650600228229401496703205276"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_neg(self, pow: u64) -> Natural {
        self.neg_mod_power_of_2(pow)
    }
}

impl ModPowerOf2NegAssign for Natural {
    /// Replaces `self` with `-self` mod 2<sup>`pow`</sup>. Assumes the input is already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_neg_assign(5);
    /// assert_eq!(n.to_string(), "0");
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_neg_assign(100);
    /// assert_eq!(n.to_string(), "0");
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_power_of_2_neg_assign(8);
    /// assert_eq!(n.to_string(), "156");
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_power_of_2_neg_assign(100);
    /// assert_eq!(n.to_string(), "1267650600228229401496703205276");
    /// ```
    #[inline]
    fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
        self.neg_mod_power_of_2_assign(pow);
    }
}
