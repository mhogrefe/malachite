use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::{Mul, MulAssign};

/// Multiplies an `Integer` by an `Integer`, taking both `Integer`s by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::{One, Zero};
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::one() * Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(123) * Integer::zero()).to_string(), "0");
///     assert_eq!((Integer::from(123) * Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-123456789000").unwrap() * Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl Mul<Integer> for Integer {
    type Output = Integer;

    fn mul(mut self, other: Integer) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by an `Integer`, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::{One, Zero};
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::one() * &Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(123) * &Integer::zero()).to_string(), "0");
///     assert_eq!((Integer::from(123) * &Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-123456789000").unwrap() *
///                 &Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for Integer {
    type Output = Integer;

    fn mul(mut self, other: &'a Integer) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by an `Integer`, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::{One, Zero};
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::one() * Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(123) * Integer::zero()).to_string(), "0");
///     assert_eq!((&Integer::from(123) * Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-123456789000").unwrap() *
///                 Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Integer> for &'a Integer {
    type Output = Integer;

    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

/// Multiplies an `Integer` by an `Integer`, taking both `Integer`s by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::{One, Zero};
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::one() * &Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(123) * &Integer::zero()).to_string(), "0");
///     assert_eq!((&Integer::from(123) * &Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-123456789000").unwrap() *
///                 &Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn mul(self, other: &'a Integer) -> Integer {
        if *self == 0 || *other == 0 {
            return Small(0);
        } else if *self == 1 {
            return other.clone();
        } else if *other == 1 {
            return self.clone();
        }
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref x), &Large(ref y)) => unsafe {
                    let mut result: mpz_t = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_mul(&mut result, x, y);
                    Large(result)
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies an `Integer` by an `Integer` in place, taking the `Integer` on the RHS by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::NegativeOne;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::negative_one();
///     x *= Integer::from_str("1000").unwrap();
///     x *= Integer::from_str("2000").unwrap();
///     x *= Integer::from_str("3000").unwrap();
///     x *= Integer::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "-24000000000000");
/// }
/// ```
impl MulAssign<Integer> for Integer {
    fn mul_assign(&mut self, mut other: Integer) {
        if other == 0 {
            *self = Small(0);
        } else if *self == 1 {
            *self = other;
        } else if *self == 0 || other == 1 {
        } else if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_mul(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies an `Integer` by an `Integer` in place, taking the `Integer` on the RHS by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::NegativeOne;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::negative_one();
///     x *= &Integer::from_str("1000").unwrap();
///     x *= &Integer::from_str("2000").unwrap();
///     x *= &Integer::from_str("3000").unwrap();
///     x *= &Integer::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "-24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Integer> for Integer {
    fn mul_assign(&mut self, other: &'a Integer) {
        if *other == 0 {
            *self = Small(0);
        } else if *self == 1 {
            self.clone_from(other);
        } else if *self == 0 || *other == 1 {
        } else if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), &Large(ref y)) => unsafe {
                    gmp::mpz_mul(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}
