use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;
use malachite_base::traits::{OrdAbs, PartialOrdAbs};

/// Compares the absolute value of an `Integer` to the absolute value of another `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert!(Integer::from(-123).lt_abs(&Integer::from(-124)));
///     assert!(Integer::from(-123).le_abs(&Integer::from(-124)));
///     assert!(Integer::from(-124).gt_abs(&Integer::from(-123)));
///     assert!(Integer::from(-124).ge_abs(&Integer::from(-123)));
/// }
/// ```
impl PartialOrdAbs for Integer {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

/// Asserts that `Integer` absolute value ordering is a total order.
impl OrdAbs for Integer {
    fn cmp_abs(&self, other: &Integer) -> Ordering {
        match (self, other) {
            (&Small(ref x), y) => x.partial_cmp_abs(y).unwrap(),
            (&Large(_), &Small(ref y)) => self.partial_cmp_abs(y).unwrap(),
            (&Large(ref x), &Large(ref y)) => (unsafe { gmp::mpz_cmpabs(x, y) }).cmp(&0),
        }
    }
}
