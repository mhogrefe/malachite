use std::cmp::min;

use comparison::Max;
use num::arithmetic::traits::ModPowerOfTwo;
use num::basic::integers::PrimitiveInteger;
use num::logic::traits::{BitAccess, BitBlockAccess, BitScan, HammingDistance, SignificantBits};

macro_rules! impl_logic_traits {
    ($t:ident) => {
        impl SignificantBits for $t {
            /// Returns the number of significant bits of a primitive unsigned integer; this is the
            /// integer's width minus the number of leading zeros.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::SignificantBits;
            ///
            /// assert_eq!(0u8.significant_bits(), 0);
            /// assert_eq!(100u64.significant_bits(), 7);
            /// ```
            #[inline]
            fn significant_bits(self) -> u64 {
                u64::from(Self::WIDTH - self.leading_zeros())
            }
        }

        impl HammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `rhs`, or the number of bit flips
            /// needed to turn `self` into `rhs`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::HammingDistance;
            ///
            /// assert_eq!(123u32.hamming_distance(456), 6);
            /// assert_eq!(0u8.hamming_distance(255), 8);
            /// ```
            #[inline]
            fn hamming_distance(self, other: $t) -> u64 {
                u64::from((self ^ other).count_ones())
            }
        }

        /// Provides functions for accessing and modifying the `index`th bit of a primitive unsigned
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::logic::traits::BitAccess;
        ///
        /// let mut x = 0;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = 0u64;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive unsigned integer, or the
            /// coefficient of 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false`
            /// means 0, `true` means 1.
            ///
            /// Getting bits beyond the type's width is allowed; those bits are false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// assert_eq!(123u8.get_bit(2), false);
            /// assert_eq!(123u16.get_bit(3), true);
            /// assert_eq!(123u32.get_bit(100), false);
            /// assert_eq!(1_000_000_000_000u64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000u64.get_bit(100), false);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                index < u64::from(Self::WIDTH) && *self & (1 << index) != 0
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Setting bits beyond the type's width is disallowed.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0u8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                if index < u64::from(Self::WIDTH) {
                    *self |= 1 << index;
                } else {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Clearing bits beyond the type's width is allowed; since those bits are already
            /// false, clearing them does nothing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0x7fu8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                if index < u64::from(Self::WIDTH) {
                    *self &= !(1 << index);
                }
            }
        }

        impl BitScan for $t {
            /// Finds the smallest index of a `false` bit that is greater than or equal to
            /// `starting_index`. Since `$t` is unsigned and therefore has an implicit prefix of
            /// infinitely-many zeros, this function always returns a value.
            ///
            /// Starting beyond the type's width is allowed; the result will be the starting index.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(0), Some(0));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(20), Some(20));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(31), Some(31));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(32), Some(34));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(33), Some(34));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(34), Some(34));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(35), Some(36));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(100), Some(100));
            /// ```
            #[inline]
            fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
                Some(if start >= u64::from(Self::WIDTH) {
                    start
                } else {
                    u64::from((!(self | ((1 << start) - 1))).trailing_zeros())
                })
            }

            /// Finds the smallest index of a `true` bit that is greater than or equal to
            /// `starting_index`.
            ///
            /// If the starting index is greater than or equal to the type's width, the result will
            /// be `None` since there are no `true` bits past that point.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(0), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(20), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(31), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(32), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(33), Some(33));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(34), Some(35));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(35), Some(35));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(36), None);
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(100), None);
            /// ```
            #[inline]
            fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
                if start >= u64::from(Self::WIDTH) {
                    None
                } else {
                    let index = (self & !((1 << start) - 1)).trailing_zeros();
                    if index == Self::WIDTH {
                        None
                    } else {
                        Some(u64::from(index))
                    }
                }
            }
        }

        impl BitBlockAccess for $t {
            type Bits = $t;

            /// Extracts a block of bits whose first index is `start` and last index is `end - 1`.
            /// The block of bits has the same type as the input. If `end` is greater than the
            /// type's width, the high bits of the result are all 0.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `start < end`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitBlockAccess;
            ///
            /// assert_eq!(0xabcdu16.get_bits(4, 8), 0xc);
            /// assert_eq!(0xabcdu16.get_bits(12, 100), 0xa);
            /// assert_eq!(0xabcdu16.get_bits(5, 9), 14);
            /// assert_eq!(0xabcdu16.get_bits(5, 5), 0);
            /// assert_eq!(0xabcdu16.get_bits(100, 200), 0);
            /// ```
            fn get_bits(&self, start: u64, end: u64) -> Self {
                assert!(start <= end);
                if start >= u64::from($t::WIDTH) {
                    0
                } else {
                    (self >> start).mod_power_of_two(end - start)
                }
            }

            /// Assigns the least-significant `end - start` bits of `bits` to bits `start`
            /// (inclusive) through `end` (exclusive) of `self`. The block of bits has the same type
            /// as the input. If `bits` has fewer bits than `end - start`, the high bits are
            /// interpreted as 0. If `end` is greater than the type's width, the high bits of `bits`
            /// must be 0.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `start < end`, or if `end > $t::WIDTH` and bits `$t::WIDTH - start`
            /// through `end - start` of `bits` are nonzero.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitBlockAccess;
            ///
            /// let mut x = 0xab5du16;
            /// x.assign_bits(4, 8, &0xc);
            /// assert_eq!(x, 0xabcd);
            ///
            /// let mut x = 0xabcdu16;
            /// x.assign_bits(100, 200, &0);
            /// assert_eq!(x, 0xabcd);
            ///
            /// let mut x = 0xabcdu16;
            /// x.assign_bits(0, 100, &0x1234);
            /// assert_eq!(x, 0x1234);
            /// ```
            fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits) {
                assert!(start <= end);
                let width = u64::from($t::WIDTH);
                let bits_width = end - start;
                let bits = bits.mod_power_of_two(bits_width);
                if bits != 0 && u64::from(bits.leading_zeros()) < start {
                    panic!("Result exceeds width of output type");
                } else if start >= width {
                    // bits must be 0
                    return;
                } else {
                    *self &= !($t::MAX.mod_power_of_two(min(bits_width, width - start)) << start);
                }
                *self |= bits << start;
            }
        }
    };
}

impl_logic_traits!(u8);
impl_logic_traits!(u16);
impl_logic_traits!(u32);
impl_logic_traits!(u64);
impl_logic_traits!(u128);
impl_logic_traits!(usize);
