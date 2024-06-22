// Copyright © 2024 Mikhail Hogrefe
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
///     "3216.9908772759482"
/// );
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) << -10i8).to_string(),
///     "0.0030679615757712823"
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
/// assert_eq!(x.to_string(), "3216.9908772759482");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x <<= -10;
/// assert_eq!(x.to_string(), "0.0030679615757712823");
/// ```
pub mod shl;
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
///     "0.0030679615757712823"
/// );
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) >> -10i8).to_string(),
///     "3216.9908772759482"
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
/// assert_eq!(x.to_string(), "0.0030679615757712823");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x >>= -10;
/// assert_eq!(x.to_string(), "3216.9908772759482");
/// ```
pub mod shr;
/// An implementation of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
/// Squaring of [`Float`](super::Float)s.
pub mod square;
/// Subtraction of [`Float`](super::Float)s, and of [`Float`](super::Float)s with
/// [`Rational`](malachite_q::Rational)s.
pub mod sub;
