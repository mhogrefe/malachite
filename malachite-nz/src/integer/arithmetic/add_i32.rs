use integer::Integer;
use malachite_base::num::Assign;
use natural::Natural;
use std::ops::{Add, AddAssign};

/// Adds an `i32` to an `Integer`, taking the `Integer` by value.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + -123i32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + 0i32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + -456i32).to_string(), "-579");
///     assert_eq!((-Integer::trillion() + -123i32).to_string(), "-1000000000123");
/// }
/// ```
impl Add<i32> for Integer {
    type Output = Integer;

    fn add(mut self, other: i32) -> Integer {
        self += other;
        self
    }
}

/// Adds an `i32` to an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO + -123i32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + 0i32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + -456i32).to_string(), "-579");
///     assert_eq!((&(-Integer::trillion()) + -123i32).to_string(), "-1000000000123");
/// }
/// ```
impl<'a> Add<i32> for &'a Integer {
    type Output = Integer;

    fn add(self, other: i32) -> Integer {
        if other == 0 {
            return self.clone();
        }
        let abs_other = other.wrapping_abs() as u32;
        match *self {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            Integer { sign, ref abs } if sign == (other > 0) => Integer {
                sign,
                abs: abs + abs_other,
            },
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            Integer { sign, ref abs } if sign && *abs == abs_other || *abs > abs_other => Integer {
                sign,
                abs: (abs - abs_other).unwrap(),
            },
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            Integer { ref sign, ref abs } => Integer {
                sign: !sign,
                abs: Natural::from(abs_other - abs.to_u32().unwrap()),
            },
        }
    }
}

/// Adds an `Integer` to an `i32`, taking the `Integer` by value.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((-123i32 + Integer::ZERO).to_string(), "-123");
///     assert_eq!((0i32 + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((-456i32 + Integer::from(-123)).to_string(), "-579");
///     assert_eq!((-123i32 + -Integer::trillion()).to_string(), "-1000000000123");
/// }
/// ```
impl Add<Integer> for i32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other.add_assign(self);
        other
    }
}

/// Adds an `Integer` to an `i32`, taking the `Integer` by reference.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((-123i32 + &Integer::ZERO).to_string(), "-123");
///     assert_eq!((0i32 + &Integer::from(-123)).to_string(), "-123");
///     assert_eq!((-456i32 + &Integer::from(-123)).to_string(), "-579");
///     assert_eq!((-123i32 + &(-Integer::trillion())).to_string(), "-1000000000123");
/// }
/// ```
impl<'a> Add<&'a Integer> for i32 {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}

/// Adds an `i32` to an `Integer` in place.
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
///     let mut x = Integer::ZERO;
///     x += 1;
///     x += -2;
///     x += 3;
///     x += -4;
///     assert_eq!(x.to_string(), "-2");
/// }
/// ```
impl AddAssign<i32> for Integer {
    fn add_assign(&mut self, other: i32) {
        if other == 0 {
            return;
        }
        let abs_other = other.wrapping_abs() as u32;
        match *self {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            Integer { sign, ref mut abs } if sign == (other > 0) => *abs += abs_other,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            Integer { sign, ref mut abs } if sign && *abs == abs_other || *abs > abs_other => {
                *abs -= abs_other;
            }
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            Integer {
                ref mut sign,
                ref mut abs,
            } => {
                *sign = !*sign;
                let small_abs = abs.to_u32().unwrap();
                abs.assign(abs_other - small_abs);
            }
        }
    }
}
