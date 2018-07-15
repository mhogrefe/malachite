use malachite_base::num::{CheckedSub, Zero};
use natural::arithmetic::sub::{limbs_sub, limbs_sub_in_place_left};
use natural::Natural::{self, Large, Small};

impl Natural {
    // self -= other, return borrow
    pub(crate) fn sub_assign_no_panic<'a>(&mut self, other: &'a Natural) -> bool {
        if *other == 0 {
            false
        } else if self as *const Natural == other as *const Natural {
            *self = Small(0);
            false
        } else if self.limb_count() < other.limb_count() {
            true
        } else if let Small(y) = *other {
            self.sub_assign_u32_no_panic(y)
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
}

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
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
        if self.sub_assign_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by reference.
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
///     assert_eq!(format!("{:?}", (&Natural::ZERO.checked_sub(&Natural::from(123u32)))), "None");
///     assert_eq!(format!("{:?}", (&Natural::from(123u32).checked_sub(&Natural::ZERO))),
///         "Some(123)");
///     assert_eq!(format!("{:?}", (&Natural::from(456u32).checked_sub(&Natural::from(123u32)))),
///         "Some(333)");
///     assert_eq!(format!("{:?}", (&(Natural::trillion() * 3).checked_sub(&Natural::trillion()))),
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
