use std::ops::{Rem, RemAssign};

use malachite_base::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign,
};
use malachite_base::num::basic::traits::Zero;

use integer::Integer;
use natural::Natural;

impl Mod<Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by value and returning the
    /// remainder. The remainder has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!(Integer::from(23).mod_op(Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(Integer::from(23).mod_op(Integer::from(-10)).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(Integer::from(10)).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    #[inline]
    fn mod_op(mut self, other: Integer) -> Integer {
        self.mod_assign(other);
        self
    }
}

impl<'a> Mod<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by value and the second by
    /// reference, and returning the remainder. The remainder has the same sign as the divisor. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!(Integer::from(23).mod_op(&Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(Integer::from(23).mod_op(&Integer::from(-10)).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(&Integer::from(10)).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).mod_op(&Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    #[inline]
    fn mod_op(mut self, other: &'a Integer) -> Integer {
        self.mod_assign(other);
        self
    }
}

impl<'a> Mod<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by reference and the second
    /// by value, and returning the remainder. The remainder has the same sign as the divisor. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!((&Integer::from(23)).mod_op(Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((&Integer::from(23)).mod_op(Integer::from(-10)).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(Integer::from(10)).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    fn mod_op(self, other: Integer) -> Integer {
        let remainder = if self.sign == other.sign {
            &self.abs % other.abs
        } else {
            (&self.abs).neg_mod(other.abs)
        };
        if other.sign {
            Integer::from(remainder)
        } else {
            -remainder
        }
    }
}

impl<'a, 'b> Mod<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by reference and returning the
    /// remainder. The remainder has the same sign as the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!((&Integer::from(23)).mod_op(&Integer::from(10)).to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((&Integer::from(23)).mod_op(&Integer::from(-10)).to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(&Integer::from(10)).to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).mod_op(&Integer::from(-10)).to_string(), "-3");
    /// }
    /// ```
    fn mod_op(self, other: &'b Integer) -> Integer {
        let remainder = if self.sign == other.sign {
            &self.abs % &other.abs
        } else {
            (&self.abs).neg_mod(&other.abs)
        };
        if other.sign {
            Integer::from(remainder)
        } else {
            -remainder
        }
    }
}

impl ModAssign<Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by value and
    /// replacing `self` with the remainder. The remainder has the same sign as the divisor. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     x.mod_assign(Integer::from(10));
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x.mod_assign(Integer::from(-10));
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(Integer::from(10));
    ///     assert_eq!(x.to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(Integer::from(-10));
    ///     assert_eq!(x.to_string(), "-3");
    /// }
    /// ```
    fn mod_assign(&mut self, other: Integer) {
        if self.sign == other.sign {
            self.abs %= other.abs;
        } else {
            self.abs.neg_mod_assign(other.abs);
        };
        self.sign = other.sign || self.abs == Natural::ZERO;
    }
}

impl<'a> ModAssign<&'a Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by reference and
    /// replacing `self` with the remainder. The remainder has the same sign as the divisor. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     x.mod_assign(&Integer::from(10));
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x.mod_assign(&Integer::from(-10));
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(&Integer::from(10));
    ///     assert_eq!(x.to_string(), "7");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.mod_assign(&Integer::from(-10));
    ///     assert_eq!(x.to_string(), "-3");
    /// }
    /// ```
    fn mod_assign(&mut self, other: &'a Integer) {
        if self.sign == other.sign {
            self.abs %= &other.abs;
        } else {
            self.abs.neg_mod_assign(&other.abs);
        };
        self.sign = other.sign || self.abs == Natural::ZERO;
    }
}

impl Rem<Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by value and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Integer::from(23) % Integer::from(10)).to_string(), "3");
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((Integer::from(23) % Integer::from(-10)).to_string(), "3");
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((Integer::from(-23) % Integer::from(10)).to_string(), "-3");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((Integer::from(-23) % Integer::from(-10)).to_string(), "-3");
    /// ```
    #[inline]
    fn rem(mut self, other: Integer) -> Integer {
        self %= other;
        self
    }
}

impl<'a> Rem<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by value and the second by
    /// reference, and returning the remainder. The remainder has the same sign as the dividend. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Integer::from(23) % &Integer::from(10)).to_string(), "3");
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((Integer::from(23) % &Integer::from(-10)).to_string(), "3");
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((Integer::from(-23) % &Integer::from(10)).to_string(), "-3");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((Integer::from(-23) % &Integer::from(-10)).to_string(), "-3");
    /// ```
    #[inline]
    fn rem(mut self, other: &'a Integer) -> Integer {
        self %= other;
        self
    }
}

impl<'a> Rem<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by reference and the second
    /// by value, and returning the remainder. The remainder has the same sign as the dividend. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23) % Integer::from(10)).to_string(), "3");
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((&Integer::from(23) % Integer::from(-10)).to_string(), "3");
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((&Integer::from(-23) % Integer::from(10)).to_string(), "-3");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23) % Integer::from(-10)).to_string(), "-3");
    /// ```
    #[inline]
    fn rem(self, other: Integer) -> Integer {
        let remainder = &self.abs % other.abs;
        if self.sign {
            Integer::from(remainder)
        } else {
            -remainder
        }
    }
}

