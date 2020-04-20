use std::cmp::min;

use comparison::Max;
use num::arithmetic::traits::{ModPowerOfTwo, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{BitBlockAccess, LeadingZeros};

macro_rules! impl_bit_block_access_unsigned {
    ($t:ident) => {
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
                if start >= $t::WIDTH {
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
                let width = $t::WIDTH;
                let bits_width = end - start;
                let bits = bits.mod_power_of_two(bits_width);
                if bits != 0 && LeadingZeros::leading_zeros(bits) < start {
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

impl_bit_block_access_unsigned!(u8);
impl_bit_block_access_unsigned!(u16);
impl_bit_block_access_unsigned!(u32);
impl_bit_block_access_unsigned!(u64);
impl_bit_block_access_unsigned!(u128);
impl_bit_block_access_unsigned!(usize);

macro_rules! impl_bit_block_access_signed {
    ($t:ident, $u:ident) => {
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
    };
}

impl_bit_block_access_signed!(i8, u8);
impl_bit_block_access_signed!(i16, u16);
impl_bit_block_access_signed!(i32, u32);
impl_bit_block_access_signed!(i64, u64);
impl_bit_block_access_signed!(i128, u128);
impl_bit_block_access_signed!(isize, usize);
