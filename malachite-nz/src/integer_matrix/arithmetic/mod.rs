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
crate_test_mod! {
    /// Adding a multiple of a matrix of words to an `IntegerMatrix`. Internal machinery for
    /// [`Solve`](malachite_base::num::linear_algebra::traits::Solve), public only under
    /// `test_build`.
    add_mul_unsigned_matrix
}
/// Adding a multiple of one matrix to another, by implementations of
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign).
///
/// # add_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     IntegerMatrix::from_str("[]_{0,0}")
///         .unwrap()
///         .add_mul(
///             IntegerMatrix::from_str("[]_{0,0}").unwrap(),
///             Integer::from(5)
///         )
///         .to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .add_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::ZERO
///         )
///         .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .add_mul(IntegerMatrix::from_str("[4, 5, -6]").unwrap(), Integer::ONE)
///         .to_string(),
///     "[5, 3, -3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .add_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::NEGATIVE_ONE
///         )
///         .to_string(),
///     "[-3, -7, 9]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]").unwrap())
///         .add_mul(
///             &IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             &Integer::from(10)
///         )
///         .to_string(),
///     "[41, 48, -57]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, 0]")
///         .unwrap()
///         .add_mul(
///             &IntegerMatrix::from_str("[18446744073709551615, -1]").unwrap(),
///             &Integer::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "[340282366920938463463374607431768211456, -18446744073709551617]"
/// );
/// ```
///
/// # add_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a.add_mul_assign(
///     IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///     Integer::from(10),
/// );
/// assert_eq!(a.to_string(), "[41, 48, -57]");
/// ```
pub mod add_mul;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a matrix with its first nonzero element, in row-major order, made positive.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a matrix's elements and the matrix divided by it.
pub mod content;
/// Implementations of [`ContentChained`](malachite_base::num::arithmetic::traits::ContentChained),
/// which computes the GCD of a number and a matrix's elements.
pub mod content_chained;
/// Exact division of a matrix by a scalar, by implementations of
/// [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::num::basic::traits::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     IntegerMatrix::from_str("[]_{2,0}")
///         .unwrap()
///         .div_exact(Integer::from(5))
///         .to_string(),
///     "[]_{2,0}"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[0, -6; -255, -3]")
///         .unwrap()
///         .div_exact(Integer::ONE)
///         .to_string(),
///     "[0, -6; -255, -3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[0, -6; -255, -3]")
///         .unwrap()
///         .div_exact(Integer::NEGATIVE_ONE)
///         .to_string(),
///     "[0, 6; 255, 3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[0, -6; -255, -3]").unwrap())
///         .div_exact(&Integer::from(-3))
///         .to_string(),
///     "[0, 2; 85, 1]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3802951800684688204490109616128, 6; 0, 3]").unwrap())
///         .div_exact(Integer::from(3))
///         .to_string(),
///     "[-1267650600228229401496703205376, 2; 0, 1]"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[0, -6; -255, -3]").unwrap();
/// a.div_exact_assign(Integer::from(-3));
/// assert_eq!(a.to_string(), "[0, 2; 85, 1]");
/// ```
pub mod div_exact;
/// Left-shifting a matrix (multiplying it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShlRound`](malachite_base::num::arithmetic::traits::EntrywiseShlRound) and
/// [`EntrywiseShlRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign).
///
/// # entrywise_shl_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i8, Floor))
///     .to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i16, Down))
///     .to_string(),
///     "[0, 1; -1, 2]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i32, Nearest))
///         .to_string(),
///     "[0, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[4, -8, 12]")
///         .unwrap()
///         .entrywise_shl_round(-2i64, Exact))
///     .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .entrywise_shl_round(2i8, Exact))
///         .to_string(),
///     "[4, -8, 12]"
/// );
/// ```
///
/// # entrywise_shl_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a.entrywise_shl_round_assign(-1i8, Down);
/// assert_eq!(a.to_string(), "[0, 1; -1, 2]");
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a.entrywise_shl_round_assign(2i32, Exact);
/// assert_eq!(a.to_string(), "[4, -8, 12]");
/// ```
pub mod entrywise_shl_round;
/// Right-shifting a matrix (dividing it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShrRound`](malachite_base::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u8, Floor))
///     .to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u16, Down))
///     .to_string(),
///     "[0, 1; -1, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u32, Ceiling))
///     .to_string(),
///     "[0, 1; -1, 3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u64, Up))
///         .to_string(),
///     "[-1, 1; -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u8, Nearest))
///     .to_string(),
///     "[0, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[4, -8, 12]")
///         .unwrap()
///         .entrywise_shr_round(2u16, Exact))
///     .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1i8, Down))
///     .to_string(),
///     "[0, 1; -1, 2]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .entrywise_shr_round(-2i16, Exact))
///         .to_string(),
///     "[4, -8, 12]"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u8, Down);
/// assert_eq!(a.to_string(), "[0, 1; -1, 2]");
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1i32, Nearest);
/// assert_eq!(a.to_string(), "[0, 1; -2, 2]");
/// ```
pub mod entrywise_shr_round;
#[doc(hidden)]
pub mod fraction_matrix;
/// Implementations of
/// [`KroneckerProduct`](malachite_base::num::arithmetic::traits::KroneckerProduct) and
/// [`KroneckerProductAssign`](malachite_base::num::arithmetic::traits::KroneckerProductAssign), for
/// computing the Kronecker product of two [`IntegerMatrix`](super::IntegerMatrix)s.
pub mod kronecker_product;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying two matrices.
pub mod mul;
/// Implementations of [`Mul`](core::ops::Mul) for multiplying an
/// [`IntegerMatrix`](super::IntegerMatrix) by an
/// [`IntegerVector`](crate::integer_vector::IntegerVector) on the right.
pub mod mul_vector;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a matrix.
pub mod neg;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), for raising a square matrix
/// to a power.
pub mod pow;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying an [`IntegerMatrix`](super::IntegerMatrix) by a scalar on either side.
pub mod scalar_mul;
/// Left-shifting a matrix (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[]_{0,0}").unwrap() << 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() << 0u16).to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() << 2u32).to_string(),
///     "[4, -8, 12]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3, 0]").unwrap() << 100u64).to_string(),
///     "[-3802951800684688204490109616128, 0]"
/// );
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() << 2i8).to_string(),
///     "[4, -8, 12]"
/// );
///
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap() << -1i64).to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a <<= 2u32;
/// assert_eq!(a.to_string(), "[4, -8, 12]");
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a <<= -1i32;
/// assert_eq!(a.to_string(), "[-1, 1; -2, 2]");
/// ```
pub mod shl;
/// Right-shifting a matrix (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[]_{0,0}").unwrap() >> 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() >> 0u16).to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[4, -8, 12]").unwrap() >> 2u32).to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap() >> 1u8).to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3802951800684688204490109616128, 0]").unwrap() >> 100u64)
///         .to_string(),
///     "[-3, 0]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() >> -2i8).to_string(),
///     "[4, -8, 12]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap() >> 1i64).to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a >>= 1u32;
/// assert_eq!(a.to_string(), "[-1, 1; -2, 2]");
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a >>= -2i32;
/// assert_eq!(a.to_string(), "[4, -8, 12]");
/// ```
pub mod shr;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), for squaring a square
/// matrix.
pub mod square;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting matrices of the same shape.
pub mod sub;
/// Subtracting a multiple of one matrix from another, by implementations of
/// [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign).
///
/// # sub_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     IntegerMatrix::from_str("[]_{0,0}")
///         .unwrap()
///         .sub_mul(
///             IntegerMatrix::from_str("[]_{0,0}").unwrap(),
///             Integer::from(5)
///         )
///         .to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .sub_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::ZERO
///         )
///         .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .sub_mul(IntegerMatrix::from_str("[4, 5, -6]").unwrap(), Integer::ONE)
///         .to_string(),
///     "[-3, -7, 9]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .sub_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::NEGATIVE_ONE
///         )
///         .to_string(),
///     "[5, 3, -3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]").unwrap())
///         .sub_mul(
///             &IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             &Integer::from(10)
///         )
///         .to_string(),
///     "[-39, -52, 63]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, 0]")
///         .unwrap()
///         .sub_mul(
///             &IntegerMatrix::from_str("[18446744073709551615, -1]").unwrap(),
///             &Integer::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "[-340282366920938463463374607431768211454, 18446744073709551617]"
/// );
/// ```
///
/// # sub_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a.sub_mul_assign(
///     IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///     Integer::from(10),
/// );
/// assert_eq!(a.to_string(), "[-39, -52, 63]");
/// ```
pub mod sub_mul;
/// Implementations of [`Mul`](core::ops::Mul) for multiplying an
/// [`IntegerVector`](crate::integer_vector::IntegerVector) by an
/// [`IntegerMatrix`](super::IntegerMatrix) on the right.
pub mod vector_mul;
