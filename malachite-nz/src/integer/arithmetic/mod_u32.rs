use integer::Integer;
use malachite_base::num::{CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign};
use natural::Natural;
use std::ops::{Rem, RemAssign};

impl Mod<u32> for Integer {
    type Output = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder.
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
    /// use malachite_base::num::Mod;
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
    fn mod_op(self, other: u32) -> u32 {
        if self.sign {
            self.abs % other
        } else {
            self.abs.neg_mod(other)
        }
    }
}

impl<'a> Mod<u32> for &'a Integer {
    type Output = u32;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
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
    /// use malachite_base::num::Mod;
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
    fn mod_op(self, other: u32) -> u32 {
        if self.sign {
            &self.abs % other
        } else {
            (&self.abs).neg_mod(other)
        }
    }
}

impl ModAssign<u32> for Integer {
    /// Divides an `Integer` by a `u32`, replacing the `Integer` by the remainder. The remainder is
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
    /// use malachite_base::num::ModAssign;
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
    fn mod_assign(&mut self, other: u32) {
        if self.sign {
            self.abs.mod_assign(other);
        } else {
            self.abs.neg_mod_assign(other);
        }
        self.sign = true;
    }
}

impl Mod<Integer> for u32 {
    type Output = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the remainder.
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
    /// use malachite_base::num::Mod;
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

impl<'a> Mod<&'a Integer> for u32 {
    type Output = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
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
    /// use malachite_base::num::Mod;
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

impl Rem<u32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder.
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
    fn rem(self, other: u32) -> Integer {
        if self.sign {
            Integer::from(self.abs % other)
        } else {
            -Natural::from(self.abs % other)
        }
    }
}

impl<'a> Rem<u32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
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
    fn rem(self, other: u32) -> Integer {
        if self.sign {
            Integer::from(&self.abs % other)
        } else {
            -Natural::from(&self.abs % other)
        }
    }
}

impl RemAssign<u32> for Integer {
    /// Divides an `Integer` by a `u32`, replacing the `Integer` by the remainder. The remainder has
    /// the same sign as the dividend. The remainder has the same sign as the dividend. The quotient
    /// and remainder satisfy `self` = q * `other` + r and 0 <= |r| < `other`.
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
    fn rem_assign(&mut self, other: u32) {
        self.abs.rem_assign(other);
        self.sign |= self.abs == 0;
    }
}

impl Rem<Integer> for u32 {
    type Output = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the remainder.
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
    /// use malachite_base::num::DivRem;
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
    fn rem(self, other: Integer) -> u32 {
        self % other.abs
    }
}

impl<'a> Rem<&'a Integer> for u32 {
    type Output = u32;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
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
    /// use malachite_base::num::DivRem;
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
    fn rem(self, other: &'a Integer) -> u32 {
        self % &other.abs
    }
}

impl RemAssign<Integer> for u32 {
    /// Divides a `u32` by an `Integer` in place, taking the `Integer` by value and replacing the
    /// `u32` with the remainder. The remainder is non-negative. The quotient and remainder satisfy
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

impl<'a> RemAssign<&'a Integer> for u32 {
    /// Divides a `u32` by an `Integer` in place, taking the `Integer` by reference and replacing
    /// the `u32` with the remainder. The remainder is non-negative. The quotient and remainder
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

impl CeilingMod<u32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by value and returning the remainder.
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
    /// use malachite_base::num::CeilingMod;
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
    fn ceiling_mod(mut self, other: u32) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

impl<'a> CeilingMod<u32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `u32`, taking the `Integer` by reference and returning the
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
    /// use malachite_base::num::CeilingMod;
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
    fn ceiling_mod(self, other: u32) -> Integer {
        -Natural::from(if self.sign {
            (&self.abs).neg_mod(other)
        } else {
            &self.abs % other
        })
    }
}

impl CeilingModAssign<u32> for Integer {
    /// Divides an `Integer` by a `u32`, replacing the `Integer` by the remainder. The remainder is
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
    /// use malachite_base::num::CeilingModAssign;
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
    fn ceiling_mod_assign(&mut self, other: u32) {
        if self.sign {
            self.abs.neg_mod_assign(other);
        } else {
            self.abs.mod_assign(other);
        }
        self.sign = self.abs == 0;
    }
}

impl CeilingMod<Integer> for u32 {
    type Output = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by value and returning the remainder.
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
    /// use malachite_base::num::CeilingMod;
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

impl<'a> CeilingMod<&'a Integer> for u32 {
    type Output = Integer;

    /// Divides a `u32` by an `Integer`, taking the `Integer` by reference and returning the
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
    /// use malachite_base::num::CeilingMod;
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
