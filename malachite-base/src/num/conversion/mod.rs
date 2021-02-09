/// This module provides trait implementations for working with the digits of numbers.
pub mod digits;
/// This module provides trait implementations for converting between different number types.
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
/// This module provides traits for bitwise joining two numbers or splitting them in half.
///
/// Here are some examples of the macro-generated functions:
///
/// # join_halves
/// ```
/// use malachite_base::num::conversion::traits::JoinHalves;
///
/// assert_eq!(u16::join_halves(1, 2), 258);
/// assert_eq!(u32::join_halves(0xabcd, 0x1234), 0xabcd1234);
/// ```
///
/// # split_in_half
/// ```
/// use malachite_base::num::conversion::traits::SplitInHalf;
///
/// assert_eq!(258u16.split_in_half(), (1, 2));
/// assert_eq!(0xabcd1234u32.split_in_half(), (0xabcd, 0x1234));
/// ```
///
/// # lower_half
/// ```
/// use malachite_base::num::conversion::traits::SplitInHalf;
///
/// assert_eq!(258u16.lower_half(), 2);
/// assert_eq!(0xabcd1234u32.lower_half(), 0x1234);
/// ```
///
/// # upper_half
/// ```
/// use malachite_base::num::conversion::traits::SplitInHalf;
///
/// assert_eq!(258u16.upper_half(), 1);
/// assert_eq!(0xabcd1234u32.upper_half(), 0xabcd);
/// ```
pub mod half;
/// This module provides traits converting numbers to `Vec`s of numbers, slices to numbers, or
/// slices to `Vec`s.
///
/// Here are some examples of the macro-generated functions:
///
/// # from_other_type_slice
/// ```
/// use malachite_base::num::conversion::traits::FromOtherTypeSlice;
///
/// let xs: &[u32] = &[];
/// assert_eq!(u32::from_other_type_slice(xs), 0);
/// assert_eq!(u32::from_other_type_slice(&[123u32, 456]), 123);
///
/// assert_eq!(u8::from_other_type_slice(&[0xabcdu16, 0xef01]), 0xcd);
///
/// assert_eq!(u16::from_other_type_slice(&[0xabu8, 0xcd, 0xef]), 0xcdab);
/// assert_eq!(u64::from_other_type_slice(&[0xabu8, 0xcd, 0xef]), 0xefcdab);
/// ```
///
/// # vec_from_other_type_slice
/// ```
/// use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
///
/// assert_eq!(u32::vec_from_other_type_slice(&[123u32, 456]), &[123, 456]);
/// assert_eq!(
///     u8::vec_from_other_type_slice(&[0xcdabu16, 0x01ef, 0x4523, 0x8967]),
///     &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89]
/// );
/// assert_eq!(
///     u16::vec_from_other_type_slice(&[0xabu8, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67]),
///     &[0xcdab, 0x01ef, 0x4523, 0x67]
/// );
/// ```
///
/// # vec_from_other_type
/// ```
/// use malachite_base::num::conversion::traits::VecFromOtherType;
///
/// assert_eq!(u32::vec_from_other_type(123u32), &[123]);
/// assert_eq!(u8::vec_from_other_type(0xcdabu16), &[0xab, 0xcd]);
/// assert_eq!(u16::vec_from_other_type(0xabu8), &[0xab]);
/// ```
pub mod slice;
/// This module provides trait implementations for converting numbers to and from `String`s.
pub mod string;
/// This module defines various traits for converting numbers.
pub mod traits;
