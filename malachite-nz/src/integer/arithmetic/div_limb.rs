use std::ops::{Div, DivAssign};

use integer::Integer;
use natural::Natural;
use platform::Limb;

impl Div<Limb> for Integer {
    type Output = Integer;

    /// Divides a `Integer` by a `Limb`, taking the `Integer` by value. The quotient is rounded
    /// towards zero.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Integer::from(23u32) / 10u32).to_string(), "2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((Integer::from(-23) / 10u32).to_string(), "-2");
    /// }
    /// ```
    #[inline]
    fn div(mut self, other: Limb) -> Integer {
        self /= other;
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Div<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn div(self, other: u32) -> Integer {
        self / Limb::from(other)
    }
}

impl<'a> Div<Limb> for &'a Integer {
    type Output = Integer;

    /// Divides a `Integer` by a `Limb`, taking the `Integer` by reference. The quotient is rounded
    /// towards zero.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Integer::from(23u32) / 10u32).to_string(), "2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!((&Integer::from(-23) / 10u32).to_string(), "-2");
    /// }
    /// ```
    fn div(self, other: Limb) -> Integer {
        let quotient = &self.abs / other;
        if self.sign {
            Integer::from(quotient)
        } else {
            -quotient
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Div<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn div(self, other: u32) -> Integer {
        self / Limb::from(other)
    }
}

impl DivAssign<Limb> for Integer {
    /// Divides a `Integer` by a `Limb` in place. The quotient is rounded towards zero.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Integer::from(23u32);
    ///     x /= 10u32;
    ///     assert_eq!(x.to_string(), "2");
    ///
    ///     // -2 * 10 + -3 = -23
    ///     let mut x = Integer::from(-23);
    ///     x /= 10u32;
    ///     assert_eq!(x.to_string(), "-2");
    /// }
    /// ```
    fn div_assign(&mut self, other: Limb) {
        self.abs /= other;
        self.sign |= self.abs == 0 as Limb;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssign<u32> for Integer {
    #[inline]
    fn div_assign(&mut self, other: u32) {
        *self /= Limb::from(other);
    }
}

impl Div<Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by a `Integer`, taking the `Integer` by value.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23u32 / Integer::from(10u32), 2);
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(23u32 / Integer::from(-10), -2);
    /// }
    /// ```
    fn div(self, other: Integer) -> Integer {
        let non_negative = other.sign;
        let quotient = self / other.abs;
        if non_negative {
            Integer::from(quotient)
        } else {
            -Natural::from(quotient)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Div<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn div(self, other: Integer) -> Integer {
        Limb::from(self) / other
    }
}

impl<'a> Div<&'a Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by a `Integer`, taking the `Integer` by reference. The quotient is rounded
    /// towards zero.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23u32 / &Integer::from(10u32), 2);
    ///
    ///     // -2 * 10 + -3 = -23
    ///     assert_eq!(23u32 / &Integer::from(-10), -2);
    /// }
    /// ```
    fn div(self, other: &'a Integer) -> Integer {
        let quotient = self / &other.abs;
        if other.sign {
            Integer::from(quotient)
        } else {
            -Natural::from(quotient)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Div<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn div(self, other: &'a Integer) -> Integer {
        Limb::from(self) / other
    }
}
