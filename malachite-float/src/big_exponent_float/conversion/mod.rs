// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::{Float, InnerFloat};
use malachite_base::num::basic::traits::{Infinity as _, NaN as _, NegativeInfinity, Zero as _};
use malachite_base::num::conversion::traits::SaturatingFrom;

impl BigExponentFloat {
    // The `Float` with the sign and significand of `self`, at `self`'s precision, and with the
    // exponent clamped to `Float`'s range. A conversion whose result depends only on the sign, on
    // the significand, and on which side of thresholds well inside `Float`'s range the exponent
    // lies, such as a conversion to a primitive integer or float, gives the same result for both.
    // Zero maps to positive zero.
    pub(crate) fn to_float_clamped(&self) -> Float {
        match self {
            Self(NaN) => Float::NAN,
            Self(Infinity { sign: true }) => Float::INFINITY,
            Self(Infinity { sign: false }) => Float::NEGATIVE_INFINITY,
            Self(Zero) => Float::ZERO,
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => Float(InnerFloat::Finite {
                sign: *sign,
                exponent: i32::saturating_from(exponent)
                    .clamp(Float::MIN_EXPONENT, Float::MAX_EXPONENT),
                precision: self.get_prec().unwrap(),
                significand: significand.clone(),
            }),
        }
    }

    // Like `to_float_clamped`, but taking `self` by value.
    pub(crate) fn into_float_clamped(self) -> Float {
        let precision = self.get_prec();
        match self {
            Self(NaN) => Float::NAN,
            Self(Infinity { sign: true }) => Float::INFINITY,
            Self(Infinity { sign: false }) => Float::NEGATIVE_INFINITY,
            Self(Zero) => Float::ZERO,
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => Float(InnerFloat::Finite {
                sign,
                exponent: i32::saturating_from(&exponent)
                    .clamp(Float::MIN_EXPONENT, Float::MAX_EXPONENT),
                precision: precision.unwrap(),
                significand,
            }),
        }
    }
}

