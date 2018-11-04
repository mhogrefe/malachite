use integer::Integer;
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem,
};
use natural::Natural;
use std::u32;

impl DivMod<u32> for Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity, and the remainder is
    /// always non-negative and less than the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!(format!("{:?}", Integer::from(456u32).div_mod(123u32)), "(3, 87)");
    ///
    ///     // -8,130,081,301 * 123 + 23 = -10^12
    ///     assert_eq!(format!("{:?}", (-Integer::trillion()).div_mod(123u32)),
    ///         "(-8130081301, 23)");
    /// }
    /// ```
    fn div_mod(self, other: u32) -> (Integer, u32) {
        if self.sign {
            let (quotient, remainder) = self.abs.div_mod(other);
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = self.abs.ceiling_div_neg_mod(other);
            (-quotient, remainder)
        }
    }
}

impl<'a> DivMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// is always non-negative and less than the divisor. In other words, returns (q, r), where
    /// `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!(format!("{:?}", (&Integer::from(456u32)).div_mod(123u32)), "(3, 87)");
    ///
    ///     // -8,130,081,301 * 123 + 23 = -10^12
    ///     assert_eq!(format!("{:?}", (&-Integer::trillion()).div_mod(123u32)),
    ///         "(-8130081301, 23)");
    /// }
    /// ```
    fn div_mod(self, other: u32) -> (Integer, u32) {
        if self.sign {
            let (quotient, remainder) = (&self.abs).div_mod(other);
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = (&self.abs).ceiling_div_neg_mod(other);
            (-quotient, remainder)
        }
    }
}

impl DivAssignMod<u32> for Integer {
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32` in place, returning the remainder. The quotient is rounded
    /// towards negative infinity, and the remainder is always non-negative and less than the
    /// divisor. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     assert_eq!(x.div_assign_mod(123u32), 87u32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -8,130,081,301 * 123 + 23 = -10^12
    ///     let mut x = -Integer::trillion();
    ///     assert_eq!(x.div_assign_mod(123u32), 23u32);
    ///     assert_eq!(x.to_string(), "-8130081301");
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: u32) -> u32 {
        if self.sign {
            self.abs.div_assign_mod(other)
        } else {
            let remainder = self.abs.ceiling_div_assign_neg_mod(other);
            if self.abs == 0 {
                self.sign = true;
            }
            remainder
        }
    }
}

impl DivMod<Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the quotient
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
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456u32.div_mod(Integer::from(-123))), "(-3, 87)");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(format!("{:?}", 123u32.div_mod(Integer::trillion())), "(0, 123)");
    /// }
    /// ```
    fn div_mod(self, other: Integer) -> (Integer, u32) {
        let non_negative = other >= 0;
        let (quotient, remainder) = self.div_mod(other.abs);
        (
            if non_negative {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl<'a> DivMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
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
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456u32.div_mod(&Integer::from(-123))), "(-3, 87)");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(format!("{:?}", 123u32.div_mod(&Integer::trillion())), "(0, 123)");
    /// }
    /// ```
    fn div_mod(self, other: &'a Integer) -> (Integer, u32) {
        let (quotient, remainder) = self.div_mod(&other.abs);
        (
            if *other >= 0 {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl DivRem<u32> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero, and the remainder has the same sign as
    /// the dividend and its absolute value is less than the divisor. In other words, returns
    /// (q, r), where `self` = q * `other` + r, (r = 0 or sign(r) = sign(`self`)), and
    /// 0 <= |r| < `other`.
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
    ///     assert_eq!(format!("{:?}", Integer::from(456u32).div_rem(123u32)), "(3, 87)");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!(format!("{:?}", (-Integer::trillion()).div_rem(123u32)),
    ///         "(-8130081300, -100)");
    /// }
    /// ```
    fn div_rem(self, other: u32) -> (Integer, Integer) {
        let (quotient, remainder) = self.abs.div_mod(other);
        if self.sign {
            (Integer::from(quotient), Integer::from(remainder))
        } else {
            (-quotient, -Natural::from(remainder))
        }
    }
}

impl<'a> DivRem<u32> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero, and the remainder has the same
    /// sign as the dividend and its absolute value is less than the divisor. In other words,
    /// returns (q, r), where `self` = q * `other` + r, (r = 0 or sign(r) = sign(`self`)), and
    /// 0 <= |r| < `other`.
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
    ///     assert_eq!(format!("{:?}", (&Integer::from(456u32)).div_rem(123u32)), "(3, 87)");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!(format!("{:?}", (&-Integer::trillion()).div_rem(123u32)),
    ///         "(-8130081300, -100)");
    /// }
    /// ```
    fn div_rem(self, other: u32) -> (Integer, Integer) {
        let (quotient, remainder) = (&self.abs).div_mod(other);
        if self.sign {
            (Integer::from(quotient), Integer::from(remainder))
        } else {
            (-quotient, -Natural::from(remainder))
        }
    }
}

impl DivAssignRem<u32> for Integer {
    type RemOutput = Integer;

    /// Divides an `Integer` by a `u32` in place, returning the remainder. The quotient is rounded
    /// towards zero, and the remainder has the same sign as the dividend and its absolute value is
    /// less than the divisor. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` + r, (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     assert_eq!(x.div_assign_rem(123u32).to_string(), "87");
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     let mut x = -Integer::trillion();
    ///     assert_eq!(x.div_assign_rem(123u32).to_string(), "-100");
    ///     assert_eq!(x.to_string(), "-8130081300");
    /// }
    /// ```
    fn div_assign_rem(&mut self, other: u32) -> Integer {
        let remainder = self.abs.div_assign_mod(other);
        if self.sign {
            Integer::from(remainder)
        } else {
            if self.abs == 0 {
                self.sign = true;
            }
            -Natural::from(remainder)
        }
    }
}

impl DivRem<Integer> for u32 {
    type DivOutput = Integer;
    type RemOutput = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero, and the remainder has the same sign as
    /// the dividend and its absolute value is less than the divisor. In other words, returns
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456u32.div_rem(Integer::from(-123))), "(-3, 87)");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(format!("{:?}", 123u32.div_rem(Integer::trillion())), "(0, 123)");
    /// }
    /// ```
    fn div_rem(self, other: Integer) -> (Integer, u32) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<&'a Integer> for u32 {
    type DivOutput = Integer;
    type RemOutput = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero, and the remainder has the same
    /// sign as the dividend and its absolute value is less than the divisor. In other words,
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", 456u32.div_rem(&Integer::from(-123))), "(-3, 87)");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(format!("{:?}", 123u32.div_rem(&Integer::trillion())), "(0, 123)");
    /// }
    /// ```
    fn div_rem(self, other: &'a Integer) -> (Integer, u32) {
        self.div_mod(other)
    }
}

impl CeilingDivNegMod<u32> for Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the negative of the `Integer` divided by the `u32`. The quotient is
    /// rounded towards positive infinity, and the remainder is always non-negative and less than
    /// the divisor. In other words, returns (q, r), where `self` = q * `other` - r and
    /// 0 <= r < `other`.
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
    ///     assert_eq!(format!("{:?}", Integer::from(456u32).ceiling_div_neg_mod(123u32)),
    ///         "(4, 36)");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!(format!("{:?}", (-Integer::trillion()).ceiling_div_neg_mod(123u32)),
    ///         "(-8130081300, 100)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: u32) -> (Integer, u32) {
        if self.sign {
            let (quotient, remainder) = self.abs.ceiling_div_neg_mod(other);
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = self.abs.div_mod(other);
            (-quotient, remainder)
        }
    }
}

