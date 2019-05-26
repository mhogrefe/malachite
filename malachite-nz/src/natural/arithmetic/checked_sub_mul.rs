use malachite_base::num::traits::CheckedSubMul;

use natural::arithmetic::sub_mul::{limbs_sub_mul, limbs_sub_mul_in_place_left};
use natural::Natural::{self, Large, Small};

impl CheckedSubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by value. If b * c is greater than a, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::from(20u32)
    ///         .checked_sub_mul(Natural::from(3u32), Natural::from(4u32))),
    ///         "Some(8)");
    ///     assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32),
    ///         Natural::from(4u32))), "None");
    ///     assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///         Natural::from(0x1_0000u32), Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// }
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

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and b by value and c by reference. If b * c is greater than a, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::from(20u32)
    ///         .checked_sub_mul(Natural::from(3u32), &Natural::from(4u32))),
    ///         "Some(8)");
    ///     assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32),
    ///         &Natural::from(4u32))), "None");
    ///     assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///         Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// }
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

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and c by value and b by reference. If b * c is greater than a, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::from(20u32)
    ///         .checked_sub_mul(&Natural::from(3u32), Natural::from(4u32))),
    ///         "Some(8)");
    ///     assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32),
    ///         Natural::from(4u32))), "None");
    ///     assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///         &Natural::from(0x1_0000u32), Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// }
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

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` by value and b and c by reference. If b * c is greater than a, returns `None`.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::from(20u32)
    ///         .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
    ///         "Some(8)");
    ///     assert_eq!(format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32),
    ///         &Natural::from(4u32))), "None");
    ///     assert_eq!(format!("{:?}", Natural::trillion().checked_sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// }
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

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by reference. If b * c is greater than a, returns `None`.
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
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", (&Natural::from(20u32))
    ///         .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
    ///         "Some(8)");
    ///     assert_eq!(format!("{:?}", (&Natural::from(10u32)).checked_sub_mul(&Natural::from(3u32),
    ///         &Natural::from(4u32))), "None");
    ///     assert_eq!(format!("{:?}", (&Natural::trillion()).checked_sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32))), "Some(995705032704)");
    /// }
    /// ```
    fn checked_sub_mul(self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if let Small(small_b) = *b {
            self.checked_sub_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.checked_sub_mul(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            None
        } else {
            if let Large(ref a_limbs) = *self {
                if let Large(ref b_limbs) = *b {
                    if let Large(ref c_limbs) = *c {
                        return limbs_sub_mul(a_limbs, b_limbs, c_limbs).map(|result_limbs| {
                            let mut result = Large(result_limbs);
                            result.trim();
                            result
                        });
                    }
                }
            }
            unreachable!();
        }
    }
}

impl Natural {
    fn sub_mul_assign_helper(&mut self, b: &Natural, c: &Natural) -> bool {
        {
            if let Large(ref mut a_limbs) = *self {
                if let Large(ref b_limbs) = *b {
                    if let Large(ref c_limbs) = *c {
                        if limbs_sub_mul_in_place_left(a_limbs, b_limbs, c_limbs) {
                            return true;
                        }
                    }
                }
            }
        }
        self.trim();
        false
    }

    pub(crate) fn sub_mul_assign_no_panic(&mut self, b: Natural, c: Natural) -> bool {
        if let Small(small_b) = b {
            self.sub_mul_assign_limb_no_panic(c, small_b)
        } else if let Small(small_c) = c {
            self.sub_mul_assign_limb_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(&b, &c)
        }
    }

    pub(crate) fn sub_mul_assign_val_ref_no_panic(&mut self, b: Natural, c: &Natural) -> bool {
        if let Small(small_b) = b {
            self.sub_mul_assign_limb_ref_no_panic(c, small_b)
        } else if let Small(small_c) = *c {
            self.sub_mul_assign_limb_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(&b, c)
        }
    }

    pub(crate) fn sub_mul_assign_ref_val_no_panic(&mut self, b: &Natural, c: Natural) -> bool {
        if let Small(small_b) = *b {
            self.sub_mul_assign_limb_no_panic(c, small_b)
        } else if let Small(small_c) = c {
            self.sub_mul_assign_limb_ref_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(b, &c)
        }
    }

    pub(crate) fn sub_mul_assign_ref_ref_no_panic(&mut self, b: &Natural, c: &Natural) -> bool {
        if let Small(small_b) = *b {
            self.sub_mul_assign_limb_ref_no_panic(c, small_b)
        } else if let Small(small_c) = *c {
            self.sub_mul_assign_limb_ref_no_panic(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            true
        } else {
            self.sub_mul_assign_helper(b, c)
        }
    }
}
