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
/// Adding a multiple of one matrix to another, by implementations of
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign).
///
/// # add_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     NaturalMatrix::from_str("[]_{0,0}")
///         .unwrap()
///         .add_mul(
///             NaturalMatrix::from_str("[]_{0,0}").unwrap(),
///             Natural::from(5u32)
///         )
///         .to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     NaturalMatrix::from_str("[1, 2, 3]")
///         .unwrap()
///         .add_mul(NaturalMatrix::from_str("[4, 5, 6]").unwrap(), Natural::ZERO)
///         .to_string(),
///     "[1, 2, 3]"
/// );
/// assert_eq!(
///     NaturalMatrix::from_str("[1, 2, 3]")
///         .unwrap()
///         .add_mul(NaturalMatrix::from_str("[4, 5, 6]").unwrap(), Natural::ONE)
///         .to_string(),
///     "[5, 7, 9]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 2, 3]").unwrap())
///         .add_mul(
///             &NaturalMatrix::from_str("[4, 5, 6]").unwrap(),
///             &Natural::from(10u32)
///         )
///         .to_string(),
///     "[41, 52, 63]"
/// );
/// assert_eq!(
///     NaturalMatrix::from_str("[1, 0]")
///         .unwrap()
///         .add_mul(
///             &NaturalMatrix::from_str("[18446744073709551615, 1]").unwrap(),
///             &Natural::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "[340282366920938463463374607431768211456, 18446744073709551617]"
/// );
/// ```
///
/// # add_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[1, 2, 3]").unwrap();
/// a.add_mul_assign(
///     NaturalMatrix::from_str("[4, 5, 6]").unwrap(),
///     Natural::from(10u32),
/// );
/// assert_eq!(a.to_string(), "[41, 52, 63]");
/// ```
pub mod add_mul;
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
/// Implementations of
/// [`CharacteristicPolynomial`](malachite_base::num::arithmetic::traits::CharacteristicPolynomial),
/// for computing the characteristic polynomial of a square matrix of naturals.
pub mod characteristic_polynomial;
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
/// Implementations of [`Determinant`](malachite_base::num::arithmetic::traits::Determinant), for
/// computing the determinants of square matrices of naturals.
pub mod determinant;
/// Implementations of
/// [`DeterminantSign`](malachite_base::num::arithmetic::traits::DeterminantSign), for determining
/// the signs of the determinants of square matrices of naturals.
pub mod determinant_sign;
/// Exact division of a matrix by a scalar, by implementations of
/// [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     NaturalMatrix::from_str("[]_{2,0}")
///         .unwrap()
///         .div_exact(Natural::from(5u32))
///         .to_string(),
///     "[]_{2,0}"
/// );
/// assert_eq!(
///     NaturalMatrix::from_str("[0, 6; 255, 3]")
///         .unwrap()
///         .div_exact(Natural::ONE)
///         .to_string(),
///     "[0, 6; 255, 3]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[0, 6; 255, 3]").unwrap())
///         .div_exact(&Natural::from(3u32))
///         .to_string(),
///     "[0, 2; 85, 1]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[3802951800684688204490109616128, 6; 0, 3]").unwrap())
///         .div_exact(Natural::from(3u32))
///         .to_string(),
///     "[1267650600228229401496703205376, 2; 0, 1]"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[0, 6; 255, 3]").unwrap();
/// a.div_exact_assign(Natural::from(3u32));
/// assert_eq!(a.to_string(), "[0, 2; 85, 1]");
/// ```
pub mod div_exact;
/// Implementations of
/// [`ElementaryDivisors`](malachite_base::num::arithmetic::traits::ElementaryDivisors), for
/// computing the elementary divisors of matrices of naturals.
pub mod elementary_divisors;
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
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i8, Floor))
///     .to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i16, Ceiling))
///     .to_string(),
///     "[1, 1; 2, 3]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i32, Nearest))
///         .to_string(),
///     "[0, 1; 2, 2]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[4, 8, 12]")
///         .unwrap()
///         .entrywise_shl_round(-2i64, Exact))
///     .to_string(),
///     "[1, 2, 3]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 2, 3]")
///         .unwrap()
///         .entrywise_shl_round(2i8, Exact))
///         .to_string(),
///     "[4, 8, 12]"
/// );
/// ```
///
/// # entrywise_shl_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shl_round_assign(-1i8, Nearest);
/// assert_eq!(a.to_string(), "[0, 1; 2, 2]");
/// let mut a = NaturalMatrix::from_str("[1, 2, 3]").unwrap();
/// a.entrywise_shl_round_assign(2i32, Exact);
/// assert_eq!(a.to_string(), "[4, 8, 12]");
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
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u8, Floor))
///     .to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u16, Ceiling))
///     .to_string(),
///     "[1, 1; 2, 3]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u32, Nearest))
///         .to_string(),
///     "[0, 1; 2, 2]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[4, 8, 12]")
///         .unwrap()
///         .entrywise_shr_round(2u64, Exact))
///     .to_string(),
///     "[1, 2, 3]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1i8, Down))
///     .to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 2, 3]")
///         .unwrap()
///         .entrywise_shr_round(-2i16, Exact))
///         .to_string(),
///     "[4, 8, 12]"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u8, Nearest);
/// assert_eq!(a.to_string(), "[0, 1; 2, 2]");
/// let mut a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1i32, Up);
/// assert_eq!(a.to_string(), "[1, 1; 2, 3]");
/// ```
pub mod entrywise_shr_round;
/// Implementations of
/// [`KroneckerProduct`](malachite_base::num::arithmetic::traits::KroneckerProduct) and
/// [`KroneckerProductAssign`](malachite_base::num::arithmetic::traits::KroneckerProductAssign), for
/// computing the Kronecker product of two [`NaturalMatrix`](super::NaturalMatrix)s.
pub mod kronecker_product;
/// Implementations of [`ModAdd`](malachite_base::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::arithmetic::traits::ModAddAssign), for adding
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// Implementations of [`ModAddMul`](malachite_base::num::arithmetic::traits::ModAddMul) and
/// [`ModAddMulAssign`](malachite_base::num::arithmetic::traits::ModAddMulAssign), for adding a
/// scalar multiple of one matrix to another modulo a number.
pub mod mod_add_mul;
/// Implementations of [`ModDeterminant`](malachite_base::num::arithmetic::traits::ModDeterminant),
/// for computing the determinant of a square matrix modulo a number.
pub mod mod_determinant;
/// Implementations of [`ModHowellForm`][hf] and [`ModHowellFormAssign`][hfa], for computing the
/// Howell form of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
///
/// [hf]: malachite_base::num::arithmetic::traits::ModHowellForm
///
/// [hfa]: malachite_base::num::arithmetic::traits::ModHowellFormAssign
pub mod mod_howell_form;
/// Implementations of [`ModInverse`](malachite_base::num::arithmetic::traits::ModInverse), for
/// inverting a square matrix modulo a prime.
pub mod mod_inverse;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced), a
/// trait for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a number.
pub mod mod_is_reduced;
/// Implementations of [`ModKroneckerProduct`][kp] and [`ModKroneckerProductAssign`][kpa], for
/// computing the Kronecker product of two [`NaturalMatrix`](super::NaturalMatrix)s modulo a number.
///
/// [kp]: malachite_base::num::arithmetic::traits::ModKroneckerProduct
///
/// [kpa]: malachite_base::num::arithmetic::traits::ModKroneckerProductAssign
pub mod mod_kronecker_product;
/// Implementations of
/// [`ModLeftNullspace`](malachite_base::num::arithmetic::traits::ModLeftNullspace), for computing a
/// basis of the left nullspace of a matrix modulo a prime.
pub mod mod_left_nullspace;
/// Implementations of
/// [`ModLuDecomposition`](malachite_base::num::arithmetic::traits::ModLuDecomposition), for
/// computing an LU decomposition of a matrix modulo a prime.
pub mod mod_lu_decomposition;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign), for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign), for multiplying two
/// matrices modulo a number.
pub mod mod_mul_matrix;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) for multiplying a
/// [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right modulo a number.
pub mod mod_mul_vector;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), for negating every
/// element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`ModNullspace`](malachite_base::num::arithmetic::traits::ModNullspace), for
/// computing a basis of the right nullspace of a matrix modulo a prime.
pub mod mod_nullspace;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](malachite_base::num::arithmetic::traits::Mod), and
/// [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign), for reducing every element of
/// a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPermanent`](malachite_base::num::arithmetic::traits::ModPermanent), for
/// computing the permanents of square matrices of naturals modulo a number.
pub mod mod_permanent;
/// Implementations of [`ModPow`](malachite_base::num::arithmetic::traits::ModPow) and
/// [`ModPowAssign`](malachite_base::num::arithmetic::traits::ModPowAssign), for raising a square
/// [`NaturalMatrix`](super::NaturalMatrix) to a power modulo a number.
pub mod mod_pow;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), traits for
/// reducing every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::arithmetic::traits::ModPowerOf2Add)
/// and [`ModPowerOf2AddAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign), for
/// adding [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_add;
/// Implementations of
/// [`ModPowerOf2AddMul`](malachite_base::num::arithmetic::traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign),
/// for adding a scalar multiple of one matrix to another modulo a power of 2.
pub mod mod_power_of_2_add_mul;
/// Implementations of
/// [`ModPowerOf2Determinant`](malachite_base::num::arithmetic::traits::ModPowerOf2Determinant), for
/// computing determinants of square matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_determinant;
/// Implementations of
/// [`ModPowerOf2HowellForm`](malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm) and
/// [`ModPowerOf2HowellFormAssign`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2HowellFormAssign), for computing Howell
/// forms of matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_howell_form;
/// Implementations of
/// [`ModPowerOf2Inverse`](malachite_base::num::arithmetic::traits::ModPowerOf2Inverse), for
/// inverting square matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_inverse;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), a trait
/// for checking whether every element of a [`NaturalMatrix`](super::NaturalMatrix) is reduced
/// modulo a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2KroneckerProduct`][kp] and
/// [`ModPowerOf2KroneckerProductAssign`][kpa], for computing the Kronecker product of two
/// [`NaturalMatrix`](super::NaturalMatrix)s modulo a power of 2.
///
/// [kp]: malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct
///
/// [kpa]: malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProductAssign
pub mod mod_power_of_2_kronecker_product;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign), for
/// multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign), for
/// multiplying two matrices modulo a power of 2.
pub mod mod_power_of_2_mul_matrix;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// for multiplying a [`NaturalMatrix`](super::NaturalMatrix) by a
/// [`NaturalVector`](crate::natural_vector::NaturalVector) on the right modulo a power of 2.
pub mod mod_power_of_2_mul_vector;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign), for
/// negating every element of a [`NaturalMatrix`](super::NaturalMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;

