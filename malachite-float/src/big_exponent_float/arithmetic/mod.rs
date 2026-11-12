// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Absolute value of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod abs;
/// Addition of [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given
/// precision.
pub mod add;
pub mod div;
/// Multiplication of [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given
/// precision.
pub mod mul;
/// Negation of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod neg;
pub mod reciprocal;
/// Left-shifting a [`BigExponentFloat`](super::BigExponentFloat) (multiplying it by a power of 2).
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
pub mod shl;
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
pub mod shr;
/// An implementation of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
pub mod sqrt;
/// Squaring of [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given
/// precision.
pub mod square;
/// Subtraction of [`BigExponentFloat`](super::BigExponentFloat)s, exact or rounded to a given
/// precision.
pub mod sub;