impl<'a, 'b> Rem<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by reference and returning the
    /// remainder. The remainder has the same sign as the dividend. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23) % &Integer::from(10)).to_string(), "3");
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((&Integer::from(23) % &Integer::from(-10)).to_string(), "3");
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((&Integer::from(-23) % &Integer::from(10)).to_string(), "-3");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23) % &Integer::from(-10)).to_string(), "-3");
    /// ```
    #[inline]
    fn rem(self, other: &'b Integer) -> Integer {
        let remainder = &self.abs % &other.abs;
        if self.sign {
            Integer::from(remainder)
        } else {
            -remainder
        }
    }
}

impl RemAssign<Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by value and
    /// replacing `self` with the remainder. The remainder has the same sign as the dividend. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x %= Integer::from(10);
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x %= Integer::from(-10);
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x %= Integer::from(10);
    /// assert_eq!(x.to_string(), "-3");
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x %= Integer::from(-10);
    /// assert_eq!(x.to_string(), "-3");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Integer) {
        self.abs %= other.abs;
        self.sign = self.sign || self.abs == Natural::ZERO;
    }
}

impl<'a> RemAssign<&'a Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by reference and
    /// replacing `self` with the remainder. The remainder has the same sign as the dividend. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x %= &Integer::from(10);
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x %= &Integer::from(-10);
    /// assert_eq!(x.to_string(), "3");
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x %= &Integer::from(10);
    /// assert_eq!(x.to_string(), "-3");
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x %= &Integer::from(-10);
    /// assert_eq!(x.to_string(), "-3");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &'a Integer) {
        self.abs %= &other.abs;
        self.sign = self.sign || self.abs == Natural::ZERO;
    }
}

impl CeilingMod<Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by value and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Integer::from(23).ceiling_mod(Integer::from(10)).to_string(), "-7");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(Integer::from(23).ceiling_mod(Integer::from(-10)).to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(Integer::from(10)).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(Integer::from(-10)).to_string(), "7");
    /// }
    /// ```
    #[inline]
    fn ceiling_mod(mut self, other: Integer) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

impl<'a> CeilingMod<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by value and the second by
    /// reference, and returning the remainder. The remainder has the opposite sign of the divisor.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Integer::from(23).ceiling_mod(&Integer::from(10)).to_string(), "-7");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!(Integer::from(23).ceiling_mod(&Integer::from(-10)).to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(&Integer::from(10)).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!(Integer::from(-23).ceiling_mod(&Integer::from(-10)).to_string(), "7");
    /// }
    /// ```
    #[inline]
    fn ceiling_mod(mut self, other: &'a Integer) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

impl<'a> CeilingMod<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by reference and the second
    /// by value, and returning the remainder. The remainder has the opposite sign of the divisor.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23)).ceiling_mod(Integer::from(10)).to_string(), "-7");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((&Integer::from(23)).ceiling_mod(Integer::from(-10)).to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(Integer::from(10)).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(Integer::from(-10)).to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod(self, other: Integer) -> Integer {
        let remainder = if self.sign == other.sign {
            (&self.abs).neg_mod(other.abs)
        } else {
            &self.abs % other.abs
        };
        if other.sign {
            -remainder
        } else {
            Integer::from(remainder)
        }
    }
}

impl<'a, 'b> CeilingMod<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by reference and returning the
    /// remainder. The remainder has the opposite sign of the divisor. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23)).ceiling_mod(&Integer::from(10)).to_string(), "-7");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     assert_eq!((&Integer::from(23)).ceiling_mod(&Integer::from(-10)).to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(&Integer::from(10)).to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23)).ceiling_mod(&Integer::from(-10)).to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod(self, other: &'b Integer) -> Integer {
        let remainder = if self.sign == other.sign {
            (&self.abs).neg_mod(&other.abs)
        } else {
            &self.abs % &other.abs
        };
        if other.sign {
            -remainder
        } else {
            Integer::from(remainder)
        }
    }
}

impl CeilingModAssign<Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by value, taking
    /// the quotient and replacing `self` with the remainder. The remainder has the opposite sign of
    /// the divisor. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     x.ceiling_mod_assign(Integer::from(10));
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x.ceiling_mod_assign(Integer::from(-10));
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(Integer::from(10));
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(Integer::from(-10));
    ///     assert_eq!(x.to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod_assign(&mut self, other: Integer) {
        if self.sign == other.sign {
            self.abs.neg_mod_assign(other.abs);
        } else {
            self.abs %= other.abs;
        };
        self.sign = !other.sign || self.abs == Natural::ZERO;
    }
}

impl<'a> CeilingModAssign<&'a Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by reference,
    /// replacing `self` with the remainder. The remainder has the opposite sign of the divisor. The
    /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
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
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     x.ceiling_mod_assign(&Integer::from(10));
    ///     assert_eq!(x.to_string(), "-7");
    ///
    ///     // -3 * -10 + -7 = 23
    ///     let mut x = Integer::from(23);
    ///     x.ceiling_mod_assign(&Integer::from(-10));
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // -3 * 10 + 7 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(&Integer::from(10));
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x.ceiling_mod_assign(&Integer::from(-10));
    ///     assert_eq!(x.to_string(), "7");
    /// }
    /// ```
    fn ceiling_mod_assign(&mut self, other: &'a Integer) {
        if self.sign == other.sign {
            self.abs.neg_mod_assign(&other.abs);
        } else {
            self.abs %= &other.abs;
        };
        self.sign = !other.sign || self.abs == Natural::ZERO;
    }
}
