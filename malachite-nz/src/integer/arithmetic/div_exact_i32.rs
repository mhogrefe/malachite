use integer::Integer;
use malachite_base::num::{DivExact, DivExactAssign, UnsignedAbs, Zero};
use natural::Natural;
use std::i32;

impl DivExact<i32> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by value. The `Integer` must be
    /// exactly divisible by the `i32`. If it isn't, the behavior of this function is undefined.
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
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     assert_eq!(Integer::from(-369).div_exact(123i32).to_string(), "-3");
    ///
    ///     // 8,130,081,300 * -123 = -999,999,999,900
    ///     assert_eq!(Integer::from_str("-999999999900").unwrap().div_exact(-123i32).to_string(),
    ///         "8130081300");
    /// }
    /// ```
    fn div_exact(mut self, other: i32) -> Integer {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<i32> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `i32`, taking the `Integer` by reference. The `Integer` must be
    /// exactly divisible by the `i32`. If it isn't, the behavior of this function is undefined.
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
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // -3 * 123 = -369
    ///     assert_eq!((&Integer::from(-369)).div_exact(123i32).to_string(), "-3");
    ///
    ///     // 8,130,081,300 * -123 = -999,999,999,900
    ///     assert_eq!((&Integer::from_str("-999999999900").unwrap()).div_exact(-123i32)
    ///         .to_string(), "8130081300");
    /// }
    /// ```
    fn div_exact(self, other: i32) -> Integer {
        let abs = (&self.abs).div_exact(other.unsigned_abs());
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: self.sign == (other >= 0),
                abs,
            }
        }
    }
}

impl DivExactAssign<i32> for Integer {
    /// Divides an `Integer` by an `i32` in place. The `Integer` must be exactly divisible by the
    /// `i32`. If it isn't, the behavior of this function is undefined.
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
    /// use malachite_base::num::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // -3 * 123 = -369
    ///     let mut x = Integer::from(-369);
    ///     x.div_exact_assign(123i32);
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // -8,130,081,300 * 123 = -999,999,999,900
    ///     let mut x = Integer::from_str("-999999999900").unwrap();
    ///     x.div_exact_assign(123);
    ///     assert_eq!(x.to_string(), "-8130081300");
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: i32) {
        self.abs.div_exact_assign(other.unsigned_abs());
        self.sign = self.sign == (other >= 0) || self.abs == 0
    }
}

impl DivExact<Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by value. The `i32` must be exactly
    /// divisible by the `Integer`. If it isn't, the behavior of this function is undefined.
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
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * -123 = -369
    ///     assert_eq!((-369i32).div_exact(Integer::from(-123)).to_string(), "3");
    /// }
    /// ```
    fn div_exact(self, other: Integer) -> Integer {
        let abs = self.unsigned_abs().div_exact(other.abs);
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: (self >= 0) == other.sign,
                abs: Natural::from(abs),
            }
        }
    }
}

impl<'a> DivExact<&'a Integer> for i32 {
    type Output = Integer;

    /// Divides an `i32` by an `Integer`, taking the `Integer` by reference. The `i32` must be
    /// exactly divisible by the `Integer`. If it isn't, the behavior of this function is undefined.
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
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     // 3 * -123 = -369
    ///     assert_eq!((-369i32).div_exact(&Integer::from(-123)).to_string(), "3");
    /// }
    /// ```
    fn div_exact(self, other: &'a Integer) -> Integer {
        let abs = self.unsigned_abs().div_exact(&other.abs);
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: (self >= 0) == other.sign,
                abs: Natural::from(abs),
            }
        }
    }
}
