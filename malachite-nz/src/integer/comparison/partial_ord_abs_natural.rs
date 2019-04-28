use std::cmp::Ordering;

use malachite_base::num::traits::PartialOrdAbs;

use integer::Integer;
use natural::Natural;

/// Compares the absolute value of an `Integer` to the absolute value of a `Natural`.
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
/// use malachite_base::num::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
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
        self.abs.partial_cmp(other)
    }
}

/// Compares the absolute value of a `Natural` to the absolute value of an `Integer`.
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
/// use malachite_base::num::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
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
        self.partial_cmp(&other.abs)
    }
}
