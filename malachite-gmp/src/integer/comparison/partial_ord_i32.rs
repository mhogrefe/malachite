use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

/// Compares `self` to an `i32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert!(Integer::from(-123) < -122);
/// assert!(Integer::from(-123) <= -122);
/// assert!(Integer::from(-123) > -124);
/// assert!(Integer::from(-123) >= -124);
/// assert!(Integer::from_str("1000000000000").unwrap() > 123);
/// assert!(Integer::from_str("1000000000000").unwrap() >= 123);
/// assert!(Integer::from_str("-1000000000000").unwrap() < 123);
/// assert!(Integer::from_str("-1000000000000").unwrap() <= 123);
/// ```
impl PartialOrd<i32> for Integer {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        match *self {
            Small(ref small) => small.partial_cmp(other),
            Large(_) => Some(self.sign()),
        }
    }
}

/// Compares an `i32` to `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// assert!(-122 > Integer::from(-123));
/// assert!(-122 >= Integer::from(-123));
/// assert!(-124 < Integer::from(-123));
/// assert!(-123 <= Integer::from(-123));
/// assert!(123 < Integer::from_str("1000000000000").unwrap());
/// assert!(123 <= Integer::from_str("1000000000000").unwrap());
/// assert!(123 > Integer::from_str("-1000000000000").unwrap());
/// assert!(123 >= Integer::from_str("-1000000000000").unwrap());
/// ```
impl PartialOrd<Integer> for i32 {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        other.partial_cmp(self).map(|o| o.reverse())
    }
}
