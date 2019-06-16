use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;

use integer::Integer;
use natural::Natural;
use platform::{Limb, SignedLimb};

impl DivExact<SignedLimb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by value. The `Integer` must be
    /// exactly divisible by the `SignedLimb`. If it isn't, the behavior of this function is
    /// undefined.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
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
    #[inline]
    fn div_exact(mut self, other: SignedLimb) -> Integer {
        self.div_exact_assign(other);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivExact<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: i32) -> Integer {
        self.div_exact(SignedLimb::from(other))
    }
}

impl<'a> DivExact<SignedLimb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb`, taking the `Integer` by reference. The `Integer`
    /// must be exactly divisible by the `SignedLimb`. If it isn't, the behavior of this function is
    /// undefined.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
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
    fn div_exact(self, other: SignedLimb) -> Integer {
        let abs = (&self.abs).div_exact(other.unsigned_abs());
        if abs == 0 as Limb {
            Integer::ZERO
        } else {
            Integer {
                sign: self.sign == (other >= 0),
                abs,
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivExact<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: i32) -> Integer {
        self.div_exact(SignedLimb::from(other))
    }
}

impl DivExactAssign<SignedLimb> for Integer {
    /// Divides an `Integer` by a `SignedLimb` in place. The `Integer` must be exactly divisible by
    /// the `SignedLimb`. If it isn't, the behavior of this function is undefined.
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
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
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
    fn div_exact_assign(&mut self, other: SignedLimb) {
        self.abs.div_exact_assign(other.unsigned_abs());
        self.sign = self.sign == (other >= 0) || self.abs == 0 as Limb
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivExactAssign<i32> for Integer {
    #[inline]
    fn div_exact_assign(&mut self, other: i32) {
        self.div_exact_assign(SignedLimb::from(other));
    }
}

impl DivExact<Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by value. The `SignedLimb` must
    /// be exactly divisible by the `Integer`. If it isn't, the behavior of this function is
    /// undefined.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
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

#[cfg(not(feature = "32_bit_limbs"))]
impl DivExact<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: Integer) -> Integer {
        SignedLimb::from(self).div_exact(other)
    }
}

impl<'a> DivExact<&'a Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer`, taking the `Integer` by reference. The `SignedLimb`
    /// must be exactly divisible by the `Integer`. If it isn't, the behavior of this function is
    /// undefined.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
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

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivExact<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self).div_exact(other)
    }
}
