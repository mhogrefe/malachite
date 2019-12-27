use std::ops::Index;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, NotAssign};

use integer::Integer;
use natural::conversion::to_bits::BitIterator;
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
/// use malachite_nz::integer::conversion::to_twos_complement_bits::*;
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
/// use malachite_nz::integer::conversion::to_twos_complement_bits::*;
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
/// use malachite_nz::integer::conversion::to_twos_complement_bits::*;
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

/// A double-ended iterator over the two's complement bits of the negative of a `Natural`. The
/// forward order is ascending (least-significant first). There may be at most one implicit
/// most-significant `true` bit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NegativeBitIterator<'a> {
    pub(crate) bits: BitIterator<'a>,
    i: u64,
    j: u64,
    first_true_index: Option<u64>,
}

impl<'a> Iterator for NegativeBitIterator<'a> {
    type Item = bool;

    /// A function to iterate through the two's complement bits of the negative of a `Natural` in
    /// ascending order (least-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn next(&mut self) -> Option<bool> {
        let previous_i = self.i;
        self.bits.next().map(|bit| {
            self.i += 1;
            if let Some(first_true_index) = self.first_true_index {
                if previous_i <= first_true_index {
                    bit
                } else {
                    !bit
                }
            } else {
                if bit {
                    self.first_true_index = Some(previous_i);
                }
                bit
            }
        })
    }

    /// A function that returns the length of the negative bits iterator; that is, the `Natural`'s
    /// negative significant bit count (this is the same as its ordinary significant bit count). The
    /// format is (lower bound, Option<upper bound>), but in this case it's trivial to always have
    /// an exact bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_bits = usize::exact_from(self.bits.significant_bits);
        (significant_bits, Some(significant_bits))
    }
}

impl<'a> DoubleEndedIterator for NegativeBitIterator<'a> {
    /// A function to iterate through the two's complement bits of the negative of a `Natural` in
    /// descending order (most-significant first). This is worst-case linear since the first
    /// `next_back` call needs to determine the index of the least-significant true bit.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    fn next_back(&mut self) -> Option<bool> {
        let previous_j = self.j;
        self.bits.next_back().map(|bit| {
            if self.j != 0 {
                self.j -= 1;
            }
            if self.first_true_index.is_none() {
                let mut i = 0;
                while !self.bits[i] {
                    i += 1;
                }
                self.first_true_index = Some(i);
            }
            let first_true_index = self.first_true_index.unwrap();
            if previous_j <= first_true_index {
                bit
            } else {
                !bit
            }
        })
    }
}

trait SignExtendedBitIterator: DoubleEndedIterator<Item = bool> {
    const EXTENSION: bool;

    fn needs_sign_extension(&self) -> bool;

    fn iterate_forward(&mut self, extension_checked: &mut bool) -> Option<bool> {
        let next = self.next();
        if next.is_none() {
            if *extension_checked {
                None
            } else {
                *extension_checked = true;
                if self.needs_sign_extension() {
                    Some(Self::EXTENSION)
                } else {
                    None
                }
            }
        } else {
            next
        }
    }

    fn iterate_backward(&mut self, extension_checked: &mut bool) -> Option<bool> {
        if !*extension_checked {
            *extension_checked = true;
            if self.needs_sign_extension() {
                return Some(Self::EXTENSION);
            }
        }
        self.next_back()
    }
}

impl<'a> SignExtendedBitIterator for BitIterator<'a> {
    const EXTENSION: bool = false;

    fn needs_sign_extension(&self) -> bool {
        self[self.significant_bits - 1]
    }
}

impl<'a> SignExtendedBitIterator for NegativeBitIterator<'a> {
    const EXTENSION: bool = true;

    fn needs_sign_extension(&self) -> bool {
        let mut i = 0;
        while !self.bits[i] {
            i += 1;
        }
        let last_bit_index = self.bits.significant_bits - 1;
        let last_bit = self.bits[last_bit_index];
        let twos_complement_bit = if i == last_bit_index {
            last_bit
        } else {
            !last_bit
        };
        !twos_complement_bit
    }
}

