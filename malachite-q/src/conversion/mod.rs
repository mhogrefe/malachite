pub mod from_integer;
pub mod from_natural;
/// This module provides trait implementations for converting values of primitive integer type to
/// `Rational`s.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational::from(PrimitiveInt)
/// ```
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::from(123u32), 123);
/// assert_eq!(Rational::from(-123i32), -123);
/// ```
pub mod from_primitive_int;
pub mod integer_from_rational;
pub mod is_integer;
pub mod natural_from_rational;
/// This module provides trait implementations for converting `Rational`s to values of primitive
/// integer type.
///
/// Here are usage examples of the macro-generated functions:
///
/// # PrimitiveInt::checked_from(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::conversion::traits::CheckedFrom;
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert_eq!(u32::checked_from(&Rational::from(123)).unwrap(), 123);
/// assert_eq!(u32::checked_from(&Rational::from(-123)), None);
/// assert_eq!(u32::checked_from(&Rational::from_str("1000000000000").unwrap()), None);
/// assert_eq!(u32::checked_from(&Rational::from_str("22/7").unwrap()), None);
///
/// assert_eq!(i32::checked_from(&Rational::from(123)).unwrap(), 123);
/// assert_eq!(i32::checked_from(&Rational::from(-123)).unwrap(), -123);
/// assert_eq!(i32::checked_from(&Rational::from_str("-1000000000000").unwrap()), None);
/// assert_eq!(i32::checked_from(&Rational::from_str("1000000000000").unwrap()), None);
/// assert_eq!(i32::checked_from(&Rational::from_str("22/7").unwrap()), None);
/// ```
///
/// # PrimitiveInt::convertible_from(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert_eq!(u32::convertible_from(&Rational::from(123)), true);
/// assert_eq!(u32::convertible_from(&Rational::from(-123)), false);
/// assert_eq!(u32::convertible_from(&Rational::from_str("1000000000000").unwrap()), false);
/// assert_eq!(u32::convertible_from(&Rational::from_str("22/7").unwrap()), false);
///
/// assert_eq!(i32::convertible_from(&Rational::from(123)), true);
/// assert_eq!(i32::convertible_from(&Rational::from(-123)), true);
/// assert_eq!(i32::convertible_from(&Rational::from_str("-1000000000000").unwrap()), false);
/// assert_eq!(i32::convertible_from(&Rational::from_str("1000000000000").unwrap()), false);
/// assert_eq!(i32::convertible_from(&Rational::from_str("22/7").unwrap()), false);
/// ```
///
/// # PrimitiveInt::rounding_from(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert_eq!(u32::rounding_from(&Rational::from(123), RoundingMode::Exact), 123);
///
/// assert_eq!(u32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Floor), 3);
/// assert_eq!(u32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Down), 3);
/// assert_eq!(u32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Ceiling), 4);
/// assert_eq!(u32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Up), 4);
/// assert_eq!(u32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Nearest), 3);
///
/// assert_eq!(u32::rounding_from(&Rational::from(-123), RoundingMode::Down), 0);
/// assert_eq!(u32::rounding_from(&Rational::from(-123), RoundingMode::Ceiling), 0);
/// assert_eq!(u32::rounding_from(&Rational::from(-123), RoundingMode::Nearest), 0);
///
/// assert_eq!(u8::rounding_from(&Rational::from(1000), RoundingMode::Down), 255);
/// assert_eq!(u8::rounding_from(&Rational::from(1000), RoundingMode::Floor), 255);
/// assert_eq!(u8::rounding_from(&Rational::from(1000), RoundingMode::Nearest), 255);
///
/// assert_eq!(i32::rounding_from(&Rational::from(-123), RoundingMode::Exact), -123);
///
/// assert_eq!(i32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Floor), 3);
/// assert_eq!(i32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Down), 3);
/// assert_eq!(i32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Ceiling), 4);
/// assert_eq!(i32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Up), 4);
/// assert_eq!(i32::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Nearest), 3);
///
/// assert_eq!(i32::rounding_from(&Rational::from_str("-22/7").unwrap(), RoundingMode::Floor), -4);
/// assert_eq!(i32::rounding_from(&Rational::from_str("-22/7").unwrap(), RoundingMode::Down), -3);
/// assert_eq!(
///     i32::rounding_from(&Rational::from_str("-22/7").unwrap(), RoundingMode::Ceiling),
///     -3
/// );
/// assert_eq!(i32::rounding_from(&Rational::from_str("-22/7").unwrap(), RoundingMode::Up), -4);
/// assert_eq!(
///     i32::rounding_from(&Rational::from_str("-22/7").unwrap(), RoundingMode::Nearest),
///     -3
/// );
///
/// assert_eq!(i8::rounding_from(&Rational::from(-1000), RoundingMode::Down), -128);
/// assert_eq!(i8::rounding_from(&Rational::from(-1000), RoundingMode::Ceiling), -128);
/// assert_eq!(i8::rounding_from(&Rational::from(-1000), RoundingMode::Nearest), -128);
///
/// assert_eq!(i8::rounding_from(&Rational::from(1000), RoundingMode::Down), 127);
/// assert_eq!(i8::rounding_from(&Rational::from(1000), RoundingMode::Floor), 127);
/// assert_eq!(i8::rounding_from(&Rational::from(1000), RoundingMode::Nearest), 127);
/// ```
pub mod primitive_int_from_rational;
pub mod string;
