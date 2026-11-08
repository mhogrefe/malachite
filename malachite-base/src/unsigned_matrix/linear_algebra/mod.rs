// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use crate::num::linear_algebra::traits::{
    ModAdjugate, ModCharacteristicPolynomial, ModCofactor, ModDeterminant, ModGramSchmidt,
    ModHowellForm, ModHowellFormAssign, ModLeftNullspace, ModLuDecomposition, ModMinor,
    ModNullspace, ModPermanent, ModPowerOf2Adjugate, ModPowerOf2CharacteristicPolynomial,
    ModPowerOf2Cofactor, ModPowerOf2Determinant, ModPowerOf2GramSchmidt, ModPowerOf2HowellForm,
    ModPowerOf2HowellFormAssign, ModPowerOf2Minor, ModPowerOf2Permanent,
    ModPowerOf2SimilarityTransform, ModPowerOf2SimilarityTransformAssign,
    ModPowerOf2SmithNormalForm, ModPowerOf2SmithNormalFormAssign, ModPowerOf2Solve,
    ModPowerOf2StrongEchelonForm, ModPowerOf2StrongEchelonFormAssign, ModPowerOf2Trace, ModRank,
    ModSimilarityTransform, ModSimilarityTransformAssign, ModSmithNormalForm,
    ModSmithNormalFormAssign, ModSolve, ModSolveLowerTriangular, ModSolveUpperTriangular, ModTrace,
};

