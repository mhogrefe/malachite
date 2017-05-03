use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

/// Determines whether `self` is equal to another `Natural`.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123) == Natural::from(123));
/// assert!(Natural::from(123) != Natural::from(5));
/// ```
impl PartialEq<Natural> for Natural {
    fn eq(&self, i: &Natural) -> bool {
        match self {
            &Small(x) => {
                match i {
                    &Small(y) => x == y,
                    &Large(_) => false,
                }
            }
            &Large(x) => {
                match i {
                    &Small(_) => false,
                    &Large(y) => (unsafe { gmp::mpz_cmp(&x, &y) }) == 0,
                }
            }
        }
    }
}

/// Asserts that `Natural` equality is an equivalence relation.
impl Eq for Natural {}
