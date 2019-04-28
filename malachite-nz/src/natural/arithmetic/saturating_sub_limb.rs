#[cfg(feature = "64_bit_limbs")]
use malachite_base::conversion::WrappingFrom;
use malachite_base::num::traits::{CheckedSub, SaturatingSub, SaturatingSubAssign, Zero};
use natural::Natural;
use platform::Limb;

impl SaturatingSub<Limb> for Natural {
    type Output = Natural;

    /// Subtracts a `Limb` from a `Natural`, taking the `Natural` by value and returning zero if the
    /// `Limb` is greater than the `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSub, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((Natural::from(123u32).saturating_sub(123)).to_string(), "0");
    ///     assert_eq!((Natural::from(123u32).saturating_sub(0)).to_string(), "123");
    ///     assert_eq!((Natural::from(456u32).saturating_sub(123)).to_string(), "333");
    ///     assert_eq!((Natural::from(123u32).saturating_sub(456)).to_string(), "0");
    ///     assert_eq!((Natural::trillion().saturating_sub(123)).to_string(), "999999999877");
    /// }
    /// ```
    fn saturating_sub(self, other: Limb) -> Natural {
        self.checked_sub(other).unwrap_or(Natural::ZERO)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingSub<u32> for Natural {
    type Output = Natural;

    fn saturating_sub(self, other: u32) -> Natural {
        self.saturating_sub(Limb::from(other))
    }
}

impl<'a> SaturatingSub<Limb> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Limb` from a `Natural`, taking the `Natural` by reference and returning zero if
    /// the `Limb` is greater than the `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSub, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(123u32).saturating_sub(123)).to_string(), "0");
    ///     assert_eq!((&Natural::from(123u32).saturating_sub(0)).to_string(), "123");
    ///     assert_eq!((&Natural::from(456u32).saturating_sub(123)).to_string(), "333");
    ///     assert_eq!((&Natural::from(123u32).saturating_sub(456)).to_string(), "0");
    ///     assert_eq!((&Natural::trillion().saturating_sub(123)).to_string(), "999999999877");
    /// }
    /// ```
    fn saturating_sub(self, other: Limb) -> Natural {
        self.checked_sub(other).unwrap_or(Natural::ZERO)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingSub<u32> for &'a Natural {
    type Output = Natural;

    fn saturating_sub(self, other: u32) -> Natural {
        self.saturating_sub(Limb::from(other))
    }
}

impl SaturatingSubAssign<Limb> for Natural {
    /// Subtracts a `Limb` from a `Natural` in place. Sets the `Natural` to zero if it is smaller
    /// than the `Limb`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSubAssign, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(15u32);
    ///     x.saturating_sub_assign(1);
    ///     x.saturating_sub_assign(2);
    ///     x.saturating_sub_assign(3);
    ///     x.saturating_sub_assign(4);
    ///     assert_eq!(x.to_string(), "5");
    ///     x.saturating_sub_assign(10);
    ///     assert_eq!(x.to_string(), "0");
    /// }
    /// ```
    fn saturating_sub_assign(&mut self, other: Limb) {
        if self.sub_assign_limb_no_panic(other) {
            *self = Natural::ZERO;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingSubAssign<u32> for Natural {
    #[inline]
    fn saturating_sub_assign(&mut self, other: u32) {
        self.saturating_sub_assign(Limb::from(other))
    }
}

impl SaturatingSub<Natural> for Limb {
    type Output = Limb;

    /// Subtracts a `Natural` from a `Limb`, taking the `Natural` by value, returning zero if the
    /// `Natural` is greater than the `Limb`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSub, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(123.saturating_sub(Natural::from(123u32)), 0);
    ///     assert_eq!(123.saturating_sub(Natural::ZERO), 123);
    ///     assert_eq!(456.saturating_sub(Natural::from(123u32)), 333);
    ///     assert_eq!(123.saturating_sub(Natural::from(456u32)), 0);
    /// }
    /// ```
    fn saturating_sub(self, other: Natural) -> Limb {
        CheckedSub::checked_sub(self, &other).unwrap_or(0)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingSub<Natural> for u32 {
    type Output = u32;

    #[inline]
    fn saturating_sub(self, other: Natural) -> u32 {
        u32::wrapping_from(SaturatingSub::saturating_sub(Limb::from(self), other))
    }
}

impl<'a> SaturatingSub<&'a Natural> for Limb {
    type Output = Limb;

    /// Subtracts a `Natural` from a `Limb`, taking the `Natural` by reference, returning zero if
    /// the `Natural` is greater than the `Limb`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSub, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(123.saturating_sub(&Natural::from(123u32)), 0);
    ///     assert_eq!(123.saturating_sub(&Natural::ZERO), 123);
    ///     assert_eq!(456.saturating_sub(&Natural::from(123u32)), 333);
    ///     assert_eq!(123.saturating_sub(&Natural::from(456u32)), 0);
    /// }
    /// ```
    fn saturating_sub(self, other: &'a Natural) -> Limb {
        CheckedSub::checked_sub(self, other).unwrap_or(0)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingSub<&'a Natural> for u32 {
    type Output = u32;

    #[inline]
    fn saturating_sub(self, other: &'a Natural) -> u32 {
        u32::wrapping_from(SaturatingSub::saturating_sub(Limb::from(self), other))
    }
}

impl SaturatingSubAssign<Natural> for Limb {
    /// Subtracts a `Natural` from a `Limb` in place, taking the `Natural` by value, returning zero
    /// if the `Natural` is greater than the `Limb`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSubAssign, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = 123;
    ///     x.saturating_sub_assign(Natural::from(123u32));
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = 123;
    ///     x.saturating_sub_assign(Natural::ZERO);
    ///     assert_eq!(x, 123);
    ///
    ///     let mut x = 456;
    ///     x.saturating_sub_assign(Natural::from(123u32));
    ///     assert_eq!(x, 333);
    ///
    ///     let mut x = 123;
    ///     x.saturating_sub_assign(Natural::from(456u32));
    ///     assert_eq!(x, 0);
    /// }
    /// ```
    fn saturating_sub_assign(&mut self, other: Natural) {
        *self = SaturatingSub::saturating_sub(*self, other);
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingSubAssign<Natural> for u32 {
    #[inline]
    fn saturating_sub_assign(&mut self, other: Natural) {
        *self = SaturatingSub::saturating_sub(*self, other);
    }
}

impl<'a> SaturatingSubAssign<&'a Natural> for Limb {
    /// Subtracts a `Natural` from a `Limb` in place, taking the `Natural` by reference, returning
    /// zero if the `Natural` is greater than the `Limb`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{SaturatingSubAssign, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = 123;
    ///     x.saturating_sub_assign(&Natural::from(123u32));
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = 123;
    ///     x.saturating_sub_assign(&Natural::ZERO);
    ///     assert_eq!(x, 123);
    ///
    ///     let mut x = 456;
    ///     x.saturating_sub_assign(&Natural::from(123u32));
    ///     assert_eq!(x, 333);
    ///
    ///     let mut x = 123;
    ///     x.saturating_sub_assign(&Natural::from(456u32));
    ///     assert_eq!(x, 0);
    /// }
    /// ```
    fn saturating_sub_assign(&mut self, other: &'a Natural) {
        *self = SaturatingSub::saturating_sub(*self, other);
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingSubAssign<&'a Natural> for u32 {
    #[inline]
    fn saturating_sub_assign(&mut self, other: &'a Natural) {
        *self = SaturatingSub::saturating_sub(*self, other);
    }
}
