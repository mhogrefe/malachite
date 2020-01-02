use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_leading_zero_limbs;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitScan;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, finds the
/// lowest index greater than or equal to `starting_index` at which the `Natural` has a `false` bit.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_scan0 from mpn/generic/scan0.c.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_false_bit;
///
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 0), 0);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 20), 20);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 31), 31);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 32), 34);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 33), 34);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 34), 34);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 35), 36);
/// assert_eq!(limbs_index_of_next_false_bit(&[0, 0b1011], 100), 100);
/// ```
pub fn limbs_index_of_next_false_bit(limbs: &[Limb], starting_index: u64) -> u64 {
    let starting_limb_index = usize::exact_from(starting_index >> Limb::LOG_WIDTH);
    if starting_limb_index >= limbs.len() {
        return starting_index;
    }
    if let Some(result) = limbs[starting_limb_index]
        .index_of_next_false_bit(starting_index & u64::from(Limb::WIDTH_MASK))
    {
        if result != u64::from(Limb::WIDTH) {
            return (u64::wrapping_from(starting_limb_index) << Limb::LOG_WIDTH) + result;
        }
    }
    if starting_limb_index == limbs.len() - 1 {
        return u64::wrapping_from(limbs.len()) << Limb::LOG_WIDTH;
    }
    let false_index = starting_limb_index
        + 1
        + limbs[starting_limb_index + 1..]
            .iter()
            .take_while(|&&y| y == Limb::MAX)
            .count();
    let mut result_offset = false_index << Limb::LOG_WIDTH;
    if false_index != limbs.len() {
        result_offset += usize::wrapping_from((!limbs[false_index]).trailing_zeros());
    }
    u64::wrapping_from(result_offset)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, finds the
/// lowest index greater than or equal to `starting_index` at which the `Natural` has a `true` bit.
/// If the starting index is too large and there are no more `true` bits above it, `None` is
/// returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_scan1 from mpn/generic/scan1.c.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_true_bit;
///
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 0), Some(32));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 20), Some(32));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 31), Some(32));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 32), Some(32));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 33), Some(33));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 34), Some(35));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 35), Some(35));
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 36), None);
/// assert_eq!(limbs_index_of_next_true_bit(&[0, 0b1011], 100), None);
/// ```
pub fn limbs_index_of_next_true_bit(limbs: &[Limb], starting_index: u64) -> Option<u64> {
    let starting_limb_index = usize::exact_from(starting_index >> Limb::LOG_WIDTH);
    if starting_limb_index >= limbs.len() {
        return None;
    }
    if let Some(result) = limbs[starting_limb_index]
        .index_of_next_true_bit(starting_index & u64::from(Limb::WIDTH_MASK))
    {
        return Some((u64::wrapping_from(starting_limb_index) << Limb::LOG_WIDTH) + result);
    }
    if starting_limb_index == limbs.len() - 1 {
        return None;
    }
    let true_index =
        starting_limb_index + 1 + limbs_leading_zero_limbs(&limbs[starting_limb_index + 1..]);
    if true_index == limbs.len() {
        None
    } else {
        let result_offset = u64::wrapping_from(true_index) << Limb::LOG_WIDTH;
        Some(
            result_offset
                .checked_add(u64::from(limbs[true_index].trailing_zeros()))
                .unwrap(),
        )
    }
}

impl<'a> BitScan for &'a Natural {
    /// Finds the lowest index greater than or equal to `starting_index` at which the `Natural` has
    /// a `false` bit. This function always returns a `Some`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitScan;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(0), Some(0));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(20), Some(20));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(31), Some(31));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(32), Some(34));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(33), Some(34));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(34), Some(34));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(35), Some(36));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_false_bit(100), Some(100));
    /// ```
    fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.index_of_next_false_bit(starting_index),
            Natural(Large(ref limbs)) => Some(limbs_index_of_next_false_bit(limbs, starting_index)),
        }
    }

    /// Finds the lowest index greater than or equal to `starting_index` at which the `Natural` has
    /// a `true` bit. If the starting index is too large and there are no more `true` bits above it,
    /// `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitScan;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(0), Some(32));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(20), Some(32));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(31), Some(32));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(32), Some(32));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(33), Some(33));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(34), Some(35));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(35), Some(35));
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(36), None);
    /// assert_eq!(Natural::from(0xb_0000_0000u64).index_of_next_true_bit(100), None);
    /// ```
    fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.index_of_next_true_bit(starting_index),
            Natural(Large(ref limbs)) => limbs_index_of_next_true_bit(limbs, starting_index),
        }
    }
}
