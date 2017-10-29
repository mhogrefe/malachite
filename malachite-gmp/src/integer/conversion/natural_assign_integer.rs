use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;
use std::mem;
use malachite_base::traits::Assign;

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by value.
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(Integer::from(123));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl Assign<Integer> for Natural {
    fn assign(&mut self, mut other: Integer) {
        assert_ne!(
            other.sign(),
            Ordering::Less,
            "Cannot assign from a negative Integer. Invalid other: {}",
            other
        );
        match other {
            Integer::Small(y) => *self = Natural::Small(y as u32),
            Integer::Large(ref y) if unsafe { gmp::mpz_sizeinbase(y, 2) <= 32 } => {
                *self = Natural::Small(unsafe { gmp::mpz_get_ui(y) } as u32)
            }
            Integer::Large(ref mut y) => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut x);
                mem::swap(&mut x, y);
                *self = Natural::Large(x);
            },
        }
    }
}

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by reference.
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(&Integer::from(123));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl<'a> Assign<&'a Integer> for Natural {
    fn assign(&mut self, other: &'a Integer) {
        assert_ne!(
            other.sign(),
            Ordering::Less,
            "Cannot assign from a negative Integer. Invalid other: {}",
            other
        );
        match *other {
            Integer::Small(y) => *self = Natural::Small(y as u32),
            Integer::Large(ref y) if unsafe { gmp::mpz_sizeinbase(y, 2) <= 32 } => {
                *self = Natural::Small(unsafe { gmp::mpz_get_ui(y) } as u32)
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
