use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, BitConvertible, LeadingZeros};

use natural::Natural;
use platform::Limb;

/// Converts a slice of bits in ascending order to a `Vec` of limbs in ascending order. There may be
/// trailing zero limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `bits.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_convertible::limbs_asc_from_bits_asc;
///
/// assert!(limbs_asc_from_bits_asc(&[]).is_empty());
/// // 10^12 = 232 * 2^32 + 3567587328 = 1110100011010100101001010001000000000000b
/// assert_eq!(
///     limbs_asc_from_bits_asc(
///         &[false, false, false, false, false, false, false, false, false, false, false, false,
///           true, false, false, false, true, false, true, false, false, true, false, true, false,
///           false, true, false, true, false, true, true, false, false, false, true, false, true,
///           true, true]),
///     vec![3567587328, 232]);
/// ```
pub fn limbs_asc_from_bits_asc(bits: &[bool]) -> Vec<Limb> {
    if bits.is_empty() {
        return Vec::new();
    }
    let mut limb_count = bits.len() >> Limb::LOG_WIDTH;
    let remainder = bits.len() & usize::wrapping_from(Limb::WIDTH_MASK);
    if remainder != 0 {
        limb_count += 1;
    }
    let mut limbs = vec![0; limb_count];
    let mut limb_i = 0;
    let mut i = 0;
    let width = Limb::WIDTH;
    for &bit in bits {
        if bit {
            limbs[limb_i].set_bit(i);
        }
        i += 1;
        if i == width {
            i = 0;
            limb_i += 1;
        }
    }
    limbs
}

/// Converts a slice of bits in descending order to a `Vec` of limbs in ascending order. There may
/// be trailing zero limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `bits.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_convertible::limbs_asc_from_bits_desc;
///
/// assert!(limbs_asc_from_bits_desc(&[]).is_empty());
/// // 10^12 = 232 * 2^32 + 3567587328 = 1110100011010100101001010001000000000000b
/// assert_eq!(
///     limbs_asc_from_bits_desc(
///         &[true, true, true, false, true, false, false, false, true, true, false, true, false,
///           true, false, false, true, false, true, false, false, true, false, true, false, false,
///           false, true, false, false, false, false, false, false, false, false, false, false,
///           false, false]),
///     vec![3567587328, 232]);
/// ```
pub fn limbs_asc_from_bits_desc(bits: &[bool]) -> Vec<Limb> {
    if bits.is_empty() {
        return Vec::new();
    }
    let mut limb_count = bits.len() >> Limb::LOG_WIDTH;
    let remainder = bits.len() & usize::wrapping_from(Limb::WIDTH_MASK);
    if remainder != 0 {
        limb_count += 1;
    }
    let mut limbs = vec![0; limb_count];
    let mut limb_i = limb_count - 1;
    let width_minus_one = Limb::WIDTH - 1;
    let mut i = if remainder == 0 {
        width_minus_one
    } else {
        u64::wrapping_from(remainder) - 1
    };
    for &bit in bits {
        if bit {
            limbs[limb_i].set_bit(i);
        }
        if i == 0 {
            i = width_minus_one;
            if limb_i != 0 {
                limb_i -= 1;
            }
        } else {
            i -= 1;
        }
    }
    limbs
}

impl BitConvertible for Natural {
    /// Returns the bits of a `Natural` in ascending order, so that less significant bits have lower
    /// indices in the output vector. There are no trailing false bits.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_bits_asc().is_empty());
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).to_bits_asc(),
    ///     vec![true, false, false, true, false, true, true]);
    /// ```
    fn to_bits_asc(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        if *self == 0 {
            return bits;
        }
        let limbs = self.limbs();
        let last_index = usize::exact_from(self.limb_count()) - 1;
        let mut last = limbs[last_index];
        for limb in limbs.take(last_index) {
            for i in 0..Limb::WIDTH {
                bits.push(limb.get_bit(i));
            }
        }
        while last != 0 {
            bits.push(last.odd());
            last >>= 1;
        }
        bits
    }

    /// Returns the bits of a `Natural` in ascending order, so that less significant bits have lower
    /// indices in the output vector. There are no leading false bits.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_bits_desc().is_empty());
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).to_bits_desc(),
    ///     vec![true, true, false, true, false, false, true]);
    /// ```
    fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_bits_asc();
        bits.reverse();
        bits
    }

    /// Converts a slice of bits to a `Natural`, in ascending order, so that less significant bits
    /// have lower indices in the input slice.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_bits_asc(&[]).to_string(), "0");
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_asc(&[true, false, false, true, false, true, true]).to_string(),
    ///     "105");
    /// ```
    fn from_bits_asc(bits: &[bool]) -> Natural {
        Natural::from_owned_limbs_asc(limbs_asc_from_bits_asc(bits))
    }

    /// Converts a slice of bits to a `Natural`, in descending order, so that less significant bits
    /// have higher indices in the input slice.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_bits_desc(&[]).to_string(), "0");
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_desc(&[true, true, false, true, false, false, true]).to_string(),
    ///     "105");
    /// ```
    fn from_bits_desc(bits: &[bool]) -> Natural {
        Natural::from_owned_limbs_asc(limbs_asc_from_bits_desc(bits))
    }
}

impl Natural {
    pub fn _to_bits_desc_alt(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        if *self == 0 {
            return bits;
        }
        let mut first = true;
        for limb in self.limbs().rev() {
            let mut i = if first {
                first = false;
                Limb::WIDTH - LeadingZeros::leading_zeros(limb) - 1
            } else {
                Limb::WIDTH - 1
            };
            loop {
                bits.push(limb.get_bit(i));
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        bits
    }
}
