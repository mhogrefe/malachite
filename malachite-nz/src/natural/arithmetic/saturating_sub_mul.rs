use malachite_base::num::arithmetic::traits::{
    CheckedSubMul, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::num::basic::traits::Zero;

use natural::Natural;

impl SaturatingSubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by value. If b * c is greater than a, returns 0.
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
    ///     assert_eq!(Natural::from(20u32)
    ///         .saturating_sub_mul(Natural::from(3u32), Natural::from(4u32)).to_string(), "8");
    ///     assert_eq!(Natural::from(10u32).saturating_sub_mul(Natural::from(3u32),
    ///         Natural::from(4u32)).to_string(), "0");
    ///     assert_eq!(Natural::trillion().saturating_sub_mul(
    ///         Natural::from(0x1_0000u32), Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: Natural, c: Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

impl<'a> SaturatingSubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and b by value and c by reference. If b * c is greater than a, returns 0.
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
    ///     assert_eq!(Natural::from(20u32)
    ///         .saturating_sub_mul(Natural::from(3u32), &Natural::from(4u32)).to_string(), "8");
    ///     assert_eq!(Natural::from(10u32).saturating_sub_mul(Natural::from(3u32),
    ///         &Natural::from(4u32)).to_string(), "0");
    ///     assert_eq!(Natural::trillion().saturating_sub_mul(
    ///         Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: Natural, c: &'a Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

impl<'a> SaturatingSubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and c by value and b by reference. If b * c is greater than a, returns 0.
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
    ///     assert_eq!(Natural::from(20u32)
    ///         .saturating_sub_mul(&Natural::from(3u32), Natural::from(4u32)).to_string(), "8");
    ///     assert_eq!(Natural::from(10u32).saturating_sub_mul(&Natural::from(3u32),
    ///         Natural::from(4u32)).to_string(), "0");
    ///     assert_eq!(Natural::trillion().saturating_sub_mul(
    ///         &Natural::from(0x1_0000u32), Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: &'a Natural, c: Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

impl<'a, 'b> SaturatingSubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` by value and b and c by reference. If b * c is greater than a, returns 0.
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
    ///     assert_eq!(Natural::from(20u32)
    ///         .saturating_sub_mul(&Natural::from(3u32), &Natural::from(4u32)).to_string(), "8");
    ///     assert_eq!(Natural::from(10u32).saturating_sub_mul(&Natural::from(3u32),
    ///         &Natural::from(4u32)).to_string(), "0");
    ///     assert_eq!(Natural::trillion().saturating_sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

impl<'a, 'b, 'c> SaturatingSubMul<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by reference. If b * c is greater than a, returns 0.
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
    ///     assert_eq!((&Natural::from(20u32))
    ///         .saturating_sub_mul(&Natural::from(3u32), &Natural::from(4u32)).to_string(), "8");
    ///     assert_eq!((&Natural::from(10u32)).saturating_sub_mul(&Natural::from(3u32),
    ///         &Natural::from(4u32)).to_string(), "0");
    ///     assert_eq!((&Natural::trillion()).saturating_sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul(self, b: &'b Natural, c: &'c Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubMulAssign<Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b and c by value. If b * c is greater than a, sets `self` to 0.
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
    ///     let mut x = Natural::from(20u32);
    ///     x.saturating_sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::from(10u32);
    ///     x.saturating_sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.saturating_sub_mul_assign(Natural::from(0x1_0000u32), Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul_assign(&mut self, b: Natural, c: Natural) {
        if self.sub_mul_assign_no_panic(b, c) {
            *self = Natural::ZERO;
        }
    }
}

impl<'a> SaturatingSubMulAssign<Natural, &'a Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b by value and c by reference. If b * c is greater than a, sets `self` to 0.
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
    ///     let mut x = Natural::from(20u32);
    ///     x.saturating_sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::from(10u32);
    ///     x.saturating_sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.saturating_sub_mul_assign(Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul_assign(&mut self, b: Natural, c: &'a Natural) {
        if self.sub_mul_assign_val_ref_no_panic(b, c) {
            *self = Natural::ZERO;
        }
    }
}

impl<'a> SaturatingSubMulAssign<&'a Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b by reference and c by value. If b * c is greater than a, sets `self` to 0.
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
    ///     let mut x = Natural::from(20u32);
    ///     x.saturating_sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::from(10u32);
    ///     x.saturating_sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.saturating_sub_mul_assign(&Natural::from(0x1_0000u32), Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul_assign(&mut self, b: &'a Natural, c: Natural) {
        if self.sub_mul_assign_ref_val_no_panic(b, c) {
            *self = Natural::ZERO;
        }
    }
}

impl<'a, 'b> SaturatingSubMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b and c by reference. If b * c is greater than a, sets `self` to 0.
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
    ///     let mut x = Natural::from(20u32);
    ///     x.saturating_sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::from(10u32);
    ///     x.saturating_sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 0);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.saturating_sub_mul_assign(&Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn saturating_sub_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        if self.sub_mul_assign_ref_ref_no_panic(b, c) {
            *self = Natural::ZERO;
        }
    }
}
