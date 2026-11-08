// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering;

/// Calculates the trace of a square matrix: the sum of its diagonal elements.
pub trait Trace {
    type Output;

    fn to_trace(&self) -> Self::Output;

    fn into_trace(self) -> Self::Output;
}

/// Calculates the trace of a square matrix modulo $2^k$: the sum of its diagonal elements modulo
/// $2^k$.
///
/// The elements must already be reduced modulo $2^k$.
pub trait ModPowerOf2Trace {
    type Output;

    fn to_mod_power_of_2_trace(&self, pow: u64) -> Self::Output;

    fn into_mod_power_of_2_trace(self, pow: u64) -> Self::Output;
}

/// Calculates the trace of a square matrix modulo another number $m$: the sum of its diagonal
/// elements modulo $m$.
///
/// The elements must already be reduced modulo $m$.
pub trait ModTrace<M> {
    type Output;

    fn to_mod_trace(&self, m: M) -> Self::Output;

    fn into_mod_trace(self, m: M) -> Self::Output;
}

/// Computes the Howell form of a matrix modulo $m$: the unique matrix whose rows span the same
/// module over $\mathbb{Z}/m\mathbb{Z}$ as the rows of the original, in a canonical upper echelon
/// form with its zero rows last. The elements of the matrix must already be reduced modulo $m$.
pub trait ModHowellForm<M = Self> {
    type Output;

    fn mod_howell_form(self, m: M) -> Self::Output;
}

/// Replaces a matrix with its Howell form modulo $m$. The elements of the matrix must already be
/// reduced modulo $m$.
pub trait ModHowellFormAssign<M = Self> {
    fn mod_howell_form_assign(&mut self, m: M);
}

/// Computes the strong echelon form of a matrix modulo $m$: the Howell form, but with each nonzero
/// row in the row whose index is the column of its pivot. The elements of the matrix must already
/// be reduced modulo $m$.
pub trait ModStrongEchelonForm<M = Self> {
    type Output;

    fn mod_strong_echelon_form(self, m: M) -> Self::Output;
}

/// Replaces a matrix with its strong echelon form modulo $m$. The elements of the matrix must
/// already be reduced modulo $m$.
pub trait ModStrongEchelonFormAssign<M = Self> {
    fn mod_strong_echelon_form_assign(&mut self, m: M);
}

/// Computes the Howell form of a matrix modulo $2^k$: the unique matrix whose rows span the same
/// module over $\mathbb{Z}/2^k\mathbb{Z}$ as the rows of the original, in a canonical upper echelon
/// form with its zero rows last. The elements of the matrix must already be reduced modulo $2^k$.
pub trait ModPowerOf2HowellForm {
    type Output;

    fn mod_power_of_2_howell_form(self, pow: u64) -> Self::Output;
}

/// Replaces a matrix with its Howell form modulo $2^k$. The elements of the matrix must already be
/// reduced modulo $2^k$.
pub trait ModPowerOf2HowellFormAssign {
    fn mod_power_of_2_howell_form_assign(&mut self, pow: u64);
}

/// Computes the strong echelon form of a matrix modulo $2^k$: the Howell form, but with each
/// nonzero row in the row whose index is the column of its pivot. The elements of the matrix must
/// already be reduced modulo $2^k$.
pub trait ModPowerOf2StrongEchelonForm {
    type Output;

    fn mod_power_of_2_strong_echelon_form(self, pow: u64) -> Self::Output;
}

/// Replaces a matrix with its strong echelon form modulo $2^k$. The elements of the matrix must
/// already be reduced modulo $2^k$.
pub trait ModPowerOf2StrongEchelonFormAssign {
    fn mod_power_of_2_strong_echelon_form_assign(&mut self, pow: u64);
}

/// Computes an LU decomposition of a matrix modulo a prime $m$: a permutation $P$, a lower
/// triangular matrix $L$ with ones on its diagonal, and an upper triangular matrix $U$ in row
/// echelon form, such that $PA = LU$. The elements of the matrix must already be reduced modulo
/// $m$.
pub trait ModLuDecomposition<M = Self> {
    type Output;

