use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::{Shl, ShlAssign};

/// Shifts a `Integer` left (multiplies it by a power of 2), taking the `Integer` by value.
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
///     assert_eq!((Integer::zero() << 10).to_string(), "0");
///     assert_eq!((Integer::from(123) << 2).to_string(), "492");
///     assert_eq!((Integer::from(123) << 100).to_string(), "155921023828072216384094494261248");
///     assert_eq!((Integer::from(-123) << 2).to_string(), "-492");
///     assert_eq!((Integer::from(-123) << 100).to_string(), "-155921023828072216384094494261248");
/// }
/// ```
impl Shl<u32> for Integer {
    type Output = Integer;

    fn shl(mut self, other: u32) -> Integer {
        self <<= other;
        self
    }
}

/// Shifts a `Integer` left (multiplies it by a power of 2), taking the `Integer` by reference.
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
///     assert_eq!((&Integer::zero() << 10).to_string(), "0");
///     assert_eq!((&Integer::from(123) << 2).to_string(), "492");
///     assert_eq!((&Integer::from(123) << 100).to_string(), "155921023828072216384094494261248");
///     assert_eq!((&Integer::from(-123) << 2).to_string(), "-492");
///     assert_eq!((&Integer::from(-123) << 100).to_string(), "-155921023828072216384094494261248");
/// }
/// ```
impl<'a> Shl<u32> for &'a Integer {
    type Output = Integer;

    fn shl(self, other: u32) -> Integer {
        if other == 0 || self == &0 {
            return self.clone();
        }
        match *self {
            Small(small) if other < (if small >= 0 { small } else { !small }).leading_zeros() => {
                Small(small << other)
            }
            Small(small) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_si(&mut result, small.into());
                gmp::mpz_mul_2exp(&mut result, &result, other.into());
                Large(result)
            },
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut result);
                gmp::mpz_mul_2exp(&mut result, large, other.into());
                Large(result)
            },
        }
    }
}

/// Shifts a `Integer` left (multiplies it by a power of 2) in place.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::{NegativeOne, One};
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::one();
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "1024");
///     let mut x = Integer::negative_one();
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "-1024");
/// }
/// ```
impl ShlAssign<u32> for Integer {
    fn shl_assign(&mut self, other: u32) {
        if other == 0 || *self == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                if other < (if *small >= 0 { *small } else { !*small }).leading_zeros() {
                    Some(*small << other)
                } else {
                    None
                }
            },
            {
                unsafe { gmp::mpz_mul_2exp(large, large, other.into()) }
            }
        );
    }
}
