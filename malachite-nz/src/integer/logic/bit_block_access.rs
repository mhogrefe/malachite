use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros, TrailingZeros};
use malachite_base::round::RoundingMode;
use malachite_base::vecs::vec_delete_left;

use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use integer::Integer;
use natural::arithmetic::add::limbs_vec_add_limb_in_place;
use natural::arithmetic::mod_power_of_two::limbs_mod_power_of_two_in_place;
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::limbs_sub_limb_in_place;
use natural::logic::bit_block_access::limbs_assign_bits_helper;
use natural::logic::not::limbs_not_in_place;
use natural::logic::trailing_zeros::limbs_trailing_zeros;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Returns the limbs obtained by taking a slice of bits beginning at index `start` of the negative
/// of `limb` and ending at index `end - 1`. `start` must be less than or equal to `end`, but apart
/// from that there are no restrictions on the index values. If they index beyond the physical size
/// of the input limbs, the function interprets them as pointing to `true` bits. `limb` must be
/// positive.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `end * Limb::WIDTH`
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_neg_limb_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_neg_limb_get_bits(0x1234_5678, 16, 48), vec![0xffff_edcb]);
/// assert_eq!(limbs_neg_limb_get_bits(0x1234_5678, 4, 16), vec![0xa98]);
/// assert_eq!(
///     limbs_neg_limb_get_bits(0x1234_5678, 0, 100),
///     vec![0xedcb_a988, 0xffff_ffff, 0xffff_ffff, 0xf]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_neg_limb_get_bits(0x1234_5678, 10, 10), empty);
/// ```
pub fn limbs_neg_limb_get_bits(limb: Limb, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = TrailingZeros::trailing_zeros(limb);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let bit_len = end - start;
    if start >= Limb::WIDTH {
        let mut result_limbs =
            vec![
                Limb::MAX;
                usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling))
            ];
        limbs_mod_power_of_two_in_place(&mut result_limbs, bit_len);
        return result_limbs;
    }
    let mut result_limbs = vec![limb >> start];
    result_limbs.resize(usize::exact_from(end >> Limb::LOG_WIDTH) + 1, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut result_limbs);
    } else {
        limbs_not_in_place(&mut result_limbs);
    }
    limbs_mod_power_of_two_in_place(&mut result_limbs, bit_len);
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs obtained by taking a slice of bits beginning at index `start` of the negative of the
/// `Natural` and ending at index `end - 1`. `start` must be less than or equal to `end`, but apart
/// from that there are no restrictions on the index values. If they index beyond the physical size
/// of the input limbs, the function interprets them as pointing to `true` bits. The input slice
/// cannot only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`limbs.len()`, `end * Limb::WIDTH`)
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_slice_neg_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_slice_neg_get_bits(&[0x1234_5678, 0xabcd_ef01], 16, 48), vec![0x10fe_edcb]);
/// assert_eq!(limbs_slice_neg_get_bits(&[0x1234_5678, 0xabcd_ef01], 4, 16), vec![0xa98]);
/// assert_eq!(
///     limbs_slice_neg_get_bits(&[0x1234_5678, 0xabcd_ef01], 0, 100),
///     vec![0xedcb_a988, 0x5432_10fe, 0xffff_ffff, 0xf]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_slice_neg_get_bits(&[0x1234_5678, 0xabcd_ef01], 10, 10), empty);
/// ```
pub fn limbs_slice_neg_get_bits(limbs: &[Limb], start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = limbs_trailing_zeros(limbs);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = limbs.len();
    let bit_len = end - start;
    if limb_start >= len {
        let mut result_limbs =
            vec![
                Limb::MAX;
                usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling))
            ];
        limbs_mod_power_of_two_in_place(&mut result_limbs, bit_len);
        return result_limbs;
    }
    let limb_end = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    let mut result_limbs = (if limb_end >= len {
        &limbs[limb_start..]
    } else {
        &limbs[limb_start..limb_end]
    })
    .to_vec();
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut result_limbs, offset);
    }
    result_limbs.resize(limb_end - limb_start, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut result_limbs);
    } else {
        limbs_not_in_place(&mut result_limbs);
    }
    limbs_mod_power_of_two_in_place(&mut result_limbs, bit_len);
    result_limbs
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs obtained by taking a slice of bits beginning at index `start` of the negative of the
/// `Natural` and ending at index `end - 1`. `start` must be less than or equal to `end`, but apart
/// from that there are no restrictions on the index values. If they index beyond the physical size
/// of the input limbs, the function interprets them as pointing to `true` bits. The input slice
/// cannot only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`limbs.len()`, `end * Limb::WIDTH`)
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_vec_neg_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_vec_neg_get_bits(vec![0x1234_5678, 0xabcd_ef01], 16, 48), vec![0x10fe_edcb]);
/// assert_eq!(limbs_vec_neg_get_bits(vec![0x1234_5678, 0xabcd_ef01], 4, 16), vec![0xa98]);
/// assert_eq!(
///     limbs_vec_neg_get_bits(vec![0x1234_5678, 0xabcd_ef01], 0, 100),
///     vec![0xedcb_a988, 0x5432_10fe, 0xffff_ffff, 0xf]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_vec_neg_get_bits(vec![0x1234_5678, 0xabcd_ef01], 10, 10), empty);
/// ```
pub fn limbs_vec_neg_get_bits(mut limbs: Vec<Limb>, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = limbs_trailing_zeros(&limbs);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = limbs.len();
    let bit_len = end - start;
    if limb_start >= len {
        limbs = vec![
            Limb::MAX;
            usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling))
        ];
        limbs_mod_power_of_two_in_place(&mut limbs, bit_len);
        return limbs;
    }
    let limb_end = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    limbs.truncate(limb_end);
    vec_delete_left(&mut limbs, limb_start);
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut limbs, offset);
    }
    limbs.resize(limb_end - limb_start, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut limbs);
    } else {
        limbs_not_in_place(&mut limbs);
    }
    limbs_mod_power_of_two_in_place(&mut limbs, bit_len);
    limbs
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural` n, writes the
/// limbs of `bits` into the limbs of -n, starting at bit `start` of -n (inclusive) and ending at
/// bit `end` of -n (exclusive). The bit indices do not need to be aligned with any limb boundaries.
/// If `bits` has more than `end` - `start` bits, only the first `end` - `start` bits are written.
/// If `bits` has fewer than `end` - `start` bits, the remaining written bits are one. `limbs` may
/// be extended to accommodate the new bits. `start` must be smaller than `end`, and `limbs` cannot
/// only contain zeros.
///
/// Time: worst case O(max(n / 2 ^ `Limb::WIDTH`, m))
///
/// Additional memory: worst case O(n)
///
/// where n = `end` and m = `limbs.len()`
///
/// # Panics
/// Panics if `start` >= `end` or `limbs` only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_neg_assign_bits;
/// use malachite_nz::platform::Limb;
///
/// let mut limbs = vec![123];
/// limbs_neg_assign_bits(&mut limbs, 64, 128, &[456]);
/// assert_eq!(limbs, &[123, 0, 4294966839, 4294967295]);
///
/// let mut limbs = vec![123];
/// limbs_neg_assign_bits(&mut limbs, 80, 100, &[456]);
/// assert_eq!(limbs, &[123, 0, 4265017344, 15]);
///
/// let mut limbs = vec![123, 456];
/// limbs_neg_assign_bits(&mut limbs, 80, 100, &[789, 321]);
/// assert_eq!(limbs, &[123, 456, 4243193856, 15]);
/// ```
pub fn limbs_neg_assign_bits(limbs: &mut Vec<Limb>, start: u64, end: u64, bits: &[Limb]) {
    assert!(start < end);
    assert!(!limbs_sub_limb_in_place(limbs, 1));
    limbs_assign_bits_helper(limbs, start, end, bits, true);
    limbs_vec_add_limb_in_place(limbs, 1);
}

