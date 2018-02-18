use integer::Integer;
use natural::Natural;

impl Integer {
    /// Takes a `Integer` mod a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Unlike rem_power_of_two, this function always returns a non-negative number.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).mod_power_of_two(8).to_string(), "4");
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!(Integer::from(-1611).mod_power_of_two(4).to_string(), "5");
    /// ```
    pub fn mod_power_of_two(self, other: u32) -> Natural {
        if self.sign {
            self.abs.mod_power_of_two(other)
        } else {
            self.abs.neg_mod_power_of_two(other)
        }
    }

    /// Takes a `Integer` mod a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2<pow>`other`</pow> + r and 0 <= r < 2<pow>`other`</pow>.
    ///
    /// Unlike rem_power_of_two_ref, this function always returns a non-negative number.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).mod_power_of_two_ref(8).to_string(), "4");
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!(Integer::from(-1611).mod_power_of_two_ref(4).to_string(), "5");
    /// ```
    pub fn mod_power_of_two_ref(&self, other: u32) -> Natural {
        if self.sign {
            self.abs.mod_power_of_two_ref(other)
        } else {
            self.abs.neg_mod_power_of_two_ref(other)
        }
    }

    /// Takes a `Integer` mod a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2<pow>`other`</pow> + r and 0 <= r < 2<pow>`other`</pow>.
    ///
    /// Unlike rem_power_of_two_assign, this function always assigns a non-negative number.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.mod_power_of_two_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.mod_power_of_two_assign(4);
    /// assert_eq!(x.to_string(), "5");
    /// ```
    pub fn mod_power_of_two_assign(&mut self, other: u32) {
        if self.sign {
            self.abs.mod_power_of_two_assign(other)
        } else {
            self.sign = true;
            self.abs.neg_mod_power_of_two_assign(other)
        }
    }

    /// Takes a `Integer` rem a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2<pow>`other`</pow> + r, r == 0 or (sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2<pow>`other`</pow>.
    ///
    /// Unlike `mod_power_of_two`, this function always returns zero or a number with the same sign as
    /// `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).rem_power_of_two(8).to_string(), "4");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).rem_power_of_two(4).to_string(), "-11");
    /// ```
    pub fn rem_power_of_two(self, other: u32) -> Integer {
        let abs_rem = self.abs.mod_power_of_two(other);
        Integer {
            sign: self.sign || abs_rem == 0,
            abs: abs_rem,
        }
    }

    /// Takes a `Integer` rem a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2<pow>`other`</pow> + r, (r == 0 or sgn(r) == sgn(`self`)),
    /// and 0 <= |r| < 2<pow>`other`</pow>.
    ///
    /// Unlike `mod_power_of_two_ref  , this function always returns zero or a number with the same
    /// sign as `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).rem_power_of_two_ref(8).to_string(), "4");
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).rem_power_of_two_ref(4).to_string(), "-11");
    /// ```
    pub fn rem_power_of_two_ref(&self, other: u32) -> Integer {
        let abs_rem = self.abs.mod_power_of_two_ref(other);
        Integer {
            sign: self.sign || abs_rem == 0,
            abs: abs_rem,
        }
    }

    /// Takes a `Integer` rem a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2<pow>`other`</pow> + r, (r == 0 or sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2<pow>`other`</pow>.
    ///
    /// Unlike `mod_power_of_two_assign, this function does never changes the sign of `self`, except
    /// possibly to set `self` to 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.rem_power_of_two_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.rem_power_of_two_assign(4);
    /// assert_eq!(x.to_string(), "-11");
    /// ```
    pub fn rem_power_of_two_assign(&mut self, other: u32) {
        self.abs.mod_power_of_two_assign(other);
        if self.abs == 0 {
            self.sign = true;
        }
    }

    /// Takes a `Integer` ceiling-mod a power of 2, taking the `Integer` by value. In other words,
    /// returns r, where `self` = q * 2<pow>`other`</pow> + r and 0 <= -r < 2<pow>`other`</pow>.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!(Integer::from(260).ceiling_mod_power_of_two(8).to_string(), "-252");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).ceiling_mod_power_of_two(4).to_string(), "-11");
    /// ```
    pub fn ceiling_mod_power_of_two(self, other: u32) -> Integer {
        let abs_mod = if self.sign {
            self.abs.neg_mod_power_of_two(other)
        } else {
            self.abs.mod_power_of_two(other)
        };
        Integer {
            sign: abs_mod == 0,
            abs: abs_mod,
        }
    }

    /// Takes a `Integer` ceiling-mod a power of 2, taking the `Integer` by reference. In other
    /// words, returns r, where `self` = q * 2<pow>`other`</pow> + r and
    /// 0 <= -r < 2<pow>`other`</pow>.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!(Integer::from(260).ceiling_mod_power_of_two_ref(8).to_string(), "-252");
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).ceiling_mod_power_of_two_ref(4).to_string(), "-11");
    /// ```
    pub fn ceiling_mod_power_of_two_ref(&self, other: u32) -> Integer {
        let abs_mod = if self.sign {
            self.abs.neg_mod_power_of_two_ref(other)
        } else {
            self.abs.mod_power_of_two_ref(other)
        };
        Integer {
            sign: abs_mod == 0,
            abs: abs_mod,
        }
    }

    /// Takes a `Integer` ceiling-mod a power of 2 in place. In other words, replaces `self` with r,
    /// where `self` = q * 2<pow>`other`</pow> + r and 0 <= -r < 2<pow>`other`</pow>.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// let mut x = Integer::from(260);
    /// x.ceiling_mod_power_of_two_assign(8);
    /// assert_eq!(x.to_string(), "-252");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.ceiling_mod_power_of_two_assign(4);
    /// assert_eq!(x.to_string(), "-11");
    /// ```
    pub fn ceiling_mod_power_of_two_assign(&mut self, other: u32) {
        if self.sign {
            self.abs.neg_mod_power_of_two_assign(other)
        } else {
            self.abs.mod_power_of_two_assign(other)
        };
        self.sign = self.abs == 0;
    }
}
