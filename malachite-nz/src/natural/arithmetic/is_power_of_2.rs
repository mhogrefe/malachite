use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::slices::slice_test_zero;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is an integer power of 2.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
///
/// assert_eq!(limbs_is_power_of_2(&[3]), false);
/// assert_eq!(limbs_is_power_of_2(&[0, 0b1000]), true);
/// assert_eq!(limbs_is_power_of_2(&[1, 0b1000]), false);
/// assert_eq!(limbs_is_power_of_2(&[0, 0b1010]), false);
/// ```
pub fn limbs_is_power_of_2(xs: &[Limb]) -> bool {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    slice_test_zero(xs_init) && xs_last.is_power_of_2()
}

impl IsPowerOf2 for Natural {
    /// Determines whether a `Natural` is an integer power of 2.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::arithmetic::traits::IsPowerOf2;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.is_power_of_2(), false);
    /// assert_eq!(Natural::from(123u32).is_power_of_2(), false);
    /// assert_eq!(Natural::from(0x80u32).is_power_of_2(), true);
    /// assert_eq!(Natural::trillion().is_power_of_2(), false);
    /// assert_eq!(Natural::from_str("1099511627776").unwrap().is_power_of_2(), true);
    /// ```
    fn is_power_of_2(&self) -> bool {
        match *self {
            Natural(Small(small)) => small.is_power_of_2(),
            Natural(Large(ref limbs)) => limbs_is_power_of_2(limbs),
        }
    }
}
