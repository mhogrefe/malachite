use integer::Integer;
use malachite_base::traits::Assign;
use std::ops::{Add, AddAssign};

/// Adds a `u32` to an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + 123u32).to_string(), "123");
///     assert_eq!((Integer::from(-123) + 0u32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + 456u32).to_string(), "333");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() + 123u32).to_string(),
///                "-999999999877");
/// }
/// ```
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, other: u32) -> Integer {
        self += other;
        self
    }
}

/// Adds a `u32` to an `Integer`, taking the `Integer` by reference.
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + 123u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + 0u32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + 456u32).to_string(), "333");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() + 123u32).to_string(),
///                "-999999999877");
/// }
/// ```
impl<'a> Add<u32> for &'a Integer {
    type Output = Integer;

    fn add(self, other: u32) -> Integer {
        if other == 0 {
            return self.clone();
        }
        if *self == 0 {
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
                abs: (abs - other).unwrap(),
            },
            // e.g. -5 + 10 or -5 + 5; self becomes non-negative
            Integer { ref abs, .. } => Integer::from(other - abs.to_u32().unwrap()),
        }
    }
}

/// Adds an `Integer` to a `u32`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123u32 + Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((456u32 + Integer::from(-123)).to_string(), "333");
///     assert_eq!((123u32 + Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-999999999877");
/// }
/// ```
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds an `Integer` to a `u32`, taking the `Integer` by reference.
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123u32 + &Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 + &Integer::from(-123)).to_string(), "-123");
///     assert_eq!((456u32 + &Integer::from(-123)).to_string(), "333");
///     assert_eq!((123u32 + &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-999999999877");
/// }
/// ```
impl<'a> Add<&'a Integer> for u32 {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}

/// Adds a `u32` to an `Integer` in place.
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
/// let mut x = Integer::from(-10);
/// x += 1;
/// x += 2;
/// x += 3;
/// x += 4;
/// assert_eq!(x.to_string(), "0");
/// ```
impl AddAssign<u32> for Integer {
    fn add_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        if *self == 0 {
            self.assign(other);
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
                let small_abs = abs.to_u32().unwrap();
                abs.assign(other - small_abs);
            }
        }
    }
}
