use malachite_base::num::arithmetic::traits::{CeilingLogTwo, CheckedLogTwo, FloorLogTwo};

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use natural::arithmetic::is_power_of_two::limbs_is_power_of_two;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
/// floor of the base-2 logarithm of the `Natural`.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_two::limbs_floor_log_two;
///
/// assert_eq!(limbs_floor_log_two(&[0b11]), 1);
/// assert_eq!(limbs_floor_log_two(&[0, 0b1101]), 35);
/// ```
pub fn limbs_floor_log_two(xs: &[Limb]) -> u64 {
    limbs_significant_bits(xs) - 1
}

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
/// ceiling of the base-2 logarithm of the `Natural`.
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
/// use malachite_nz::natural::arithmetic::log_two::limbs_ceiling_log_two;
///
/// assert_eq!(limbs_ceiling_log_two(&[0b11]), 2);
/// assert_eq!(limbs_ceiling_log_two(&[0, 0b1101]), 36);
/// ```
pub fn limbs_ceiling_log_two(xs: &[Limb]) -> u64 {
    let floor_log_two = limbs_floor_log_two(xs);
    if limbs_is_power_of_two(xs) {
        floor_log_two
    } else {
        floor_log_two + 1
    }
}

//TODO test
pub fn limbs_checked_log_two(xs: &[Limb]) -> Option<u64> {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    if slice_test_zero(xs_init) {
        xs_last
            .checked_log_two()
            .map(|log| log + (u64::exact_from(xs_init.len()) << Limb::LOG_WIDTH))
    } else {
        None
    }
}

impl<'a> FloorLogTwo for &'a Natural {
    /// Returns the floor of the base-2 logarithm of a positive `Natural`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorLogTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).floor_log_two(), 1);
    /// assert_eq!(Natural::from(100u32).floor_log_two(), 6);
    /// ```
    fn floor_log_two(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.floor_log_two(),
            Natural(Large(ref limbs)) => limbs_floor_log_two(limbs),
        }
    }
}

impl<'a> CeilingLogTwo for &'a Natural {
    /// Returns the ceiling of the base-2 logarithm of a positive `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingLogTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).ceiling_log_two(), 2);
    /// assert_eq!(Natural::from(100u32).ceiling_log_two(), 7);
    /// ```
    fn ceiling_log_two(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.ceiling_log_two(),
            Natural(Large(ref limbs)) => limbs_ceiling_log_two(limbs),
        }
    }
}

//TODO test
impl<'a> CheckedLogTwo for &'a Natural {
    fn checked_log_two(self) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.checked_log_two(),
            Natural(Large(ref limbs)) => limbs_checked_log_two(limbs),
        }
    }
}
