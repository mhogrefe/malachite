use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::Not;
use traits::NotAssign;

/// Returns the bitwise complement of an `Integer`, as if it were represented in two's complement,
/// taking the `Integer` by value.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!((!Integer::from(0)).to_string(), "-1");
/// assert_eq!((!Integer::from(123)).to_string(), "-124");
/// assert_eq!((!Integer::from(-123)).to_string(), "122");
/// ```
impl Not for Integer {
    type Output = Integer;

    fn not(mut self) -> Integer {
        self.not_assign();
        self
    }
}

/// Returns the bitwise complement of an `Integer`, as if it were represented in two's complement,
/// taking the `Integer` by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!((!&Integer::from(0)).to_string(), "-1");
/// assert_eq!((!&Integer::from(123)).to_string(), "-124");
/// assert_eq!((!&Integer::from(-123)).to_string(), "122");
/// ```
impl<'a> Not for &'a Integer {
    type Output = Integer;

    fn not(self) -> Integer {
        match *self {
            Small(small) => Small(!small),
            Large(ref large) => unsafe {
                let mut complement: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut complement);
                gmp::mpz_com(&mut complement, large);
                Integer::Large(complement)
            },
        }
    }
}

/// Replaces an `Integer` with its bitwise complement, as if it were represented in two's
/// complement.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::NotAssign;
///
/// let mut x = Integer::from(0);
/// x.not_assign();
/// assert_eq!(x.to_string(), "-1");
///
/// let mut x = Integer::from(123);
/// x.not_assign();
/// assert_eq!(x.to_string(), "-124");
///
/// let mut x = Integer::from(-123);
/// x.not_assign();
/// assert_eq!(x.to_string(), "122");
/// ```
impl NotAssign for Integer {
    fn not_assign(&mut self) {
        match *self {
            Small(ref mut small) => {
                *small = !*small;
            }
            Large(ref mut large) => unsafe {
                gmp::mpz_com(large, large);
            },
        }
    }
}