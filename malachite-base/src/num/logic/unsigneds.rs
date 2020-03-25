use std::cmp::min;
use std::marker::PhantomData;
use std::ops::Index;

use comparison::Max;
use named::Named;
use num::arithmetic::traits::{
    DivRound, ModPowerOfTwo, Parity, PowerOfTwo, SaturatingSubAssign, TrueCheckedShl,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::{
    BitAccess, BitBlockAccess, BitConvertible, BitIterable, BitScan, CountOnes, HammingDistance,
    LeadingZeros, LowMask, PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
    SignificantBits, TrailingZeros,
};
use round::RoundingMode;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitivePowerOfTwoDigitIterator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    pub(crate) value: T,
    pub(crate) log_base: u64,
    pub(crate) some_remaining: bool,
    // If `n` is nonzero, this index initially points to the least-significant bit of the least-
    // significant digit, and is left-shifted by next().
    pub(crate) i: u64,
    // If `n` is nonzero, this mask initially points to the least-significant bit of the most-
    // significant nonzero digit, and is right-shifted by next_back().
    pub(crate) j: u64,
    boo: PhantomData<U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator for PrimitivePowerOfTwoDigitIterator<T, U>
where
    U: WrappingFrom<<T as BitBlockAccess>::Bits>,
{
    type Item = U;

    /// A function to iterate through the digits of a primitive unsigned integer in ascending order
    /// (least-significant first). The base is 2<sup>`log_base`</sup> and the output type is `U`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::PowerOfTwoDigitIterable;
    /// use malachite_base::num::logic::unsigneds::PrimitivePowerOfTwoDigitIterator;
    ///
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.next(), None);
    ///
    /// // 107 = 1101011b
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.next(), Some(3));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(1));
    /// assert_eq!(digits.next(), None);
    /// ```
    fn next(&mut self) -> Option<U> {
        if self.some_remaining {
            let digit = U::wrapping_from(self.value.get_bits(self.i, self.i + self.log_base));
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.i += self.log_base;
            Some(digit)
        } else {
            None
        }
    }

    /// A function that returns the length of the digits iterator; that is, the value's significant
    /// digit count. The format is (lower bound, Option<upper bound>), but in this case it's trivial
    /// to always have an exact bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::PowerOfTwoDigitIterable;
    /// use malachite_base::num::logic::unsigneds::PrimitivePowerOfTwoDigitIterator;
    ///
    /// let digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.size_hint(), (0, Some(0)));
    ///
    /// let digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.size_hint(), (4, Some(4)));
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = usize::exact_from(
            self.value
                .significant_bits()
                .div_round(self.log_base, RoundingMode::Ceiling),
        );
        (significant_digits, Some(significant_digits))
    }
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> DoubleEndedIterator
    for PrimitivePowerOfTwoDigitIterator<T, U>
where
    U: WrappingFrom<<T as BitBlockAccess>::Bits>,
{
    /// A function to iterate through the digits of a primitive unsigned integer in descending order
    /// (most-significant first). The base is 2<sup>`log_base`</sup> and the output type is `U`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::PowerOfTwoDigitIterable;
    /// use malachite_base::num::logic::unsigneds::PrimitivePowerOfTwoDigitIterator;
    ///
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.next_back(), None);
    ///
    /// // 107 = 1101011b
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.next_back(), Some(1));
    /// assert_eq!(digits.next_back(), Some(2));
    /// assert_eq!(digits.next_back(), Some(2));
    /// assert_eq!(digits.next_back(), Some(3));
    /// assert_eq!(digits.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<U> {
        if self.some_remaining {
            if self.i == self.j {
                self.some_remaining = false;
            }
            let digit = U::wrapping_from(self.value.get_bits(self.j, self.j + self.log_base));
            self.j.saturating_sub_assign(self.log_base);
            Some(digit)
        } else {
            None
        }
    }
}

/// This allows for some optimizations, e.g. when collecting into a `Vec`.
impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> ExactSizeIterator
    for PrimitivePowerOfTwoDigitIterator<T, U>
