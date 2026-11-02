// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`AbsSquared`](malachite_base::num::arithmetic::traits::AbsSquared), the
/// squared norm of a vector: the sum of the squares of its elements.
pub mod abs_squared;
/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// vectors.
pub mod add;
/// Implementations of [`AddDot`](malachite_base::num::arithmetic::traits::AddDot) and
/// [`AddDotAssign`](malachite_base::num::arithmetic::traits::AddDotAssign), for adding the dot
/// product of two vectors to a number.
pub mod add_dot;
/// Implementations of [`AddDotReversed`](malachite_base::num::arithmetic::traits::AddDotReversed)
/// and [`AddDotReversedAssign`](malachite_base::num::arithmetic::traits::AddDotReversedAssign), for
/// adding the dot product of a vector and the reverse of another to a number.
pub mod add_dot_reversed;
/// Adding a multiple of one vector to another, by implementations of
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign).
///
/// # add_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     NaturalVector::from_str("()")
///         .unwrap()
///         .add_mul(NaturalVector::from_str("()").unwrap(), Natural::from(5u32))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(1, 2, 3)")
///         .unwrap()
///         .add_mul(NaturalVector::from_str("(4, 5, 6)").unwrap(), Natural::ZERO)
///         .to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(1, 2, 3)")
///         .unwrap()
///         .add_mul(NaturalVector::from_str("(4, 5, 6)").unwrap(), Natural::ONE)
///         .to_string(),
///     "(5, 7, 9)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
///         .add_mul(
///             &NaturalVector::from_str("(4, 5, 6)").unwrap(),
///             &Natural::from(10u32)
///         )
///         .to_string(),
///     "(41, 52, 63)"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(1, 0)")
///         .unwrap()
///         .add_mul(
///             &NaturalVector::from_str("(18446744073709551615, 1)").unwrap(),
///             &Natural::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "(340282366920938463463374607431768211456, 18446744073709551617)"
/// );
/// ```
///
/// # add_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v.add_mul_assign(
///     NaturalVector::from_str("(4, 5, 6)").unwrap(),
///     Natural::from(10u32),
/// );
/// assert_eq!(v.to_string(), "(41, 52, 63)");
/// ```
pub mod add_mul;
/// Implementations of [`AddMulShl`](malachite_base::num::arithmetic::traits::AddMulShl) and
/// [`AddMulShlAssign`](malachite_base::num::arithmetic::traits::AddMulShlAssign), for adding a
/// scalar multiple of one vector, shifted left, to another.
pub mod add_mul_shl;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a vector
/// with non-negative elements, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of [`CheckedAbs`](malachite_base::num::arithmetic::traits::CheckedAbs), for the
/// Euclidean norm of a vector when it is exact.
pub mod checked_abs;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a vector's elements and the vector divided by it.
pub mod content;
/// Implementations of [`ContentChained`](malachite_base::num::arithmetic::traits::ContentChained),
/// which computes the GCD of a number and a vector's elements.
pub mod content_chained;
/// Exact division of a vector by a scalar, by implementations of
/// [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     NaturalVector::from_str("()")
///         .unwrap()
///         .div_exact(Natural::from(5u32))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(0, 6, 255)")
///         .unwrap()
///         .div_exact(Natural::ONE)
///         .to_string(),
///     "(0, 6, 255)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(0, 6, 255)").unwrap())
///         .div_exact(Natural::from(3u32))
///         .to_string(),
///     "(0, 2, 85)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3802951800684688204490109616128, 6)").unwrap())
///         .div_exact(Natural::from(3u32))
///         .to_string(),
///     "(1267650600228229401496703205376, 2)"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(0, 6, 255)").unwrap();
/// v.div_exact_assign(Natural::from(3u32));
/// assert_eq!(v.to_string(), "(0, 2, 85)");
/// ```
pub mod div_exact;
/// Implementations of [`Dot`](malachite_base::num::arithmetic::traits::Dot), the dot product of two
/// vectors.
pub mod dot;
/// Implementations of [`DotReversed`](malachite_base::num::arithmetic::traits::DotReversed), the
/// dot product of a vector and the reverse of another.
pub mod dot_reversed;
/// Implementations of [`DotSign`](malachite_base::num::arithmetic::traits::DotSign), the sign of
/// the dot product of two vectors.
pub mod dot_sign;
/// An implementation of [`ElementLcm`](malachite_base::num::arithmetic::traits::ElementLcm), the
/// LCM of the elements.
pub mod element_lcm;
/// An implementation of
/// [`ElementProduct`](malachite_base::num::arithmetic::traits::ElementProduct), the product of the
/// elements.
pub mod element_product;
/// An implementation of [`ElementSum`](malachite_base::num::arithmetic::traits::ElementSum), the
/// sum of the elements.
pub mod element_sum;
/// Dividing every element of a vector by a scalar and rounding each quotient, by implementations of
/// [`EntrywiseDivRound`](malachite_base::num::arithmetic::traits::EntrywiseDivRound) and
/// [`EntrywiseDivRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseDivRoundAssign).
///
/// # entrywise_div_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRound;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_div_round(Natural::TWO, Floor)
///         .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)").unwrap())
///         .entrywise_div_round(Natural::TWO, Ceiling)
///         .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)").unwrap())
///         .entrywise_div_round(Natural::TWO, Nearest)
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(3, 6, 255)")
///         .unwrap()
///         .entrywise_div_round(Natural::from(3u32), Exact)
///         .to_string(),
///     "(1, 2, 85)"
/// );
/// ```
///
/// # entrywise_div_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRoundAssign;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_div_round_assign(Natural::TWO, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// ```
pub mod entrywise_div_round;
/// Multiplying two vectors element by element, by implementations of
/// [`EntrywiseMul`](malachite_base::num::arithmetic::traits::EntrywiseMul) and
/// [`EntrywiseMulAssign`](malachite_base::num::arithmetic::traits::EntrywiseMulAssign).
pub mod entrywise_mul;
/// Left-shifting a vector (multiplying it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShlRound`](malachite_base::num::arithmetic::traits::EntrywiseShlRound) and
/// [`EntrywiseShlRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign).
///
/// # entrywise_shl_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i8, Floor))
///     .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i16, Ceiling))
///     .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i32, Nearest))
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(4, 8, 12)")
///         .unwrap()
///         .entrywise_shl_round(-2i64, Exact))
///     .to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3)")
///         .unwrap()
///         .entrywise_shl_round(2i8, Exact))
///         .to_string(),
///     "(4, 8, 12)"
/// );
/// ```
///
/// # entrywise_shl_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shl_round_assign(-1i8, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v.entrywise_shl_round_assign(2i32, Exact);
/// assert_eq!(v.to_string(), "(4, 8, 12)");
/// ```
pub mod entrywise_shl_round;
/// Right-shifting a vector (dividing it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShrRound`](malachite_base::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u8, Floor))
///     .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u16, Ceiling))
///     .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u32, Nearest))
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(4, 8, 12)")
///         .unwrap()
///         .entrywise_shr_round(2u64, Exact))
///     .to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1i8, Down))
///     .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3)")
///         .unwrap()
///         .entrywise_shr_round(-2i16, Exact))
///         .to_string(),
///     "(4, 8, 12)"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1u8, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1i32, Up);
/// assert_eq!(v.to_string(), "(1, 1, 2, 3)");
/// ```
pub mod entrywise_shr_round;
/// An implementation of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest height of any
/// element.
pub mod height;
/// An implementation of [`L1Norm`](malachite_base::num::arithmetic::traits::L1Norm), the sum of the
/// absolute values of the elements.
pub mod l1_norm;
/// Implementations of [`Div`](core::ops::Div) and [`DivAssign`](core::ops::DivAssign), for dividing
/// a vector by a scalar.
///
/// # div
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("()").unwrap() / Natural::from(5u32)).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(0, 7, 255)").unwrap() / Natural::ONE).to_string(),
///     "(0, 7, 255)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(0, 7, 255)").unwrap() / Natural::TWO).to_string(),
///     "(0, 3, 127)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3802951800684688204490109616128, 7)").unwrap()
///         / Natural::from(3u32))
///     .to_string(),
///     "(1267650600228229401496703205376, 2)"
/// );
/// ```
///
/// # div_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(0, 7, 255)").unwrap();
/// v /= Natural::TWO;
/// assert_eq!(v.to_string(), "(0, 3, 127)");
/// ```
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
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("()").unwrap() << 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() << 0u16).to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() << 2u32).to_string(),
///     "(4, 8, 12)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3, 0)").unwrap() << 100u64).to_string(),
///     "(3802951800684688204490109616128, 0)"
/// );
///
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() << 2i8).to_string(),
///     "(4, 8, 12)"
/// );
///
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)").unwrap() << -1i64).to_string(),
///     "(0, 1, 1, 2)"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v <<= 2u32;
/// assert_eq!(v.to_string(), "(4, 8, 12)");
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v <<= -1i32;
/// assert_eq!(v.to_string(), "(0, 1, 1, 2)");
/// ```
pub mod shl;
/// Right-shifting a vector (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("()").unwrap() >> 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() >> 0u16).to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(4, 8, 12)").unwrap() >> 2u32).to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)").unwrap() >> 1u8).to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3802951800684688204490109616128, 0)").unwrap() >> 100u64)
///         .to_string(),
///     "(3, 0)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() >> -2i8).to_string(),
///     "(4, 8, 12)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(4, 8, 12)").unwrap() >> 2i64).to_string(),
///     "(1, 2, 3)"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(4, 8, 12)").unwrap();
/// v >>= 2u32;
/// assert_eq!(v.to_string(), "(1, 2, 3)");
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v >>= -2i32;
/// assert_eq!(v.to_string(), "(4, 8, 12)");
/// ```
pub mod shr;
/// Implementations of [`SubDot`](malachite_base::num::arithmetic::traits::SubDot) and
/// [`SubDotAssign`](malachite_base::num::arithmetic::traits::SubDotAssign), for subtracting the dot
/// product of two vectors from a number.
pub mod sub_dot;
/// Implementations of [`SubDotReversed`](malachite_base::num::arithmetic::traits::SubDotReversed)
/// and [`SubDotReversedAssign`](malachite_base::num::arithmetic::traits::SubDotReversedAssign), for
/// subtracting the dot product of a vector and the reverse of another from a number.
pub mod sub_dot_reversed;
/// Implementations of [`Sum`](core::iter::Sum), for adding up vectors entry by entry.
pub mod sum;
