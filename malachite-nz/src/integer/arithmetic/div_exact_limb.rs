use integer::Integer;
use malachite_base::num::{DivExact, DivExactAssign, Zero};
use natural::Natural;
use platform::Limb;

impl DivExact<Limb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by value. The `Integer` must be
    /// exactly divisible by the `Limb`. If it isn't, the behavior of this function is undefined.
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
    ///     // -3 * 123 = -369
    ///     assert_eq!(Integer::from(-369).div_exact(123u32).to_string(), "-3");
    ///
    ///     // -8,130,081,300 * 123 = -999,999,999,900
    ///     assert_eq!(Integer::from_str("-999999999900").unwrap().div_exact(123u32).to_string(),
    ///         "-8130081300");
    /// }
    /// ```
    #[inline]
    fn div_exact(mut self, other: Limb) -> Integer {
        self.div_exact_assign(other);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivExact<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: u32) -> Integer {
        self.div_exact(Limb::from(other))
    }
}

impl<'a> DivExact<Limb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb`, taking the `Integer` by reference. The `Integer` must be
    /// exactly divisible by the `Limb`. If it isn't, the behavior of this function is undefined.
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
    ///     assert_eq!((&Integer::from(-369)).div_exact(123u32).to_string(), "-3");
    ///
    ///     // -8,130,081,300 * 123 = -999,999,999,900
    ///     assert_eq!((&Integer::from_str("-999999999900").unwrap()).div_exact(123u32).to_string(),
    ///         "-8130081300");
    /// }
    /// ```
    fn div_exact(self, other: Limb) -> Integer {
        let abs = (&self.abs).div_exact(other);
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: self.sign,
                abs,
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivExact<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: u32) -> Integer {
        self.div_exact(Limb::from(other))
    }
}

impl DivExactAssign<Limb> for Integer {
    /// Divides an `Integer` by a `Limb` in place. The `Integer` must be exactly divisible by the
    /// `Limb`. If it isn't, the behavior of this function is undefined.
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
    ///     x.div_exact_assign(123u32);
    ///     assert_eq!(x.to_string(), "-3");
    ///
    ///     // -8,130,081,300 * 123 = -999,999,999,900
    ///     let mut x = Integer::from_str("-999999999900").unwrap();
    ///     x.div_exact_assign(123u32);
    ///     assert_eq!(x.to_string(), "-8130081300");
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: Limb) {
        self.abs.div_exact_assign(other);
        self.sign |= self.abs == 0;
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivExactAssign<u32> for Integer {
    #[inline]
    fn div_exact_assign(&mut self, other: u32) {
        self.div_exact_assign(Limb::from(other));
    }
}

impl DivExact<Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by value. The `Limb` must be exactly
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
    ///     // -3 * -123 = 369
    ///     assert_eq!(369u32.div_exact(Integer::from(-123)).to_string(), "-3");
    /// }
    /// ```
    fn div_exact(self, other: Integer) -> Integer {
        let abs = self.div_exact(other.abs);
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: other.sign,
                abs: Natural::from(abs),
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivExact<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: Integer) -> Integer {
        Limb::from(self).div_exact(other)
    }
}

impl<'a> DivExact<&'a Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer`, taking the `Integer` by reference. The `Limb` must be
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
    ///     // -3 * -123 = 369
    ///     assert_eq!(369u32.div_exact(&Integer::from(-123)).to_string(), "-3");
    /// }
    /// ```
    fn div_exact(self, other: &'a Integer) -> Integer {
        let abs = self.div_exact(&other.abs);
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: other.sign,
                abs: Natural::from(abs),
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivExact<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn div_exact(self, other: &'a Integer) -> Integer {
        Limb::from(self).div_exact(other)
    }
}
