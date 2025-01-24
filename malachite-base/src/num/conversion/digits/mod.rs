// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`Digits`](super::traits::Digits), a trait for extracting digits from numbers and constructing
/// numbers from digits.
///
/// # to_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(0u8.to_digits_asc(&6u64), &[]);
/// assert_eq!(2u16.to_digits_asc(&6u32), &[2]);
/// assert_eq!(
///     123456u32.to_digits_asc(&3u16),
///     &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]
/// );
/// ```
///
/// # to_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(0u8.to_digits_asc(&6u64), &[]);
/// assert_eq!(2u16.to_digits_asc(&6u32), &[2]);
/// assert_eq!(
///     123456u32.to_digits_desc(&3u16),
///     &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]
/// );
/// ```
///
/// # from_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(
///     u8::from_digits_asc(&64, [0u64, 0, 0].iter().cloned()),
///     Some(0)
/// );
/// assert_eq!(
///     u32::from_digits_asc(&3, [0u64, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2].iter().cloned()),
///     Some(123456)
/// );
/// assert_eq!(
///     u32::from_digits_asc(&8, [3u16, 7, 1].iter().cloned()),
///     Some(123)
/// );
///
/// assert!(u64::from_digits_asc(&64, [1u8; 1000].iter().cloned()).is_none());
/// assert!(u64::from_digits_asc(&2, [2u8].iter().cloned()).is_none());
/// assert!(u8::from_digits_asc(&1000, [1u16, 2, 3].iter().cloned()).is_none());
/// ```
///
/// # from_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(
///     u8::from_digits_desc(&64, [0u64, 0, 0].iter().cloned()),
///     Some(0)
/// );
/// assert_eq!(
///     u32::from_digits_desc(&3, [2u64, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0].iter().cloned()),
///     Some(123456)
/// );
/// assert_eq!(
///     u32::from_digits_desc(&8, [1u16, 7, 3].iter().cloned()),
///     Some(123)
/// );
///
/// assert!(u64::from_digits_desc(&64, [1u8; 1000].iter().cloned()).is_none());
/// assert!(u64::from_digits_desc(&2, [2u8].iter().cloned()).is_none());
/// assert!(u8::from_digits_desc(&1000, [1u16, 2, 3].iter().cloned()).is_none());
/// ```
pub mod general_digits;
/// [`PowerOf2DigitIterable`](super::traits::PowerOf2DigitIterable), a trait for producing
/// [`PrimitivePowerOf2DigitIterator`](power_of_2_digit_iterable::PrimitivePowerOf2DigitIterator), a
/// double-ended iterator for iterating over a number's base-$2^k$ digits.
///
/// # power_of_2_digits
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
///
/// let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(0u8, 2);
/// assert!(digits.next().is_none());
///
/// // 107 = 1101011b
/// let digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(107u32, 2);
/// assert_eq!(digits.collect_vec(), &[3, 2, 2, 1]);
///
/// let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(0u8, 2);
/// assert!(digits.next_back().is_none());
///
/// // 107 = 1101011b
/// let digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(107u32, 2);
/// assert_eq!(digits.rev().collect_vec(), &[1, 2, 2, 3]);
/// ```
pub mod power_of_2_digit_iterable;
/// [`PowerOf2Digits`](super::traits::PowerOf2Digits), a trait for extracting base-$2^k$ $digits
/// from numbers and constructing numbers from digits.
///
/// # to_power_of_2_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
///
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&0u8, 6),
///     &[]
/// );
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&2u16, 6),
///     &[2]
/// );
/// // 123_10 = 173_8
/// assert_eq!(
///     PowerOf2Digits::<u16>::to_power_of_2_digits_asc(&123u32, 3),
///     &[3, 7, 1]
/// );
/// ```
///
/// # to_power_of_2_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
///
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_desc(&0u8, 6),
///     &[]
/// );
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_desc(&2u16, 6),
///     &[2]
/// );
/// // 123_10 = 173_8
/// assert_eq!(
///     PowerOf2Digits::<u16>::to_power_of_2_digits_desc(&123u32, 3),
///     &[1, 7, 3]
/// );
/// ```
///
/// # from_power_of_2_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
///
/// assert_eq!(
///     u8::from_power_of_2_digits_asc(6, [0u64, 0, 0].iter().cloned()),
///     Some(0)
/// );
/// assert_eq!(
///     u16::from_power_of_2_digits_asc(6, [2u64, 0].iter().cloned()),
///     Some(2)
/// );
/// assert_eq!(
///     u32::from_power_of_2_digits_asc(3, [3u16, 7, 1].iter().cloned()),
///     Some(123)
/// );
///
/// assert!(u8::from_power_of_2_digits_asc(4, [1u64; 100].iter().cloned()).is_none());
/// assert!(u8::from_power_of_2_digits_asc(1, [2u64].iter().cloned()).is_none());
/// ```
///
/// # from_power_of_2_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
///
/// assert_eq!(
///     u8::from_power_of_2_digits_desc(6, [0u64, 0, 0].iter().cloned()),
///     Some(0)
/// );
/// assert_eq!(
///     u16::from_power_of_2_digits_desc(6, [0u64, 2].iter().cloned()),
///     Some(2)
/// );
/// assert_eq!(
///     u32::from_power_of_2_digits_desc(3, [1u16, 7, 3].iter().cloned()),
///     Some(123)
/// );
///
/// assert!(u8::from_power_of_2_digits_desc(4, [1u64; 100].iter().cloned()).is_none());
/// assert!(u8::from_power_of_2_digits_desc(1, [2u64].iter().cloned()).is_none());
/// ```
pub mod power_of_2_digits;
