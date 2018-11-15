use integer::Integer;
use malachite_base::num::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign, UnsignedAbs,
};
use natural::Natural;
use std::ops::{Rem, RemAssign};

impl Mod<i32> for Integer {
    type Output = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-negative and less than the absolute value of the divisor. In
    /// other words, returns r, where `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(Integer::from(456).mod_op(123i32), 87);
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(Integer::from(456).mod_op(-123i32), 87);
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!(Integer::from(-456).mod_op(123i32), 36);
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!(Integer::from(-456).mod_op(-123i32), 36);
    /// }
    /// ```
    fn mod_op(self, other: i32) -> u32 {
        self.mod_op(other.unsigned_abs())
    }
}

impl<'a> Mod<i32> for &'a Integer {
    type Output = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-negative and less than the absolute value of the
    /// divisor. In other words, returns r, where `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((&Integer::from(456)).mod_op(123i32), 87);
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!((&Integer::from(456)).mod_op(-123i32), 87);
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!((&Integer::from(-456)).mod_op(123i32), 36);
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!((&Integer::from(-456)).mod_op(-123i32), 36);
    /// }
    /// ```
    fn mod_op(self, other: i32) -> u32 {
        self.mod_op(other.unsigned_abs())
    }
}

impl ModAssign<i32> for Integer {
    /// Divides an `Integer` by an `i32`, replacing the `Integer` by the remainder. The remainder is
    /// always non-negative and less than the absolute value of the divisor. In other words,
    /// replaces `self` with r, where `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     x.mod_assign(123i32);
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     x.mod_assign(-123i32);
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     let mut x = Integer::from(-456);
    ///     x.mod_assign(123i32);
    ///     assert_eq!(x.to_string(), "36");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     let mut x = Integer::from(-456);
    ///     x.mod_assign(-123i32);
    ///     assert_eq!(x.to_string(), "36");
    /// }
    /// ```
    fn mod_assign(&mut self, other: i32) {
        self.mod_assign(other.unsigned_abs())
    }
}

impl Mod<Integer> for i32 {
    type Output = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-negative and less than the absolute value of the divisor. In
    /// other words, returns r, where `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456i32.mod_op(Integer::from(123)).to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(456i32.mod_op(Integer::from(-123)).to_string(), "87");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!((-456i32).mod_op(Integer::from(123)).to_string(), "36");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!((-456i32).mod_op(Integer::from(-123)).to_string(), "36");
    /// }
    /// ```
    fn mod_op(self, other: Integer) -> Natural {
        let self_abs = self.unsigned_abs();
        if self >= 0 {
            Natural::from(self_abs % other.abs)
        } else {
            self_abs.neg_mod(other.abs)
        }
    }
}

impl<'a> Mod<&'a Integer> for i32 {
    type Output = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-negative and less than the absolute value of the
    /// divisor. In other words, returns r, where `self` = q * `other` + r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456i32.mod_op(&Integer::from(123)).to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!(456i32.mod_op(&Integer::from(-123)).to_string(), "87");
    ///
    ///     // -4 * 123 + 36 = -456
    ///     assert_eq!((-456i32).mod_op(&Integer::from(123)).to_string(), "36");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     assert_eq!((-456i32).mod_op(&Integer::from(-123)).to_string(), "36");
    /// }
    /// ```
    fn mod_op(self, other: &'a Integer) -> Natural {
        let self_abs = self.unsigned_abs();
        if self >= 0 {
            Natural::from(self_abs % &other.abs)
        } else {
            self_abs.neg_mod(&other.abs)
        }
    }
}