/// Functions for converting a [`Float`] to a [`BigExponentFloat`].
pub mod big_exponent_float_from_float;
/// Functions for converting a [`BigExponentFloat`] to a [`Float`], exactly or with rounding.
pub mod float_from_big_exponent_float;
/// Functions for approximating a real number, given its bits, with a [`BigExponentFloat`].
pub mod from_bits;
/// Functions for approximating a real number, given its digits, with a [`BigExponentFloat`].
pub mod from_digits;
/// Functions for converting a [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger)
/// to a [`BigExponentFloat`].
pub mod from_gaussian_integer;
/// Functions for converting a
/// [`GaussianRational`](malachite_q::gaussian_rational::GaussianRational) to a
/// [`BigExponentFloat`].
pub mod from_gaussian_rational;
/// Functions for converting an [`Integer`](malachite_nz::integer::Integer) to a
/// [`BigExponentFloat`].
pub mod from_integer;
/// Functions for converting a [`Natural`](malachite_nz::natural::Natural) to a
/// [`BigExponentFloat`].
pub mod from_natural;
/// Functions for converting a primitive float to a [`BigExponentFloat`].
///
/// # from
/// ```
/// use malachite_float::BigExponentFloat;
///
/// assert_eq!(format!("{:?}", BigExponentFloat::from(1.5f64)), "0x1.8");
/// assert_eq!(format!("{:?}", BigExponentFloat::from(-0.0f32)), "0x0.0");
/// assert_eq!(format!("{:?}", BigExponentFloat::from(f64::NAN)), "NaN");
/// ```
pub mod from_primitive_float;
/// Functions for converting a primitive integer to a [`BigExponentFloat`].
///
/// # from
/// ```
/// use malachite_float::BigExponentFloat;
///
/// assert_eq!(format!("{:?}", BigExponentFloat::from(0u8)), "0x0.0");
/// assert_eq!(format!("{:?}", BigExponentFloat::from(12u32)), "0xc.0");
/// assert_eq!(format!("{:?}", BigExponentFloat::from(-12i64)), "-0xc.0");
/// assert_eq!(
///     format!("{:?}", BigExponentFloat::from(u128::MAX)),
///     "0xffffffffffffffffffffffffffffffff.0"
/// );
/// ```
pub mod from_primitive_int;
/// Functions for converting a [`Rational`](malachite_q::Rational) to a [`BigExponentFloat`],
/// exactly or with rounding.
pub mod from_rational;
/// Functions for converting a [`BigExponentFloat`] to a
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
pub mod gaussian_integer_from_big_exponent_float;
/// Functions for converting a [`BigExponentFloat`] to a
/// [`GaussianRational`](malachite_q::gaussian_rational::GaussianRational).
pub mod gaussian_rational_from_big_exponent_float;
/// Functions for converting a [`BigExponentFloat`] to an
/// [`Integer`](malachite_nz::integer::Integer).
pub mod integer_from_big_exponent_float;
/// Functions for determining whether a [`BigExponentFloat`] is a Gaussian integer.
pub mod is_gaussian_integer;
/// Functions for determining whether a [`BigExponentFloat`] is an integer.
pub mod is_integer;
/// Functions for determining whether a [`BigExponentFloat`] is a real number.
pub mod is_real;
/// Functions for getting and setting the mantissa and exponent of a [`BigExponentFloat`].
///
/// # raw_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
/// use malachite_float::{BigExponentFloat, Float};
///
/// let (m, e) = BigExponentFloat::from(Float::from(-6.0)).raw_mantissa_and_exponent();
/// assert_eq!(&m >> m.trailing_zeros().unwrap(), 3);
/// assert_eq!(e, 3);
/// ```
///
/// # from_raw_mantissa_and_exponent
/// ```
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
/// use malachite_float::BigExponentFloat;
/// use malachite_nz::integer::Integer;
///
/// let m = BigExponentFloat::ONE.to_significand().unwrap();
/// let x = BigExponentFloat::from_raw_mantissa_and_exponent(m, Integer::from(3));
/// assert_eq!(format!("{x:?}"), "0x4.0");
/// ```
///
/// # integer_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
/// use malachite_float::{BigExponentFloat, Float};
///
/// let (m, e) = BigExponentFloat::from(Float::from(-0.75)).integer_mantissa_and_exponent();
/// assert_eq!(m, 3);
/// assert_eq!(e, -2);
/// ```
///
/// # from_integer_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
/// use malachite_float::BigExponentFloat;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// let x = BigExponentFloat::from_integer_mantissa_and_exponent(
///     Natural::from(3u32),
///     Integer::from(-2),
/// )
/// .unwrap();
/// assert_eq!(format!("{x:?}"), "0x0.c");
/// ```
///
/// # sci_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_float::{BigExponentFloat, Float};
///
/// let (m, e) = BigExponentFloat::from(Float::from(-6.0)).sci_mantissa_and_exponent();
/// assert_eq!(format!("{m:?}"), "0x1.8");
/// assert_eq!(e, 2);
/// ```
///
/// # from_sci_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_float::{BigExponentFloat, Float};
/// use malachite_nz::integer::Integer;
///
/// let m = BigExponentFloat::from(Float::from(1.5));
/// let x = BigExponentFloat::from_sci_mantissa_and_exponent(m, Integer::from(2)).unwrap();
/// assert_eq!(format!("{x:?}"), "0x6.0");
/// let m = BigExponentFloat::from(Float::from(3.0));
/// assert!(BigExponentFloat::from_sci_mantissa_and_exponent(m, Integer::from(2)).is_none());
/// ```
pub mod mantissa_and_exponent;
/// Functions for converting a [`BigExponentFloat`] to a
/// [`Natural`](malachite_nz::natural::Natural).
pub mod natural_from_big_exponent_float;
/// Functions for converting a [`BigExponentFloat`] to a primitive float.
///
/// # rounding_from
/// ```
/// use malachite_base::num::conversion::traits::{IntegerMantissaAndExponent, RoundingFrom};
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::{BigExponentFloat, Float};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::cmp::Ordering::*;
///
/// let x = BigExponentFloat::from(Float::from(1.0f64 / 3.0));
/// assert_eq!(f32::rounding_from(&x, Floor), (0.3333333, Less));
/// assert_eq!(f32::rounding_from(&x, Ceiling), (0.33333334, Greater));
///
/// // A value too large for any `Float` still rounds as a large value should.
/// let big = BigExponentFloat::from_integer_mantissa_and_exponent(
///     Natural::from(1u32),
///     Integer::from(1u64 << 40),
/// )
/// .unwrap();
/// assert_eq!(f64::rounding_from(&big, Nearest), (f64::INFINITY, Greater));
/// assert_eq!(f64::rounding_from(&big, Down), (f64::MAX, Less));
/// ```
///
/// # try_from
/// ```
/// use malachite_float::float::conversion::primitive_float_from_float::FloatFromFloatError;
/// use malachite_float::{BigExponentFloat, Float};
///
/// assert_eq!(
///     f32::try_from(BigExponentFloat::from(Float::from(1.5))),
///     Ok(1.5)
/// );
/// assert_eq!(
///     f32::try_from(BigExponentFloat::from(Float::from(1.0f64 / 3.0))),
///     Err(FloatFromFloatError::Inexact)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_float::{BigExponentFloat, Float};
///
/// assert!(f32::convertible_from(&BigExponentFloat::from(Float::from(
///     1.5
/// ))));
/// assert!(!f32::convertible_from(&BigExponentFloat::from(
///     Float::from(1.0f64 / 3.0)
/// )));
/// ```
pub mod primitive_float_from_big_exponent_float;
/// Functions for converting a [`BigExponentFloat`] to a primitive integer.
///
/// # rounding_from
/// ```
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::{BigExponentFloat, Float};
/// use std::cmp::Ordering::*;
///
/// let x = BigExponentFloat::from(Float::from(-2.5));
/// assert_eq!(i8::rounding_from(&x, Floor), (-3, Less));
/// assert_eq!(i8::rounding_from(&x, Nearest), (-2, Greater));
/// assert_eq!(u8::rounding_from(&x, Ceiling), (0, Greater));
/// ```
///
/// # try_from
/// ```
/// use malachite_base::num::conversion::from::UnsignedFromFloatError::*;
/// use malachite_float::{BigExponentFloat, Float};
///
/// assert_eq!(
///     u8::try_from(BigExponentFloat::from(Float::from(200.0))),
///     Ok(200)
/// );
/// assert_eq!(
///     u8::try_from(BigExponentFloat::from(Float::from(300.0))),
///     Err(FloatNonIntegerOrOutOfRange)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_float::{BigExponentFloat, Float};
///
/// assert!(u8::convertible_from(&BigExponentFloat::from(Float::from(
///     200.0
/// ))));
/// assert!(!u8::convertible_from(&BigExponentFloat::from(Float::from(
///     2.5
/// ))));
/// ```
pub mod primitive_int_from_big_exponent_float;
/// Functions for converting a [`BigExponentFloat`] to a [`Rational`](malachite_q::Rational).
pub mod rational_from_big_exponent_float;
#[cfg(feature = "serde")]
pub mod serde;
/// Functions for converting a [`BigExponentFloat`] to a string, and for parsing one, in a
/// power-of-2 base.
pub mod string;