/// Berkowitz's division-free algorithm for characteristic polynomials, generic over the ring of
/// elements.
#[doc(hidden)]
pub mod berkowitz_over_ring;
/// Danilevsky's method and the reduction to Hessenberg form for characteristic polynomials, generic
/// over the ring of elements.
#[doc(hidden)]
pub mod charpoly_over_ring;
/// The Gram-Schmidt process shared by the Gram-Schmidt orthogonalizations of all the matrix types,
/// generic over the ring of elements.
#[doc(hidden)]
pub mod gram_schmidt_over_ring;
/// Implementations of [`ModAdjugate`], for computing the adjugates of square matrices modulo a
/// number.
pub mod mod_adjugate;
/// Implementations of [`ModCharacteristicPolynomial`], for computing characteristic polynomials of
/// square matrices modulo a number.
pub mod mod_characteristic_polynomial;
/// Implementations of [`ModCofactor`], for computing the cofactors of square matrices modulo a
/// number.
pub mod mod_cofactor;
/// Implementations of [`ModDeterminant`], for computing the determinant of a square matrix modulo a
/// number.
pub mod mod_determinant;
/// Implementations of [`ModGramSchmidt`], for applying the Gram-Schmidt process to the columns of a
/// matrix modulo a number.
pub mod mod_gram_schmidt;
/// Implementations of [`ModHowellForm`] and [`ModHowellFormAssign`], for computing the Howell form
/// of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
///
/// # mod_howell_form
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::linear_algebra::traits::ModHowellForm;
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
/// use malachite_base::num::linear_algebra::traits::ModHowellFormAssign;
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
/// Implementations of [`ModInverse`](crate::num::modular::traits::ModInverse), for inverting a
/// square matrix modulo a prime.
pub mod mod_inverse;
/// Implementations of [`ModLeftNullspace`], for computing a basis of the left nullspace of a matrix
/// modulo a prime.
pub mod mod_left_nullspace;
/// Implementations of [`ModLuDecomposition`], for computing an LU decomposition of a matrix modulo
/// a prime.
pub mod mod_lu_decomposition;
/// Implementations of [`ModMinor`], for computing the minors of square matrices modulo a number.
pub mod mod_minor;
/// Implementations of [`ModNullspace`], for computing a basis of the right nullspace of a matrix
/// modulo a prime.
pub mod mod_nullspace;
/// Implementations of [`ModPermanent`], for computing the permanents of square matrices of unsigned
/// integers modulo a number.
pub mod mod_permanent;
/// Implementations of [`ModPowerOf2Adjugate`], for computing the adjugates of square matrices
/// modulo powers of 2.
pub mod mod_power_of_2_adjugate;
/// Implementations of [`ModPowerOf2CharacteristicPolynomial`], for computing characteristic
/// polynomials of square matrices modulo powers of 2.
pub mod mod_power_of_2_characteristic_polynomial;
/// Implementations of [`ModPowerOf2Cofactor`], for computing the cofactors of square matrices
/// modulo powers of 2.
pub mod mod_power_of_2_cofactor;
/// Implementations of [`ModPowerOf2Determinant`], for computing determinants of square matrices
/// modulo powers of 2.
pub mod mod_power_of_2_determinant;
/// Implementations of [`ModPowerOf2GramSchmidt`], for applying the Gram-Schmidt process to the
/// columns of a matrix modulo a power of 2.
pub mod mod_power_of_2_gram_schmidt;
/// Implementations of [`ModPowerOf2HowellForm`] and [`ModPowerOf2HowellFormAssign`], for computing
/// Howell forms of matrices modulo powers of 2.
pub mod mod_power_of_2_howell_form;
/// Implementations of [`ModPowerOf2Inverse`](crate::num::modular::traits::ModPowerOf2Inverse), for
/// inverting square matrices modulo powers of 2.
pub mod mod_power_of_2_inverse;
/// Implementations of [`ModPowerOf2Minor`], for computing the minors of square matrices modulo
/// powers of 2.
pub mod mod_power_of_2_minor;
/// Implementations of [`ModPowerOf2Permanent`], for computing the permanents of square matrices of
/// unsigned integers modulo a power of 2.
pub mod mod_power_of_2_permanent;
/// Implementations of [`ModPowerOf2SimilarityTransform`] and
/// [`ModPowerOf2SimilarityTransformAssign`], for applying the similarity transform of FLINT's
/// `fmpz_mat_similarity` to a square matrix modulo a power of 2.
pub mod mod_power_of_2_similarity_transform;
/// Implementations of [`ModPowerOf2SmithNormalForm`] and [`ModPowerOf2SmithNormalFormAssign`], for
/// computing Smith normal forms of matrices modulo powers of 2.
pub mod mod_power_of_2_smith_normal_form;
/// Implementations of [`ModPowerOf2Solve`], for solving linear systems modulo powers of 2.
pub mod mod_power_of_2_solve;
/// Implementations of [`ModPowerOf2StrongEchelonForm`] and [`ModPowerOf2StrongEchelonFormAssign`],
/// for computing strong echelon forms of matrices modulo powers of 2.
pub mod mod_power_of_2_strong_echelon_form;
/// An implementation of [`ModPowerOf2Trace`], the sum of the diagonal elements of a square matrix
/// modulo a power of 2.
pub mod mod_power_of_2_trace;
/// Implementations of [`ModRank`], for computing the rank of a matrix modulo a prime.
pub mod mod_rank;
/// Implementations of [`ModReducedRowEchelonForm`][rref] and
/// [`ModReducedRowEchelonFormAssign`][rrefa], for computing the reduced row echelon form of a
/// matrix modulo a prime.
///
/// [rref]: crate::num::linear_algebra::traits::ModReducedRowEchelonForm
///
/// [rrefa]: crate::num::linear_algebra::traits::ModReducedRowEchelonFormAssign
pub mod mod_reduced_row_echelon_form;
/// Implementations of [`ModSimilarityTransform`] and [`ModSimilarityTransformAssign`], for applying
/// the similarity transform of FLINT's `nmod_mat_similarity` to a square matrix modulo a number.
pub mod mod_similarity_transform;
/// Implementations of [`ModSmithNormalForm`] and [`ModSmithNormalFormAssign`], for computing Smith
/// normal forms of matrices modulo any modulus.
pub mod mod_smith_normal_form;
/// Implementations of [`ModSolve`], for solving a system of linear equations modulo a prime.
pub mod mod_solve;
/// Implementations of [`ModSolveLowerTriangular`], for solving a lower triangular system of
/// equations modulo a number.
pub mod mod_solve_lower_triangular;
/// Implementations of [`ModSolveUpperTriangular`], for solving a upper triangular system of
/// equations modulo a number.
pub mod mod_solve_upper_triangular;
/// Implementations of [`ModStrongEchelonForm`][sef] and [`ModStrongEchelonFormAssign`][sefa], for
/// computing the strong echelon form of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a
/// number.
///
/// [sef]: crate::num::linear_algebra::traits::ModStrongEchelonForm
///
/// [sefa]: crate::num::linear_algebra::traits::ModStrongEchelonFormAssign
///
/// # mod_strong_echelon_form
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::linear_algebra::traits::ModStrongEchelonForm;
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
/// use malachite_base::num::linear_algebra::traits::ModStrongEchelonFormAssign;
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
/// Implementations of [`ModTrace`], the sum of the diagonal elements of a square matrix modulo a
/// number.
pub mod mod_trace;
/// The permanent algorithms shared by the modular permanents of `UnsignedMatrix`s and
/// malachite-nz's `NaturalMatrix`s, generic over the ring of residues.
#[doc(hidden)]
pub mod permanent_over_ring;
/// The similarity transform shared by the similarity transforms of all the matrix types, generic
/// over the ring of elements.
#[doc(hidden)]
pub mod similarity_over_ring;