    fn mod_lu_decomposition(self, m: M) -> Self::Output;
}

/// Computes a fraction-free LU decomposition of an integer matrix $A$: a permutation $P$, a lower
/// triangular matrix $L$, an upper triangular matrix $U$ in row echelon form, and a denominator,
/// such that $PA = LD^{-1}U$, where $D$ is a diagonal matrix formed from the pivots.
pub trait FractionFreeLuDecomposition {
    type Output;

    fn fraction_free_lu_decomposition(self) -> Self::Output;
}

/// Computes the determinant of a square matrix.
pub trait Determinant {
    type Output;

    fn determinant(self) -> Self::Output;
}

/// Computes the minor of a square matrix at row $i$ and column $j$: the determinant of the matrix
/// obtained by deleting row $i$ and column $j$.
pub trait Minor {
    type Output;

    fn minor(self, i: u64, j: u64) -> Self::Output;
}

/// Computes the cofactor of a square matrix at row $i$ and column $j$: $(-1)^{i + j}$ times its
/// minor there, so that expanding along row $i$ gives $\det A = \sum_j A_{i, j} C_{i, j}$.
pub trait Cofactor {
    type Output;

    fn cofactor(self, i: u64, j: u64) -> Self::Output;
}

/// Computes the adjugate of a square matrix: the transpose of its matrix of cofactors, which
/// satisfies $A \operatorname{adj}(A) = \operatorname{adj}(A) A = (\det A) I$.
pub trait Adjugate {
    type Output;

    fn adjugate(self) -> Self::Output;
}

/// Applies the similarity transform $A \mapsto P^{-1}AP$ of FLINT's `fmpz_mat_similarity` to a
/// square matrix, where $P$ is the identity matrix with some of the elements of row `r` replaced by
/// `d`. See the implementations for exactly which.
pub trait SimilarityTransform<D> {
    type Output;

    fn similarity_transform(self, r: u64, d: D) -> Self::Output;
}

/// Applies the similarity transform $A \mapsto P^{-1}AP$ of FLINT's `fmpz_mat_similarity` to a
/// square matrix in place, where $P$ is the identity matrix with some of the elements of row `r`
/// replaced by `d`. See the implementations for exactly which.
pub trait SimilarityTransformAssign<D> {
    fn similarity_transform_assign(&mut self, r: u64, d: D);
}

/// Applies the similarity transform $A \mapsto P^{-1}AP$ of FLINT's `nmod_mat_similarity` and
/// `fmpz_mod_mat_similarity` to a square matrix modulo $m$, where $P$ is the identity matrix with
/// some of the elements of row `r` replaced by `d`. See the implementations for exactly which. The
/// matrix and `d` must already be reduced modulo $m$.
pub trait ModSimilarityTransform<D, M = D> {
    type Output;

    fn mod_similarity_transform(self, r: u64, d: D, m: M) -> Self::Output;
}

/// Applies the similarity transform $A \mapsto P^{-1}AP$ of FLINT's `nmod_mat_similarity` and
/// `fmpz_mod_mat_similarity` to a square matrix modulo $m$, in place, where $P$ is the identity
/// matrix with some of the elements of row `r` replaced by `d`. See the implementations for exactly
/// which. The matrix and `d` must already be reduced modulo $m$.
pub trait ModSimilarityTransformAssign<D, M = D> {
    fn mod_similarity_transform_assign(&mut self, r: u64, d: D, m: M);
}

/// Applies the similarity transform $A \mapsto P^{-1}AP$ of FLINT's `fmpz_mat_similarity` to a
/// square matrix modulo $2^k$, where $P$ is the identity matrix with some of the elements of row
/// `r` replaced by `d`. See the implementations for exactly which. The matrix and `d` must already
/// be reduced modulo $2^k$.
pub trait ModPowerOf2SimilarityTransform<D> {
    type Output;

    fn mod_power_of_2_similarity_transform(self, r: u64, d: D, pow: u64) -> Self::Output;
}

