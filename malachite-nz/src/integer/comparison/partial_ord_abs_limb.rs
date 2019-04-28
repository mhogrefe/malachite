use integer::Integer;
use malachite_base::num::traits::PartialOrdAbs;
use platform::Limb;
use std::cmp::Ordering;

/// Compares the absolute value of an `Integer` to a `Limb`.
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
/// use malachite_base::num::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert!(Integer::from(123).gt_abs(&122));
///     assert!(Integer::from(123).ge_abs(&122));
///     assert!(Integer::from(123).lt_abs(&124));
///     assert!(Integer::from(123).le_abs(&124));
///     assert!(Integer::trillion().gt_abs(&123));
///     assert!(Integer::trillion().ge_abs(&123));
///     assert!((-Integer::trillion()).gt_abs(&123));
///     assert!((-Integer::trillion()).ge_abs(&123));
/// }
/// ```
impl PartialOrdAbs<Limb> for Integer {
    fn partial_cmp_abs(&self, other: &Limb) -> Option<Ordering> {
        self.abs.partial_cmp(other)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl PartialOrdAbs<u32> for Integer {
    #[inline]
    fn partial_cmp_abs(&self, other: &u32) -> Option<Ordering> {
        self.partial_cmp_abs(&Limb::from(*other))
    }
}

/// Compares a `Limb` to the absolute value of an `Integer`.
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
/// use malachite_base::num::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert!(122.lt_abs(&Integer::from(123)));
///     assert!(122.le_abs(&Integer::from(123)));
///     assert!(124.gt_abs(&Integer::from(123)));
///     assert!(123.ge_abs(&Integer::from(123)));
///     assert!(123.lt_abs(&Integer::trillion()));
///     assert!(123.le_abs(&Integer::trillion()));
///     assert!(123.lt_abs(&(-Integer::trillion())));
///     assert!(123.le_abs(&(-Integer::trillion())));
/// }
/// ```
impl PartialOrdAbs<Integer> for Limb {
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        self.partial_cmp(&other.abs)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl PartialOrdAbs<Integer> for u32 {
    #[inline]
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        Limb::from(*self).partial_cmp_abs(other)
    }
}
