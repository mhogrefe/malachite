use integer::Integer;
use malachite_base::num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, ModPowerOf2, ModPowerOf2Assign, NegModPowerOf2,
    NegModPowerOf2Assign, RemPowerOf2, RemPowerOf2Assign,
};
use natural::Natural;

impl ModPowerOf2 for Integer {
    type Output = Natural;

    /// Takes an `Integer` mod a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
    ///
    /// Unlike rem_power_of_2, this function always returns a non-negative number.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).mod_power_of_2(8).to_string(), "4");
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!(Integer::from(-1611).mod_power_of_2(4).to_string(), "5");
    /// ```
    fn mod_power_of_2(self, pow: u64) -> Natural {
        if self.sign {
            self.abs.mod_power_of_2(pow)
        } else {
            self.abs.neg_mod_power_of_2(pow)
        }
    }
}

impl<'a> ModPowerOf2 for &'a Integer {
    type Output = Natural;

    /// Takes an `Integer` mod a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
    ///
    /// Unlike rem_power_of_2_ref, this function always returns a non-negative number.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Integer::from(260)).mod_power_of_2(8).to_string(), "4");
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!((&Integer::from(-1611)).mod_power_of_2(4).to_string(), "5");
    /// ```
    fn mod_power_of_2(self, pow: u64) -> Natural {
        if self.sign {
            (&self.abs).mod_power_of_2(pow)
        } else {
            (&self.abs).neg_mod_power_of_2(pow)
        }
    }
}

impl ModPowerOf2Assign for Integer {
    /// Reduces an `Integer` mod a power of 2 in place. In other words, replaces `self` with r,
    /// where `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
    ///
    /// Unlike rem_power_of_2_assign, this function always assigns a non-negative number.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.mod_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.mod_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "5");
    /// ```
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        if self.sign {
            self.abs.mod_power_of_2_assign(pow)
        } else {
            self.sign = true;
            self.abs.neg_mod_power_of_2_assign(pow)
        }
    }
}

impl RemPowerOf2 for Integer {
    type Output = Integer;

    /// Takes an `Integer` rem a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2<sup>`pow`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2<sup>`pow`</sup>.
    ///
    /// Unlike `mod_power_of_2`, this function always returns zero or a number with the same sign
    /// as `self`.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).rem_power_of_2(8).to_string(), "4");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).rem_power_of_2(4).to_string(), "-11");
    /// ```
    fn rem_power_of_2(self, pow: u64) -> Integer {
        let abs_rem = self.abs.mod_power_of_2(pow);
        Integer {
            sign: self.sign || abs_rem == 0,
            abs: abs_rem,
        }
    }
}

impl<'a> RemPowerOf2 for &'a Integer {
    type Output = Integer;

    /// Takes an `Integer` rem a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2<sup>`pow`</sup> + r, (r == 0 or sgn(r) == sgn(`self`)),
    /// and 0 <= |r| < 2<sup>`pow`</sup>.
    ///
    /// Unlike `mod_power_of_2_ref, this function always returns zero or a number with the same
    /// sign as `self`.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Integer::from(260)).rem_power_of_2(8).to_string(), "4");
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!((&Integer::from(-1611)).rem_power_of_2(4).to_string(), "-11");
    /// ```
    fn rem_power_of_2(self, pow: u64) -> Integer {
        let abs_rem = (&self.abs).mod_power_of_2(pow);
        Integer {
            sign: self.sign || abs_rem == 0,
            abs: abs_rem,
        }
    }
}

impl RemPowerOf2Assign for Integer {
    /// Reduces an `Integer` rem a power of 2 in place. In other words, replaces `self` with r,
    /// where `self` = q * 2<sup>`pow`</sup> + r, (r == 0 or sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2<sup>`pow`</sup>.
    ///
    /// Unlike `mod_power_of_2_assign, this function does never changes the sign of `self`, except
    /// possibly to set `self` to 0.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.rem_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.rem_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "-11");
    /// ```
    fn rem_power_of_2_assign(&mut self, pow: u64) {
        self.abs.mod_power_of_2_assign(pow);
        if self.abs == 0 {
            self.sign = true;
        }
    }
}

impl CeilingModPowerOf2 for Integer {
    type Output = Integer;

    /// Takes an `Integer` ceiling-mod a power of 2, taking the `Integer` by value. In other words,
    /// returns r, where `self` = q * 2<sup>`pow`</sup> + r and 0 <= -r < 2<sup>`pow`</sup>.
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
    /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!(Integer::from(260).ceiling_mod_power_of_2(8).to_string(), "-252");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).ceiling_mod_power_of_2(4).to_string(), "-11");
    /// ```
    fn ceiling_mod_power_of_2(self, pow: u64) -> Integer {
        let abs_mod = if self.sign {
            self.abs.neg_mod_power_of_2(pow)
        } else {
            self.abs.mod_power_of_2(pow)
        };
        Integer {
            sign: abs_mod == 0,
            abs: abs_mod,
        }
    }
}

impl<'a> CeilingModPowerOf2 for &'a Integer {
    type Output = Integer;

    /// Takes an `Integer` ceiling-mod a power of 2, taking the `Integer` by reference. In other
    /// words, returns r, where `self` = q * 2<sup>`pow`</sup> + r and
    /// 0 <= -r < 2<sup>`pow`</sup>.
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
    /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!((&Integer::from(260)).ceiling_mod_power_of_2(8).to_string(), "-252");
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!((&Integer::from(-1611)).ceiling_mod_power_of_2(4).to_string(), "-11");
    /// ```
    fn ceiling_mod_power_of_2(self, pow: u64) -> Integer {
        let abs_mod = if self.sign {
            (&self.abs).neg_mod_power_of_2(pow)
        } else {
            (&self.abs).mod_power_of_2(pow)
        };
        Integer {
            sign: abs_mod == 0,
            abs: abs_mod,
        }
    }
}

impl CeilingModPowerOf2Assign for Integer {
    /// Reduces an `Integer` ceiling-mod a power of 2 in place. In other words, replaces `self` with
    /// r, where `self` = q * 2<sup>`pow`</sup> + r and 0 <= -r < 2<sup>`pow`</sup>.
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
    /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// let mut x = Integer::from(260);
    /// x.ceiling_mod_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "-252");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.ceiling_mod_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "-11");
    /// ```
    fn ceiling_mod_power_of_2_assign(&mut self, pow: u64) {
        if self.sign {
            self.abs.neg_mod_power_of_2_assign(pow)
        } else {
            self.abs.mod_power_of_2_assign(pow)
        };
        self.sign = self.abs == 0;
    }
}
