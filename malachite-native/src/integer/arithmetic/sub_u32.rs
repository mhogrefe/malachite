use integer::Integer;
use malachite_base::traits::{Assign, NegAssign};
use natural::Natural;
use std::ops::{Sub, SubAssign};

/// Subtracts a `u32` from an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((Integer::from(123) - 123u32).to_string(), "0");
/// assert_eq!((Integer::from(-123) - 0u32).to_string(), "-123");
/// assert_eq!((Integer::from(123) - 456u32).to_string(), "-333");
/// assert_eq!((Integer::trillion() - 123u32).to_string(), "999999999877");
/// ```
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, other: u32) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts a `u32` from an `Integer`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((&Integer::from(123) - 123u32).to_string(), "0");
/// assert_eq!((&Integer::from(-123) - 0u32).to_string(), "-123");
/// assert_eq!((&Integer::from(123) - 456u32).to_string(), "-333");
/// assert_eq!((&Integer::trillion() - 123u32).to_string(), "999999999877");
/// ```
impl<'a> Sub<u32> for &'a Integer {
    type Output = Integer;

    fn sub(self, other: u32) -> Integer {
        if other == 0 {
            return self.clone();
        }
        if *self == 0 {
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
            } if *abs >= other =>
            {
                Integer {
                    sign: true,
                    abs: (abs - other).unwrap(),
                }
            }
            // e.g. 5 - 10; self becomes negative
            Integer { ref abs, .. } => Integer {
                sign: false,
                abs: Natural::from(other - abs.to_u32().unwrap()),
            },
        }
    }
}

/// Subtracts an `Integer` from a `u32`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((123u32 - Integer::from(123)).to_string(), "0");
/// assert_eq!((0u32 - Integer::from(-123)).to_string(), "123");
/// assert_eq!((456u32 - Integer::from(123)).to_string(), "333");
/// assert_eq!((123u32 - Integer::trillion()).to_string(), "-999999999877");
/// ```
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

/// Subtracts an `Integer` from a `u32`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((123u32 - &Integer::from(123)).to_string(), "0");
/// assert_eq!((0u32 - &Integer::from(-123)).to_string(), "123");
/// assert_eq!((456u32 - &Integer::from(123)).to_string(), "333");
/// assert_eq!((123u32 - &Integer::trillion()).to_string(), "-999999999877");
/// ```
impl<'a> Sub<&'a Integer> for u32 {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        -(other - self)
    }
}

/// Subtracts a `u32` from an `Integer` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// let mut x = Integer::from(15);
/// x -= 1;
/// x -= 2;
/// x -= 3;
/// x -= 4;
/// assert_eq!(x.to_string(), "5");
/// ```
impl SubAssign<u32> for Integer {
    fn sub_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        if *self == 0 {
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
            } if *abs >= other =>
            {
                *abs -= other
            }
            // e.g. 5 - 10; self becomes negative
            Integer {
                ref mut sign,
                ref mut abs,
            } => {
                *sign = false;
                let small_abs = abs.to_u32().unwrap();
                abs.assign(other - small_abs);
            }
        }
    }
}
