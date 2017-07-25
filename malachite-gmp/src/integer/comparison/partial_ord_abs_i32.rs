use integer::Integer;
use std::cmp::Ordering;
use traits::PartialOrdAbs;

/// Compares the absolute value of an `Integer` to the absolute value of an `i32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::PartialOrdAbs;
/// use std::str::FromStr;
///
/// assert!(Integer::from(-123).gt_abs(&-122));
/// assert!(Integer::from(-123).ge_abs(&-122));
/// assert!(Integer::from(-123).lt_abs(&-124));
/// assert!(Integer::from(-123).le_abs(&-124));
/// assert!(Integer::from_str("1000000000000").unwrap().gt_abs(&123));
/// assert!(Integer::from_str("1000000000000").unwrap().ge_abs(&123));
/// assert!(Integer::from_str("-1000000000000").unwrap().gt_abs(&123));
/// assert!(Integer::from_str("-1000000000000").unwrap().ge_abs(&123));
/// ```
impl PartialOrdAbs<i32> for Integer {
    fn partial_cmp_abs(&self, other: &i32) -> Option<Ordering> {
        self.partial_cmp_abs(&(other.abs() as u32))
    }
}

/// Compares the absolute value of an `i32` to the absolute value of an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::PartialOrdAbs;
/// use std::str::FromStr;
///
/// assert!((-122).lt_abs(&Integer::from(-123)));
/// assert!((-122).le_abs(&Integer::from(-123)));
/// assert!((-124).gt_abs(&Integer::from(-123)));
/// assert!((-123).ge_abs(&Integer::from(-123)));
/// assert!(123.lt_abs(&Integer::from_str("1000000000000").unwrap()));
/// assert!(123.le_abs(&Integer::from_str("1000000000000").unwrap()));
/// assert!(123.lt_abs(&Integer::from_str("-1000000000000").unwrap()));
/// assert!(123.le_abs(&Integer::from_str("-1000000000000").unwrap()));
/// ```
impl PartialOrdAbs<Integer> for i32 {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        (self.abs() as u32).partial_cmp_abs(other)
    }
}