/// Implementations of
/// [`ModPowerOf2Permanent`](malachite_base::num::arithmetic::traits::ModPowerOf2Permanent), for
/// computing the permanents of square matrices of naturals modulo a power of 2.
pub mod mod_power_of_2_permanent;
/// Implementations of [`ModPowerOf2Pow`](malachite_base::num::arithmetic::traits::ModPowerOf2Pow)
/// and [`ModPowerOf2PowAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign), for
/// raising a square matrix to a power modulo a power of 2.
pub mod mod_power_of_2_pow;
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
/// Implementations of [`ModPowerOf2SimilarityTransform`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2SimilarityTransform) and
/// [`ModPowerOf2SimilarityTransformAssign`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2SimilarityTransformAssign), for applying the
/// similarity transform of FLINT's `fmpz_mat_similarity` to a square matrix of naturals modulo a
/// power of 2.
pub mod mod_power_of_2_similarity_transform;
/// Implementations of [`ModPowerOf2SmithNormalForm`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm) and
/// [`ModPowerOf2SmithNormalFormAssign`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalFormAssign), for computing Smith
/// normal forms of matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_smith_normal_form;
/// Implementations of
/// [`ModPowerOf2Solve`](malachite_base::num::arithmetic::traits::ModPowerOf2Solve), for solving
/// linear systems of naturals modulo powers of 2.
pub mod mod_power_of_2_solve;
/// Implementations of
/// [`ModPowerOf2Square`](malachite_base::num::arithmetic::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign),
/// for squaring a square matrix modulo a power of 2.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2StrongEchelonForm`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonForm) and
/// [`ModPowerOf2StrongEchelonFormAssign`](
/// malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonFormAssign), for computing
/// strong echelon forms of matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_strong_echelon_form;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign), for
/// subtracting [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;