impl Rem<i32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the remainder.
    /// The remainder has the same sign as the dividend and its absolute value is less than the
    /// absolute value of the divisor. In other words, returns r, where `self` = q * `other` + r,
    /// (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < |`other`|.
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
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((Integer::from(456) % 123i32).to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!((Integer::from(456) % -123i32).to_string(), "87");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!((Integer::from(-456) % 123i32).to_string(), "-87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!((Integer::from(-456) % -123i32).to_string(), "-87");
    /// }
    /// ```
    fn rem(self, other: i32) -> Integer {
        self % other.unsigned_abs()
    }
}

impl<'a> Rem<i32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend and its absolute value is less
    /// than the absolute value of the divisor. In other words, returns r, where
    /// `self` = q * `other` + r, (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < |`other`|.
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
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((&Integer::from(456) % 123i32).to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!((&Integer::from(456) % -123i32).to_string(), "87");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!((&Integer::from(-456) % 123i32).to_string(), "-87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!((&Integer::from(-456) % -123i32).to_string(), "-87");
    /// }
    /// ```
    fn rem(self, other: i32) -> Integer {
        self % other.unsigned_abs()
    }
}

impl RemAssign<i32> for Integer {
    /// Divides an `Integer` by an `i32`, replacing the `Integer` by the remainder. The remainder
    /// has the same sign as the dividend and its absolute value is less than the absolute value of
    /// the divisor. In other words, replaces `self` with r, where `self` = q * `other` + r,
    /// (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < |`other`|.
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
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     x %= 123i32;
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     let mut x = Integer::from(456);
    ///     x %= -123i32;
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     let mut x = Integer::from(-456);
    ///     x %= 123i32;
    ///     assert_eq!(x.to_string(), "-87");
    ///
    ///     // 4 * -123 + 36 = -456
    ///     let mut x = Integer::from(-456);
    ///     x %= -123i32;
    ///     assert_eq!(x.to_string(), "-87");
    /// }
    /// ```
    fn rem_assign(&mut self, other: i32) {
        *self %= other.unsigned_abs();
    }
}

impl Rem<Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder has the same sign as the dividend and its absolute value is less than the
    /// absolute value of the divisor. In other words, returns r, where `self` = q * `other` + r and
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((456i32 % Integer::from(123)).to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!((456i32 % Integer::from(-123)).to_string(), "87");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!((-456i32 % Integer::from(123)).to_string(), "-87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!((-456i32 % Integer::from(-123)).to_string(), "-87");
    /// }
    /// ```
    fn rem(self, other: Integer) -> Integer {
        let self_abs = self.unsigned_abs();
        let remainder = self_abs % other.abs;
        if self >= 0 {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        }
    }
}

impl<'a> Rem<&'a Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend and its absolute value is less
    /// than the absolute value of the divisor. In other words, returns r, where
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((456i32 % &Integer::from(123)).to_string(), "87");
    ///
    ///     // -3 * -123 + 87 = 456
    ///     assert_eq!((456i32 % &Integer::from(-123)).to_string(), "87");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!((-456i32 % &Integer::from(123)).to_string(), "-87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!((-456i32 % &Integer::from(-123)).to_string(), "-87");
    /// }
    /// ```
    fn rem(self, other: &'a Integer) -> Integer {
        let self_abs = self.unsigned_abs();
        let remainder = self_abs % &other.abs;
        if self >= 0 {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        }
    }
}

impl NegMod<i32> for Integer {
    type Output = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the remainder
    /// of the negative of the `Integer` divided by the `i32`. The remainder is always non-negative
    /// and less than the absolute value of the divisor. In other words, returns r, where
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(Integer::from(456).neg_mod(123i32), 36);
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(Integer::from(456).neg_mod(-123i32), 36);
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!(Integer::from(-456).neg_mod(123i32), 87);
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!(Integer::from(-456).neg_mod(-123i32), 87);
    /// }
    /// ```
    fn neg_mod(self, other: i32) -> u32 {
        self.neg_mod(other.unsigned_abs())
    }
}

impl<'a> NegMod<i32> for &'a Integer {
    type Output = u32;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// remainder of the negative of the `Integer` divided by the `i32`. The remainder is always
    /// non-negative and less than the absolute value of the divisor. In other words, returns r,
    /// where `self` = q * `other` - r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!((&Integer::from(456)).neg_mod(123i32), 36);
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!((&Integer::from(456)).neg_mod(-123i32), 36);
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!((&Integer::from(-456)).neg_mod(123i32), 87);
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!((&Integer::from(-456)).neg_mod(-123i32), 87);
    /// }
    /// ```
    fn neg_mod(self, other: i32) -> u32 {
        self.neg_mod(other.unsigned_abs())
    }
}

impl NegModAssign<i32> for Integer {
    /// Divides the negative of an `Integer` by an `i32`, replacing the `Integer` by the remainder.
    /// In other words, replaces `self` with r, where `self` = q * `other` - r and
    /// 0 <= r < |`other`|.
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
    /// use malachite_base::num::NegModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     let mut x = Integer::from(456);
    ///     x.neg_mod_assign(123i32);
    ///     assert_eq!(x.to_string(), "36");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     let mut x = Integer::from(456);
    ///     x.neg_mod_assign(-123i32);
    ///     assert_eq!(x.to_string(), "36");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     let mut x = Integer::from(-456);
    ///     x.neg_mod_assign(123i32);
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     let mut x = Integer::from(-456);
    ///     x.neg_mod_assign(-123i32);
    ///     assert_eq!(x.to_string(), "87");
    /// }
    /// ```
    fn neg_mod_assign(&mut self, other: i32) {
        self.neg_mod_assign(other.unsigned_abs())
    }
}

impl NegMod<Integer> for i32 {
    type Output = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the remainder
    /// of the negative of the `i32` divided by the `Integer`. The remainder is always non-negative
    /// and less than the absolute value of the divisor. In other words, returns r, where
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(456i32.neg_mod(Integer::from(123)).to_string(), "36");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(456i32.neg_mod(Integer::from(-123)).to_string(), "36");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!((-456i32).neg_mod(Integer::from(123)).to_string(), "87");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!((-456i32).neg_mod(Integer::from(-123)).to_string(), "87");
    /// }
    /// ```
    fn neg_mod(self, other: Integer) -> Natural {
        let self_abs = self.unsigned_abs();
        if self >= 0 {
            self_abs.neg_mod(other.abs)
        } else {
            Natural::from(self_abs % other.abs)
        }
    }
}

impl<'a> NegMod<&'a Integer> for i32 {
    type Output = Natural;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder of the negative of the `i32` divided by the `Integer`. The remainder is always
    /// non-negative and less than the absolute value of the divisor. In other words, returns r,
    /// where `self` = q * `other` - r and 0 <= r < |`other`|.
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(456i32.neg_mod(&Integer::from(123)).to_string(), "36");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(456i32.neg_mod(&Integer::from(-123)).to_string(), "36");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!((-456i32).neg_mod(&Integer::from(123)).to_string(), "87");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!((-456i32).neg_mod(&Integer::from(-123)).to_string(), "87");
    /// }
    /// ```
    fn neg_mod(self, other: &'a Integer) -> Natural {
        let self_abs = self.unsigned_abs();
        if self >= 0 {
            self_abs.neg_mod(&other.abs)
        } else {
            Natural::from(self_abs % &other.abs)
        }
    }
}

impl CeilingMod<i32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-positive and its absolute value is less than the absolute value
    /// of the divisor. In other words, returns r, where `self` = q * `other` + r and
    /// 0 <= -r < |`other`|.
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
    /// use malachite_base::num::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 + -36 = 456
    ///     assert_eq!(Integer::from(456).ceiling_mod(123i32).to_string(), "-36");
    ///
    ///     // -4 * -123 + -36 = 456
    ///     assert_eq!(Integer::from(456).ceiling_mod(-123i32).to_string(), "-36");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!(Integer::from(-456).ceiling_mod(123i32).to_string(), "-87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!(Integer::from(-456).ceiling_mod(-123i32).to_string(), "-87");
    /// }
    /// ```
    fn ceiling_mod(self, other: i32) -> Integer {
        self.ceiling_mod(other.unsigned_abs())
    }
}

impl<'a> CeilingMod<i32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-positive and its absolute value is less than the
    /// absolute value of the divisor. In other words, returns r, where `self` = q * `other` + r and
    /// 0 <= -r < |`other`|.
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
    /// use malachite_base::num::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 + -36 = 456
    ///     assert_eq!((&Integer::from(456)).ceiling_mod(123i32).to_string(), "-36");
    ///
    ///     // -4 * -123 + -36 = 456
    ///     assert_eq!((&Integer::from(456)).ceiling_mod(-123i32).to_string(), "-36");
    ///
    ///     // -3 * 123 + -87 = -456
    ///     assert_eq!((&Integer::from(-456)).ceiling_mod(123i32).to_string(), "-87");
    ///
    ///     // 3 * -123 + -87 = -456
    ///     assert_eq!((&Integer::from(-456)).ceiling_mod(-123i32).to_string(), "-87");
    /// }
    /// ```
    fn ceiling_mod(self, other: i32) -> Integer {
        self.ceiling_mod(other.unsigned_abs())
    }
}

