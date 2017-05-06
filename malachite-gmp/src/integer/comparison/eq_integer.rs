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
        match (self, i) {
            (&Small(x), &Small(y)) => x == y,
            (&Large(x), &Large(y)) => (unsafe { gmp::mpz_cmp(&x, &y) }) == 0,
            _ => false,
        }
    }
}

/// Asserts that `Integer` equality is an equivalence relation.
impl Eq for Integer {}
