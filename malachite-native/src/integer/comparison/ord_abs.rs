use integer::Integer;
use std::cmp::Ordering;
use traits::{OrdAbs, PartialOrdAbs};

/// Compares the absolute value of an `Integer` to the absolute value of another `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits(), other.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::PartialOrdAbs;
///
/// assert!(Integer::from(-123).lt_abs(&Integer::from(-124)));
/// assert!(Integer::from(-123).le_abs(&Integer::from(-124)));
/// assert!(Integer::from(-124).gt_abs(&Integer::from(-123)));
/// assert!(Integer::from(-124).ge_abs(&Integer::from(-123)));
/// ```
impl PartialOrdAbs for Integer {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

/// Asserts that `Integer` absolute value ordering is a total order.
impl OrdAbs for Integer {
    fn cmp_abs(&self, other: &Integer) -> Ordering {
        self.abs.cmp(&other.abs)
    }
}
