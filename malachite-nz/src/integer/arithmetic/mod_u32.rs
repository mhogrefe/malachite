use integer::Integer;
use malachite_base::num::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegAssign, NegMod, NegModAssign,
};
use natural::Natural;
use std::ops::{Rem, RemAssign};

impl Mod<u32> for Integer {
    type Output = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-negative and less than the divisor. In other words, returns r,
    /// where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!(Integer::from(456u32).mod_op(123), 87);
    ///
    ///     // -8,130,081,301 * 123 + 23 = -10^12
    ///     assert_eq!((-Integer::trillion()).mod_op(123), 23);
    /// }
    /// ```
    fn mod_op(self, other: u32) -> u32 {
        if self.sign {
            self.abs.mod_op(other)
        } else {
            self.abs.neg_mod(other)
        }
    }
}

impl<'a> Mod<u32> for &'a Integer {
    type Output = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-negative and less than the divisor. In other words,
    /// returns r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!((&Integer::from(456u32)).mod_op(123), 87);
    ///
    ///     // -8,130,081,301 * 123 + 23 = -10^12
    ///     assert_eq!((&-Integer::trillion()).mod_op(123), 23);
    /// }
    /// ```
    fn mod_op(self, other: u32) -> u32 {
        if self.sign {
            (&self.abs).mod_op(other)
        } else {
            (&self.abs).neg_mod(other)
        }
    }
}

impl ModAssign<u32> for Integer {
    /// Divides an `Integer` by a `u32`, replacing the `Integer` by the remainder. The remainder is
    /// always non-negative and less than the divisor. In other words, replaces `self` with r,
    /// where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     x.mod_assign(123);
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // -8,130,081,301 * 123 + 23 = -10^12
    ///     let mut x = -Integer::trillion();
    ///     x.mod_assign(123);
    ///     assert_eq!(x.to_string(), "23");
    /// }
    /// ```
    fn mod_assign(&mut self, other: u32) {
        *self = Integer::from((&*self).mod_op(other));
    }
}

impl Mod<Integer> for u32 {
    type Output = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-negative and less than the absolute value of the divisor. In
    /// other words, returns r, where `self` = q * |`other`| + r and 0 <= r < |`other`|.
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
    ///     assert_eq!(456.mod_op(Integer::from(-123)), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.mod_op(-Integer::trillion()), 123);
    /// }
    /// ```
    fn mod_op(self, other: Integer) -> u32 {
        self.mod_op(other.abs)
    }
}

impl<'a> Mod<&'a Integer> for u32 {
    type Output = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
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
    ///     assert_eq!(456u32.mod_op(&Integer::from(-123)), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123u32.mod_op(&-Integer::trillion()), 123);
    /// }
    /// ```
    fn mod_op(self, other: &'a Integer) -> u32 {
        self.mod_op(&other.abs)
    }
}

impl ModAssign<Integer> for u32 {
    /// Divides a `u32` by an `Integer` in place, taking the `Integer` by value and replacing the
    /// `u32` with the remainder. The remainder is always non-negative and less than the absolute
    /// value of the divisor. In other words, replaces `self` with r, where `self` = q * `other` + r
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
    /// use malachite_base::num::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n.mod_assign(Integer::from(123u32));
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n.mod_assign(-Integer::trillion());
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn mod_assign(&mut self, other: Integer) {
        *self = self.mod_op(other);
    }
}

impl<'a> ModAssign<&'a Integer> for u32 {
    /// Divides a `u32` by an `Integer` in place, taking the `Integer` by reference and replacing
    /// the `u32` with the remainder. The remainder is always non-negative and less than the
    /// absolute value of the divisor. In other words, replaces `self` with r, where
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
    /// use malachite_base::num::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n.mod_assign(&Integer::from(123u32));
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n.mod_assign(&-Integer::trillion());
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn mod_assign(&mut self, other: &'a Integer) {
        *self = self.mod_op(other);
    }
}

impl Rem<u32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder.
    /// The remainder has the same sign as the dividend and its absolute value is less than the
    /// divisor. In other words, returns r, where `self` = q * `other` + r,
    /// (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < `other`.
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
    ///     assert_eq!((Integer::from(456u32) % 123).to_string(), "87");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!((-Integer::trillion() % 123).to_string(), "-100");
    /// }
    /// ```
    fn rem(self, other: u32) -> Integer {
        let remainder = self.abs % other;
        if self.sign {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        }
    }
}

