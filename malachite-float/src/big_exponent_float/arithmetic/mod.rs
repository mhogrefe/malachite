// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Absolute value of [`BigExponentFloat`](super::BigExponentFloat)s. Addition of
/// [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given precision.
/// Multiplication of [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given
/// precision. Negation of [`BigExponentFloat`](super::BigExponentFloat)s. Left-shifting a
/// [`BigExponentFloat`](super::BigExponentFloat) (multiplying it by a power of 2).
///
/// # shl
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::BigExponentFloat;
///
/// assert_eq!(BigExponentFloat::ZERO << 10, BigExponentFloat::ZERO);
/// assert_eq!(BigExponentFloat::INFINITY << 10, BigExponentFloat::INFINITY);
/// assert_eq!(
///     format!("{:?}", BigExponentFloat::from(1.5) << 10u8),
///     "0x6.0E+2"
/// );
/// assert_eq!(
///     format!("{:?}", BigExponentFloat::from(1.5) << -10i8),
///     "0x0.006"
/// );
///
/// assert_eq!(&BigExponentFloat::ZERO << 10, BigExponentFloat::ZERO);
/// assert_eq!(
///     &BigExponentFloat::INFINITY << 10,
///     BigExponentFloat::INFINITY
/// );
/// assert_eq!(
///     format!("{:?}", &BigExponentFloat::from(1.5) << 10u8),
///     "0x6.0E+2"
/// );
/// assert_eq!(
///     format!("{:?}", &BigExponentFloat::from(1.5) << -10i8),
///     "0x0.006"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::BigExponentFloat;
///
/// let mut x = BigExponentFloat::ZERO;
/// x <<= 10;
/// assert_eq!(x, BigExponentFloat::ZERO);
///
/// let mut x = BigExponentFloat::INFINITY;
/// x <<= 10;
/// assert_eq!(x, BigExponentFloat::INFINITY);
///
/// let mut x = BigExponentFloat::from(1.5);
/// x <<= 10;
/// assert_eq!(format!("{x:?}"), "0x6.0E+2");
///
/// let mut x = BigExponentFloat::from(1.5);
/// x <<= -10;
/// assert_eq!(format!("{x:?}"), "0x0.006");
/// ```
/// Right-shifting a [`BigExponentFloat`](super::BigExponentFloat) (dividing it by a power of 2).
///
/// # shr
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::BigExponentFloat;
///
/// assert_eq!(BigExponentFloat::ZERO >> 10, BigExponentFloat::ZERO);
/// assert_eq!(BigExponentFloat::INFINITY >> 10, BigExponentFloat::INFINITY);
/// assert_eq!(
///     format!("{:?}", BigExponentFloat::from(1.5) >> 10u8),
///     "0x0.006"
/// );
/// assert_eq!(
///     format!("{:?}", BigExponentFloat::from(1.5) >> -10i8),
///     "0x6.0E+2"
/// );
///
/// assert_eq!(&BigExponentFloat::ZERO >> 10, BigExponentFloat::ZERO);
/// assert_eq!(
///     &BigExponentFloat::INFINITY >> 10,
///     BigExponentFloat::INFINITY
/// );
/// assert_eq!(
///     format!("{:?}", &BigExponentFloat::from(1.5) >> 10u8),
///     "0x0.006"
/// );
/// assert_eq!(
///     format!("{:?}", &BigExponentFloat::from(1.5) >> -10i8),
///     "0x6.0E+2"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::BigExponentFloat;
///
/// let mut x = BigExponentFloat::ZERO;
/// x >>= 10;
/// assert_eq!(x, BigExponentFloat::ZERO);
///
/// let mut x = BigExponentFloat::INFINITY;
/// x >>= 10;
/// assert_eq!(x, BigExponentFloat::INFINITY);
///
/// let mut x = BigExponentFloat::from(1.5);
/// x >>= 10;
/// assert_eq!(format!("{x:?}"), "0x0.006");
///
/// let mut x = BigExponentFloat::from(1.5);
/// x >>= -10;
/// assert_eq!(format!("{x:?}"), "0x6.0E+2");
/// ```
/// An implementation of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number. Squaring of [`BigExponentFloat`](super::BigExponentFloat)s,
/// exact or rounded to a given precision. Subtraction of
/// [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given precision.
pub mod abs;
pub mod abs_squared;
pub mod add;
pub mod add_mul;
pub mod add_rational;
pub mod average;
pub mod canonical_unit_i_pow;
pub mod canonicalize_gaussian_unit;
pub mod canonicalize_unit;
pub mod cbrt;
pub mod conjugate;
pub mod div;
pub mod div_rational;
pub mod dot;
pub mod fractional_part;
pub mod hypot;
pub mod is_power_of_2;
pub mod is_unit;
pub mod mul;
pub mod mul_add_mul;
pub mod mul_rational;
pub mod mul_sub_mul;
pub mod neg;
pub mod positive_difference;
pub mod pow;
pub mod power_of_2;
pub mod product;
pub mod rational_div_big_exponent_float;
pub mod reciprocal;
pub mod reciprocal_sqrt;
pub mod rem;
pub mod root;
pub mod round_to_integer;
pub mod shl;
pub mod shr;
pub mod sign;
pub mod sqrt;
pub mod square;
pub mod sub;
pub mod sub_mul;
pub mod sub_rational;
pub mod sum;