impl Natural {
    fn neg_get_bits(&self, start: u64, end: u64) -> Natural {
        let limbs = match *self {
            Natural(Small(small)) => limbs_neg_limb_get_bits(small, start, end),
            Natural(Large(ref limbs)) => limbs_slice_neg_get_bits(limbs, start, end),
        };
        let mut bits = Natural(Large(limbs));
        bits.trim();
        bits
    }

    fn neg_get_bits_owned(self, start: u64, end: u64) -> Natural {
        let limbs = match self {
            Natural(Small(small)) => limbs_neg_limb_get_bits(small, start, end),
            Natural(Large(limbs)) => limbs_vec_neg_get_bits(limbs, start, end),
        };
        let mut bits = Natural(Large(limbs));
        bits.trim();
        bits
    }

    fn neg_assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if start == end {
            return;
        }
        let bits_width = end - start;
        if bits_width <= Limb::WIDTH {
            if let Natural(Small(ref mut small_self)) = self {
                if let Natural(Small(small_bits)) = bits {
                    let small_bits = (!small_bits).mod_power_of_two(bits_width);
                    if small_bits == 0 || LeadingZeros::leading_zeros(small_bits) >= start {
                        let mut new_small_self = *small_self - 1;
                        new_small_self.assign_bits(start, end, &small_bits);
                        let (sum, overflow) = new_small_self.overflowing_add(1);
                        if !overflow {
                            *small_self = sum;
                            return;
                        }
                    }
                }
            }
        }
        let limbs = self.promote_in_place();
        match *bits {
            Natural(Small(small_bits)) => limbs_neg_assign_bits(limbs, start, end, &[small_bits]),
            Natural(Large(ref bits_limbs)) => limbs_neg_assign_bits(limbs, start, end, bits_limbs),
        }
        self.trim();
    }
}

