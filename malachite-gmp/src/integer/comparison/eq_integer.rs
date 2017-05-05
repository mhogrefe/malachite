use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

/// Determines whether `self` is equal to another `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(Integer::from(-123) == Integer::from(-123));
/// assert!(Integer::from(-123) != Integer::from(5));
/// ```
impl PartialEq<Integer> for Integer {
    fn eq(&self, i: &Integer) -> bool {
        match *self {
            Small(x) => {
                match *i {
                    Small(y) => x == y,
                    Large(_) => false,
                }
            }
            Large(x) => {
                match *i {
                    Small(_) => false,
                    Large(y) => (unsafe { gmp::mpz_cmp(&x, &y) }) == 0,
                }
            }
        }
    }
}

/// Asserts that `Integer` equality is an equivalence relation.
impl Eq for Integer {}
