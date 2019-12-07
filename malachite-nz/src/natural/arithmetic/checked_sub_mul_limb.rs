use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};

use natural::arithmetic::sub_mul_limb::{
    limbs_sub_mul_limb_greater, limbs_sub_mul_limb_greater_in_place_left,
    limbs_sub_mul_limb_greater_in_place_right,
};
use natural::Natural::{self, Large};
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
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
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
        if self.sub_mul_assign_limb_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
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
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
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
        if self.sub_mul_assign_limb_ref_no_panic(b, c) {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
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
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
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
    fn checked_sub_mul(self, mut b: Natural, c: Limb) -> Option<Natural> {
        if b.sub_mul_right_assign_limb_no_panic(self, c) {
            None
        } else {
            Some(b)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
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
    /// use malachite_base::num::arithmetic::traits::CheckedSubMul;
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
            Some(self.clone())
        } else if c == 1 {
            self.checked_sub(b)
        } else if self.limb_count() < b.limb_count() {
            None
        } else {
            let (result, fallback) = match (&self, &b) {
                (&Large(ref a_limbs), &Large(ref b_limbs)) => {
                    (limbs_sub_mul_limb_greater(a_limbs, b_limbs, c), false)
                }
                _ => (None, true),
            };
            if fallback {
                self.checked_sub(b * Natural::from(c))
            } else {
                result.map(|limbs| {
                    let mut result = Large(limbs);
                    result.trim();
                    result
                })
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a, 'b> CheckedSubMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub_mul(self, b: &'a Natural, c: u32) -> Option<Natural> {
        self.checked_sub_mul(b, Limb::from(c))
    }
}

impl Natural {
    pub(crate) fn sub_mul_assign_limb_no_panic(&mut self, b: Natural, c: Limb) -> bool {
        if c == 0 || b == 0 as Limb {
            false
        } else if c == 1 {
            self.sub_assign_no_panic(b)
        } else if self.limb_count() < b.limb_count() {
            true
        } else {
            let (borrow, fallback) = match (&mut *self, &b) {
                (&mut Large(ref mut a_limbs), &Large(ref b_limbs)) => (
                    limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0,
                    false,
                ),
                _ => (false, true),
            };
            if fallback {
                self.sub_assign_no_panic(b * Natural::from(c))
            } else if borrow {
                true
            } else {
                self.trim();
                false
            }
        }
    }

    pub(crate) fn sub_mul_assign_limb_ref_no_panic(&mut self, b: &Natural, c: Limb) -> bool {
        if c == 0 || *b == 0 as Limb {
            false
        } else if c == 1 {
            self.sub_assign_ref_no_panic(b)
        } else if self.limb_count() < b.limb_count() {
            true
        } else {
            let (borrow, fallback) = match (&mut *self, &b) {
                (&mut Large(ref mut a_limbs), &Large(ref b_limbs)) => (
                    limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0,
                    false,
                ),
                _ => (false, true),
            };
            if fallback {
                self.sub_assign_no_panic(b * Natural::from(c))
            } else if borrow {
                true
            } else {
                self.trim();
                false
            }
        }
    }

    // self = &a - self * c, return borrow
    pub(crate) fn sub_mul_right_assign_limb_no_panic(&mut self, a: &Natural, c: Limb) -> bool {
        if c == 0 || *self == 0 as Limb {
            *self = a.clone();
            false
        } else if c == 1 {
            self.sub_right_assign_no_panic(a)
        } else if a.limb_count() < self.limb_count() {
            true
        } else {
            let (borrow, fallback) = match (&a, &mut *self) {
                (&Large(ref a_limbs), &mut Large(ref mut b_limbs)) => (
                    limbs_sub_mul_limb_greater_in_place_right(a_limbs, b_limbs, c) != 0,
                    false,
                ),
                _ => (false, true),
            };
            if fallback {
                *self *= Natural::from(c);
                self.sub_right_assign_no_panic(a)
            } else if borrow {
                true
            } else {
                self.trim();
                false
            }
        }
    }
}