/// Applies the similarity transform $A \mapsto P^{-1}AP$ of FLINT's `fmpz_mat_similarity` to a
/// square matrix modulo $2^k$, in place, where $P$ is the identity matrix with some of the elements
/// of row `r` replaced by `d`. See the implementations for exactly which. The matrix and `d` must
/// already be reduced modulo $2^k$.
pub trait ModPowerOf2SimilarityTransformAssign<D> {
    fn mod_power_of_2_similarity_transform_assign(&mut self, r: u64, d: D, pow: u64);
}

/// Computes the characteristic polynomial of a square matrix.
pub trait CharacteristicPolynomial {
    type Output;

    fn characteristic_polynomial(self) -> Self::Output;
}

/// Computes the characteristic polynomial of a square matrix modulo $m$. The elements of the matrix
/// must already be reduced modulo $m$.
pub trait ModCharacteristicPolynomial<M = Self> {
    type Output;

    fn mod_characteristic_polynomial(self, m: M) -> Self::Output;
}

/// Computes the characteristic polynomial of a square matrix modulo $2^k$. The elements of the
/// matrix must already be reduced modulo $2^k$.
pub trait ModPowerOf2CharacteristicPolynomial {
    type Output;

    fn mod_power_of_2_characteristic_polynomial(self, pow: u64) -> Self::Output;
}

/// Computes the permanent of a square matrix modulo $m$. The elements of the matrix must already be
/// reduced modulo $m$.
pub trait ModPermanent<M = Self> {
    type Output;

    fn mod_permanent(self, m: M) -> Self::Output;
}

/// Computes the permanent of a square matrix modulo $2^k$. The elements of the matrix must already
/// be reduced modulo $2^k$.
pub trait ModPowerOf2Permanent {
    type Output;

    fn mod_power_of_2_permanent(self, pow: u64) -> Self::Output;
}

/// Computes the permanent of a square matrix.
pub trait Permanent {
    type Output;

    fn permanent(self) -> Self::Output;
}

/// Determines the sign of the determinant of a square matrix, as the ordering of the determinant
/// against 0, without computing the determinant when it can be avoided.
pub trait DeterminantSign {
    fn determinant_sign(self) -> Ordering;
}

/// Computes the rank of a matrix: the number of linearly independent rows, or equivalently columns.
pub trait Rank {
    fn rank(self) -> u64;
}

/// Computes the reduced row echelon form of a matrix. For a matrix over a field, the result is the
/// reduced row echelon form itself; for an integer matrix, it is the form scaled by a denominator,
/// together with that denominator.
pub trait ReducedRowEchelonForm {
    type Output;

    fn reduced_row_echelon_form(self) -> Self::Output;
}

/// Replaces a matrix with its reduced row echelon form. For an integer matrix, the form is scaled
/// by a denominator, which is returned.
pub trait ReducedRowEchelonFormAssign {
    type Output;

    fn reduced_row_echelon_form_assign(&mut self) -> Self::Output;
}

/// Computes a basis of the right nullspace of a matrix: the vectors $x$ with $Ax = 0$. The basis
/// vectors are the columns of the result.
pub trait Nullspace {
    type Output;

    fn nullspace(self) -> Self::Output;
}

/// Computes the (row) Hermite normal form of a matrix: the unique matrix in row echelon form whose
/// rows span the same lattice, with positive pivots and with the elements above each pivot
/// nonnegative and less than it.
pub trait HermiteNormalForm {
    type Output;

    fn hermite_normal_form(self) -> Self::Output;
}

/// Replaces a matrix with its (row) Hermite normal form.
pub trait HermiteNormalFormAssign {
    fn hermite_normal_form_assign(&mut self);
}

/// Computes the (row) Hermite normal form $H$ of a matrix $A$, together with a unimodular matrix
/// $U$ such that $UA = H$.
pub trait HermiteNormalFormAndTransform {
    type Output;

    fn hermite_normal_form_and_transform(self) -> Self::Output;
}

/// Computes the Smith normal form of a matrix: the unique diagonal matrix $S = UAV$, for unimodular
/// $U$ and $V$, whose diagonal elements are nonnegative and each divide the next.
pub trait SmithNormalForm {
    type Output;

    fn smith_normal_form(self) -> Self::Output;
}

