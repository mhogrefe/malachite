use std::cmp::Ordering;

use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};

use integer::Integer;

/// Compares the absolute value of an `Integer` to the absolute value of another `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits()`, `other.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-123).lt_abs(&Integer::from(-124)));
/// assert!(Integer::from(-123).le_abs(&Integer::from(-124)));
/// assert!(Integer::from(-124).gt_abs(&Integer::from(-123)));
/// assert!(Integer::from(-124).ge_abs(&Integer::from(-123)));
/// ```
impl PartialOrdAbs for Integer {
    #[inline]
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
