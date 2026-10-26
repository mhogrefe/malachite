// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Berkowitz's division-free algorithm for characteristic polynomials, generic over the ring of
/// elements.
#[doc(hidden)]
pub mod berkowitz_over_ring;
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
/// Exact division of a matrix by a scalar, by implementations of
/// [`DivExact`](crate::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](crate::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[]_{2,0}")
///         .unwrap()
///         .div_exact(5)
///         .to_string(),
///     "[]_{2,0}"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[0, 6; 255, 3]")
///         .unwrap()
///         .div_exact(1)
///         .to_string(),
///     "[0, 6; 255, 3]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[0, 6; 255, 3]").unwrap())
///         .div_exact(3)
///         .to_string(),
///     "[0, 2; 85, 1]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u64>::from_str("[18446744073709551614, 4]").unwrap())
///         .div_exact(2)
///         .to_string(),
///     "[9223372036854775807, 2]"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[0, 6; 255, 3]").unwrap();
/// a.div_exact_assign(3);
/// assert_eq!(a.to_string(), "[0, 2; 85, 1]");
/// ```
pub mod div_exact;
/// Right-shifting a matrix (dividing it by a power of 2) and rounding every element, by
/// implementations of [`EntrywiseShrRound`](crate::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](crate::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let v = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// assert_eq!(
///     (&v).entrywise_shr_round(1u8, Floor).to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (&v).entrywise_shr_round(1u16, Ceiling).to_string(),
///     "[1, 1; 2, 3]"
/// );
/// // Ties round to even.
/// assert_eq!(
///     (&v).entrywise_shr_round(1u32, Nearest).to_string(),
///     "[0, 1; 2, 2]"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[4, 8, 12]")
///         .unwrap()
///         .entrywise_shr_round(2u64, Exact)
///         .to_string(),
///     "[1, 2, 3]"
/// );
/// // Shifting by at least the width rounds every element to 0 or 1.
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[255, 128, 127]")
///         .unwrap()
///         .entrywise_shr_round(8u8, Nearest)
///         .to_string(),
///     "[1, 0, 0]"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[255, 1, 0]")
///         .unwrap()
///         .entrywise_shr_round(100u128, Ceiling)
///         .to_string(),
///     "[1, 1, 0]"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u8, Nearest);
/// assert_eq!(a.to_string(), "[0, 1; 2, 2]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u32, Up);
/// assert_eq!(a.to_string(), "[1, 1; 2, 3]");
/// ```
pub mod entrywise_shr_round;
/// Implementations of [`ModAdd`](crate::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::arithmetic::traits::ModAddAssign), for adding
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a number.
pub mod mod_add;
/// Implementations of [`ModAddMul`](crate::num::arithmetic::traits::ModAddMul) and
/// [`ModAddMulAssign`](crate::num::arithmetic::traits::ModAddMulAssign), for adding a scalar
/// multiple of one matrix to another modulo a number.
pub mod mod_add_mul;
/// The characteristic polynomial of a square matrix modulo a number.
#[doc(hidden)]
pub mod mod_characteristic_polynomial;
/// Implementations of [`ModDeterminant`](crate::num::arithmetic::traits::ModDeterminant), for
/// computing the determinant of a square matrix modulo a number.
pub mod mod_determinant;
/// Implementations of [`ModHowellForm`](crate::num::arithmetic::traits::ModHowellForm) and
/// [`ModHowellFormAssign`](crate::num::arithmetic::traits::ModHowellFormAssign), for computing the
/// Howell form of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
///
/// # mod_howell_form
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModHowellForm;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let a = UnsignedMatrix::<u8>::from_str("[4, 2; 2, 0]").unwrap();
/// assert_eq!(a.mod_howell_form(8).to_string(), "[2, 0; 0, 2]");
///
/// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
/// let a = UnsignedMatrix::<u8>::from_str("[2, 1]").unwrap();
/// assert_eq!(a.mod_howell_form(4).to_string(), "[2, 1; 0, 2]");
///
/// // The zero rows come last.
/// let a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
/// assert_eq!(a.mod_howell_form(4).to_string(), "[0, 2; 0, 0; 0, 0]");
///
/// let a = UnsignedMatrix::<u8>::from_str("[3, 6; 6, 3]").unwrap();
/// assert_eq!(a.mod_howell_form(9).to_string(), "[3, 6; 0, 0]");
/// ```
///
/// # mod_howell_form_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModHowellFormAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[4, 2; 2, 0]").unwrap();
/// a.mod_howell_form_assign(8);
/// assert_eq!(a.to_string(), "[2, 0; 0, 2]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[2, 1]").unwrap();
/// a.mod_howell_form_assign(4);
/// assert_eq!(a.to_string(), "[2, 1; 0, 2]");
/// ```
pub mod mod_howell_form;
/// Implementations of [`ModInverse`](crate::num::arithmetic::traits::ModInverse), for inverting a
/// square matrix modulo a prime.
pub mod mod_inverse;
/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a number.
pub mod mod_is_reduced;
/// Implementations of [`ModKroneckerProduct`](crate::num::arithmetic::traits::ModKroneckerProduct)
/// and [`ModKroneckerProductAssign`](crate::num::arithmetic::traits::ModKroneckerProductAssign),
/// for computing the Kronecker product of two [`UnsignedMatrix`](super::UnsignedMatrix)s modulo a
/// number.
pub mod mod_kronecker_product;
/// Implementations of [`ModLeftNullspace`](crate::num::arithmetic::traits::ModLeftNullspace), for
/// computing a basis of the left nullspace of a matrix modulo a prime.
pub mod mod_left_nullspace;
/// Implementations of [`ModLuDecomposition`](crate::num::arithmetic::traits::ModLuDecomposition),
/// for computing an LU decomposition of a matrix modulo a prime.
pub mod mod_lu_decomposition;
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
/// Implementations of [`ModNullspace`](crate::num::arithmetic::traits::ModNullspace), for computing
/// a basis of the right nullspace of a matrix modulo a prime.
pub mod mod_nullspace;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](crate::num::arithmetic::traits::Mod), and
/// [`ModAssign`](crate::num::arithmetic::traits::ModAssign), for reducing every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPermanent`](crate::num::arithmetic::traits::ModPermanent), for computing
/// the permanents of square matrices of unsigned integers modulo a number.
pub mod mod_permanent;
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
/// Implementations of [`ModPowerOf2AddMul`](crate::num::arithmetic::traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](crate::num::arithmetic::traits::ModPowerOf2AddMulAssign), for adding
/// a scalar multiple of one matrix to another modulo a power of 2.
pub mod mod_power_of_2_add_mul;
/// Implementations of
/// [`ModPowerOf2Determinant`](crate::num::arithmetic::traits::ModPowerOf2Determinant), for
/// computing determinants of square matrices modulo powers of 2.
pub mod mod_power_of_2_determinant;
/// Implementations of
/// [`ModPowerOf2HowellForm`](crate::num::arithmetic::traits::ModPowerOf2HowellForm) and
/// [`ModPowerOf2HowellFormAssign`](crate::num::arithmetic::traits::ModPowerOf2HowellFormAssign),
/// for computing Howell forms of matrices modulo powers of 2.
pub mod mod_power_of_2_howell_form;
/// Implementations of [`ModPowerOf2Inverse`](crate::num::arithmetic::traits::ModPowerOf2Inverse),
/// for inverting square matrices modulo powers of 2.
pub mod mod_power_of_2_inverse;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2KroneckerProduct`][kp] and
/// [`ModPowerOf2KroneckerProductAssign`][kpa], for computing the Kronecker product of two
/// [`UnsignedMatrix`](super::UnsignedMatrix)s modulo a power of 2.
///
/// [kp]: crate::num::arithmetic::traits::ModPowerOf2KroneckerProduct
///
/// [kpa]: crate::num::arithmetic::traits::ModPowerOf2KroneckerProductAssign
pub mod mod_power_of_2_kronecker_product;
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

/// Implementations of
/// [`ModPowerOf2Permanent`](crate::num::arithmetic::traits::ModPowerOf2Permanent), for computing
/// the permanents of square matrices of unsigned integers modulo a power of 2.
pub mod mod_power_of_2_permanent;
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
/// Implementations of [`ModPowerOf2SimilarityTransform`](
/// crate::num::arithmetic::traits::ModPowerOf2SimilarityTransform) and
/// [`ModPowerOf2SimilarityTransformAssign`](
/// crate::num::arithmetic::traits::ModPowerOf2SimilarityTransformAssign), for applying the
/// similarity transform of FLINT's `fmpz_mat_similarity` to a square matrix modulo a power of 2.
pub mod mod_power_of_2_similarity_transform;
/// Implementations of
/// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm) and
/// [`ModPowerOf2SmithNormalFormAssign`](
/// crate::num::arithmetic::traits::ModPowerOf2SmithNormalFormAssign), for computing Smith normal
/// forms of matrices modulo powers of 2.
pub mod mod_power_of_2_smith_normal_form;
/// Implementations of [`ModPowerOf2Solve`](crate::num::arithmetic::traits::ModPowerOf2Solve), for
/// solving linear systems modulo powers of 2.
pub mod mod_power_of_2_solve;
/// Implementations of [`ModPowerOf2Square`](crate::num::arithmetic::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](crate::num::arithmetic::traits::ModPowerOf2SquareAssign), for
/// squaring a square matrix modulo a power of 2.
pub mod mod_power_of_2_square;
/// Implementations of
/// [`ModPowerOf2StrongEchelonForm`](crate::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)
/// and [`ModPowerOf2StrongEchelonFormAssign`](
/// crate::num::arithmetic::traits::ModPowerOf2StrongEchelonFormAssign), for computing strong
/// echelon forms of matrices modulo powers of 2.
pub mod mod_power_of_2_strong_echelon_form;
/// Implementations of [`ModPowerOf2Sub`](crate::num::arithmetic::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::arithmetic::traits::ModPowerOf2SubAssign), for subtracting
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a power of 2.
pub mod mod_power_of_2_sub;

/// Implementations of [`ModPowerOf2SubMul`](crate::num::arithmetic::traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](crate::num::arithmetic::traits::ModPowerOf2SubMulAssign), for
/// subtracting a scalar multiple of one matrix from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul;
/// An implementation of [`ModPowerOf2Trace`](crate::num::arithmetic::traits::ModPowerOf2Trace), the
/// sum of the diagonal elements of a square matrix modulo a power of 2.
pub mod mod_power_of_2_trace;
/// Implementations of [`ModRank`](crate::num::arithmetic::traits::ModRank), for computing the rank
/// of a matrix modulo a prime.
pub mod mod_rank;
/// Implementations of [`ModReducedRowEchelonForm`][rref] and
/// [`ModReducedRowEchelonFormAssign`][rrefa], for computing the reduced row echelon form of a
/// matrix modulo a prime.
///
/// [rref]: crate::num::arithmetic::traits::ModReducedRowEchelonForm
///
/// [rrefa]: crate::num::arithmetic::traits::ModReducedRowEchelonFormAssign
pub mod mod_reduced_row_echelon_form;
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
/// Implementations of [`ModSimilarityTransform`](
/// crate::num::arithmetic::traits::ModSimilarityTransform) and
/// [`ModSimilarityTransformAssign`](crate::num::arithmetic::traits::ModSimilarityTransformAssign),
/// for applying the similarity transform of FLINT's `nmod_mat_similarity` to a square matrix modulo
/// a number.
pub mod mod_similarity_transform;
/// Implementations of [`ModSmithNormalForm`](crate::num::arithmetic::traits::ModSmithNormalForm)
/// and [`ModSmithNormalFormAssign`](crate::num::arithmetic::traits::ModSmithNormalFormAssign), for
/// computing Smith normal forms of matrices modulo any modulus.
pub mod mod_smith_normal_form;
/// Implementations of [`ModSolve`](crate::num::arithmetic::traits::ModSolve), for solving a system
/// of linear equations modulo a prime.
pub mod mod_solve;
/// Implementations of
/// [`ModSolveLowerTriangular`](crate::num::arithmetic::traits::ModSolveLowerTriangular), for
/// solving a lower triangular system of equations modulo a number.
pub mod mod_solve_lower_triangular;
/// Implementations of
/// [`ModSolveUpperTriangular`](crate::num::arithmetic::traits::ModSolveUpperTriangular), for
/// solving a upper triangular system of equations modulo a number.
pub mod mod_solve_upper_triangular;
/// Implementations of [`ModSquare`](crate::num::arithmetic::traits::ModSquare) and
/// [`ModSquareAssign`](crate::num::arithmetic::traits::ModSquareAssign), for squaring a square
/// matrix modulo a number.
pub mod mod_square;
/// Implementations of [`ModStrongEchelonForm`][sef] and [`ModStrongEchelonFormAssign`][sefa], for
/// computing the strong echelon form of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a
/// number.
///
/// [sef]: crate::num::arithmetic::traits::ModStrongEchelonForm
///
/// [sefa]: crate::num::arithmetic::traits::ModStrongEchelonFormAssign
///
/// # mod_strong_echelon_form
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModStrongEchelonForm;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let a = UnsignedMatrix::<u8>::from_str("[4, 2; 2, 0]").unwrap();
/// assert_eq!(a.mod_strong_echelon_form(8).to_string(), "[2, 0; 0, 2]");
///
/// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
/// let a = UnsignedMatrix::<u8>::from_str("[2, 1]").unwrap();
/// assert_eq!(a.mod_strong_echelon_form(4).to_string(), "[2, 1; 0, 2]");
///
/// // Each nonzero row is in the row whose index is the column of its pivot.
/// let a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
/// assert_eq!(
///     a.mod_strong_echelon_form(4).to_string(),
///     "[0, 0; 0, 2; 0, 0]"
/// );
///
/// let a = UnsignedMatrix::<u8>::from_str("[3, 6; 6, 3]").unwrap();
/// assert_eq!(a.mod_strong_echelon_form(9).to_string(), "[3, 6; 0, 0]");
/// ```
///
/// # mod_strong_echelon_form_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModStrongEchelonFormAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[4, 2; 2, 0]").unwrap();
/// a.mod_strong_echelon_form_assign(8);
/// assert_eq!(a.to_string(), "[2, 0; 0, 2]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[2, 1]").unwrap();
/// a.mod_strong_echelon_form_assign(4);
/// assert_eq!(a.to_string(), "[2, 1; 0, 2]");
/// ```
pub mod mod_strong_echelon_form;
/// Implementations of [`ModSub`](crate::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](crate::num::arithmetic::traits::ModSubAssign), for subtracting
/// [`UnsignedMatrix`](super::UnsignedMatrix)s of the same shape modulo a number.
pub mod mod_sub;

