use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;
use traits::Assign;

/// Assigns a `Natural` to `self`.
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
            Natural::Small(y) if y & 0x8000_0000 != 0 => unsafe {
                let mut assigned: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut assigned, y.into());
                *self = Integer::Large(assigned);
            },
            Natural::Small(y) => {
                match *self {
                    Integer::Small(ref mut x) => *x = y as i32,
                    Integer::Large(_) => *self = Integer::Small(y as i32),
                }
            }
            Natural::Large(ref y) => {
                match *self {
                    Integer::Small(_) => unsafe {
                        let mut assigned: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set(&mut assigned, y);
                        *self = Integer::Large(assigned);
                    },
                    Integer::Large(ref mut x) => unsafe {
                        gmp::mpz_set(x, y);
                    },
                }
            }
        }
    }
}
