use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

/// Returns the sign of an `Integer`. Interpret the result as the result of a comparison to zero, so
/// that `Equal` means zero, `Greater` means positive, and `Less` means negative.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
/// use std::cmp::Ordering;
///
/// fn main() {
///     assert_eq!(Integer::zero().sign(), Ordering::Equal);
///     assert_eq!(Integer::from(123).sign(), Ordering::Greater);
///     assert_eq!(Integer::from(-123).sign(), Ordering::Less);
/// }
/// ```
impl Integer {
    pub fn sign(&self) -> Ordering {
        match *self {
            Small(small) => small.cmp(&0),
            Large(ref large) => (unsafe { gmp::mpz_sgn(large) }).cmp(&0),
        }
    }
}
