// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`BalancedMod`](malachite_base::num::modular::traits::BalancedMod), which
/// reduce every element of a [`NaturalMatrix`](super::NaturalMatrix) to the representative closest
/// to zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::modular::traits::BalancedModPowerOf2), which reduce
/// every element of a [`NaturalMatrix`](super::NaturalMatrix) to the representative closest to zero
/// modulo a power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`Crt`](malachite_base::num::modular::traits::Crt), which combine a matrix of
/// residues modulo any number with one modulo a word-sized number by the Chinese remainder theorem.
pub mod crt;
/// Implementations of [`ModAdd`](malachite_base::num::modular::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::modular::traits::ModAddAssign), for adding
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// Implementations of [`ModAddMul`](malachite_base::num::modular::traits::ModAddMul) and
/// [`ModAddMulAssign`](malachite_base::num::modular::traits::ModAddMulAssign), for adding a scalar
/// multiple of one matrix to another modulo a number.
pub mod mod_add_mul;
/// An implementation of [`ModIsReduced`](malachite_base::num::modular::traits::ModIsReduced), a
/// trait for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a number.
pub mod mod_is_reduced;
/// Implementations of [`ModKroneckerProduct`][kp] and [`ModKroneckerProductAssign`][kpa], for
/// computing the Kronecker product of two [`NaturalMatrix`](super::NaturalMatrix)s modulo a number.
///
/// [kp]: malachite_base::num::modular::traits::ModKroneckerProduct
///
/// [kpa]: malachite_base::num::modular::traits::ModKroneckerProductAssign
pub mod mod_kronecker_product;
/// Implementations of [`ModMul`](malachite_base::num::modular::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::modular::traits::ModMulAssign), for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModMul`](malachite_base::num::modular::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::modular::traits::ModMulAssign), for multiplying two
/// matrices modulo a number.
pub mod mod_mul_matrix;
/// Implementations of [`ModMul`](malachite_base::num::modular::traits::ModMul) for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right modulo a number.
pub mod mod_mul_vector;
/// Implementations of [`ModNeg`](malachite_base::num::modular::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::modular::traits::ModNegAssign), for negating every element
/// of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](malachite_base::num::modular::traits::Mod), and
/// [`ModAssign`](malachite_base::num::modular::traits::ModAssign), for reducing every element of a
/// [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPow`](malachite_base::num::modular::traits::ModPow) and
/// [`ModPowAssign`](malachite_base::num::modular::traits::ModPowAssign), for raising a square
/// [`NaturalMatrix`](super::NaturalMatrix) to a power modulo a number.
pub mod mod_pow;
/// Implementations of [`ModPowerOf2`](malachite_base::num::modular::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::modular::traits::ModPowerOf2Assign), traits for
/// reducing every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::modular::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](malachite_base::num::modular::traits::ModPowerOf2AddAssign), for adding
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_add;
/// Implementations of
/// [`ModPowerOf2AddMul`](malachite_base::num::modular::traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](malachite_base::num::modular::traits::ModPowerOf2AddMulAssign), for
/// adding a scalar multiple of one matrix to another modulo a power of 2.
pub mod mod_power_of_2_add_mul;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::modular::traits::ModPowerOf2IsReduced), a trait
/// for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2KroneckerProduct`][kp] and
/// [`ModPowerOf2KroneckerProductAssign`][kpa], for computing the Kronecker product of two
/// [`NaturalMatrix`](super::NaturalMatrix)s modulo a power of 2.
///
/// [kp]: malachite_base::num::modular::traits::ModPowerOf2KroneckerProduct
///
/// [kpa]: malachite_base::num::modular::traits::ModPowerOf2KroneckerProductAssign
pub mod mod_power_of_2_kronecker_product;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](malachite_base::num::modular::traits::ModPowerOf2MulAssign), for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](malachite_base::num::modular::traits::ModPowerOf2MulAssign), for
/// multiplying two matrices modulo a power of 2.
pub mod mod_power_of_2_mul_matrix;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::modular::traits::ModPowerOf2Mul) for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right modulo a power of 2.
pub mod mod_power_of_2_mul_vector;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::modular::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](malachite_base::num::modular::traits::ModPowerOf2NegAssign), for
/// negating every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Pow`](malachite_base::num::modular::traits::ModPowerOf2Pow) and
/// [`ModPowerOf2PowAssign`](malachite_base::num::modular::traits::ModPowerOf2PowAssign), for
/// raising a square matrix to a power modulo a power of 2.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2Shl`](malachite_base::num::modular::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](malachite_base::num::modular::traits::ModPowerOf2ShlAssign), for
/// left-shifting a matrix (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModPowerOf2Shl;
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
/// use malachite_base::num::modular::traits::ModPowerOf2ShlAssign;
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
/// Implementations of
/// [`ModPowerOf2Square`](malachite_base::num::modular::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](malachite_base::num::modular::traits::ModPowerOf2SquareAssign), for
/// squaring a square matrix modulo a power of 2.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::modular::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](malachite_base::num::modular::traits::ModPowerOf2SubAssign), for
/// subtracting [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of
/// [`ModPowerOf2SubMul`](malachite_base::num::modular::traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](malachite_base::num::modular::traits::ModPowerOf2SubMulAssign), for
/// subtracting a scalar multiple of one matrix from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul;
/// Implementations of [`ModShl`](malachite_base::num::modular::traits::ModShl) and
/// [`ModShlAssign`](malachite_base::num::modular::traits::ModShlAssign), for left-shifting a vector
/// (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShl;
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
/// use malachite_base::num::modular::traits::ModShlAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[5, 1, 3]").unwrap();
/// a.mod_shl_assign(1u8, Natural::from(7u32));
/// assert_eq!(a.to_string(), "[3, 2, 6]");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSquare`](malachite_base::num::modular::traits::ModSquare) and
/// [`ModSquareAssign`](malachite_base::num::modular::traits::ModSquareAssign), for squaring a
/// square [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_square;
/// Implementations of [`ModSub`](malachite_base::num::modular::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::modular::traits::ModSubAssign), for subtracting
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_sub;
/// Implementations of [`ModSubMul`](malachite_base::num::modular::traits::ModSubMul) and
/// [`ModSubMulAssign`](malachite_base::num::modular::traits::ModSubMulAssign), for subtracting a
/// scalar multiple of one matrix from another modulo a number.
pub mod mod_sub_mul;
/// [`NaturalMatrix::multi_crt`](super::NaturalMatrix::multi_crt), which combines matrices of
/// residues modulo many word-sized moduli by the Chinese remainder theorem.
pub mod multi_crt;
/// [`NaturalMatrix::multi_mod`](super::NaturalMatrix::multi_mod), which reduces a matrix modulo
/// many word-sized moduli.
pub mod multi_mod;
/// Implementations of [`ModMul`](malachite_base::num::modular::traits::ModMul) for multiplying a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) by a
/// [`NaturalMatrix`](super::NaturalMatrix) on the right modulo a number.
pub mod vector_mod_mul;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::modular::traits::ModPowerOf2Mul) for
/// multiplying a [`NaturalVector`](crate::natural_vector::NaturalVector) by a
/// [`NaturalMatrix`](super::NaturalMatrix) on the right modulo a power of 2.
pub mod vector_mod_power_of_2_mul;
