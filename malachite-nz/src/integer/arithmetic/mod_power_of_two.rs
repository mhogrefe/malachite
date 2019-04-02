use integer::Integer;
use malachite_base::num::{
    CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, ModPowerOfTwo, ModPowerOfTwoAssign,
    NegModPowerOfTwo, NegModPowerOfTwoAssign, RemPowerOfTwo, RemPowerOfTwoAssign,
};
use natural::Natural;
use platform::Limb;

impl ModPowerOfTwo for Integer {
    type Output = Natural;

    /// Takes a `Integer` mod a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Unlike rem_power_of_two, this function always returns a non-negative number.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::ModPowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!(Integer::from(260).mod_power_of_two(8).to_string(), "4");
    ///
    ///     // -101 * 2^4 + 5 = -1611
    ///     assert_eq!(Integer::from(-1611).mod_power_of_two(4).to_string(), "5");
    /// }
    /// ```
    fn mod_power_of_two(self, other: u64) -> Natural {
        if self.sign {
            self.abs.mod_power_of_two(other)
        } else {
            self.abs.neg_mod_power_of_two(other)
        }
    }
}

impl<'a> ModPowerOfTwo for &'a Integer {
    type Output = Natural;

    /// Takes a `Integer` mod a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2<pow>`other`</pow> + r and 0 <= r < 2<pow>`other`</pow>.
    ///
    /// Unlike rem_power_of_two_ref, this function always returns a non-negative number.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::ModPowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!((&Integer::from(260)).mod_power_of_two(8).to_string(), "4");
    ///     // -101 * 2^4 + 5 = -1611
    ///     assert_eq!((&Integer::from(-1611)).mod_power_of_two(4).to_string(), "5");
    /// }
    /// ```
    fn mod_power_of_two(self, other: u64) -> Natural {
        if self.sign {
            (&self.abs).mod_power_of_two(other)
        } else {
            (&self.abs).neg_mod_power_of_two(other)
        }
    }
}

impl ModPowerOfTwoAssign for Integer {
    /// Takes a `Integer` mod a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2<pow>`other`</pow> + r and 0 <= r < 2<pow>`other`</pow>.
    ///
    /// Unlike rem_power_of_two_assign, this function always assigns a non-negative number.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::ModPowerOfTwoAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     let mut x = Integer::from(260);
    ///     x.mod_power_of_two_assign(8);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // -101 * 2^4 + 5 = -1611
    ///     let mut x = Integer::from(-1611);
    ///     x.mod_power_of_two_assign(4);
    ///     assert_eq!(x.to_string(), "5");
    /// }
    /// ```
    fn mod_power_of_two_assign(&mut self, other: u64) {
        if self.sign {
            self.abs.mod_power_of_two_assign(other)
        } else {
            self.sign = true;
            self.abs.neg_mod_power_of_two_assign(other)
        }
    }
}

impl RemPowerOfTwo for Integer {
    type Output = Integer;

    /// Takes a `Integer` rem a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2<pow>`other`</pow> + r, r == 0 or (sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2<pow>`other`</pow>.
    ///
    /// Unlike `mod_power_of_two`, this function always returns zero or a number with the same sign
    /// as `self`.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::RemPowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!(Integer::from(260).rem_power_of_two(8).to_string(), "4");
    ///
    ///     // -100 * 2^4 + -11 = -1611
    ///     assert_eq!(Integer::from(-1611).rem_power_of_two(4).to_string(), "-11");
    /// }
    /// ```
    fn rem_power_of_two(self, other: u64) -> Integer {
        let abs_rem = self.abs.mod_power_of_two(other);
        Integer {
            sign: self.sign || abs_rem == 0 as Limb,
            abs: abs_rem,
        }
    }
}

impl<'a> RemPowerOfTwo for &'a Integer {
    type Output = Integer;

    /// Takes a `Integer` rem a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2<pow>`other`</pow> + r, (r == 0 or sgn(r) == sgn(`self`)),
    /// and 0 <= |r| < 2<pow>`other`</pow>.
    ///
    /// Unlike `mod_power_of_two_ref, this function always returns zero or a number with the same
    /// sign as `self`.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::RemPowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!((&Integer::from(260)).rem_power_of_two(8).to_string(), "4");
    ///     // -100 * 2^4 + -11 = -1611
    ///     assert_eq!((&Integer::from(-1611)).rem_power_of_two(4).to_string(), "-11");
    /// }
    /// ```
    fn rem_power_of_two(self, other: u64) -> Integer {
        let abs_rem = (&self.abs).mod_power_of_two(other);
        Integer {
            sign: self.sign || abs_rem == 0 as Limb,
            abs: abs_rem,
        }
    }
}

