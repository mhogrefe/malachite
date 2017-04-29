use natural::Natural::{self, Large, Small};
use gmp_mpfr_sys::gmp::{self, mpz_t};
use std::mem;
use traits::Assign;

/// Assigns a `&Natural` to `self`.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Natural::from(456);
/// x.assign(&Natural::from(123));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Natural> for Natural {
    fn assign(&mut self, other: &'a Natural) {
        match other {
            &Small(y) => {
                match self {
                    &mut Small(ref mut x) => *x = y,
                    &mut Large(_) => *self = Small(y),
                }
            }
            &Large(y) => {
                match self {
                    &mut Small(_) => unsafe {
                        let mut assigned: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set(&mut assigned, &y);
                        *self = Large(assigned);
                    },
                    &mut Large(ref mut x) => unsafe {
                        gmp::mpz_set(x, &y);
                    },
                }
            }
        }
    }
}
