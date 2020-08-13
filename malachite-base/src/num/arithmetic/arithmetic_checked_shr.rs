use std::ops::{Neg, Shr};

use num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr, UnsignedAbs};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;

//TODO clean wheres

fn _arithmetic_checked_shr_unsigned_signed<
    T: PrimitiveInteger,
    U: Ord + WrappingFrom<u64>,
    S: Copy + Ord + Zero,
>(
    x: T,
    bits: S,
) -> Option<T>
where
    S: UnsignedAbs<Output = U>,
    T: ArithmeticCheckedShl<U, Output = T> + Shr<U, Output = T>,
{
    if bits < S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let abs_bits = bits.unsigned_abs();
        Some(if abs_bits >= U::wrapping_from(T::WIDTH) {
            T::ZERO
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shr_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shr_unsigned_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShr<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2). If the result is too large
                    /// to fit in a `$t`, `None` is returned. Zero may be shifted by any amount, and
                    /// any number may be shifted by any non-negative amount; shifting by a large
                    /// amount returns `Some(0)`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
                    ///
                    /// assert_eq!(100u8.arithmetic_checked_shr(3), Some(12u8));
                    /// assert_eq!(100u8.arithmetic_checked_shr(100), Some(0u8));
                    /// assert_eq!(3u8.arithmetic_checked_shr(-6), Some(192u8));
                    /// assert_eq!(3u8.arithmetic_checked_shr(-7), None);
                    /// assert_eq!(3u8.arithmetic_checked_shr(-100), None);
                    /// assert_eq!(0u8.arithmetic_checked_shr(-100), Some(0u8));
                    /// ```
                    #[inline]
                    fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shr_unsigned_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shr_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shr_unsigned_signed);

fn _arithmetic_checked_shr_signed_signed<
    T: PrimitiveInteger,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + Zero,
>(
    x: T,
    bits: S,
) -> Option<T>
where
    S: UnsignedAbs<Output = U>,
    T: Neg<Output = T> + ArithmeticCheckedShl<U, Output = T> + Shr<U, Output = T>,
{
    if bits < S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let width = U::wrapping_from(T::WIDTH);
        let abs_bits = bits.unsigned_abs();
        Some(if width != U::ZERO && abs_bits >= width {
            -T::iverson(x < T::ZERO)
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shr_signed_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shr_signed_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShr<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2). If the result is too large
                    /// to fit in a `$t`, `None` is returned. Zero may be shifted by any amount, and
                    /// any number may be shifted by any non-negative amount; shifting by a large
                    /// amount returns `Some(0)` if `self` is positive, and `Some(-1)` if `self` is
                    /// negative.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
                    ///
                    /// assert_eq!(100i8.arithmetic_checked_shr(3), Some(12i8));
                    /// assert_eq!((-100i8).arithmetic_checked_shr(3), Some(-13i8));
                    /// assert_eq!(100i8.arithmetic_checked_shr(100), Some(0i8));
                    /// assert_eq!((-100i8).arithmetic_checked_shr(100), Some(-1i8));
                    /// assert_eq!(3i8.arithmetic_checked_shr(-5), Some(96i8));
                    /// assert_eq!(3i8.arithmetic_checked_shr(-6), None);
                    /// assert_eq!((-3i8).arithmetic_checked_shr(-5), Some(-96i8));
                    /// assert_eq!((-3i8).arithmetic_checked_shr(-6), None);
                    /// assert_eq!(3i8.arithmetic_checked_shr(-100), None);
                    /// assert_eq!((-3i8).arithmetic_checked_shr(-100), None);
                    /// assert_eq!(0i8.arithmetic_checked_shr(-100), Some(0i8));
                    /// ```
                    #[inline]
                    fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shr_signed_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shr_signed_signed_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shr_signed_signed);
