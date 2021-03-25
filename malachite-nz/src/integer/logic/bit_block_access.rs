use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use integer::Integer;
use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::vecs::vec_delete_left;
use natural::arithmetic::add::limbs_vec_add_limb_in_place;
use natural::arithmetic::mod_power_of_two::limbs_vec_mod_power_of_two_in_place;
use natural::arithmetic::shr::limbs_slice_shr_in_place;
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
/// of the input limbs, the function interprets them as pointing to `true` bits. `x` must be
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
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_neg_limb_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_neg_limb_get_bits(0x12345678, 16, 48), vec![0xffff_edcb]);
/// assert_eq!(limbs_neg_limb_get_bits(0x12345678, 4, 16), vec![0xa98]);
/// assert_eq!(
///     limbs_neg_limb_get_bits(0x12345678, 0, 100),
///     vec![0xedcb_a988, u32::MAX, u32::MAX, 0xf]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_neg_limb_get_bits(0x12345678, 10, 10), empty);
/// ```
pub fn limbs_neg_limb_get_bits(x: Limb, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = TrailingZeros::trailing_zeros(x);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let bit_len = end - start;
    let mut out = if start >= Limb::WIDTH {
        vec![
            Limb::MAX;
            usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling))
        ]
    } else {
        let mut out = vec![x >> start];
        out.resize(usize::exact_from(end >> Limb::LOG_WIDTH) + 1, 0);
        if trailing_zeros >= start {
            limbs_twos_complement_in_place(&mut out);
        } else {
            limbs_not_in_place(&mut out);
        }
        out
    };
    limbs_vec_mod_power_of_two_in_place(&mut out, bit_len);
    out
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
/// where n = max(`xs.len()`, `end * Limb::WIDTH`)
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_slice_neg_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_slice_neg_get_bits(&[0x12345678, 0xabcdef01], 16, 48), vec![0x10feedcb]);
/// assert_eq!(limbs_slice_neg_get_bits(&[0x12345678, 0xabcdef01], 4, 16), vec![0xa98]);
/// assert_eq!(
///     limbs_slice_neg_get_bits(&[0x12345678, 0xabcdef01], 0, 100),
///     vec![0xedcb_a988, 0x543210fe, u32::MAX, 0xf]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_slice_neg_get_bits(&[0x12345678, 0xabcdef01], 10, 10), empty);
/// ```
pub fn limbs_slice_neg_get_bits(xs: &[Limb], start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = limbs_trailing_zeros(xs);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let start_i = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = xs.len();
    let bit_len = end - start;
    if start_i >= len {
        let mut out =
            vec![
                Limb::MAX;
                usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling))
            ];
        limbs_vec_mod_power_of_two_in_place(&mut out, bit_len);
        return out;
    }
    let end_i = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    let mut out = (if end_i >= len {
        &xs[start_i..]
    } else {
        &xs[start_i..end_i]
    })
    .to_vec();
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut out, offset);
    }
    out.resize(end_i - start_i, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut out);
    } else {
        limbs_not_in_place(&mut out);
    }
    limbs_vec_mod_power_of_two_in_place(&mut out, bit_len);
    out
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
/// where n = max(`xs.len()`, `end * Limb::WIDTH`)
///
/// # Panics
/// Panics if `start` > `end`.
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_vec_neg_get_bits;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_vec_neg_get_bits(vec![0x12345678, 0xabcdef01], 16, 48), vec![0x10feedcb]);
/// assert_eq!(limbs_vec_neg_get_bits(vec![0x12345678, 0xabcdef01], 4, 16), vec![0xa98]);
/// assert_eq!(
///     limbs_vec_neg_get_bits(vec![0x12345678, 0xabcdef01], 0, 100),
///     vec![0xedcb_a988, 0x543210fe, u32::MAX, 0xf]
/// );
/// let empty: Vec<Limb> = Vec::new();
/// assert_eq!(limbs_vec_neg_get_bits(vec![0x12345678, 0xabcdef01], 10, 10), empty);
/// ```
pub fn limbs_vec_neg_get_bits(mut xs: Vec<Limb>, start: u64, end: u64) -> Vec<Limb> {
    assert!(start <= end);
    let trailing_zeros = limbs_trailing_zeros(&xs);
    if trailing_zeros >= end {
        return Vec::new();
    }
    let start_i = usize::exact_from(start >> Limb::LOG_WIDTH);
    let len = xs.len();
    let bit_len = end - start;
    if start_i >= len {
        xs = vec![
            Limb::MAX;
            usize::exact_from(bit_len.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling))
        ];
        limbs_vec_mod_power_of_two_in_place(&mut xs, bit_len);
        return xs;
    }
    let end_i = usize::exact_from(end >> Limb::LOG_WIDTH) + 1;
    xs.truncate(end_i);
    vec_delete_left(&mut xs, start_i);
    let offset = start & Limb::WIDTH_MASK;
    if offset != 0 {
        limbs_slice_shr_in_place(&mut xs, offset);
    }
    xs.resize(end_i - start_i, 0);
    if trailing_zeros >= start {
        limbs_twos_complement_in_place(&mut xs);
    } else {
        limbs_not_in_place(&mut xs);
    }
    limbs_vec_mod_power_of_two_in_place(&mut xs, bit_len);
    xs
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural` n, writes the
/// limbs of `bits` into the limbs of -n, starting at bit `start` of -n (inclusive) and ending at
/// bit `end` of -n (exclusive). The bit indices do not need to be aligned with any limb boundaries.
/// If `bits` has more than `end` - `start` bits, only the first `end` - `start` bits are written.
/// If `bits` has fewer than `end` - `start` bits, the remaining written bits are one. `xs` may be
/// extended to accommodate the new bits. `start` must be smaller than `end`, and `xs` cannot only
/// contain zeros.
///
/// Time: worst case O(max(n / 2 ^ `Limb::WIDTH`, m))
///
/// Additional memory: worst case O(n)
///
/// where n = `end` and m = `xs.len()`
///
/// # Panics
/// Panics if `start` >= `end` or `xs` only contains zeros.
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_block_access::limbs_neg_assign_bits;
/// use malachite_nz::platform::Limb;
///
/// let mut xs = vec![123];
/// limbs_neg_assign_bits(&mut xs, 64, 128, &[456]);
/// assert_eq!(xs, &[123, 0, 4294966839, u32::MAX]);
///
/// let mut xs = vec![123];
/// limbs_neg_assign_bits(&mut xs, 80, 100, &[456]);
/// assert_eq!(xs, &[123, 0, 4265017344, 15]);
///
/// let mut xs = vec![123, 456];
/// limbs_neg_assign_bits(&mut xs, 80, 100, &[789, 321]);
/// assert_eq!(xs, &[123, 456, 4243193856, 15]);
/// ```
pub fn limbs_neg_assign_bits(xs: &mut Vec<Limb>, start: u64, end: u64, bits: &[Limb]) {
    assert!(start < end);
    assert!(!limbs_sub_limb_in_place(xs, 1));
    limbs_assign_bits_helper(xs, start, end, bits, true);
    limbs_vec_add_limb_in_place(xs, 1);
}

