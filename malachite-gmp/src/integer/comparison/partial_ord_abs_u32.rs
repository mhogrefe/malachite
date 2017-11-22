use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;
use std::i32;
use malachite_base::traits::PartialOrdAbs;

/// Compares the absolute value of an `Integer` to a `u32`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert!(Integer::from(123).gt_abs(&122));
///     assert!(Integer::from(123).ge_abs(&122));
///     assert!(Integer::from(123).lt_abs(&124));
///     assert!(Integer::from(123).le_abs(&124));
///     assert!(Integer::from_str("1000000000000").unwrap().gt_abs(&123));
///     assert!(Integer::from_str("1000000000000").unwrap().ge_abs(&123));
///     assert!(Integer::from_str("-1000000000000").unwrap().gt_abs(&123));
///     assert!(Integer::from_str("-1000000000000").unwrap().ge_abs(&123));
/// }
/// ```
impl PartialOrdAbs<u32> for Integer {
    fn partial_cmp_abs(&self, other: &u32) -> Option<Ordering> {
        match *self {
            Small(i32::MIN) if *other == 0x8000_0000 as u32 => Some(Ordering::Equal),
            Small(_) if *other > i32::max_value() as u32 => Some(Ordering::Less),
            Small(small) => small.abs().partial_cmp(&(*other as i32)),
            Large(ref large) => Some(unsafe { gmp::mpz_cmpabs_ui(large, (*other).into()) }.cmp(
                &0,
            )),
        }
    }
}

/// Compares a `u32` to the absolute value of an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert!(122.lt_abs(&Integer::from(123)));
///     assert!(122.le_abs(&Integer::from(123)));
///     assert!(124.gt_abs(&Integer::from(123)));
///     assert!(123.ge_abs(&Integer::from(123)));
///     assert!(123.lt_abs(&Integer::from_str("1000000000000").unwrap()));
///     assert!(123.le_abs(&Integer::from_str("1000000000000").unwrap()));
///     assert!(123.lt_abs(&Integer::from_str("-1000000000000").unwrap()));
///     assert!(123.le_abs(&Integer::from_str("-1000000000000").unwrap()));
/// }
/// ```
impl PartialOrdAbs<Integer> for u32 {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        match *other {
            Small(i32::MIN) if *self == 0x8000_0000 as u32 => Some(Ordering::Equal),
            Small(_) if *self > i32::max_value() as u32 => Some(Ordering::Greater),
            Small(small) => (*self as i32).partial_cmp(&small.abs()),
            Large(ref large) => Some(0.cmp(&unsafe { gmp::mpz_cmpabs_ui(large, (*self).into()) })),
        }
    }
}
