use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;

use natural::arithmetic::sub::{limbs_sub, limbs_sub_in_place_left, limbs_vec_sub_in_place_right};
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by value. If the second `Natural`
/// is greater than the first, returns `None`.
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
/// use malachite_base::num::arithmetic::traits::CheckedSub;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", Natural::ZERO.checked_sub(Natural::from(123u32))), "None");
///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(Natural::ZERO)), "Some(123)");
///     assert_eq!(format!("{:?}", Natural::from(456u32).checked_sub(Natural::from(123u32))),
///         "Some(333)");
///     assert_eq!(format!("{:?}", (Natural::trillion() * 3).checked_sub(Natural::trillion())),
///         "Some(2000000000000)");
/// }
/// ```
impl CheckedSub<Natural> for Natural {
    type Output = Natural;

    fn checked_sub(mut self, other: Natural) -> Option<Natural> {
        if self.sub_assign_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference. If the second `Natural` is greater than the first, returns `None`.
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
/// use malachite_base::num::arithmetic::traits::CheckedSub;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", Natural::ZERO.checked_sub(&Natural::from(123u32))), "None");
///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(&Natural::ZERO)), "Some(123)");
///     assert_eq!(format!("{:?}", Natural::from(456u32).checked_sub(&Natural::from(123u32))),
///         "Some(333)");
///     assert_eq!(format!("{:?}", (Natural::trillion() * 3).checked_sub(&Natural::trillion())),
///         "Some(2000000000000)");
/// }
/// ```
impl<'a> CheckedSub<&'a Natural> for Natural {
    type Output = Natural;

    fn checked_sub(mut self, other: &'a Natural) -> Option<Natural> {
        if self.sub_assign_ref_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value. If the second `Natural` is greater than the first, returns `None`.
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
/// use malachite_base::num::arithmetic::traits::CheckedSub;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", (&Natural::ZERO).checked_sub(Natural::from(123u32))), "None");
///     assert_eq!(format!("{:?}", (&Natural::from(123u32)).checked_sub(Natural::ZERO)),
///         "Some(123)");
///     assert_eq!(format!("{:?}", (&Natural::from(456u32)).checked_sub(Natural::from(123u32))),
///         "Some(333)");
///     assert_eq!(format!("{:?}", (&(Natural::trillion() * 3)).checked_sub(Natural::trillion())),
///         "Some(2000000000000)");
/// }
/// ```
impl<'a> CheckedSub<Natural> for &'a Natural {
    type Output = Natural;

    fn checked_sub(self, mut other: Natural) -> Option<Natural> {
        if other.sub_right_assign_no_panic(self) {
            None
        } else {
            Some(other)
        }
    }
}

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by reference. If the second
/// `Natural` is greater than the first, returns `None`.
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
/// use malachite_base::num::arithmetic::traits::CheckedSub;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", (&Natural::ZERO).checked_sub(&Natural::from(123u32))), "None");
///     assert_eq!(format!("{:?}", (&Natural::from(123u32)).checked_sub(&Natural::ZERO)),
///         "Some(123)");
///     assert_eq!(format!("{:?}", (&Natural::from(456u32)).checked_sub(&Natural::from(123u32))),
///         "Some(333)");
///     assert_eq!(format!("{:?}", (&(Natural::trillion() * 3)).checked_sub(&Natural::trillion())),
///         "Some(2000000000000)");
/// }
/// ```
impl<'a, 'b> CheckedSub<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn checked_sub(self, other: &'a Natural) -> Option<Natural> {
        if self as *const Natural == other as *const Natural {
            Some(Natural::ZERO)
        } else {
            match (self, other) {
                (x, &Small(0)) => Some(x.clone()),
                (x, &Small(y)) => x.checked_sub(y),
                (&Small(_), _) => None,
                (&Large(ref xs), &Large(ref ys)) => {
                    if self < other {
                        None
                    } else {
                        let mut difference = Large(limbs_sub(xs, ys).0);
                        difference.trim();
                        Some(difference)
                    }
                }
            }
        }
    }
}

impl Natural {
    // self -= other, return borrow
    pub(crate) fn sub_assign_no_panic(&mut self, other: Natural) -> bool {
        if other == 0 as Limb {
            false
        } else if self.limb_count() < other.limb_count() {
            true
        } else if let Small(y) = other {
            self.sub_assign_limb_no_panic(y)
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    if limbs_sub_in_place_left(xs, ys) {
                        return true;
                    }
                }
                _ => unreachable!(),
            }
            self.trim();
            false
        }
    }

    // self -= &other, return borrow
    pub(crate) fn sub_assign_ref_no_panic(&mut self, other: &Natural) -> bool {
        if *other == 0 as Limb {
            false
        } else if self as *const Natural == other as *const Natural {
            *self = Small(0);
            false
        } else if self.limb_count() < other.limb_count() {
            true
        } else if let Small(y) = *other {
            self.sub_assign_limb_no_panic(y)
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    if limbs_sub_in_place_left(xs, ys) {
                        return true;
                    }
                }
                _ => unreachable!(),
            }
            self.trim();
            false
        }
    }

    // self = &other - self, return borrow
    pub(crate) fn sub_right_assign_no_panic(&mut self, other: &Natural) -> bool {
        if self as *const Natural == other as *const Natural {
            *self = Small(0);
            false
        } else if self.limb_count() > other.limb_count() {
            true
        } else if let Small(y) = *self {
            if let Some(result) = other.checked_sub(y) {
                *self = result;
                false
            } else {
                true
            }
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    if limbs_vec_sub_in_place_right(ys, xs) {
                        return true;
                    }
                }
                _ => unreachable!(),
            }
            self.trim();
            false
        }
    }
}
