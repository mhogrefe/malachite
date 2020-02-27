use std::cmp::Ordering;

use malachite_base::limbs::limbs_leading_zero_limbs;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitScan, TrailingZeros};

use integer::Integer;
use natural::logic::bit_scan::{limbs_index_of_next_false_bit, limbs_index_of_next_true_bit};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, finds the lowest index greater than or equal to `starting_index` at which the
/// `Integer` has a `false` bit. If the starting index is too large and there are no more `false`
/// bits above it, `None` is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_false_bit_neg;
///
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 0), Some(0));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 20), Some(20));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 31), Some(31));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 32), Some(34));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 33), Some(34));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 34), Some(34));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 35), None);
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 100), None);
/// ```
///
/// This is mpz_scan0 from mpz/scan0.c, GMP 6.1.2.
pub fn limbs_index_of_next_false_bit_neg(limbs: &[Limb], mut starting_index: u64) -> Option<u64> {
    let n = limbs.len();
    let i = limbs_leading_zero_limbs(limbs);
    assert!(i < n);
    let starting_limb_index = usize::exact_from(starting_index >> Limb::LOG_WIDTH);
    if starting_limb_index >= n {
        return None;
    }
    let after_boundary_offset = (u64::wrapping_from(i) + 1) << Limb::LOG_WIDTH;
    match starting_limb_index.cmp(&i) {
        Ordering::Equal => {
            let within_limb_index = starting_index & Limb::WIDTH_MASK;
            if let Some(result) = limbs[i]
                .wrapping_neg()
                .index_of_next_false_bit(within_limb_index)
            {
                if result < Limb::WIDTH {
                    return Some((u64::wrapping_from(i) << Limb::LOG_WIDTH) + result);
                } else {
                    starting_index = 0;
                }
            }
        }
        Ordering::Less => {
            return Some(starting_index);
        }
        Ordering::Greater => {
            starting_index -= after_boundary_offset;
        }
    }
    limbs_index_of_next_true_bit(&limbs[i + 1..], starting_index)
        .map(|result| result + after_boundary_offset)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, finds the lowest index greater than or equal to `starting_index` at which the
/// `Integer` has a `true` bit.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
///
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 0), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 20), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 31), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 32), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 33), 33);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 34), 35);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 35), 35);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 36), 36);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 100), 100);
/// ```
///
/// This is mpz_scan1 from mpz/scan1.c, GMP 6.1.2.
pub fn limbs_index_of_next_true_bit_neg(limbs: &[Limb], mut starting_index: u64) -> u64 {
    let n = limbs.len();
    let i = limbs_leading_zero_limbs(limbs);
    assert!(i < n);
    let mut starting_limb_index = usize::exact_from(starting_index >> Limb::LOG_WIDTH);
    if starting_limb_index >= n {
        return starting_index;
    }
    let after_boundary_offset = (u64::wrapping_from(i) + 1) << Limb::LOG_WIDTH;
    if starting_limb_index < i {
        starting_index = u64::wrapping_from(i) << Limb::LOG_WIDTH;
        starting_limb_index = i;
    }
    if starting_limb_index == i {
        let within_limb_index = starting_index & Limb::WIDTH_MASK;
        if let Some(result) = limbs[i]
            .wrapping_neg()
            .index_of_next_true_bit(within_limb_index)
        {
            return (u64::wrapping_from(i) << Limb::LOG_WIDTH) + result;
        } else {
            starting_index = 0;
        }
    } else {
        starting_index -= after_boundary_offset;
    }
    limbs_index_of_next_false_bit(&limbs[i + 1..], starting_index) + after_boundary_offset
}

impl<'a> BitScan for &'a Integer {
    /// Finds the lowest index greater than or equal to `starting_index` at which the `Integer` has
    /// a `false` bit. If the `Integer` as negative, and the starting index is too large and there
    /// are no more `false` bits above it, `None` is returned.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(0), Some(0));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(20), Some(20));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(31), Some(31));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(32), Some(34));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(33), Some(34));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(34), Some(34));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(35), None);
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(100), None);
    /// ```
    fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
        if self.sign {
            self.abs.index_of_next_false_bit(starting_index)
        } else {
            self.abs.index_of_next_false_bit_neg(starting_index)
        }
    }

    /// Finds the lowest index greater than or equal to `starting_index` at which the `Integer` has
    /// a `true` bit. If the `Integer` is non-negative, and the starting index is too large and
    /// there are no more `true` bits above it, `None` is returned.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(0), Some(32));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(20), Some(32));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(31), Some(32));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(32), Some(32));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(33), Some(33));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(34), Some(35));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(35), Some(35));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(36), Some(36));
    /// assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(100), Some(100));
    /// ```
    fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
        if self.sign {
            self.abs.index_of_next_true_bit(starting_index)
        } else {
            Some(self.abs.index_of_next_true_bit_neg(starting_index))
        }
    }
}

impl Natural {
    // self != 0
    fn index_of_next_false_bit_neg(&self, starting_index: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => {
                if starting_index >= Limb::WIDTH {
                    None
                } else {
                    let index =
                        TrailingZeros::trailing_zeros((small - 1) & !((1 << starting_index) - 1));
                    if index == Limb::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
            Natural(Large(ref limbs)) => limbs_index_of_next_false_bit_neg(limbs, starting_index),
        }
    }

    // self != 0
    fn index_of_next_true_bit_neg(&self, starting_index: u64) -> u64 {
        match *self {
            Natural(Small(small)) => {
                if starting_index >= Limb::WIDTH {
                    starting_index
                } else {
                    TrailingZeros::trailing_zeros(!((small - 1) | ((1 << starting_index) - 1)))
                }
            }
            Natural(Large(ref limbs)) => limbs_index_of_next_true_bit_neg(limbs, starting_index),
        }
    }
}
