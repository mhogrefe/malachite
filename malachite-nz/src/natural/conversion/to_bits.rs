use std::ops::Index;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};

use natural::conversion::to_limbs::LimbIterator;
use natural::Natural;
use platform::Limb;

/// A double-ended iterator over the bits of a `Natural`. The forward order is ascending (least-
/// significant first). The iterator does not iterate over the implicit leading false bits.
///
/// This struct also supports retrieving bits by index. This functionality is completely independent
/// of the iterator's state. Indexing the implicit leading false bits is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitIterator<'a> {
    pub(crate) significant_bits: u64,
    pub(crate) limbs: LimbIterator<'a>,
    some_remaining: bool,
    indices_are_in_same_limb: bool,
    current_limb_forward: Limb,
    current_limb_back: Limb,
    // If `n` is nonzero, this index initially points to the least-significant bit in the least-
    // significant limb, and is incremented by next().
    i: u64,
    // If `n` is nonzero, this index initially points to the most-significant nonzero bit in the
    // most-significant limb, and is decremented by next_back().
    j: u64,
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    /// A function to iterate through the bits of a `Natural` in ascending order (least-significant
    /// first).
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.bits().next(), None);
    ///
    ///     // 105 = 1101001b
    ///     let n = Natural::from(105u32);
    ///     let mut bits = n.bits();
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(false));
    ///     assert_eq!(bits.next(), Some(false));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(false));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), Some(true));
    ///     assert_eq!(bits.next(), None);
    /// }
    /// ```
    fn next(&mut self) -> Option<bool> {
        if self.some_remaining {
            let bit = self.current_limb_forward.get_bit(self.i);
            if self.indices_are_in_same_limb && self.i == self.j {
                self.some_remaining = false;
            }
            self.i += 1;
            if self.i == u64::from(Limb::WIDTH) {
                self.i = 0;
                match self.limbs.next() {
                    Some(next) => self.current_limb_forward = next,
                    None => {
                        self.current_limb_forward = self.current_limb_back;
                        self.indices_are_in_same_limb = true;
                    }
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
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.bits().size_hint(), (0, Some(0)));
    ///     assert_eq!(Natural::from(105u32).bits().size_hint(), (7, Some(7)));
    /// }
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_bits = usize::checked_from(self.significant_bits).unwrap();
        (significant_bits, Some(significant_bits))
    }
}

impl<'a> DoubleEndedIterator for BitIterator<'a> {
    /// A function to iterate through the bits of a `Natural` in descending order (most-significant
    /// first).
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.bits().next_back(), None);
    ///
    ///     // 105 = 1101001b
    ///     let n = Natural::from(105u32);
    ///     let mut bits = n.bits();
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(false));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), Some(false));
    ///     assert_eq!(bits.next_back(), Some(false));
    ///     assert_eq!(bits.next_back(), Some(true));
    ///     assert_eq!(bits.next_back(), None);
    /// }
    /// ```
    fn next_back(&mut self) -> Option<bool> {
        if self.some_remaining {
            if self.indices_are_in_same_limb && self.i == self.j {
                self.some_remaining = false;
            }
            let bit = self.current_limb_back.get_bit(self.j);
            if self.j == 0 {
                self.j = u64::from(Limb::WIDTH) - 1;
                match self.limbs.next_back() {
                    Some(next_back) => self.current_limb_back = next_back,
                    None => {
                        self.current_limb_back = self.current_limb_forward;
                        self.indices_are_in_same_limb = true;
                    }
                }
            } else {
                self.j -= 1;
            }
            Some(bit)
        } else {
            None
        }
    }
}

/// This allows for some optimizations, e.g. when collecting into a `Vec`.
impl<'a> ExactSizeIterator for BitIterator<'a> {}

impl<'a> Index<u64> for BitIterator<'a> {
    type Output = bool;

    /// A function to retrieve bits by index. The index is the power of 2<sup>32</sub> of which the
    /// limbs is a coefficient. Indexing at or above the significant bit count returns false bits.
    ///
    /// This is equivalent to the `get_bit` function.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.bits()[0], false);
    ///
    ///     // 105 = 1101001b
    ///     let n = Natural::from(105u32);
    ///     let bits = n.bits();
    ///     assert_eq!(bits[0], true);
    ///     assert_eq!(bits[1], false);
    ///     assert_eq!(bits[2], false);
    ///     assert_eq!(bits[3], true);
    ///     assert_eq!(bits[4], false);
    ///     assert_eq!(bits[5], true);
    ///     assert_eq!(bits[6], true);
    ///     assert_eq!(bits[7], false);
    ///     assert_eq!(bits[100], false);
    /// }
    /// ```
    fn index(&self, index: u64) -> &bool {
        if self.limbs.n.get_bit(index) {
            &true
        } else {
            &false
        }
    }
}

impl Natural {
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert!(Natural::ZERO.to_bits_asc().is_empty());
    ///     // 105 = 1101001b
    ///     assert_eq!(Natural::from(105u32).to_bits_asc(),
    ///         vec![true, false, false, true, false, true, true]);
    /// }
    /// ```
    pub fn to_bits_asc(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        if *self == 0 as Limb {
            return bits;
        }
        let limbs = self.limbs();
        let last_index = usize::checked_from(self.limb_count()).unwrap() - 1;
        let mut last = limbs[last_index];
        for limb in limbs.take(last_index) {
            for i in 0..Limb::WIDTH.into() {
                bits.push(limb.get_bit(i));
            }
        }
        while last != 0 {
            bits.push(last.get_bit(0));
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert!(Natural::ZERO.to_bits_desc().is_empty());
    ///     // 105 = 1101001b
    ///     assert_eq!(Natural::from(105u32).to_bits_desc(),
    ///         vec![true, true, false, true, false, false, true]);
    /// }
    /// ```
    pub fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        if *self == 0 as Limb {
            return bits;
        }
        let mut first = true;
        for limb in self.limbs().rev() {
            let mut i = u64::from(if first {
                first = false;
                Limb::WIDTH - limb.leading_zeros() - 1
            } else {
                Limb::WIDTH - 1
            });
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
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert!(Natural::ZERO.bits().next().is_none());
    ///     // 105 = 1101001b
    ///     assert_eq!(Natural::from(105u32).bits().collect::<Vec<bool>>(),
    ///         vec![true, false, false, true, false, true, true]);
    ///
    ///     assert!(Natural::ZERO.bits().next_back().is_none());
    ///     // 105 = 1101001b
    ///     assert_eq!(Natural::from(105u32).bits().rev().collect::<Vec<bool>>(),
    ///         vec![true, true, false, true, false, false, true]);
    /// }
    /// ```
    pub fn bits(&self) -> BitIterator {
        let significant_bits = self.significant_bits();
        let mut bits = BitIterator {
            significant_bits,
            limbs: self.limbs(),
            some_remaining: significant_bits != 0,
            indices_are_in_same_limb: significant_bits <= u64::from(Limb::WIDTH),
            current_limb_forward: 0,
            current_limb_back: 0,
            i: 0,
            j: 0,
        };
        if let Some(next) = bits.limbs.next() {
            bits.current_limb_forward = next;
        }
        if let Some(next_back) = bits.limbs.next_back() {
            bits.current_limb_back = next_back;
        } else {
            bits.current_limb_back = bits.current_limb_forward;
        }
        let remainder = significant_bits & u64::from(Limb::WIDTH_MASK);
        if remainder != 0 {
            bits.j = remainder - 1;
        } else {
            bits.j = u64::from(Limb::WIDTH) - 1;
        }
        bits
    }
}
