use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;
use std::ops::Neg;

/// Returns the negative of a `Natural`, taking the `Natural` by value and returning an `Integer`.
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
///     assert_eq!((-Natural::zero()).to_string(), "0");
///     assert_eq!((-Natural::from(123u32)).to_string(), "-123");
/// }
/// ```
impl Neg for Natural {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        match self {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                Integer::Small(-(small as i32))
            }
            Natural::Small(0x8000_0000) => Integer::Small(i32::min_value()),
            Natural::Small(small) => unsafe {
                let mut negative: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut negative, small.into());
                gmp::mpz_neg(&mut negative, &negative);
                Integer::Large(negative)
            },
            Natural::Large(ref mut large) => unsafe {
                let mut negative: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut negative);
                mem::swap(&mut negative, large);
                gmp::mpz_neg(&mut negative, &negative);
                Integer::Large(negative)
            },
        }
    }
}

/// Returns the negative of a `Natural`, taking the `Natural` by reference and returning an
/// `Integer`.
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
///     assert_eq!((-&Natural::zero()).to_string(), "0");
///     assert_eq!((-&Natural::from(123u32)).to_string(), "-123");
/// }
/// ```
impl<'a> Neg for &'a Natural {
    type Output = Integer;

    fn neg(self) -> Integer {
        match *self {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                Integer::Small(-(small as i32))
            }
            Natural::Small(0x8000_0000) => Integer::Small(i32::min_value()),
            Natural::Small(small) => unsafe {
                let mut negative: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut negative, small.into());
                gmp::mpz_neg(&mut negative, &negative);
                Integer::Large(negative)
            },
            Natural::Large(ref large) => unsafe {
                let mut negative: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut negative);
                gmp::mpz_neg(&mut negative, large);
                Integer::Large(negative)
            },
        }
    }
}
