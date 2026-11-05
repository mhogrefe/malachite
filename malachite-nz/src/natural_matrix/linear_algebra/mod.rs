// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::{
    CharacteristicPolynomial, Determinant, DeterminantSign, ElementaryDivisors,
    ModCharacteristicPolynomial, ModDeterminant, ModGramSchmidt, ModLeftNullspace,
    ModLuDecomposition, ModNullspace, ModPermanent, ModPowerOf2CharacteristicPolynomial,
    ModPowerOf2Determinant, ModPowerOf2GramSchmidt, ModPowerOf2HowellForm,
    ModPowerOf2HowellFormAssign, ModPowerOf2Permanent, ModPowerOf2SimilarityTransform,
    ModPowerOf2SimilarityTransformAssign, ModPowerOf2SmithNormalForm,
    ModPowerOf2SmithNormalFormAssign, ModPowerOf2Solve, ModPowerOf2StrongEchelonForm,
    ModPowerOf2StrongEchelonFormAssign, ModPowerOf2Trace, ModRank, ModSimilarityTransform,
    ModSimilarityTransformAssign, ModSmithNormalForm, ModSmithNormalFormAssign, ModSolve,
    ModSolveLowerTriangular, ModSolveUpperTriangular, ModTrace, Permanent, Rank, SmithNormalForm,
    SmithNormalFormAssign, Trace,
};

/// Implementations of [`CharacteristicPolynomial`], for computing the characteristic polynomial of
/// a square matrix of naturals.
pub mod characteristic_polynomial;
/// Implementations of [`Determinant`], for computing the determinants of square matrices of
/// naturals.
pub mod determinant;
/// Implementations of [`DeterminantSign`], for determining the signs of the determinants of square
/// matrices of naturals.
pub mod determinant_sign;
/// Implementations of [`ElementaryDivisors`], for computing the elementary divisors of matrices of
/// naturals.
pub mod elementary_divisors;
/// Implementations of [`ModCharacteristicPolynomial`], for computing characteristic polynomials of
/// square matrices modulo a number.
pub mod mod_characteristic_polynomial;
/// Implementations of [`ModDeterminant`], for computing the determinant of a square matrix modulo a
/// number.
pub mod mod_determinant;
/// Implementations of [`ModGramSchmidt`], for applying the Gram-Schmidt process to the columns of a
/// matrix modulo a [`Natural`](crate::natural::Natural).
pub mod mod_gram_schmidt;
/// Implementations of [`ModHowellForm`][hf] and [`ModHowellFormAssign`][hfa], for computing the
/// Howell form of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
///
/// [hf]: malachite_base::num::linear_algebra::traits::ModHowellForm
///
/// [hfa]: malachite_base::num::linear_algebra::traits::ModHowellFormAssign
pub mod mod_howell_form;
/// Implementations of [`ModInverse`](malachite_base::num::modular::traits::ModInverse), for
/// inverting a square matrix modulo a prime.
pub mod mod_inverse;
/// Implementations of [`ModLeftNullspace`], for computing a basis of the left nullspace of a matrix
/// modulo a prime.
pub mod mod_left_nullspace;
/// Implementations of [`ModLuDecomposition`], for computing an LU decomposition of a matrix modulo
/// a prime.
pub mod mod_lu_decomposition;
/// Implementations of [`ModNullspace`], for computing a basis of the right nullspace of a matrix
/// modulo a prime.
pub mod mod_nullspace;
/// Implementations of [`ModPermanent`], for computing the permanents of square matrices of naturals
/// modulo a number.
pub mod mod_permanent;
/// Implementations of [`ModPowerOf2CharacteristicPolynomial`], for computing characteristic
/// polynomials of square matrices modulo powers of 2.
pub mod mod_power_of_2_characteristic_polynomial;
/// Implementations of [`ModPowerOf2Determinant`], for computing determinants of square matrices of
/// naturals modulo powers of 2.
pub mod mod_power_of_2_determinant;
/// Implementations of [`ModPowerOf2GramSchmidt`], for applying the Gram-Schmidt process to the
/// columns of a matrix modulo a power of 2.
pub mod mod_power_of_2_gram_schmidt;
/// Implementations of [`ModPowerOf2HowellForm`] and [`ModPowerOf2HowellFormAssign`], for computing
/// Howell forms of matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_howell_form;
/// Implementations of
/// [`ModPowerOf2Inverse`](malachite_base::num::modular::traits::ModPowerOf2Inverse), for inverting
/// square matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_inverse;
/// Implementations of [`ModPowerOf2Permanent`], for computing the permanents of square matrices of
/// naturals modulo a power of 2.
pub mod mod_power_of_2_permanent;
/// Implementations of [`ModPowerOf2SimilarityTransform`] and
/// [`ModPowerOf2SimilarityTransformAssign`], for applying the similarity transform of FLINT's
/// `fmpz_mat_similarity` to a square matrix of naturals modulo a power of 2.
pub mod mod_power_of_2_similarity_transform;
/// Implementations of [`ModPowerOf2SmithNormalForm`] and [`ModPowerOf2SmithNormalFormAssign`], for
/// computing Smith normal forms of matrices of naturals modulo powers of 2.
pub mod mod_power_of_2_smith_normal_form;
/// Implementations of [`ModPowerOf2Solve`], for solving linear systems of naturals modulo powers of
/// 2.
pub mod mod_power_of_2_solve;
/// Implementations of [`ModPowerOf2StrongEchelonForm`] and [`ModPowerOf2StrongEchelonFormAssign`],
/// for computing strong echelon forms of matrices of naturals modulo powers of 2.
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
/// [rref]: malachite_base::num::linear_algebra::traits::ModReducedRowEchelonForm
///
/// [rrefa]: malachite_base::num::linear_algebra::traits::ModReducedRowEchelonFormAssign
pub mod mod_reduced_row_echelon_form;
/// Implementations of [`ModSimilarityTransform`] and [`ModSimilarityTransformAssign`], for applying
/// the similarity transform of FLINT's `fmpz_mod_mat_similarity` to a square matrix of naturals
/// modulo a number.
pub mod mod_similarity_transform;
/// Implementations of [`ModSmithNormalForm`] and [`ModSmithNormalFormAssign`], for computing Smith
/// normal forms of matrices of naturals modulo any modulus.
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
/// computing the strong echelon form of a [`NaturalMatrix`](super::NaturalMatrix) modulo a number.
///
/// [sef]: malachite_base::num::linear_algebra::traits::ModStrongEchelonForm
///
/// [sefa]: malachite_base::num::linear_algebra::traits::ModStrongEchelonFormAssign
pub mod mod_strong_echelon_form;
/// Implementations of [`ModTrace`], the sum of the diagonal elements of a square matrix modulo a
/// number.
pub mod mod_trace;
/// Implementations of [`Permanent`], for computing the permanents of square matrices of naturals.
pub mod permanent;
/// Implementations of [`Rank`], for finding the ranks of matrices of naturals.
pub mod rank;
/// Implementations of [`SmithNormalForm`] and [`SmithNormalFormAssign`], for computing the Smith
/// normal forms of matrices of naturals.
pub mod smith_normal_form;
/// An implementation of [`Trace`], the sum of the diagonal elements of a square matrix.
pub mod trace;