impl RemPowerOfTwoAssign for Integer {
    /// Takes a `Integer` rem a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2<pow>`other`</pow> + r, (r == 0 or sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2<pow>`other`</pow>.
    ///
    /// Unlike `mod_power_of_two_assign, this function does never changes the sign of `self`, except
    /// possibly to set `self` to 0.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::RemPowerOfTwoAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     let mut x = Integer::from(260);
    ///     x.rem_power_of_two_assign(8);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // -100 * 2^4 + -11 = -1611
    ///     let mut x = Integer::from(-1611);
    ///     x.rem_power_of_two_assign(4);
    ///     assert_eq!(x.to_string(), "-11");
    /// }
    /// ```
    fn rem_power_of_two_assign(&mut self, other: u64) {
        self.abs.mod_power_of_two_assign(other);
        if self.abs == 0 as Limb {
            self.sign = true;
        }
    }
}

impl CeilingModPowerOfTwo for Integer {
    type Output = Integer;

    /// Takes a `Integer` ceiling-mod a power of 2, taking the `Integer` by value. In other words,
    /// returns r, where `self` = q * 2<pow>`other`</pow> + r and 0 <= -r < 2<pow>`other`</pow>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingModPowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 2^8 + -252 = 260
    ///     assert_eq!(Integer::from(260).ceiling_mod_power_of_two(8).to_string(), "-252");
    ///
    ///     // -100 * 2^4 + -11 = -1611
    ///     assert_eq!(Integer::from(-1611).ceiling_mod_power_of_two(4).to_string(), "-11");
    /// }
    /// ```
    fn ceiling_mod_power_of_two(self, other: u64) -> Integer {
        let abs_mod = if self.sign {
            self.abs.neg_mod_power_of_two(other)
        } else {
            self.abs.mod_power_of_two(other)
        };
        Integer {
            sign: abs_mod == 0 as Limb,
            abs: abs_mod,
        }
    }
}

impl<'a> CeilingModPowerOfTwo for &'a Integer {
    type Output = Integer;

    /// Takes a `Integer` ceiling-mod a power of 2, taking the `Integer` by reference. In other
    /// words, returns r, where `self` = q * 2<pow>`other`</pow> + r and
    /// 0 <= -r < 2<pow>`other`</pow>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingModPowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 2^8 + -252 = 260
    ///     assert_eq!((&Integer::from(260)).ceiling_mod_power_of_two(8).to_string(), "-252");
    ///     // -100 * 2^4 + -11 = -1611
    ///     assert_eq!((&Integer::from(-1611)).ceiling_mod_power_of_two(4).to_string(), "-11");
    /// }
    /// ```
    fn ceiling_mod_power_of_two(self, other: u64) -> Integer {
        let abs_mod = if self.sign {
            (&self.abs).neg_mod_power_of_two(other)
        } else {
            (&self.abs).mod_power_of_two(other)
        };
        Integer {
            sign: abs_mod == 0 as Limb,
            abs: abs_mod,
        }
    }
}

impl CeilingModPowerOfTwoAssign for Integer {
    /// Takes a `Integer` ceiling-mod a power of 2 in place. In other words, replaces `self` with r,
    /// where `self` = q * 2<pow>`other`</pow> + r and 0 <= -r < 2<pow>`other`</pow>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingModPowerOfTwoAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 2^8 + -252 = 260
    ///     let mut x = Integer::from(260);
    ///     x.ceiling_mod_power_of_two_assign(8);
    ///     assert_eq!(x.to_string(), "-252");
    ///     
    ///     // -100 * 2^4 + -11 = -1611
    ///     let mut x = Integer::from(-1611);
    ///     x.ceiling_mod_power_of_two_assign(4);
    ///     assert_eq!(x.to_string(), "-11");
    /// }
    /// ```
    fn ceiling_mod_power_of_two_assign(&mut self, other: u64) {
        if self.sign {
            self.abs.neg_mod_power_of_two_assign(other)
        } else {
            self.abs.mod_power_of_two_assign(other)
        };
        self.sign = self.abs == 0 as Limb;
    }
}
