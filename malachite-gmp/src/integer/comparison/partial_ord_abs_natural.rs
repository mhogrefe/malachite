use gmp_mpfr_sys::gmp;
use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;
use malachite_base::traits::PartialOrdAbs;

/// Compares the absolute value of an `Integer` to the absolute value of a `Natural`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert!(Integer::from(123).gt_abs(&Natural::from(122u32)));
///     assert!(Integer::from(123).ge_abs(&Natural::from(122u32)));
///     assert!(Integer::from(123).lt_abs(&Natural::from(124u32)));
///     assert!(Integer::from(123).le_abs(&Natural::from(124u32)));
///     assert!(Integer::from(-124).gt_abs(&Natural::from(123u32)));
///     assert!(Integer::from(-124).ge_abs(&Natural::from(123u32)));
/// }
/// ```
impl PartialOrdAbs<Natural> for Integer {
    fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
        match (self, other) {
            (&Integer::Small(x), &Natural::Small(y)) => (x.wrapping_abs() as u32).partial_cmp(&y),
            (&Integer::Small(x), &Natural::Large(ref y)) => {
                0.partial_cmp(&unsafe { gmp::mpz_cmpabs_ui(y, x.wrapping_abs() as u64) })
            }
            (&Integer::Large(ref x), &Natural::Small(y)) => {
                (unsafe { gmp::mpz_cmpabs_ui(x, y.into()) }).partial_cmp(&0)
            }
            (&Integer::Large(ref x), &Natural::Large(ref y)) => {
                (unsafe { gmp::mpz_cmpabs(x, y) }).partial_cmp(&0)
            }
        }
    }
}

/// Compares the absolute value of a `Natural` to the absolute value of an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert!(Natural::from(123u32).gt_abs(&Integer::from(122)));
///     assert!(Natural::from(123u32).ge_abs(&Integer::from(122)));
///     assert!(Natural::from(123u32).lt_abs(&Integer::from(124)));
///     assert!(Natural::from(123u32).le_abs(&Integer::from(124)));
///     assert!(Natural::from(123u32).lt_abs(&Integer::from(-124)));
///     assert!(Natural::from(123u32).le_abs(&Integer::from(-124)));
/// }
/// ```
impl PartialOrdAbs<Integer> for Natural {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        match (self, other) {
            (&Natural::Small(x), &Integer::Small(y)) => x.partial_cmp(&(y.wrapping_abs() as u32)),
            (&Natural::Small(x), &Integer::Large(ref y)) => {
                0.partial_cmp(&unsafe { gmp::mpz_cmpabs_ui(y, x.into()) })
            }
            (&Natural::Large(ref x), &Integer::Small(y)) => {
                (unsafe { gmp::mpz_cmpabs_ui(x, y.wrapping_abs() as u64) }).partial_cmp(&0)
            }
            (&Natural::Large(ref x), &Integer::Large(ref y)) => {
                (unsafe { gmp::mpz_cmpabs(x, y) }).partial_cmp(&0)
            }
        }
    }
}