impl BitBlockAccess for Integer {
    type Bits = Natural;

    /// Extracts a block of bits whose first index is `start` and last index is `end - 1`. The input
    /// is taken by reference, and the resulting bits are returned as a `Natural`. If `end` is
    /// greater than the type's width, the high bits of the result are all 0 if `self` is
    /// non-negative and 1 if `self` is negative.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(`self.significant_bits()`, end)
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (-Natural::from(0xabcd_ef01_1234_5678u64)).get_bits(16, 48),
    ///     Natural::from(0x10fe_edcbu32)
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcd_ef01_1234_5678u64).get_bits(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     (-Natural::from(0xabcd_ef01_1234_5678u64)).get_bits(0, 100),
    ///     Natural::from_str("1267650600215849587758112418184").unwrap()
    /// );
    /// assert_eq!(Integer::from(0xabcd_ef01_1234_5678u64).get_bits(10, 10), Natural::ZERO);
    /// ```
    fn get_bits(&self, start: u64, end: u64) -> Natural {
        if self.sign {
            self.abs.get_bits(start, end)
        } else {
            self.abs.neg_get_bits(start, end)
        }
    }

    /// Extracts a block of bits whose first index is `start` and last index is `end - 1`. The input
    /// is taken by value, and the resulting bits are returned as a `Natural`. If `end` is greater
    /// than the type's width, the high bits of the result are all 0 if `self` is non-negative and 1
    /// if `self` is negative.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(`self.significant_bits()`, end)
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (-Natural::from(0xabcd_ef01_1234_5678u64)).get_bits_owned(16, 48),
    ///     Natural::from(0x10fe_edcbu32)
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcd_ef01_1234_5678u64).get_bits_owned(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     (-Natural::from(0xabcd_ef01_1234_5678u64)).get_bits_owned(0, 100),
    ///     Natural::from_str("1267650600215849587758112418184").unwrap()
    /// );
    /// assert_eq!(Integer::from(0xabcd_ef01_1234_5678u64).get_bits_owned(10, 10), Natural::ZERO);
    /// ```
    fn get_bits_owned(self, start: u64, end: u64) -> Natural {
        if self.sign {
            self.abs.get_bits_owned(start, end)
        } else {
            self.abs.neg_get_bits_owned(start, end)
        }
    }

    /// Writes the bits of `bits` to `self`. The first index that the bits are written to in `self`
    /// is `start` and last index is `end - 1`. The bit indices do not need to be aligned with any
    /// limb boundaries. If `bits` has more than `end` - `start` bits, only the first
    /// `end` - `start` bits are written. If `bits` has fewer than `end` - `start` bits, the
    /// remaining written bits are zero or one, depending on the sign of `self`. `self` may be
    /// extended to accommodate the new bits. `start` must be less than or equal to `end`.
    ///
    /// Time: worst case O(max(n, m))
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `end`, m = `self.significant_bits()`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Integer::from(123);
    /// n.assign_bits(5, 7, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "27");
    ///
    /// let mut n = Integer::from(-123);
    /// n.assign_bits(64, 128, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "-340282366920938455033212565746503123067");
    ///
    /// let mut n = Integer::from(-123);
    /// n.assign_bits(80, 100, &Natural::from(456u32));
    /// assert_eq!(n.to_string(), "-1267098121128665515963862483067");
    /// ```
    fn assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if self.sign {
            self.abs.assign_bits(start, end, bits);
        } else {
            self.abs.neg_assign_bits(start, end, bits);
        }
    }
}
