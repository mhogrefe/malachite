use std::ops::{Add, AddAssign};

use malachite_base::num::conversion::traits::CheckedFrom;

use integer::Integer;
use natural::Natural;
use platform::Limb;

/// Adds a `Limb` to an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + 123u32).to_string(), "123");
///     assert_eq!((Integer::from(-123) + 0u32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + 456u32).to_string(), "333");
///     assert_eq!(((-Integer::trillion()) + 123u32).to_string(), "-999999999877");
/// }
/// ```
impl Add<Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn add(mut self, other: Limb) -> Integer {
        self += other;
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Add<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn add(self, other: u32) -> Integer {
        self + Limb::from(other)
    }
}

/// Adds a `Limb` to an `Integer`, taking the `Integer` by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + 123u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + 0u32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + 456u32).to_string(), "333");
///     assert_eq!((&(-Integer::trillion()) + 123u32).to_string(), "-999999999877");
/// }
/// ```
impl<'a> Add<Limb> for &'a Integer {
    type Output = Integer;

    fn add(self, other: Limb) -> Integer {
        if other == 0 {
            return self.clone();
        }
        if *self == 0 as Limb {
            return Integer::from(other);
        }
        match *self {
            // e.g. 10 + 5; self stays positive
            Integer {
                sign: true,
                ref abs,
            } => Integer {
                sign: true,
                abs: abs + other,
            },
            // e.g. -10 + 5; self stays negative
            Integer {
                sign: false,
                ref abs,
            } if *abs > other => Integer {
                sign: false,
                abs: abs - other,
            },
            // e.g. -5 + 10 or -5 + 5; self becomes non-negative
            Integer { ref abs, .. } => Integer::from(other - Limb::checked_from(abs).unwrap()),
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Add<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add(self, other: u32) -> Integer {
        self + Limb::from(other)
    }
}

/// Adds an `Integer` to a `Limb`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((123u32 + Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((456u32 + Integer::from(-123)).to_string(), "333");
///     assert_eq!((123u32 + -Integer::trillion()).to_string(), "-999999999877");
/// }
/// ```
impl Add<Integer> for Limb {
    type Output = Integer;

    #[inline]
    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Add<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn add(self, other: Integer) -> Integer {
        Limb::from(self) + other
    }
}

/// Adds an `Integer` to a `Limb`, taking the `Integer` by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((123u32 + &Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 + &Integer::from(-123)).to_string(), "-123");
///     assert_eq!((456u32 + &Integer::from(-123)).to_string(), "333");
///     assert_eq!((123u32 + &(-Integer::trillion())).to_string(), "-999999999877");
/// }
/// ```
impl<'a> Add<&'a Integer> for Limb {
    type Output = Integer;

    #[inline]
    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Add<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn add(self, other: &'a Integer) -> Integer {
        Limb::from(self) + other
    }
}

/// Adds a `Limb` to an `Integer` in place.
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
/// let mut x = Integer::from(-10);
/// x += 1;
/// x += 2;
/// x += 3;
/// x += 4;
/// assert_eq!(x.to_string(), "0");
/// ```
impl AddAssign<Limb> for Integer {
    fn add_assign(&mut self, other: Limb) {
        if other == 0 {
            return;
        }
        if *self == 0 as Limb {
            *self = Integer::from(other);
            return;
        }
        match *self {
            // e.g. 10 + 5; self stays positive
            Integer {
                sign: true,
                ref mut abs,
            } => *abs += other,
            // e.g. -10 + 5; self stays negative
            Integer {
                sign: false,
                ref mut abs,
            } if *abs > other => *abs -= other,
            // e.g. -5 + 10 or -5 + 5; self becomes non-negative
            Integer {
                ref mut sign,
                ref mut abs,
            } => {
                *sign = true;
                let small_abs = Limb::checked_from(&*abs).unwrap();
                *abs = Natural::from(other - small_abs);
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl AddAssign<u32> for Integer {
    #[inline]
    fn add_assign(&mut self, other: u32) {
        *self += Limb::from(other);
    }
}
