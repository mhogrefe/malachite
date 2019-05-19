use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{CheckedSub, CheckedSubMul};

use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_greater_in_place_left;
use natural::Natural::{self, Large, Small};
use platform::Limb;

impl CheckedSubMul<Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by value. If b * c is greater than a, returns `None`.
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
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(10u32).checked_sub_mul(Natural::from(3u32), 4)),
    ///         "None");
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(15u32).checked_sub_mul(Natural::from(3u32), 4)),
    ///         "Some(3)");
    ///     assert_eq!(
    ///         format!("{:?}", Natural::trillion().checked_sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32)), "Some(995705032704)");
    /// }
    /// ```
    fn checked_sub_mul(mut self, b: Natural, c: Limb) -> Option<Natural> {
        if self.sub_mul_assign_limb_no_panic(&b, c) {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CheckedSubMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub_mul(self, b: Natural, c: u32) -> Option<Natural> {
        self.checked_sub_mul(b, Limb::from(c))
    }
}

impl<'a> CheckedSubMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by value and b by reference. If b * c is greater than a, returns `None`.
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
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(10u32).checked_sub_mul(&Natural::from(3u32), 4)),
    ///         "None");
    ///     assert_eq!(
    ///         format!("{:?}", Natural::from(15u32).checked_sub_mul(&Natural::from(3u32), 4)),
    ///         "Some(3)");
    ///     assert_eq!(
    ///         format!("{:?}", Natural::trillion().checked_sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32)), "Some(995705032704)");
    /// }
    /// ```
    fn checked_sub_mul(mut self, b: &'a Natural, c: Limb) -> Option<Natural> {
        if self.sub_mul_assign_limb_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedSubMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub_mul(self, b: &'a Natural, c: u32) -> Option<Natural> {
        self.checked_sub_mul(b, Limb::from(c))
    }
}

impl<'a> CheckedSubMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by reference and b by value. If b * c is greater than a, returns `None`.
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
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(10u32)).checked_sub_mul(Natural::from(3u32), 4)),
    ///         "None");
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(15u32)).checked_sub_mul(Natural::from(3u32), 4)),
    ///         "Some(3)");
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::trillion()).checked_sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32)), "Some(995705032704)");
    /// }
    /// ```
    fn checked_sub_mul(self, b: Natural, c: Limb) -> Option<Natural> {
        if c == 0 || b == 0 as Limb {
            return Some(self.clone());
        }
        let a_limb_count = self.limb_count();
        let b_limb_count = b.limb_count();
        if a_limb_count < b_limb_count {
            return None;
        } else if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                return self.checked_sub(product);
            }
        }
        let mut a_limbs = self.to_limbs_asc();
        let borrow = match b {
            Small(small_b) => limbs_sub_mul_limb_greater_in_place_left(&mut a_limbs, &[small_b], c),
            Large(ref b_limbs) => {
                limbs_sub_mul_limb_greater_in_place_left(&mut a_limbs, b_limbs, c)
            }
        };
        let nonzero_borrow = {
            if a_limb_count == b_limb_count {
                borrow != 0
            } else {
                limbs_sub_limb_in_place(
                    &mut a_limbs[usize::checked_from(b_limb_count).unwrap()..],
                    borrow,
                )
            }
        };
        if nonzero_borrow {
            None
        } else {
            let mut difference = Large(a_limbs);
            difference.trim();
            Some(difference)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedSubMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub_mul(self, b: Natural, c: u32) -> Option<Natural> {
        self.checked_sub_mul(b, Limb::from(c))
    }
}

impl<'a, 'b> CheckedSubMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by reference. If b * c is greater than a, returns `None`.
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
    /// use malachite_base::num::traits::CheckedSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(10u32)).checked_sub_mul(&Natural::from(3u32), 4)),
    ///         "None");
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::from(15u32)).checked_sub_mul(&Natural::from(3u32), 4)),
    ///         "Some(3)");
    ///     assert_eq!(
    ///         format!("{:?}", (&Natural::trillion()).checked_sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32)), "Some(995705032704)");
    /// }
    /// ```
    fn checked_sub_mul(self, b: &'a Natural, c: Limb) -> Option<Natural> {
        if c == 0 || *b == 0 as Limb {
            return Some(self.clone());
        }
        let a_limb_count = self.limb_count();
        let b_limb_count = b.limb_count();
        if a_limb_count < b_limb_count {
            return None;
        } else if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self.checked_sub(product);
            }
        }
        let mut a_limbs = self.to_limbs_asc();
        let borrow = match *b {
            Small(small_b) => limbs_sub_mul_limb_greater_in_place_left(&mut a_limbs, &[small_b], c),
            Large(ref b_limbs) => {
                limbs_sub_mul_limb_greater_in_place_left(&mut a_limbs, b_limbs, c)
            }
        };
        let nonzero_borrow = {
            if a_limb_count == b_limb_count {
                borrow != 0
            } else {
                limbs_sub_limb_in_place(
                    &mut a_limbs[usize::checked_from(b_limb_count).unwrap()..],
                    borrow,
                )
            }
        };
        if nonzero_borrow {
            None
        } else {
            let mut difference = Large(a_limbs);
            difference.trim();
            Some(difference)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> CheckedSubMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub_mul(self, b: &'a Natural, c: u32) -> Option<Natural> {
        self.checked_sub_mul(b, Limb::from(c))
    }
}

impl Natural {
    pub(crate) fn sub_mul_assign_limb_no_panic(&mut self, b: &Natural, c: Limb) -> bool {
        if c == 0 || *b == 0 as Limb {
            return false;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self.sub_assign_limb_no_panic(product);
            }
        }
        let a_limb_count = self.limb_count();
        let b_limb_count = b.limb_count();
        if a_limb_count < b_limb_count {
            return true;
        }
        let nonzero_borrow = {
            let a_limbs = self.promote_in_place();
            let borrow = match *b {
                Small(small) => limbs_sub_mul_limb_greater_in_place_left(a_limbs, &[small], c),
                Large(ref b_limbs) => limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c),
            };
            if a_limb_count == b_limb_count {
                borrow != 0
            } else {
                limbs_sub_limb_in_place(
                    &mut a_limbs[usize::checked_from(b_limb_count).unwrap()..],
                    borrow,
                )
            }
        };
        if !nonzero_borrow {
            self.trim();
        }
        nonzero_borrow
    }
}
