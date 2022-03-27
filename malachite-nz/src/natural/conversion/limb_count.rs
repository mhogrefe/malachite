use malachite_base::num::conversion::traits::WrappingFrom;
use natural::InnerNatural::{Large, Small};
use natural::Natural;

impl Natural {
    /// Returns the number of limbs of a `Natural`. Zero has 0 limbs.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::ZERO.limb_count(), 0);
    ///     assert_eq!(Natural::from(123u32).limb_count(), 1);
    ///     assert_eq!(Natural::trillion().limb_count(), 2);
    /// }
    /// ```
    pub fn limb_count(&self) -> u64 {
        match *self {
            natural_zero!() => 0,
            Natural(Small(_)) => 1,
            Natural(Large(ref limbs)) => u64::wrapping_from(limbs.len()),
        }
    }
}
