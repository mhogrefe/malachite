use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem,
};
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::CheckedFrom;

use integer::Integer;
use natural::Natural;
use platform::Limb;

impl DivMod<Limb> for Integer {
    type DivOutput = Integer;
    type ModOutput = Limb;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value and returning the quotient
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
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
    #[inline]
    fn div_mod(mut self, other: Limb) -> (Integer, Limb) {
        let remainder = self.div_assign_mod(other);
        (self, remainder)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivMod<u32> for Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    #[inline]
    fn div_mod(self, other: u32) -> (Integer, u32) {
        let (quotient, remainder) = self.div_mod(Limb::from(other));
        (quotient, u32::checked_from(remainder).unwrap())
    }
}

impl<'a> DivMod<Limb> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Limb;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference and returning the
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
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
    fn div_mod(self, other: Limb) -> (Integer, Limb) {
        if self.sign {
            let (quotient, remainder) = (&self.abs).div_mod(other);
            (Integer::from(quotient), remainder)
        } else {
            let (quotient, remainder) = (&self.abs).ceiling_div_neg_mod(other);
            (-quotient, remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = u32;

    #[inline]
    fn div_mod(self, other: u32) -> (Integer, u32) {
        let (quotient, remainder) = self.div_mod(Limb::from(other));
        (quotient, u32::checked_from(remainder).unwrap())
    }
}

impl DivAssignMod<Limb> for Integer {
    type ModOutput = Limb;

    /// Divides an `Integer` by a `Limb` in place, returning the remainder. The quotient is rounded
    /// towards negative infinity, and the remainder is non-negative. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
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
    fn div_assign_mod(&mut self, other: Limb) -> Limb {
        if self.sign {
            self.abs.div_assign_mod(other)
        } else {
            let remainder = self.abs.ceiling_div_assign_neg_mod(other);
            if self.abs == 0 as Limb {
                self.sign = true;
            }
            remainder
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssignMod<u32> for Integer {
    type ModOutput = u32;

    #[inline]
    fn div_assign_mod(&mut self, other: u32) -> u32 {
        u32::checked_from(self.div_assign_mod(Limb::from(other))).unwrap()
    }
}

impl DivMod<Integer> for Limb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity, and the remainder has the
    /// same sign as the divisor. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
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
            (-Natural::from(quotient), -remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivMod<Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn div_mod(self, other: Integer) -> (Integer, Integer) {
        Limb::from(self).div_mod(other)
    }
}

impl<'a> DivMod<&'a Integer> for Limb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// has the same sign as the divisor. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
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
            (-Natural::from(quotient), -remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        Limb::from(self).div_mod(other)
    }
}

impl DivRem<Limb> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value and returning the quotient
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
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
    #[inline]
    fn div_rem(mut self, other: Limb) -> (Integer, Integer) {
        let remainder = self.div_assign_rem(other);
        (self, remainder)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivRem<u32> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: u32) -> (Integer, Integer) {
        self.div_rem(Limb::from(other))
    }
}

impl<'a> DivRem<Limb> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference and returning the
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
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
    fn div_rem(self, other: Limb) -> (Integer, Integer) {
        let (quotient, remainder) = (&self.abs).div_mod(other);
        if self.sign {
            (Integer::from(quotient), Integer::from(remainder))
        } else {
            (-quotient, -Natural::from(remainder))
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivRem<u32> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: u32) -> (Integer, Integer) {
        self.div_rem(Limb::from(other))
    }
}

impl DivAssignRem<Limb> for Integer {
    type RemOutput = Integer;

    /// Divides an `Integer` by a `Limb` in place, returning the remainder. The quotient is rounded
    /// towards zero and the remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
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
    fn div_assign_rem(&mut self, other: Limb) -> Integer {
        let remainder = self.abs.div_assign_mod(other);
        if self.sign {
            Integer::from(remainder)
        } else {
            if self.abs == 0 as Limb {
                self.sign = true;
            }
            -Natural::from(remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssignRem<u32> for Integer {
    type RemOutput = Integer;

    #[inline]
    fn div_assign_rem(&mut self, other: u32) -> Integer {
        self.div_assign_rem(Limb::from(other))
    }
}

impl DivRem<Integer> for Limb {
    type DivOutput = Integer;
    type RemOutput = Limb;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero and the remainder is non-negative. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
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
    fn div_rem(self, other: Integer) -> (Integer, Limb) {
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

#[cfg(not(feature = "32_bit_limbs"))]
impl DivRem<Integer> for u32 {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: Integer) -> (Integer, Integer) {
        Limb::from(self).div_mod(other)
    }
}

impl<'a> DivRem<&'a Integer> for Limb {
    type DivOutput = Integer;
    type RemOutput = Limb;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder is
    /// non-negative. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < |`other`|.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
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
    fn div_rem(self, other: &'a Integer) -> (Integer, Limb) {
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

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivRem<&'a Integer> for u32 {
    type DivOutput = Integer;
    type RemOutput = Integer;

    #[inline]
    fn div_rem(self, other: &'a Integer) -> (Integer, Integer) {
        Limb::from(self).div_mod(other)
    }
}

impl CeilingDivMod<Limb> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value and returning the quotient
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
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
    #[inline]
    fn ceiling_div_mod(mut self, other: Limb) -> (Integer, Integer) {
        let remainder = self.ceiling_div_assign_mod(other);
        (self, remainder)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivMod<u32> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: u32) -> (Integer, Integer) {
        self.ceiling_div_mod(Limb::from(other))
    }
}

impl<'a> CeilingDivMod<Limb> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference and returning the
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
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
    fn ceiling_div_mod(self, other: Limb) -> (Integer, Integer) {
        let (quotient, remainder) = if self.sign {
            (&self.abs).ceiling_div_neg_mod(other)
        } else {
            (&self.abs).div_mod(other)
        };
        (
            Integer {
                sign: self.sign || quotient == 0 as Limb,
                abs: quotient,
            },
            -Natural::from(remainder),
        )
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingDivMod<u32> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: u32) -> (Integer, Integer) {
        self.ceiling_div_mod(Limb::from(other))
    }
}

impl CeilingDivAssignMod<Limb> for Integer {
    type ModOutput = Integer;

    /// Divides an `Integer` by a `Limb` in place, taking the quotient and returning the remainder.
    /// The quotient is rounded towards positive infinity and the remainder is non-positive. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignMod;
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
    fn ceiling_div_assign_mod(&mut self, other: Limb) -> Integer {
        let remainder = -Natural::from(if self.sign {
            self.abs.ceiling_div_assign_neg_mod(other)
        } else {
            self.abs.div_assign_mod(other)
        });
        self.sign |= self.abs == 0 as Limb;
        remainder
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivAssignMod<u32> for Integer {
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_assign_mod(&mut self, other: u32) -> Integer {
        self.ceiling_div_assign_mod(Limb::from(other))
    }
}

impl CeilingDivMod<Integer> for Limb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value and returning the quotient
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
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
            (Integer::from(quotient), -remainder)
        } else {
            let (quotient, remainder) = self.div_mod(other.abs);
            (-Natural::from(quotient), Integer::from(remainder))
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivMod<Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        Limb::from(self).ceiling_div_mod(other)
    }
}

impl<'a> CeilingDivMod<&'a Integer> for Limb {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference and returning the
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
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
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
            (Integer::from(quotient), -remainder)
        } else {
            let (quotient, remainder) = self.div_mod(&other.abs);
            (-Natural::from(quotient), Integer::from(remainder))
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingDivMod<&'a Integer> for u32 {
    type DivOutput = Integer;
    type ModOutput = Integer;

    #[inline]
    fn ceiling_div_mod(self, other: &'a Integer) -> (Integer, Integer) {
        Limb::from(self).ceiling_div_mod(other)
    }
}
