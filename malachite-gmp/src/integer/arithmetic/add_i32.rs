use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Add, AddAssign};
use std::mem;

/// Adds an `i32` to an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(0) + -123i32).to_string(), "-123");
/// assert_eq!((Integer::from(-123) + 0i32).to_string(), "-123");
/// assert_eq!((Integer::from(-123) + -456i32).to_string(), "-579");
/// assert_eq!((Integer::from_str("-1000000000000").unwrap() + -123i32).to_string(),
///            "-1000000000123");
/// ```
impl Add<i32> for Integer {
    type Output = Integer;

    fn add(mut self, other: i32) -> Integer {
        self += other;
        self
    }
}

/// Adds an `Integer` to an `i32`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((-123i32 + Integer::from(0)).to_string(), "-123");
/// assert_eq!((0i32 + Integer::from(-123)).to_string(), "-123");
/// assert_eq!((-456i32 + Integer::from(-123)).to_string(), "-579");
/// assert_eq!((-123i32 + Integer::from_str("-1000000000000").unwrap()).to_string(),
///            "-1000000000123");
/// ```
impl Add<Integer> for i32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds an `i32` to an `Integer` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// let mut x = Integer::new();
/// x += 1;
/// x += -2;
/// x += 3;
/// x += -4;
/// assert_eq!(x.to_string(), "-2");
/// ```
impl AddAssign<i32> for Integer {
    fn add_assign(&mut self, other: i32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(self,
                                        small,
                                        large,
                                        {
                                            small.checked_add(other)
                                        },
                                        {
                                            if other > 0 {
                                                unsafe {
                                                    gmp::mpz_add_ui(large,
                                                                    large,
                                                                    other as gmp::limb_t)
                                                }
                                            } else {
                                                unsafe {
                                                    gmp::mpz_sub_ui(large,
                                                                    large,
                                                                    other.wrapping_neg() as
                                                                    gmp::limb_t)
                                                }
                                            }
                                        });
        self.demote_if_small();
    }
}
