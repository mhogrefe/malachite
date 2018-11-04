use integer::Integer;
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem, UnsignedAbs,
};
use natural::Natural;

impl DivMod<i32> for Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity, and the remainder is
    /// always non-negative and less than the absolute value of the divisor. In other words, returns
    /// (q, r), where `self` = q * `other` + r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).div_mod(123)), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).div_mod(-123)), "(-3, 87)");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).div_mod(123)), "(-4, 36)");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).div_mod(-123)), "(4, 36)");
    /// }
    /// ```
    fn div_mod(self, other: i32) -> (Integer, u32) {
        let other_abs = other.unsigned_abs();
        let (quotient, remainder) = if self.sign {
            self.abs.div_mod(other_abs)
        } else {
            self.abs.ceiling_div_neg_mod(other_abs)
        };
        (
            if (other >= 0) == self.sign {
                Integer::from(quotient)
            } else {
                -quotient
            },
            remainder,
        )
    }
}

impl<'a> DivMod<i32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// is always non-negative and less than the absolute value of the divisor. In other words,
    /// returns (q, r), where `self` = q * `other` + r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).div_mod(123)), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).div_mod(-123)), "(-3, 87)");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).div_mod(123)), "(-4, 36)");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).div_mod(-123)), "(4, 36)");
    /// }
    /// ```
    fn div_mod(self, other: i32) -> (Integer, u32) {
        let other_abs = other.unsigned_abs();
        let (quotient, remainder) = if self.sign {
            (&self.abs).div_mod(other_abs)
        } else {
            (&self.abs).ceiling_div_neg_mod(other_abs)
        };
        (
            if (other >= 0) == self.sign {
                Integer::from(quotient)
            } else {
                -quotient
            },
            remainder,
        )
    }
}

impl DivAssignMod<i32> for Integer {
    type ModOutput = u32;

    /// Divides an `Integer` by an `i32` in place, returning the remainder. The quotient is rounded
    /// towards negative infinity, and the remainder is always non-negative and less than the
    /// absolute value of the divisor. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` + r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.div_assign_mod(123), 87u32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.div_assign_mod(-123), 87u32);
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.div_assign_mod(123), 36u32);
    ///     assert_eq!(x.to_string(), "-4");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.div_assign_mod(-123), 36u32);
    ///     assert_eq!(x.to_string(), "4");
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: i32) -> u32 {
        let other_abs = other.unsigned_abs();
        let remainder = if self.sign {
            self.abs.div_assign_mod(other_abs)
        } else {
            self.abs.ceiling_div_assign_neg_mod(other_abs)
        };
        self.sign ^= other < 0;
        if !self.sign && self.abs == 0 {
            self.sign = true;
        }
        remainder
    }
}