/// Implementations of [`ModSubMul`](crate::num::arithmetic::traits::ModSubMul) and
/// [`ModSubMulAssign`](crate::num::arithmetic::traits::ModSubMulAssign), for subtracting a scalar
/// multiple of one matrix from another modulo a number.
pub mod mod_sub_mul;
/// Implementations of [`ModTrace`](crate::num::arithmetic::traits::ModTrace), the sum of the
/// diagonal elements of a square matrix modulo a number.
pub mod mod_trace;
/// The permanent algorithms shared by the modular permanents of `UnsignedMatrix`s and
/// malachite-nz's `NaturalMatrix`s, generic over the ring of residues.
#[doc(hidden)]
pub mod permanent_over_ring;
/// Right-shifting a matrix (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// assert_eq!(
///     (UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap() >> 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap() >> 0u16).to_string(),
///     "[1, 2; 3, 5]"
/// );
/// assert_eq!(
///     (UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap() >> 1u32).to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[255, 128]").unwrap() >> 7u64).to_string(),
///     "[1, 1]"
/// );
/// // Shifting by at least the width zeroes every element, keeping the shape.
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[255, 128]").unwrap() >> 8u128).to_string(),
///     "[0, 0]"
/// );
/// assert_eq!(
///     (UnsignedMatrix::<u64>::from_str("[18446744073709551615, 1]").unwrap() >> 63usize)
///         .to_string(),
///     "[1, 0]"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// a >>= 1u8;
/// assert_eq!(a.to_string(), "[0, 1; 1, 2]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[255, 128]").unwrap();
/// a >>= 200u32;
/// assert_eq!(a.to_string(), "[0, 0]");
/// ```
pub mod shr;
/// The similarity transform shared by the similarity transforms of all the matrix types, generic
/// over the ring of elements.
#[doc(hidden)]
pub mod similarity_over_ring;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) for multiplying an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) by an
/// [`UnsignedMatrix`](super::UnsignedMatrix) on the right modulo a number.
pub mod vector_mod_mul;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) for
/// multiplying an [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) by an
/// [`UnsignedMatrix`](super::UnsignedMatrix) on the right modulo a power of 2.
pub mod vector_mod_power_of_2_mul;
