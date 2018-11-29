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
    /// non-negative. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23u32).div_mod(10u32)), "(2, 3)");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).div_mod(10u32)), "(-3, 7)");
    /// }
    /// ```
    fn div_mod(mut self, other: u32) -> (Integer, u32) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

impl<'a> DivMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// is non-negative. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23u32)).div_mod(10u32)), "(2, 3)");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).div_mod(10u32)), "(-3, 7)");
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
    /// towards negative infinity, and the remainder is non-negative. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23u32);
    ///     assert_eq!(x.div_assign_mod(10u32), 3u32);
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.div_assign_mod(10u32), 7u32);
    ///     assert_eq!(x.to_string(), "-3");
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
    type ModOutput = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity, and the remainder has the
    /// same sign as the divisor. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= |r| < |`other`|.
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_mod(Integer::from(10))), "(2, 3)");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_mod(Integer::from(-10))), "(-3, -7)");
    /// }
    /// ```
    fn div_mod(self, other: Integer) -> (Integer, Integer) {
        if other.sign {
            let (quotient, remainder) = self.div_mod(other.abs);
            (Integer::from(quotient), Integer::from(remainder))
        } else {
            let (quotient, remainder) = self.ceiling_div_neg_mod(other.abs);
            (-Natural::from(quotient), -Natural::from(remainder))
        }
    }
}

impl<'a> DivMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// has the same sign as the divisor. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_mod(&Integer::from(10))), "(2, 3)");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_mod(&Integer::from(-10))), "(-3, -7)");
    /// }
    /// ```
    fn div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        if other.sign {
            let (quotient, remainder) = self.div_mod(&other.abs);
            (Integer::from(quotient), Integer::from(remainder))
        } else {
            let (quotient, remainder) = self.ceiling_div_neg_mod(&other.abs);
            (-Natural::from(quotient), -Natural::from(remainder))
        }
    }
}

impl DivRem<u32> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero and the remainder has the same sign as
    /// the dividend. The quotient and remainder satisfy `self` = q * `other` + r and
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23u32).div_rem(10u32)), "(2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).div_rem(10u32)), "(-2, -3)");
    /// }
    /// ```
    fn div_rem(mut self, other: u32) -> (Integer, Integer) {
        let remainder = self.div_assign_rem(other);
        (self, remainder)
    }
}

impl<'a> DivRem<u32> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the dividend. The quotient and remainder satisfy `self` = q * `other` + r and
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23u32)).div_rem(10u32)), "(2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).div_rem(10u32)), "(-2, -3)");
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
    /// towards zero and the remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < `other`.
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
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23u32);
    ///     assert_eq!(x.div_assign_rem(10u32).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.div_assign_rem(10u32).to_string(), "-3");
    ///     assert_eq!(x.to_string(), "-2");
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
    /// and remainder. The quotient is rounded towards zero and the remainder is non-negative. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < |`other`|.
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_rem(Integer::from(10))), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_rem(Integer::from(-10))), "(-2, 3)");
    /// }
    /// ```
    fn div_rem(self, other: Integer) -> (Integer, u32) {
        let (quotient, remainder) = self.div_mod(other.abs);
        (
            if other.sign {
                Integer::from(quotient)
            } else {
                -Natural::from(quotient)
            },
            remainder,
        )
    }
}

impl<'a> DivRem<&'a Integer> for u32 {
    type DivOutput = Integer;
    type RemOutput = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder is
    /// non-negative. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < |`other`|.
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_rem(&Integer::from(10))), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.div_rem(&Integer::from(-10))), "(-2, 3)");
    /// }
    /// ```
    fn div_rem(self, other: &'a Integer) -> (Integer, u32) {
        let (quotient, remainder) = self.div_mod(&other.abs);
        (
            if other.sign {
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
    /// and the remainder. The quotient is rounded towards positive infinity and the remainder is
    /// non-positive. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::CeilingDivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23u32).ceiling_div_mod(10u32)), "(3, -7)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).ceiling_div_mod(10u32)), "(-2, -3)");
    /// }
    /// ```
    fn ceiling_div_mod(mut self, other: u32) -> (Integer, Integer) {
        let remainder = self.ceiling_div_assign_mod(other);
        (self, remainder)
    }
}

impl<'a> CeilingDivMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// quotient and the The quotient is rounded towards positive infinity and the remainder is
    /// non-positive. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::CeilingDivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23u32)).ceiling_div_mod(10u32)), "(3, -7)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).ceiling_div_mod(10u32)), "(-2, -3)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: u32) -> (Integer, Integer) {
        let (quotient, remainder) = if self.sign {
            (&self.abs).ceiling_div_neg_mod(other)
        } else {
            (&self.abs).div_mod(other)
        };
        (
            Integer {
                sign: self.sign || quotient == 0,
                abs: quotient,
            },
            -Natural::from(remainder),
        )
    }
}

impl CeilingDivAssignMod<u32> for Integer {
    type ModOutput = Integer;

    /// Divides an `Integer` by a `u32` in place, taking the quotient and returning the remainder.
    /// The quotient is rounded towards positive infinity and the remainder is non-positive. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < `other`.
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
    ///     // 3 * 10 + -7 = 23
    ///     let mut x = Integer::from(23u32);
    ///     assert_eq!(x.ceiling_div_assign_mod(10u32), -7);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.ceiling_div_assign_mod(10u32), -3);
    ///     assert_eq!(x.to_string(), "-2");
    /// }
    /// ```
    fn ceiling_div_assign_mod(&mut self, other: u32) -> Integer {
        let remainder = -Natural::from(if self.sign {
            self.abs.ceiling_div_assign_neg_mod(other)
        } else {
            self.abs.div_assign_mod(other)
        });
        self.sign |= self.abs == 0;
        remainder
    }
}

impl CeilingDivMod<Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and the remainder. The quotient is rounded towards positive infinity and the remainder has
    /// the opposite sign of the divisor. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(format!("{:?}", 23u32.ceiling_div_mod(Integer::from(10u32))), "(3, -7)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.ceiling_div_mod(Integer::from(-10))), "(-2, 3)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        if other.sign {
            let (quotient, remainder) = self.ceiling_div_neg_mod(other.abs);
            (Integer::from(quotient), -Natural::from(remainder))
        } else {
            let (quotient, remainder) = self.div_mod(other.abs);
            (-Natural::from(quotient), Integer::from(remainder))
        }
    }
}

impl<'a> CeilingDivMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and the remainder. The quotient is rounded towards positive infinity and the
    /// remainder has the opposite sign of the divisor. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(format!("{:?}", 23u32.ceiling_div_mod(&Integer::from(10u32))), "(3, -7)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23u32.ceiling_div_mod(&Integer::from(-10))), "(-2, 3)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        if other.sign {
            let (quotient, remainder) = self.ceiling_div_neg_mod(&other.abs);
            (Integer::from(quotient), -Natural::from(remainder))
        } else {
            let (quotient, remainder) = self.div_mod(&other.abs);
            (-Natural::from(quotient), Integer::from(remainder))
        }
    }
}