impl DivMod<Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity, and the remainder is
    /// always non-negative and less than the absolute value of the divisor. In other words, returns
    /// (q, r), where `self` = q * `other` + r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_mod(Integer::from(123))), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_mod(Integer::from(-123))), "(-3, 87)");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_mod(Integer::from(123))), "(-4, 36)");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_mod(Integer::from(-123))), "(4, 36)");
    /// }
    /// ```
    fn div_mod(self, other: Integer) -> (Integer, Natural) {
        let self_abs = self.unsigned_abs();
        let (quotient, remainder) = if self >= 0 {
            let (quotient, remainder) = self_abs.div_mod(other.abs);
            (quotient, Natural::from(remainder))
        } else {
            self_abs.ceiling_div_neg_mod(other.abs)
        };
        (
            if (self >= 0) == other.sign {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl<'a> DivMod<&'a Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// is always non-negative and less than the absolute value of the divisor. In other words,
    /// returns (q, r), where `self` = q * `other` + r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_mod(&Integer::from(123))), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_mod(&Integer::from(-123))), "(-3, 87)");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_mod(&Integer::from(123))), "(-4, 36)");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_mod(&Integer::from(-123))), "(4, 36)");
    /// }
    /// ```
    fn div_mod(self, other: &'a Integer) -> (Integer, Natural) {
        let self_abs = self.unsigned_abs();
        let (quotient, remainder) = if self >= 0 {
            let (quotient, remainder) = self_abs.div_mod(&other.abs);
            (quotient, Natural::from(remainder))
        } else {
            self_abs.ceiling_div_neg_mod(&other.abs)
        };
        (
            if (self >= 0) == other.sign {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl DivRem<i32> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero, and the remainder has the same sign as
    /// the dividend and its absolute value is less than the absolute value of the divisor. In other
    /// words, returns (q, r), where `self` = q * `other` + r, (r = 0 or sign(r) = sign(`self`)),
    /// and 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).div_rem(123)), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).div_rem(-123)), "(-3, 87)");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).div_rem(123)), "(-3, -87)");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).div_rem(-123)), "(3, -87)");
    /// }
    /// ```
    fn div_rem(self, other: i32) -> (Integer, Integer) {
        let other_abs = other.unsigned_abs();
        let (quotient, remainder) = self.abs.div_mod(other_abs);
        let quotient = if (other >= 0) == self.sign {
            Integer::from(quotient)
        } else {
            -quotient
        };
        let remainder = if self.sign {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        };
        (quotient, remainder)
    }
}

impl<'a> DivRem<i32> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero, and the remainder has the same
    /// sign as the dividend and its absolute value is less than the absolute value of the divisor.
    /// In other words, returns (q, r), where `self` = q * `other` + r,
    /// (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).div_rem(123)), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).div_rem(-123)), "(-3, 87)");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).div_rem(123)), "(-3, -87)");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).div_rem(-123)), "(3, -87)");
    /// }
    /// ```
    fn div_rem(self, other: i32) -> (Integer, Integer) {
        let other_abs = other.unsigned_abs();
        let (quotient, remainder) = (&self.abs).div_mod(other_abs);
        let quotient = if (other >= 0) == self.sign {
            Integer::from(quotient)
        } else {
            -quotient
        };
        let remainder = if self.sign {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        };
        (quotient, remainder)
    }
}

impl DivAssignRem<i32> for Integer {
    type RemOutput = Integer;

    /// Divides an `Integer` by an `i32` in place, returning the remainder. The quotient is rounded
    /// towards zero, and the remainder has the same sign as the dividend and its absolute value is
    /// less than the absolute value of the divisor. In other words, returns `self` with q and returns
    /// r, where `self` = q * `other` + r, (r = 0 or sign(r) = sign(`self`)), and
    /// 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivAssignRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.div_assign_rem(123i32).to_string(), "87");
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.div_assign_rem(-123i32).to_string(), "87");
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.div_assign_rem(123i32).to_string(), "-87");
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.div_assign_rem(-123i32).to_string(), "-87");
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn div_assign_rem(&mut self, other: i32) -> Integer {
        let other_abs = other.unsigned_abs();
        let remainder = self.abs.div_assign_mod(other_abs);
        let remainder = if self.sign {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        };
        self.sign ^= other < 0;
        if !self.sign && self.abs == 0 {
            self.sign = true;
        }
        remainder
    }
}

impl DivRem<Integer> for i32 {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero, and the remainder has the same sign as
    /// the dividend and its absolute value is less than the absolute value of the divisor. In other
    /// words, returns (q, r), where `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_rem(Integer::from(123))), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_rem(Integer::from(-123))), "(-3, 87)");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_rem(Integer::from(123))), "(-3, -87)");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_rem(Integer::from(-123))), "(3, -87)");
    /// }
    /// ```
    fn div_rem(self, other: Integer) -> (Integer, Integer) {
        let self_abs = self.unsigned_abs();
        let (quotient, remainder) = self_abs.div_mod(other.abs);
        let quotient = if (self >= 0) == other.sign {
            Integer::from(quotient)
        } else {
            -Natural::from(quotient)
        };
        let remainder = if self >= 0 {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        };
        (quotient, remainder)
    }
}

impl<'a> DivRem<&'a Integer> for i32 {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero, and the remainder has the same
    /// sign as the dividend and its absolute value is less than the absolute value of the divisor.
    /// In other words, returns (q, r), where `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_rem(&Integer::from(123))), "(3, 87)");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456.div_rem(&Integer::from(-123))), "(-3, 87)");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_rem(&Integer::from(123))), "(-3, -87)");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (-456).div_rem(&Integer::from(-123))), "(3, -87)");
    /// }
    /// ```
    fn div_rem(self, other: &'a Integer) -> (Integer, Integer) {
        let self_abs = self.unsigned_abs();
        let (quotient, remainder) = self_abs.div_mod(&other.abs);
        let quotient = if (self >= 0) == other.sign {
            Integer::from(quotient)
        } else {
            -Natural::from(quotient)
        };
        let remainder = if self >= 0 {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        };
        (quotient, remainder)
    }
}

impl CeilingDivNegMod<i32> for Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the negative of the `Integer` divided by the `i32`. The quotient is
    /// rounded towards positive infinity, and the remainder is always non-negative and less than
    /// the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` - r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivNegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).ceiling_div_neg_mod(123)), "(4, 36)");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).ceiling_div_neg_mod(-123)), "(-4, 36)");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).ceiling_div_neg_mod(123)), "(-3, 87)");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).ceiling_div_neg_mod(-123)), "(3, 87)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: i32) -> (Integer, u32) {
        let other_abs = other.unsigned_abs();
        let (quotient, remainder) = if self.sign {
            self.abs.ceiling_div_neg_mod(other_abs)
        } else {
            self.abs.div_mod(other_abs)
        };
        (
            if (other >= 0) == self.sign {
                Integer::from(quotient)
            } else {
                -quotient
            },
            remainder,
        )
    }
}

