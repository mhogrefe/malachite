#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod arithmetic {
    pub mod abs;
    pub mod add_mul;
    pub mod arithmetic_checked_shl;
    pub mod arithmetic_checked_shr;
    pub mod checked_abs;
    pub mod checked_add;
    pub mod checked_add_mul;
    pub mod checked_div;
    pub mod checked_mul;
    pub mod checked_neg;
    pub mod checked_next_power_of_two;
    pub mod checked_pow;
    pub mod checked_square;
    pub mod checked_sub;
    pub mod checked_sub_mul;
    pub mod div_exact;
    pub mod div_mod;
    pub mod div_round;
    pub mod divisible_by;
    pub mod divisible_by_power_of_two;
    pub mod eq_mod;
    pub mod eq_mod_power_of_two;
    pub mod is_power_of_two;
    pub mod mod_add;
    pub mod mod_is_reduced;
    pub mod mod_mul;
    pub mod mod_neg;
    pub mod mod_op;
    pub mod mod_pow;
    pub mod mod_power_of_two;
    pub mod mod_power_of_two_add;
    pub mod mod_power_of_two_is_reduced;
    pub mod mod_power_of_two_mul;
    pub mod mod_power_of_two_neg;
    pub mod mod_power_of_two_pow;
    pub mod mod_power_of_two_shl;
    pub mod mod_power_of_two_shr;
    pub mod mod_power_of_two_square;
    pub mod mod_power_of_two_sub;
    pub mod mod_shl;
    pub mod mod_shr;
    pub mod mod_square;
    pub mod mod_sub;
    pub mod neg;
    pub mod next_power_of_two;
    pub mod overflowing_abs;
    pub mod overflowing_add;
    pub mod overflowing_add_mul;
    pub mod overflowing_div;
    pub mod overflowing_mul;
    pub mod overflowing_neg;
    pub mod overflowing_pow;
    pub mod overflowing_square;
    pub mod overflowing_sub;
    pub mod overflowing_sub_mul;
    pub mod parity;
    pub mod pow;
    pub mod power_of_two;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_two;
    pub mod saturating_abs;
    pub mod saturating_add;
    pub mod saturating_add_mul;
    pub mod saturating_mul;
    pub mod saturating_neg;
    pub mod saturating_pow;
    pub mod saturating_square;
    pub mod saturating_sub;
    pub mod saturating_sub_mul;
    pub mod shl_round;
    pub mod shr_round;
    pub mod sign;
    pub mod square;
    pub mod sub_mul;
    pub mod traits;
    pub mod unsigneds;
    pub mod wrapping_abs;
    pub mod wrapping_add;
    pub mod wrapping_add_mul;
    pub mod wrapping_div;
    pub mod wrapping_mul;
    pub mod wrapping_neg;
    pub mod wrapping_pow;
    pub mod wrapping_square;
    pub mod wrapping_sub;
    pub mod wrapping_sub_mul;
    pub mod x_mul_y_is_zz;
    pub mod xx_add_yy_is_zz;
    pub mod xx_div_mod_y_is_qr;
    pub mod xx_sub_yy_is_zz;
    pub mod xxx_add_yyy_is_zzz;
    pub mod xxx_sub_yyy_is_zzz;
    pub mod xxxx_add_yyyy_is_zzzz;
}
/// This module defines the traits for primitive integers and some of their basic functionality.
pub mod basic {
    /// This module defines `PrimitiveInt`.
    ///
    /// Here are usage examples of the associated constants:
    ///
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    ///
    /// assert_eq!(u32::WIDTH, 32);
    /// assert_eq!(u32::LOG_WIDTH, 5);
    /// assert_eq!(u32::WIDTH_MASK, 0x1f);
    /// ```
    pub mod integers;
    /// This module defines `PrimitiveSigned`.
    pub mod signeds;
    /// This module defines traits for constants and the Iverson bracket.
    ///
    /// Here are usage examples of the Iverson bracket:
    ///
    /// ```
    /// use malachite_base::num::basic::traits::Iverson;
    ///
    /// assert_eq!(u32::iverson(false), 0);
    /// assert_eq!(i8::iverson(true), 1);
    /// ```
    pub mod traits;
    /// This module defines `PrimitiveUnsigned`.
    pub mod unsigneds;
}
pub mod comparison {
    pub mod ord_abs;
    pub mod traits;
}
/// This module provides traits for converting to and from numbers.
pub mod conversion {
    /// This module provides traits for converting between different number types.
    ///
    /// Here are usage examples of the macro-generated functions:
    ///
    /// # checked_from
    /// ```
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    ///
    /// assert_eq!(u8::checked_from(123u8), Some(123));
    /// assert_eq!(i32::checked_from(-5i32), Some(-5));
    ///
    /// assert_eq!(u16::checked_from(123u8), Some(123));
    /// assert_eq!(i64::checked_from(-5i32), Some(-5));
    /// assert_eq!(u32::checked_from(5u64), Some(5));
    ///
    /// assert_eq!(u8::checked_from(1000u16), None);
    /// assert_eq!(u32::checked_from(-5i32), None);
    /// assert_eq!(i32::checked_from(3000000000u32), None);
    /// assert_eq!(i8::checked_from(-1000i16), None);
    /// ```
    ///
    /// # wrapping_from
    /// ```
    /// use malachite_base::num::conversion::traits::WrappingFrom;
    ///
    /// assert_eq!(u8::wrapping_from(123u8), 123);
    /// assert_eq!(i32::wrapping_from(-5i32), -5);
    ///
    /// assert_eq!(u16::wrapping_from(123u8), 123);
    /// assert_eq!(i64::wrapping_from(-5i32), -5);
    /// assert_eq!(u32::wrapping_from(5u64), 5);
    ///
    /// assert_eq!(u8::wrapping_from(1000u16), 232);
    /// assert_eq!(u32::wrapping_from(-5i32), 4294967291);
    /// assert_eq!(i32::wrapping_from(3000000000u32), -1294967296);
    /// assert_eq!(i8::wrapping_from(-1000i16), 24);
    /// ```
    ///
    /// # saturating_from
    /// ```
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    ///
    /// assert_eq!(u8::saturating_from(123u8), 123);
    /// assert_eq!(i32::saturating_from(-5i32), -5);
    ///
    /// assert_eq!(u16::saturating_from(123u8), 123);
    /// assert_eq!(i64::saturating_from(-5i32), -5);
    /// assert_eq!(u32::saturating_from(5u64), 5);
    ///
    /// assert_eq!(u8::saturating_from(1000u16), 255);
    /// assert_eq!(u32::saturating_from(-5i32), 0);
    /// assert_eq!(i32::saturating_from(3000000000u32), 2147483647);
    /// assert_eq!(i8::saturating_from(-1000i16), -128);
    /// ```
    ///
    /// # overflowing_from
    /// ```
    /// use malachite_base::num::conversion::traits::OverflowingFrom;
    ///
    /// assert_eq!(u8::overflowing_from(123u8), (123, false));
    /// assert_eq!(i32::overflowing_from(-5i32), (-5, false));
    ///
    /// assert_eq!(u16::overflowing_from(123u8), (123, false));
    /// assert_eq!(i64::overflowing_from(-5i32), (-5, false));
    /// assert_eq!(u32::overflowing_from(5u64), (5, false));
    ///
    /// assert_eq!(u8::overflowing_from(1000u16), (232, true));
    /// assert_eq!(u32::overflowing_from(-5i32), (4294967291, true));
    /// assert_eq!(i32::overflowing_from(3000000000u32), (-1294967296, true));
    /// assert_eq!(i8::overflowing_from(-1000i16), (24, true));
    /// ```
    ///
    /// # convertible_from
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    ///
    /// assert_eq!(u8::convertible_from(123u8), true);
    /// assert_eq!(i32::convertible_from(-5i32), true);
    ///
    /// assert_eq!(u16::convertible_from(123u8), true);
    /// assert_eq!(i64::convertible_from(-5i32), true);
    /// assert_eq!(u32::convertible_from(5u64), true);
    ///
    /// assert_eq!(u8::convertible_from(1000u16), false);
    /// assert_eq!(u32::convertible_from(-5i32), false);
    /// assert_eq!(i32::convertible_from(3000000000u32), false);
    /// assert_eq!(i8::convertible_from(-1000i16), false);
    /// ```
    pub mod from;
    pub mod half;
    pub mod slice;
    pub mod traits;
}
/// This module contains iterators that generate primitive integers without repetition.
pub mod exhaustive;
pub mod floats;
pub mod iterator;
pub mod logic {
    pub mod bit_access;
    pub mod bit_block_access;
    pub mod bit_convertible;
    pub mod bit_iterable;
    pub mod bit_scan;
    pub mod count_ones;
    pub mod count_zeros;
    pub mod hamming_distance;
    pub mod leading_zeros;
    pub mod low_mask;
    pub mod not;
    pub mod power_of_two_digit_iterable;
    pub mod power_of_two_digits;
    pub mod rotate;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod traits;
}
/// This module contains iterators that generate primitive integers randomly.
pub mod random;
