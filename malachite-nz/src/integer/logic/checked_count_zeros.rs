use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::logic::traits::{CountOnes, CountZeros};

use integer::Integer;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s, as the limbs (in ascending order) of a `Natural`, counts the
/// number of zeros in the binary expansion of the negative (two's complement) of the `Natural`.
/// `limbs` cannot be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::checked_count_zeros::limbs_count_zeros_neg;
///
/// assert_eq!(limbs_count_zeros_neg(&[0, 1, 2]), 33);
/// assert_eq!(limbs_count_zeros_neg(&[1, 0xffff_ffff]), 32);
/// ```
pub fn limbs_count_zeros_neg(limbs: &[Limb]) -> u64 {
    let mut sum = 0;
    let mut nonzero_limb_seen = false;
    for &limb in limbs.iter() {
        sum += if nonzero_limb_seen {
            CountOnes::count_ones(limb)
        } else if limb == 0 {
            Limb::WIDTH
        } else {
            nonzero_limb_seen = true;
            CountZeros::count_zeros(limb.wrapping_neg())
        };
    }
    sum
}

impl Integer {
    /// Counts the number of zeros in the binary expansion of an `Integer`. If the `Integer` is
    /// non-negative, the number of zeros is infinite, so `None` is returned.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.checked_count_zeros(), None);
    /// // -105 = 10010111 in two's complement
    /// assert_eq!(Integer::from(-105).checked_count_zeros(), Some(3));
    /// assert_eq!(Integer::from(105).checked_count_zeros(), None);
    /// // -10^12 = 10001011100101011010110101111000000000000 in two's complement
    /// assert_eq!((-Integer::trillion()).checked_count_zeros(), Some(24));
    /// ```
    pub fn checked_count_zeros(&self) -> Option<u64> {
        if self.sign {
            None
        } else {
            Some(self.abs.count_zeros_neg())
        }
    }
}

impl Natural {
    fn count_zeros_neg(&self) -> u64 {
        match *self {
            Natural(Small(small)) => CountZeros::count_zeros(small.wrapping_neg()),
            Natural(Large(ref limbs)) => limbs_count_zeros_neg(limbs),
        }
    }
}
