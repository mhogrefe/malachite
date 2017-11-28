use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;
use std::ops::Not;

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by value and returning an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((!Natural::ZERO).to_string(), "-1");
///     assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// }
/// ```
impl Not for Natural {
    type Output = Integer;

    fn not(mut self) -> Integer {
        match self {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                Integer::Small(!(small as i32))
            }
            Natural::Small(small) => unsafe {
                let mut complement: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut complement, small.into());
                gmp::mpz_com(&mut complement, &complement);
                Integer::Large(complement)
            },
            Natural::Large(ref mut large) => unsafe {
                let mut complement: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut complement);
                mem::swap(&mut complement, large);
                gmp::mpz_com(&mut complement, &complement);
                Integer::Large(complement)
            },
        }
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by reference and returning an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((!&Natural::ZERO).to_string(), "-1");
///     assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// }
/// ```
impl<'a> Not for &'a Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        match *self {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                Integer::Small(!(small as i32))
            }
            Natural::Small(small) => unsafe {
                let mut complement: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut complement, small.into());
                gmp::mpz_com(&mut complement, &complement);
                Integer::Large(complement)
            },
            Natural::Large(ref large) => unsafe {
                let mut complement: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut complement);
                gmp::mpz_com(&mut complement, large);
                Integer::Large(complement)
            },
        }
    }
}
