use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::{Mul, MulAssign};
use malachite_base::traits::{Assign, Zero};

/// Multiplies an `Integer` by an `i32`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO * 123i32).to_string(), "0");
///     assert_eq!((Integer::from(123i32) * 1i32).to_string(), "123");
///     assert_eq!((Integer::from(123i32) * -456i32).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() * 123i32).to_string(),
///                "-123000000000000");
/// }
/// ```
impl Mul<i32> for Integer {
    type Output = Integer;

    fn mul(mut self, other: i32) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by an `i32`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO * 123i32).to_string(), "0");
///     assert_eq!((&Integer::from(123i32) * 1i32).to_string(), "123");
///     assert_eq!((&Integer::from(123i32) * -456i32).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() * 123i32).to_string(),
///                "-123000000000000");
/// }
/// ```
impl<'a> Mul<i32> for &'a Integer {
    type Output = Integer;

    fn mul(self, other: i32) -> Integer {
        if *self == 0 || other == 0 {
            return Integer::ZERO;
        } else if other == 1 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                match small.checked_mul(other) {
                    Some(product) => Small(product),
                    None => unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_si(&mut result, small.into());
                        gmp::mpz_mul_si(&mut result, &result, other.into());
                        Large(result)
                    },
                }
            }
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut result, large);
                gmp::mpz_mul_si(&mut result, large, other.into());
                Large(result)
            },
        }
    }
}

/// Multiplies an `i32` by an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((123i32 * Integer::ZERO).to_string(), "0");
///     assert_eq!((1i32 * Integer::from(123i32)).to_string(), "123");
///     assert_eq!((-456i32 * Integer::from(123i32)).to_string(), "-56088");
///     assert_eq!((123i32 * Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-123000000000000");
/// }
/// ```
impl Mul<Integer> for i32 {
    type Output = Integer;

    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

/// Multiplies an `i32` by an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((123i32 * &Integer::ZERO).to_string(), "0");
///     assert_eq!((1i32 * &Integer::from(123i32)).to_string(), "123");
///     assert_eq!((-456i32 * &Integer::from(123i32)).to_string(), "-56088");
///     assert_eq!((123i32 * &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-123000000000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for i32 {
    type Output = Integer;

    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}

/// Multiplies an `Integer` by an `i32` in place.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::NegativeOne;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x *= -1i32;
///     x *= -2i32;
///     x *= -3i32;
///     x *= -4i32;
///     assert_eq!(x.to_string(), "-24");
/// }
/// ```
impl MulAssign<i32> for Integer {
    fn mul_assign(&mut self, other: i32) {
        if *self == 0 || other == 0 {
            self.assign(0i32);
            return;
        } else if other == 1 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                small.checked_mul(other)
            },
            {
                unsafe { gmp::mpz_mul_si(large, large, other.into()) }
            }
        );
        self.demote_if_small();
    }
}