impl<'a> Rem<u32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend and its absolute value is less
    /// than the divisor. In other words, returns r, where `self` = q * `other` + r,
    /// (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < `other`.
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
    ///     assert_eq!((&Integer::from(456u32) % 123).to_string(), "87");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!((&-Integer::trillion() % 123).to_string(), "-100");
    /// }
    /// ```
    fn rem(self, other: u32) -> Integer {
        let remainder = &self.abs % other;
        if self.sign {
            Integer::from(remainder)
        } else {
            -Natural::from(remainder)
        }
    }
}

impl RemAssign<u32> for Integer {
    /// Divides an `Integer` by a `u32`, replacing the `Integer` by the remainder. The remainder has
    /// the same sign as the dividend and its absolute value is less than the divisor. In other
    /// words, replaces `self` with r, where `self` = q * `other` + r,
    /// (r = 0 or sign(r) = sign(`self`)), and 0 <= |r| < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     x %= 123;
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     let mut x = -Integer::trillion();
    ///     x %= 123;
    ///     assert_eq!(x.to_string(), "-100");
    /// }
    /// ```
    fn rem_assign(&mut self, other: u32) {
        *self = &*self % other;
    }
}

impl Rem<Integer> for u32 {
    type Output = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder has the same sign as the dividend and its absolute value is less than the
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456 % Integer::from(-123), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123 % Integer::trillion(), 123);
    /// }
    /// ```
    fn rem(self, other: Integer) -> u32 {
        self.mod_op(other)
    }
}

impl<'a> Rem<&'a Integer> for u32 {
    type Output = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder has the same sign as the dividend and its absolute value is less
    /// than the divisor. In other words, returns r, where `self` = q * `other` + r and
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
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456 % &Integer::from(-123), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123 % &-Integer::trillion(), 123);
    /// }
    /// ```
    fn rem(self, other: &'a Integer) -> u32 {
        self.mod_op(other)
    }
}

impl RemAssign<Integer> for u32 {
    /// Divides a `u32` by an `Integer` in place, taking the `Integer` by value and replacing the
    /// `u32` with the remainder. The remainder has the same sign as the dividend and its absolute
    /// value is less than the divisor. In other words, replaces `self` with r, where
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
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n %= Integer::from(123u32);
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n %= -Integer::trillion();
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn rem_assign(&mut self, other: Integer) {
        *self = *self % other;
    }
}

impl<'a> RemAssign<&'a Integer> for u32 {
    /// Divides a `u32` by an `Integer` in place, taking the `Integer` by reference and replacing
    /// the `u32` with the remainder. The remainder has the same sign as the dividend and its
    /// absolute value is less than the divisor. In other words, replaces `self` with r, where
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
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n %= &Integer::from(123u32);
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n %= &-Integer::trillion();
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn rem_assign(&mut self, other: &'a Integer) {
        *self = *self % other;
    }
}

impl NegMod<u32> for Integer {
    type Output = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder
    /// of the negative of the `Integer` divided by the `u32`. The remainder is always non-negative
    /// and less than the divisor. In other words, returns r, where `self` = q * `other` - r and
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(Integer::from(456u32).neg_mod(123), 36);
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!((-Integer::trillion()).neg_mod(123), 100);
    /// }
    /// ```
    fn neg_mod(self, other: u32) -> u32 {
        if self.sign {
            self.abs.neg_mod(other)
        } else {
            self.abs.mod_op(other)
        }
    }
}

impl<'a> NegMod<u32> for &'a Integer {
    type Output = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// remainder of the negative of the `Integer` divided by the `u32`. The remainder is always
    /// non-negative and less than the divisor. In other words, returns r, where
    /// `self` = q * `other` - r and 0 <= r < `other`.
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
    ///     assert_eq!((&Integer::from(456u32)).neg_mod(123), 36);
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     assert_eq!((&-Integer::trillion()).neg_mod(123), 100);
    /// }
    /// ```
    fn neg_mod(self, other: u32) -> u32 {
        if self.sign {
            (&self.abs).neg_mod(other)
        } else {
            (&self.abs).mod_op(other)
        }
    }
}

impl NegModAssign<u32> for Integer {
    /// Divides the negative of an `Integer` by a `u32`, replacing the `Integer` by the remainder.
    /// In other words, replaces `self` with r, where `self` = q * `other` - r and 0 <= r < `other`.
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
    ///     let mut x = Integer::from(456u32);
    ///     x.neg_mod_assign(123);
    ///     assert_eq!(x.to_string(), "36");
    ///
    ///     // -8,130,081,300 * 123 - 100 = -10^12
    ///     let mut x = -Integer::trillion();
    ///     x.neg_mod_assign(123);
    ///     assert_eq!(x.to_string(), "100");
    /// }
    /// ```
    fn neg_mod_assign(&mut self, other: u32) {
        *self = Integer::from((&*self).neg_mod(other));
    }
}

impl NegMod<Integer> for u32 {
    type Output = Natural;

