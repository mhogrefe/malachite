use std::ops::{Rem, RemAssign};

use malachite_base::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign,
};
#[cfg(feature = "64_bit_limbs")]
use malachite_base::num::conversion::traits::CheckedFrom;

use integer::Integer;
use natural::Natural;
use platform::Limb;

impl Mod<Limb> for Integer {
    type Output = Limb;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value and returning the remainder.
    /// The remainder is non-negative. The quotient and remainder satisfy `self` = q * `other` + r
    /// and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Integer::from(23u32).mod_op(10u32), 3);
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(10), 7);
    /// }
    /// ```
    fn mod_op(self, other: Limb) -> Limb {
        if self.sign {
            self.abs % other
        } else {
            self.abs.neg_mod(other)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Mod<u32> for Integer {
    type Output = u32;

    #[inline]
    fn mod_op(self, other: u32) -> u32 {
        u32::checked_from(self.mod_op(Limb::from(other))).unwrap()
    }
}

impl<'a> Mod<Limb> for &'a Integer {
    type Output = Limb;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is non-negative. The quotient and remainder satisfy
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23u32)).mod_op(10u32), 3);
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(10), 7);
    /// }
    /// ```
    fn mod_op(self, other: Limb) -> Limb {
        if self.sign {
            &self.abs % other
        } else {
            (&self.abs).neg_mod(other)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Mod<u32> for &'a Integer {
    type Output = u32;

    #[inline]
    fn mod_op(self, other: u32) -> u32 {
        u32::checked_from(self.mod_op(Limb::from(other))).unwrap()
    }
}

impl ModAssign<Limb> for Integer {
    /// Divides an `Integer` by a `Limb`, replacing the `Integer` by the remainder. The remainder is
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23u32);
    ///     x.mod_assign(10u32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(10u32);
    ///     assert_eq!(x.to_string(), "7");
    /// }
    /// ```
    fn mod_assign(&mut self, other: Limb) {
        if self.sign {
            self.abs.mod_assign(other);
        } else {
            self.abs.neg_mod_assign(other);
        }
        self.sign = true;
    }
}

#[cfg(feature = "64_bit_limbs")]
impl ModAssign<u32> for Integer {
    #[inline]
    fn mod_assign(&mut self, other: u32) {
        self.mod_assign(Limb::from(other));
    }
}