impl<'a> CeilingDivNegMod<i32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the negative of the `Integer` divided by the `i32`. The
    /// quotient is rounded towards positive infinity, and the remainder is always non-negative and
    /// less than the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` - r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivNegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).ceiling_div_neg_mod(123)), "(4, 36)");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).ceiling_div_neg_mod(-123)),
    ///         "(-4, 36)");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).ceiling_div_neg_mod(123)),
    ///         "(-3, 87)");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).ceiling_div_neg_mod(-123)),
    ///         "(3, 87)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: i32) -> (Integer, u32) {
        let other_abs = other.unsigned_abs();
        let (quotient, remainder) = if self.sign {
            (&self.abs).ceiling_div_neg_mod(other_abs)
        } else {
            (&self.abs).div_mod(other_abs)
        };
        (
            if (other >= 0) == self.sign {
                Integer::from(quotient)
            } else {
                -quotient
            },
            remainder,
        )
    }
}

impl CeilingDivAssignNegMod<i32> for Integer {
    type ModOutput = u32;

    /// Divides an `Integer` by an `i32` in place, taking the the quotient and returning the
    /// remainder of the negative of the `Integer` divided by the `i32`. The quotient is rounded
    /// towards positive infinity, and the remainder is always non-negative and less than the
    /// absolute value of the divisor. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` - r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivAssignNegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(123), 36u32);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(-123), 36u32);
    ///     assert_eq!(x.to_string(), "-4");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(123), 87u32);
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(-123), 87u32);
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: i32) -> u32 {
        let other_abs = other.unsigned_abs();
        let remainder = if self.sign {
            self.abs.ceiling_div_assign_neg_mod(other_abs)
        } else {
            self.abs.div_assign_mod(other_abs)
        };
        self.sign ^= other < 0;
        if !self.sign && self.abs == 0 {
            self.sign = true;
        }
        remainder
    }
}

