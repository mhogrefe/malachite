use std::cmp::min;
use std::ops::Neg;

use num::arithmetic::traits::{ModPowerOfTwo, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{BitBlockAccess, LeadingZeros};

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

fn _get_bits_unsigned<T: ModPowerOfTwo<Output = T> + PrimitiveInt>(
    x: &T,
    start: u64,
    end: u64,
) -> T {
    assert!(start <= end);
    if start >= T::WIDTH {
        T::ZERO
    } else {
        (*x >> start).mod_power_of_two(end - start)
    }
}

fn _assign_bits_unsigned<T: ModPowerOfTwo<Output = T> + PrimitiveInt>(
    x: &mut T,
    start: u64,
    end: u64,
    bits: &T,
) {
    assert!(start <= end);
    let width = T::WIDTH;
    let bits_width = end - start;
    let bits = bits.mod_power_of_two(bits_width);
    if bits != T::ZERO && LeadingZeros::leading_zeros(bits) < start {
        panic!(ERROR_MESSAGE);
    } else if start < width {
        *x &= !(T::MAX.mod_power_of_two(min(bits_width, width - start)) << start);
        *x |= bits << start;
    }
}

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
            #[inline]
            fn get_bits(&self, start: u64, end: u64) -> Self {
                _get_bits_unsigned(self, start, end)
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
            #[inline]
            fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits) {
                _assign_bits_unsigned(self, start, end, bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_block_access_unsigned);

fn _get_bits_signed<T: ModPowerOfTwo<Output = U> + Neg<Output = T> + PrimitiveInt, U>(
    x: &T,
    start: u64,
    end: u64,
) -> U {
    assert!(start <= end);
    (if start >= T::WIDTH {
        -T::iverson(*x < T::ZERO)
    } else {
        *x >> start
    })
    .mod_power_of_two(end - start)
}

fn _assign_bits_signed<
    T: PrimitiveInt + UnsignedAbs<Output = U> + WrappingFrom<U>,
    U: BitBlockAccess<Bits = U> + ModPowerOfTwo<Output = U> + PrimitiveInt,
>(
    x: &mut T,
    start: u64,
    end: u64,
    bits: &U,
) {
    assert!(start <= end);
    if *x >= T::ZERO {
        let mut abs_x = x.unsigned_abs();
        abs_x.assign_bits(start, end, bits);
        if abs_x.get_highest_bit() {
            panic!(ERROR_MESSAGE);
        }
        *x = T::wrapping_from(abs_x);
    } else {
        let width = T::WIDTH - 1;
        let bits_width = end - start;
        let bits = bits.mod_power_of_two(bits_width);
        let max = U::MAX;
        if bits_width > width + 1 {
            panic!(ERROR_MESSAGE);
        } else if start >= width {
            if bits != max.mod_power_of_two(bits_width) {
                panic!(ERROR_MESSAGE);
            }
        } else {
            let lower_width = width - start;
            if end > width && bits >> lower_width != max.mod_power_of_two(end - width) {
                panic!(ERROR_MESSAGE);
            } else {
                *x &= T::wrapping_from(
                    !(max.mod_power_of_two(min(bits_width, lower_width)) << start),
                );
                *x |= T::wrapping_from(bits << start);
            }
        }
    }
}

macro_rules! impl_bit_block_access_signed {
    ($u:ident, $s:ident) => {
        impl BitBlockAccess for $s {
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
            /// Panics if `start < end` or `self < 0 && end - start > $s::WIDTH`.
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
            #[inline]
            fn get_bits(&self, start: u64, end: u64) -> Self::Bits {
                _get_bits_signed(self, start, end)
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
            /// Panics if `start < end`, or if `end >= $s::WIDTH` and bits `$s::WIDTH - start`
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
            #[inline]
            fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits) {
                _assign_bits_signed(self, start, end, bits)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_bit_block_access_signed);
