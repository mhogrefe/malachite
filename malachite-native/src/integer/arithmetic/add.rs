use integer::Integer;
use std::mem::swap;
use std::ops::{Add, AddAssign};

/// Adds a `Integer` to a `Integer`, taking ownership of both `Integer`s.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
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
/// use malachite_native::integer::Integer;
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
        let add_strategy;
        match (&mut (*self), &other) {
            (&mut Integer { sign: sx, .. },
             &Integer {
                 sign: sy,
                 abs: ref ay,
             }) if sx == (sy && *ay != 0) => add_strategy = 0,
            (&mut Integer {
                 sign: sx,
                 abs: ref mut ax,
             },
             &Integer { abs: ref ay, .. }) if sx && *ax == *ay || *ax > *ay => add_strategy = 1,
            _ => add_strategy = 2,
        }
        match add_strategy {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            0 => self.abs += other.abs,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            1 => self.abs -= &other.abs,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= &other.abs;
            }
        }
    }
}
