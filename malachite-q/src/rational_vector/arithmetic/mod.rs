// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// vectors.
pub mod add;
/// Adding a multiple of one vector to another, by implementations of
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign).
///
/// # add_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_q::Rational;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     RationalVector::from_str("()")
///         .unwrap()
///         .add_mul(RationalVector::from_str("()").unwrap(), Rational::from(5))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     RationalVector::from_str("(1/2, -2, 3)")
///         .unwrap()
///         .add_mul(
///             RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             Rational::ZERO
///         )
///         .to_string(),
///     "(1/2, -2, 3)"
/// );
/// assert_eq!(
///     RationalVector::from_str("(1/2, -2, 3)")
///         .unwrap()
///         .add_mul(
///             RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             Rational::ONE
///         )
///         .to_string(),
///     "(9/2, -5/3, -3)"
/// );
/// assert_eq!(
///     RationalVector::from_str("(1/2, -2, 3)")
///         .unwrap()
///         .add_mul(
///             RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             Rational::NEGATIVE_ONE
///         )
///         .to_string(),
///     "(-7/2, -7/3, 9)"
/// );
/// assert_eq!(
///     (&RationalVector::from_str("(1/2, -2, 3)").unwrap())
///         .add_mul(
///             &RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             &Rational::from_signeds(3, 2)
///         )
///         .to_string(),
///     "(13/2, -3/2, -6)"
/// );
/// ```
///
/// # add_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_q::Rational;
/// use malachite_q::rational_vector::RationalVector;
///
/// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
/// v.add_mul_assign(
///     RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///     Rational::from_signeds(3, 2),
/// );
/// assert_eq!(v.to_string(), "(13/2, -3/2, -6)");
/// ```
pub mod add_mul;
/// Implementations of [`AddMulShl`](malachite_base::num::arithmetic::traits::AddMulShl) and
/// [`AddMulShlAssign`](malachite_base::num::arithmetic::traits::AddMulShlAssign), for adding a
/// scalar multiple of one vector, shifted left, to another.
pub mod add_mul_shl;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart) and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a vector with its first nonzero element made positive.
pub mod canonical_primitive_part;
/// Implementations of
/// [`CanonicalizeSign`](malachite_base::num::arithmetic::traits::CanonicalizeSign) and
/// [`CanonicalizeSignAssign`](malachite_base::num::arithmetic::traits::CanonicalizeSignAssign),
/// which negate a vector if its pivot is negative.
pub mod canonicalize_sign;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which split a vector into a rational scalar and a vector of coprime integers.
pub mod content;
/// Implementations of [`EntrywiseAbs`](malachite_base::num::arithmetic::traits::EntrywiseAbs) and
/// [`EntrywiseAbsAssign`](malachite_base::num::arithmetic::traits::EntrywiseAbsAssign), which
/// replace every element of a vector by its absolute value.
pub mod entrywise_abs;
/// An implementation of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest height of any
/// element.
pub mod height;
/// An implementation of [`L1Norm`](malachite_base::num::arithmetic::traits::L1Norm), the sum of the
/// absolute values of the elements.
pub mod l1_norm;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a vector.
pub mod neg;
/// Implementations of [`Div`](core::ops::Div) and [`DivAssign`](core::ops::DivAssign), for dividing
/// a vector by a scalar.
pub mod scalar_div;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying a vector by a scalar, with the scalar on either side.
pub mod scalar_mul;
/// Left-shifting a vector (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     (RationalVector::from_str("()").unwrap() << 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (RationalVector::from_str("(1/2, -3/4, 3)").unwrap() << 0u16).to_string(),
///     "(1/2, -3/4, 3)"
/// );
/// assert_eq!(
///     (RationalVector::from_str("(1/2, -3/4, 3)").unwrap() << 2u32).to_string(),
///     "(2, -3, 12)"
/// );
/// assert_eq!(
///     (&RationalVector::from_str("(1/8, 0)").unwrap() << 100u64).to_string(),
///     "(158456325028528675187087900672, 0)"
/// );
///
/// assert_eq!(
///     (RationalVector::from_str("(1/2, -3/4, 3)").unwrap() << 2i8).to_string(),
///     "(2, -3, 12)"
/// );
/// assert_eq!(
///     (&RationalVector::from_str("(1/2, -3/4, 3)").unwrap() << -2i64).to_string(),
///     "(1/8, -3/16, 3/4)"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_vector::RationalVector;
///
/// let mut v = RationalVector::from_str("(1/2, -3/4, 3)").unwrap();
/// v <<= 2u32;
/// assert_eq!(v.to_string(), "(2, -3, 12)");
///
/// let mut v = RationalVector::from_str("(1/2, -3/4, 3)").unwrap();
/// v <<= -2i32;
/// assert_eq!(v.to_string(), "(1/8, -3/16, 3/4)");
/// ```
pub mod shl;
/// Right-shifting a vector (dividing it by a power of 2), by implementations of
/// [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     (RationalVector::from_str("()").unwrap() >> 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (RationalVector::from_str("(1/2, -3/4, 3)").unwrap() >> 0u16).to_string(),
///     "(1/2, -3/4, 3)"
/// );
/// assert_eq!(
///     (RationalVector::from_str("(1/2, -3/4, 3)").unwrap() >> 2u32).to_string(),
///     "(1/8, -3/16, 3/4)"
/// );
/// assert_eq!(
///     (&RationalVector::from_str("(12, 0)").unwrap() >> 100u64).to_string(),
///     "(3/316912650057057350374175801344, 0)"
/// );
///
/// assert_eq!(
///     (RationalVector::from_str("(1/2, -3/4, 3)").unwrap() >> 2i8).to_string(),
///     "(1/8, -3/16, 3/4)"
/// );
/// assert_eq!(
///     (&RationalVector::from_str("(1/2, -3/4, 3)").unwrap() >> -2i64).to_string(),
///     "(2, -3, 12)"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_vector::RationalVector;
///
/// let mut v = RationalVector::from_str("(1/2, -3/4, 3)").unwrap();
/// v >>= 2u32;
/// assert_eq!(v.to_string(), "(1/8, -3/16, 3/4)");
///
/// let mut v = RationalVector::from_str("(1/2, -3/4, 3)").unwrap();
/// v >>= -2i32;
/// assert_eq!(v.to_string(), "(2, -3, 12)");
/// ```
pub mod shr;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting vectors.
pub mod sub;
/// Subtracting a multiple of one vector from another, by implementations of
/// [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign).
///
/// # sub_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_q::Rational;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     RationalVector::from_str("()")
///         .unwrap()
///         .sub_mul(RationalVector::from_str("()").unwrap(), Rational::from(5))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     RationalVector::from_str("(1/2, -2, 3)")
///         .unwrap()
///         .sub_mul(
///             RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             Rational::ZERO
///         )
///         .to_string(),
///     "(1/2, -2, 3)"
/// );
/// assert_eq!(
///     RationalVector::from_str("(1/2, -2, 3)")
///         .unwrap()
///         .sub_mul(
///             RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             Rational::ONE
///         )
///         .to_string(),
///     "(-7/2, -7/3, 9)"
/// );
/// assert_eq!(
///     RationalVector::from_str("(1/2, -2, 3)")
///         .unwrap()
///         .sub_mul(
///             RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             Rational::NEGATIVE_ONE
///         )
///         .to_string(),
///     "(9/2, -5/3, -3)"
/// );
/// assert_eq!(
///     (&RationalVector::from_str("(1/2, -2, 3)").unwrap())
///         .sub_mul(
///             &RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///             &Rational::from_signeds(3, 2)
///         )
///         .to_string(),
///     "(-11/2, -5/2, 12)"
/// );
/// ```
///
/// # sub_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_q::Rational;
/// use malachite_q::rational_vector::RationalVector;
///
/// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
/// v.sub_mul_assign(
///     RationalVector::from_str("(4, 1/3, -6)").unwrap(),
///     Rational::from_signeds(3, 2),
/// );
/// assert_eq!(v.to_string(), "(-11/2, -5/2, 12)");
/// ```
pub mod sub_mul;
/// Implementations of [`SubMulShl`](malachite_base::num::arithmetic::traits::SubMulShl) and
/// [`SubMulShlAssign`](malachite_base::num::arithmetic::traits::SubMulShlAssign), for subtracting a
/// scalar multiple of one vector, shifted left, from another.
pub mod sub_mul_shl;
