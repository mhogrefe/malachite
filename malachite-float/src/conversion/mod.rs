// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of the [`From`] trait for converting an
/// [`Integer`](malachite_nz::integer::Integer) to a [`Float`](crate::Float).
pub mod from_integer;
/// Implementations of the [`From`] trait for converting an
/// [`Natural`](malachite_nz::natural::Natural) to a [`Float`](crate::Float).
pub mod from_natural;
/// Various functions and implementations of the [`From`] trait for converting a primitive float to
/// a [`Float`](crate::Float).
///
/// # from
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_float::Float;
///
/// assert_eq!(Float::from(f64::NAN).to_string(), "NaN");
/// assert_eq!(Float::from(f64::INFINITY).to_string(), "Infinity");
/// assert_eq!(Float::from(f64::NEGATIVE_INFINITY).to_string(), "-Infinity");
/// assert_eq!(Float::from(0.0).to_string(), "0.0");
/// assert_eq!(Float::from(-0.0).to_string(), "-0.0");
/// assert_eq!(Float::from(123.0).to_string(), "123.0");
/// assert_eq!(Float::from(-123.0).to_string(), "-123.0");
/// ```
///
/// # from_primitive_float_prec
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (x, o) = Float::from_primitive_float_prec(f64::NAN, 4);
/// assert_eq!(x.to_string(), "NaN");
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_primitive_float_prec(1.0 / 3.0, 4);
/// assert_eq!(x.to_string(), "0.34");
/// assert_eq!(o, Greater);
///
/// let (x, o) = Float::from_primitive_float_prec(123.0, 4);
/// assert_eq!(x.to_string(), "1.2e2");
/// assert_eq!(o, Less);
/// ```
///
/// # from_primitive_float_prec_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (x, o) = Float::from_primitive_float_prec_round(f64::NAN, 4, Floor);
/// assert_eq!(x.to_string(), "NaN");
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_primitive_float_prec_round(1.0 / 3.0, 4, Floor);
/// assert_eq!(x.to_string(), "0.31");
/// assert_eq!(o, Less);
///
/// let (x, o) = Float::from_primitive_float_prec_round(1.0 / 3.0, 4, Ceiling);
/// assert_eq!(x.to_string(), "0.34");
/// assert_eq!(o, Greater);
///
/// let (x, o) = Float::from_primitive_float_prec_round(1.0 / 3.0, 4, Nearest);
/// assert_eq!(x.to_string(), "0.34");
/// assert_eq!(o, Greater);
/// ```
pub mod from_primitive_float;
/// Various functions and implementations of the [`From`] trait for converting a primitive integer
/// to a [`Float`](crate::Float).
///
/// # from
/// ```
/// use malachite_float::Float;
///
/// assert_eq!(Float::from(0u32).to_string(), "0.0");
/// assert_eq!(Float::from(123u32).to_string(), "123.0");
/// assert_eq!(Float::from(123u32).get_prec(), Some(7));
///
/// assert_eq!(Float::from(0i32).to_string(), "0.0");
/// assert_eq!(Float::from(123i32).to_string(), "123.0");
/// assert_eq!(Float::from(123i32).get_prec(), Some(7));
/// assert_eq!(Float::from(-123i32).to_string(), "-123.0");
/// ```
///
/// # from_unsigned_prec
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (x, o) = Float::from_unsigned_prec(0u32, 10);
/// assert_eq!(x.to_string(), "0.0");
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_unsigned_prec(123u32, 20);
/// assert_eq!(x.to_string(), "123.0");
/// assert_eq!(x.get_prec(), Some(20));
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_unsigned_prec(123u32, 4);
/// assert_eq!(x.to_string(), "1.2e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Less);
/// ```
///
/// # from_signed_prec
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (x, o) = Float::from_signed_prec(0i32, 10);
/// assert_eq!(x.to_string(), "0.0");
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_signed_prec(123i32, 20);
/// assert_eq!(x.to_string(), "123.0");
/// assert_eq!(x.get_prec(), Some(20));
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_signed_prec(123i32, 4);
/// assert_eq!(x.to_string(), "1.2e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Less);
///
/// let (x, o) = Float::from_signed_prec(-123i32, 20);
/// assert_eq!(x.to_string(), "-123.0");
/// assert_eq!(x.get_prec(), Some(20));
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_signed_prec(-123i32, 4);
/// assert_eq!(x.to_string(), "-1.2e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Greater);
/// ```
///
/// # from_unsigned_prec_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (x, o) = Float::from_unsigned_prec_round(0u32, 10, Exact);
/// assert_eq!(x.to_string(), "0.0");
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_unsigned_prec_round(123u32, 20, Exact);
/// assert_eq!(x.to_string(), "123.0");
/// assert_eq!(x.get_prec(), Some(20));
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_unsigned_prec_round(123u32, 4, Floor);
/// assert_eq!(x.to_string(), "1.2e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Less);
///
/// let (x, o) = Float::from_unsigned_prec_round(123u32, 4, Ceiling);
/// assert_eq!(x.to_string(), "1.3e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Greater);
/// ```
///
/// # from_signed_prec_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (x, o) = Float::from_signed_prec_round(0i32, 10, Exact);
/// assert_eq!(x.to_string(), "0.0");
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_signed_prec_round(123i32, 20, Exact);
/// assert_eq!(x.to_string(), "123.0");
/// assert_eq!(x.get_prec(), Some(20));
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_signed_prec_round(123i32, 4, Floor);
/// assert_eq!(x.to_string(), "1.2e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Less);
///
/// let (x, o) = Float::from_signed_prec_round(123i32, 4, Ceiling);
/// assert_eq!(x.to_string(), "1.3e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Greater);
///
/// let (x, o) = Float::from_signed_prec_round(-123i32, 20, Exact);
/// assert_eq!(x.to_string(), "-123.0");
/// assert_eq!(x.get_prec(), Some(20));
/// assert_eq!(o, Equal);
///
/// let (x, o) = Float::from_signed_prec_round(-123i32, 4, Floor);
/// assert_eq!(x.to_string(), "-1.3e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Less);
///
/// let (x, o) = Float::from_signed_prec_round(-123i32, 4, Ceiling);
/// assert_eq!(x.to_string(), "-1.2e2");
/// assert_eq!(x.get_prec(), Some(4));
/// assert_eq!(o, Greater);
/// ```
pub mod from_primitive_int;
/// Implementations of the [`From`] trait for converting a [`Rational`](malachite_q::Rational) to a
/// [`Float`](crate::Float).
pub mod from_rational;
/// Implementations of traits for converting a [`Float`](crate::Float) to an
/// [`Integer`](malachite_nz::integer::Integer).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
pub mod integer_from_float;
/// An implementation of [`IsInteger`](malachite_base::num::conversion::traits::IsInteger), a trait
/// for determining whether a number is an integer.
pub mod is_integer;
/// Implementations of traits for converting [`Float`](crate::Float)s to and from
/// mantissa-and-exponent representations.
///
/// The traits are
/// [`RawMantissaAndExponent`](malachite_base::num::conversion::traits::RawMantissaAndExponent),
/// [`IntegerMantissaAndExponent`](malachite_base::num::conversion::traits::IntegerMantissaAndExponent),
/// and [`SciMantissaAndExponent`](malachite_base::num::conversion::traits::SciMantissaAndExponent).
///
/// Here are some examples of the macro-generated functions:
///
/// # sci_mantissa_and_exponent
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::Float;
/// use malachite_nz::natural::Natural;
/// use malachite_q::Rational;
///
/// let (m, e): (f64, i32) = (&Float::ONE).sci_mantissa_and_exponent();
/// assert_eq!(NiceFloat(m), NiceFloat(1.0));
/// assert_eq!(e, 0);
///
/// let (m, e): (f64, i32) = (&Float::from(std::f64::consts::PI)).sci_mantissa_and_exponent();
/// assert_eq!(NiceFloat(m), NiceFloat(std::f64::consts::FRAC_PI_2));
/// assert_eq!(e, 1);
///
/// let (m, e): (f64, i32) =
///     (&Float::from(Natural::from(3u32).pow(50u64))).sci_mantissa_and_exponent();
/// assert_eq!(NiceFloat(m), NiceFloat(1.187662594419065));
/// assert_eq!(e, 79);
///
/// let (m, e): (f64, i32) = (&Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0)
///     .sci_mantissa_and_exponent();
/// assert_eq!(NiceFloat(m), NiceFloat(1.6839799530592128));
/// assert_eq!(e, -80);
/// ```
///
/// # from_sci_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_float::Float;
///
/// assert_eq!(
///     <&Float as SciMantissaAndExponent<f64, _, _>>::from_sci_mantissa_and_exponent(1.0, 0)
///         .unwrap()
///         .to_string(),
///     "1.0"
/// );
/// assert_eq!(
///     <&Float as SciMantissaAndExponent<f64, _, _>>::from_sci_mantissa_and_exponent(
///         std::f64::consts::FRAC_PI_2,
///         1
///     )
///     .unwrap()
///     .to_string(),
///     "3.141592653589793"
/// );
/// assert_eq!(
///     <&Float as SciMantissaAndExponent<f64, _, _>>::from_sci_mantissa_and_exponent(
///         1.187662594419065,
///         79
///     )
///     .unwrap()
///     .to_string(),
///     "7.178979876918526e23"
/// );
/// assert_eq!(
///     <&Float as SciMantissaAndExponent<f64, _, _>>::from_sci_mantissa_and_exponent(
///         1.6839799530592128,
///         -80
///     )
///     .unwrap()
///     .to_string(),
///     "1.392955569098538e-24"
/// );
/// ```
pub mod mantissa_and_exponent;
/// Implementations of traits for converting a [`Float`](crate::Float) to a
/// [`Natural`](malachite_nz::natural::Natural).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
pub mod natural_from_float;
/// Functions and implementations of traits for converting a [`Float`](crate::Float) to a primitive
/// float.
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # convertible_from
/// ```
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_float::Float;
///
/// assert_eq!(f32::convertible_from(&Float::NAN), true);
/// assert_eq!(f32::convertible_from(&Float::INFINITY), true);
/// assert_eq!(f32::convertible_from(&Float::ZERO), true);
/// assert_eq!(f32::convertible_from(&Float::from(1.5)), true);
/// assert_eq!(f32::convertible_from(&Float::from(-1.5)), true);
/// assert_eq!(f32::convertible_from(&Float::from(123.0)), true);
/// assert_eq!(f32::convertible_from(&Float::from(-123.0)), true);
///
/// // Even though precision is high, the value is just 1.0 and can be converted
/// assert_eq!(f32::convertible_from(&Float::one_prec(100)), true);
///
/// let mut x = Float::one_prec(40);
/// x.increment();
///
/// // precision too high for f32
/// assert_eq!(f32::convertible_from(&x), false);
///
/// // but not for f64
/// assert_eq!(f64::convertible_from(&x), true);
///
/// assert_eq!(f32::convertible_from(&Float::power_of_2(100u64)), true);
/// assert_eq!(f32::convertible_from(&Float::power_of_2(1000u64)), false);
/// assert_eq!(f64::convertible_from(&Float::power_of_2(1000u64)), true);
/// assert_eq!(f64::convertible_from(&Float::power_of_2(10000u64)), false);
/// ```
///
/// # try_from
/// ```
/// use malachite_base::num::arithmetic::traits::PowerOf2;
/// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::conversion::primitive_float_from_float::FloatFromFloatError;
/// use malachite_float::Float;
///
/// assert_eq!(
///     NiceFloat(f32::try_from(Float::NAN).unwrap()),
///     NiceFloat(f32::NAN)
/// );
/// assert_eq!(f32::try_from(Float::INFINITY), Ok(f32::INFINITY));
/// assert_eq!(f32::try_from(Float::ZERO), Ok(0.0));
/// assert_eq!(f32::try_from(Float::from(1.5)), Ok(1.5));
/// assert_eq!(f32::try_from(Float::from(-1.5)), Ok(-1.5));
/// assert_eq!(f32::try_from(Float::from(123.0)), Ok(123.0));
/// assert_eq!(f32::try_from(Float::from(-123.0)), Ok(-123.0));
///
/// // Even though precision is high, the value is just 1.0 and can be converted
/// assert_eq!(f32::try_from(Float::one_prec(100)), Ok(1.0));
///
/// let mut x = Float::one_prec(40);
/// x.increment();
///
/// // precision too high for f32
/// assert_eq!(f32::try_from(x.clone()), Err(FloatFromFloatError::Inexact));
///
/// // but not for f64
/// assert_eq!(
///     NiceFloat(f64::try_from(x).unwrap()),
///     NiceFloat(1.000000000001819)
/// );
///
/// assert_eq!(
///     NiceFloat(f32::try_from(Float::power_of_2(100u64)).unwrap()),
///     NiceFloat(1.2676506e30)
/// );
/// assert_eq!(
///     f32::try_from(Float::power_of_2(1000u64)),
///     Err(FloatFromFloatError::Overflow)
/// );
/// assert_eq!(
///     NiceFloat(f64::try_from(Float::power_of_2(1000u64)).unwrap()),
///     NiceFloat(1.0715086071862673e301)
/// );
/// assert_eq!(
///     f64::try_from(Float::power_of_2(10000u64)),
///     Err(FloatFromFloatError::Overflow)
/// );
///
/// assert_eq!(
///     NiceFloat(f32::try_from(&Float::NAN).unwrap()),
///     NiceFloat(f32::NAN)
/// );
/// assert_eq!(f32::try_from(&Float::INFINITY), Ok(f32::INFINITY));
/// assert_eq!(f32::try_from(&Float::ZERO), Ok(0.0));
/// assert_eq!(f32::try_from(&Float::from(1.5)), Ok(1.5));
/// assert_eq!(f32::try_from(&Float::from(-1.5)), Ok(-1.5));
/// assert_eq!(f32::try_from(&Float::from(123.0)), Ok(123.0));
/// assert_eq!(f32::try_from(&Float::from(-123.0)), Ok(-123.0));
///
/// // Even though precision is high, the value is just 1.0 and can be converted
/// assert_eq!(f32::try_from(&Float::one_prec(100)), Ok(1.0));
///
/// let mut x = Float::one_prec(40);
/// x.increment();
///
/// // precision too high for f32
/// assert_eq!(f32::try_from(&x), Err(FloatFromFloatError::Inexact));
///
/// // but not for f64
/// assert_eq!(
///     NiceFloat(f64::try_from(&x).unwrap()),
///     NiceFloat(1.000000000001819)
/// );
///
/// assert_eq!(
///     NiceFloat(f32::try_from(&Float::power_of_2(100u64)).unwrap()),
///     NiceFloat(1.2676506e30)
/// );
/// assert_eq!(
///     f32::try_from(&Float::power_of_2(1000u64)),
///     Err(FloatFromFloatError::Overflow)
/// );
/// assert_eq!(
///     NiceFloat(f64::try_from(&Float::power_of_2(1000u64)).unwrap()),
///     NiceFloat(1.0715086071862673e301)
/// );
/// assert_eq!(
///     f64::try_from(&Float::power_of_2(10000u64)),
///     Err(FloatFromFloatError::Overflow)
/// );
/// ```
///
/// # rounding_from
/// ```
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use malachite_q::Rational;
/// use std::cmp::Ordering::*;
///
/// let f = Float::from_rational_prec(Rational::from_signeds(1, 3), 100).0;
///
/// let (x, o) = f32::rounding_from(f.clone(), Floor);
/// assert_eq!(NiceFloat(x), NiceFloat(0.3333333));
/// assert_eq!(o, Less);
///
/// let (x, o) = f32::rounding_from(f.clone(), Ceiling);
/// assert_eq!(NiceFloat(x), NiceFloat(0.33333334));
/// assert_eq!(o, Greater);
///
/// let (x, o) = f32::rounding_from(f.clone(), Nearest);
/// assert_eq!(NiceFloat(x), NiceFloat(0.33333334));
/// assert_eq!(o, Greater);
///
/// let (x, o) = f32::rounding_from(&f, Floor);
/// assert_eq!(NiceFloat(x), NiceFloat(0.3333333));
/// assert_eq!(o, Less);
///
/// let (x, o) = f32::rounding_from(&f, Ceiling);
/// assert_eq!(NiceFloat(x), NiceFloat(0.33333334));
/// assert_eq!(o, Greater);
///
/// let (x, o) = f32::rounding_from(&f, Nearest);
/// assert_eq!(NiceFloat(x), NiceFloat(0.33333334));
/// assert_eq!(o, Greater);
/// ```
pub mod primitive_float_from_float;
/// Functions and implementations of traits for converting a [`Float`](crate::Float) to a primitive
/// integer.
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # rounding_from
/// ```
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(u8::rounding_from(Float::from(1.5), Floor), (1, Less));
/// assert_eq!(u8::rounding_from(Float::from(1.5), Ceiling), (2, Greater));
/// assert_eq!(u8::rounding_from(Float::from(1.5), Nearest), (2, Greater));
///
/// assert_eq!(u8::rounding_from(Float::from(256.0), Down), (255, Less));
/// assert_eq!(u8::rounding_from(Float::from(256.0), Floor), (255, Less));
/// assert_eq!(u8::rounding_from(Float::from(256.0), Nearest), (255, Less));
///
/// assert_eq!(u8::rounding_from(Float::from(-123.0), Down), (0, Greater));
/// assert_eq!(
///     u8::rounding_from(Float::from(-123.0), Ceiling),
///     (0, Greater)
/// );
/// assert_eq!(
///     u8::rounding_from(Float::from(-123.0), Nearest),
///     (0, Greater)
/// );
///
/// assert_eq!(i8::rounding_from(Float::from(1.5), Floor), (1, Less));
/// assert_eq!(i8::rounding_from(Float::from(1.5), Ceiling), (2, Greater));
/// assert_eq!(i8::rounding_from(Float::from(1.5), Nearest), (2, Greater));
///
/// assert_eq!(i8::rounding_from(Float::from(-1.5), Floor), (-2, Less));
/// assert_eq!(i8::rounding_from(Float::from(-1.5), Ceiling), (-1, Greater));
/// assert_eq!(i8::rounding_from(Float::from(-1.5), Nearest), (-2, Less));
///
/// assert_eq!(i8::rounding_from(Float::from(128.0), Down), (127, Less));
/// assert_eq!(i8::rounding_from(Float::from(128.0), Floor), (127, Less));
/// assert_eq!(i8::rounding_from(Float::from(128.0), Nearest), (127, Less));
///
/// assert_eq!(
///     i8::rounding_from(Float::from(-129.0), Down),
///     (-128, Greater)
/// );
/// assert_eq!(
///     i8::rounding_from(Float::from(-129.0), Ceiling),
///     (-128, Greater)
/// );
/// assert_eq!(
///     i8::rounding_from(Float::from(-129.0), Nearest),
///     (-128, Greater)
/// );
///
/// assert_eq!(u8::rounding_from(&Float::from(1.5), Floor), (1, Less));
/// assert_eq!(u8::rounding_from(&Float::from(1.5), Ceiling), (2, Greater));
/// assert_eq!(u8::rounding_from(&Float::from(1.5), Nearest), (2, Greater));
///
/// assert_eq!(u8::rounding_from(&Float::from(256.0), Down), (255, Less));
/// assert_eq!(u8::rounding_from(&Float::from(256.0), Floor), (255, Less));
/// assert_eq!(u8::rounding_from(&Float::from(256.0), Nearest), (255, Less));
///
/// assert_eq!(u8::rounding_from(&Float::from(-123.0), Down), (0, Greater));
/// assert_eq!(
///     u8::rounding_from(&Float::from(-123.0), Ceiling),
///     (0, Greater)
/// );
/// assert_eq!(
///     u8::rounding_from(&Float::from(-123.0), Nearest),
///     (0, Greater)
/// );
///
/// assert_eq!(i8::rounding_from(&Float::from(1.5), Floor), (1, Less));
/// assert_eq!(i8::rounding_from(&Float::from(1.5), Ceiling), (2, Greater));
/// assert_eq!(i8::rounding_from(&Float::from(1.5), Nearest), (2, Greater));
///
/// assert_eq!(i8::rounding_from(&Float::from(-1.5), Floor), (-2, Less));
/// assert_eq!(
///     i8::rounding_from(&Float::from(-1.5), Ceiling),
///     (-1, Greater)
/// );
/// assert_eq!(i8::rounding_from(&Float::from(-1.5), Nearest), (-2, Less));
///
/// assert_eq!(i8::rounding_from(&Float::from(128.0), Down), (127, Less));
/// assert_eq!(i8::rounding_from(&Float::from(128.0), Floor), (127, Less));
/// assert_eq!(i8::rounding_from(&Float::from(128.0), Nearest), (127, Less));
///
/// assert_eq!(
///     i8::rounding_from(&Float::from(-129.0), Down),
///     (-128, Greater)
/// );
/// assert_eq!(
///     i8::rounding_from(&Float::from(-129.0), Ceiling),
///     (-128, Greater)
/// );
/// assert_eq!(
///     i8::rounding_from(&Float::from(-129.0), Nearest),
///     (-128, Greater)
/// );
/// ```
///
/// # try_from
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
/// use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
/// use malachite_float::Float;
///
/// assert_eq!(u8::try_from(Float::ZERO).unwrap(), 0);
/// assert_eq!(u8::try_from(Float::from(123.0)).unwrap(), 123);
///
/// assert_eq!(
///     u8::try_from(Float::from(-123.0)),
///     Err(UnsignedFromFloatError::FloatNegative)
/// );
/// assert_eq!(
///     u8::try_from(Float::from(256.0)),
///     Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     u8::try_from(Float::from(1.5)),
///     Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     u8::try_from(Float::INFINITY),
///     Err(UnsignedFromFloatError::FloatInfiniteOrNan)
/// );
/// assert_eq!(
///     u8::try_from(Float::NAN),
///     Err(UnsignedFromFloatError::FloatInfiniteOrNan)
/// );
///
/// assert_eq!(i8::try_from(Float::ZERO).unwrap(), 0);
/// assert_eq!(i8::try_from(Float::from(123.0)).unwrap(), 123);
/// assert_eq!(i8::try_from(Float::from(-123.0)).unwrap(), -123);
///
/// assert_eq!(
///     i8::try_from(Float::from(128.0)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     i8::try_from(Float::from(-129.0)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     i8::try_from(Float::from(1.5)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     i8::try_from(Float::INFINITY),
///     Err(SignedFromFloatError::FloatInfiniteOrNan)
/// );
/// assert_eq!(
///     i8::try_from(Float::NAN),
///     Err(SignedFromFloatError::FloatInfiniteOrNan)
/// );
///
/// assert_eq!(u8::try_from(&Float::ZERO).unwrap(), 0);
/// assert_eq!(u8::try_from(&Float::from(123.0)).unwrap(), 123);
///
/// assert_eq!(
///     u8::try_from(&Float::from(-123.0)),
///     Err(UnsignedFromFloatError::FloatNegative)
/// );
/// assert_eq!(
///     u8::try_from(&Float::from(256.0)),
///     Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     u8::try_from(&Float::from(1.5)),
///     Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     u8::try_from(&Float::INFINITY),
///     Err(UnsignedFromFloatError::FloatInfiniteOrNan)
/// );
/// assert_eq!(
///     u8::try_from(&Float::NAN),
///     Err(UnsignedFromFloatError::FloatInfiniteOrNan)
/// );
///
/// assert_eq!(i8::try_from(&Float::ZERO).unwrap(), 0);
/// assert_eq!(i8::try_from(&Float::from(123.0)).unwrap(), 123);
/// assert_eq!(i8::try_from(&Float::from(-123.0)).unwrap(), -123);
///
/// assert_eq!(
///     i8::try_from(&Float::from(128.0)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     i8::try_from(&Float::from(-129.0)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     i8::try_from(&Float::from(1.5)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     i8::try_from(&Float::INFINITY),
///     Err(SignedFromFloatError::FloatInfiniteOrNan)
/// );
/// assert_eq!(
///     i8::try_from(&Float::NAN),
///     Err(SignedFromFloatError::FloatInfiniteOrNan)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_float::Float;
///
/// assert_eq!(u8::convertible_from(&Float::ZERO), true);
/// assert_eq!(u8::convertible_from(&Float::from(123.0)), true);
///
/// assert_eq!(u8::convertible_from(&Float::from(256.0)), false);
/// assert_eq!(u8::convertible_from(&Float::from(-123.0)), false);
/// assert_eq!(u8::convertible_from(&Float::from(1.5)), false);
/// assert_eq!(u8::convertible_from(&Float::INFINITY), false);
/// assert_eq!(u8::convertible_from(&Float::NAN), false);
///
/// assert_eq!(i8::convertible_from(&Float::ZERO), true);
/// assert_eq!(i8::convertible_from(&Float::from(123.0)), true);
/// assert_eq!(i8::convertible_from(&Float::from(-123.0)), true);
///
/// assert_eq!(i8::convertible_from(&Float::from(128.0)), false);
/// assert_eq!(i8::convertible_from(&Float::from(-129.0)), false);
/// assert_eq!(i8::convertible_from(&Float::from(1.5)), false);
/// assert_eq!(i8::convertible_from(&Float::INFINITY), false);
/// assert_eq!(i8::convertible_from(&Float::NAN), false);
/// ```
pub mod primitive_int_from_float;
/// Implementations of traits for converting a [`Float`](crate::Float) to a
/// [`Rational`](malachite_q::Rational).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
pub mod rational_from_float;
/// Implementations of traits for converting [`Float`](crate::Float)s to and from [`String`]s.
///
/// Warning: these implementations are unstable and will definitely be changed in the future.
pub mod string;
