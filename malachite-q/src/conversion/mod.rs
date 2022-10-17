/// Implementations of traits for converting [`Rational`](crate::Rational)s to and from continued
/// fractions.
pub mod continued_fraction;
/// Implementations of traits for working with the digits of [`Rational`](crate::Rational)s.
pub mod digits;
/// Functions for converting a primitive float to a [`Rational`](crate::Rational), choosing the
/// [`Rational`](crate::Rational) with minimal denominator that rounds to the given float.
pub mod from_float_simplest;
/// Implementations of the [`From`] trait for converting an
/// [`Integer`](malachite_nz::integer::Integer) to a [`Rational`](crate::Rational).
pub mod from_integer;
/// Implementations of the [`From`] trait for converting a
/// [`Integer`](malachite_nz::natural::Natural) to a [`Rational`](crate::Rational).
pub mod from_natural;
/// Functions for constructing a [`Rational`](crate::Rational) from a numerator and denominator,
/// or from a sign, numerator, and denominator.
pub mod from_numerator_and_denominator;
/// An implementation of the [`TryFrom`] trait for converting a primitive float to a
/// [`Rational`](crate::Rational).
///
/// # try_from
/// ```
/// use malachite_base::strings::ToDebugString;
/// use malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::try_from(0.0).to_debug_string(), "Ok(0)");
/// assert_eq!(Rational::try_from(1.5).to_debug_string(), "Ok(3/2)");
/// assert_eq!(Rational::try_from(-1.5).to_debug_string(), "Ok(-3/2)");
/// assert_eq!(Rational::try_from(0.1f32).to_debug_string(), "Ok(13421773/134217728)");
/// assert_eq!(Rational::try_from(f32::NAN), Err(RationalFromPrimitiveFloatError));
/// ```
pub mod from_primitive_float;
/// Implementations of the [`From`] trait for converting a primitive integer to a
/// [`Rational`](crate::Rational).
///
/// # from
/// ```
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::from(123u32), 123);
/// assert_eq!(Rational::from(-123i32), -123);
/// ```
pub mod from_primitive_int;
/// Implementations of traits for converting a [`Rational`](crate::Rational) to an
/// [`Integer`](malachite_nz::integer::Integer).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
pub mod integer_from_rational;
/// An implementation of [`IsInteger`](malachite_base::num::conversion::traits::IsInteger), a trait
/// for determining whether a number is an integer.
pub mod is_integer;
/// An implementation of
/// [`SciMantissaAndExponent`](malachite_base::num::conversion::traits::SciMantissaAndExponent), a
/// trait for converting numbers to and from a mantissa-and-exponent representation.
///
/// See [`PrimitiveFloat`](malachite_base::num::basic::floats::PrimitiveFloat) for a description of
/// the different types of mantissas and exponents.
///
/// # sci_mantissa_and_exponent
/// ```
/// extern crate malachite_base;
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
///
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_q::Rational;
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
/// Functions for mutating a [`Rational`](crate::Rational)'s numerator and/or denominator in place.
pub mod mutate_numerator_and_denominator;
/// Implementations of traits for converting a [`Rational`](crate::Rational) to a
/// [`Natural`](malachite_nz::natural::Natural).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
pub mod natural_from_rational;
/// Functions and implementations of traits for converting a [`Rational`](crate::Rational) to a
/// primitive float.
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # rounding_from
/// ```
/// extern crate malachite_base;
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
/// # try_from
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_q::conversion::primitive_float_from_rational::PrimitiveFloatFromRationalError;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     f32::try_from(Rational::from_signeds(1i8, 3)),
///     Err(PrimitiveFloatFromRationalError)
/// );
/// assert_eq!(f32::try_from(Rational::ONE_HALF), Ok(0.5));
/// assert_eq!(f32::try_from(Rational::power_of_2(200u64)), Err(PrimitiveFloatFromRationalError));
/// assert_eq!(f32::try_from(Rational::power_of_2(-200i64)), Err(PrimitiveFloatFromRationalError));
/// assert_eq!(
///     f32::try_from(-Rational::power_of_2(-200i64)),
///     Err(PrimitiveFloatFromRationalError)
/// );
///
/// assert_eq!(
///     f32::try_from(&Rational::from_signeds(1i8, 3)),
///     Err(PrimitiveFloatFromRationalError)
/// );
/// assert_eq!(f32::try_from(&Rational::ONE_HALF), Ok(0.5));
/// assert_eq!(f32::try_from(&Rational::power_of_2(200u64)), Err(PrimitiveFloatFromRationalError));
/// assert_eq!(
///     f32::try_from(&Rational::power_of_2(-200i64)), Err(PrimitiveFloatFromRationalError)
/// );
/// assert_eq!(
///     f32::try_from(&-Rational::power_of_2(-200i64)), Err(PrimitiveFloatFromRationalError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// extern crate malachite_base;
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
///
/// assert_eq!(f32::convertible_from(&Rational::from_signeds(1i8, 3)), false);
/// assert_eq!(f32::convertible_from(&Rational::ONE_HALF), true);
/// assert_eq!(f32::convertible_from(&Rational::power_of_2(200u64)), false);
/// assert_eq!(f32::convertible_from(&Rational::power_of_2(-200i64)), false);
/// assert_eq!(f32::convertible_from(&-Rational::power_of_2(-200i64)), false);
/// ```
pub mod primitive_float_from_rational;
/// Implementations of traits for converting a [`Rational`](crate::Rational) to a primitive
/// integer.
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # try_from
/// ```
/// use malachite_q::conversion::primitive_int_from_rational::{
///     SignedFromRationalError,
///     UnsignedFromRationalError
/// };
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert_eq!(u32::try_from(&Rational::from(123)).unwrap(), 123);
/// assert_eq!(u32::try_from(&Rational::from(-123)), Err(UnsignedFromRationalError));
/// assert_eq!(
///     u32::try_from(&Rational::from_str("1000000000000").unwrap()),
///     Err(UnsignedFromRationalError)
/// );
/// assert_eq!(u32::try_from(&Rational::from_signeds(22, 7)), Err(UnsignedFromRationalError));
///
/// assert_eq!(i32::try_from(&Rational::from(123)).unwrap(), 123);
/// assert_eq!(i32::try_from(&Rational::from(-123)).unwrap(), -123);
/// assert_eq!(
///     i32::try_from(&Rational::from_str("-1000000000000").unwrap()),
///     Err(SignedFromRationalError)
/// );
/// assert_eq!(
///     i32::try_from(&Rational::from_str("1000000000000").unwrap()),
///     Err(SignedFromRationalError)
/// );
/// assert_eq!(i32::try_from(&Rational::from_signeds(22, 7)), Err(SignedFromRationalError));
/// ```
///
/// # convertible_from
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert_eq!(u32::convertible_from(&Rational::from(123)), true);
/// assert_eq!(u32::convertible_from(&Rational::from(-123)), false);
/// assert_eq!(u32::convertible_from(&Rational::from_str("1000000000000").unwrap()), false);
/// assert_eq!(u32::convertible_from(&Rational::from_signeds(22, 7)), false);
///
/// assert_eq!(i32::convertible_from(&Rational::from(123)), true);
/// assert_eq!(i32::convertible_from(&Rational::from(-123)), true);
/// assert_eq!(i32::convertible_from(&Rational::from_str("-1000000000000").unwrap()), false);
/// assert_eq!(i32::convertible_from(&Rational::from_str("1000000000000").unwrap()), false);
/// assert_eq!(i32::convertible_from(&Rational::from_signeds(22, 7)), false);
/// ```
///
/// # rounding_from
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_q::Rational;
///
/// assert_eq!(u32::rounding_from(&Rational::from(123), RoundingMode::Exact), 123);
///
/// assert_eq!(u32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Floor), 3);
/// assert_eq!(u32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Down), 3);
/// assert_eq!(u32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Ceiling), 4);
/// assert_eq!(u32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Up), 4);
/// assert_eq!(u32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Nearest), 3);
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
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Floor), 3);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Down), 3);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Ceiling), 4);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Up), 4);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Nearest), 3);
///
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Floor), -4);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Down), -3);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Ceiling), -3);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Up), -4);
/// assert_eq!(i32::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Nearest), -3);
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
/// Implementations of traits for converting [`Rational`](crate::Rational)s to and from
/// [`String`]s.
pub mod string;
/// Functions for extracting or referencing the numerator and/or denominator of a
/// [`Rational`](crate::Rational).
pub mod to_numerator_and_denominator;
/// Various traits for performing arithmetic operations on numbers.
pub mod traits;
