use integer::Integer::{self, Large, Small};
use gmp_mpfr_sys::gmp::{self, mpz_t};
use std::mem;
use traits::Assign;

/// Assigns a `&Integer` to `self`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(456);
/// x.assign(&Integer::from(-123));
/// assert_eq!(x.to_string(), "-123");
/// ```
impl<'a> Assign<&'a Integer> for Integer {
    fn assign(&mut self, other: &'a Integer) {
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
