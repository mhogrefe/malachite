use malachite_base::limbs::limbs_delete_left;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitBlockAccess;

use natural::arithmetic::mod_power_of_two::limbs_mod_power_of_two_in_place;
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs obtained by taking a slice of bits beginning at index `start` of the input slice and
/// ending at index `end - 1`. `start` must be less than or equal to `end`, but apart from that
/// there are no restrictions on the index values. If they index beyond the physical size of the
/// input limbs, the function interprets them as pointing to `false` bits.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_block_access::limbs_slice_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_slice_get_bits(&[0x1234_5678, 0xabcd_ef01], 16, 48), vec![0xef01_1234]);
/// assert_eq!(limbs_slice_get_bits(&[0x1234_5678, 0xabcd_ef01], 4, 16), vec![0x567]);
/// assert_eq!(
///     limbs_slice_get_bits(&[0x1234_5678, 0xabcd_ef01], 0, 100),
///     vec![0x1234_5678, 0xabcd_ef01]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_slice_get_bits(&[0x1234_5678, 0xabcd_ef01], 10, 10), empty);
/// ```
pub fn limbs_slice_get_bits(limbs: &[Limb], start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = limbs.len();
    if limb_start >= len {
        return Vec::new();
    }
    let limb_end = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    let mut result_limbs = (if limb_end >= len {
        &limbs[limb_start..]
    } else {
        &limbs[limb_start..limb_end]
    })
    .to_vec();
    let offset = u32::exact_from(start & u64::from(Limb::WIDTH_MASK));
    if offset != 0 {
        limbs_slice_shr_in_place(&mut result_limbs, offset);
    }
    limbs_mod_power_of_two_in_place(&mut result_limbs, end - start);
    result_limbs
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs obtained by taking a slice of bits beginning at index `start` of the input slice and
/// ending at index `end - 1`. `start` must be less than or equal to `end`, but apart from that
/// there are no restrictions on the index values. If they index beyond the physical size of the
/// input limbs, the function interprets them as pointing to `false` bits.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_block_access::limbs_vec_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 16, 48), vec![0xef01_1234, 0]);
/// assert_eq!(limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 4, 16), vec![0x567]);
/// assert_eq!(
///     limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 0, 100),
///     vec![0x1234_5678, 0xabcd_ef01]
/// );
/// assert_eq!(limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 10, 10), vec![0]);
/// ```
pub fn limbs_vec_get_bits(mut limbs: Vec<Limb>, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    if limb_start >= limbs.len() {
        return Vec::new();
    }
    limbs_mod_power_of_two_in_place(&mut limbs, end);
    limbs_delete_left(&mut limbs, limb_start);
    let offset = u32::exact_from(start & u64::from(Limb::WIDTH_MASK));
    if offset != 0 {
        limbs_slice_shr_in_place(&mut limbs, offset);
    }
    limbs
}

impl BitBlockAccess for Natural {
    type Bits = Natural;

    /// Extracts a block of bits whose first index is `start` and last index is `end - 1`. The input
    /// is taken by reference, and the resulting bits are returned as a `Natural`. If `end` is
    /// greater than the type's width, the high bits of the result are all 0.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `start` > `end`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(0xabcd_ef01_1234_5678u64).get_bits(16, 48),
    ///     Natural::from(0xef01_1234u32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcd_ef01_1234_5678u64).get_bits(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcd_ef01_1234_5678u64).get_bits(0, 100),
    ///     Natural::from(0xabcd_ef01_1234_5678u64)
    /// );
    /// assert_eq!(Natural::from(0xabcd_ef01_1234_5678u64).get_bits(10, 10), Natural::ZERO);
    /// ```
    fn get_bits(&self, start: u64, end: u64) -> Natural {
        match *self {
            Natural(Small(small)) => Natural(Small(small.get_bits(start, end))),
            Natural(Large(ref limbs)) => {
                let mut bits = Natural(Large(limbs_slice_get_bits(limbs, start, end)));
                bits.trim();
                bits
            }
        }
    }

    /// Extracts a block of bits whose first index is `start` and last index is `end - 1`. The input
    /// is taken by value, and the resulting bits are returned as a `Natural`. If `end` is greater
    /// than the type's width, the high bits of the result are all 0.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `start` > `end`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(0xabcd_ef01_1234_5678u64).get_bits_owned(16, 48),
    ///     Natural::from(0xef01_1234u32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcd_ef01_1234_5678u64).get_bits_owned(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     Natural::from(0xabcd_ef01_1234_5678u64).get_bits_owned(0, 100),
    ///     Natural::from(0xabcd_ef01_1234_5678u64)
    /// );
    /// assert_eq!(Natural::from(0xabcd_ef01_1234_5678u64).get_bits_owned(10, 10), Natural::ZERO);
    /// ```
    fn get_bits_owned(self, start: u64, end: u64) -> Natural {
        match self {
            Natural(Small(small)) => Natural(Small(small.get_bits(start, end))),
            Natural(Large(limbs)) => {
                let mut bits = Natural(Large(limbs_vec_get_bits(limbs, start, end)));
                bits.trim();
                bits
            }
        }
    }

    fn assign_bits(&mut self, _start: u64, _end: u64, _bits: &Natural) {
        unimplemented!();
    }
}
