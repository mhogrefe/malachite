use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Sub, SubAssign};
use std::mem;

/// Subtracts an `i32` from an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO - 123i32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) - -0i32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) - 456i32).to_string(), "-579");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() - 123i32).to_string(),
///                "-1000000000123");
/// }
/// ```
impl Sub<i32> for Integer {
    type Output = Integer;

    fn sub(mut self, other: i32) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts an `i32` from an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO - 123i32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) - -0i32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) - 456i32).to_string(), "-579");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() - 123i32).to_string(),
///                "-1000000000123");
/// }
/// ```
impl<'a> Sub<i32> for &'a Integer {
    type Output = Integer;

    fn sub(self, other: i32) -> Integer {
        if other == 0 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                match small.checked_sub(other) {
                    Some(sum) => Small(sum),
                    None => unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_si(&mut result, small.into());
                        if other > 0 {
                            gmp::mpz_sub_ui(&mut result, &result, other as u64);
                        } else {
                            gmp::mpz_add_ui(
                                &mut result,
                                &result,
                                (other.wrapping_abs() as u32).into(),
                            );
                        }
                        let mut result = Large(result);
                        result.demote_if_small();
                        result
                    },
                }
            }
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut result, large);
                if other > 0 {
                    gmp::mpz_sub_ui(&mut result, &result, other as u64);
                } else {
                    gmp::mpz_add_ui(&mut result, &result, (other.wrapping_abs() as u32).into());
                }
                let mut result = Large(result);
                result.demote_if_small();
                result
            },
        }
    }
}

/// Subtracts an `Integer` from an `i32`, taking the `Integer` by value.
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
///     assert_eq!((-123i32 - Integer::ZERO).to_string(), "-123");
///     assert_eq!((0i32 - Integer::from(123)).to_string(), "-123");
///     assert_eq!((-456i32 - Integer::from(123)).to_string(), "-579");
///     assert_eq!((-123i32 - Integer::from_str("1000000000000").unwrap()).to_string(),
///                "-1000000000123");
/// }
/// ```
impl Sub<Integer> for i32 {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

/// Subtracts an `Integer` from an `i32`, taking the `Integer` by reference.
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
///     assert_eq!((-123i32 - &Integer::ZERO).to_string(), "-123");
///     assert_eq!((0i32 - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((-456i32 - &Integer::from(123)).to_string(), "-579");
///     assert_eq!((-123i32 - &Integer::from_str("1000000000000").unwrap()).to_string(),
///                "-1000000000123");
/// }
/// ```
impl<'a> Sub<&'a Integer> for i32 {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        -(other - self)
    }
}

/// Subtracts an `i32` from an `Integer` in place.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x -= -1;
///     x -= 2;
///     x -= -3;
///     x -= 4;
///     assert_eq!(x.to_string(), "-2");
/// }
/// ```
impl SubAssign<i32> for Integer {
    fn sub_assign(&mut self, other: i32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                small.checked_sub(other)
            },
            {
                if other > 0 {
                    unsafe { gmp::mpz_sub_ui(large, large, other as gmp::limb_t) }
                } else {
                    unsafe { gmp::mpz_add_ui(large, large, other.wrapping_neg() as gmp::limb_t) }
                }
            }
        );
        self.demote_if_small();
    }
}
