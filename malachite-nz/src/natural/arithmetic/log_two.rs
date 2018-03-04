use malachite_base::num::{CeilingLogTwo, FloorLogTwo};
use natural::arithmetic::is_power_of_two::limbs_is_power_of_two;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, returns the floor
/// of the base-2 logarithm of the `Natural`.
///
/// This function assumes that `limbs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::log_two::limbs_floor_log_two;
///
/// assert_eq!(limbs_floor_log_two(&[0b11]), 1);
/// assert_eq!(limbs_floor_log_two(&[0, 0b1101]), 35);
/// ```
pub fn limbs_floor_log_two(limbs: &[u32]) -> u64 {
    limbs_significant_bits(limbs) - 1
}

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, returns the
/// ceiling of the base-2 logarithm of the `Natural`.
///
/// This function assumes that `limbs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::log_two::limbs_ceiling_log_two;
///
/// assert_eq!(limbs_ceiling_log_two(&[0b11]), 2);
/// assert_eq!(limbs_ceiling_log_two(&[0, 0b1101]), 36);
/// ```
pub fn limbs_ceiling_log_two(limbs: &[u32]) -> u64 {
    let floor_log_two = limbs_floor_log_two(limbs);
    if limbs_is_power_of_two(limbs) {
        floor_log_two
    } else {
        floor_log_two + 1
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
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::FloorLogTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(3u32).floor_log_two(), 1);
    ///     assert_eq!(Natural::from(100u32).floor_log_two(), 6);
    /// }
    /// ```
    fn floor_log_two(self) -> u64 {
        match *self {
            Small(small) => small.floor_log_two(),
            Large(ref limbs) => limbs_floor_log_two(limbs),
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
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CeilingLogTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(3u32).ceiling_log_two(), 2);
    ///     assert_eq!(Natural::from(100u32).ceiling_log_two(), 7);
    /// }
    /// ```
    fn ceiling_log_two(self) -> u64 {
        match *self {
            Small(small) => small.ceiling_log_two(),
            Large(ref limbs) => limbs_ceiling_log_two(limbs),
        }
    }
}
