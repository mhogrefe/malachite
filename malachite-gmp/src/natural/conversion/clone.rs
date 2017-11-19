use natural::Natural::{self, Large, Small};
use gmp_mpfr_sys::gmp::{self, mpz_t};
use malachite_base::traits::Zero;
use std::mem;

/// Clones a `Natural`, producing a new `Natural`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.clone_from(&Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
///
///     let mut y = x.clone();
///     assert_eq!(y.to_string(), "123");
///     y.assign(789u32);
///     assert_eq!(x.to_string(), "123");
///     assert_eq!(y.to_string(), "789");
/// }
/// ```
impl Clone for Natural {
    fn clone(&self) -> Natural {
        let mut cloned = Natural::zero();
        cloned.clone_from(self);
        cloned
    }

    fn clone_from(&mut self, source: &Natural) {
        match *source {
            Small(y) => {
                match *self {
                    Small(ref mut x) => *x = y,
                    Large(_) => *self = Small(y),
                }
            }
            Large(ref y) => {
                match *self {
                    Small(_) => unsafe {
                        let mut assigned: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set(&mut assigned, y);
                        *self = Large(assigned);
                    },
                    Large(ref mut x) => unsafe {
                        gmp::mpz_set(x, y);
                    },
                }
            }
        }
    }
}