/// Replaces a matrix with its Smith normal form.
pub trait SmithNormalFormAssign {
    fn smith_normal_form_assign(&mut self);
}

/// Computes the Smith normal form of a matrix modulo $m$: the unique diagonal matrix $UAV$ over
/// $\mathbb{Z}/m\mathbb{Z}$, for invertible $U$ and $V$, whose diagonal elements are divisors of
/// $m$ that each divide the next, with 0 standing for $m$. The elements of the matrix must already
/// be reduced modulo $m$.
pub trait ModSmithNormalForm<M = Self> {
    type Output;

    fn mod_smith_normal_form(self, m: M) -> Self::Output;
}

/// Replaces a matrix with its Smith normal form modulo $m$. The elements of the matrix must already
/// be reduced modulo $m$.
pub trait ModSmithNormalFormAssign<M = Self> {
    fn mod_smith_normal_form_assign(&mut self, m: M);
}

/// Computes the Smith normal form of a matrix modulo $2^k$: the unique diagonal matrix $UAV$ over
/// $\mathbb{Z}/2^k\mathbb{Z}$, for invertible $U$ and $V$, whose diagonal elements are powers of 2
/// in increasing order, followed by zeros. The elements of the matrix must already be reduced
/// modulo $2^k$.
pub trait ModPowerOf2SmithNormalForm {
    type Output;

    fn mod_power_of_2_smith_normal_form(self, pow: u64) -> Self::Output;
}

/// Replaces a matrix with its Smith normal form modulo $2^k$. The elements of the matrix must
/// already be reduced modulo $2^k$.
pub trait ModPowerOf2SmithNormalFormAssign {
    fn mod_power_of_2_smith_normal_form_assign(&mut self, pow: u64);
}

/// Computes the Smith normal form $S$ of a matrix $A$, together with unimodular matrices $U$ and
/// $V$ such that $UAV = S$.
pub trait SmithNormalFormAndTransforms {
    type Output;

    fn smith_normal_form_and_transforms(self) -> Self::Output;
}

/// Computes the elementary divisors of a matrix: the nonzero diagonal elements of its Smith normal
/// form, one for each unit of rank, in increasing order of divisibility.
pub trait ElementaryDivisors {
    type Output;

    fn elementary_divisors(self) -> Self::Output;
}

/// Computes the rank of a matrix modulo a prime $m$. The elements of the matrix must already be
/// reduced modulo $m$.
pub trait ModRank<M = Self> {
    fn mod_rank(self, m: M) -> u64;
}

/// Computes the determinant of a square matrix modulo $m$. The elements of the matrix must already
/// be reduced modulo $m$.
pub trait ModDeterminant<M = Self> {
    type Output;

    fn mod_determinant(self, m: M) -> Self::Output;
}

/// Computes the determinant of a square matrix modulo $2^k$. The elements of the matrix must
/// already be reduced modulo $2^k$.
pub trait ModPowerOf2Determinant {
    type Output;

    fn mod_power_of_2_determinant(self, pow: u64) -> Self::Output;
}

/// Computes the minor of a square matrix at row $i$ and column $j$ modulo $m$: the determinant
/// modulo $m$ of the matrix obtained by deleting row $i$ and column $j$. The elements of the matrix
/// must already be reduced modulo $m$.
pub trait ModMinor<M = Self> {
    type Output;

    fn mod_minor(self, i: u64, j: u64, m: M) -> Self::Output;
}

/// Computes the cofactor of a square matrix at row $i$ and column $j$ modulo $m$: $(-1)^{i + j}$
/// times its minor there, modulo $m$. The elements of the matrix must already be reduced modulo
/// $m$.
pub trait ModCofactor<M = Self> {
    type Output;

    fn mod_cofactor(self, i: u64, j: u64, m: M) -> Self::Output;
}

/// Computes the adjugate of a square matrix modulo $m$: the transpose of its matrix of cofactors
/// modulo $m$, which satisfies $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det
/// A) I \pmod m$. The elements of the matrix must already be reduced modulo $m$.
pub trait ModAdjugate<M = Self> {
    type Output;

    fn mod_adjugate(self, m: M) -> Self::Output;
}

