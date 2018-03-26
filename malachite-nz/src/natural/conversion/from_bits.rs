use malachite_base::num::{BitAccess, PrimitiveInteger};
use natural::Natural;

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
/// use malachite_nz::natural::conversion::from_bits::limbs_asc_from_bits_asc;
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
pub fn limbs_asc_from_bits_asc(bits: &[bool]) -> Vec<u32> {
    if bits.is_empty() {
        return Vec::new();
    }
    let mut limb_count = bits.len() >> u32::LOG_WIDTH;
    let remainder = bits.len() & (u32::WIDTH_MASK as usize);
    if remainder != 0 {
        limb_count += 1;
    }
    let mut limbs = vec![0; limb_count];
    let mut limb_i = 0;
    let mut i = 0;
    let width = u64::from(u32::WIDTH);
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
/// use malachite_nz::natural::conversion::from_bits::limbs_asc_from_bits_desc;
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
pub fn limbs_asc_from_bits_desc(bits: &[bool]) -> Vec<u32> {
    if bits.is_empty() {
        return Vec::new();
    }
    let mut limb_count = bits.len() >> u32::LOG_WIDTH;
    let remainder = bits.len() & (u32::WIDTH_MASK as usize);
    if remainder != 0 {
        limb_count += 1;
    }
    let mut limbs = vec![0; limb_count];
    let mut limb_i = limb_count - 1;
    let width_minus_one = u64::from(u32::WIDTH) - 1;
    let mut i = if remainder == 0 {
        width_minus_one
    } else {
        remainder as u64 - 1
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

impl Natural {
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_bits_asc(&[]).to_string(), "0");
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_asc(&[true, false, false, true, false, true, true]).to_string(),
    ///     "105");
    /// ```
    pub fn from_bits_asc(bits: &[bool]) -> Natural {
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_bits_desc(&[]).to_string(), "0");
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bits_desc(&[true, true, false, true, false, false, true]).to_string(),
    ///     "105");
    /// ```
    pub fn from_bits_desc(bits: &[bool]) -> Natural {
        Natural::from_owned_limbs_asc(limbs_asc_from_bits_desc(bits))
    }
}
