// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::{
    Adjugate, CharacteristicPolynomial, Cofactor, Determinant, DeterminantSign, ElementaryDivisors,
    HermiteNormalForm, HermiteNormalFormAssign, Minor, Nullspace, Permanent, Rank,
    ReducedRowEchelonForm, ReducedRowEchelonFormAssign, SimilarityTransform,
    SimilarityTransformAssign, SmithNormalForm, SmithNormalFormAndTransforms,
    SmithNormalFormAssign, Solve, Trace,
};

crate_test_mod! {
    /// Solving a linear system of any shape by Dixon's p-adic lifting. Internal machinery for
    /// malachite-q's `RationalMatrix` solvers, public only under `test_build`.
    can_solve_dixon
}
crate_test_mod! {
    /// Solving a linear system of any shape with the multimodular algorithm. Internal machinery for
    /// [`Solve`], public only under `test_build`.
    can_solve_multi_mod
}
crate_test_mod! {
    /// Certifying, from an LU decomposition modulo a prime, the rank of an integer matrix. Internal
    /// machinery for [`Solve`], public only under `test_build`.
    certify_lu_mod_p
}
/// Implementations of [`Adjugate`], for computing the adjugates of square matrices.
pub mod adjugate;
/// Implementations of [`CharacteristicPolynomial`], for computing the characteristic polynomial of
/// a square matrix.
pub mod characteristic_polynomial;
/// A bound on the coefficients of the characteristic polynomial of a square matrix.
pub mod characteristic_polynomial_bound;
/// Implementations of [`Cofactor`], for computing the cofactors of square matrices.
pub mod cofactor;
/// Implementations of [`Determinant`], for computing the determinants of square integer matrices.
pub mod determinant;
/// The method [`determinant_bound`](super::IntegerMatrix::determinant_bound), an upper bound for
/// the absolute value of the determinant from Hadamard's inequality.
pub mod determinant_bound;
crate_test_mod! {
    /// A divisor of the determinant of a square integer matrix, found by solving a linear system.
    /// Internal machinery for [`Determinant`], public only under `test_build`.
    determinant_divisor
}
/// Implementations of [`DeterminantSign`], for determining the signs of the determinants of square
/// integer matrices.
pub mod determinant_sign;
/// Implementations of [`ElementaryDivisors`], for computing the elementary divisors of integer
/// matrices.
pub mod elementary_divisors;
/// Implementations of [`FractionFreeLuDecomposition`][fflu], for computing a fraction-free LU
/// decomposition of a matrix.
///
/// [fflu]: malachite_base::num::linear_algebra::traits::FractionFreeLuDecomposition
pub mod fraction_free_lu_decomposition;
/// Implementations of [`HermiteNormalForm`] and [`HermiteNormalFormAssign`], for computing the
/// Hermite normal forms of integer matrices.
pub mod hermite_normal_form;
/// Implementations of [`HermiteNormalFormAndTransform`][hnft], for computing the Hermite normal
/// forms of integer matrices together with transformation matrices.
///
/// [hnft]: malachite_base::num::linear_algebra::traits::HermiteNormalFormAndTransform
pub mod hermite_normal_form_and_transform;
crate_test_mod! {
    /// The algorithm of Pernet and Stein for computing Hermite normal forms. Internal machinery for
    /// [`HermiteNormalForm`], public only under `test_build`.
    hermite_normal_form_pernet_stein
}
/// Implementations of [`Inverse`](malachite_base::num::arithmetic::traits::Inverse), for inverting
/// square integer matrices.
pub mod inverse;
/// The method [`is_in_hermite_normal_form`](super::IntegerMatrix::is_in_hermite_normal_form), which
/// checks whether a matrix is in Hermite normal form.
pub mod is_in_hermite_normal_form;
crate_test_mod! {
    /// A check of whether a matrix is in fraction-free reduced row echelon form with a given
    /// denominator and rank. Internal machinery for the rank certificates of [`Solve`] and a future
    /// reduced row echelon form, public only under `test_build`.
    is_in_reduced_row_echelon_form_with_rank
}
/// The method [`is_in_smith_normal_form`](super::IntegerMatrix::is_in_smith_normal_form), which
/// checks whether a matrix is in Smith normal form.
pub mod is_in_smith_normal_form;
/// Implementations of [`Minor`], for computing the minors of square matrices.
pub mod minor;
/// Implementations of [`Nullspace`], for finding bases of the right nullspaces of integer matrices.
pub mod nullspace;
/// Implementations of [`Permanent`], for computing the permanents of square integer matrices.
pub mod permanent;
/// Implementations of [`Rank`], for finding the ranks of integer matrices.
pub mod rank;
/// Implementations of [`ReducedRowEchelonForm`] and [`ReducedRowEchelonFormAssign`], for finding
/// the fraction-free reduced row echelon forms of integer matrices.
pub mod reduced_row_echelon_form;
/// Implementations of [`SimilarityTransform`] and [`SimilarityTransformAssign`], for applying the
/// similarity transform of FLINT's `fmpz_mat_similarity` to a square matrix.
pub mod similarity_transform;
/// Implementations of [`SmithNormalForm`] and [`SmithNormalFormAssign`], for computing the Smith
/// normal forms of integer matrices.
pub mod smith_normal_form;
/// Implementations of [`SmithNormalFormAndTransforms`], for computing the Smith normal forms of
/// integer matrices together with transformation matrices.
pub mod smith_normal_form_and_transforms;
/// Implementations of [`Solve`], for solving linear systems over the rationals with integer
/// matrices.
pub mod solve;
/// The method [`solve_bounds`](super::IntegerMatrix::solve_bounds), upper bounds for the numerators
/// and denominators of the solution of a linear system.
pub mod solve_bounds;
crate_test_mod! {
    /// Solving a square linear system by Dixon's p-adic lifting. Internal machinery for [`Solve`],
    /// public only under `test_build`.
    solve_dixon
}
crate_test_mod! {
    /// Solving a square linear system with the multimodular algorithm. Internal machinery for
    /// [`Solve`], public only under `test_build`.
    solve_multi_mod
}
/// The method [`submatrix_determinant_bound`](super::IntegerMatrix::submatrix_determinant_bound),
/// an upper bound for the absolute values of the determinants of the square submatrices of a
/// matrix.
pub mod submatrix_determinant_bound;
/// An implementation of [`Trace`], the sum of the diagonal elements of a square matrix.
pub mod trace;
crate_test_mod! {
    /// LU decompositions of integer matrices modulo word-sized primes, and the solving of systems
    /// with them. Internal machinery for [`Solve`], public only under `test_build`.
    word_lu
}