/// Computes the minor of a square matrix at row $i$ and column $j$ modulo $2^k$: the determinant
/// modulo $2^k$ of the matrix obtained by deleting row $i$ and column $j$. The elements of the
/// matrix must already be reduced modulo $2^k$.
pub trait ModPowerOf2Minor {
    type Output;

    fn mod_power_of_2_minor(self, i: u64, j: u64, pow: u64) -> Self::Output;
}

/// Computes the cofactor of a square matrix at row $i$ and column $j$ modulo $2^k$: $(-1)^{i + j}$
/// times its minor there, modulo $2^k$. The elements of the matrix must already be reduced modulo
/// $2^k$.
pub trait ModPowerOf2Cofactor {
    type Output;

    fn mod_power_of_2_cofactor(self, i: u64, j: u64, pow: u64) -> Self::Output;
}

/// Computes the adjugate of a square matrix modulo $2^k$: the transpose of its matrix of cofactors
/// modulo $2^k$, which satisfies $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv
/// (\det A) I \pmod{2^k}$. The elements of the matrix must already be reduced modulo $2^k$.
pub trait ModPowerOf2Adjugate {
    type Output;

    fn mod_power_of_2_adjugate(self, pow: u64) -> Self::Output;
}

/// Computes the reduced row echelon form of a matrix modulo a prime $m$. The elements of the matrix
/// must already be reduced modulo $m$.
pub trait ModReducedRowEchelonForm<M = Self> {
    type Output;

    fn mod_reduced_row_echelon_form(self, m: M) -> Self::Output;
}

/// Replaces a matrix with its reduced row echelon form modulo a prime $m$. The elements of the
/// matrix must already be reduced modulo $m$.
pub trait ModReducedRowEchelonFormAssign<M = Self> {
    fn mod_reduced_row_echelon_form_assign(&mut self, m: M);
}

/// Computes a basis of the right nullspace of a matrix modulo a prime $m$: the vectors $x$ with $Ax
/// = 0$. The basis vectors are the columns of the result. The elements of the matrix must already
/// be reduced modulo $m$.
pub trait ModNullspace<M = Self> {
    type Output;

    fn mod_nullspace(self, m: M) -> Self::Output;
}

/// Computes a basis of the left nullspace of a matrix modulo a prime $m$: the vectors $x$ with $xA
/// = 0$. The basis vectors are the rows of the result. The elements of the matrix must already be
/// reduced modulo $m$.
pub trait ModLeftNullspace<M = Self> {
    type Output;

    fn mod_left_nullspace(self, m: M) -> Self::Output;
}

/// Solves $AX = B$ (or $Ax = b$) for $X$, returning a solution if there is one and `None`
/// otherwise.
pub trait Solve<RHS = Self> {
    type Output;

    fn solve(self, b: RHS) -> Option<Self::Output>;
}

/// Solves $AX = B$ (or $Ax = b$) modulo a prime $m$ for $X$, returning a solution if there is one
/// and `None` otherwise. The elements of $A$ and $B$ must already be reduced modulo $m$.
pub trait ModSolve<RHS = Self, M = Self> {
    type Output;

    fn mod_solve(self, b: RHS, m: M) -> Option<Self::Output>;
}

/// Solves $AX = B$ (or $Ax = b$) modulo $2^k$ for $X$, returning a solution if there is one and
/// `None` otherwise. The elements of $A$ and $B$ must already be reduced modulo $2^k$.
pub trait ModPowerOf2Solve<RHS = Self> {
    type Output;

    fn mod_power_of_2_solve(self, b: RHS, pow: u64) -> Option<Self::Output>;
}

/// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a square lower triangular matrix. Only the
/// elements of $L$ on and below its diagonal are used; if `unit_diagonal` is `true`, the diagonal
/// elements are taken to be 1 and are not used either.
pub trait ModSolveLowerTriangular<RHS = Self, M = Self> {
    type Output;

    fn mod_solve_lower_triangular(self, b: RHS, m: M, unit_diagonal: bool) -> Self::Output;
}

/// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a square upper triangular matrix. Only the
/// elements of $U$ on and above its diagonal are used; if `unit_diagonal` is `true`, the diagonal
/// elements are taken to be 1 and are not used either.
pub trait ModSolveUpperTriangular<RHS = Self, M = Self> {
    type Output;

    fn mod_solve_upper_triangular(self, b: RHS, m: M, unit_diagonal: bool) -> Self::Output;
}

/// Applies the Gram-Schmidt process to the columns of a matrix, without normalizing them.
pub trait GramSchmidt {
    type Output;

    fn gram_schmidt(self) -> Self::Output;
}

/// Applies the Gram-Schmidt process to the columns of a matrix, in place, without normalizing them.
pub trait GramSchmidtAssign {
    fn gram_schmidt_assign(&mut self);
}

/// Applies the Gram-Schmidt process to the columns of a matrix modulo $m$, without normalizing
/// them, or returns `None` if a squared length it needs to divide by is not invertible modulo $m$.
/// The elements of the matrix must already be reduced modulo $m$.
pub trait ModGramSchmidt<M = Self> {
    type Output;

    fn mod_gram_schmidt(self, m: M) -> Option<Self::Output>;
}

/// Applies the Gram-Schmidt process to the columns of a matrix modulo $2^k$, without normalizing
/// them, or returns `None` if a squared length it needs to divide by is even. The elements of the
/// matrix must already be reduced modulo $2^k$.
pub trait ModPowerOf2GramSchmidt {
    type Output;

    fn mod_power_of_2_gram_schmidt(self, pow: u64) -> Option<Self::Output>;
}

/// Determines whether vectors are linearly independent: whether no nontrivial linear combination of
/// them is zero. The vectors are given as a slice, and must all have the same dimension.
pub trait AreLinearlyIndependent: Sized {
    fn are_linearly_independent(xs: &[Self]) -> bool;
}

/// Determines whether vectors are linearly independent modulo $m$: whether no nontrivial linear
/// combination of them, with coefficients in $\mathbb{Z}/m\mathbb{Z}$, is zero modulo $m$. The
/// vectors are given as a slice, must all have the same dimension, and must already be reduced
/// modulo $m$.
pub trait ModAreLinearlyIndependent<M = Self>: Sized {
    fn mod_are_linearly_independent(xs: &[Self], m: M) -> bool;
}

/// Determines whether vectors are linearly independent modulo $2^k$: whether no nontrivial linear
/// combination of them, with coefficients in $\mathbb{Z}/2^k\mathbb{Z}$, is zero modulo $2^k$. The
/// vectors are given as a slice, must all have the same dimension, and must already be reduced
/// modulo $2^k$.
pub trait ModPowerOf2AreLinearlyIndependent: Sized {
    fn mod_power_of_2_are_linearly_independent(xs: &[Self], pow: u64) -> bool;
}

/// Determines whether points, given as vectors, are affinely independent: whether no nontrivial
/// linear combination of them whose coefficients sum to zero is zero. The points are given as a
/// slice, and must all have the same dimension.
pub trait AreAffinelyIndependent: Sized {
    fn are_affinely_independent(xs: &[Self]) -> bool;
}

/// Determines whether points, given as vectors, are affinely independent modulo $m$: whether no
/// nontrivial linear combination of them, with coefficients in $\mathbb{Z}/m\mathbb{Z}$ that sum to
/// zero, is zero modulo $m$. The points are given as a slice, must all have the same dimension, and
/// must already be reduced modulo $m$.
pub trait ModAreAffinelyIndependent<M = Self>: Sized {
    fn mod_are_affinely_independent(xs: &[Self], m: M) -> bool;
}

/// Determines whether points, given as vectors, are affinely independent modulo $2^k$: whether no
/// nontrivial linear combination of them, with coefficients in $\mathbb{Z}/2^k\mathbb{Z}$ that sum
/// to zero, is zero modulo $2^k$. The points are given as a slice, must all have the same
/// dimension, and must already be reduced modulo $2^k$.
pub trait ModPowerOf2AreAffinelyIndependent: Sized {
    fn mod_power_of_2_are_affinely_independent(xs: &[Self], pow: u64) -> bool;
}
