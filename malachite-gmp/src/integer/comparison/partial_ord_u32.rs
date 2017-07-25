use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

/// Compares an `Integer` to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert!(Integer::from(123) > 122);
/// assert!(Integer::from(123) >= 122);
/// assert!(Integer::from(123) < 124);
/// assert!(Integer::from(123) <= 124);
/// assert!(Integer::from_str("1000000000000").unwrap() > 123);
/// assert!(Integer::from_str("1000000000000").unwrap() >= 123);
/// assert!(Integer::from_str("-1000000000000").unwrap() < 123);
/// assert!(Integer::from_str("-1000000000000").unwrap() <= 123);
/// ```
impl PartialOrd<u32> for Integer {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match *self {
            Small(_) if *other > i32::max_value() as u32 => Some(Ordering::Less),
            Small(small) => small.partial_cmp(&(*other as i32)),
            Large(ref large) => Some(unsafe { gmp::mpz_cmp_ui(large, (*other).into()) }.cmp(&0)),
        }
    }
}

/// Compares a `u32` to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert!(122 < Integer::from(123));
/// assert!(122 <= Integer::from(123));
/// assert!(124 > Integer::from(123));
/// assert!(123 >= Integer::from(123));
/// assert!(123 < Integer::from_str("1000000000000").unwrap());
/// assert!(123 <= Integer::from_str("1000000000000").unwrap());
/// assert!(123 > Integer::from_str("-1000000000000").unwrap());
/// assert!(123 >= Integer::from_str("-1000000000000").unwrap());
/// ```
impl PartialOrd<Integer> for u32 {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        match *other {
            Small(_) if *self > i32::max_value() as u32 => Some(Ordering::Greater),
            Small(ref small) => (*self as i32).partial_cmp(small),
            Large(ref large) => Some(0.cmp(&unsafe { gmp::mpz_cmp_ui(large, (*self).into()) })),
        }
    }
}
