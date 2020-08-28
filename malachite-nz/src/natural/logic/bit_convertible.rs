use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, BitConvertible};
use natural::arithmetic::shr::limbs_slice_shr_in_place;
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
    bits.chunks(usize::wrapping_from(Limb::WIDTH))
        .map(Limb::from_bits_asc)
        .collect()
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
    bits.rchunks(usize::wrapping_from(Limb::WIDTH))
        .map(Limb::from_bits_desc)
        .collect()
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

    /// TODO doc
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::natural::Natural;
    /// use std::iter::empty;
    ///
    /// assert_eq!(Natural::from_bit_iterator_asc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bit_iterator_asc(
    ///         [true, false, false, true, false, true, true].iter().cloned()
    ///     ),
    ///     105
    /// );
    /// ```
    fn from_bit_iterator_asc<I: Iterator<Item = bool>>(xs: I) -> Natural {
        Natural::from_owned_limbs_asc(
            xs.chunks(usize::wrapping_from(Limb::WIDTH))
                .into_iter()
                .map(Limb::from_bit_iterator_asc)
                .collect(),
        )
    }

    /// TODO doc
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::natural::Natural;
    /// use std::iter::empty;
    ///
    /// assert_eq!(Natural::from_bit_iterator_desc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Natural::from_bit_iterator_desc(
    ///         [true, true, false, true, false, false, true].iter().cloned()
    ///     ),
    ///     105
    /// );
    /// ```
    fn from_bit_iterator_desc<I: Iterator<Item = bool>>(xs: I) -> Natural {
        let mut limbs = Vec::new();
        let mut last_width = 0;
        for chunk in &xs.chunks(usize::exact_from(Limb::WIDTH)) {
            let mut limb = 0;
            let mut i = 0;
            for bit in chunk {
                limb <<= 1;
                if bit {
                    limb |= 1;
                }
                i += 1;
            }
            last_width = i;
            limbs.push(limb);
        }
        match limbs.len() {
            0 => Natural::ZERO,
            1 => Natural::from(limbs[0]),
            _ => {
                limbs.reverse();
                if last_width != Limb::WIDTH {
                    let smallest_limb = limbs[0];
                    limbs[0] = 0;
                    limbs_slice_shr_in_place(&mut limbs, Limb::WIDTH - last_width);
                    limbs[0] |= smallest_limb;
                }
                Natural::from_owned_limbs_asc(limbs)
            }
        }
    }
}
