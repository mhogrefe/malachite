use malachite_base::num::arithmetic::traits::{
    CheckedSubMul, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::num::basic::traits::Zero;

use natural::Natural;
use platform::Limb;

impl SaturatingSubMul<Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by value. If b * c is greater than a, returns 0.
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
    /// use malachite_base::num::arithmetic::traits::SaturatingSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(10u32).saturating_sub_mul(Natural::from(3u32), 4).to_string(),
    ///         "0");
    ///     assert_eq!(Natural::from(15u32).saturating_sub_mul(Natural::from(3u32), 4).to_string(),
    ///         "3");
    ///     assert_eq!(Natural::trillion().saturating_sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingSubMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn saturating_sub_mul(self, b: Natural, c: u32) -> Natural {
        self.saturating_sub_mul(b, Limb::from(c))
    }
}

impl<'a> SaturatingSubMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by value and b by reference. If b * c is greater than a, returns 0.
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
    /// use malachite_base::num::arithmetic::traits::SaturatingSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(10u32).saturating_sub_mul(&Natural::from(3u32), 4).to_string(),
    ///         "0");
    ///     assert_eq!(Natural::from(15u32).saturating_sub_mul(&Natural::from(3u32), 4).to_string(),
    ///         "3");
    ///     assert_eq!(Natural::trillion().saturating_sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: &'a Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingSubMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn saturating_sub_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.saturating_sub_mul(b, Limb::from(c))
    }
}

impl<'a> SaturatingSubMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by reference and b by value. If b * c is greater than a, returns 0.
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
    /// use malachite_base::num::arithmetic::traits::SaturatingSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(10u32)).saturating_sub_mul(Natural::from(3u32), 4).to_string(),
    ///         "0");
    ///     assert_eq!(
    ///         (&Natural::from(15u32)).saturating_sub_mul(Natural::from(3u32), 4).to_string(),
    ///         "3");
    ///     assert_eq!((&Natural::trillion()).saturating_sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingSubMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn saturating_sub_mul(self, b: Natural, c: u32) -> Natural {
        self.saturating_sub_mul(b, Limb::from(c))
    }
}

impl<'a, 'b> SaturatingSubMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by reference. If b * c is greater than a, returns 0.
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
    /// use malachite_base::num::arithmetic::traits::SaturatingSubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(10u32)).saturating_sub_mul(&Natural::from(3u32), 4).to_string(),
    ///         "0");
    ///     assert_eq!(
    ///         (&Natural::from(15u32)).saturating_sub_mul(&Natural::from(3u32), 4).to_string(),
    ///         "3");
    ///     assert_eq!((&Natural::trillion()).saturating_sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: &'a Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> SaturatingSubMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn saturating_sub_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.saturating_sub_mul(b, Limb::from(c))
    }
}

impl SaturatingSubMulAssign<Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by value. If b * c is greater than a, sets `self` to 0.
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
    /// use malachite_base::num::arithmetic::traits::SaturatingSubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(10u32);
    ///     x.saturating_sub_mul_assign(Natural::from(3u32), 4);
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = Natural::from(15u32);
    ///     x.saturating_sub_mul_assign(Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.saturating_sub_mul_assign(Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul_assign(&mut self, b: Natural, c: Limb) {
        if self.sub_mul_assign_limb_no_panic(b, c) {
            *self = Natural::ZERO;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingSubMulAssign<Natural, u32> for Natural {
    #[inline]
    fn saturating_sub_mul_assign(&mut self, b: Natural, c: u32) {
        self.saturating_sub_mul_assign(b, Limb::from(c));
    }
}

impl<'a> SaturatingSubMulAssign<&'a Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by reference. If b * c is greater than a, sets `self` to 0.
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
    /// use malachite_base::num::arithmetic::traits::SaturatingSubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(10u32);
    ///     x.saturating_sub_mul_assign(&Natural::from(3u32), 4);
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = Natural::from(15u32);
    ///     x.saturating_sub_mul_assign(&Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.saturating_sub_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if self.sub_mul_assign_limb_ref_no_panic(b, c) {
            *self = Natural::ZERO;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingSubMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn saturating_sub_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.saturating_sub_mul_assign(b, Limb::from(c));
    }
}
