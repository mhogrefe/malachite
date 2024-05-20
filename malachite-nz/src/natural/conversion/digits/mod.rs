// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Digits`](malachite_base::num::conversion::traits::Digits), a trait for
/// extracting digits from [`Natural`](crate::natural::Natural)s and constructing
/// [`Natural`](crate::natural::Natural)s from digits.
///
/// # to_digits_asc
/// ```
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::ZERO.to_digits_asc(&6u64).is_empty());
/// assert_eq!(Natural::TWO.to_digits_asc(&6u32), &[2]);
/// assert_eq!(
///     Natural::from(123456u32).to_digits_asc(&3u16),
///     &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]
/// );
/// ```
///
/// # to_digits_desc
/// ```
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::ZERO.to_digits_desc(&6u64).is_empty());
/// assert_eq!(Natural::TWO.to_digits_desc(&6u32), &[2]);
/// assert_eq!(
///     Natural::from(123456u32).to_digits_desc(&3u16),
///     &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]
/// );
/// ```
///
/// # from_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::from_digits_asc(&64u64, [0, 0, 0].iter().cloned()).to_debug_string(),
///     "Some(0)"
/// );
/// assert_eq!(
///     Natural::from_digits_asc(&3u64, [0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2].iter().cloned())
///         .to_debug_string(),
///     "Some(123456)"
/// );
/// assert_eq!(
///     Natural::from_digits_asc(&8u16, [3, 7, 1].iter().cloned()).to_debug_string(),
///     "Some(123)"
/// );
/// assert_eq!(
///     Natural::from_digits_asc(&8u16, [3, 10, 1].iter().cloned()).to_debug_string(),
///     "None"
/// );
/// ```
///
/// # from_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::from_digits_desc(&64u64, [0, 0, 0].iter().cloned()).to_debug_string(),
///     "Some(0)"
/// );
/// assert_eq!(
///     Natural::from_digits_desc(&3u64, [2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0].iter().cloned())
///         .to_debug_string(),
///     "Some(123456)"
/// );
/// assert_eq!(
///     Natural::from_digits_desc(&8u16, [1, 7, 3].iter().cloned()).to_debug_string(),
///     "Some(123)"
/// );
/// assert_eq!(
///     Natural::from_digits_desc(&8u16, [3, 10, 1].iter().cloned()).to_debug_string(),
///     "None"
/// );
/// ```
pub mod general_digits;
/// An implementation of
/// [`PowerOf2DigitIterable`](malachite_base::num::conversion::traits::PowerOf2DigitIterable), a
/// trait for iterating over a [`Natural`](crate::natural::Natural)'s base-$2^k$ digits.
///
/// # power_of_2_digits
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
/// use malachite_nz::natural::Natural;
///
/// let n = Natural::ZERO;
/// assert!(PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2)
///     .next()
///     .is_none());
///
/// // 107 = 1223_4
/// let n = Natural::from(107u32);
/// assert_eq!(
///     PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 2).collect_vec(),
///     vec![3, 2, 2, 1]
/// );
///
/// let n = Natural::ZERO;
/// assert!(PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2)
///     .next_back()
///     .is_none());
///
/// // 107 = 1223_4
/// let n = Natural::from(107u32);
/// assert_eq!(
///     PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 2)
///         .rev()
///         .collect_vec(),
///     vec![1, 2, 2, 3]
/// );
/// ```
pub mod power_of_2_digit_iterable;
/// Implementations of [`PowerOf2Digits`](malachite_base::num::conversion::traits::PowerOf2Digits),
/// a trait for extracting base-$2^k$ digits from [`Natural`](crate::natural::Natural)s and
/// constructing [`Natural`](crate::natural::Natural)s from such digits.
///
/// # to_power_of_2_digits_asc
/// ```
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&Natural::ZERO, 6),
///     Vec::<u64>::new()
/// );
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&Natural::TWO, 6),
///     vec![2]
/// );
///
/// // 123_10 = 173_8
/// assert_eq!(
///     PowerOf2Digits::<u16>::to_power_of_2_digits_asc(&Natural::from(123u32), 3),
///     vec![3, 7, 1]
/// );
/// ```
///
/// # to_power_of_2_digits_desc
/// ```
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_desc(&Natural::ZERO, 6),
///     Vec::<u64>::new()
/// );
/// assert_eq!(
///     PowerOf2Digits::<u64>::to_power_of_2_digits_desc(&Natural::TWO, 6),
///     vec![2]
/// );
///
/// // 123_10 = 173_8
/// assert_eq!(
///     PowerOf2Digits::<u16>::to_power_of_2_digits_desc(&Natural::from(123u32), 3),
///     vec![1, 7, 3]
/// );
/// ```
///
/// # from_power_of_2_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::from_power_of_2_digits_asc(6, [0u64, 0, 0].iter().cloned()).to_debug_string(),
///     "Some(0)"
/// );
/// assert_eq!(
///     Natural::from_power_of_2_digits_asc(6, [2u64, 0].iter().cloned()).to_debug_string(),
///     "Some(2)"
/// );
/// assert_eq!(
///     Natural::from_power_of_2_digits_asc(3, [3u16, 7, 1].iter().cloned()).to_debug_string(),
///     "Some(123)"
/// );
/// assert_eq!(
///     Natural::from_power_of_2_digits_asc(3, [100u8].iter().cloned()).to_debug_string(),
///     "None"
/// );
/// ```
///
/// # from_power_of_2_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOf2Digits;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::from_power_of_2_digits_desc(6, [0u64, 0, 0].iter().cloned()).to_debug_string(),
///     "Some(0)"
/// );
/// assert_eq!(
///     Natural::from_power_of_2_digits_desc(6, [0u64, 2].iter().cloned()).to_debug_string(),
///     "Some(2)"
/// );
/// assert_eq!(
///     Natural::from_power_of_2_digits_desc(3, [1u16, 7, 3].iter().cloned()).to_debug_string(),
///     "Some(123)"
/// );
/// assert_eq!(
///     Natural::from_power_of_2_digits_desc(3, [100u8].iter().cloned()).to_debug_string(),
///     "None"
/// );
/// ```
pub mod power_of_2_digits;
