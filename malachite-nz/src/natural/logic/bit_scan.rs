use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitScan, TrailingZeros};
use malachite_base::slices::slice_leading_zeros;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, finds the
/// lowest index greater than or equal to `start` at which the `Natural` has a `false` bit.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_scan0 from mpn/generic/scan0.c, GMP 6.1.2.
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
pub fn limbs_index_of_next_false_bit(xs: &[Limb], start: u64) -> u64 {
    let starting_limb_index = usize::exact_from(start >> Limb::LOG_WIDTH);
    if starting_limb_index >= xs.len() {
        return start;
    }
    if let Some(result) = xs[starting_limb_index].index_of_next_false_bit(start & Limb::WIDTH_MASK)
    {
        if result != Limb::WIDTH {
            return (u64::wrapping_from(starting_limb_index) << Limb::LOG_WIDTH) + result;
        }
    }
    if starting_limb_index == xs.len() - 1 {
        return u64::wrapping_from(xs.len()) << Limb::LOG_WIDTH;
    }
    let false_index = starting_limb_index
        + 1
        + xs[starting_limb_index + 1..]
            .iter()
            .take_while(|&&y| y == Limb::MAX)
            .count();
    let mut result_offset = false_index << Limb::LOG_WIDTH;
    if false_index != xs.len() {
        result_offset += usize::wrapping_from((!xs[false_index]).trailing_zeros());
    }
    u64::wrapping_from(result_offset)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, finds the
/// lowest index greater than or equal to `start` at which the `Natural` has a `true` bit. If the
/// starting index is too large and there are no more `true` bits above it, `None` is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_scan1 from mpn/generic/scan1.c, GMP 6.1.2.
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
pub fn limbs_index_of_next_true_bit(xs: &[Limb], start: u64) -> Option<u64> {
    let starting_limb_index = usize::exact_from(start >> Limb::LOG_WIDTH);
    if starting_limb_index >= xs.len() {
        return None;
    }
    if let Some(result) = xs[starting_limb_index].index_of_next_true_bit(start & Limb::WIDTH_MASK) {
        return Some((u64::wrapping_from(starting_limb_index) << Limb::LOG_WIDTH) + result);
    }
    if starting_limb_index == xs.len() - 1 {
        return None;
    }
    let true_index = starting_limb_index + 1 + slice_leading_zeros(&xs[starting_limb_index + 1..]);
    if true_index == xs.len() {
        None
    } else {
        let result_offset = u64::wrapping_from(true_index) << Limb::LOG_WIDTH;
        Some(
            result_offset
                .checked_add(TrailingZeros::trailing_zeros(xs[true_index]))
                .unwrap(),
        )
    }
}

impl<'a> BitScan for &'a Natural {
    /// Finds the lowest index greater than or equal to `start` at which the `Natural` has a `false`
    /// bit. This function always returns a `Some`.
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
    fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.index_of_next_false_bit(start),
            Natural(Large(ref limbs)) => Some(limbs_index_of_next_false_bit(limbs, start)),
        }
    }

    /// Finds the lowest index greater than or equal to `start` at which the `Natural` has a `true`
    /// bit. If the starting index is too large and there are no more `true` bits above it, `None`
    /// is returned.
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
    fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.index_of_next_true_bit(start),
            Natural(Large(ref limbs)) => limbs_index_of_next_true_bit(limbs, start),
        }
    }
}
