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
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod), which
/// reduce every element of a [`NaturalMatrix`](super::NaturalMatrix) to the representative closest
/// to zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::arithmetic::traits::BalancedModPowerOf2), which
/// reduce every element of a [`NaturalMatrix`](super::NaturalMatrix) to the representative closest
/// to zero modulo a power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a matrix
/// with non-negative elements, are the same as the primitive part.
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
/// Implementations of [`Crt`](malachite_base::num::arithmetic::traits::Crt), which combine a matrix
/// of residues modulo any number with one modulo a word-sized number by the Chinese remainder
/// theorem.
pub mod crt;
/// Implementations of [`ModAdd`](malachite_base::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::arithmetic::traits::ModAddAssign), for adding
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced), a
/// trait for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a number.
pub mod mod_is_reduced;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign), for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right modulo a number.
pub mod mod_mul_vector;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), for negating every
/// element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](malachite_base::num::arithmetic::traits::Mod), and
/// [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign), for reducing every element of
/// a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), traits for
/// reducing every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::arithmetic::traits::ModPowerOf2Add)
/// and [`ModPowerOf2AddAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign), for
/// adding [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_add;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), a trait
/// for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign), for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// for multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right modulo a power of 2.
pub mod mod_power_of_2_mul_vector;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign), for
/// negating every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;

/// Implementations of [`ModPowerOf2Shl`](malachite_base::num::arithmetic::traits::ModPowerOf2Shl)
/// and [`ModPowerOf2ShlAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign), for
/// left-shifting a matrix (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let a = NaturalMatrix::from_str("[5, 1; 3, 7]").unwrap();
/// assert_eq!(
///     a.clone().mod_power_of_2_shl(1u8, 3).to_string(),
///     "[2, 2; 6, 6]"
/// );
/// // Shifting by at least the power zeroes every element, keeping the shape.
/// assert_eq!(a.mod_power_of_2_shl(3u32, 3).to_string(), "[0, 0; 0, 0]");
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 128, 1]").unwrap())
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "[2, 0, 2]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 1]").unwrap())
///         .mod_power_of_2_shl(100u128, 101)
///         .to_string(),
///     "[1267650600228229401496703205376, 1267650600228229401496703205376]"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[5, 1; 3, 7]").unwrap();
/// a.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(a.to_string(), "[2, 2; 6, 6]");
///
/// let mut a = NaturalMatrix::from_str("[5, 1; 3, 7]").unwrap();
/// a.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(a.to_string(), "[0, 0; 0, 0]");
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign), for
/// subtracting [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;

/// Implementations of [`ModShl`](malachite_base::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](malachite_base::num::arithmetic::traits::ModShlAssign), for left-shifting a
/// vector (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShl;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let a = NaturalMatrix::from_str("[5, 1, 3]").unwrap();
/// assert_eq!(
///     a.clone().mod_shl(1u8, Natural::from(7u32)).to_string(),
///     "[3, 2, 6]"
/// );
/// // The modulus need not be odd, so elements can become zero, keeping the shape.
/// assert_eq!(
///     a.mod_shl(3u32, Natural::from(8u32)).to_string(),
///     "[0, 0, 0]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 1]").unwrap())
///         .mod_shl(
///             100u64,
///             Natural::from_str("1000000000000000000000000000000").unwrap()
///         )
///         .to_string(),
///     "[267650600228229401496703205376, 267650600228229401496703205376]"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[5, 1, 3]").unwrap();
/// a.mod_shl_assign(1u8, Natural::from(7u32));
/// assert_eq!(a.to_string(), "[3, 2, 6]");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_sub;

/// Implementations of [`Mul`](core::ops::Mul) for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right.
pub mod mul_vector;
/// [`NaturalMatrix::multi_crt`](super::NaturalMatrix::multi_crt), which combines matrices of
/// residues modulo many word-sized moduli by the Chinese remainder theorem.
pub mod multi_crt;
/// [`NaturalMatrix::multi_mod`](super::NaturalMatrix::multi_mod), which reduces a matrix modulo
/// many word-sized moduli.
pub mod multi_mod;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a scalar on either side.
pub mod scalar_mul;

/// Left-shifting a matrix (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     (NaturalMatrix::from_str("[]_{0,0}").unwrap() << 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap() << 0u16).to_string(),
///     "[1, 2; 3, 4]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap() << 2u32).to_string(),
///     "[4, 8; 12, 16]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[3; 0]").unwrap() << 100u64).to_string(),
///     "[3802951800684688204490109616128; 0]"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
/// a <<= 2u32;
/// assert_eq!(a.to_string(), "[4, 8; 12, 16]");
/// ```
pub mod shl;
/// An implementation of [`Trace`](malachite_base::num::arithmetic::traits::Trace), the sum of the
/// diagonal elements of a square matrix.
pub mod trace;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) for multiplying a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) by a
/// [`NaturalMatrix`](super::NaturalMatrix) on the right modulo a number.
pub mod vector_mod_mul;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// for multiplying a [`NaturalVector`](crate::natural_vector::NaturalVector) by a
/// [`NaturalMatrix`](super::NaturalMatrix) on the right modulo a power of 2.
pub mod vector_mod_power_of_2_mul;
/// Implementations of [`Mul`](core::ops::Mul) for multiplying a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) by a
/// [`NaturalMatrix`](super::NaturalMatrix) on the right.
pub mod vector_mul;
