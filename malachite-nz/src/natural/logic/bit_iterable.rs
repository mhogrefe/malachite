use std::ops::Index;

use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitIterable, SignificantBits};

use natural::conversion::to_limbs::LimbIterator;
use natural::Natural;
use platform::Limb;

/// A double-ended iterator over the bits of a `Natural`. The forward order is ascending (least-
/// significant first). The iterator does not iterate over the implicit leading false bits.
///
/// This struct also supports retrieving bits by index. This functionality is completely independent
/// of the iterator's state. Indexing the implicit leading false bits is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NaturalBitIterator<'a> {
    pub(crate) significant_bits: u64,
    pub(crate) limbs: LimbIterator<'a>,
    some_remaining: bool,
    indices_are_in_same_limb: bool,
    current_limb_forward: Limb,
    current_limb_back: Limb,
    // If `n` is nonzero, this mask initially points to the least-significant bit, and is left-
    // shifted by next().
    i_mask: Limb,
    // If `n` is nonzero, this mask initially points to the most-significant nonzero bit, and is
    // right-shifted by next_back().
    j_mask: Limb,
}

impl<'a> Iterator for NaturalBitIterator<'a> {
    type Item = bool;

    /// A function to iterate through the bits of a `Natural` in ascending order (least-significant
    /// first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.bits().next(), None);
    ///
    /// // 105 = 1101001b
    /// let n = Natural::from(105u32);
    /// let mut bits = n.bits();
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(false));
    /// assert_eq!(bits.next(), Some(false));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(false));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), None);
    /// ```
    fn next(&mut self) -> Option<bool> {
        if self.some_remaining {
            let bit = self.current_limb_forward & self.i_mask != 0;
            if self.indices_are_in_same_limb && self.i_mask == self.j_mask {
                self.some_remaining = false;
            }
            self.i_mask <<= 1;
            if self.i_mask == 0 {
                self.i_mask = 1;
                if let Some(next) = self.limbs.next() {
                    self.current_limb_forward = next;
                } else {
                    self.current_limb_forward = self.current_limb_back;
                    self.indices_are_in_same_limb = true;
                }
            }
            Some(bit)
        } else {
            None
        }
    }

    /// A function that returns the length of the bits iterator; that is, the `Natural`'s
    /// significant bit count. The format is (lower bound, Option<upper bound>), but in this case
    /// it's trivial to always have an exact bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.bits().size_hint(), (0, Some(0)));
    /// assert_eq!(Natural::from(105u32).bits().size_hint(), (7, Some(7)));
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_bits = usize::exact_from(self.significant_bits);
        (significant_bits, Some(significant_bits))
    }
}

impl<'a> DoubleEndedIterator for NaturalBitIterator<'a> {
    /// A function to iterate through the bits of a `Natural` in descending order (most-significant
    /// first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.bits().next_back(), None);
    ///
    /// // 105 = 1101001b
    /// let n = Natural::from(105u32);
    /// let mut bits = n.bits();
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(false));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(false));
    /// assert_eq!(bits.next_back(), Some(false));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<bool> {
        if self.some_remaining {
            if self.indices_are_in_same_limb && self.i_mask == self.j_mask {
                self.some_remaining = false;
            }
            let bit = self.current_limb_back & self.j_mask != 0;
            self.j_mask >>= 1;
            if self.j_mask == 0 {
                self.j_mask = Limb::power_of_two(Limb::WIDTH - 1);
                if let Some(next_back) = self.limbs.next_back() {
                    self.current_limb_back = next_back;
                } else {
                    self.current_limb_back = self.current_limb_forward;
                    self.indices_are_in_same_limb = true;
                }
            }
            Some(bit)
        } else {
            None
        }
    }
}

/// This allows for some optimizations, e.g. when collecting into a `Vec`.
impl<'a> ExactSizeIterator for NaturalBitIterator<'a> {}

impl<'a> Index<u64> for NaturalBitIterator<'a> {
    type Output = bool;

    /// A function to retrieve bits by index. The index is the power of 2 of which the bit is a
    /// coefficient. Indexing at or above the significant bit count returns false bits.
    ///
    /// This is equivalent to the `get_bit` function.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.bits()[0], false);
    ///
    /// // 105 = 1101001b
    /// let n = Natural::from(105u32);
    /// let bits = n.bits();
    /// assert_eq!(bits[0], true);
    /// assert_eq!(bits[1], false);
    /// assert_eq!(bits[2], false);
    /// assert_eq!(bits[3], true);
    /// assert_eq!(bits[4], false);
    /// assert_eq!(bits[5], true);
    /// assert_eq!(bits[6], true);
    /// assert_eq!(bits[7], false);
    /// assert_eq!(bits[100], false);
    /// ```
    fn index(&self, index: u64) -> &bool {
        if self.limbs.n.get_bit(index) {
            &true
        } else {
            &false
        }
    }
}

impl<'a> BitIterable for &'a Natural {
    type BitIterator = NaturalBitIterator<'a>;

    /// Returns a double-ended iterator over the bits of a `Natural`. The forward order is
    /// ascending, so that less significant bits appear first. There are no trailing false bits
    /// going forward, or leading falses going backward.
    ///
    /// If it's necessary to get a `Vec` of all the bits, consider using `to_bits_asc` or
    /// `to_limbs_desc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::BitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.bits().next().is_none());
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).bits().collect::<Vec<bool>>(),
    ///     vec![true, false, false, true, false, true, true]);
    ///
    /// assert!(Natural::ZERO.bits().next_back().is_none());
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).bits().rev().collect::<Vec<bool>>(),
    ///     vec![true, true, false, true, false, false, true]);
    /// ```
    fn bits(self) -> NaturalBitIterator<'a> {
        let significant_bits = self.significant_bits();
        let remainder = significant_bits & Limb::WIDTH_MASK;
        let mut bits = NaturalBitIterator {
            significant_bits,
            limbs: self.limbs(),
            some_remaining: significant_bits != 0,
            indices_are_in_same_limb: significant_bits <= Limb::WIDTH,
            current_limb_forward: 0,
            current_limb_back: 0,
            i_mask: 1,
            j_mask: if remainder != 0 {
                Limb::power_of_two(remainder - 1)
            } else {
                Limb::power_of_two(Limb::WIDTH - 1)
            },
        };
        if let Some(next) = bits.limbs.next() {
            bits.current_limb_forward = next;
        }
        if let Some(next_back) = bits.limbs.next_back() {
            bits.current_limb_back = next_back;
        } else {
            bits.current_limb_back = bits.current_limb_forward;
        }
        bits
    }
}
