use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;
use std::mem;
use traits::Assign;

/// Assigns an `Integer` to `self`.
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Natural::from(456u32);
/// x.assign(&Integer::from(123));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Integer> for Natural {
    fn assign(&mut self, other: &'a Integer) {
        assert_ne!(other.sign(),
                   Ordering::Less,
                   "Cannot assign from a negative Integer. Invalid other: {}",
                   other);
        match *other {
            Integer::Small(y) => {
                match *self {
                    Natural::Small(ref mut x) => *x = y as u32,
                    Natural::Large(_) => *self = Natural::Small(y as u32),
                }
            }
            Integer::Large(ref y) if unsafe { gmp::mpz_sizeinbase(y, 2) <= 32 } => {
                *self = Natural::Small(unsafe { gmp::mpz_get_ui(y) } as u32);
            }
            Integer::Large(ref y) => {
                match *self {
                    Natural::Small(_) => unsafe {
                        let mut assigned: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set(&mut assigned, y);
                        *self = Natural::Large(assigned);
                    },
                    Natural::Large(ref mut x) => unsafe {
                        gmp::mpz_set(x, y);
                    },
                }
            }
        }
    }
}
