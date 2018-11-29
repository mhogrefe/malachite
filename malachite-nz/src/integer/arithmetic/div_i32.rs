use integer::Integer;
use malachite_base::num::UnsignedAbs;
use natural::Natural;
use std::ops::{Div, DivAssign};

impl Div<i32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value. The quotient is rounded
    /// towards zero.
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
    ///     assert_eq!((Integer::from(23) / 10i32).to_string(), "2");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!((Integer::from(23) / -10i32).to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((Integer::from(-23) / 10i32).to_string(), "-2");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((Integer::from(-23) / -10i32).to_string(), "2");
    /// }
    /// ```
    fn div(self, other: i32) -> Integer {
        let quotient = self.abs / other.unsigned_abs();
        if (other >= 0) == self.sign {
            Integer::from(quotient)
        } else {
            -quotient
        }
    }
}

impl<'a> Div<i32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference. The quotient is rounded
    /// towards zero.
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
    ///     assert_eq!((&Integer::from(23) / 10i32).to_string(), "2");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!((&Integer::from(23) / -10i32).to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23) / 10i32).to_string(), "-2");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23) / -10i32).to_string(), "2");
    /// }
    /// ```
    fn div(self, other: i32) -> Integer {
        let quotient = &self.abs / other.unsigned_abs();
        if (other >= 0) == self.sign {
            Integer::from(quotient)
        } else {
            -quotient
        }
    }
}

impl DivAssign<i32> for Integer {
    /// Divides an `Integer` by an `i32` in place. The quotient is rounded towards zero.
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
    ///     let mut x = Integer::from(23);
    ///     x /= 10i32;
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     let mut x = Integer::from(23);
    ///     x /= -10i32;
    ///     assert_eq!(x.to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x /= 10i32;
    ///     assert_eq!(x.to_string(), "-2");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x /= -10i32;
    ///     assert_eq!(x.to_string(), "2");
    /// }
    /// ```
    fn div_assign(&mut self, other: i32) {
        self.abs /= other.unsigned_abs();
        self.sign ^= other < 0;
        if !self.sign && self.abs == 0 {
            self.sign = true;
        }
    }
}

impl Div<Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value. The quotient is rounded
    /// towards zero.
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
    ///     assert_eq!((23i32 / Integer::from(10)).to_string(), "2");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!((23i32 / Integer::from(-10)).to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((-23i32 / Integer::from(10)).to_string(), "-2");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((-23i32 / Integer::from(-10)).to_string(), "2");
    /// }
    /// ```
    fn div(self, other: Integer) -> Integer {
        let quotient = self.unsigned_abs() / other.abs;
        if (self >= 0) == other.sign {
            Integer::from(quotient)
        } else {
            -Natural::from(quotient)
        }
    }
}

impl<'a> Div<&'a Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference. The quotient is rounded
    /// towards zero.
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
    ///     assert_eq!((23i32 / &Integer::from(10)).to_string(), "2");
    ///
    ///     // -2 * -10 + 3 = 23
    ///     assert_eq!((23i32 / &Integer::from(-10)).to_string(), "-2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((-23i32 / &Integer::from(10)).to_string(), "-2");
    ///
    ///     // 2 * -10 + -3 = -23
    ///     assert_eq!((-23i32 / &Integer::from(-10)).to_string(), "2");
    /// }
    /// ```
    fn div(self, other: &'a Integer) -> Integer {
        let quotient = self.unsigned_abs() / &other.abs;
        if (self >= 0) == other.sign {
            Integer::from(quotient)
        } else {
            -Natural::from(quotient)
        }
    }
}
