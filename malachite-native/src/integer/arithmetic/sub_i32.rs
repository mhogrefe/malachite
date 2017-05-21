use integer::Integer;
use std::ops::{Sub, SubAssign};
use traits::{Assign, NegAssign};

/// Subtracts an `i32` from an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(0) - 123i32).to_string(), "-123");
/// assert_eq!((Integer::from(-123) - -0i32).to_string(), "-123");
/// assert_eq!((Integer::from(-123) - 456i32).to_string(), "-579");
/// assert_eq!((Integer::from_str("-1000000000000").unwrap() - 123i32).to_string(),
///            "-1000000000123");
/// ```
impl Sub<i32> for Integer {
    type Output = Integer;

    fn sub(mut self, other: i32) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts an `Integer` from an `i32`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((-123i32 - Integer::from(0)).to_string(), "-123");
/// assert_eq!((0i32 - Integer::from(123)).to_string(), "-123");
/// assert_eq!((-456i32 - Integer::from(123)).to_string(), "-579");
/// assert_eq!((-123i32 - Integer::from_str("1000000000000").unwrap()).to_string(),
///            "-1000000000123");
/// ```
impl Sub<Integer> for i32 {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

/// Subtracts an `i32` from an `Integer` in place.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// let mut x = Integer::new();
/// x -= -1;
/// x -= 2;
/// x -= -3;
/// x -= 4;
/// assert_eq!(x.to_string(), "-2");
/// ```
impl SubAssign<i32> for Integer {
    fn sub_assign(&mut self, other: i32) {
        if other == 0 {
            return;
        }
        if *self == 0 {
            self.assign(other);
            self.neg_assign();
            return;
        }
        let abs_other = other.wrapping_abs() as u32;
        match *self {
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            Integer { sign, ref mut abs } if sign == (other < 0) => *abs += abs_other,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            Integer { sign, ref mut abs } if sign && *abs == abs_other || *abs > abs_other => {
                *abs -= abs_other;
            }
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of self is flipped
            Integer { ref mut sign, ref mut abs } => {
                *sign = !*sign;
                let small_abs = abs.to_u32().unwrap();
                abs.assign(abs_other - small_abs);
            }
        }
    }
}