impl Natural {
    fn neg_get_bits(&self, start: u64, end: u64) -> Natural {
        Natural::from_owned_limbs_asc(match *self {
            Natural(Small(small)) => limbs_neg_limb_get_bits(small, start, end),
            Natural(Large(ref limbs)) => limbs_slice_neg_get_bits(limbs, start, end),
        })
    }

    fn neg_get_bits_owned(self, start: u64, end: u64) -> Natural {
        Natural::from_owned_limbs_asc(match self {
            Natural(Small(small)) => limbs_neg_limb_get_bits(small, start, end),
            Natural(Large(limbs)) => limbs_vec_neg_get_bits(limbs, start, end),
        })
    }

    fn neg_assign_bits(&mut self, start: u64, end: u64, bits: &Natural) {
        if start == end {
            return;
        }
        let bits_width = end - start;
        if bits_width <= Limb::WIDTH {
            if let (&mut Natural(Small(ref mut small_self)), &Natural(Small(small_bits))) =
                (&mut *self, bits)
            {
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
    /// # Examples
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
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits(16, 48),
    ///     Natural::from(0x10feedcbu32)
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcdef0112345678u64).get_bits(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits(0, 100),
    ///     Natural::from_str("1267650600215849587758112418184").unwrap()
    /// );
    /// assert_eq!(Integer::from(0xabcdef0112345678u64).get_bits(10, 10), Natural::ZERO);
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
    /// # Examples
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
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits_owned(16, 48),
    ///     Natural::from(0x10feedcbu32)
    /// );
    /// assert_eq!(
    ///     Integer::from(0xabcdef0112345678u64).get_bits_owned(4, 16),
    ///     Natural::from(0x567u32)
    /// );
    /// assert_eq!(
    ///     (-Natural::from(0xabcdef0112345678u64)).get_bits_owned(0, 100),
    ///     Natural::from_str("1267650600215849587758112418184").unwrap()
    /// );
    /// assert_eq!(Integer::from(0xabcdef0112345678u64).get_bits_owned(10, 10), Natural::ZERO);
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
    /// # Examples
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
