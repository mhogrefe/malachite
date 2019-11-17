use std::ops::{Rem, RemAssign};

use malachite_base::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign, UnsignedAbs,
};

use integer::Integer;
use natural::Natural;
use platform::{Limb, SignedLimb};

impl Mod<SignedLimb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value and returning the
    /// remainder. The remainder has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Integer::from(23).mod_op(10i32).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(Integer::from(23).mod_op(-10i32).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(10i32).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(-10i32).to_string(), "-3");
    /// }
    /// ```
    #[inline]
    fn mod_op(mut self, other: SignedLimb) -> Integer {
        self.mod_assign(other);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Mod<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn mod_op(self, other: i32) -> Integer {
        self.mod_op(SignedLimb::from(other))
    }
}

impl<'a> Mod<SignedLimb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23)).mod_op(10i32).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((&Integer::from(23)).mod_op(-10i32).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(10i32).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(-10i32).to_string(), "-3");
    /// }
    /// ```
    fn mod_op(self, other: SignedLimb) -> Integer {
        let remainder = if self.sign == (other >= 0) {
            &self.abs % other.unsigned_abs()
        } else {
            (&self.abs).neg_mod(other.unsigned_abs())
        };
        if other >= 0 {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Mod<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn mod_op(self, other: i32) -> Integer {
        self.mod_op(SignedLimb::from(other))
    }
}

impl ModAssign<SignedLimb> for Integer {
    /// Divides an `Integer` by a `SignedLimb`, replacing the `Integer` by the remainder. The
    /// remainder has the same sign as the divisor. The quotient and remainder satisfy
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     x.mod_assign(10i32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x.mod_assign(-10i32);
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(10i32);
    ///     assert_eq!(x.to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(-10i32);
    ///     assert_eq!(x.to_string(), "-3");
    /// }
    /// ```
    fn mod_assign(&mut self, other: SignedLimb) {
        if self.sign == (other >= 0) {
            self.abs %= other.unsigned_abs();
        } else {
            self.abs.neg_mod_assign(other.unsigned_abs());
        }
        self.sign = other >= 0 || self.abs == 0 as Limb;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl ModAssign<i32> for Integer {
    #[inline]
    fn mod_assign(&mut self, other: i32) {
        self.mod_assign(SignedLimb::from(other));
    }
}

impl Mod<Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value and returning the
    /// remainder. The remainder has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23i32.mod_op(Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(23i32.mod_op(Integer::from(-10)).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((-23i32).mod_op(Integer::from(10)).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((-23i32).mod_op(Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    fn mod_op(self, other: Integer) -> Integer {
        let remainder = if (self >= 0) == other.sign {
            Natural::from(self.unsigned_abs() % other.abs)
        } else {
            self.unsigned_abs().neg_mod(other.abs)
        };
        if other.sign {
            Integer::from(remainder)
        } else {
            -remainder
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Mod<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn mod_op(self, other: Integer) -> Integer {
        SignedLimb::from(self).mod_op(other)
    }
}

impl<'a> Mod<&'a Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23i32.mod_op(&Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(23i32.mod_op(&Integer::from(-10)).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((-23i32).mod_op(&Integer::from(10)).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((-23i32).mod_op(&Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    fn mod_op(self, other: &'a Integer) -> Integer {
        let remainder = if (self >= 0) == other.sign {
            Natural::from(self.unsigned_abs() % &other.abs)
        } else {
            self.unsigned_abs().neg_mod(&other.abs)
        };
        if other.sign {
            Integer::from(remainder)
        } else {
            -remainder
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Mod<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn mod_op(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self).mod_op(other)
    }
}

impl Rem<SignedLimb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Integer::from(23) % 10i32).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((Integer::from(23) % -10i32).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((Integer::from(-23) % 10i32).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((Integer::from(-23) % -10i32).to_string(), "-3");
    /// }
    /// ```
    #[inline]
    fn rem(mut self, other: SignedLimb) -> Integer {
        self %= other;
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Rem<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn rem(self, other: i32) -> Integer {
        self % SignedLimb::from(other)
    }
}

impl<'a> Rem<SignedLimb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23) % 10i32).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((&Integer::from(23) % -10i32).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23) % 10i32).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23) % -10i32).to_string(), "-3");
    /// }
    /// ```
    fn rem(self, other: SignedLimb) -> Integer {
        if self.sign {
            Integer::from(&self.abs % other.unsigned_abs())
        } else {
            -Natural::from(&self.abs % other.unsigned_abs())
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Rem<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn rem(self, other: i32) -> Integer {
        self % SignedLimb::from(other)
    }
}

impl RemAssign<SignedLimb> for Integer {
    /// Divides an `Integer` by a `SignedLimb`, replacing the `Integer` by the remainder. The
    /// remainder has the same sign as the dividend. The quotient and remainder satisfy
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     x %= 10i32;
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x %= -10i32;
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x %= 10i32;
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x %= -10i32;
    ///     assert_eq!(x.to_string(), "-3");
    /// }
    /// ```
    fn rem_assign(&mut self, other: SignedLimb) {
        self.abs %= other.unsigned_abs();
        self.sign |= self.abs == 0 as Limb;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl RemAssign<i32> for Integer {
    #[inline]
    fn rem_assign(&mut self, other: i32) {
        *self %= SignedLimb::from(other);
    }
}

impl Rem<Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((23i32 % Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((23i32 % Integer::from(-10)).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((-23i32 % Integer::from(10)).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((-23i32 % Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    fn rem(self, other: Integer) -> Integer {
        if self >= 0 {
            Integer::from(self.unsigned_abs() % other.abs)
        } else {
            -Natural::from(self.unsigned_abs() % other.abs)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Rem<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn rem(self, other: Integer) -> Integer {
        SignedLimb::from(self) % other
    }
}

impl<'a> Rem<&'a Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    ///     assert_eq!((23i32 % &Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((23i32 % &Integer::from(-10)).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((-23i32 % &Integer::from(10)).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((-23i32 % &Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    fn rem(self, other: &'a Integer) -> Integer {
        if self >= 0 {
            Integer::from(self.unsigned_abs() % &other.abs)
        } else {
            -Natural::from(self.unsigned_abs() % &other.abs)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Rem<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn rem(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self) % other
    }
}

impl CeilingMod<SignedLimb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!(Integer::from(23).ceiling_mod(10i32).to_string(), "-7");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!(Integer::from(23).ceiling_mod(-10i32).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(10i32).to_string(), "-3");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(-10i32).to_string(), "7");
    /// }
    /// ```
    #[inline]
    fn ceiling_mod(mut self, other: SignedLimb) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingMod<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn ceiling_mod(self, other: i32) -> Integer {
        self.ceiling_mod(SignedLimb::from(other))
    }
}

impl<'a> CeilingMod<SignedLimb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     assert_eq!((&Integer::from(23)).ceiling_mod(10i32).to_string(), "-7");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!((&Integer::from(23)).ceiling_mod(-10i32).to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(10i32).to_string(), "-3");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(-10i32).to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod(self, other: SignedLimb) -> Integer {
        let remainder = if self.sign == (other >= 0) {
            (&self.abs).neg_mod(other.unsigned_abs())
        } else {
            &self.abs % other.unsigned_abs()
        };
        if other >= 0 {
            -Natural::from(remainder)
        } else {
            Integer::from(remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingMod<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn ceiling_mod(self, other: i32) -> Integer {
        self.ceiling_mod(SignedLimb::from(other))
    }
}

impl CeilingModAssign<SignedLimb> for Integer {
    /// Divides an `Integer` by a `SignedLimb`, replacing the `Integer` by the remainder. The
    /// remainder has the opposite sign of the divisor. The quotient and remainder satisfy
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
    /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x.ceiling_mod_assign(10i32);
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     x.ceiling_mod_assign(-10i32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(10i32);
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 3 * -10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(-10i32);
    ///     assert_eq!(x.to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod_assign(&mut self, other: SignedLimb) {
        if self.sign == (other >= 0) {
            self.abs.neg_mod_assign(other.unsigned_abs());
        } else {
            self.abs %= other.unsigned_abs();
        };
        self.sign = other < 0 || self.abs == 0 as Limb;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingModAssign<i32> for Integer {
    #[inline]
    fn ceiling_mod_assign(&mut self, other: i32) {
        self.ceiling_mod_assign(SignedLimb::from(other));
    }
}

impl CeilingMod<Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(23i32.ceiling_mod(Integer::from(10)).to_string(), "-7");
    ///
    ///     // -3 * -10 - 7 = 23
    ///     assert_eq!(23i32.ceiling_mod(Integer::from(-10)).to_string(), "3");
    ///
    ///     // -3 * 10 - 3 = -23
    ///     assert_eq!((-23i32).ceiling_mod(Integer::from(10)).to_string(), "-3");
    ///
    ///     // 3 * -10 - 3 = -23
    ///     assert_eq!((-23i32).ceiling_mod(Integer::from(-10)).to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod(self, other: Integer) -> Integer {
        let remainder = if (self >= 0) == other.sign {
            self.unsigned_abs().neg_mod(other.abs)
        } else {
            Natural::from(self.unsigned_abs() % other.abs)
        };
        if other.sign {
            -remainder
        } else {
            Integer::from(remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingMod<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn ceiling_mod(self, other: Integer) -> Integer {
        SignedLimb::from(self).ceiling_mod(other)
    }
}

impl<'a> CeilingMod<&'a Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(23i32.ceiling_mod(&Integer::from(10)).to_string(), "-7");
    ///
    ///     // -3 * -10 - 7 = 23
    ///     assert_eq!(23i32.ceiling_mod(&Integer::from(-10)).to_string(), "3");
    ///
    ///     // -3 * 10 - 3 = -23
    ///     assert_eq!((-23i32).ceiling_mod(&Integer::from(10)).to_string(), "-3");
    ///
    ///     // 3 * -10 - 3 = -23
    ///     assert_eq!((-23i32).ceiling_mod(&Integer::from(-10)).to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod(self, other: &'a Integer) -> Integer {
        let remainder = if (self >= 0) == other.sign {
            self.unsigned_abs().neg_mod(&other.abs)
        } else {
            Natural::from(self.unsigned_abs() % &other.abs)
        };
        if other.sign {
            -remainder
        } else {
            Integer::from(remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingMod<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn ceiling_mod(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self).ceiling_mod(other)
    }
}
