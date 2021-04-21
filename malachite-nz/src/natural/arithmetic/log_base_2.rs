use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
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
/// use malachite_nz::natural::arithmetic::log_base_2::limbs_floor_log_base_2;
///
/// assert_eq!(limbs_floor_log_base_2(&[0b11]), 1);
/// assert_eq!(limbs_floor_log_base_2(&[0, 0b1101]), 35);
/// ```
pub fn limbs_floor_log_base_2(xs: &[Limb]) -> u64 {
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
/// use malachite_nz::natural::arithmetic::log_base_2::limbs_ceiling_log_base_2;
///
/// assert_eq!(limbs_ceiling_log_base_2(&[0b11]), 2);
/// assert_eq!(limbs_ceiling_log_base_2(&[0, 0b1101]), 36);
/// ```
pub fn limbs_ceiling_log_base_2(xs: &[Limb]) -> u64 {
    let floor_log_base_2 = limbs_floor_log_base_2(xs);
    if limbs_is_power_of_2(xs) {
        floor_log_base_2
    } else {
        floor_log_base_2 + 1
    }
}

//TODO test
pub fn limbs_checked_log_base_2(xs: &[Limb]) -> Option<u64> {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    if slice_test_zero(xs_init) {
        xs_last
            .checked_log_base_2()
            .map(|log| log + (u64::exact_from(xs_init.len()) << Limb::LOG_WIDTH))
    } else {
        None
    }
}

impl<'a> FloorLogBase2 for &'a Natural {
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
    /// use malachite_base::num::arithmetic::traits::FloorLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).floor_log_base_2(), 1);
    /// assert_eq!(Natural::from(100u32).floor_log_base_2(), 6);
    /// ```
    fn floor_log_base_2(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.floor_log_base_2(),
            Natural(Large(ref limbs)) => limbs_floor_log_base_2(limbs),
        }
    }
}

impl<'a> CeilingLogBase2 for &'a Natural {
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
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).ceiling_log_base_2(), 2);
    /// assert_eq!(Natural::from(100u32).ceiling_log_base_2(), 7);
    /// ```
    fn ceiling_log_base_2(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.ceiling_log_base_2(),
            Natural(Large(ref limbs)) => limbs_ceiling_log_base_2(limbs),
        }
    }
}

//TODO test
impl<'a> CheckedLogBase2 for &'a Natural {
    fn checked_log_base_2(self) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.checked_log_base_2(),
            Natural(Large(ref limbs)) => limbs_checked_log_base_2(limbs),
        }
    }
}
