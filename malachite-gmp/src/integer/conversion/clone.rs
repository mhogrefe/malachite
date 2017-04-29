use integer::Integer::{self, Large, Small};
use gmp_mpfr_sys::gmp::{self, mpz_t};
use std::mem;

/// Clones `self`, producing a new `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(456);
/// x.clone_from(&Integer::from(-123));
/// assert_eq!(x.to_string(), "-123");
///
/// let mut y = x.clone();
/// assert_eq!(y.to_string(), "-123");
/// y.assign(789);
/// assert_eq!(x.to_string(), "-123");
/// assert_eq!(y.to_string(), "789");
/// ```
impl Clone for Integer {
    fn clone(&self) -> Integer {
        let mut cloned = Integer::new();
        cloned.clone_from(self);
        cloned
    }

    fn clone_from(&mut self, source: &Integer) {
        match source {
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
