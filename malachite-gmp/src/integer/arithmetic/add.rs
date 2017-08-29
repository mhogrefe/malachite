use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Add, AddAssign};
use std::mem;

/// Adds an `Integer` to an `Integer`, taking both `Integer`s by value.
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

/// Adds an `Integer` to an `Integer`, taking the left `Integer` by value and the right `Integer` by
/// reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(0) + &Integer::from(123)).to_string(), "123");
/// assert_eq!((Integer::from(-123) + &Integer::from(0)).to_string(), "-123");
/// assert_eq!((Integer::from(-123) + &Integer::from(456)).to_string(), "333");
/// assert_eq!((Integer::from_str("-1000000000000").unwrap() + &Integer::from_str("2000000000000")
///            .unwrap()).to_string(), "1000000000000");
/// ```
impl<'a> Add<&'a Integer> for Integer {
    type Output = Integer;

    fn add(mut self, other: &'a Integer) -> Integer {
        self += other;
        self
    }
}

/// Adds an `Integer` to an `Integer`, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(0) + Integer::from(123)).to_string(), "123");
/// assert_eq!((&Integer::from(-123) + Integer::from(0)).to_string(), "-123");
/// assert_eq!((&Integer::from(-123) + Integer::from(456)).to_string(), "333");
/// assert_eq!((&Integer::from_str("-1000000000000").unwrap() + Integer::from_str("2000000000000")
///            .unwrap()).to_string(), "1000000000000");
/// ```
impl<'a> Add<Integer> for &'a Integer {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds an `Integer` to an `Integer`, taking both `Integer`s by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(0) + &Integer::from(123)).to_string(), "123");
/// assert_eq!((&Integer::from(-123) + &Integer::from(0)).to_string(), "-123");
/// assert_eq!((&Integer::from(-123) + &Integer::from(456)).to_string(), "333");
/// assert_eq!((&Integer::from_str("-1000000000000").unwrap() + &Integer::from_str("2000000000000")
///            .unwrap()).to_string(), "1000000000000");
/// ```
impl<'a, 'b> Add<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        if *self == 0 {
            return other.clone();
        } else if *other == 0 {
            return self.clone();
        }
        if let Small(y) = *other {
            self + y
        } else if let Small(x) = *self {
            other + x
        } else {
            match (self, other) {
                (&Large(ref x), &Large(ref y)) => unsafe {
                    let mut result: mpz_t = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_add(&mut result, x, y);
                    let mut result = Large(result);
                    result.demote_if_small();
                    result
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Adds an `Integer` to an `Integer` in place, taking the `Integer` on the RHS by value.
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
        if *self == 0 {
            *self = other;
        } else if other == 0 {
        } else if let Small(y) = other {
            *self += y;
        } else if let Small(x) = *self {
            other += x;
            *self = other;
        } else {
            match ((&mut (*self)), other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_add(x, x, y);
                },
                _ => unreachable!(),
            }
            self.demote_if_small();
        }
    }
}

/// Adds an `Integer` to an `Integer` in place, taking the `Integer` on the RHS by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// let mut x = Integer::new();
/// x += &Integer::from_str("-1000000000000").unwrap();
/// x += &Integer::from_str("2000000000000").unwrap();
/// x += &Integer::from_str("-3000000000000").unwrap();
/// x += &Integer::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "2000000000000");
/// ```
impl<'a> AddAssign<&'a Integer> for Integer {
    fn add_assign(&mut self, other: &'a Integer) {
        if *self == 0 {
            self.clone_from(other);
        } else if *other == 0 {
        } else if let Small(y) = *other {
            *self += y;
        } else if let Small(x) = *self {
            *self = other + x;
        } else {
            match ((&mut (*self)), other) {
                (&mut Large(ref mut x), &Large(ref y)) => unsafe {
                    gmp::mpz_add(x, x, y);
                },
                _ => unreachable!(),
            }
            self.demote_if_small();
        }
    }
}