impl CeilingModAssign<i32> for Integer {
    /// Divides an `Integer` by an `i32`, replacing the `Integer` by the remainder. The remainder is
    /// always non-positive and its absolute value is less than the absolute value of the divisor.
    /// In other words, replaces `self` with r, where `self` = q * `other` + r and
    /// 0 <= -r < |`other`|.
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
    /// use malachite_base::num::CeilingModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     let mut x = Integer::from(456);
    ///     x.ceiling_mod_assign(123i32);
    ///     assert_eq!(x.to_string(), "-36");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     let mut x = Integer::from(456);
    ///     x.ceiling_mod_assign(-123i32);
    ///     assert_eq!(x.to_string(), "-36");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     let mut x = Integer::from(-456);
    ///     x.ceiling_mod_assign(123i32);
    ///     assert_eq!(x.to_string(), "-87");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     let mut x = Integer::from(-456);
    ///     x.ceiling_mod_assign(-123i32);
    ///     assert_eq!(x.to_string(), "-87");
    /// }
    /// ```
    fn ceiling_mod_assign(&mut self, other: i32) {
        self.ceiling_mod_assign(other.unsigned_abs());
    }
}

impl CeilingMod<Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-positive and its absolute value is less than the absolute value
    /// of the divisor. In other words, returns r, where `self` = q * `other` + r and
    /// 0 <= -r < |`other`|.
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
    /// use malachite_base::num::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(456i32.ceiling_mod(Integer::from(123)).to_string(), "-36");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(456i32.ceiling_mod(Integer::from(-123)).to_string(), "-36");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!((-456i32).ceiling_mod(Integer::from(123)).to_string(), "-87");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!((-456i32).ceiling_mod(Integer::from(-123)).to_string(), "-87");
    /// }
    /// ```
    fn ceiling_mod(self, other: Integer) -> Integer {
        -self.neg_mod(other)
    }
}

impl<'a> CeilingMod<&'a Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-positive and its absolute value is less than the
    /// absolute value of the divisor. In other words, returns (q, r), where
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
    /// use malachite_base::num::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(456i32.ceiling_mod(&Integer::from(123)).to_string(), "-36");
    ///
    ///     // -4 * -123 - 36 = 456
    ///     assert_eq!(456i32.ceiling_mod(&Integer::from(-123)).to_string(), "-36");
    ///
    ///     // -3 * 123 - 87 = -456
    ///     assert_eq!((-456i32).ceiling_mod(&Integer::from(123)).to_string(), "-87");
    ///
    ///     // 3 * -123 - 87 = -456
    ///     assert_eq!((-456i32).ceiling_mod(&Integer::from(-123)).to_string(), "-87");
    /// }
    /// ```
    fn ceiling_mod(self, other: &'a Integer) -> Integer {
        -self.neg_mod(other)
    }
}
