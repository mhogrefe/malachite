use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Mul, MulAssign};
use traits::Assign;

/// Multiplies a `Natural` by a `u32`, taking the `Natural` by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) * 123).to_string(), "0");
/// assert_eq!((Natural::from(123u32) * 1).to_string(), "123");
/// assert_eq!((Natural::from(123u32) * 456).to_string(), "56088");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() * 123).to_string(), "123000000000000");
/// ```
impl Mul<u32> for Natural {
    type Output = Natural;

    fn mul(mut self, other: u32) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `u32`, taking the `Natural` by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) * 123).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) * 1).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) * 456).to_string(), "56088");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() * 123).to_string(), "123000000000000");
/// ```
impl<'a> Mul<u32> for &'a Natural {
    type Output = Natural;

    fn mul(self, other: u32) -> Natural {
        if *self == 0 || other == 0 {
            return Natural::from(0u32);
        } else if other == 1 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                match small.checked_mul(other) {
                    Some(product) => Small(product),
                    None => unsafe {
                        let mut result: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set_ui(&mut result, small.into());
                        gmp::mpz_mul_ui(&mut result, &result, other.into());
                        Large(result)
                    },
                }
            }
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut result, large);
                gmp::mpz_mul_ui(&mut result, large, other.into());
                Large(result)
            },
        }
    }
}

/// Multiplies a `u32` by a `Natural`, taking the `Natural` by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 * Natural::from(0u32)).to_string(), "0");
/// assert_eq!((1 * Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 * Natural::from(123u32)).to_string(), "56088");
/// assert_eq!((123 * Natural::from_str("1000000000000").unwrap()).to_string(), "123000000000000");
/// ```
impl Mul<Natural> for u32 {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `u32` by a `Natural`, taking the `Natural` by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 * &Natural::from(0u32)).to_string(), "0");
/// assert_eq!((1 * &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 * &Natural::from(123u32)).to_string(), "56088");
/// assert_eq!((123 * &Natural::from_str("1000000000000").unwrap()).to_string(), "123000000000000");
/// ```
impl<'a> Mul<&'a Natural> for u32 {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        other * self
    }
}

/// Multiplies a `Natural` by a `u32` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// let mut x = Natural::from(1u32);
/// x *= 1;
/// x *= 2;
/// x *= 3;
/// x *= 4;
/// assert_eq!(x.to_string(), "24");
/// ```
impl MulAssign<u32> for Natural {
    fn mul_assign(&mut self, other: u32) {
        if *self == 0 || other == 0 {
            self.assign(0u32);
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
                unsafe { gmp::mpz_mul_ui(large, large, other.into()) }
            }
        );
    }
}