impl<'a> CeilingDivNegMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the negative of the `Integer` divided by the `u32`. The
    /// quotient is rounded towards positive infinity, and the remainder is always non-negative and
    /// less than the divisor. In other words, returns (q, r), where `self` = q * `other` - r and
    /// 0 <= r < `other`.
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
    ///     assert_eq!(format!("{:?}", (&Integer::from(456u32)).ceiling_div_neg_mod(123u32)),
    ///         "(4, 36)");
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     assert_eq!(format!("{:?}", (&-Integer::trillion()).ceiling_div_neg_mod(123u32)),
    ///         "(-8130081300, 100)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: u32) -> (Integer, u32) {
        if self.sign {
            let (quotient, remainder) = (&self.abs).ceiling_div_neg_mod(other);
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = (&self.abs).div_mod(other);
            (-quotient, remainder)
        }
    }
}

impl CeilingDivAssignNegMod<u32> for Integer {
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32` in place, taking the the quotient and returning the
    /// remainder of the negative of the `Integer` divided by the `u32`. The quotient is rounded
    /// towards positive infinity, and the remainder is always non-negative and less than the
    /// divisor. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` - r and 0 <= r < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(123u32), 36);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     let mut x = -Integer::trillion();
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(123u32), 100);
    ///     assert_eq!(x.to_string(), "-8130081300");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: u32) -> u32 {
        if self.sign {
            self.abs.ceiling_div_assign_neg_mod(other)
        } else {
            let remainder = self.abs.div_assign_mod(other);
            if self.abs == 0 {
                self.sign = true;
            }
            remainder
        }
    }
}

