use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::{Mul, MulAssign};
use malachite_base::traits::{Assign, Zero};

/// Multiplies an `Integer` by a `u32`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO * 123u32).to_string(), "0");
///     assert_eq!((Integer::from(123i32) * 1u32).to_string(), "123");
///     assert_eq!((Integer::from(-123i32) * 456u32).to_string(), "-56088");
///     assert_eq!((-Integer::trillion() * 123u32).to_string(), "-123000000000000");
/// }
/// ```
impl Mul<u32> for Integer {
    type Output = Integer;

    fn mul(mut self, other: u32) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by a `u32`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO * 123u32).to_string(), "0");
///     assert_eq!((&Integer::from(123i32) * 1u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123i32) * 456u32).to_string(), "-56088");
///     assert_eq!((&(-Integer::trillion()) * 123u32).to_string(), "-123000000000000");
/// }
/// ```
impl<'a> Mul<u32> for &'a Integer {
    type Output = Integer;

    fn mul(self, other: u32) -> Integer {
        if *self == 0 || other == 0 {
            return Integer::ZERO;
        } else if other == 1 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                let product = small as i64 * other as i64;
                if product >= i32::min_value() as i64 && product <= i32::max_value() as i64 {
                    Small(product as i32)
                } else {
                    unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_si(&mut result, small.into());
                        gmp::mpz_mul_ui(&mut result, &result, other.into());
                        Large(result)
                    }
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

/// Multiplies a `u32` by an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((123u32 * Integer::ZERO).to_string(), "0");
///     assert_eq!((1u32 * Integer::from(123i32)).to_string(), "123");
///     assert_eq!((456u32 * Integer::from(-123i32)).to_string(), "-56088");
///     assert_eq!((123u32 * -Integer::trillion()).to_string(), "-123000000000000");
/// }
/// ```
impl Mul<Integer> for u32 {
    type Output = Integer;

    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

/// Multiplies a `u32` by an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((123u32 * &Integer::ZERO).to_string(), "0");
///     assert_eq!((1u32 * &Integer::from(123i32)).to_string(), "123");
///     assert_eq!((456u32 * &Integer::from(-123i32)).to_string(), "-56088");
///     assert_eq!((123u32 * &(-Integer::trillion())).to_string(), "-123000000000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for u32 {
    type Output = Integer;

    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}

/// Multiplies an `Integer` by a `u32` in place.
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
///     x *= 1u32;
///     x *= 2u32;
///     x *= 3u32;
///     x *= 4u32;
///     assert_eq!(x.to_string(), "-24");
/// }
/// ```
impl MulAssign<u32> for Integer {
    fn mul_assign(&mut self, other: u32) {
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
                let product = *small as i64 * other as i64;
                if product >= i32::min_value() as i64 && product <= i32::max_value() as i64 {
                    Some(product as i32)
                } else {
                    None
                }
            },
            { unsafe { gmp::mpz_mul_ui(large, large, other.into()) } }
        );
    }
}
