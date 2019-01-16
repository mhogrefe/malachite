use integer::Integer;
use malachite_base::num::{PartialOrdAbs, UnsignedAbs};
use platform::SignedLimb;
use std::cmp::Ordering;

/// Compares the absolute value of an `Integer` to the absolute value of a `SignedLimb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert!(Integer::from(-123).gt_abs(&-122));
///     assert!(Integer::from(-123).ge_abs(&-122));
///     assert!(Integer::from(-123).lt_abs(&-124));
///     assert!(Integer::from(-123).le_abs(&-124));
///     assert!(Integer::trillion().gt_abs(&123));
///     assert!(Integer::trillion().ge_abs(&123));
///     assert!((-Integer::trillion()).gt_abs(&123));
///     assert!((-Integer::trillion()).ge_abs(&123));
/// }
/// ```
impl PartialOrdAbs<SignedLimb> for Integer {
    fn partial_cmp_abs(&self, other: &SignedLimb) -> Option<Ordering> {
        self.abs.partial_cmp(&other.unsigned_abs())
    }
}

/// Compares the absolute value of a `SignedLimb` to the absolute value of an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert!((-122).lt_abs(&Integer::from(-123)));
///     assert!((-122).le_abs(&Integer::from(-123)));
///     assert!((-124).gt_abs(&Integer::from(-123)));
///     assert!((-123).ge_abs(&Integer::from(-123)));
///     assert!(123.lt_abs(&Integer::trillion()));
///     assert!(123.le_abs(&Integer::trillion()));
///     assert!(123.lt_abs(&(-Integer::trillion())));
///     assert!(123.le_abs(&(-Integer::trillion())));
/// }
/// ```
impl PartialOrdAbs<Integer> for SignedLimb {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        self.unsigned_abs().partial_cmp(&other.abs)
    }
}
