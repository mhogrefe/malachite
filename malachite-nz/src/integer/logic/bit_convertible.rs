use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitConvertible, LeadingZeros, NotAssign};

use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use integer::Integer;
use natural::logic::bit_convertible::{limbs_asc_from_bits_asc, limbs_asc_from_bits_desc};
use natural::Natural;
use platform::Limb;

/// Given the bits of a non-negative `Integer`, in ascending order, checks whether the most
/// significant bit is `false`; if it isn't, appends an extra `false` bit. This way the `Integer`'s
/// non-negativity is preserved in its bits.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_convertible::bits_to_twos_complement_bits_non_negative;
///
/// let mut bits = vec![false, true, false];
/// bits_to_twos_complement_bits_non_negative(&mut bits);
/// assert_eq!(bits, &[false, true, false]);
///
/// let mut bits = vec![true, false, true];
/// bits_to_twos_complement_bits_non_negative(&mut bits);
/// assert_eq!(bits, &[true, false, true, false]);
/// ```
pub fn bits_to_twos_complement_bits_non_negative(bits: &mut Vec<bool>) {
    if !bits.is_empty() && *bits.last().unwrap() {
        // Sign-extend with an extra false bit to indicate a positive Integer
        bits.push(false);
    }
}

/// Given the bits of the absolute value of a negative `Integer`, in ascending order, converts the
/// bits to two's complement. Returns whether there is a carry left over from the two's complement
/// conversion process.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `bits.len()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_convertible::bits_slice_to_twos_complement_bits_negative;
///
/// let mut bits = &mut [true, false, true];
/// assert!(!bits_slice_to_twos_complement_bits_negative(bits));
/// assert_eq!(bits, &[true, true, false]);
///
/// let mut bits = &mut [false, false, false];
/// assert!(bits_slice_to_twos_complement_bits_negative(bits));
/// assert_eq!(bits, &[false, false, false]);
/// ```
pub fn bits_slice_to_twos_complement_bits_negative(bits: &mut [bool]) -> bool {
    let mut true_seen = false;
    for bit in bits.iter_mut() {
        if true_seen {
            bit.not_assign();
        } else if *bit {
            true_seen = true;
        }
    }
    !true_seen
}

/// Given the bits of the absolute value of a negative `Integer`, in ascending order, converts the
/// bits to two's complement and checks whether the most significant bit is `true`; if it isn't,
/// appends an extra `true` bit. This way the `Integer`'s negativity is preserved in its bits. The
/// bits cannot be empty or contain only `false`s.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `bits.len()`
///
/// # Panics
/// Panics if `bits` contains only `false`s.
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_convertible::bits_vec_to_twos_complement_bits_negative;
///
/// let mut bits = vec![true, false, false];
/// bits_vec_to_twos_complement_bits_negative(&mut bits);
/// assert_eq!(bits, &[true, true, true]);
///
/// let mut bits = vec![true, false, true];
/// bits_vec_to_twos_complement_bits_negative(&mut bits);
/// assert_eq!(bits, &[true, true, false, true]);
/// ```
pub fn bits_vec_to_twos_complement_bits_negative(bits: &mut Vec<bool>) {
    assert!(!bits_slice_to_twos_complement_bits_negative(bits));
    if !bits.is_empty() && !bits.last().unwrap() {
        // Sign-extend with an extra true bit to indicate a negative Integer
        bits.push(true);
    }
}

fn limbs_asc_from_negative_twos_complement_limbs_asc(mut limbs: Vec<Limb>) -> Vec<Limb> {
    {
        let most_significant_limb = limbs.last_mut().unwrap();
        let leading_zeros = LeadingZeros::leading_zeros(*most_significant_limb);
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

impl BitConvertible for Integer {
    /// Returns the bits of an `Integer` in ascending order, so that less significant bits have
    /// lower indices in the output vector. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is positive, and if
    /// the bit is `true` it is negative. There are no trailing `false` bits if the `Integer` is
    /// positive or trailing `true` bits if the `Integer` is negative, except as necessary to
    /// include the correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This method is more efficient than `to_bits_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::ZERO.to_bits_asc().is_empty());
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).to_bits_asc(),
    ///     vec![true, false, false, true, false, true, true, false]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).to_bits_asc(),
    ///     vec![true, true, true, false, true, false, false, true]
    /// );
    /// ```
    fn to_bits_asc(&self) -> Vec<bool> {
        let mut bits = self.abs.to_bits_asc();
        if self.sign {
            bits_to_twos_complement_bits_non_negative(&mut bits);
        } else {
            bits_vec_to_twos_complement_bits_negative(&mut bits);
        }
        bits
    }

    /// Returns the bits of an `Integer` in descending order, so that less significant bits have
    /// higher indices in the output vector. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is positive, and if
    /// the bit is `true` it is negative. There are no leading `false` bits if the `Integer` is
    /// non-negative or `true` bits if `Integer` is negative, except as necessary to include the
    /// correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This method is less efficient than `to_bits_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::ZERO.to_bits_desc().is_empty());
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).to_bits_desc(),
    ///     vec![false, true, true, false, true, false, false, true]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).to_bits_desc(),
    ///     vec![true, false, false, true, false, true, true, true]
    /// );
    /// ```
    fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_bits_asc();
        bits.reverse();
        bits
    }

    /// Converts a slice of bits to an `Integer`, in ascending order, so that less significant bits
    /// have lower indices in the input slice. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is non-negative,
    /// and if the bit is `true` it is negative. If `bits` is empty, zero is returned.
    ///
    /// This method is more efficient than `from_bits_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_bits_asc(&[]).to_string(), "0");
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_asc(
    ///         &[true, false, false, true, false, true, true, false]
    ///     ).to_string(),
    ///     "105"
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_asc(
    ///         &[true, true, true, false, true, false, false, true]
    ///     ).to_string(),
    ///     "-105"
    /// );
    /// ```
    fn from_bits_asc(bits: &[bool]) -> Integer {
        if bits.is_empty() {
            Integer::ZERO
        } else if !bits.last().unwrap() {
            Integer::from(Natural::from_bits_asc(bits))
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
    /// This method is less efficient than `from_bits_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_bits_desc(&[]).to_string(), "0");
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_desc(
    ///         &[false, true, true, false, true, false, false, true]
    ///     ).to_string(),
    ///     "105"
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_desc(
    ///         &[true, false, false, true, false, true, true, true]
    ///     ).to_string(),
    ///     "-105"
    /// );
    /// ```
    fn from_bits_desc(bits: &[bool]) -> Integer {
        if bits.is_empty() {
            Integer::ZERO
        } else if !bits[0] {
            Integer::from(Natural::from_bits_desc(bits))
        } else {
            let limbs = limbs_asc_from_negative_twos_complement_bits_desc(bits);
            -Natural::from_owned_limbs_asc(limbs)
        }
    }
}
