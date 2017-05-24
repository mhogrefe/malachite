use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::ops::{Add, AddAssign};

/// Adds a `Integer` to a `Integer`, taking ownership of both `Integer`s.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(0) + Integer::from(123)).to_string(), "123");
/// assert_eq!((Integer::from(-123) + Integer::from(0)).to_string(), "-123");
/// assert_eq!((Integer::from(-123) + Integer::from(456)).to_string(), "333");
/// assert_eq!((Integer::from_str("-1000000000000").unwrap() + Integer::from_str("2000000000000")
///            .unwrap()).to_string(), "1000000000000");
/// ```
impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(mut self, other: Integer) -> Integer {
        self += other;
        self
    }
}

/// Adds a `Integer` to a `Integer` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// let mut x = Integer::new();
/// x += Integer::from_str("-1000000000000").unwrap();
/// x += Integer::from_str("2000000000000").unwrap();
/// x += Integer::from_str("-3000000000000").unwrap();
/// x += Integer::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "2000000000000");
/// ```
impl AddAssign<Integer> for Integer {
    fn add_assign(&mut self, mut other: Integer) {
        if other == 0 {
            return;
        }
        if let Small(y) = other {
            *self += y;
        } else if let Small(x) = *self {
            other += x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_add(x, x, y);
                },
                _ => unreachable!(),
            }
        }
        self.demote_if_small();
    }
}
