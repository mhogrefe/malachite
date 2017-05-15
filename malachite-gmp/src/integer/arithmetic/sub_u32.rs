use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Sub, SubAssign};
use std::mem;

/// Subtracts a `u32` from an `Integer`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((Integer::from(123) - 123).to_string(), "0");
/// assert_eq!((Integer::from(-123) - 0).to_string(), "-123");
/// assert_eq!((Integer::from(123) - 456).to_string(), "-333");
/// assert_eq!((Integer::from_str("1000000000000").unwrap() - 123).to_string(), "999999999877");
/// ```
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, other: u32) -> Integer {
        self.sub_assign(other);
        self
    }
}

/// Subtracts an `Integer` from a `u32`, taking ownership of the input `Integer`.
///
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((123 - Integer::from(123)).to_string(), "0");
/// assert_eq!((0 - Integer::from(-123)).to_string(), "123");
/// assert_eq!((456 - Integer::from(123)).to_string(), "333");
/// assert_eq!((123 - Integer::from_str("1000000000000").unwrap()).to_string(), "-999999999877");
/// ```
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other.sub_assign(self);
        -other
    }
}

/// Subtracts a `u32` from an `Integer` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// let mut x = Integer::from(15);
/// x -= 1;
/// x -= 2;
/// x -= 3;
/// x -= 4;
/// assert_eq!(x.to_string(), "5");
/// ```
impl SubAssign<u32> for Integer {
    fn sub_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(self,
                                        small,
                                        large,
                                        {
                                            let difference = *small as i64 - other as i64;
                                            if difference >= i32::min_value() as i64 &&
                                               difference <= i32::max_value() as i64 {
                                                Some(difference as i32)
                                            } else {
                                                None
                                            }
                                        },
                                        {
                                            unsafe { gmp::mpz_sub_ui(large, large, other.into()) }
                                        });
        self.demote_if_small();
    }
}
