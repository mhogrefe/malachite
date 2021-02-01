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
pub mod general_digits;
pub mod power_of_two_digit_iterable;
pub mod power_of_two_digits;