/// A double-ended iterator over the twos-complement bits of an `Integer`. The forward order is
/// ascending (least-significant first). The most significant bit corresponds to the sign of the
/// `Integer`; `false` for non-negative and `true` for negative. This means that there may be a
/// single most-significant sign-extension bit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TwosComplementBitIterator<'a> {
    Zero,
    Positive(BitIterator<'a>, bool),
    Negative(NegativeBitIterator<'a>, bool),
}

impl<'a> Iterator for TwosComplementBitIterator<'a> {
    type Item = bool;

    /// A function to iterate through the twos-complement bits of an `Integer` in ascending order
    /// (least-significant first). The last bit may be a sign-extension bit.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_bits().next(), None);
    ///
    ///     // -105 = 10010111 in two's complement
    ///     let n = Integer::from(-105);
    ///     let mut bits = n.twos_complement_bits();
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(false));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(false));
    ///     assert_eq!(bits.next(), Some(false));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), None);
    /// }
    /// ```
    fn next(&mut self) -> Option<bool> {
        match *self {
            TwosComplementBitIterator::Zero => None,
            TwosComplementBitIterator::Positive(ref mut bits, ref mut extension_checked) => {
                bits.iterate_forward(extension_checked)
            }
            TwosComplementBitIterator::Negative(ref mut bits, ref mut extension_checked) => {
                bits.iterate_forward(extension_checked)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for TwosComplementBitIterator<'a> {
    /// A function to iterate through the twos-complement bits of an `Integer` in descending order
    /// (most-significant first). The first bit may be a sign-extension bit.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_bits().next_back(), None);
    ///
    ///     // -105 = 10010111 in two's complement
    ///     let n = Integer::from(-105);
    ///     let mut bits = n.twos_complement_bits();
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(false));
    ///     assert_eq!(bits.next_back(), Some(false));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(false));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), None);
    /// }
    /// ```
    fn next_back(&mut self) -> Option<bool> {
        match *self {
            TwosComplementBitIterator::Zero => None,
            TwosComplementBitIterator::Positive(ref mut bits, ref mut extension_checked) => {
                bits.iterate_backward(extension_checked)
            }
            TwosComplementBitIterator::Negative(ref mut bits, ref mut extension_checked) => {
                bits.iterate_backward(extension_checked)
            }
        }
    }
}

impl<'a> Index<u64> for TwosComplementBitIterator<'a> {
    type Output = bool;

    /// A function to retrieve two's complement bits by index. Indexing at or above the significant
    /// bit count is allowed.
    ///
    /// This is equivalent to the `get_bit` function.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_bits()[0], false);
    ///
    ///     // -105 = 10010111 in two's complement
    ///     let n = Integer::from(-105);
    ///     let bits = n.twos_complement_bits();
    ///     assert_eq!(bits[0], true);
    ///     assert_eq!(bits[1], true);
    ///     assert_eq!(bits[2], true);
    ///     assert_eq!(bits[3], false);
    ///     assert_eq!(bits[4], true);
    ///     assert_eq!(bits[5], false);
    ///     assert_eq!(bits[6], false);
    ///     assert_eq!(bits[7], true);
    ///     assert_eq!(bits[100], true);
    /// }
    /// ```
    fn index(&self, index: u64) -> &bool {
        let bit = match *self {
            TwosComplementBitIterator::Zero => false,
            TwosComplementBitIterator::Positive(ref bits, _) => bits.limbs.n.get_bit(index),
            TwosComplementBitIterator::Negative(ref bits, _) => {
                bits.bits.limbs.n.get_bit_neg(index)
            }
        };
        if bit {
            &true
        } else {
            &false
        }
    }
}

impl Integer {
    /// Returns the bits of an `Integer` in ascending order, so that less significant bits have
    /// lower indices in the output vector. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is positive, and if
    /// the bit is `true` it is negative. There are no trailing `false` bits if the `Integer` is
    /// positive or trailing `true` bits if the `Integer` is negative, except as necessary to
    /// include the correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This method is more efficient than `to_twos_complement_bits_desc`.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_bits_asc().is_empty());
    ///     // 105 = 01101001b, with a leading false bit to indicate sign
    ///     assert_eq!(Integer::from(105).to_twos_complement_bits_asc(),
    ///         vec![true, false, false, true, false, true, true, false]);
    ///     // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    ///     assert_eq!(Integer::from(-105).to_twos_complement_bits_asc(),
    ///         vec![true, true, true, false, true, false, false, true]);
    /// }
    /// ```
    pub fn to_twos_complement_bits_asc(&self) -> Vec<bool> {
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
    /// This method is less efficient than `to_twos_complement_bits_asc`.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_bits_desc().is_empty());
    ///     // 105 = 01101001b, with a leading false bit to indicate sign
    ///     assert_eq!(Integer::from(105).to_twos_complement_bits_desc(),
    ///         vec![false, true, true, false, true, false, false, true]);
    ///     // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    ///     assert_eq!(Integer::from(-105).to_twos_complement_bits_desc(),
    ///         vec![true, false, false, true, false, true, true, true]);
    /// }
    /// ```
    pub fn to_twos_complement_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_twos_complement_bits_asc();
        bits.reverse();
        bits
    }

    /// Returns a double-ended iterator over the twos-complement bits of an `Integer`. The forward
    /// order is ascending, so that less significant bits appear first. There may be a
    /// most-significant sign-extension bit.
    ///
    /// If it's necessary to get a `Vec` of all the twos_complement bits, consider using
    /// `to_twos_complement_bits_asc` or `to_twos_complement_bits_desc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_bits().next(), None);
    ///     // 105 = 01101001b, with a leading false bit to indicate sign
    ///     assert_eq!(Integer::from(105).twos_complement_bits().collect::<Vec<bool>>(),
    ///         vec![true, false, false, true, false, true, true, false]);
    ///     // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    ///     assert_eq!(Integer::from(-105).twos_complement_bits().collect::<Vec<bool>>(),
    ///         vec![true, true, true, false, true, false, false, true]);
    ///
    ///     assert_eq!(Integer::ZERO.twos_complement_bits().next_back(), None);
    ///     // 105 = 01101001b, with a leading false bit to indicate sign
    ///     assert_eq!(Integer::from(105).twos_complement_bits().rev().collect::<Vec<bool>>(),
    ///         vec![false, true, true, false, true, false, false, true]);
    ///     // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    ///     assert_eq!(Integer::from(-105).twos_complement_bits().rev().collect::<Vec<bool>>(),
    ///         vec![true, false, false, true, false, true, true, true]);
    /// }
    /// ```
    pub fn twos_complement_bits(&self) -> TwosComplementBitIterator {
        if *self == 0 as Limb {
            TwosComplementBitIterator::Zero
        } else if self.sign {
            TwosComplementBitIterator::Positive(self.abs.bits(), false)
        } else {
            TwosComplementBitIterator::Negative(self.abs.negative_bits(), false)
        }
    }
}

impl Natural {
    /// Returns a double-ended iterator over the two's complement bits of the negative of a
    /// `Natural`. The forward order is ascending, so that less significant bits appear first. There
    /// may be at most one trailing `true` bit going forward, or leading `true` bit going backward.
    /// The `Natural` cannot be zero.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn negative_bits(&self) -> NegativeBitIterator {
        assert_ne!(*self, 0 as Limb, "Cannot get negative bits of 0.");
        let bits = self.bits();
        NegativeBitIterator {
            bits,
            first_true_index: None,
            i: 0,
            j: bits.significant_bits - 1,
        }
    }
}
