use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use integer::Integer;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::Zero;
use natural::conversion::from_bits::{limbs_asc_from_bits_asc, limbs_asc_from_bits_desc};
use natural::Natural;
use platform::Limb;

fn limbs_asc_from_negative_twos_complement_limbs_asc(mut limbs: Vec<Limb>) -> Vec<Limb> {
    {
        let most_significant_limb = limbs.last_mut().unwrap();
        let leading_zeros = most_significant_limb.leading_zeros();
        if leading_zeros != 0 {
            *most_significant_limb |= !((1 << (Limb::WIDTH - leading_zeros)) - 1);
        }
    }
    assert!(!limbs_twos_complement_in_place(&mut limbs));
    limbs
}

fn limbs_asc_from_negative_twos_complement_bits_asc(bits: &[bool]) -> Vec<Limb> {
    limbs_asc_from_negative_twos_complement_limbs_asc(limbs_asc_from_bits_asc(bits))
}

fn limbs_asc_from_negative_twos_complement_bits_desc(bits: &[bool]) -> Vec<Limb> {
    limbs_asc_from_negative_twos_complement_limbs_asc(limbs_asc_from_bits_desc(bits))
}

impl Integer {
    /// Converts a slice of bits to an `Integer`, in ascending order, so that less significant bits
    /// have lower indices in the input slice. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is non-negative,
    /// and if the bit is `true` it is negative. If `bits` is empty, zero is returned.
    ///
    /// This method is more efficient than `from_twos_complement_bits_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_twos_complement_bits_asc(&[]).to_string(), "0");
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(Integer::from_twos_complement_bits_asc(
    ///         &[true, false, false, true, false, true, true, false]).to_string(), "105");
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(Integer::from_twos_complement_bits_asc(
    ///         &[true, true, true, false, true, false, false, true]).to_string(), "-105");

    /// ```
    pub fn from_twos_complement_bits_asc(bits: &[bool]) -> Integer {
        if bits.is_empty() {
            Integer::ZERO
        } else if !bits.last().unwrap() {
            Natural::from_bits_asc(bits).into()
        } else {
            let limbs = limbs_asc_from_negative_twos_complement_bits_asc(bits);
            -Natural::from_owned_limbs_asc(limbs)
        }
    }

    /// Converts a slice of bits to an `Integer`, in descending order, so that less significant bits
    /// have higher indices in the input slice. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is non-negative,
    /// and if the bit is `true` it is negative. If `bits` is empty, zero is returned.
    ///
    /// This method is less efficient than `from_twos_complement_bits_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_twos_complement_bits_desc(&[]).to_string(), "0");
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(Integer::from_twos_complement_bits_desc(
    ///         &[false, true, true, false, true, false, false, true]).to_string(), "105");
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(Integer::from_twos_complement_bits_desc(
    ///         &[true, false, false, true, false, true, true, true]).to_string(), "-105");
    /// ```
    pub fn from_twos_complement_bits_desc(bits: &[bool]) -> Integer {
        if bits.is_empty() {
            Integer::ZERO
        } else if !bits[0] {
            Natural::from_bits_desc(bits).into()
        } else {
            let limbs = limbs_asc_from_negative_twos_complement_bits_desc(bits);
            -Natural::from_owned_limbs_asc(limbs)
        }
    }
}
