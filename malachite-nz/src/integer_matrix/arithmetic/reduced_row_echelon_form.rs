// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010-2012, 2025, 2026 Fredrik Johansson
//
//      Copyright © 2014 Alex J. Best
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::certify_lu_mod_p::rref_upper_certify_lu_mod_p;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::fflu_in_place_with_options;
use crate::integer_matrix::arithmetic::mul::multi_mod::next_prime;
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    DivExact, ReducedRowEchelonForm, ReducedRowEchelonFormAssign, SubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::*;

// The pivot columns of the first `rank` rows of the `width`-wide matrix with elements `xs`, which
// is in row echelon form, followed by the other columns, in increasing order.
pub(crate) fn pivots_and_nonpivots(xs: &[Integer], width: usize, rank: usize) -> Vec<usize> {
    let mut pivots = Vec::with_capacity(width);
    let mut nonpivots = Vec::with_capacity(width - rank);
    let mut j = 0;
    for row in xs.chunks_exact(width).take(rank) {
        while row[j] == 0u32 {
            nonpivots.push(j);
            j += 1;
        }
        pivots.push(j);
        j += 1;
    }
    nonpivots.extend(j..width);
    pivots.extend(nonpivots);
    pivots
}

// Replaces the matrix `a` with its fraction-free reduced row echelon form, returning the rank and
// the denominator: the rows of the fraction-free LU decomposition are reduced by back substitution
// so that each pivot column holds the last pivot, `den`, in its pivot's row and zeros elsewhere.
//
// This is equivalent to `fmpz_mat_rref_fflu` from `fmpz_mat/rref_fflu.c`, FLINT 3.6.0.
crate_test_fn! {
rref_fflu(a: &mut IntegerMatrix) -> (usize, Integer) {
    let (m, n) = (a.height, a.width);
    let mut perm: Vec<usize> = (0..m).collect();
    let (rank, den) = fflu_in_place_with_options(&mut a.elements, m, n, &mut perm, false, true);
    let r = &mut a.elements;
    // Clear the bottom.
    for x in &mut r[rank * n..] {
        *x = Integer::ZERO;
    }
    // Convert the row echelon form to the reduced row echelon form.
    if rank > 1 {
        let pivots_nonpivots = pivots_and_nonpivots(r, n, rank);
        let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
        for &np in nonpivots {
            for i in (0..rank - 1).rev() {
                let mut t = &den * &r[i * n + np];
                for (j, &pivot) in pivots.iter().enumerate().skip(i + 1) {
                    t.sub_mul_assign(&r[i * n + pivot], &r[j * n + np]);
                }
                r[i * n + np] = t.div_exact(&r[i * n + pivots[i]]);
            }
        }
        // Clear the pivot columns.
        for (i, &pivot) in pivots.iter().enumerate() {
            for j in 0..rank {
                r[j * n + pivot] = if i == j { den.clone() } else { Integer::ZERO };
            }
        }
    }
    (rank, den)
}}

// The primes, just above 2^16, modulo which `rref_mul` finds the structure of the reduced row
// echelon form.
const RREF_MUL_MODULUS_MIN: Limb = 1 << 16;

// Returns the fraction-free reduced row echelon form of `a`, with its rank and denominator: the
// rank and the pivot columns are found from an LU decomposition modulo a prime, and the rows of the
// form are computed over the integers by solving a nonsingular system and then certified; if the
// certification fails, the prime was unlucky and another is tried.
//
// This is equivalent to `fmpz_mat_rref_mul` from `fmpz_mat/rref_mul.c`, FLINT 3.6.0.
crate_test_fn! {
rref_mul(a: &IntegerMatrix) -> (usize, IntegerMatrix, Integer) {
    let (m, n) = (a.height, a.width);
    let mut p = RREF_MUL_MODULUS_MIN;
    loop {
        p = next_prime(p);
        let lu = WordLu::new(a, p);
        let rank = lu.rank;
        // Stop early if the rank is the number of columns: the form is the identity, with zero rows
        // below it, and the denominator is 1.
        if rank == n {
            let mut r = vec![Integer::ZERO; m * n];
            for i in 0..n {
                r[i * (n + 1)] = Integer::ONE;
            }
            return (
                rank,
                IntegerMatrix {
                    elements: r,
                    height: m,
                    width: n,
                },
                Integer::ONE,
            );
        }
        if let Some((e, den)) = rref_upper_certify_lu_mod_p(a, rank, &lu.perm, &lu.pivots_nonpivots)
        {
            let mut r = e.elements;
            r.resize(m * n, Integer::ZERO);
            return (
                rank,
                IntegerMatrix {
                    elements: r,
                    height: m,
                    width: n,
                },
                den,
            );
        }
    }
}}

