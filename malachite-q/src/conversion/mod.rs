pub mod continued_fraction;
pub mod digits;
/// Implements traits for converting `Rational`s to floating-point numbers.
///
/// Here are usage examples of the macro-generated functions:
///
/// # PrimitiveFloat.rounding_from(Rational, RoundingMode)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_q::Rational;
///
/// let one_third = Rational::from_signeds(1i8, 3);
/// assert_eq!(f32::rounding_from(one_third.clone(), RoundingMode::Floor), 0.3333333);
/// assert_eq!(f32::rounding_from(one_third, RoundingMode::Ceiling), 0.33333334);
/// assert_eq!(f32::rounding_from(Rational::ONE_HALF, RoundingMode::Exact), 0.5);
/// let big = Rational::power_of_2(200u64);
/// assert_eq!(f32::rounding_from(big.clone(), RoundingMode::Down), f32::MAX_FINITE);
/// assert_eq!(f32::rounding_from(big, RoundingMode::Up), f32::POSITIVE_INFINITY);
/// let small = Rational::power_of_2(-200i64);
/// assert_eq!(NiceFloat(f32::rounding_from(small.clone(), RoundingMode::Down)), NiceFloat(0.0));
/// assert_eq!(f32::rounding_from(small.clone(), RoundingMode::Up), f32::MIN_POSITIVE_SUBNORMAL);
/// assert_eq!(NiceFloat(f32::rounding_from(-&small, RoundingMode::Down)), NiceFloat(-0.0));
/// assert_eq!(f32::rounding_from(-small, RoundingMode::Up), -f32::MIN_POSITIVE_SUBNORMAL);
/// ```
///
/// # PrimitiveFloat.from(Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_q::Rational;
///
/// assert_eq!(f32::from(Rational::from_signeds(1i8, 3)), 0.33333334);
/// assert_eq!(f32::from(Rational::ONE_HALF), 0.5);
/// assert_eq!(f32::from(Rational::power_of_2(200u64)), f32::MAX_FINITE);
/// assert_eq!(NiceFloat(f32::from(Rational::power_of_2(-200i64))), NiceFloat(0.0));
/// assert_eq!(NiceFloat(f32::from(-Rational::power_of_2(-200i64))), NiceFloat(-0.0));
/// ```
///
/// # PrimitiveFloat.checked_from(Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::conversion::traits::CheckedFrom;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_q::Rational;
///
/// assert_eq!(f32::checked_from(Rational::from_signeds(1i8, 3)), None);
/// assert_eq!(f32::checked_from(Rational::ONE_HALF), Some(0.5));
/// assert_eq!(f32::checked_from(Rational::power_of_2(200u64)), None);
/// assert_eq!(f32::checked_from(Rational::power_of_2(-200i64)), None);
/// assert_eq!(f32::checked_from(-Rational::power_of_2(-200i64)), None);
/// ```
///
/// # PrimitiveFloat.convertible_from(Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_q::Rational;
///
/// assert_eq!(f32::convertible_from(Rational::from_signeds(1i8, 3)), false);
/// assert_eq!(f32::convertible_from(Rational::ONE_HALF), true);
/// assert_eq!(f32::convertible_from(Rational::power_of_2(200u64)), false);
/// assert_eq!(f32::convertible_from(Rational::power_of_2(-200i64)), false);
/// assert_eq!(f32::convertible_from(-Rational::power_of_2(-200i64)), false);
/// ```
///
/// # PrimitiveFloat.rounding_from(&Rational, RoundingMode)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_q::Rational;
///
/// let one_third = Rational::from_signeds(1i8, 3);
/// assert_eq!(f32::rounding_from(&one_third, RoundingMode::Floor), 0.3333333);
/// assert_eq!(f32::rounding_from(&one_third, RoundingMode::Ceiling), 0.33333334);
/// assert_eq!(f32::rounding_from(&Rational::ONE_HALF, RoundingMode::Exact), 0.5);
/// let big = Rational::power_of_2(200u64);
/// assert_eq!(f32::rounding_from(&big, RoundingMode::Down), f32::MAX_FINITE);
/// assert_eq!(f32::rounding_from(&big, RoundingMode::Up), f32::POSITIVE_INFINITY);
/// let small = Rational::power_of_2(-200i64);
/// assert_eq!(NiceFloat(f32::rounding_from(&small, RoundingMode::Down)), NiceFloat(0.0));
/// assert_eq!(f32::rounding_from(&small, RoundingMode::Up), f32::MIN_POSITIVE_SUBNORMAL);
/// assert_eq!(NiceFloat(f32::rounding_from(&-&small, RoundingMode::Down)), NiceFloat(-0.0));
/// assert_eq!(f32::rounding_from(&-small, RoundingMode::Up), -f32::MIN_POSITIVE_SUBNORMAL);
/// ```
///
/// # PrimitiveFloat.from(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_q::Rational;
///
/// assert_eq!(f32::from(&Rational::from_signeds(1i8, 3)), 0.33333334);
/// assert_eq!(f32::from(&Rational::ONE_HALF), 0.5);
/// assert_eq!(f32::from(&Rational::power_of_2(200u64)), f32::MAX_FINITE);
/// assert_eq!(NiceFloat(f32::from(&Rational::power_of_2(-200i64))), NiceFloat(0.0));
/// assert_eq!(NiceFloat(f32::from(&-Rational::power_of_2(-200i64))), NiceFloat(-0.0));
/// ```
///
/// # PrimitiveFloat.checked_from(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::conversion::traits::CheckedFrom;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_q::Rational;
///
/// assert_eq!(f32::checked_from(&Rational::from_signeds(1i8, 3)), None);
/// assert_eq!(f32::checked_from(&Rational::ONE_HALF), Some(0.5));
/// assert_eq!(f32::checked_from(&Rational::power_of_2(200u64)), None);
/// assert_eq!(f32::checked_from(&Rational::power_of_2(-200i64)), None);
/// assert_eq!(f32::checked_from(&-Rational::power_of_2(-200i64)), None);
/// ```
///
/// # PrimitiveFloat.convertible_from(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_q::Rational;
///
/// assert_eq!(f32::convertible_from(&Rational::from_signeds(1i8, 3)), false);
/// assert_eq!(f32::convertible_from(&Rational::ONE_HALF), true);
/// assert_eq!(f32::convertible_from(&Rational::power_of_2(200u64)), false);
/// assert_eq!(f32::convertible_from(&Rational::power_of_2(-200i64)), false);
/// assert_eq!(f32::convertible_from(&-Rational::power_of_2(-200i64)), false);
/// ```
pub mod floating_point_from_rational;
pub mod from_float_simplest;
/// Implements traits for converting floating-point numbers to `Rational`s.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational.from(PrimitiveFloat)
/// ```
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::from(0.0), 0);
/// assert_eq!(Rational::from(1.5).to_string(), "3/2");
/// assert_eq!(Rational::from(-1.5).to_string(), "-3/2");
/// assert_eq!(Rational::from(0.1f32).to_string(), "13421773/134217728");
/// ```
pub mod from_floating_point;
pub mod from_integer;
pub mod from_natural;
/// Trait implementations for converting values of primitive integer type to `Rational`s.
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
/// Traits for converting numbers to and from mantissa and exponent representations.
///
/// Here are some examples of the macro-generated functions:
///
/// # sci_mantissa_and_exponent
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_q::Rational;
///
/// let test = |n: Rational, mantissa: f32, exponent: i64| {
///     let (m, e) = n.clone().sci_mantissa_and_exponent();
///     assert_eq!(NiceFloat(m), NiceFloat(mantissa));
///     assert_eq!(e, exponent);
///
///     let (m, e) = (&n).sci_mantissa_and_exponent();
///     assert_eq!(NiceFloat(m), NiceFloat(mantissa));
///     assert_eq!(e, exponent);
/// };
/// test(Rational::from(3u32), 1.5, 1);
/// test(Rational::from(123u32), 1.921875, 6);
/// test(Rational::from_signeds(1, 123), 1.0406504, -7);
/// test(Rational::from_signeds(22, 7), 1.5714285, 1);
/// ```
///
/// # from_sci_mantissa_and_exponent
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// let test = |mantissa: f32, exponent: i64, out: Option<Rational>| {
///     assert_eq!(
///         <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
///             mantissa, exponent
///         ),
///         out
///     );
///
///     assert_eq!(
///         <Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
///             mantissa, exponent
///         ),
///         out
///     );
/// };
/// test(1.5, 1, Some(Rational::from(3u32)));
/// test(1.51, 1, Some(Rational::from_signeds(6333399, 2097152)));
/// test(1.921875, 6, Some(Rational::from(123u32)));
///
/// test(2.0, 1, None);
/// test(10.0, 1, None);
/// test(0.5, 1, None);
/// ```
pub mod mantissa_and_exponent;
pub mod natural_from_rational;
/// Trait implementations for converting `Rational`s to values of primitive integer type.
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
pub mod traits;
