use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Add, AddAssign};
use std::mem;

/// Adds a `u32` to an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO + 123u32).to_string(), "123");
///     assert_eq!((Integer::from(-123) + 0u32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + 456u32).to_string(), "333");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() + 123u32).to_string(),
///                "-999999999877");
/// }
/// ```
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, other: u32) -> Integer {
        self += other;
        self
    }
}

/// Adds a `u32` to an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO + 123u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + 0u32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + 456u32).to_string(), "333");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() + 123u32).to_string(),
///                "-999999999877");
/// }
/// ```
impl<'a> Add<u32> for &'a Integer {
    type Output = Integer;

    fn add(self, other: u32) -> Integer {
        if other == 0 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                let sum = small as i64 + other as i64;
                if sum >= i32::min_value() as i64 && sum <= i32::max_value() as i64 {
                    Small(sum as i32)
                } else {
                    unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_si(&mut result, small.into());
                        gmp::mpz_add_ui(&mut result, &result, other.into());
                        let mut result = Large(result);
                        result.demote_if_small();
                        result
                    }
                }
            }
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut result, large);
                gmp::mpz_add_ui(&mut result, &result, other.into());
                let mut result = Large(result);
                result.demote_if_small();
                result
            },
        }
    }
}

/// Adds an `Integer` to a `u32`, taking the `Integer` by value.
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
///     assert_eq!((123u32 + Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((456u32 + Integer::from(-123)).to_string(), "333");
///     assert_eq!((123u32 + Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-999999999877");
/// }
/// ```
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds an `Integer` to a `u32`, taking the `Integer` by reference.
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
///     assert_eq!((123u32 + &Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 + &Integer::from(-123)).to_string(), "-123");
///     assert_eq!((456u32 + &Integer::from(-123)).to_string(), "333");
///     assert_eq!((123u32 + &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-999999999877");
/// }
/// ```
impl<'a> Add<&'a Integer> for u32 {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}

/// Adds a `u32` to an `Integer` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
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
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                let sum = *small as i64 + other as i64;
                if sum >= i32::min_value() as i64 && sum <= i32::max_value() as i64 {
                    Some(sum as i32)
                } else {
                    None
                }
            },
            {
                unsafe { gmp::mpz_add_ui(large, large, other.into()) }
            }
        );
        self.demote_if_small();
    }
}
