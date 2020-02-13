use std::cmp::min;
use std::cmp::Ordering;
use std::ops::Index;

use comparison::Max;
use num::arithmetic::traits::{ModPowerOfTwo, Sign, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::{
    BitAccess, BitBlockAccess, BitConvertible, BitIterable, BitScan, CheckedHammingDistance,
    CountOnes, LeadingZeros, SignificantBits, TrailingZeros,
};
use num::logic::unsigneds::PrimitiveUnsignedBitIterator;

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

macro_rules! impl_logic_traits {
    ($t:ident, $u:ident) => {
        /// Returns the number of significant bits of a primitive signed integer; this is the
        /// integer's width minus the number of leading zeros of its absolute value.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// use malachite_base::num::logic::traits::SignificantBits;
        ///
        /// assert_eq!(0i8.significant_bits(), 0);
        /// assert_eq!((-100i64).significant_bits(), 7);
        /// ```
        impl SignificantBits for $t {
            #[inline]
            fn significant_bits(self) -> u64 {
                self.unsigned_abs().significant_bits()
            }
        }

        impl CheckedHammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `rhs`, or the number of bit flips
            /// needed to turn `self` into `rhs`. If `self` and `rhs` have opposite signs, then the
            /// number of flips would be infinite, so the result is `None`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::CheckedHammingDistance;
            ///
            /// assert_eq!(123i32.checked_hamming_distance(456), Some(6));
            /// assert_eq!(0i8.checked_hamming_distance(127), Some(7));
            /// assert_eq!(0i8.checked_hamming_distance(-1), None);
            /// ```
            #[inline]
            fn checked_hamming_distance(self, other: $t) -> Option<u64> {
                if (self >= 0) == (other >= 0) {
                    Some(CountOnes::count_ones(self ^ other))
                } else {
                    None
                }
            }
        }

        /// Provides functions for accessing and modifying the `index`th bit of a primitive signed
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// Negative integers are represented in two's complement.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::logic::traits::BitAccess;
        ///
        /// let mut x = 0i8;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -0x100i16;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, -156);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, -256);
        ///
        /// let mut x = 0i32;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -1i64;
        /// x.flip_bit(10);
        /// assert_eq!(x, -1025);
        /// x.flip_bit(10);
        /// assert_eq!(x, -1);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive signed integer, or the
            /// coefficient of 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false` means
            /// 0, `true` means 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Accessing bits beyond the type's width is allowed; those bits are false if the
            /// integer is non-negative and true if it is negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// assert_eq!(123i8.get_bit(2), false);
            /// assert_eq!(123i16.get_bit(3), true);
            /// assert_eq!(123i32.get_bit(100), false);
            /// assert_eq!((-123i8).get_bit(0), true);
            /// assert_eq!((-123i16).get_bit(1), false);
            /// assert_eq!((-123i32).get_bit(100), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(100), false);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(12), true);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(100), true);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                if index < Self::WIDTH {
                    self & (1 << index) != 0
                } else {
                    *self < 0
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Setting bits beyond the type's width is disallowed if the integer is non-negative;
            /// if it is negative, it's allowed but does nothing since those bits are already true.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self >= 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0i8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -0x100i16;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, -156);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH {
                    *self |= 1 << index;
                } else if *self >= 0 {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Clearing bits beyond the type's width is disallowed if the integer is negative; if
            /// it is non-negative, it's allowed but does nothing since those bits are already
            /// false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self < 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0x7fi8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -156i16;
            /// x.clear_bit(2);
            /// x.clear_bit(5);
            /// x.clear_bit(6);
            /// assert_eq!(x, -256);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH {
                    *self &= !(1 << index);
                } else if *self < 0 {
                    panic!(
                        "Cannot clear bit {} in negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }
        }

        impl BitScan for $t {
            /// Finds the smallest index of a `false` bit that is greater than or equal to
            /// `starting_index`.
            ///
            /// If the input is negative and starting index is greater than or equal to the type's
            /// width, the result will be `None` since there are no `false` bits past that point. If
            /// the input is non-negative, the result will be the starting index.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(0), Some(0));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(20), Some(20));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(31), Some(31));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(32), Some(34));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(33), Some(34));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(34), Some(34));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(35), None);
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(100), None);
            /// ```
            #[inline]
            fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
                if start >= Self::WIDTH - 1 {
                    if self >= 0 {
                        Some(start)
                    } else {
                        None
                    }
                } else {
                    let index = u64::from((!(self | ((1 << start) - 1))).trailing_zeros());
                    if index == $t::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
            }

            /// Finds the smallest index of a `true` bit that is greater than or equal to
            /// `starting_index`.
            ///
            /// If the input is non-negative and starting index is greater than or equal to the
            /// type's width, the result will be `None` since there are no `true` bits past that
            /// point. If the input is negative, the result will be the starting index.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(0), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(20), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(31), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(32), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(33), Some(33));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(34), Some(35));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(35), Some(35));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(36), Some(36));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(100), Some(100));
            /// ```
            #[inline]
            fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
                if start >= Self::WIDTH - 1 {
                    if self >= 0 {
                        None
                    } else {
                        Some(start)
                    }
                } else {
                    let index = TrailingZeros::trailing_zeros(self & !((1 << start) - 1));
                    if index == $t::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
        }

        impl BitBlockAccess for $t {
            type Bits = $u;

            /// Extracts a block of bits whose first index is `start` and last index is `end - 1`.
            /// The type of the block of bits is the unsigned version of the input type. If `end` is
            /// greater than the type's width, the high bits of the result are all 0, or all 1,
            /// depending on the input value's sign; and if the input is negative and `end - start`
            /// is greater than the type's width, the function panics.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `start < end` or `self < 0 && end - start > $t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitBlockAccess;
            ///
            /// assert_eq!((-0x5433i16).get_bits(4, 8), 0xc);
            /// assert_eq!((-0x5433i16).get_bits(5, 9), 14);
            /// assert_eq!((-0x5433i16).get_bits(5, 5), 0);
            /// assert_eq!((-0x5433i16).get_bits(100, 104), 0xf);
            /// ```
            fn get_bits(&self, start: u64, end: u64) -> Self::Bits {
                assert!(start <= end);
                (if start >= $t::WIDTH {
                    if *self >= 0 {
                        0
                    } else {
                        -1
                    }
                } else {
                    self >> start
                })
                .mod_power_of_two(end - start)
            }

            /// Assigns the least-significant `end - start` bits of `bits` to bits `start`
            /// (inclusive) through `end` (exclusive) of `self`. The type of the block of bits is
            /// the unsigned version of the input type. If `bits` has fewer bits than `end - start`,
            /// the high bits are interpreted as 0 or 1, depending on the sign of `self`. If `end`
            /// is greater than the type's width, the high bits of `bits` must be 0 or 1, depending
            /// on the sign of `self`.
            ///
            /// The sign of `self` remains unchanged, since only a finite number of bits are changed
            /// and the sign is determined by the implied infinite prefix of bits.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `start < end`, or if `end >= $t::WIDTH` and bits `$t::WIDTH - start`
            /// through `end - start` of `bits` are not equal to the original sign bit of `self`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitBlockAccess;
            ///
            /// let mut x = 0x2b5di16;
            /// x.assign_bits(4, 8, &0xc);
            /// assert_eq!(x, 0x2bcd);
            ///
            /// let mut x = -0x5413i16;
            /// x.assign_bits(4, 8, &0xc);
            /// assert_eq!(x, -0x5433);
            ///
            /// let mut x = -0x5433i16;
            /// x.assign_bits(100, 104, &0xf);
            /// assert_eq!(x, -0x5433);
            /// ```
            fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits) {
                assert!(start <= end);
                if *self >= 0 {
                    let mut abs_self = self.unsigned_abs();
                    abs_self.assign_bits(start, end, bits);
                    if abs_self.get_highest_bit() {
                        panic!("Result exceeds width of output type");
                    }
                    *self = $t::wrapping_from(abs_self);
                } else {
                    let width = $t::WIDTH - 1;
                    let bits_width = end - start;
                    let bits = bits.mod_power_of_two(bits_width);
                    let max = Self::Bits::MAX;
                    if bits_width > width + 1 {
                        panic!("Result exceeds width of output type");
                    } else if start >= width {
                        if bits != max.mod_power_of_two(bits_width) {
                            panic!("Result exceeds width of output type");
                        }
                    } else {
                        let lower_width = width - start;
                        if end > width && bits >> lower_width != max.mod_power_of_two(end - width) {
                            panic!("Result exceeds width of output type");
                        } else {
                            *self &= $t::wrapping_from(
                                !(max.mod_power_of_two(min(bits_width, lower_width)) << start),
                            );
                            *self |= $t::wrapping_from(bits << start);
                        }
                    }
                }
            }
        }

        impl BitConvertible for $t {
            /// Returns a `Vec` containing the bits of `self` in ascending order: least- to most-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, the last bit is the sign
            /// bit: `false` if `self` is non-negative and `true` if `self` is negative.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0i8.to_bits_asc(), &[]);
            /// assert_eq!(2i16.to_bits_asc(), &[false, true, false]);
            /// assert_eq!(
            ///     (-123i32).to_bits_asc(),
            ///     &[true, false, true, false, false, false, false, true]
            /// );
            /// ```
            fn to_bits_asc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                let mut x = *self;
                if *self >= 0 {
                    while x != 0 {
                        bits.push(x.get_bit(0));
                        x >>= 1;
                    }
                    if !bits.is_empty() {
                        bits.push(false);
                    }
                } else {
                    while x != -1 {
                        bits.push(x.get_bit(0));
                        x >>= 1;
                    }
                    bits.push(true);
                }
                bits
            }

            /// Returns a `Vec` containing the bits of `self` in ascending order: most- to least-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, the first bit is the
            /// sign bit: `false` if `self` is non-negative and `true` if `self` is negative.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0i8.to_bits_desc(), &[]);
            /// assert_eq!(2i16.to_bits_desc(), &[false, true, false]);
            /// assert_eq!(
            ///     (-123i32).to_bits_desc(),
            ///     &[true, false, false, false, false, true, false, true]
            /// );
            /// ```
            fn to_bits_desc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                if *self >= 0 {
                    if *self == 0 {
                        return bits;
                    }
                    bits.push(false);
                    bits.push(true);
                    if *self == 1 {
                        return bits;
                    }
                    let mut mask = 1 << ($t::WIDTH - u64::from(self.leading_zeros()) - 2);
                    while mask != 0 {
                        bits.push(*self & mask != 0);
                        mask >>= 1;
                    }
                } else {
                    bits.push(true);
                    if *self == -1 {
                        return bits;
                    }
                    bits.push(false);
                    if *self == -2 {
                        return bits;
                    }
                    let mut mask = 1 << ($t::WIDTH - LeadingZeros::leading_zeros(!*self) - 2);
                    while mask != 0 {
                        bits.push(*self & mask != 0);
                        mask >>= 1;
                    }
                }
                bits
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(i8::from_bits_asc(&[]), 0);
            /// assert_eq!(i16::from_bits_asc(&[false, true, false]), 2);
            /// assert_eq!(
            ///     i32::from_bits_asc(&[true, false, true, false, false, false, false, true]),
            ///     -123
            /// );
            /// ```
            fn from_bits_asc(bits: &[bool]) -> $t {
                if bits.is_empty() {
                    0
                } else if !*bits.last().unwrap() {
                    $t::exact_from($u::from_bits_asc(bits))
                } else {
                    let trailing_trues = bits.iter().rev().take_while(|&&bit| bit).count();
                    let significant_bits = bits.len() - trailing_trues;
                    assert!(significant_bits < usize::exact_from($t::WIDTH));
                    let mut u: $u = !((1 << significant_bits) - 1);
                    let mut mask = 1;
                    for &bit in &bits[..significant_bits] {
                        if bit {
                            u |= mask;
                        }
                        mask <<= 1;
                    }
                    $t::wrapping_from(u)
                }
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(i8::from_bits_desc(&[]), 0);
            /// assert_eq!(i16::from_bits_desc(&[false, true, false]), 2);
            /// assert_eq!(
            ///     i32::from_bits_desc(&[true, false, false, false, false, true, false, true]),
            ///     -123
            /// );
            /// ```
            fn from_bits_desc(bits: &[bool]) -> $t {
                if bits.is_empty() {
                    0
                } else if !bits[0] {
                    $t::exact_from($u::from_bits_desc(bits))
                } else {
                    let leading_trues = bits.iter().take_while(|&&bit| bit).count();
                    let significant_bits = bits.len() - leading_trues;
                    assert!(significant_bits < usize::exact_from($t::WIDTH));
                    let mut mask: $u = 1 << significant_bits;
                    let mut u = !(mask - 1);
                    for &bit in &bits[leading_trues..] {
                        mask >>= 1;
                        if bit {
                            u |= mask;
                        }
                    }
                    $t::wrapping_from(u)
                }
            }
        }

        impl BitIterable for $t {
            type BitIterator = PrimitiveSignedBitIterator<$t>;

            /// Returns a double-ended iterator over the bits of a primitive signed integer. The
            /// forward order is ascending, so that less significant bits appear first. There are no
            /// trailing sign bits going forward, or leading sign bits going backward.
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
            fn bits(self) -> PrimitiveSignedBitIterator<$t> {
                let unsigned = $u::wrapping_from(self);
                let significant_bits = match self.sign() {
                    Ordering::Equal => 0,
                    Ordering::Greater => unsigned.significant_bits() + 1,
                    Ordering::Less => (!unsigned).significant_bits() + 1,
                };
                PrimitiveSignedBitIterator(PrimitiveUnsignedBitIterator {
                    value: unsigned,
                    some_remaining: significant_bits != 0,
                    i_mask: 1,
                    j_mask: 1 << significant_bits.saturating_sub(1),
                })
            }
        }
    };
}