/// Implementations of
/// [`ModPowerOf2SubMul`](malachite_base::num::arithmetic::traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubMulAssign),
/// for subtracting a scalar multiple of one matrix from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul;
/// An implementation of
/// [`ModPowerOf2Trace`](malachite_base::num::arithmetic::traits::ModPowerOf2Trace), the sum of the
/// diagonal elements of a square matrix modulo a power of 2.
pub mod mod_power_of_2_trace;
/// Implementations of [`ModRank`](malachite_base::num::arithmetic::traits::ModRank), for computing
/// the rank of a matrix modulo a prime.
pub mod mod_rank;
/// Implementations of [`ModReducedRowEchelonForm`][rref] and
/// [`ModReducedRowEchelonFormAssign`][rrefa], for computing the reduced row echelon form of a
/// matrix modulo a prime.
///
/// [rref]: malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm
///
/// [rrefa]: malachite_base::num::arithmetic::traits::ModReducedRowEchelonFormAssign
pub mod mod_reduced_row_echelon_form;
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
/// Implementations of [`ModSimilarityTransform`](
/// malachite_base::num::arithmetic::traits::ModSimilarityTransform) and
/// [`ModSimilarityTransformAssign`](
/// malachite_base::num::arithmetic::traits::ModSimilarityTransformAssign), for applying the
/// similarity transform of FLINT's `fmpz_mod_mat_similarity` to a square matrix of naturals modulo
/// a number.
pub mod mod_similarity_transform;
/// Implementations of
/// [`ModSmithNormalForm`](malachite_base::num::arithmetic::traits::ModSmithNormalForm) and
/// [`ModSmithNormalFormAssign`](malachite_base::num::arithmetic::traits::ModSmithNormalFormAssign),
/// for computing Smith normal forms of matrices of naturals modulo any modulus.
pub mod mod_smith_normal_form;
/// Implementations of [`ModSolve`](malachite_base::num::arithmetic::traits::ModSolve), for solving
/// a system of linear equations modulo a prime.
pub mod mod_solve;
/// Implementations of
/// [`ModSolveLowerTriangular`](malachite_base::num::arithmetic::traits::ModSolveLowerTriangular),
/// for solving a lower triangular system of equations modulo a number.
pub mod mod_solve_lower_triangular;
/// Implementations of
/// [`ModSolveUpperTriangular`](malachite_base::num::arithmetic::traits::ModSolveUpperTriangular),
/// for solving a upper triangular system of equations modulo a number.
pub mod mod_solve_upper_triangular;
/// Implementations of [`ModSquare`](malachite_base::num::arithmetic::traits::ModSquare) and
/// [`ModSquareAssign`](malachite_base::num::arithmetic::traits::ModSquareAssign), for squaring a
/// square [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
pub mod mod_square;
/// Implementations of [`ModStrongEchelonForm`][sef] and [`ModStrongEchelonFormAssign`][sefa], for
/// computing the strong echelon form of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
///
/// [sef]: malachite_base::num::arithmetic::traits::ModStrongEchelonForm
///
/// [sefa]: malachite_base::num::arithmetic::traits::ModStrongEchelonFormAssign
pub mod mod_strong_echelon_form;
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting
/// [`NaturalMatrix`](super::NaturalMatrix)s of the same shape modulo a number.
pub mod mod_sub;

