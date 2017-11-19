use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Sub, SubAssign};
use std::mem;
use malachite_base::traits::{NegAssign, Zero};

/// Subtracts an `Integer` from an `Integer`, taking both `Integer`s by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::zero() - Integer::from(123)).to_string(), "-123");
///     assert_eq!((Integer::from(123) - Integer::zero()).to_string(), "123");
///     assert_eq!((Integer::from(456) - Integer::from(-123)).to_string(), "579");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() -
///                 Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl Sub<Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, other: Integer) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts an `Integer` from an `Integer`, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::zero() - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((Integer::from(123) - &Integer::zero()).to_string(), "123");
///     assert_eq!((Integer::from(456) - &Integer::from(-123)).to_string(), "579");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() -
///                 &Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl<'a> Sub<&'a Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, other: &'a Integer) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts an `Integer` from an `Integer`, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::zero() - Integer::from(123)).to_string(), "-123");
///     assert_eq!((&Integer::from(123) - Integer::zero()).to_string(), "123");
///     assert_eq!((&Integer::from(456) - Integer::from(-123)).to_string(), "579");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() -
///                 Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl<'a> Sub<Integer> for &'a Integer {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

/// Subtracts an `Integer` from an `Integer`, taking both `Integer`s by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::zero() - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((&Integer::from(123) - &Integer::zero()).to_string(), "123");
///     assert_eq!((&Integer::from(456) - &Integer::from(-123)).to_string(), "579");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() -
///                 &Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl<'a, 'b> Sub<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        if self as *const Integer == other as *const Integer {
            Integer::zero()
        } else if *self == 0 {
            -other
        } else if *other == 0 {
            self.clone()
        } else if let Small(y) = *other {
            self - y
        } else if let Small(x) = *self {
            x - other
        } else {
            match (self, other) {
                (&Large(ref x), &Large(ref y)) => unsafe {
                    let mut result: mpz_t = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_sub(&mut result, x, y);
                    let mut result = Large(result);
                    result.demote_if_small();
                    result
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Subtracts an `Integer` from an `Integer` in place, taking the `Integer` on the RHS by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::zero();
///     x -= Integer::from_str("-1000000000000").unwrap();
///     x -= Integer::from_str("2000000000000").unwrap();
///     x -= Integer::from_str("-3000000000000").unwrap();
///     x -= Integer::from_str("4000000000000").unwrap();
///     assert_eq!(x.to_string(), "-2000000000000");
/// }
/// ```
impl SubAssign<Integer> for Integer {
    fn sub_assign(&mut self, mut other: Integer) {
        if *self == 0 {
            *self = other;
            self.neg_assign();
        } else if other == 0 {
        } else if let Small(y) = other {
            *self -= y;
        } else if let Small(x) = *self {
            other -= x;
            *self = -other;
        } else {
            match ((&mut (*self)), other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_sub(x, x, y);
                },
                _ => unreachable!(),
            }
            self.demote_if_small();
        }
    }
}

/// Subtracts an `Integer` from an `Integer` in place, taking the `Integer` on the RHS by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::zero();
///     x -= &Integer::from_str("-1000000000000").unwrap();
///     x -= &Integer::from_str("2000000000000").unwrap();
///     x -= &Integer::from_str("-3000000000000").unwrap();
///     x -= &Integer::from_str("4000000000000").unwrap();
///     assert_eq!(x.to_string(), "-2000000000000");
/// }
/// ```
impl<'a> SubAssign<&'a Integer> for Integer {
    fn sub_assign(&mut self, other: &'a Integer) {
        if *self == 0 {
            self.clone_from(other);
            self.neg_assign();
        } else if *other == 0 {
        } else if let Small(y) = *other {
            *self -= y;
        } else if let Small(x) = *self {
            *self = x - other;
        } else {
            match ((&mut (*self)), other) {
                (&mut Large(ref mut x), &Large(ref y)) => unsafe {
                    gmp::mpz_sub(x, x, y);
                },
                _ => unreachable!(),
            }
            self.demote_if_small();
        }
    }
}
