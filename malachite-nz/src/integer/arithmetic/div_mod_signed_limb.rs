use integer::Integer;
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem, UnsignedAbs,
};
use natural::Natural;
use platform::SignedLimb;

impl DivMod<SignedLimb> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// has the same sign as the divisor. The quotient and remainder satisfy
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23).div_mod(10)), "(2, 3)");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23).div_mod(-10)), "(-3, -7)");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).div_mod(10)), "(-3, 7)");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).div_mod(-10)), "(2, -3)");
    /// }
    /// ```
    #[inline]
    fn div_mod(mut self, other: SignedLimb) -> (Integer, Integer) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivMod<i32> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn div_mod(self, other: i32) -> (Integer, Integer) {
        self.div_mod(SignedLimb::from(other))
    }
}

impl<'a> DivMod<SignedLimb> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// has the same sign as the divisor. The quotient and remainder satisfy
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23)).div_mod(10)), "(2, 3)");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23)).div_mod(-10)), "(-3, -7)");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).div_mod(10)), "(-3, 7)");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).div_mod(-10)), "(2, -3)");
    /// }
    /// ```
    fn div_mod(self, other: SignedLimb) -> (Integer, Integer) {
        let (quotient, remainder) = if self.sign == (other >= 0) {
            let (quotient, remainder) = (&self.abs).div_mod(other.unsigned_abs());
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = (&self.abs).ceiling_div_neg_mod(other.unsigned_abs());
            (-quotient, remainder)
        };
        (
            quotient,
            if other >= 0 {
                Integer::from(remainder)
            } else {
                -Natural::from(remainder)
            },
        )
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivMod<i32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn div_mod(self, other: i32) -> (Integer, Integer) {
        self.div_mod(SignedLimb::from(other))
    }
}

impl DivAssignMod<SignedLimb> for Integer {
    type ModOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb` in place, returning the remainder. The quotient is
    /// rounded towards negative infinity, and the remainder has the same sign as the divisor. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    ///     let mut x = Integer::from(23);
    ///     assert_eq!(x.div_assign_mod(10i32).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     assert_eq!(x.div_assign_mod(-10i32).to_string(), "-7");
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.div_assign_mod(10i32).to_string(), "7");
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.div_assign_mod(-10i32).to_string(), "-3");
    ///     assert_eq!(x.to_string(), "2");
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: SignedLimb) -> Integer {
        let remainder = if self.sign == (other >= 0) {
            self.sign = true;
            self.abs.div_assign_mod(other.unsigned_abs())
        } else {
            let remainder = self.abs.ceiling_div_assign_neg_mod(other.unsigned_abs());
            if self.abs != 0 {
                self.sign = false;
            }
            remainder
        };
        if other >= 0 {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivAssignMod<i32> for Integer {
    type ModOutput = Integer;

    #[inline]
    fn div_assign_mod(&mut self, other: i32) -> Integer {
        self.div_assign_mod(SignedLimb::from(other))
    }
}

impl DivMod<Integer> for SignedLimb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value and returning the
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
    ///     assert_eq!(format!("{:?}", 23.div_mod(Integer::from(10))), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.div_mod(Integer::from(-10))), "(-3, -7)");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_mod(Integer::from(10))), "(-3, 7)");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_mod(Integer::from(-10))), "(2, -3)");
    /// }
    /// ```
    fn div_mod(self, other: Integer) -> (Integer, Integer) {
        let (quotient, remainder) = if (self >= 0) == other.sign {
            let (quotient, remainder) = self.unsigned_abs().div_mod(other.abs);
            (Integer::from(quotient), Natural::from(remainder))
        } else {
            let (quotient, remainder) = self.unsigned_abs().ceiling_div_neg_mod(other.abs);
            (-Natural::from(quotient), remainder)
        };
        (
            quotient,
            if other.sign {
                Integer::from(remainder)
            } else {
                -remainder
            },
        )
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivMod<Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn div_mod(self, other: Integer) -> (Integer, Integer) {
        SignedLimb::from(self).div_mod(other)
    }
}

impl<'a> DivMod<&'a Integer> for SignedLimb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference and returning the
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
    ///     assert_eq!(format!("{:?}", 23.div_mod(&Integer::from(10))), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.div_mod(&Integer::from(-10))), "(-3, -7)");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_mod(&Integer::from(10))), "(-3, 7)");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_mod(&Integer::from(-10))), "(2, -3)");
    /// }
    /// ```
    fn div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        let (quotient, remainder) = if (self >= 0) == other.sign {
            let (quotient, remainder) = self.unsigned_abs().div_mod(&other.abs);
            (Integer::from(quotient), Natural::from(remainder))
        } else {
            let (quotient, remainder) = self.unsigned_abs().ceiling_div_neg_mod(&other.abs);
            (-Natural::from(quotient), remainder)
        };
        (
            quotient,
            if other.sign {
                Integer::from(remainder)
            } else {
                -remainder
            },
        )
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivMod<&'a Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        SignedLimb::from(self).div_mod(other)
    }
}

