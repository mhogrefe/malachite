use integer::Integer;
use std::ops::{AddAssign, Sub, SubAssign};
use traits::Assign;

/// Subtracts a `u32` from an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(123) - 123).to_string(), "0");
/// assert_eq!((Integer::from(-123) - 0).to_string(), "-123");
/// assert_eq!((Integer::from(123) - 456).to_string(), "-333");
/// assert_eq!((Integer::from_str("1000000000000").unwrap() - 123).to_string(), "999999999877");
/// ```
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, other: u32) -> Integer {
        self.sub_assign(other);
        self
    }
}

/// Subtracts an `Integer` from a `u32`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((123 - Integer::from(123)).to_string(), "0");
/// assert_eq!((0 - Integer::from(-123)).to_string(), "123");
/// assert_eq!((456 - Integer::from(123)).to_string(), "333");
/// assert_eq!((123 - Integer::from_str("1000000000000").unwrap()).to_string(), "-999999999877");
/// ```
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other.sub_assign(self);
        -other
    }
}

/// Subtracts a `u32` from an `Integer` in place.
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
        match *self {
            Integer { sign: false, ref mut abs } => {
                abs.add_assign(other);
                return;
            }
            Integer { sign: true, ref mut abs } if *abs >= other => {
                abs.sub_assign(other);
                return;
            }
            _ => {}
        }
        // self > 0 and self < other
        self.sign = false;
        let small_abs = self.abs.to_u32().unwrap();
        self.abs.assign(other - small_abs);
    }
}
