use integer::Integer;
use malachite_base::num::{Assign, Zero};
use platform::Limb;
use std::ops::{Mul, MulAssign};

impl Mul<Limb> for Integer {
    type Output = Integer;

    /// Multiplies an `Integer` by a `Limb`, taking the `Integer` by value.
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((Integer::ZERO * 123u32).to_string(), "0");
    ///     assert_eq!((Integer::from(123i32) * 1u32).to_string(), "123");
    ///     assert_eq!((Integer::from(-123i32) * 456u32).to_string(), "-56088");
    ///     assert_eq!(((-Integer::trillion()) * 123u32).to_string(), "-123000000000000");
    /// }
    /// ```
    #[inline]
    fn mul(mut self, other: Limb) -> Integer {
        self *= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Mul<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn mul(self, other: u32) -> Integer {
        self * Limb::from(other)
    }
}

impl<'a> Mul<Limb> for &'a Integer {
    type Output = Integer;

    /// Multiplies an `Integer` by a `Limb`, taking the `Integer` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO * 123u32).to_string(), "0");
    ///     assert_eq!((&Integer::from(123i32) * 1u32).to_string(), "123");
    ///     assert_eq!((&Integer::from(-123i32) * 456u32).to_string(), "-56088");
    ///     assert_eq!((&(-Integer::trillion()) * 123u32).to_string(), "-123000000000000");
    /// }
    /// ```
    fn mul(self, other: Limb) -> Integer {
        if *self == 0 as Limb || other == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: self.sign,
                abs: &self.abs * other,
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Mul<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn mul(self, other: u32) -> Integer {
        self * Limb::from(other)
    }
}

impl Mul<Integer> for Limb {
    type Output = Integer;

    /// Multiplies a `Limb` by an `Integer`, taking the `Integer` by value.
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((123u32 * Integer::ZERO).to_string(), "0");
    ///     assert_eq!((1u32 * Integer::from(123i32)).to_string(), "123");
    ///     assert_eq!((456u32 * Integer::from(-123i32)).to_string(), "-56088");
    ///     assert_eq!((123u32 * -Integer::trillion()).to_string(), "-123000000000000");
    /// }
    /// ```
    #[inline]
    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Mul<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn mul(self, other: Integer) -> Integer {
        Limb::from(self) * other
    }
}

impl<'a> Mul<&'a Integer> for Limb {
    type Output = Integer;

    /// Multiplies a `Limb` by an `Integer`, taking the `Integer` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((123u32 * &Integer::ZERO).to_string(), "0");
    ///     assert_eq!((1u32 * &Integer::from(123i32)).to_string(), "123");
    ///     assert_eq!((456u32 * &Integer::from(-123i32)).to_string(), "-56088");
    ///     assert_eq!((123u32 * &(-Integer::trillion())).to_string(), "-123000000000000");
    /// }
    /// ```
    #[inline]
    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Mul<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn mul(self, other: &'a Integer) -> Integer {
        Limb::from(self) * other
    }
}

impl MulAssign<Limb> for Integer {
    /// Multiplies an `Integer` by a `Limb` in place.
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
    /// use malachite_base::num::NegativeOne;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut x = Integer::NEGATIVE_ONE;
    ///     x *= 1u32;
    ///     x *= 2u32;
    ///     x *= 3u32;
    ///     x *= 4u32;
    ///     assert_eq!(x.to_string(), "-24");
    /// }
    /// ```
    fn mul_assign(&mut self, other: Limb) {
        if *self == 0 as Limb || other == 0 {
            self.assign(0 as Limb);
        } else {
            self.abs *= other;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl MulAssign<u32> for Integer {
    #[inline]
    fn mul_assign(&mut self, other: u32) {
        *self *= Limb::from(other);
    }
}
