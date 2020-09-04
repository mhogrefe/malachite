use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};

use natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left,
};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

macro_rules! large_left {
    ($a_limbs: ident, $b_limbs: ident, $c_limbs: ident) => {
        (
            Natural(Large(ref mut $a_limbs)),
            Natural(Large(ref $b_limbs)),
            Natural(Large(ref $c_limbs)),
        )
    }
}

macro_rules! large_right {
    ($self: ident, $a_limbs: ident, $b_limbs: ident, $c_limbs: ident) => {{
        let borrow = $a_limbs.len() < $b_limbs.len() + $c_limbs.len() - 1
            || limbs_sub_mul_in_place_left($a_limbs, $b_limbs, $c_limbs);
        if !borrow {
            $self.trim();
        }
        borrow
    }};
}

impl Natural {
    fn checked_sub_mul_limb_ref_ref(&self, b: &Natural, c: Limb) -> Option<Natural> {
        match (self, b, c) {
            (a, _, 0) | (a, natural_zero!(), _) => Some(a.clone()),
            (a, b @ Natural(Small(_)), c) => a.checked_sub(b * Natural::from(c)),
            (Natural(Small(_)), _, _) => None,
            (&Natural(Large(ref a_limbs)), &Natural(Large(ref b_limbs)), c) => {
                if a_limbs.len() >= b_limbs.len() {
                    limbs_sub_mul_limb_greater(a_limbs, b_limbs, c)
                        .map(Natural::from_owned_limbs_asc)
                } else {
                    None
                }
            }
        }
    }

    fn sub_mul_assign_limb_no_panic(&mut self, b: Natural, c: Limb) -> bool {
        match (&mut *self, b, c) {
            (_, _, 0) | (_, natural_zero!(), _) => false,
            (a, b @ Natural(Small(_)), c) => a.sub_assign_no_panic(b * Natural::from(c)),
            (Natural(Small(_)), _, _) => true,
            (Natural(Large(ref mut a_limbs)), Natural(Large(ref b_limbs)), c) => {
                let borrow = a_limbs.len() < b_limbs.len()
                    || limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0;
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    fn sub_mul_assign_limb_ref_no_panic(&mut self, b: &Natural, c: Limb) -> bool {
        match (&mut *self, b, c) {
            (_, _, 0) | (_, natural_zero!(), _) => false,
            (a, b @ Natural(Small(_)), c) => a.sub_assign_no_panic(b * Natural::from(c)),
            (Natural(Small(_)), _, _) => true,
            (Natural(Large(ref mut a_limbs)), Natural(Large(ref b_limbs)), c) => {
                let borrow = a_limbs.len() < b_limbs.len()
                    || limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0;
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    pub(crate) fn sub_mul_assign_no_panic(&mut self, b: Natural, c: Natural) -> bool {
        match (&mut *self, b, c) {
            (a, Natural(Small(small_b)), c) => a.sub_mul_assign_limb_no_panic(c, small_b),
            (a, b, Natural(Small(small_c))) => a.sub_mul_assign_limb_no_panic(b, small_c),
            (Natural(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }

    pub(crate) fn sub_mul_assign_val_ref_no_panic(&mut self, b: Natural, c: &Natural) -> bool {
        match (&mut *self, &b, c) {
            (ref mut a, Natural(Small(small_b)), c) => {
                a.sub_mul_assign_limb_ref_no_panic(c, *small_b)
            }
            (ref mut a, _, Natural(Small(small_c))) => a.sub_mul_assign_limb_no_panic(b, *small_c),
            (Natural(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }

    pub(crate) fn sub_mul_assign_ref_val_no_panic(&mut self, b: &Natural, c: Natural) -> bool {
        match (&mut *self, b, &c) {
            (ref mut a, Natural(Small(small_b)), _) => a.sub_mul_assign_limb_no_panic(c, *small_b),
            (ref mut a, b, Natural(Small(small_c))) => {
                a.sub_mul_assign_limb_ref_no_panic(b, *small_c)
            }
            (Natural(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }

    pub(crate) fn sub_mul_assign_ref_ref_no_panic(&mut self, b: &Natural, c: &Natural) -> bool {
        match (&mut *self, b, c) {
            (ref mut a, Natural(Small(small_b)), c) => {
                a.sub_mul_assign_limb_ref_no_panic(c, *small_b)
            }
            (ref mut a, b, Natural(Small(small_c))) => {
                a.sub_mul_assign_limb_ref_no_panic(b, *small_c)
            }
            (Natural(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }
}

impl CheckedSubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self`, y, and z by value. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(Natural::from(3u32), Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32),
    ///     Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     Natural::from(0x1_0000u32), Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: Natural, c: Natural) -> Option<Natural> {
        if self.sub_mul_assign_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self` and y by value and z by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(Natural::from(3u32), &Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32),
    ///     &Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: Natural, c: &'a Natural) -> Option<Natural> {
        if self.sub_mul_assign_val_ref_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self` and z by value and y by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(&Natural::from(3u32), Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32),
    ///     Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     &Natural::from(0x1_0000u32), Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: &'a Natural, c: Natural) -> Option<Natural> {
        if self.sub_mul_assign_ref_val_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, 'b> CheckedSubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self` by value and y and z by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(20u32)
    ///     .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32),
    ///     &Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///     &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(mut self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if self.sub_mul_assign_ref_ref_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, 'b, 'c> CheckedSubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (self), taking
    /// `self`, y, and z by reference. If y * z is greater than `self`, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", (&Natural::from(20u32))
    ///     .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
    ///     "Some(8)");
    /// assert_eq!(format!("{:?}", (&Natural::from(10u32)).checked_sub_mul(&Natural::from(3u32),
    ///     &Natural::from(4u32))), "None");
    /// assert_eq!(format!("{:?}", (&Natural::trillion()).checked_sub_mul(
    ///     &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// ```
    fn checked_sub_mul(self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        match (self, b, c) {
            (a, Natural(Small(small_b)), c) => a.checked_sub_mul_limb_ref_ref(c, *small_b),
            (a, b, Natural(Small(small_c))) => a.checked_sub_mul_limb_ref_ref(b, *small_c),
            (Natural(Small(_)), _, _) => None,
            (
                Natural(Large(ref a_limbs)),
                Natural(Large(ref b_limbs)),
                Natural(Large(ref c_limbs)),
            ) => {
                if a_limbs.len() >= b_limbs.len() + c_limbs.len() - 1 {
                    limbs_sub_mul(a_limbs, b_limbs, c_limbs).map(Natural::from_owned_limbs_asc)
                } else {
                    None
                }
            }
        }
    }
}