/// Implementations of [`ModSubMul`](malachite_base::num::arithmetic::traits::ModSubMul) and
/// [`ModSubMulAssign`](malachite_base::num::arithmetic::traits::ModSubMulAssign), for subtracting a
/// scalar multiple of one matrix from another modulo a number.
pub mod mod_sub_mul;
/// Implementations of [`ModTrace`](malachite_base::num::arithmetic::traits::ModTrace), the sum of
/// the diagonal elements of a square matrix modulo a number.
pub mod mod_trace;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying two matrices.
pub mod mul;
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
/// Implementations of [`Permanent`](malachite_base::num::arithmetic::traits::Permanent), for
/// computing the permanents of square matrices of naturals.
pub mod permanent;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), for raising a square matrix
/// to a power.
pub mod pow;
/// Implementations of [`Rank`](malachite_base::num::arithmetic::traits::Rank), for finding the
/// ranks of matrices of naturals.
pub mod rank;
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
///     (NaturalMatrix::from_str("[1, 2, 3]").unwrap() << 0u16).to_string(),
///     "[1, 2, 3]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2, 3]").unwrap() << 2u32).to_string(),
///     "[4, 8, 12]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[3, 0]").unwrap() << 100u64).to_string(),
///     "[3802951800684688204490109616128, 0]"
/// );
///
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2, 3]").unwrap() << 2i8).to_string(),
///     "[4, 8, 12]"
/// );
///
/// assert_eq!(
///     (&NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap() << -1i64).to_string(),
///     "[0, 1; 1, 2]"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[1, 2, 3]").unwrap();
/// a <<= 2u32;
/// assert_eq!(a.to_string(), "[4, 8, 12]");
///
/// let mut a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
/// a <<= -1i32;
/// assert_eq!(a.to_string(), "[0, 1; 1, 2]");
/// ```
pub mod shl;
/// Right-shifting a matrix (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     (NaturalMatrix::from_str("[]_{0,0}").unwrap() >> 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2, 3]").unwrap() >> 0u16).to_string(),
///     "[1, 2, 3]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[4, 8, 12]").unwrap() >> 2u32).to_string(),
///     "[1, 2, 3]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap() >> 1u8).to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[3802951800684688204490109616128, 0]").unwrap() >> 100u64)
///         .to_string(),
///     "[3, 0]"
/// );
/// assert_eq!(
///     (NaturalMatrix::from_str("[1, 2, 3]").unwrap() >> -2i8).to_string(),
///     "[4, 8, 12]"
/// );
/// assert_eq!(
///     (&NaturalMatrix::from_str("[4, 8, 12]").unwrap() >> 2i64).to_string(),
///     "[1, 2, 3]"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let mut a = NaturalMatrix::from_str("[4, 8, 12]").unwrap();
/// a >>= 2u32;
/// assert_eq!(a.to_string(), "[1, 2, 3]");
/// let mut a = NaturalMatrix::from_str("[1, 2, 3]").unwrap();
/// a >>= -2i32;
/// assert_eq!(a.to_string(), "[4, 8, 12]");
/// ```
pub mod shr;
/// Implementations of [`SmithNormalForm`](malachite_base::num::arithmetic::traits::SmithNormalForm)
/// and [`SmithNormalFormAssign`](malachite_base::num::arithmetic::traits::SmithNormalFormAssign),
/// for computing the Smith normal forms of matrices of naturals.
pub mod smith_normal_form;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), for squaring a square
/// matrix.
pub mod square;
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
