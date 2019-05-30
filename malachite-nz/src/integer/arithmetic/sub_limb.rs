use std::ops::{Sub, SubAssign};

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::conversion::traits::{Assign, CheckedFrom};

use integer::Integer;
use natural::Natural;
use platform::Limb;

/// Subtracts a `Limb` from an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(123) - 123u32).to_string(), "0");
/// assert_eq!((Integer::from(-123) - 0u32).to_string(), "-123");
/// assert_eq!((Integer::from(123) - 456u32).to_string(), "-333");
/// assert_eq!((Integer::trillion() - 123u32).to_string(), "999999999877");
/// ```
impl Sub<Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn sub(mut self, other: Limb) -> Integer {
        self -= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Sub<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn sub(self, other: u32) -> Integer {
        self - Limb::from(other)
    }
}

/// Subtracts a `Limb` from an `Integer`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((&Integer::from(123) - 123u32).to_string(), "0");
/// assert_eq!((&Integer::from(-123) - 0u32).to_string(), "-123");
/// assert_eq!((&Integer::from(123) - 456u32).to_string(), "-333");
/// assert_eq!((&Integer::trillion() - 123u32).to_string(), "999999999877");
/// ```
impl<'a> Sub<Limb> for &'a Integer {
    type Output = Integer;

    fn sub(self, other: Limb) -> Integer {
        if other == 0 {
            return self.clone();
        }
        if *self == 0 as Limb {
            return -Integer::from(other);
        }
        match *self {
            // e.g. -10 - 5; self stays negative
            Integer {
                sign: false,
                ref abs,
            } => Integer {
                sign: false,
                abs: abs + other,
            },
            // e.g. 10 - 5 or 5 - 5; self stays non-negative
            Integer {
                sign: true,
                ref abs,
            } if *abs >= other => Integer {
                sign: true,
                abs: abs - other,
            },
            // e.g. 5 - 10; self becomes negative
            Integer { ref abs, .. } => Integer {
                sign: false,
                abs: Natural::from(other - Limb::checked_from(abs).unwrap()),
            },
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Sub<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn sub(self, other: u32) -> Integer {
        self - Limb::from(other)
    }
}

/// Subtracts an `Integer` from a `Limb`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((123u32 - Integer::from(123)).to_string(), "0");
/// assert_eq!((0u32 - Integer::from(-123)).to_string(), "123");
/// assert_eq!((456u32 - Integer::from(123)).to_string(), "333");
/// assert_eq!((123u32 - Integer::trillion()).to_string(), "-999999999877");
/// ```
impl Sub<Integer> for Limb {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Sub<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn sub(self, other: Integer) -> Integer {
        Limb::from(self) - other
    }
}

/// Subtracts an `Integer` from a `Limb`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((123u32 - &Integer::from(123)).to_string(), "0");
/// assert_eq!((0u32 - &Integer::from(-123)).to_string(), "123");
/// assert_eq!((456u32 - &Integer::from(123)).to_string(), "333");
/// assert_eq!((123u32 - &Integer::trillion()).to_string(), "-999999999877");
/// ```
impl<'a> Sub<&'a Integer> for Limb {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        -(other - self)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Sub<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn sub(self, other: &'a Integer) -> Integer {
        Limb::from(self) - other
    }
}

/// Subtracts a `Limb` from an `Integer` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::from(15);
/// x -= 1;
/// x -= 2;
/// x -= 3;
/// x -= 4;
/// assert_eq!(x.to_string(), "5");
/// ```
impl SubAssign<Limb> for Integer {
    fn sub_assign(&mut self, other: Limb) {
        if other == 0 {
            return;
        }
        if *self == 0 as Limb {
            self.assign(other);
            self.neg_assign();
            return;
        }
        match *self {
            // e.g. -10 - 5; self stays negative
            Integer {
                sign: false,
                ref mut abs,
            } => *abs += other,
            // e.g. 10 - 5 or 5 - 5; self stays non-negative
            Integer {
                sign: true,
                ref mut abs,
            } if *abs >= other => *abs -= other,
            // e.g. 5 - 10; self becomes negative
            Integer {
                ref mut sign,
                ref mut abs,
            } => {
                *sign = false;
                let small_abs = Limb::checked_from(&*abs).unwrap();
                abs.assign(other - small_abs);
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SubAssign<u32> for Integer {
    #[inline]
    fn sub_assign(&mut self, other: u32) {
        *self -= Limb::from(other);
    }
}