impl CeilingDivNegMod<Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the negative of the `u32` divided by the `Integer`. The quotient is
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
    ///     assert_eq!(format!("{:?}", 456u32.ceiling_div_neg_mod(Integer::from(-123))),
    ///         "(-4, 36)");
    ///
    ///     // 1 * 10^12 - 999,999,999,877 = 123
    ///     assert_eq!(format!("{:?}", 123u32.ceiling_div_neg_mod(Integer::trillion())),
    ///         "(1, 999999999877)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: Integer) -> (Integer, Natural) {
        let non_negative = other >= 0;
        let (quotient, remainder) = self.ceiling_div_neg_mod(other.abs);
        (
            if non_negative {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl<'a> CeilingDivNegMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the negative of the `u32` divided by the `Integer`. The
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
    ///     assert_eq!(format!("{:?}", 456u32.ceiling_div_neg_mod(&Integer::from(-123))),
    ///         "(-4, 36)");
    ///
    ///     // 1 * 10^12 - 999,999,999,877 = 123
    ///     assert_eq!(format!("{:?}", 123u32.ceiling_div_neg_mod(&Integer::trillion())),
    ///         "(1, 999999999877)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: &'a Integer) -> (Integer, Natural) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(&other.abs);
        (
            if *other >= 0 {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl CeilingDivMod<u32> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the `Integer` divided by the `u32`. The quotient is rounded towards
    /// positive infinity, and the remainder is always non-positive and its absolute value is less
    /// than the divisor. In other words, returns (q, r), where `self` = q * `other` + r and
    /// 0 <= -r < `other`.
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
    ///     assert_eq!(format!("{:?}", Integer::from(456u32).ceiling_div_mod(123u32)), "(4, -36)");
    ///
    ///     // -8,130,081,300 * 123 + -100 = -10^12
    ///     assert_eq!(format!("{:?}", (-Integer::trillion()).ceiling_div_mod(123u32)),
    ///         "(-8130081300, -100)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: u32) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -Natural::from(remainder))
    }
}

impl<'a> CeilingDivMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the `Integer` divided by the `u32`. The quotient is rounded
    /// towards positive infinity, and the remainder is always non-positive and its absolute value
    /// is less than the divisor. In other words, returns (q, r), where `self` = q * `other` + r and
    /// 0 <= -r < `other`.
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
    ///     assert_eq!(format!("{:?}", (&Integer::from(456u32)).ceiling_div_mod(123u32)),
    ///         "(4, -36)");
    ///
    ///     // 8,130,081,301 * 123 + -23 = 10^12
    ///     assert_eq!(format!("{:?}", (&-Integer::trillion()).ceiling_div_mod(123u32)),
    ///         "(-8130081300, -100)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: u32) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -Natural::from(remainder))
    }
}

impl CeilingDivAssignMod<u32> for Integer {
    type ModOutput = Integer;

    /// Divides an `Integer` by a `u32` in place, taking the quotient and returning the remainder of
    /// the `Integer` divided by the `u32`. The quotient is rounded towards positive infinity, and
    /// the remainder is always non-positive and its absolute value is less than the divisor. In
    /// other words, replaces `self` with q and returns r, where `self` = q * `other` + r and
    /// 0 <= -r < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     assert_eq!(x.ceiling_div_assign_mod(123u32), -36);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // 8,130,081,301 * 123 + -23 = 10^12
    ///     let mut x = -Integer::trillion();
    ///     assert_eq!(x.ceiling_div_assign_mod(123u32), -100);
    ///     assert_eq!(x.to_string(), "-8130081300");
    /// }
    /// ```
    fn ceiling_div_assign_mod(&mut self, other: u32) -> Integer {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        -Natural::from(remainder)
    }
}

impl CeilingDivMod<Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and the remainder of the `u32` divided by the `Integer`. The quotient is rounded towards
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
    ///     assert_eq!(format!("{:?}", 456u32.ceiling_div_mod(Integer::from(123u32))), "(4, -36)");
    ///
    ///     // 1 * 10^12 + -999,999,999,877 = 123
    ///     assert_eq!(format!("{:?}", 123u32.ceiling_div_mod(-Integer::trillion())),
    ///         "(-1, -999999999877)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -remainder)
    }
}

impl<'a> CeilingDivMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and the remainder of the `u32` divided by the `Integer`. The quotient is rounded
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
    ///     assert_eq!(format!("{:?}", 456u32.ceiling_div_mod(&Integer::from(123u32))), "(4, -36)");
    ///
    ///     // 1 * 10^12 + -999,999,999,877 = 123
    ///     assert_eq!(format!("{:?}", 123u32.ceiling_div_mod(&-Integer::trillion())),
    ///         "(-1, -999999999877)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(other);
        (quotient, -remainder)
    }
}
