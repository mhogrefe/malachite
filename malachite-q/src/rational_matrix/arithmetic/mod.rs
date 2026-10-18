// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// matrices of the same shape.
pub mod add;
/// Implementations of [`Mul`](core::ops::Mul) for multiplying a
/// [`RationalMatrix`](super::RationalMatrix) by a
/// [`RationalVector`](crate::rational_vector::RationalVector) on the right.
pub mod mul_vector;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a matrix.
pub mod neg;

/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying a [`RationalMatrix`](super::RationalMatrix) by a scalar on either side.
pub mod scalar_mul;
/// Left-shifting a matrix (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert_eq!(
///     (RationalMatrix::from_str("[]_{0,0}").unwrap() << 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() << 0u16).to_string(),
///     "[1/2, -3/4, 3]"
/// );
/// assert_eq!(
///     (RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() << 2u32).to_string(),
///     "[2, -3, 12]"
/// );
/// assert_eq!(
///     (&RationalMatrix::from_str("[1/8, 0]").unwrap() << 100u64).to_string(),
///     "[158456325028528675187087900672, 0]"
/// );
///
/// assert_eq!(
///     (RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() << 2i8).to_string(),
///     "[2, -3, 12]"
/// );
/// assert_eq!(
///     (&RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() << -2i64).to_string(),
///     "[1/8, -3/16, 3/4]"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// let mut a = RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap();
/// a <<= 2u32;
/// assert_eq!(a.to_string(), "[2, -3, 12]");
///
/// let mut a = RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap();
/// a <<= -2i32;
/// assert_eq!(a.to_string(), "[1/8, -3/16, 3/4]");
/// ```
pub mod shl;
/// Right-shifting a matrix (dividing it by a power of 2), by implementations of
/// [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert_eq!(
///     (RationalMatrix::from_str("[]_{0,0}").unwrap() >> 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() >> 0u16).to_string(),
///     "[1/2, -3/4, 3]"
/// );
/// assert_eq!(
///     (RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() >> 2u32).to_string(),
///     "[1/8, -3/16, 3/4]"
/// );
/// assert_eq!(
///     (&RationalMatrix::from_str("[12, 0]").unwrap() >> 100u64).to_string(),
///     "[3/316912650057057350374175801344, 0]"
/// );
///
/// assert_eq!(
///     (RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() >> 2i8).to_string(),
///     "[1/8, -3/16, 3/4]"
/// );
/// assert_eq!(
///     (&RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap() >> -2i64).to_string(),
///     "[2, -3, 12]"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// let mut a = RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap();
/// a >>= 2u32;
/// assert_eq!(a.to_string(), "[1/8, -3/16, 3/4]");
///
/// let mut a = RationalMatrix::from_str("[1/2, -3/4, 3]").unwrap();
/// a >>= -2i32;
/// assert_eq!(a.to_string(), "[2, -3, 12]");
/// ```
pub mod shr;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting matrices of the same shape.
pub mod sub;

/// Implementations of [`Mul`](core::ops::Mul) for multiplying a
/// [`RationalVector`](crate::rational_vector::RationalVector) by a
/// [`RationalMatrix`](super::RationalMatrix) on the right.
pub mod vector_mul;
