use std::cmp::min;
use std::cmp::Ordering;
use std::ops::Index;

use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::BitIterable;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveUnsignedBitIterator<T: PrimitiveUnsigned> {
    pub(crate) value: T,
    pub(crate) some_remaining: bool,
    // If `n` is nonzero, this mask initially points to the least-significant bit, and is left-
    // shifted by next().
    pub(crate) i_mask: T,
    // If `n` is nonzero, this mask initially points to the most-significant nonzero bit, and is
    // right-shifted by next_back().
    pub(crate) j_mask: T,
}

impl<T: PrimitiveUnsigned> Iterator for PrimitiveUnsignedBitIterator<T> {
    type Item = bool;

    /// A function to iterate through the bits of a primitive unsigned integer in ascending order
    /// (least-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0u8.bits().next(), None);
    ///
    /// // 105 = 1101001b
    /// let mut bits = 105u32.bits();
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
            let bit = self.value & self.i_mask != T::ZERO;
            if self.i_mask == self.j_mask {
                self.some_remaining = false;
            }
            self.i_mask <<= 1;
            Some(bit)
        } else {
            None
        }
    }

    /// A function that returns the length of the bits iterator; that is, the value's significant
    /// bit count. The format is (lower bound, Option<upper bound>), but in this case it's trivial
    /// to always have an exact bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0u8.bits().size_hint(), (0, Some(0)));
    /// assert_eq!(105u32.bits().size_hint(), (7, Some(7)));
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_bits = usize::exact_from(self.value.significant_bits());
        (significant_bits, Some(significant_bits))
    }
}

impl<T: PrimitiveUnsigned> DoubleEndedIterator for PrimitiveUnsignedBitIterator<T> {
    /// A function to iterate through the bits of a primitive unsigned integer in descending order
    /// (most-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0u8.bits().next_back(), None);
    ///
    /// // 105 = 1101001b
    /// let mut bits = 105u32.bits();
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
            if self.i_mask == self.j_mask {
                self.some_remaining = false;
            }
            let bit = self.value & self.j_mask != T::ZERO;
            self.j_mask >>= 1;
            Some(bit)
        } else {
            None
        }
    }
}

/// This allows for some optimizations, e.g. when collecting into a `Vec`.
impl<T: PrimitiveUnsigned> ExactSizeIterator for PrimitiveUnsignedBitIterator<T> {}

impl<T: PrimitiveUnsigned> Index<u64> for PrimitiveUnsignedBitIterator<T> {
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
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0u8.bits()[0], false);
    ///
    /// // 105 = 1101001b
    /// let bits = 105u32.bits();
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
        if self.value.get_bit(index) {
            &true
        } else {
            &false
        }
    }
}

pub fn _bits_unsigned<T: PrimitiveUnsigned>(x: T) -> PrimitiveUnsignedBitIterator<T> {
    let significant_bits = x.significant_bits();
    PrimitiveUnsignedBitIterator {
        value: x,
        some_remaining: significant_bits != 0,
        i_mask: T::ONE,
        j_mask: T::power_of_two(significant_bits.saturating_sub(1)),
    }
}

macro_rules! impl_bit_iterable_unsigned {
    ($t:ident) => {
        impl BitIterable for $t {
            type BitIterator = PrimitiveUnsignedBitIterator<$t>;

            /// Returns a double-ended iterator over the bits of a primitive unsigned integer. The
            /// forward order is ascending, so that less significant bits appear first. There are no
            /// trailing false bits going forward, or leading falses going backward.
            ///
            /// If it's necessary to get a `Vec` of all the bits, consider using `to_bits_asc` or
            /// `to_bits_desc` instead.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitIterable;
            ///
            /// assert!(0u8.bits().next().is_none());
            /// // 105 = 1101001b
            /// assert_eq!(105u32.bits().collect::<Vec<bool>>(),
            ///     vec![true, false, false, true, false, true, true]);
            ///
            /// assert!(0u8.bits().next_back().is_none());
            /// // 105 = 1101001b
            /// assert_eq!(105u32.bits().rev().collect::<Vec<bool>>(),
            ///     vec![true, true, false, true, false, false, true]);
            /// ```
            #[inline]
            fn bits(self) -> PrimitiveUnsignedBitIterator<$t> {
                _bits_unsigned(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_iterable_unsigned);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveSignedBitIterator<T: PrimitiveSigned>(
    PrimitiveUnsignedBitIterator<T::UnsignedOfEqualWidth>,
);

impl<T: PrimitiveSigned> Iterator for PrimitiveSignedBitIterator<T> {
    type Item = bool;

    /// A function to iterate through the bits of a primitive signed integer in ascending order
    /// (least-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0i8.bits().next(), None);
    ///
    /// // -105 = 10010111 in two's complement
    /// let mut bits = (-105i32).bits();
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(false));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), Some(false));
    /// assert_eq!(bits.next(), Some(false));
    /// assert_eq!(bits.next(), Some(true));
    /// assert_eq!(bits.next(), None);
    /// ```
    fn next(&mut self) -> Option<bool> {
        self.0.next()
    }
}

impl<T: PrimitiveSigned> DoubleEndedIterator for PrimitiveSignedBitIterator<T> {
    /// A function to iterate through the bits of a primitive signed integer in descending order
    /// (most-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0i8.bits().next_back(), None);
    ///
    /// // -105 = 10010111 in two's complement
    /// let mut bits = (-105i32).bits();
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(false));
    /// assert_eq!(bits.next_back(), Some(false));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(false));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), Some(true));
    /// assert_eq!(bits.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<bool> {
        self.0.next_back()
    }
}