impl_logic_traits!(i8, u8);
impl_logic_traits!(i16, u16);
impl_logic_traits!(i32, u32);
impl_logic_traits!(i64, u64);
impl_logic_traits!(i128, u128);
impl_logic_traits!(isize, usize);

pub fn _to_bits_asc_signed_naive<T: PrimitiveSigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    if n == T::ZERO {
        return bits;
    }
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    let last_bit = *bits.last().unwrap();
    if last_bit != (n < T::ZERO) {
        bits.push(!last_bit);
    }
    bits
}

pub fn _to_bits_desc_signed_naive<T: PrimitiveSigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    if n == T::ZERO {
        return bits;
    }
    let significant_bits = n.significant_bits();
    let last_bit = n.get_bit(significant_bits - 1);
    if last_bit != (n < T::ZERO) {
        bits.push(!last_bit);
    }
    for i in (0..significant_bits).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn _from_bits_asc_signed_naive<T: PrimitiveSigned>(bits: &[bool]) -> T {
    if bits.is_empty() {
        return T::ZERO;
    }
    let mut n;
    if *bits.last().unwrap() {
        n = T::NEGATIVE_ONE;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { None } else { Some(u64::exact_from(i)) })
        {
            n.clear_bit(i);
        }
    } else {
        n = T::ZERO;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { Some(u64::exact_from(i)) } else { None })
        {
            n.set_bit(i);
        }
    };
    n
}

pub fn _from_bits_desc_signed_naive<T: PrimitiveSigned>(bits: &[bool]) -> T {
    if bits.is_empty() {
        return T::ZERO;
    }
    let mut n;
    if bits[0] {
        n = T::NEGATIVE_ONE;
        for i in bits.iter().rev().enumerate().filter_map(|(i, &bit)| {
            if bit {
                None
            } else {
                Some(u64::exact_from(i))
            }
        }) {
            n.clear_bit(i);
        }
    } else {
        n = T::ZERO;
        for i in bits.iter().rev().enumerate().filter_map(|(i, &bit)| {
            if bit {
                Some(u64::exact_from(i))
            } else {
                None
            }
        }) {
            n.set_bit(i);
        }
    };
    n
}
