use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;

use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
    limbs_vec_sub_in_place_right,
};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

impl Natural {
    pub(crate) fn checked_sub_limb(mut self, other: Limb) -> Option<Natural> {
        if self.sub_assign_limb_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }

    pub(crate) fn checked_sub_limb_ref(&self, other: Limb) -> Option<Natural> {
        if other == 0 {
            return Some(self.clone());
        }
        match *self {
            Natural(Small(small)) => small.checked_sub(other).map(|u| Natural(Small(u))),
            Natural(Large(ref limbs)) => {
                if *self < other {
                    None
                } else {
                    let mut diff = Natural(Large(limbs_sub_limb(limbs, other).0));
                    diff.trim();
                    Some(diff)
                }
            }
        }
    }

    // self -= other, return borrow
    pub(crate) fn sub_assign_limb_no_panic(&mut self, other: Limb) -> bool {
        if other == 0 {
            return false;
        }
        match *self {
            Natural(Small(ref mut small)) => {
                return match small.checked_sub(other) {
                    Some(diff) => {
                        *small = diff;
                        false
                    }
                    None => true,
                };
            }
            Natural(Large(ref mut limbs)) => {
                if limbs_sub_limb_in_place(limbs, other) {
                    return true;
                }
            }
        }
        self.trim();
        false
    }
}

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
/// assert_eq!(format!("{:?}", Natural::ZERO.checked_sub(Natural::from(123u32))), "None");
/// assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(Natural::ZERO)), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32).checked_sub(Natural::from(123u32))),
///     "Some(333)");
/// assert_eq!(
///     format!(
///         "{:?}",
///         (Natural::trillion() * Natural::from(3u32)).checked_sub(Natural::trillion())
///     ),
///     "Some(2000000000000)"
/// );
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
/// assert_eq!(format!("{:?}", Natural::ZERO.checked_sub(&Natural::from(123u32))), "None");
/// assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(&Natural::ZERO)), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32).checked_sub(&Natural::from(123u32))),
///     "Some(333)");
/// assert_eq!(
///     format!(
///         "{:?}",
///         (Natural::trillion() * Natural::from(3u32)).checked_sub(&Natural::trillion())
///     ),
///     "Some(2000000000000)"
/// );
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
/// assert_eq!(format!("{:?}", (&Natural::ZERO).checked_sub(Natural::from(123u32))), "None");
/// assert_eq!(format!("{:?}", (&Natural::from(123u32)).checked_sub(Natural::ZERO)),
///     "Some(123)");
/// assert_eq!(format!("{:?}", (&Natural::from(456u32)).checked_sub(Natural::from(123u32))),
///     "Some(333)");
/// assert_eq!(
///     format!(
///         "{:?}",
///         (&(Natural::trillion() * Natural::from(3u32))).checked_sub(Natural::trillion())
///     ),
///     "Some(2000000000000)"
/// );
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
/// assert_eq!(format!("{:?}", (&Natural::ZERO).checked_sub(&Natural::from(123u32))), "None");
/// assert_eq!(format!("{:?}", (&Natural::from(123u32)).checked_sub(&Natural::ZERO)),
///     "Some(123)");
/// assert_eq!(format!("{:?}", (&Natural::from(456u32)).checked_sub(&Natural::from(123u32))),
///     "Some(333)");
/// assert_eq!(
///     format!(
///         "{:?}",
///         (&(Natural::trillion() * Natural::from(3u32))).checked_sub(&Natural::trillion())
///     ),
///     "Some(2000000000000)"
/// );
/// ```
impl<'a, 'b> CheckedSub<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn checked_sub(self, other: &'a Natural) -> Option<Natural> {
        if self as *const Natural == other as *const Natural {
            Some(Natural::ZERO)
        } else {
            match (self, other) {
                (x, &natural_zero!()) => Some(x.clone()),
                (x, &Natural(Small(y))) => x.checked_sub_limb_ref(y),
                (&Natural(Small(_)), _) => None,
                (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                    if self < other {
                        None
                    } else {
                        let mut diff = Natural(Large(limbs_sub(xs, ys).0));
                        diff.trim();
                        Some(diff)
                    }
                }
            }
        }
    }
}

impl Natural {
    // self -= other, return borrow
    pub(crate) fn sub_assign_no_panic(&mut self, other: Natural) -> bool {
        if other == 0 {
            false
        } else if self.limb_count() < other.limb_count() {
            true
        } else if let Natural(Small(y)) = other {
            self.sub_assign_limb_no_panic(y)
        } else {
            match (&mut (*self), other) {
                (&mut Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
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
        if *other == 0 {
            false
        } else if self as *const Natural == other as *const Natural {
            *self = Natural::ZERO;
            false
        } else if self.limb_count() < other.limb_count() {
            true
        } else if let Natural(Small(y)) = *other {
            self.sub_assign_limb_no_panic(y)
        } else {
            match (&mut (*self), other) {
                (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
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
            *self = Natural::ZERO;
            false
        } else if self.limb_count() > other.limb_count() {
            true
        } else if let Natural(Small(y)) = *self {
            if let Some(result) = other.checked_sub_limb_ref(y) {
                *self = result;
                false
            } else {
                true
            }
        } else {
            match (&mut (*self), other) {
                (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
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