impl CeilingDivNegMod<Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the negative of the `i32` divided by the `Integer`. The quotient is
    /// rounded towards positive infinity, and the remainder is always non-negative and less than
    /// the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` - r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivNegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_neg_mod(Integer::from(123))), "(4, 36)");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_neg_mod(Integer::from(-123))), "(-4, 36)");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_neg_mod(Integer::from(123))), "(-3, 87)");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_neg_mod(Integer::from(-123))), "(3, 87)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: Integer) -> (Integer, Natural) {
        let self_abs = self.unsigned_abs();
        let (quotient, remainder) = if self >= 0 {
            self_abs.ceiling_div_neg_mod(other.abs)
        } else {
            let (quotient, remainder) = self_abs.div_mod(other.abs);
            (quotient, Natural::from(remainder))
        };
        (
            if (self >= 0) == other.sign {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl<'a> CeilingDivNegMod<&'a Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the negative of the `i32` divided by the `Integer`. The
    /// quotient is rounded towards positive infinity, and the remainder is always non-negative and
    /// less than the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` - r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivNegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_neg_mod(&Integer::from(123))), "(4, 36)");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_neg_mod(&Integer::from(-123))), "(-4, 36)");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_neg_mod(&Integer::from(123))),
    ///         "(-3, 87)");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_neg_mod(&Integer::from(-123))),
    ///         "(3, 87)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: &'a Integer) -> (Integer, Natural) {
        let self_abs = self.unsigned_abs();
        let (quotient, remainder) = if self >= 0 {
            self_abs.ceiling_div_neg_mod(&other.abs)
        } else {
            let (quotient, remainder) = self_abs.div_mod(&other.abs);
            (quotient, Natural::from(remainder))
        };
        (
            if (self >= 0) == other.sign {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl CeilingDivMod<i32> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the `Integer` divided by the `i32`. The quotient is rounded towards
    /// positive infinity, and the remainder is always non-positive and its absolute value is less
    /// than the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` + r and 0 <= -r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 + -36 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).ceiling_div_mod(123)), "(4, -36)");
    ///
    ///     // -4 * -123 + -36 = 456
    ///     assert_eq!(format!("{:?}", Integer::from(456).ceiling_div_mod(-123)), "(-4, -36)");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).ceiling_div_mod(123)), "(-3, -87)");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(format!("{:?}", Integer::from(-456).ceiling_div_mod(-123)), "(3, -87)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: i32) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -Natural::from(remainder))
    }
}

impl<'a> CeilingDivMod<i32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the `Integer` divided by the `i32`. The quotient is rounded
    /// towards positive infinity, and the remainder is always non-positive and its absolute value
    /// is less than the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` + r and 0 <= -r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 + -36 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).ceiling_div_mod(123)), "(4, -36)");
    ///
    ///     // -4 * -123 + -36 = 456
    ///     assert_eq!(format!("{:?}", (&Integer::from(456)).ceiling_div_mod(-123)), "(-4, -36)");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).ceiling_div_mod(123)), "(-3, -87)");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(format!("{:?}", (&Integer::from(-456)).ceiling_div_mod(-123)), "(3, -87)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: i32) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -Natural::from(remainder))
    }
}

impl CeilingDivAssignMod<i32> for Integer {
    type ModOutput = Integer;

    /// Divides an `Integer` by an `i32` in place, taking the quotient and returning the remainder
    /// of the `Integer` divided by the `i32`. The quotient is rounded towards positive infinity,
    /// and the remainder is always non-positive and its absolute value is less than the absolute
    /// value of the divisor. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` + r and 0 <= -r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 + -36 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.ceiling_div_assign_mod(123i32).to_string(), "-36");
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // -4 * -123+ -36 = 456
    ///     let mut x = Integer::from(456);
    ///     assert_eq!(x.ceiling_div_assign_mod(-123i32).to_string(), "-36");
    ///     assert_eq!(x.to_string(), "-4");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.ceiling_div_assign_mod(123i32).to_string(), "-87");
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     let mut x = Integer::from(-456);
    ///     assert_eq!(x.ceiling_div_assign_mod(-123i32).to_string(), "-87");
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn ceiling_div_assign_mod(&mut self, other: i32) -> Integer {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        -Natural::from(remainder)
    }
}

impl CeilingDivMod<Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the `i32` divided by the `Integer`. The quotient is rounded towards
    /// positive infinity, and the remainder is always non-positive and its absolute value is less
    /// than the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` + r and 0 <= -r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_mod(Integer::from(123))), "(4, -36)");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_mod(Integer::from(-123))), "(-4, -36)");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_mod(Integer::from(123))), "(-3, -87)");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_mod(Integer::from(-123))), "(3, -87)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -remainder)
    }
}

impl<'a> CeilingDivMod<&'a Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the `i32` divided by the `Integer`. The quotient is rounded
    /// towards positive infinity, and the remainder is always non-positive and its absolute value
    /// is less than the absolute value of the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` + r and 0 <= -r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingDivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_mod(&Integer::from(123))), "(4, -36)");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(format!("{:?}", 456.ceiling_div_mod(&Integer::from(-123))), "(-4, -36)");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_mod(&Integer::from(123))), "(-3, -87)");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(format!("{:?}", (-456).ceiling_div_mod(&Integer::from(-123))), "(3, -87)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -remainder)
    }
}
