// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`CanonicalPrimitivePart`](
/// crate::num::arithmetic::traits::CanonicalPrimitivePart), [`CanonicalPrimitivePartAssign`](
/// crate::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// crate::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a matrix with
/// non-negative elements, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](crate::num::arithmetic::traits::Content),
/// [`PrimitivePart`](crate::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](crate::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](crate::num::arithmetic::traits::ContentAndPrimitivePart), which
/// compute the GCD of a matrix's elements and the matrix divided by it.
pub mod content;
/// Implementations of [`ContentChained`](crate::num::arithmetic::traits::ContentChained), which
/// computes the GCD of a number and a matrix's elements.
pub mod content_chained;
/// Implementations of [`ModAdd`](crate::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::arithmetic::traits::ModAddAssign), for adding
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a number.
pub mod mod_is_reduced;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](crate::num::arithmetic::traits::ModMulAssign), for multiplying an
/// [`UnsignedMatrix`](super::UnsignedMatrix) by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](crate::num::arithmetic::traits::ModMulAssign), for multiplying two matrices
/// modulo a number.
pub mod mod_mul_matrix;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) for multiplying an
/// [`UnsignedMatrix`](super::UnsignedMatrix) by an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) on the right modulo a number.
pub mod mod_mul_vector;
/// Implementations of [`ModNeg`](crate::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::arithmetic::traits::ModNegAssign), for negating every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](crate::num::arithmetic::traits::Mod), and
/// [`ModAssign`](crate::num::arithmetic::traits::ModAssign), for reducing every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPow`](crate::num::arithmetic::traits::ModPow) and
/// [`ModPowAssign`](crate::num::arithmetic::traits::ModPowAssign), for raising a square matrix to a
/// power modulo a number.
pub mod mod_pow;
/// Implementations of [`ModPowerOf2`](crate::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::arithmetic::traits::ModPowerOf2Assign), traits for reducing
/// every element of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](crate::num::arithmetic::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](crate::num::arithmetic::traits::ModPowerOf2AddAssign), for adding
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_add;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::arithmetic::traits::ModPowerOf2MulAssign), for multiplying
/// an [`UnsignedMatrix`](super::UnsignedMatrix) by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::arithmetic::traits::ModPowerOf2MulAssign), for multiplying
/// two matrices modulo a power of 2.
pub mod mod_power_of_2_mul_matrix;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) for
/// multiplying an [`UnsignedMatrix`](super::UnsignedMatrix) by an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) on the right modulo a power of 2.
pub mod mod_power_of_2_mul_vector;
/// Implementations of [`ModPowerOf2Neg`](crate::num::arithmetic::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::arithmetic::traits::ModPowerOf2NegAssign), for negating
/// every element of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;

/// Implementations of [`ModPowerOf2Pow`](crate::num::arithmetic::traits::ModPowerOf2Pow) and
/// [`ModPowerOf2PowAssign`](crate::num::arithmetic::traits::ModPowerOf2PowAssign), for raising a
/// square matrix to a power modulo a power of 2.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2Shl`](crate::num::arithmetic::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](crate::num::arithmetic::traits::ModPowerOf2ShlAssign), for
/// left-shifting a matrix (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 7]").unwrap();
/// assert_eq!(
///     a.clone().mod_power_of_2_shl(1u8, 3).to_string(),
///     "[2, 2; 6, 6]"
/// );
/// // Shifting by at least the power zeroes every element, keeping the shape.
/// assert_eq!(a.mod_power_of_2_shl(3u32, 3).to_string(), "[0, 0; 0, 0]");
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[1, 128, 1]").unwrap())
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "[2, 0, 2]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u64>::from_str("[1, 1]").unwrap())
///         .mod_power_of_2_shl(63u128, 64)
///         .to_string(),
///     "[9223372036854775808, 9223372036854775808]"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 7]").unwrap();
/// a.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(a.to_string(), "[2, 2; 6, 6]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 7]").unwrap();
/// a.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(a.to_string(), "[0, 0; 0, 0]");
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Square`](crate::num::arithmetic::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](crate::num::arithmetic::traits::ModPowerOf2SquareAssign), for
/// squaring a square matrix modulo a power of 2.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2Sub`](crate::num::arithmetic::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::arithmetic::traits::ModPowerOf2SubAssign), for subtracting
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;

/// An implementation of [`ModPowerOf2Trace`](crate::num::arithmetic::traits::ModPowerOf2Trace), the
/// sum of the diagonal elements of a square matrix modulo a power of 2.
pub mod mod_power_of_2_trace;
/// Implementations of [`ModShl`](crate::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](crate::num::arithmetic::traits::ModShlAssign), for left-shifting a matrix
/// (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShl;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
/// assert_eq!(a.clone().mod_shl(1u8, 7).to_string(), "[3, 2, 6]");
/// // The modulus need not be odd, so elements can become zero, keeping the shape.
/// assert_eq!(a.mod_shl(3u32, 8).to_string(), "[0, 0, 0]");
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap())
///         .mod_shl(8u64, 255)
///         .to_string(),
///     "[1, 2]"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
/// a.mod_shl_assign(1u8, 7);
/// assert_eq!(a.to_string(), "[3, 2, 6]");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSquare`](crate::num::arithmetic::traits::ModSquare) and
/// [`ModSquareAssign`](crate::num::arithmetic::traits::ModSquareAssign), for squaring a square
/// matrix modulo a number.
pub mod mod_square;
/// Implementations of [`ModSub`](crate::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](crate::num::arithmetic::traits::ModSubAssign), for subtracting
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a number.
pub mod mod_sub;

/// Implementations of [`ModTrace`](crate::num::arithmetic::traits::ModTrace), the sum of the
/// diagonal elements of a square matrix modulo a number.
pub mod mod_trace;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) for multiplying an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) by an
/// [`UnsignedMatrix`](super::UnsignedMatrix) on the right modulo a number.
pub mod vector_mod_mul;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) for
/// multiplying an [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) by an
/// [`UnsignedMatrix`](super::UnsignedMatrix) on the right modulo a power of 2.
pub mod vector_mod_power_of_2_mul;
