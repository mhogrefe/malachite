use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::ops::{Add, AddAssign};
use std::mem;

/// Adds an `i32` to an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((Integer::zero() + -123i32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + 0i32).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + -456i32).to_string(), "-579");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() + -123i32).to_string(),
///                "-1000000000123");
/// }
/// ```
impl Add<i32> for Integer {
    type Output = Integer;

    fn add(mut self, other: i32) -> Integer {
        self += other;
        self
    }
}

/// Adds an `i32` to an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::zero() + -123i32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + 0i32).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + -456i32).to_string(), "-579");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() + -123i32).to_string(),
///                "-1000000000123");
/// }
/// ```
impl<'a> Add<i32> for &'a Integer {
    type Output = Integer;

    fn add(self, other: i32) -> Integer {
        if other == 0 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                match small.checked_add(other) {
                    Some(sum) => Small(sum),
                    None => unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_si(&mut result, small.into());
                        if other > 0 {
                            gmp::mpz_add_ui(&mut result, &result, other as u64);
                        } else {
                            gmp::mpz_sub_ui(
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
                    gmp::mpz_add_ui(&mut result, &result, other as u64);
                } else {
                    gmp::mpz_sub_ui(&mut result, &result, (other.wrapping_abs() as u32).into());
                }
                let mut result = Large(result);
                result.demote_if_small();
                result
            },
        }
    }
}

/// Adds an `Integer` to an `i32`, taking the `Integer` by value.
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
///     assert_eq!((-123i32 + Integer::zero()).to_string(), "-123");
///     assert_eq!((0i32 + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((-456i32 + Integer::from(-123)).to_string(), "-579");
///     assert_eq!((-123i32 + Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-1000000000123");
/// }
/// ```
impl Add<Integer> for i32 {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds an `Integer` to an `i32`, taking the `Integer` by reference.
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
///     assert_eq!((-123i32 + &Integer::zero()).to_string(), "-123");
///     assert_eq!((0i32 + &Integer::from(-123)).to_string(), "-123");
///     assert_eq!((-456i32 + &Integer::from(-123)).to_string(), "-579");
///     assert_eq!((-123i32 + &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-1000000000123");
/// }
/// ```
impl<'a> Add<&'a Integer> for i32 {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}

/// Adds an `i32` to an `Integer` in place.
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
///     let mut x = Integer::zero();
///     x += 1;
///     x += -2;
///     x += 3;
///     x += -4;
///     assert_eq!(x.to_string(), "-2");
/// }
/// ```
impl AddAssign<i32> for Integer {
    fn add_assign(&mut self, other: i32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                small.checked_add(other)
            },
            {
                if other > 0 {
                    unsafe { gmp::mpz_add_ui(large, large, other as u64) }
                } else {
                    unsafe { gmp::mpz_sub_ui(large, large, (other.wrapping_abs() as u32).into()) }
                }
            }
        );
        self.demote_if_small();
    }
}
