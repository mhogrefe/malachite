use integer::Integer;
use std::ops::{Add, AddAssign, SubAssign};
use traits::Assign;

/// Adds a `u32` to an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(0) + 123).to_string(), "123");
/// assert_eq!((Integer::from(-123) + 0).to_string(), "-123");
/// assert_eq!((Integer::from(-123) + 456).to_string(), "333");
/// assert_eq!((Integer::from_str("-1000000000000").unwrap() + 123).to_string(), "-999999999877");
/// ```
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, other: u32) -> Integer {
        self.add_assign(other);
        self
    }
}

/// Adds an `Integer` to a `u32`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((123 + Integer::from(0)).to_string(), "123");
/// assert_eq!((0 + Integer::from(-123)).to_string(), "-123");
/// assert_eq!((456 + Integer::from(-123)).to_string(), "333");
/// assert_eq!((123 + Integer::from_str("-1000000000000").unwrap()).to_string(), "-999999999877");
/// ```
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other.add_assign(self);
        other
    }
}

/// Adds a `u32` to an `Integer` in place.
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
        match *self {
            Integer { sign: true, ref mut abs } => {
                abs.add_assign(other);
                return;
            }
            Integer { sign: false, ref mut abs } if *abs > other => {
                abs.sub_assign(other);
                return;
            }
            _ => {}
        }
        // self < 0 and |self| <= other
        self.sign = true;
        let small_abs = self.abs.to_u32().unwrap();
        self.abs.assign(other - small_abs);
    }
}
