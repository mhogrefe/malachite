use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;
use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_greater_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
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
        match (self, other) {
            (_, 0) => Some(self.clone()),
            (Natural(Small(small)), other) => small.checked_sub(other).map(Natural::from),
            (Natural(Large(ref limbs)), other) => {
                if *self < other {
                    None
                } else {
                    Some(Natural::from_owned_limbs_asc(
                        limbs_sub_limb(limbs, other).0,
                    ))
                }
            }
        }
    }

    // self -= other, return borrow
    fn sub_assign_limb_no_panic(&mut self, other: Limb) -> bool {
        match (&mut *self, other) {
            (_, 0) => false,
            (Natural(Small(ref mut x)), y) => match x.checked_sub(y) {
                Some(diff) => {
                    *x = diff;
                    false
                }
                None => true,
            },
            (Natural(Large(ref mut xs)), y) => {
                let borrow = limbs_sub_limb_in_place(xs, y);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    // self -= other, return borrow
    pub(crate) fn sub_assign_no_panic(&mut self, other: Natural) -> bool {
        match (&mut *self, other) {
            (_, natural_zero!()) => false,
            (x, Natural(Small(y))) => x.sub_assign_limb_no_panic(y),
            (Natural(Small(_)), _) => true,
            (&mut Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                let borrow = xs.len() < ys.len() || limbs_sub_greater_in_place_left(xs, ys);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    // self -= &other, return borrow
    pub(crate) fn sub_assign_ref_no_panic(&mut self, other: &Natural) -> bool {
        match (&mut *self, other) {
            (_, natural_zero!()) => false,
            (x, y) if std::ptr::eq(x, y) => {
                *self = Natural::ZERO;
                false
            }
            (x, &Natural(Small(y))) => x.sub_assign_limb_no_panic(y),
            (Natural(Small(_)), _) => true,
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                let borrow = xs.len() < ys.len() || limbs_sub_greater_in_place_left(xs, ys);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    // self = &other - self, return borrow
    pub(crate) fn sub_right_assign_no_panic(&mut self, other: &Natural) -> bool {
        match (&mut *self, other) {
            (natural_zero!(), y) => {
                *self = y.clone();
                false
            }
            (x, y) if std::ptr::eq(x, y) => {
                *self = Natural::ZERO;
                false
            }
            (Natural(Small(x)), y) => y.checked_sub_limb_ref(*x).map_or(true, |result| {
                *self = result;
                false
            }),
            (_, Natural(Small(_))) => true,
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                let borrow = xs.len() > ys.len() || limbs_vec_sub_in_place_right(ys, xs);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }
}

impl CheckedSub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by value. If the second
    /// `Natural` is greater than the first, returns `None`.
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
    fn checked_sub(mut self, other: Natural) -> Option<Natural> {
        if self.sub_assign_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSub<&'a Natural> for Natural {
    type Output = Natural;

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
    fn checked_sub(mut self, other: &'a Natural) -> Option<Natural> {
        if self.sub_assign_ref_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSub<Natural> for &'a Natural {
    type Output = Natural;

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
    fn checked_sub(self, mut other: Natural) -> Option<Natural> {
        if other.sub_right_assign_no_panic(self) {
            None
        } else {
            Some(other)
        }
    }
}

impl<'a, 'b> CheckedSub<&'a Natural> for &'b Natural {
    type Output = Natural;

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
    fn checked_sub(self, other: &'a Natural) -> Option<Natural> {
        match (self, other) {
            (x, y) if std::ptr::eq(x, y) => Some(Natural::ZERO),
            (x, &natural_zero!()) => Some(x.clone()),
            (x, &Natural(Small(y))) => x.checked_sub_limb_ref(y),
            (&Natural(Small(_)), _) => None,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                if self < other {
                    None
                } else {
                    Some(Natural::from_owned_limbs_asc(limbs_sub(xs, ys).0))
                }
            }
        }
    }
}