// Whether `fmpz_mat_rref` uses the fraction-free method for an `r` by `c` matrix: when there are at
// most 3 rows or at most 2 columns, or when the matrix is wide: wider than tall with at most 20
// rows, or, with 21 to 100 rows, wider than tall by a margin that grows with the height. The margin
// is computed in double precision, as in FLINT.
fn rref_uses_fflu(r: usize, c: usize) -> bool {
    if r <= 3 || c <= 2 || (r <= 20 && c > r) {
        return true;
    }
    if r > 20 && r <= 100 {
        let rf = f64::rounding_from(u64::exact_from(r), Nearest).0;
        let margin = rf + (rf - 20.0) / 80.0 * rf;
        return f64::rounding_from(u64::exact_from(c), Nearest).0 > margin;
    }
    false
}

// Replaces `a` with its fraction-free reduced row echelon form and returns the rank and the
// denominator, choosing the method as `fmpz_mat_rref` does.
//
// This is equivalent to `fmpz_mat_rref` from `fmpz_mat/rref.c`, FLINT 3.6.0.
crate_test_fn! {
rref_in_place(a: &mut IntegerMatrix) -> (usize, Integer) {
    if rref_uses_fflu(a.height, a.width) {
        rref_fflu(a)
    } else {
        let (rank, r, den) = rref_mul(a);
        *a = r;
        (rank, den)
    }
}}

impl ReducedRowEchelonForm for IntegerMatrix {
    type Output = (Self, Integer);

    /// Computes the fraction-free reduced row echelon form $R$ of an [`IntegerMatrix`], with its
    /// denominator $d$, taking the matrix by value.
    ///
    /// $R/d$ is the reduced row echelon form of the matrix over the rationals: $R$ has the same
    /// shape, its nonzero rows come first and their number is the rank, each pivot column has $d$
    /// in its pivot's row and zeros elsewhere, and $d$ is nonzero. $d$ is the determinant of a
    /// square submatrix of the matrix, up to sign, and is not in general the smallest denominator.
    ///
    /// The method is chosen as FLINT's `fmpz_mat_rref` chooses it: for matrices with at most 3 rows
    /// or at most 2 columns, and for wide ones, the rows of the fraction-free LU decomposition are
    /// reduced by back substitution; otherwise, the rank and pivot columns are found modulo a prime
    /// near $2^{16}$, and the rows of the form are computed by solving a nonsingular system over
    /// the integers and certified, with another prime tried if the certification fails.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ReducedRowEchelonForm;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// let (r, d) = a.reduced_row_echelon_form();
    /// assert_eq!(r.to_string(), "[-3, 0, 3; 0, -3, -6; 0, 0, 0]");
    /// assert_eq!(d.to_string(), "-3");
    ///
    /// let a = IntegerMatrix::from_str("[0, 2, 1; 0, 4, 3]").unwrap();
    /// let (r, d) = a.reduced_row_echelon_form();
    /// assert_eq!(r.to_string(), "[0, 2, 0; 0, 0, 2]");
    /// assert_eq!(d.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_rref` from `fmpz_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned.
    #[inline]
    fn reduced_row_echelon_form(mut self) -> (Self, Integer) {
        let den = self.reduced_row_echelon_form_assign();
        (self, den)
    }
}