impl DivRem<SignedLimb> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the dividend. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23).div_rem(10)), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23).div_rem(-10)), "(-2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).div_rem(10)), "(-2, -3)");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).div_rem(-10)), "(2, -3)");
    /// }
    /// ```
    #[inline]
    fn div_rem(mut self, other: SignedLimb) -> (Integer, Integer) {
        let remainder = self.div_assign_rem(other);
        (self, remainder)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivRem<i32> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: i32) -> (Integer, Integer) {
        self.div_rem(SignedLimb::from(other))
    }
}

impl<'a> DivRem<SignedLimb> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the dividend. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= |r| < |`other`|.
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
    ///     assert_eq!(format!("{:?}", (&Integer::from(23)).div_rem(10)), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23)).div_rem(-10)), "(-2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).div_rem(10)), "(-2, -3)");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).div_rem(-10)), "(2, -3)");
    /// }
    /// ```
    fn div_rem(self, other: SignedLimb) -> (Integer, Integer) {
        let (quotient, remainder) = (&self.abs).div_mod(other.unsigned_abs());
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

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivRem<i32> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: i32) -> (Integer, Integer) {
        self.div_rem(SignedLimb::from(other))
    }
}

impl DivAssignRem<SignedLimb> for Integer {
    type RemOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb` in place, returning the remainder. The quotient is
    /// rounded towards zero and the remainder has the same sign as the dividend. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    ///     let mut x = Integer::from(23);
    ///     assert_eq!(x.div_assign_rem(10i32).to_string(), "3");
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     assert_eq!(x.div_assign_rem(-10i32).to_string(), "3");
    ///     assert_eq!(x.to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.div_assign_rem(10i32).to_string(), "-3");
    ///     assert_eq!(x.to_string(), "-2");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.div_assign_rem(-10i32).to_string(), "-3");
    ///     assert_eq!(x.to_string(), "2");
    /// }
    /// ```
    fn div_assign_rem(&mut self, other: SignedLimb) -> Integer {
        let remainder = self.abs.div_assign_mod(other.unsigned_abs());
        let remainder = if self.sign {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        };
        self.sign = self.sign == (other >= 0) || self.abs == 0;
        remainder
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivAssignRem<i32> for Integer {
    type RemOutput = Integer;

    #[inline]
    fn div_assign_rem(&mut self, other: i32) -> Integer {
        self.div_assign_rem(SignedLimb::from(other))
    }
}

impl DivRem<Integer> for SignedLimb {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the dividend. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.div_rem(Integer::from(10))), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.div_rem(Integer::from(-10))), "(-2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_rem(Integer::from(10))), "(-2, -3)");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_rem(Integer::from(-10))), "(2, -3)");
    /// }
    /// ```
    fn div_rem(self, other: Integer) -> (Integer, Integer) {
        let (quotient, remainder) = self.unsigned_abs().div_mod(other.abs);
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

#[cfg(feature = "64_bit_limbs")]
impl DivRem<Integer> for i32 {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: Integer) -> (Integer, Integer) {
        SignedLimb::from(self).div_mod(other)
    }
}

impl<'a> DivRem<&'a Integer> for SignedLimb {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the dividend. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.div_rem(&Integer::from(10))), "(2, 3)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.div_rem(&Integer::from(-10))), "(-2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_rem(&Integer::from(10))), "(-2, -3)");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (-23).div_rem(&Integer::from(-10))), "(2, -3)");
    /// }
    /// ```
    fn div_rem(self, other: &'a Integer) -> (Integer, Integer) {
        let (quotient, remainder) = self.unsigned_abs().div_mod(&other.abs);
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

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivRem<&'a Integer> for i32 {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: &'a Integer) -> (Integer, Integer) {
        SignedLimb::from(self).div_mod(other)
    }
}

impl CeilingDivMod<SignedLimb> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value and returning the
    /// quotient and remainder. The quotient is rounded towards positive infinity and the remainder
    /// has the opposite sign of the divisor. The quotient and remainder satisfy
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
    ///     assert_eq!(format!("{:?}", Integer::from(23).ceiling_div_mod(10)), "(3, -7)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Integer::from(23).ceiling_div_mod(-10)), "(-2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).ceiling_div_mod(10)), "(-2, -3)");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(format!("{:?}", Integer::from(-23).ceiling_div_mod(-10)), "(3, 7)");
    /// }
    /// ```
    #[inline]
    fn ceiling_div_mod(mut self, other: SignedLimb) -> (Integer, Integer) {
        let remainder = self.ceiling_div_assign_mod(other);
        (self, remainder)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CeilingDivMod<i32> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: i32) -> (Integer, Integer) {
        self.ceiling_div_mod(SignedLimb::from(other))
    }
}

