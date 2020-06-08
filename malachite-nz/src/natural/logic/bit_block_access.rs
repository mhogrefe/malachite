use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros};
use malachite_base::rounding_mode::RoundingMode;
use malachite_base::slices::slice_set_zero;
use malachite_base::vecs::vec_delete_left;

use natural::arithmetic::mod_power_of_two::limbs_vec_mod_power_of_two_in_place;
use natural::arithmetic::shl_u::limbs_slice_shl_in_place;
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::logic::not::limbs_not_in_place;
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
pub fn limbs_slice_get_bits(xs: &[Limb], start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = xs.len();
    if limb_start >= len {
        return Vec::new();
    }
    let limb_end = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    let mut result_limbs = (if limb_end >= len {
        &xs[limb_start..]
    } else {
        &xs[limb_start..limb_end]
    })
    .to_vec();
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut result_limbs, offset);
    }
    limbs_vec_mod_power_of_two_in_place(&mut result_limbs, end - start);
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
/// assert_eq!(limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 16, 48), &[0xef01_1234, 0]);
/// assert_eq!(limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 4, 16), &[0x567]);
/// assert_eq!(
///     limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 0, 100),
///     &[0x1234_5678, 0xabcd_ef01]
/// );
/// assert_eq!(limbs_vec_get_bits(vec![0x1234_5678, 0xabcd_ef01], 10, 10), &[0]);
/// ```
pub fn limbs_vec_get_bits(mut xs: Vec<Limb>, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    if limb_start >= xs.len() {
        return Vec::new();
    }
    limbs_vec_mod_power_of_two_in_place(&mut xs, end);
    vec_delete_left(&mut xs, limb_start);
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut xs, offset);
    }
    xs
}

/// Copy values from `ys` into `xs`.
///
/// If `ys` has the same length as `xs`, the usual copy is performed.
/// If `ys` is longer than `xs`, the first `xs.len()` limbs of `ys` are copied.
/// If `ys` is shorter than `xs`, `ys` is copied and the remaining bits of `xs` are filled with
/// zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
fn copy_from_diff_len_slice(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len <= ys_len {
        xs.copy_from_slice(&ys[..xs_len]);
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
        xs_lo.copy_from_slice(ys);
        slice_set_zero(xs_hi);
    }
}

pub(crate) fn limbs_assign_bits_helper(
    xs: &mut Vec<Limb>,
    start: u64,
    end: u64,
    mut bits: &[Limb],
    invert: bool,
) {
    let start_limb = usize::exact_from(start >> Limb::LOG_WIDTH);
    let end_limb = usize::exact_from((end - 1) >> Limb::LOG_WIDTH) + 1;
    let bits_limb_width =
        usize::exact_from((end - start).shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if bits_limb_width < bits.len() {
        bits = &bits[..bits_limb_width];
    }
    let start_remainder = start & Limb::WIDTH_MASK;
    let end_remainder = end & Limb::WIDTH_MASK;
    if end_limb > xs.len() {
        // Possible inefficiency here: we might write many zeros only to delete them later.
        xs.resize(end_limb, 0);
    }
    let limbs = &mut xs[start_limb..end_limb];
    assert!(!limbs.is_empty());
    let original_first_limb = limbs[0];
    let original_last_limb = *limbs.last().unwrap();
    copy_from_diff_len_slice(limbs, bits);
    if invert {
        limbs_not_in_place(limbs);
    }
    if start_remainder != 0 {
        limbs_slice_shl_in_place(limbs, start_remainder);
        limbs[0] |= original_first_limb.mod_power_of_two(start_remainder);
    }
    if end_remainder != 0 {
        limbs.last_mut().unwrap().assign_bits(
            end_remainder,
            Limb::WIDTH,
            &(original_last_limb >> end_remainder),
        );
    }
}

/// Writes the limbs of `bits` into the limbs of `limbs`, starting at bit `start` of `limbs`
/// (inclusive) and ending at bit `end` of `limbs` (exclusive). The bit indices do not need to be
/// aligned with any limb boundaries. If `bits` has more than `end` - `start` bits, only the first
/// `end` - `start` bits are written. If `bits` has fewer than `end` - `start` bits, the remaining
/// written bits are zero. `limbs` may be extended to accommodate the new bits. `start` must be
/// smaller than `end`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `end`.
///
/// # Panics
/// Panics if `start` >= `end`.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_block_access::limbs_assign_bits;
/// use malachite_nz::platform::Limb;
///
/// let mut limbs = vec![123];
/// limbs_assign_bits(&mut limbs, 64, 128, &[456]);
/// assert_eq!(limbs, &[123, 0, 456, 0]);
///
/// let mut limbs = vec![123];
/// limbs_assign_bits(&mut limbs, 80, 100, &[456]);
/// assert_eq!(limbs, &[123, 0, 29884416, 0]);
///
/// let mut limbs = vec![123, 456];
/// limbs_assign_bits(&mut limbs, 80, 100, &[789, 321]);
/// assert_eq!(limbs, &[123, 456, 51707904, 0]);
/// ```
pub fn limbs_assign_bits(xs: &mut Vec<Limb>, start: u64, end: u64, bits: &[Limb]) {
    assert!(start < end);
    limbs_assign_bits_helper(xs, start, end, bits, false);
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
                Natural::from_owned_limbs_asc(limbs_slice_get_bits(limbs, start, end))
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
    /// assert_eq!(Natural::from(0xabcd_ef01_1234_5678u64).get_bits_owned(10, 10), 0);
    /// ```
    fn get_bits_owned(self, start: u64, end: u64) -> Natural {
        match self {
            Natural(Small(small)) => Natural(Small(small.get_bits(start, end))),
            Natural(Large(limbs)) => {
                Natural::from_owned_limbs_asc(limbs_vec_get_bits(limbs, start, end))
            }
        }
    }

    /// Writes the bits of `bits` to `self`. The first index that the bits are written to in `self`
    /// is `start` and last index is `end - 1`. The bit indices do not need to be aligned with any
    /// limb boundaries. If `bits` has more than `end` - `start` bits, only the first
    /// `end` - `start` bits are written. If `bits` has fewer than `end` - `start` bits, the
    /// remaining written bits are zero. `self` may be extended to accommodate the new bits. `start`
    /// must be less than or equal to `end`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `end`
    ///
    /// # Panics
    /// Panics if `start` > `end`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(123u32);
    /// n.assign_bits(5, 7, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "27");
    ///
    /// let mut n = Natural::from(123u32);
    /// n.assign_bits(64, 128, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "8411715297611555537019");
    ///
    /// let mut n = Natural::from(123u32);
    /// n.assign_bits(80, 100, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "551270173744270903666016379");
    /// ```
    fn assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if start == end {
            return;
        }
        if let Natural(Small(ref mut small_self)) = self {
            if let Natural(Small(small_bits)) = bits {
                let bits_width = end - start;
                let small_bits = small_bits.mod_power_of_two(bits_width);
                if small_bits == 0 || LeadingZeros::leading_zeros(small_bits) >= start {
                    small_self.assign_bits(start, end, &small_bits);
                    return;
                }
            }
        }
        let limbs = self.promote_in_place();
        match *bits {
            Natural(Small(small_bits)) => limbs_assign_bits(limbs, start, end, &[small_bits]),
            Natural(Large(ref bits_limbs)) => limbs_assign_bits(limbs, start, end, bits_limbs),
        }
        self.trim();
    }
}