impl<T: PrimitiveSigned> Index<u64> for PrimitiveSignedBitIterator<T> {
    type Output = bool;

    /// A function to retrieve bits by index. The index is the power of 2 of which the bit is a
    /// coefficient. Indexing at or above the significant bit count returns false or true bits,
    /// depending on the value's sign.
    ///
    /// This is equivalent to the `get_bit` function.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0i8.bits()[0], false);
    ///
    /// // -105 = 10010111 in two's complement
    /// let bits = (-105i32).bits();
    /// assert_eq!(bits[0], true);
    /// assert_eq!(bits[1], true);
    /// assert_eq!(bits[2], true);
    /// assert_eq!(bits[3], false);
    /// assert_eq!(bits[4], true);
    /// assert_eq!(bits[5], false);
    /// assert_eq!(bits[6], false);
    /// assert_eq!(bits[7], true);
    /// assert_eq!(bits[100], true);
    /// ```
    fn index(&self, index: u64) -> &bool {
        if self.0[min(index, T::WIDTH - 1)] {
            &true
        } else {
            &false
        }
    }
}

pub fn _bits_signed<U: PrimitiveUnsigned, S>(x: S) -> PrimitiveSignedBitIterator<S>
where
    S: PrimitiveSigned<UnsignedOfEqualWidth = U>,
    U: WrappingFrom<S>,
{
    let unsigned = U::wrapping_from(x);
    let significant_bits = match x.sign() {
        Ordering::Equal => 0,
        Ordering::Greater => unsigned.significant_bits() + 1,
        Ordering::Less => (!unsigned).significant_bits() + 1,
    };
    PrimitiveSignedBitIterator(PrimitiveUnsignedBitIterator {
        value: unsigned,
        some_remaining: significant_bits != 0,
        i_mask: U::ONE,
        j_mask: U::power_of_two(significant_bits.saturating_sub(1)),
    })
}

macro_rules! impl_bit_iterable_signed {
    ($u:ident, $s:ident) => {
        impl BitIterable for $s {
            type BitIterator = PrimitiveSignedBitIterator<$s>;

            /// Returns a double-ended iterator over the bits of a primitive signed integer. The
            /// forward order is ascending, so that less significant bits appear first. There are no
            /// trailing sign bits going forward, or leading sign bits going backward.
            ///
            /// If it's necessary to get a `Vec` of all the bits, consider using `to_bits_asc` or
            /// `to_bits_desc` instead.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitIterable;
            ///
            /// assert_eq!(0i8.bits().next(), None);
            /// // 105 = 01101001b, with a leading false bit to indicate sign
            /// assert_eq!(105i32.bits().collect::<Vec<bool>>(),
            ///     vec![true, false, false, true, false, true, true, false]);
            /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
            /// assert_eq!((-105i32).bits().collect::<Vec<bool>>(),
            ///     vec![true, true, true, false, true, false, false, true]);
            ///
            /// assert_eq!(0i8.bits().next_back(), None);
            /// // 105 = 01101001b, with a leading false bit to indicate sign
            /// assert_eq!(105i32.bits().rev().collect::<Vec<bool>>(),
            ///     vec![false, true, true, false, true, false, false, true]);
            /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
            /// assert_eq!((-105i32).bits().rev().collect::<Vec<bool>>(),
            ///     vec![true, false, false, true, false, true, true, true]);
            /// ```
            #[inline]
            fn bits(self) -> PrimitiveSignedBitIterator<$s> {
                _bits_signed::<$u, $s>(self)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_bit_iterable_signed);