impl<'a> CeilingDivMod<SignedLimb> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards positive infinity and the remainder
    /// has the opposite sign of the divisor. The quotient and remainder satisfy
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
    ///     assert_eq!(format!("{:?}", (&Integer::from(23)).ceiling_div_mod(10)), "(3, -7)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Integer::from(23)).ceiling_div_mod(-10)), "(-2, 3)");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).ceiling_div_mod(10)), "(-2, -3)");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (&Integer::from(-23)).ceiling_div_mod(-10)), "(3, 7)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: SignedLimb) -> (Integer, Integer) {
        let (quotient, remainder) = if self.sign == (other >= 0) {
            let (quotient, remainder) = (&self.abs).ceiling_div_neg_mod(other.unsigned_abs());
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = (&self.abs).div_mod(other.unsigned_abs());
            (-quotient, remainder)
        };
        (
            quotient,
            if other >= 0 {
                -Natural::from(remainder)
            } else {
                Integer::from(remainder)
            },
        )
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CeilingDivMod<i32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: i32) -> (Integer, Integer) {
        self.ceiling_div_mod(SignedLimb::from(other))
    }
}

impl CeilingDivAssignMod<SignedLimb> for Integer {
    type ModOutput = Integer;

    /// Divides an `Integer` by a `SignedLimb` in place, taking the quotient and returning the
    /// remainder. The quotient is rounded towards positive infinity and the remainder has the
    /// opposite sign of the divisor. The quotient and remainder satisfy `self` = q * `other` + r
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
    /// use malachite_base::num::CeilingDivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     assert_eq!(x.ceiling_div_assign_mod(10i32).to_string(), "-7");
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     assert_eq!(x.ceiling_div_assign_mod(-10i32).to_string(), "3");
    ///     assert_eq!(x.to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.ceiling_div_assign_mod(10i32).to_string(), "-3");
    ///     assert_eq!(x.to_string(), "-2");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     assert_eq!(x.ceiling_div_assign_mod(-10i32).to_string(), "7");
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn ceiling_div_assign_mod(&mut self, other: SignedLimb) -> Integer {
        let remainder = if self.sign == (other >= 0) {
            self.sign = true;
            self.abs.ceiling_div_assign_neg_mod(other.unsigned_abs())
        } else {
            let remainder = self.abs.div_assign_mod(other.unsigned_abs());
            self.sign = self.abs == 0;
            remainder
        };
        if other >= 0 {
            -Natural::from(remainder)
        } else {
            Integer::from(remainder)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CeilingDivAssignMod<i32> for Integer {
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_assign_mod(&mut self, other: i32) -> Integer {
        self.ceiling_div_assign_mod(SignedLimb::from(other))
    }
}

impl CeilingDivMod<Integer> for SignedLimb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value and returning the
    /// quotient and remainder. The quotient is rounded towards positive infinity and the remainder
    /// has the opposite sign of the divisor. The quotient and remainder satisfy
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
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(format!("{:?}", 23.ceiling_div_mod(Integer::from(10))), "(3, -7)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.ceiling_div_mod(Integer::from(-10))), "(-2, 3)");
    ///
    ///     // -3 * 10 - 3 = -23
    ///     assert_eq!(format!("{:?}", (-23).ceiling_div_mod(Integer::from(10))), "(-2, -3)");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (-23).ceiling_div_mod(Integer::from(-10))), "(3, 7)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        let result_sign = (self >= 0) == other.sign;
        let (quotient, remainder) = if result_sign {
            self.unsigned_abs().ceiling_div_neg_mod(other.abs)
        } else {
            let (quotient, remainder) = self.unsigned_abs().div_mod(other.abs);
            (quotient, Natural::from(remainder))
        };
        let quotient = Integer {
            sign: result_sign || quotient == 0,
            abs: Natural::from(quotient),
        };
        let remainder = if other.sign {
            -remainder
        } else {
            Integer::from(remainder)
        };
        (quotient, remainder)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CeilingDivMod<Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        SignedLimb::from(self).ceiling_div_mod(other)
    }
}

impl<'a> CeilingDivMod<&'a Integer> for SignedLimb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards positive infinity and the remainder
    /// has the opposite sign of the divisor. The quotient and remainder satisfy
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
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(format!("{:?}", 23.ceiling_div_mod(&Integer::from(10))), "(3, -7)");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(format!("{:?}", 23.ceiling_div_mod(&Integer::from(-10))), "(-2, 3)");
    ///
    ///     // -3 * 10 - 3 = -23
    ///     assert_eq!(format!("{:?}", (-23).ceiling_div_mod(&Integer::from(10))), "(-2, -3)");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(format!("{:?}", (-23).ceiling_div_mod(&Integer::from(-10))), "(3, 7)");
    /// }
    /// ```
    fn ceiling_div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        let result_sign = (self >= 0) == other.sign;
        let (quotient, remainder) = if result_sign {
            self.unsigned_abs().ceiling_div_neg_mod(&other.abs)
        } else {
            let (quotient, remainder) = self.unsigned_abs().div_mod(&other.abs);
            (quotient, Natural::from(remainder))
        };
        let quotient = Integer {
            sign: result_sign || quotient == 0,
            abs: Natural::from(quotient),
        };
        let remainder = if other.sign {
            -remainder
        } else {
            Integer::from(remainder)
        };
        (quotient, remainder)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CeilingDivMod<&'a Integer> for i32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        SignedLimb::from(self).ceiling_div_mod(other)
    }
}