impl ReducedRowEchelonForm for &IntegerMatrix {
    type Output = (IntegerMatrix, Integer);

    /// Computes the fraction-free reduced row echelon form $R$ of an [`IntegerMatrix`], with its
    /// denominator $d$, taking the matrix by reference.
    ///
    /// $R/d$ is the reduced row echelon form of the matrix over the rationals: $R$ has the same
    /// shape, its nonzero rows come first and their number is the rank, each pivot column has $d$
    /// in its pivot's row and zeros elsewhere, and $d$ is nonzero. $d$ is the determinant of a
    /// square submatrix of the matrix, up to sign, and is not in general the smallest denominator.
    ///
    /// The method is chosen as FLINT's `fmpz_mat_rref` chooses it: for matrices with at most 3 rows
    /// or at most 2 columns, and for wide ones, the rows of the fraction-free LU decomposition are
    /// reduced by back substitution; otherwise, the rank and pivot columns are found modulo a prime
    /// near $2^{16}$, and the rows of the form are computed by solving a nonsingular system over
    /// the integers and certified, with another prime tried if the certification fails.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ReducedRowEchelonForm;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// let (r, d) = (&a).reduced_row_echelon_form();
    /// assert_eq!(r.to_string(), "[-3, 0, 3; 0, -3, -6; 0, 0, 0]");
    /// assert_eq!(d.to_string(), "-3");
    ///
    /// let a = IntegerMatrix::from_str("[0, 2, 1; 0, 4, 3]").unwrap();
    /// let (r, d) = (&a).reduced_row_echelon_form();
    /// assert_eq!(r.to_string(), "[0, 2, 0; 0, 0, 2]");
    /// assert_eq!(d.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_rref` from `fmpz_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned.
    fn reduced_row_echelon_form(self) -> (IntegerMatrix, Integer) {
        if rref_uses_fflu(self.height, self.width) {
            let mut a = self.clone();
            let den = rref_fflu(&mut a).1;
            (a, den)
        } else {
            let (_, r, den) = rref_mul(self);
            (r, den)
        }
    }
}

impl ReducedRowEchelonFormAssign for IntegerMatrix {
    type Output = Integer;

    /// Replaces an [`IntegerMatrix`] with its fraction-free reduced row echelon form $R$, returning
    /// the denominator $d$.
    ///
    /// $R/d$ is the reduced row echelon form of the matrix over the rationals: $R$ has the same
    /// shape, its nonzero rows come first and their number is the rank, each pivot column has $d$
    /// in its pivot's row and zeros elsewhere, and $d$ is nonzero. $d$ is the determinant of a
    /// square submatrix of the matrix, up to sign, and is not in general the smallest denominator.
    ///
    /// The method is chosen as FLINT's `fmpz_mat_rref` chooses it: for matrices with at most 3 rows
    /// or at most 2 columns, and for wide ones, the rows of the fraction-free LU decomposition are
    /// reduced by back substitution; otherwise, the rank and pivot columns are found modulo a prime
    /// near $2^{16}$, and the rows of the form are computed by solving a nonsingular system over
    /// the integers and certified, with another prime tried if the certification fails.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ReducedRowEchelonFormAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.reduced_row_echelon_form_assign().to_string(), "-3");
    /// assert_eq!(a.to_string(), "[-3, 0, 3; 0, -3, -6; 0, 0, 0]");
    ///
    /// let mut a = IntegerMatrix::from_str("[0, 2, 1; 0, 4, 3]").unwrap();
    /// assert_eq!(a.reduced_row_echelon_form_assign().to_string(), "2");
    /// assert_eq!(a.to_string(), "[0, 2, 0; 0, 0, 2]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_rref` from `fmpz_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned.
    #[inline]
    fn reduced_row_echelon_form_assign(&mut self) -> Integer {
        rref_in_place(self).1
    }
}
