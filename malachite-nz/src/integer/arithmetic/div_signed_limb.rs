use integer::Integer;
use malachite_base::num::UnsignedAbs;
use natural::Natural;
use platform::SignedLimb;
use std::ops::{Div, DivAssign};

impl Div<SignedLimb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value. The quotient is
    /// rounded towards zero.
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
    #[inline]
    fn div(mut self, other: SignedLimb) -> Integer {
        self /= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Div<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn div(self, other: i32) -> Integer {
        self / SignedLimb::from(other)
    }
}

impl<'a> Div<SignedLimb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference. The quotient is
    /// rounded towards zero.
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
    fn div(self, other: SignedLimb) -> Integer {
        let quotient = &self.abs / other.unsigned_abs();
        if (other >= 0) == self.sign {
            Integer::from(quotient)
        } else {
            -quotient
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Div<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn div(self, other: i32) -> Integer {
        self / SignedLimb::from(other)
    }
}

impl DivAssign<SignedLimb> for Integer {
    /// Divides an `Integer` by a `SignedLimb` in place. The quotient is rounded towards zero.
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
    fn div_assign(&mut self, other: SignedLimb) {
        self.abs /= other.unsigned_abs();
        self.sign ^= other < 0;
        if !self.sign && self.abs == 0 {
            self.sign = true;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivAssign<i32> for Integer {
    #[inline]
    fn div_assign(&mut self, other: i32) {
        *self /= SignedLimb::from(other);
    }
}

impl Div<Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value. The quotient is
    /// rounded towards zero.
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

#[cfg(feature = "64_bit_limbs")]
impl Div<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn div(self, other: Integer) -> Integer {
        SignedLimb::from(self) / other
    }
}

impl<'a> Div<&'a Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference. The quotient is
    /// rounded towards zero.
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

#[cfg(feature = "64_bit_limbs")]
impl<'a> Div<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn div(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self) / other
    }
}