    /// Divides the negative of a `u32` by an `Integer`, taking the `Integer` by value and returning
    /// the remainder. In other words, returns r, where `self` = q * `other` - r and
    /// 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!(456.neg_mod(Integer::from(123u32)).to_string(), "36");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.neg_mod(-Integer::trillion()).to_string(), "999999999877");
    /// }
    /// ```
    fn neg_mod(self, other: Integer) -> Natural {
        self.neg_mod(other.abs)
    }
}

impl<'a> NegMod<&'a Integer> for u32 {
    type Output = Natural;

    /// Divides the negative of a `u32` by an `Integer`, taking the `Integer` by value and returning
    /// the remainder. In other words, returns r, where `self` = q * `other` - r and
    /// 0 <= r < |`other`|.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!(456.neg_mod(&Integer::from(123u32)).to_string(), "36");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.neg_mod(&-Integer::trillion()).to_string(), "999999999877");
    /// }
    /// ```
    fn neg_mod(self, other: &'a Integer) -> Natural {
        self.neg_mod(&other.abs)
    }
}

impl CeilingMod<u32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-positive and its absolute value is less than the divisor. In
    /// other words, returns r, where `self` = q * `other` + r and 0 <= -r < `other`.
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
    ///     assert_eq!(Integer::from(456u32).ceiling_mod(123).to_string(), "-36");
    ///
    ///     // -8,130,081,300 * 123 + -100 = -10^12
    ///     assert_eq!((-Integer::trillion()).ceiling_mod(123).to_string(), "-100");
    /// }
    /// ```
    fn ceiling_mod(self, other: u32) -> Integer {
        -Natural::from(self.neg_mod(other))
    }
}

impl<'a> CeilingMod<u32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-positive and its absolute value is less than the
    /// divisor. In other words, returns r, where `self` = q * `other` + r and 0 <= -r < `other`.
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
    ///     assert_eq!((&Integer::from(456u32)).ceiling_mod(123).to_string(), "-36");
    ///
    ///     // -8,130,081,300 * 123 + -100 = -10^12
    ///     assert_eq!((&-Integer::trillion()).ceiling_mod(123).to_string(), "-100");
    /// }
    /// ```
    fn ceiling_mod(self, other: u32) -> Integer {
        -Natural::from(self.neg_mod(other))
    }
}

impl CeilingModAssign<u32> for Integer {
    /// Divides an `Integer` by a `u32`, replacing the `Integer` by the remainder. The remainder is
    /// always non-positive and its absolute value is less than the divisor. In other words,
    /// replaces `self` with r, where `self` = q * `other` + r and 0 <= -r < `other`.
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
    ///     // 4 * 123 + -36 = 456
    ///     let mut x = Integer::from(456u32);
    ///     x.ceiling_mod_assign(123);
    ///     assert_eq!(x.to_string(), "-36");
    ///
    ///     // 8,130,081,301 * 123 + -23 = 10^12
    ///     let mut x = -Integer::trillion();
    ///     x.ceiling_mod_assign(123);
    ///     assert_eq!(x.to_string(), "-100");
    /// }
    /// ```
    fn ceiling_mod_assign(&mut self, other: u32) {
        self.neg_mod_assign(other);
        self.neg_assign();
    }
}

impl CeilingMod<Integer> for u32 {
    type Output = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the remainder.
    /// The remainder is always non-positive and its absolute value is less than the absolute value
    /// of the divisor. In other words, returns (q, r), where `self` = q * `other` + r and
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
    ///     assert_eq!(456.ceiling_mod(Integer::from(123u32)).to_string(), "-36");
    ///
    ///     // 1 * 10^12 + -999,999,999,877 = 123
    ///     assert_eq!(123.ceiling_mod(-Integer::trillion()).to_string(), "-999999999877");
    /// }
    /// ```
    fn ceiling_mod(self, other: Integer) -> Integer {
        -self.neg_mod(other)
    }
}

impl<'a> CeilingMod<&'a Integer> for u32 {
    type Output = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
    /// remainder. The remainder is always non-positive and its absolute value is less than the
    /// absolute value of the divisor. In other words, returns (q, r), where
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
    /// use malachite_base::num::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 4 * 123 + -36 = 456
    ///     assert_eq!(456.ceiling_mod(&Integer::from(123u32)).to_string(), "-36");
    ///
    ///     // 1 * 10^12 + -999,999,999,877 = 123
    ///     assert_eq!(123.ceiling_mod(&-Integer::trillion()).to_string(), "-999999999877");
    /// }
    /// ```
    fn ceiling_mod(self, other: &'a Integer) -> Integer {
        -self.neg_mod(other)
    }
}
