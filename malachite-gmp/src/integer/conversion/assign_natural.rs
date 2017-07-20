use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;
use traits::Assign;

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by value.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(-456);
/// x.assign(Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl Assign<Natural> for Integer {
    fn assign(&mut self, mut other: Natural) {
        match other {
            Natural::Small(y) if y <= i32::max_value() as u32 => *self = Integer::Small(y as i32),
            Natural::Small(y) => unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, y.into());
                *self = Integer::Large(large)
            },
            Natural::Large(ref mut y) => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut x);
                mem::swap(&mut x, y);
                *self = Integer::Large(x);
            },
        }
    }
}

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by reference.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(-456);
/// x.assign(&Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Natural> for Integer {
    fn assign(&mut self, other: &'a Natural) {
        match *other {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                *self = Integer::Small(small as i32)
            }
            Natural::Small(small) => unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, small.into());
                *self = Integer::Large(large)
            },
            Natural::Large(ref large) => unsafe {
                let mut large_copy: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut large_copy, large);
                *self = Integer::Large(large_copy)
            },
        }
    }
}
