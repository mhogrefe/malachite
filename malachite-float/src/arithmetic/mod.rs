// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Absolute value of [`Float`](super::Float)s.
pub mod abs;
/// Addition of [`Float`](super::Float)s, and of [`Float`](super::Float)s with
/// [`Rational`](malachite_q::Rational)s.
pub mod add;
/// Division of [`Float`](super::Float)s, of [`Float`](super::Float)s by
/// [`Rational`](malachite_q::Rational)s, and of [`Rational`](malachite_q::Rational)s by
/// [`Float`](super::Float)s.
pub mod div;
/// An implementations of [`IsPowerOf2`](malachite_base::num::arithmetic::traits::IsPowerOf2), a
/// trait for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
/// Multiplication of [`Float`](super::Float)s, and of [`Float`](super::Float)s with
/// [`Rational`](malachite_q::Rational)s.
pub mod mul;
/// Negation of [`Float`](super::Float)s.
pub mod neg;
/// Implementations of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait for
/// computing a power of 2.
pub mod power_of_2;
/// Implementations of [`Reciprocal`](malachite_base::num::arithmetic::traits::Reciprocal) and
/// [`ReciprocalAssign`](malachite_base::num::arithmetic::traits::ReciprocalAssign), traits for
/// computing the reciprocal of a number.
pub mod reciprocal;
/// Left-shifting a [`Float`](super::Float) (multiplying it by a power of 2).
///
/// # shl
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// assert_eq!(Float::ZERO << 10, 0);
/// assert_eq!(Float::INFINITY << 10, Float::INFINITY);
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) << 10u8).to_string(),
///     "3216.990877275948"
/// );
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) << -10i8).to_string(),
///     "0.003067961575771282"
/// );
///
/// assert_eq!(&Float::ZERO << 10, 0);
/// assert_eq!(&Float::INFINITY << 10, Float::INFINITY);
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) << 10u8).to_string(),
///     "3216.990877275948"
/// );
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) << -10i8).to_string(),
///     "0.003067961575771282"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// let mut x = Float::ZERO;
/// x <<= 10;
/// assert_eq!(x, 0);
///
/// let mut x = Float::INFINITY;
/// x <<= 10;
/// assert_eq!(x, Float::INFINITY);
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x <<= 10;
/// assert_eq!(x.to_string(), "3216.990877275948");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x <<= -10;
/// assert_eq!(x.to_string(), "0.003067961575771282");
/// ```
pub mod shl;
/// Implementations of [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound) and
/// [`ShlRoundAssign`](malachite_base::num::arithmetic::traits::ShlRoundAssign), traits for
/// multiplying a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode). For [`Float`](super::Float)s,
/// rounding is only necessary in the cases of overflow and underflow.
///
/// # shl_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.990877275948");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "0.003067961575771282");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "too_big");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.990877275948");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "0.003067961575771282");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "too_big");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(10u8, Nearest), Equal);
/// assert_eq!(x.to_string(), "3216.990877275948");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(-10i8, Nearest), Equal);
/// assert_eq!(x.to_string(), "0.003067961575771282");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(u32::MAX, Floor), Less);
/// assert_eq!(x.to_string(), "too_big");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(u32::MAX, Ceiling), Greater);
/// assert_eq!(x.to_string(), "Infinity");
/// ```
pub mod shl_round;
/// Right-shifting a [`Float`](super::Float) (dividing it by a power of 2).
///
/// # shr
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// assert_eq!(Float::ZERO >> 10, 0);
/// assert_eq!(Float::INFINITY >> 10, Float::INFINITY);
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) >> 10u8).to_string(),
///     "0.003067961575771282"
/// );
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) >> -10i8).to_string(),
///     "3216.990877275948"
/// );
///
/// assert_eq!(&Float::ZERO >> 10, 0);
/// assert_eq!(&Float::INFINITY >> 10, Float::INFINITY);
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) >> 10u8).to_string(),
///     "0.003067961575771282"
/// );
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) >> -10i8).to_string(),
///     "3216.990877275948"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// let mut x = Float::ZERO;
/// x >>= 10;
/// assert_eq!(x, 0);
///
/// let mut x = Float::INFINITY;
/// x >>= 10;
/// assert_eq!(x, Float::INFINITY);
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x >>= 10;
/// assert_eq!(x.to_string(), "0.003067961575771282");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x >>= -10;
/// assert_eq!(x.to_string(), "3216.990877275948");
/// ```
pub mod shr;
/// Implementations of [`ShlRound`](malachite_base::num::arithmetic::traits::ShrRound) and
/// [`ShrRoundAssign`](malachite_base::num::arithmetic::traits::ShrRoundAssign), traits for dividing
/// a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode). For [`Float`](super::Float)s,
/// rounding is only necessary in the cases of overflow and underflow.
///
/// # shr_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "0.003067961575771282");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.990877275948");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "too_small");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "0.003067961575771282");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.990877275948");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "too_small");
/// assert_eq!(o, Greater);
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(10u8, Nearest), Equal);
/// assert_eq!(x.to_string(), "0.003067961575771282");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(-10i8, Nearest), Equal);
/// assert_eq!(x.to_string(), "3216.990877275948");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(u32::MAX, Floor), Less);
/// assert_eq!(x.to_string(), "0.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(u32::MAX, Ceiling), Greater);
/// assert_eq!(x.to_string(), "too_small");
/// ```
pub mod shr_round;
/// An implementation of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
/// Squaring of [`Float`](super::Float)s.
pub mod square;
/// Subtraction of [`Float`](super::Float)s, of [`Float`](super::Float)s by
/// [`Rational`](malachite_q::Rational)s, and of [`Rational`](malachite_q::Rational)s by
/// [`Float`](super::Float)s.
pub mod sub;
