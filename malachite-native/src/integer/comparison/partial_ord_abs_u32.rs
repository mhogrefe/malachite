use integer::Integer;
use malachite_base::traits::PartialOrdAbs;
use std::cmp::Ordering;

/// Compares the absolute value of an `Integer` to a `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_native::integer::Integer;
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
        self.abs.partial_cmp(other)
    }
}

/// Compares a `u32` to the absolute value of an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::PartialOrdAbs;
/// use malachite_native::integer::Integer;
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
        self.partial_cmp(&other.abs)
    }
}
