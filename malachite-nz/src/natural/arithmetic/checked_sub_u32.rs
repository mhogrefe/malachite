use malachite_base::misc::CheckedFrom;
use malachite_base::num::CheckedSub;
use natural::arithmetic::sub_u32::{limbs_sub_limb, limbs_sub_limb_in_place};
use natural::Natural::{self, Large, Small};

impl Natural {
    // self -= other, return borrow
    pub(crate) fn sub_assign_u32_no_panic(&mut self, other: u32) -> bool {
        if other == 0 {
            return false;
        }
        match *self {
            Small(ref mut small) => {
                return match small.checked_sub(other) {
                    Some(difference) => {
                        *small = difference;
                        false
                    }
                    None => true,
                };
            }
            Large(ref mut limbs) => {
                if limbs_sub_limb_in_place(limbs, other) {
                    return true;
                }
            }
        }
        self.trim();
        false
    }
}

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by value. If the `u32` is greater than
/// the `Natural`, returns `None`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{CheckedSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(123)), "Some(0)");
///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(0)), "Some(123)");
///     assert_eq!(format!("{:?}", Natural::from(456u32).checked_sub(123)), "Some(333)");
///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(456)), "None");
///     assert_eq!(format!("{:?}", Natural::trillion().checked_sub(123)), "Some(999999999877)");
/// }
/// ```
impl CheckedSub<u32> for Natural {
    type Output = Natural;

    fn checked_sub(mut self, other: u32) -> Option<Natural> {
        if self.sub_assign_u32_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by reference. If the `u32` is greater
/// than the `Natural`, returns `None`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{CheckedSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", &Natural::from(123u32).checked_sub(123)), "Some(0)");
///     assert_eq!(format!("{:?}", &Natural::from(123u32).checked_sub(0)), "Some(123)");
///     assert_eq!(format!("{:?}", &Natural::from(456u32).checked_sub(123)), "Some(333)");
///     assert_eq!(format!("{:?}", &Natural::from(123u32).checked_sub(456)), "None");
///     assert_eq!(format!("{:?}", &Natural::trillion().checked_sub(123)), "Some(999999999877)");
/// }
/// ```
impl<'a> CheckedSub<u32> for &'a Natural {
    type Output = Natural;

    fn checked_sub(self, other: u32) -> Option<Natural> {
        if other == 0 {
            return Some(self.clone());
        }
        match *self {
            Small(small) => small.checked_sub(other).map(Small),
            Large(ref limbs) => {
                if *self < other {
                    None
                } else {
                    let mut difference = Large(limbs_sub_limb(limbs, other).0);
                    difference.trim();
                    Some(difference)
                }
            }
        }
    }
}

/// Subtracts a `Natural` from a `u32`, taking the `Natural` by value. If the `Natural` is greater
/// than the `u32`, returns `None`.
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
/// use malachite_base::num::{CheckedSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123.checked_sub(Natural::from(123u32)), Some(0));
///     assert_eq!(123.checked_sub(Natural::ZERO), Some(123));
///     assert_eq!(456.checked_sub(Natural::from(123u32)), Some(333));
///     assert_eq!(123.checked_sub(Natural::from(456u32)), None);
///     assert_eq!(123.checked_sub(Natural::trillion()), None);
/// }
/// ```
impl CheckedSub<Natural> for u32 {
    type Output = u32;

    fn checked_sub(self, other: Natural) -> Option<u32> {
        CheckedSub::checked_sub(self, &other)
    }
}

/// Subtracts a `Natural` from a `u32`, taking the `Natural` by reference. If the `Natural` is
/// greater than the `u32`, returns `None`.
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
/// use malachite_base::num::{CheckedSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123.checked_sub(&Natural::from(123u32)), Some(0));
///     assert_eq!(123.checked_sub(&Natural::ZERO), Some(123));
///     assert_eq!(456.checked_sub(&Natural::from(123u32)), Some(333));
///     assert_eq!(123.checked_sub(&Natural::from(456u32)), None);
///     assert_eq!(123.checked_sub(&Natural::trillion()), None);
/// }
/// ```
impl<'a> CheckedSub<&'a Natural> for u32 {
    type Output = u32;

    fn checked_sub(self, other: &'a Natural) -> Option<u32> {
        u32::checked_from(other).and_then(|x| self.checked_sub(x))
    }
}