impl Mod<Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder is has the same sign as the divisor. The quotient and remainder satisfy
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23u32.mod_op(Integer::from(10u32)), 3);
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(23u32.mod_op(Integer::from(-10)), -7);
    /// }
    /// ```
    fn mod_op(self, other: Integer) -> Integer {
        if other.sign {
            Integer::from(self % other.abs)
        } else {
            -self.neg_mod(other.abs)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Mod<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn mod_op(self, other: Integer) -> Integer {
        Limb::from(self).mod_op(other)
    }
}

impl<'a> Mod<&'a Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23u32.mod_op(&Integer::from(10u32)), 3);
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(23u32.mod_op(&Integer::from(-10)), -7);
    /// }
    /// ```
    fn mod_op(self, other: &'a Integer) -> Integer {
        if other.sign {
            Integer::from(self % &other.abs)
        } else {
            -self.neg_mod(&other.abs)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Mod<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn mod_op(self, other: &'a Integer) -> Integer {
        Limb::from(self).mod_op(other)
    }
}

impl Rem<Limb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value and returning the remainder.
    /// The remainder has the same sign as the dividend. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < `other`.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Integer::from(23u32) % 10u32).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((Integer::from(-23) % 10u32).to_string(), "-3");
    /// }
    /// ```
    #[inline]
    fn rem(mut self, other: Limb) -> Integer {
        self %= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Rem<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn rem(self, other: u32) -> Integer {
        self % Limb::from(other)
    }
}

impl<'a> Rem<Limb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < `other`.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23u32) % 10u32).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23) % 10u32).to_string(), "-3");
    /// }
    /// ```
    fn rem(self, other: Limb) -> Integer {
        if self.sign {
            Integer::from(&self.abs % other)
        } else {
            -Natural::from(&self.abs % other)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Rem<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn rem(self, other: u32) -> Integer {
        self % Limb::from(other)
    }
}

impl RemAssign<Limb> for Integer {
    /// Divides an `Integer` by a `Limb`, replacing the `Integer` by the remainder. The remainder
    /// has the same sign as the dividend. The remainder has the same sign as the dividend. The
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23u32);
    ///     x %= 10u32;
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x %= 10u32;
    ///     assert_eq!(x.to_string(), "-3");
    /// }
    /// ```
    fn rem_assign(&mut self, other: Limb) {
        self.abs.rem_assign(other);
        self.sign |= self.abs == 0 as Limb;
    }
}

#[cfg(feature = "64_bit_limbs")]
impl RemAssign<u32> for Integer {
    #[inline]
    fn rem_assign(&mut self, other: u32) {
        *self %= Limb::from(other);
    }
}

impl Rem<Integer> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder is non-negative. The quotient and remainder satisfy `self` = q * `other` + r
    /// and 0 <= r < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23u32 % Integer::from(10u32), 3);
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(23u32 % Integer::from(-10), 3);
    /// }
    /// ```
    fn rem(self, other: Integer) -> Limb {
        self % other.abs
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Rem<Integer> for u32 {
    type Output = u32;

    #[inline]
    fn rem(self, other: Integer) -> u32 {
        u32::checked_from(Limb::from(self) % other).unwrap()
    }
}

impl<'a> Rem<&'a Integer> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is non-negative. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23u32 % &Integer::from(10u32), 3);
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(23u32 % &Integer::from(-10), 3);
    /// }
    /// ```
    fn rem(self, other: &'a Integer) -> Limb {
        self % &other.abs
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Rem<&'a Integer> for u32 {
    type Output = u32;

    #[inline]
    fn rem(self, other: &'a Integer) -> u32 {
        u32::checked_from(Limb::from(self) % other).unwrap()
    }
}

impl RemAssign<Integer> for Limb {
    /// Divides a `Limb` by an `Integer` in place, taking the `Integer` by value and replacing the
    /// `Limb` with the remainder. The remainder is non-negative. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23u32;
    ///     n %= Integer::from(10u32);
    ///     assert_eq!(n, 3);
    ///
    ///     // -2 * -10 + 3 = 23
    ///     let mut n = 23u32;
    ///     n %= Integer::from(-10);
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    fn rem_assign(&mut self, other: Integer) {
        *self %= other.abs;
    }
}

#[cfg(feature = "64_bit_limbs")]
impl RemAssign<Integer> for u32 {
    fn rem_assign(&mut self, other: Integer) {
        *self %= other.abs;
    }
}

impl<'a> RemAssign<&'a Integer> for Limb {
    /// Divides a `Limb` by an `Integer` in place, taking the `Integer` by reference and replacing
    /// the `Limb` with the remainder. The remainder is non-negative. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23u32;
    ///     n %= &Integer::from(10u32);
    ///     assert_eq!(n, 3);
    ///
    ///     // -2 * -10 + 3 = 23
    ///     let mut n = 23u32;
    ///     n %= &Integer::from(-10);
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    fn rem_assign(&mut self, other: &'a Integer) {
        *self %= &other.abs;
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> RemAssign<&'a Integer> for u32 {
    fn rem_assign(&mut self, other: &'a Integer) {
        *self %= &other.abs;
    }
}

impl CeilingMod<Limb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value and returning the remainder.
    /// The remainder is non-positive. The quotient and remainder satisfy `self` = q * `other` + r
    /// and 0 <= |r| < `other`.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(Integer::from(23u32).ceiling_mod(10u32).to_string(), "-7");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(10u32).to_string(), "-3");
    /// }
    /// ```
    #[inline]
    fn ceiling_mod(mut self, other: Limb) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CeilingMod<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn ceiling_mod(self, other: u32) -> Integer {
        self.ceiling_mod(Limb::from(other))
    }
}

impl<'a> CeilingMod<Limb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is non-positive. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < `other`.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!((&Integer::from(23u32)).ceiling_mod(10u32).to_string(), "-7");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(10u32).to_string(), "-3");
    /// }
    /// ```
    fn ceiling_mod(self, other: Limb) -> Integer {
        -Natural::from(if self.sign {
            (&self.abs).neg_mod(other)
        } else {
            &self.abs % other
        })
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CeilingMod<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn ceiling_mod(self, other: u32) -> Integer {
        self.ceiling_mod(Limb::from(other))
    }
}

impl CeilingModAssign<Limb> for Integer {
    /// Divides an `Integer` by a `Limb`, replacing the `Integer` by the remainder. The remainder is
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
    /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     let mut x = Integer::from(23u32);
    ///     x.ceiling_mod_assign(10u32);
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(10u32);
    ///     assert_eq!(x.to_string(), "-3");
    /// }
    /// ```
    fn ceiling_mod_assign(&mut self, other: Limb) {
        if self.sign {
            self.abs.neg_mod_assign(other);
        } else {
            self.abs.mod_assign(other);
        }
        self.sign = self.abs == 0 as Limb;
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CeilingModAssign<u32> for Integer {
    #[inline]
    fn ceiling_mod_assign(&mut self, other: u32) {
        self.ceiling_mod_assign(Limb::from(other));
    }
}

impl CeilingMod<Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder has the opposite sign of the divisor. The quotient and remainder satisfy
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(23u32.ceiling_mod(Integer::from(10u32)).to_string(), "-7");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(23u32.ceiling_mod(Integer::from(-10)).to_string(), "3");
    /// }
    /// ```
    fn ceiling_mod(self, other: Integer) -> Integer {
        if other.sign {
            -self.neg_mod(other.abs)
        } else {
            Integer::from(self % other.abs)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CeilingMod<Integer> for u32 {
    type Output = Integer;

    fn ceiling_mod(self, other: Integer) -> Integer {
        if other.sign {
            -self.neg_mod(other.abs)
        } else {
            Integer::from(self % other.abs)
        }
    }
}

impl<'a> CeilingMod<&'a Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(23u32.ceiling_mod(&Integer::from(10u32)).to_string(), "-7");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(23u32.ceiling_mod(&Integer::from(-10)).to_string(), "3");
    /// }
    /// ```
    fn ceiling_mod(self, other: &'a Integer) -> Integer {
        if other.sign {
            -self.neg_mod(&other.abs)
        } else {
            Integer::from(self % &other.abs)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CeilingMod<&'a Integer> for u32 {
    type Output = Integer;

    fn ceiling_mod(self, other: &'a Integer) -> Integer {
        if other.sign {
            -self.neg_mod(&other.abs)
        } else {
            Integer::from(self % &other.abs)
        }
    }
}
