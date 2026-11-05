// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::{
    CharacteristicPolynomial, Determinant, DeterminantSign, GramSchmidt, GramSchmidtAssign,
    Nullspace, Permanent, Rank, ReducedRowEchelonForm, ReducedRowEchelonFormAssign,
    SimilarityTransform, SimilarityTransformAssign, Solve, Trace,
};

/// Implementations of [`CharacteristicPolynomial`], for computing the characteristic polynomial of
/// a square matrix.
pub mod characteristic_polynomial;
/// Implementations of [`Determinant`], for computing the determinants of square rational matrices.
pub mod determinant;
/// Implementations of [`DeterminantSign`], for determining the signs of the determinants of square
/// rational matrices.
pub mod determinant_sign;
/// Implementations of [`GramSchmidt`] and [`GramSchmidtAssign`], for applying the Gram-Schmidt
/// process to the columns of a rational matrix.
pub mod gram_schmidt;
/// Implementations of [`Inverse`](malachite_base::num::arithmetic::traits::Inverse), for inverting
/// square rational matrices.
pub mod inverse;
/// Implementations of [`Nullspace`], for finding bases of the right nullspaces of rational
/// matrices.
pub mod nullspace;
/// Implementations of [`Permanent`], for computing the permanents of square rational matrices.
pub mod permanent;
/// Implementations of [`Rank`], for finding the ranks of rational matrices.
pub mod rank;
/// Implementations of [`ReducedRowEchelonForm`] and [`ReducedRowEchelonFormAssign`], for finding
/// the reduced row echelon forms of rational matrices.
pub mod reduced_row_echelon_form;
/// Implementations of [`SimilarityTransform`] and [`SimilarityTransformAssign`], for applying the
/// similarity transform of FLINT's `fmpq_mat_similarity` to a square matrix.
pub mod similarity_transform;
/// Implementations of [`Solve`], for solving linear systems with rational matrices.
pub mod solve;
/// An implementation of [`Trace`], the sum of the diagonal elements of a square matrix.
pub mod trace;
