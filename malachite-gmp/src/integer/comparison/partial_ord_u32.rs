use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

/// Compares `self` to a `u32`.
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
            Small(_) if other & 0x8000_0000 != 0 => Some(Ordering::Less),
            Small(ref x) => x.partial_cmp(&(*other as i32)),
            Large(ref x) => Some(unsafe { gmp::mpz_cmp_ui(x, (*other).into()) }.cmp(&0)),
        }
    }
}

/// Compares a `u32` to `self`.
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
        other.partial_cmp(self).map(|o| o.reverse())
    }
}
