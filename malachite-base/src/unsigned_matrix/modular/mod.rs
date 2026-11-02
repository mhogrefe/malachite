// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`ModAdd`](crate::num::modular::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::modular::traits::ModAddAssign), for adding
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// Implementations of [`ModAddMul`](crate::num::modular::traits::ModAddMul) and
/// [`ModAddMulAssign`](crate::num::modular::traits::ModAddMulAssign), for adding a scalar multiple
/// of one matrix to another modulo a number.
pub mod mod_add_mul;
/// An implementation of [`ModIsReduced`](crate::num::modular::traits::ModIsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a number.
pub mod mod_is_reduced;
/// Implementations of [`ModKroneckerProduct`](crate::num::modular::traits::ModKroneckerProduct) and
/// [`ModKroneckerProductAssign`](crate::num::modular::traits::ModKroneckerProductAssign), for
/// computing the Kronecker product of two [`UnsignedMatrix`](super::UnsignedMatrix)s modulo a
/// number.
pub mod mod_kronecker_product;
/// Implementations of [`ModMul`](crate::num::modular::traits::ModMul) and
/// [`ModMulAssign`](crate::num::modular::traits::ModMulAssign), for multiplying an
/// [`UnsignedMatrix`](super::UnsignedMatrix) by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModMul`](crate::num::modular::traits::ModMul) and
/// [`ModMulAssign`](crate::num::modular::traits::ModMulAssign), for multiplying two matrices modulo
/// a number.
pub mod mod_mul_matrix;
/// Implementations of [`ModMul`](crate::num::modular::traits::ModMul) for multiplying an
/// [`UnsignedMatrix`](super::UnsignedMatrix) by an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) on the right modulo a number.
pub mod mod_mul_vector;
/// Implementations of [`ModNeg`](crate::num::modular::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::modular::traits::ModNegAssign), for negating every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](crate::num::modular::traits::Mod), and
/// [`ModAssign`](crate::num::modular::traits::ModAssign), for reducing every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPow`](crate::num::modular::traits::ModPow) and
/// [`ModPowAssign`](crate::num::modular::traits::ModPowAssign), for raising a square matrix to a
/// power modulo a number.
pub mod mod_pow;
/// Implementations of [`ModPowerOf2`](crate::num::modular::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::modular::traits::ModPowerOf2Assign), traits for reducing every
/// element of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](crate::num::modular::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](crate::num::modular::traits::ModPowerOf2AddAssign), for adding
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_add;
/// Implementations of [`ModPowerOf2AddMul`](crate::num::modular::traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](crate::num::modular::traits::ModPowerOf2AddMulAssign), for adding a
/// scalar multiple of one matrix to another modulo a power of 2.
pub mod mod_power_of_2_add_mul;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::modular::traits::ModPowerOf2IsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2KroneckerProduct`][kp] and
/// [`ModPowerOf2KroneckerProductAssign`][kpa], for computing the Kronecker product of two
/// [`UnsignedMatrix`](super::UnsignedMatrix)s modulo a power of 2.
///
/// [kp]: crate::num::modular::traits::ModPowerOf2KroneckerProduct
///
/// [kpa]: crate::num::modular::traits::ModPowerOf2KroneckerProductAssign
pub mod mod_power_of_2_kronecker_product;
/// Implementations of [`ModPowerOf2Mul`](crate::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::modular::traits::ModPowerOf2MulAssign), for multiplying an
/// [`UnsignedMatrix`](super::UnsignedMatrix) by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Mul`](crate::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::modular::traits::ModPowerOf2MulAssign), for multiplying two
/// matrices modulo a power of 2.
pub mod mod_power_of_2_mul_matrix;
/// Implementations of [`ModPowerOf2Mul`](crate::num::modular::traits::ModPowerOf2Mul) for
/// multiplying an [`UnsignedMatrix`](super::UnsignedMatrix) by an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) on the right modulo a power of 2.
pub mod mod_power_of_2_mul_vector;
/// Implementations of [`ModPowerOf2Neg`](crate::num::modular::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::modular::traits::ModPowerOf2NegAssign), for negating every
/// element of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Pow`](crate::num::modular::traits::ModPowerOf2Pow) and
/// [`ModPowerOf2PowAssign`](crate::num::modular::traits::ModPowerOf2PowAssign), for raising a
/// square matrix to a power modulo a power of 2.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2Shl`](crate::num::modular::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](crate::num::modular::traits::ModPowerOf2ShlAssign), for left-shifting a
/// matrix (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModPowerOf2Shl;
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
/// use malachite_base::num::modular::traits::ModPowerOf2ShlAssign;
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
/// Implementations of [`ModPowerOf2Square`](crate::num::modular::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](crate::num::modular::traits::ModPowerOf2SquareAssign), for squaring
/// a square matrix modulo a power of 2.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2Sub`](crate::num::modular::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::modular::traits::ModPowerOf2SubAssign), for subtracting
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModPowerOf2SubMul`](crate::num::modular::traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](crate::num::modular::traits::ModPowerOf2SubMulAssign), for
/// subtracting a scalar multiple of one matrix from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul;
/// Implementations of [`ModShl`](crate::num::modular::traits::ModShl) and
/// [`ModShlAssign`](crate::num::modular::traits::ModShlAssign), for left-shifting a matrix
/// (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShl;
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
/// use malachite_base::num::modular::traits::ModShlAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
/// a.mod_shl_assign(1u8, 7);
/// assert_eq!(a.to_string(), "[3, 2, 6]");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSquare`](crate::num::modular::traits::ModSquare) and
/// [`ModSquareAssign`](crate::num::modular::traits::ModSquareAssign), for squaring a square matrix
/// modulo a number.
pub mod mod_square;
/// Implementations of [`ModSub`](crate::num::modular::traits::ModSub) and
/// [`ModSubAssign`](crate::num::modular::traits::ModSubAssign), for subtracting
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a number.
pub mod mod_sub;
/// Implementations of [`ModSubMul`](crate::num::modular::traits::ModSubMul) and
/// [`ModSubMulAssign`](crate::num::modular::traits::ModSubMulAssign), for subtracting a scalar
/// multiple of one matrix from another modulo a number.
pub mod mod_sub_mul;
/// Implementations of [`ModMul`](crate::num::modular::traits::ModMul) for multiplying an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) by an
/// [`UnsignedMatrix`](super::UnsignedMatrix) on the right modulo a number.
pub mod vector_mod_mul;
/// Implementations of [`ModPowerOf2Mul`](crate::num::modular::traits::ModPowerOf2Mul) for
/// multiplying an [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) by an
/// [`UnsignedMatrix`](super::UnsignedMatrix) on the right modulo a power of 2.
pub mod vector_mod_power_of_2_mul;
