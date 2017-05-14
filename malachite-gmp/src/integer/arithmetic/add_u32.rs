use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Add, AddAssign};
use std::mem;

/// Adds a `u32` to an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_gmp::integer::Integer;
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
/// use malachite_gmp::integer::Integer;
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
        mutate_with_possible_promotion!(self,
                                        small,
                                        large,
                                        {
                                            let sum = *small as i64 + other as i64;
                                            if sum >= i32::min_value() as i64 &&
                                               sum <= i32::max_value() as i64 {
                                                Some(sum as i32)
                                            } else {
                                                None
                                            }
                                        },
                                        {
                                            unsafe { gmp::mpz_add_ui(large, large, other.into()) }
                                        });
    }
}
