pub mod power_of_two_digit_iterable;
/// This module provides traits for extracting digits from numbers and constructing numbers from
/// digits, where the base is a power of two.
///
/// Here are usage examples of the macro-generated functions:
///
/// # to_power_of_two_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&0u8, 6), &[]);
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&2u16, 6), &[2]);
/// // 123_10 = 173_8
/// assert_eq!(PowerOfTwoDigits::<u16>::to_power_of_two_digits_asc(&123u32, 3), &[3, 7, 1]);
/// ```
///
/// # to_power_of_two_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&0u8, 6), &[]);
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&2u16, 6), &[2]);
/// // 123_10 = 173_8
/// assert_eq!(PowerOfTwoDigits::<u16>::to_power_of_two_digits_desc(&123u32, 3), &[1, 7, 3]);
/// ```
///
/// # from_power_of_two_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// let digits: &[u64] = &[0, 0, 0];
/// assert_eq!(u8::from_power_of_two_digits_asc(6, digits.iter().cloned()), 0);
///
/// let digits: &[u64] = &[2, 0];
/// assert_eq!(u16::from_power_of_two_digits_asc(6, digits.iter().cloned()), 2);
///
/// let digits: &[u16] = &[3, 7, 1];
/// assert_eq!(u32::from_power_of_two_digits_asc(3, digits.iter().cloned()), 123);
/// ```
///
/// # from_power_of_two_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// let digits: &[u64] = &[0, 0, 0];
/// assert_eq!(u8::from_power_of_two_digits_desc(6, digits.iter().cloned()), 0);
///
/// let digits: &[u64] = &[0, 2];
/// assert_eq!(u16::from_power_of_two_digits_desc(6, digits.iter().cloned()), 2);
///
/// let digits: &[u16] = &[1, 7, 3];
/// assert_eq!(u32::from_power_of_two_digits_desc(3, digits.iter().cloned()), 123);
/// ```
pub mod power_of_two_digits;
