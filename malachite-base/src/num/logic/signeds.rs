use num::arithmetic::traits::{ModPowerOfTwo, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::logic::traits::{
    BitAccess, BitBlockAccess, BitScan, CheckedHammingDistance, SignificantBits,
};

macro_rules! impl_logic_traits {
    ($t:ident) => {
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
                    Some(u64::from((self ^ other).count_ones()))
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
                if index < u64::from(Self::WIDTH) {
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
                if index < u64::from(Self::WIDTH) {
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
                if index < u64::from(Self::WIDTH) {
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
                if start >= u64::from(Self::WIDTH) - 1 {
                    if self >= 0 {
                        Some(start)
                    } else {
                        None
                    }
                } else {
                    let index = (!(self | ((1 << start) - 1))).trailing_zeros();
                    if index == $t::WIDTH {
                        None
                    } else {
                        Some(u64::from(index))
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
                if start >= u64::from(Self::WIDTH) - 1 {
                    if self >= 0 {
                        None
                    } else {
                        Some(start)
                    }
                } else {
                    let index = (self & !((1 << start) - 1)).trailing_zeros();
                    if index == $t::WIDTH {
                        None
                    } else {
                        Some(u64::from(index))
                    }
                }
            }
        }

        impl BitBlockAccess for $t {
            type Output = <$t as PrimitiveSigned>::UnsignedOfEqualWidth;

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
            /// Panics if `self < 0 && end - start > $t::WIDTH`.
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
            fn get_bits(&self, start: u64, end: u64) -> Self::Output {
                assert!(start <= end);
                (if start >= u64::from($t::WIDTH) {
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
        }
    };
}

impl_logic_traits!(i8);
impl_logic_traits!(i16);
impl_logic_traits!(i32);
impl_logic_traits!(i64);
impl_logic_traits!(i128);
impl_logic_traits!(isize);