where
    U: WrappingFrom<<T as BitBlockAccess>::Bits>,
{
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> PowerOfTwoDigitIterator<U>
    for PrimitivePowerOfTwoDigitIterator<T, U>
where
    U: WrappingFrom<<T as BitBlockAccess>::Bits>,
{
    /// A function to retrieve base-2<sup>`log_base`</sup> digits by index. Indexing at or above the
    /// significant digit count returns zero. The output type is `U`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::logic::traits::{PowerOfTwoDigitIterable, PowerOfTwoDigitIterator};
    /// use malachite_base::num::logic::unsigneds::PrimitivePowerOfTwoDigitIterator;
    ///
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.get(0), 0);
    ///
    /// // 107 = 1101011b
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.get(0), 3);
    /// assert_eq!(digits.get(1), 2);
    /// assert_eq!(digits.get(2), 2);
    /// assert_eq!(digits.get(100), 0);
    /// ```
    fn get(&self, index: u64) -> U {
        let i = index * self.log_base;
        U::wrapping_from(self.value.get_bits(i, i + self.log_base))
    }
}

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
                Self::WIDTH - LeadingZeros::leading_zeros(self)
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
                CountOnes::count_ones(self ^ other)
            }
        }

        impl LowMask for $t {
            /// Returns a value with the least significant `bits` bits on and the remaining bits
            /// off.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than the width of `$t`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::LowMask;
            ///
            /// assert_eq!(u16::low_mask(0), 0);
            /// assert_eq!(u8::low_mask(3), 0x7);
            /// assert_eq!(u8::low_mask(8), 0xff);
            /// assert_eq!(u64::low_mask(40), 0xff_ffff_ffff);
            /// ```
            #[inline]
            fn low_mask(bits: u64) -> $t {
                assert!(bits <= $t::WIDTH);
                if bits == $t::WIDTH {
                    $t::MAX
                } else {
                    $t::power_of_two(bits) - 1
                }
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
                index < Self::WIDTH && *self & $t::power_of_two(index) != 0
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
                if index < Self::WIDTH {
                    *self |= $t::power_of_two(index);
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
                if index < Self::WIDTH {
                    *self &= !$t::power_of_two(index);
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
                Some(if start >= Self::WIDTH {
                    start
                } else {
                    TrailingZeros::trailing_zeros(!(self | $t::low_mask(start)))
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
                if start >= Self::WIDTH {
                    None
                } else {
                    let index = TrailingZeros::trailing_zeros(self & !$t::low_mask(start));
                    if index == Self::WIDTH {
                        None
                    } else {
                        Some(index)
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

        impl BitConvertible for $t {
            /// Returns a `Vec` containing the bits of `self` in ascending order: least- to most-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, it ends with `true`.
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
            /// assert_eq!(0u8.to_bits_asc(), &[]);
            /// assert_eq!(2u16.to_bits_asc(), &[false, true]);
            /// assert_eq!(123u32.to_bits_asc(), &[true, true, false, true, true, true, true]);
            /// ```
            fn to_bits_asc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                let mut x = *self;
                while x != 0 {
                    bits.push(x.odd());
                    x >>= 1;
                }
                bits
            }

            /// Returns a `Vec` containing the bits of `self` in descending order: most- to least-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, it begins with `true`.
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
            /// assert_eq!(0u8.to_bits_desc(), &[]);
            /// assert_eq!(2u16.to_bits_desc(), &[true, false]);
            /// assert_eq!(123u32.to_bits_desc(), &[true, true, true, true, false, true, true]);
            /// ```
            fn to_bits_desc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                if *self == 0 {
                    return bits;
                }
                bits.push(true);
                if *self == 1 {
                    return bits;
                }
                let mut mask = $t::power_of_two($t::WIDTH - LeadingZeros::leading_zeros(*self) - 2);
                while mask != 0 {
                    bits.push(*self & mask != 0);
                    mask >>= 1;
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
            /// assert_eq!(u8::from_bits_asc(&[]), 0);
            /// assert_eq!(u16::from_bits_asc(&[false, true, false]), 2);
            /// assert_eq!(u32::from_bits_asc(&[true, true, false, true, true, true, true]), 123);
            /// ```
            fn from_bits_asc(bits: &[bool]) -> $t {
                let width = usize::exact_from($t::WIDTH);
                if bits.len() > width {
                    assert!(bits[width..].iter().all(|&bit| !bit));
                }
                let mut n = 0;
                let mut mask = 1;
                for &bit in bits {
                    if bit {
                        n |= mask;
                    }
                    mask <<= 1;
                }
                n
            }

            /// Converts a slice of bits into a value. The input bits are in descending order: most-
            /// to least-significant. The function panics if the input represents a number that
            /// can't fit in $t.
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
            /// assert_eq!(u8::from_bits_desc(&[]), 0);
            /// assert_eq!(u16::from_bits_desc(&[false, true, false]), 2);
            /// assert_eq!(u32::from_bits_desc(&[true, true, true, true, false, true, true]), 123);
            /// ```
            fn from_bits_desc(mut bits: &[bool]) -> $t {
                let width = usize::exact_from($t::WIDTH);
                if bits.len() > width {
                    let (bits_lo, bits_hi) = bits.split_at(bits.len() - width);
                    assert!(bits_lo.iter().all(|&bit| !bit));
                    bits = bits_hi;
                }
                let mut n = 0;
                for &bit in bits {
                    n <<= 1;
                    if bit {
                        n |= 1;
                    }
                }
                n
            }
        }

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
            fn bits(self) -> PrimitiveUnsignedBitIterator<$t> {
                let significant_bits = self.significant_bits();
                PrimitiveUnsignedBitIterator {
                    value: self,
                    some_remaining: significant_bits != 0,
                    i_mask: 1,
                    j_mask: $t::power_of_two(significant_bits.saturating_sub(1)),
                }
            }
        }

        macro_rules! impl_logic_traits_inner {
            ($u:ident) => {
                impl PowerOfTwoDigits<$u> for $t {
                    /// Returns a `Vec` containing the digits of `self` in ascending order: least-
                    /// to most-significant, where the base is a power of two. The base-2 logarithm
                    /// of the base is specified. The type of each digit is `$u`, and `log_base`
                    /// must be no larger than the width of `$u`. If `self` is 0, the `Vec` is
                    /// empty; otherwise, it ends with a nonzero digit.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(n)
                    ///
                    /// where n = `self.significant_bits()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is
                    /// zero.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&0u8, 6),
                    ///     &[]
                    /// );
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&2u16, 6),
                    ///     &[2]
                    /// );
                    /// // 123_10 = 173_8
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_asc(&123u32, 3),
                    ///     &[3, 7, 1]
                    /// );
                    /// ```
                    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<$u> {
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let mut digits = Vec::new();
                        if *self == 0 {
                        } else if self.significant_bits() <= log_base {
                            digits.push($u::wrapping_from(*self));
                        } else {
                            let mut x = *self;
                            let mask = $u::low_mask(log_base);
                            while x != 0 {
                                digits.push($u::wrapping_from(x) & mask);
                                x >>= log_base;
                            }
                        }
                        digits
                    }

                    /// Returns a `Vec` containing the digits of `self` in descending order: most-
                    /// to least-significant, where the base is a power of two. The base-2 logarithm
                    /// of the base is specified. The type of each digit is `$u`, and `log_base`
                    /// must be no larger than the width of `$u`. If `self` is 0, the `Vec` is
                    /// empty; otherwise, it begins with a nonzero digit.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(n)
                    ///
                    /// where n = `self.significant_bits()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is
                    /// zero.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&0u8, 6),
                    ///     &[]
                    /// );
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&2u16, 6),
                    ///     &[2]
                    /// );
                    /// // 123_10 = 173_8
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_desc(&123u32, 3),
                    ///     &[1, 7, 3]
                    /// );
                    /// ```
                    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<$u> {
                        let mut digits = self.to_power_of_two_digits_asc(log_base);
                        digits.reverse();
                        digits
                    }

                    /// Converts a slice of digits into a value, where the base is a power of two.
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// ascending order: least- to most-significant. The type of each digit is `$u`,
                    /// and `log_base` must be no larger than the width of `$u`. The function panics
                    /// if the input represents a number that can't fit in $t.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// where n = `digits.len()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, if `log_base` is
                    /// zero, if the digits represent a value that isn't representable by $t, or if
                    /// some digit is greater than 2<sup>`log_base`.</sup>
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// let digits: &[u64] = &[0, 0, 0];
                    /// assert_eq!(u8::from_power_of_two_digits_asc(6, digits), 0);
                    ///
                    /// let digits: &[u64] = &[2, 0];
                    /// assert_eq!(u16::from_power_of_two_digits_asc(6, digits), 2);
                    ///
                    /// let digits: &[u16] = &[3, 7, 1];
                    /// assert_eq!(u32::from_power_of_two_digits_asc(3, digits), 123);
                    /// ```
                    fn from_power_of_two_digits_asc(log_base: u64, digits: &[$u]) -> $t {
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let mut n = 0;
                        for &digit in digits.iter().rev() {
                            assert!(digit.significant_bits() <= log_base);
                            if let Some(shifted) = n.true_checked_shl(log_base) {
                                n = shifted | $t::wrapping_from(digit);
                            } else {
                                panic!("value represented by digits is too large");
                            }
                        }
                        n
                    }

                    /// Converts a slice of digits into a value, where the base is a power of two.
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// descending order: most- to least-significant. The type of each digit is
                    /// `$u`, and `log_base` must be no larger than the width of `$u`. The function
                    /// panics if the input represents a number that can't fit in $t.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// where n = `digits.len()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, if `log_base` is
                    /// zero, if the digits represent a value that isn't representable by $t, or if
                    /// some digit is greater than 2<sup>`log_base`.</sup>
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// let digits: &[u64] = &[0, 0, 0];
                    /// assert_eq!(u8::from_power_of_two_digits_desc(6, digits), 0);
                    ///
                    /// let digits: &[u64] = &[0, 2];
                    /// assert_eq!(u16::from_power_of_two_digits_desc(6, digits), 2);
                    ///
                    /// let digits: &[u16] = &[1, 7, 3];
                    /// assert_eq!(u32::from_power_of_two_digits_desc(3, digits), 123);
                    /// ```
                    fn from_power_of_two_digits_desc(log_base: u64, digits: &[$u]) -> $t {
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let mut n = 0;
                        for &digit in digits {
                            assert!(digit.significant_bits() <= log_base);
                            if let Some(shifted) = n.true_checked_shl(log_base) {
                                n = shifted | $t::wrapping_from(digit);
                            } else {
                                panic!("value represented by digits is too large");
                            }
                        }
                        n
                    }
                }

                impl PowerOfTwoDigitIterable<$u> for $t {
                    type PowerOfTwoDigitIterator = PrimitivePowerOfTwoDigitIterator<$t, $u>;

                    /// Returns a double-ended iterator over the base-2<sup>`log_base`</sup> digits
                    /// of a primitive unsigned integer. The forward order is ascending, so that
                    /// less significant digits appear first. There are no trailing zeros going
                    /// forward, or leading zeros going backward. The type of the digits is `$u`.
                    ///
                    /// If it's necessary to get a `Vec` of all the digits, consider using
                    /// `to_power_of_to_digits_asc` or `to_power_of_two_digits_desc` instead.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// #Panics
                    ///
                    /// Panics if `log_base` is larger than the width of `$u`.
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigitIterable;
                    /// use malachite_base::num::logic::unsigneds::PrimitivePowerOfTwoDigitIterator;
                    ///
                    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
                    /// assert!(digits.next().is_none());
                    ///
                    /// // 107 = 1101011b
                    /// let mut digits =
                    ///     PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
                    /// assert_eq!(digits.collect::<Vec<u8>>(), vec![3, 2, 2, 1]);
                    ///
                    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
                    /// assert!(digits.next_back().is_none());
                    ///
                    /// // 107 = 1101011b
                    /// let mut digits =
                    ///     PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
                    /// assert_eq!(digits.rev().collect::<Vec<u8>>(), vec![1, 2, 2, 3]);
                    /// ```
                    fn power_of_two_digits(
                        self,
                        log_base: u64,
                    ) -> PrimitivePowerOfTwoDigitIterator<$t, $u> {
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let significant_digits = self
                            .significant_bits()
                            .div_round(log_base, RoundingMode::Ceiling);
                        PrimitivePowerOfTwoDigitIterator {
                            value: self,
                            log_base,
                            some_remaining: significant_digits != 0,
                            i: 0,
                            j: significant_digits.saturating_sub(1) * log_base,
                            boo: PhantomData,
                        }
                    }
                }
            };
        }

        impl_logic_traits_inner!(u8);
        impl_logic_traits_inner!(u16);
        impl_logic_traits_inner!(u32);
        impl_logic_traits_inner!(u64);
        impl_logic_traits_inner!(u128);
        impl_logic_traits_inner!(usize);
    };
}

impl_logic_traits!(u8);
impl_logic_traits!(u16);
impl_logic_traits!(u32);
impl_logic_traits!(u64);
impl_logic_traits!(u128);
impl_logic_traits!(usize);

pub fn _to_bits_asc_unsigned_naive<T: PrimitiveUnsigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn _to_bits_desc_unsigned_naive<T: PrimitiveUnsigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in (0..n.significant_bits()).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn _from_bits_asc_unsigned_naive<T: PrimitiveUnsigned>(bits: &[bool]) -> T {
    let mut n = T::ZERO;
    for i in bits
        .iter()
        .enumerate()
        .filter_map(|(i, &bit)| if bit { Some(u64::exact_from(i)) } else { None })
    {
        n.set_bit(i);
    }
    n
}

pub fn _from_bits_desc_unsigned_naive<T: PrimitiveUnsigned>(bits: &[bool]) -> T {
    let mut n = T::ZERO;
    for i in
        bits.iter()
            .rev()
            .enumerate()
            .filter_map(|(i, &bit)| if bit { Some(u64::exact_from(i)) } else { None })
    {
        n.set_bit(i);
    }
    n
}
