use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;
use traits::PartialOrdAbs;

/// Compares an `Integer` to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::PartialOrdAbs;
/// use std::str::FromStr;
///
/// assert!(Integer::from(123).gt_abs(&122));
/// assert!(Integer::from(123).ge_abs(&122));
/// assert!(Integer::from(123).lt_abs(&124));
/// assert!(Integer::from(123).le_abs(&124));
/// assert!(Integer::from_str("1000000000000").unwrap().gt_abs(&123));
/// assert!(Integer::from_str("1000000000000").unwrap().ge_abs(&123));
/// assert!(Integer::from_str("-1000000000000").unwrap().gt_abs(&123));
/// assert!(Integer::from_str("-1000000000000").unwrap().ge_abs(&123));
/// ```
impl PartialOrdAbs<u32> for Integer {
    fn partial_cmp_abs(&self, other: &u32) -> Option<Ordering> {
        match *self {
            Small(small) if small == i32::min_value() && *other == 0x8000_0000 as u32 => {
                Some(Ordering::Equal)
            }
            Small(_) if *other > i32::max_value() as u32 => Some(Ordering::Less),
            Small(small) => small.abs().partial_cmp(&(*other as i32)),
            Large(ref large) => Some(unsafe { gmp::mpz_cmpabs_ui(large, (*other).into()) }.cmp(&0)),
        }
    }
}

/// Compares a `u32` to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::PartialOrdAbs;
/// use std::str::FromStr;
///
/// assert!(122.lt_abs(&Integer::from(123)));
/// assert!(122.le_abs(&Integer::from(123)));
/// assert!(124.gt_abs(&Integer::from(123)));
/// assert!(123.ge_abs(&Integer::from(123)));
/// assert!(123.lt_abs(&Integer::from_str("1000000000000").unwrap()));
/// assert!(123.le_abs(&Integer::from_str("1000000000000").unwrap()));
/// assert!(123.lt_abs(&Integer::from_str("-1000000000000").unwrap()));
/// assert!(123.le_abs(&Integer::from_str("-1000000000000").unwrap()));
/// ```
impl PartialOrdAbs<Integer> for u32 {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        match *other {
            Small(small) if small == i32::min_value() && *self == 0x8000_0000 as u32 => {
                Some(Ordering::Equal)
            }
            Small(_) if *self > i32::max_value() as u32 => Some(Ordering::Greater),
            Small(small) => (*self as i32).partial_cmp(&small.abs()),
            Large(ref large) => Some(0.cmp(&unsafe { gmp::mpz_cmpabs_ui(large, (*self).into()) })),
        }
    }
}
