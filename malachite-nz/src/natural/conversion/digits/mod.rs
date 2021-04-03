/// This module provides traits for extracting digits from numbers and constructing numbers from
/// digits.
///
/// Here are usage examples of the macro-generated functions:
///
/// # to_digits_asc, where `base` is a primitive integer
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::basic::traits::{Zero, Two};
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::ZERO.to_digits_asc(&6u64).is_empty());
/// assert_eq!(Natural::TWO.to_digits_asc(&6u32), &[2]);
/// assert_eq!(Natural::from(123456u32).to_digits_asc(&3u16), &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]);
/// ```
///
/// # to_digits_desc, where `base` is a primitive integer
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::basic::traits::{Zero, Two};
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::ZERO.to_digits_desc(&6u64).is_empty());
/// assert_eq!(Natural::TWO.to_digits_desc(&6u32), &[2]);
/// assert_eq!(Natural::from(123456u32).to_digits_desc(&3u16), &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]);
/// ```
///
/// # from_digits_asc, where `base` is a primitive integer
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from_digits_asc(&64u64, [0, 0, 0].iter().cloned()), 0);
/// assert_eq!(
///     Natural::from_digits_asc(&3u64, [0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2].iter().cloned()),
///     123456
/// );
/// assert_eq!(Natural::from_digits_asc(&8u16, [3, 7, 1].iter().cloned()), 123);
/// ```
///
/// # from_digits_desc, where `base` is a primitive integer
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::conversion::traits::Digits;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from_digits_desc(&64u64, [0, 0, 0].iter().cloned()), 0);
/// assert_eq!(
///     Natural::from_digits_desc(&3u64, [2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0].iter().cloned()),
///     123456
/// );
/// assert_eq!(Natural::from_digits_desc(&8u16, [1, 7, 3].iter().cloned()), 123);
/// ```
pub mod general_digits;
pub mod power_of_two_digit_iterable;
pub mod power_of_two_digits;
