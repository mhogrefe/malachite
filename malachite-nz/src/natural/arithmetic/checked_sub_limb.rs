use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::conversion::traits::CheckedFrom;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::WrappingFrom;

use natural::arithmetic::sub_limb::{limbs_sub_limb, limbs_sub_limb_in_place};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

impl CheckedSub<Limb> for Natural {
    type Output = Natural;

    /// Subtracts a `Limb` from a `Natural`, taking the `Natural` by value. If the `Limb` is greater
    /// than the `Natural`, returns `None`.
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
    ///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(123)), "Some(0)");
    ///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(0)), "Some(123)");
    ///     assert_eq!(format!("{:?}", Natural::from(456u32).checked_sub(123)), "Some(333)");
    ///     assert_eq!(format!("{:?}", Natural::from(123u32).checked_sub(456)), "None");
    ///     assert_eq!(format!("{:?}", Natural::trillion().checked_sub(123)), "Some(999999999877)");
    /// }
    /// ```
    fn checked_sub(mut self, other: Limb) -> Option<Natural> {
        if self.sub_assign_limb_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CheckedSub<u32> for Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub(self, other: u32) -> Option<Natural> {
        self.checked_sub(Limb::from(other))
    }
}

impl<'a> CheckedSub<Limb> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Limb` from a `Natural`, taking the `Natural` by reference. If the `Limb` is
    /// greater than the `Natural`, returns `None`.
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
    ///     assert_eq!(format!("{:?}", &Natural::from(123u32).checked_sub(123)), "Some(0)");
    ///     assert_eq!(format!("{:?}", &Natural::from(123u32).checked_sub(0)), "Some(123)");
    ///     assert_eq!(format!("{:?}", &Natural::from(456u32).checked_sub(123)), "Some(333)");
    ///     assert_eq!(format!("{:?}", &Natural::from(123u32).checked_sub(456)), "None");
    ///     assert_eq!(format!("{:?}", &Natural::trillion().checked_sub(123)),
    ///         "Some(999999999877)");
    /// }
    /// ```
    fn checked_sub(self, other: Limb) -> Option<Natural> {
        if other == 0 {
            return Some(self.clone());
        }
        match *self {
            Natural(Small(small)) => small.checked_sub(other).map(|u| Natural(Small(u))),
            Natural(Large(ref limbs)) => {
                if *self < other {
                    None
                } else {
                    let mut difference = Natural(Large(limbs_sub_limb(limbs, other).0));
                    difference.trim();
                    Some(difference)
                }
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CheckedSub<u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn checked_sub(self, other: u32) -> Option<Natural> {
        self.checked_sub(Limb::from(other))
    }
}

impl CheckedSub<Natural> for Limb {
    type Output = Limb;

    /// Subtracts a `Natural` from a `Limb`, taking the `Natural` by value. If the `Natural` is
    /// greater than the `Limb`, returns `None`.
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
    /// use malachite_base::num::arithmetic::traits::CheckedSub;
    /// use malachite_base::num::basic::traits::Zero;
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
    #[inline]
    fn checked_sub(self, other: Natural) -> Option<Limb> {
        CheckedSub::checked_sub(self, &other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CheckedSub<Natural> for u32 {
    type Output = u32;

    #[inline]
    fn checked_sub(self, other: Natural) -> Option<u32> {
        CheckedSub::checked_sub(Limb::from(self), other).map(u32::wrapping_from)
    }
}

impl<'a> CheckedSub<&'a Natural> for Limb {
    type Output = Limb;

    /// Subtracts a `Natural` from a `Limb`, taking the `Natural` by reference. If the `Natural` is
    /// greater than the `Limb`, returns `None`.
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
    /// use malachite_base::num::arithmetic::traits::CheckedSub;
    /// use malachite_base::num::basic::traits::Zero;
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
    #[inline]
    fn checked_sub(self, other: &'a Natural) -> Option<Limb> {
        Limb::checked_from(other).and_then(|x| self.checked_sub(x))
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CheckedSub<&'a Natural> for u32 {
    type Output = u32;

    #[inline]
    fn checked_sub(self, other: &'a Natural) -> Option<u32> {
        CheckedSub::checked_sub(Limb::from(self), other).map(u32::wrapping_from)
    }
}

impl Natural {
    // self -= other, return borrow
    pub(crate) fn sub_assign_limb_no_panic(&mut self, other: Limb) -> bool {
        if other == 0 {
            return false;
        }
        match *self {
            Natural(Small(ref mut small)) => {
                return match small.checked_sub(other) {
                    Some(difference) => {
                        *small = difference;
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
