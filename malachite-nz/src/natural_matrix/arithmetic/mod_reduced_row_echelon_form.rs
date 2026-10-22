// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019 Tommy Hofmann
//
//      Copyright © 2022, 2024 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::{
    assert_reduced, lu_in_place, non_invertible_pivot,
};
use crate::natural_matrix::arithmetic::mod_solve_lower_triangular::NaturalBlockRef;
use crate::natural_matrix::arithmetic::mod_solve_upper_triangular::solve_upper;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModReducedRowEchelonForm, ModReducedRowEchelonFormAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

// The pivot columns of the first `rank` rows of the `width`-column matrix with elements `xs`, which
// is in row echelon form, followed by its other columns, each in increasing order.
fn pivots_and_nonpivots(xs: &[Natural], width: usize, rank: usize) -> Vec<usize> {
    let mut pivots_nonpivots = vec![0; width];
    let (pivots, nonpivots) = pivots_nonpivots.split_at_mut(rank);
    let mut j = 0;
    let mut k = 0;
    for (i, pivot) in pivots.iter_mut().enumerate() {
        while xs[i * width + j] == 0u32 {
            nonpivots[k] = j;
            k += 1;
            j += 1;
        }
        *pivot = j;
        j += 1;
    }
    for (nonpivot, c) in nonpivots[k..].iter_mut().zip(j..) {
        *nonpivot = c;
    }
    pivots_nonpivots
}

// Replaces `a` with its reduced row echelon form modulo the prime `m`, which does not fit in a
// limb, and returns its rank. If the rank is nonzero, also returns the pivot columns in increasing
// order, followed by the other columns in increasing order; otherwise, returns an empty `Vec`.
//
// This is equivalent to `gr_mat_rref_lu` from `gr_mat/rref_lu.c`, FLINT 3.6.0, which
// `fmpz_mod_mat_rref` calls, except that it panics at a pivot that is not invertible, where FLINT
// aborts.
pub(crate) fn rref_in_place(a: &mut NaturalMatrix, m: &Natural) -> (usize, Vec<usize>) {
    let (height, width) = (a.height, a.width);
    // A zero matrix, including one with no elements, is already in reduced row echelon form.
    if a.elements.iter().all(|x| *x == 0u32) {
        return (0, Vec::new());
    }
    let rank = lu_in_place(a, m, false)
        .unwrap_or_else(|| non_invertible_pivot(m))
        .1;
    let xs = &mut a.elements;
    // Clear L.
    for i in 0..height {
        xs[i * width..i * width + i.min(rank)].fill(Natural::ZERO);
    }
    let pivots_nonpivots = pivots_and_nonpivots(xs, width, rank);
    let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
    if rank != width {
        // Reorder U into upper triangular form U | V, with U of full rank and triangular, set V to
        // U^-1 V, and put the columns back in their original order. The elements moved out of the
        // pivot columns are overwritten below.
        let nullity = width - rank;
        let mut u = vec![Natural::ZERO; rank * rank];
        for j in 0..rank {
            for i in j..rank {
                u[j * rank + i] = core::mem::take(&mut xs[j * width + pivots[i]]);
            }
        }
        let mut v = Vec::with_capacity(rank * nullity);
        for j in 0..rank {
            for &c in nonpivots {
                v.push(core::mem::take(&mut xs[j * width + c]));
            }
        }
        solve_upper(
            &mut v,
            NaturalBlockRef {
                xs: &u,
                width: rank,
                row: 0,
                column: 0,
            },
            rank,
            nullity,
            m,
            false,
        );
        let mut v = v.into_iter();
        for j in 0..rank {
            for &c in nonpivots {
                xs[j * width + c] = v.next().unwrap();
            }
        }
    }
    // Clear the pivot columns.
    for j in 0..rank {
        for i in j..rank {
            xs[j * width + pivots[i]] = if i == j { Natural::ONE } else { Natural::ZERO };
        }
    }
    (rank, pivots_nonpivots)
}

fn mod_reduced_row_echelon_form_assign_helper(a: &mut NaturalMatrix, m: &Natural) {
    assert_reduced(a, m);
    if let Natural(Small(small_m)) = *m {
        *a = NaturalMatrix::from(to_limb_matrix(a).mod_reduced_row_echelon_form(small_m));
    } else {
        rref_in_place(a, m);
    }
}

fn mod_reduced_row_echelon_form_ref_helper(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    assert_reduced(a, m);
    if let Natural(Small(small_m)) = *m {
        NaturalMatrix::from(to_limb_matrix(a).mod_reduced_row_echelon_form(small_m))
    } else {
        let mut a = a.clone();
        rref_in_place(&mut a, m);
        a
    }
}

impl ModReducedRowEchelonForm<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the reduced row echelon form of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The reduced row echelon form
    /// of a matrix $A$ is the unique matrix $R$ of the same shape whose rows span the same space as
    /// the rows of $A$ over the field $\mathbb{Z}/m\mathbb{Z}$, and in which:
    /// - the nonzero rows come before the zero rows;
    /// - the first nonzero element of each nonzero row, its pivot, is 1, and lies to the right of
    ///   the pivot of the row above;
    /// - every other element of a column containing a pivot is 0.
    ///
    /// The number of nonzero rows is the rank of $A$, which
    /// [`ModRank`](malachite_base::num::arithmetic::traits::ModRank) computes. When $m$ fits in a
    /// limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_reduced_row_echelon_form(Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 0, 1; 0, 1, 1; 0, 0, 0]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_reduced_row_echelon_form(m).to_string(),
    ///     "[1, 2; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rref` from `fmpz_mod_mat/rref.c`, FLINT 3.6.0, except
    /// that the rank is not returned, and that it panics if an element is not reduced or a pivot is
    /// not invertible, where FLINT aborts.
    #[inline]
    fn mod_reduced_row_echelon_form(mut self, m: Natural) -> Self {
        mod_reduced_row_echelon_form_assign_helper(&mut self, &m);
        self
    }
}

impl ModReducedRowEchelonForm<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the reduced row echelon form of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The reduced row echelon form
    /// of a matrix $A$ is the unique matrix $R$ of the same shape whose rows span the same space as
    /// the rows of $A$ over the field $\mathbb{Z}/m\mathbb{Z}$, and in which:
    /// - the nonzero rows come before the zero rows;
    /// - the first nonzero element of each nonzero row, its pivot, is 1, and lies to the right of
    ///   the pivot of the row above;
    /// - every other element of a column containing a pivot is 0.
    ///
    /// The number of nonzero rows is the rank of $A$, which
    /// [`ModRank`](malachite_base::num::arithmetic::traits::ModRank) computes. When $m$ fits in a
    /// limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_reduced_row_echelon_form(&Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 0, 1; 0, 1, 1; 0, 0, 0]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_reduced_row_echelon_form(&m).to_string(),
    ///     "[1, 2; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rref` from `fmpz_mod_mat/rref.c`, FLINT 3.6.0, except
    /// that the rank is not returned, and that it panics if an element is not reduced or a pivot is
    /// not invertible, where FLINT aborts.
    #[inline]
    fn mod_reduced_row_echelon_form(mut self, m: &Natural) -> Self {
        mod_reduced_row_echelon_form_assign_helper(&mut self, m);
        self
    }
}

impl ModReducedRowEchelonForm<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the reduced row echelon form of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The reduced row echelon form
    /// of a matrix $A$ is the unique matrix $R$ of the same shape whose rows span the same space as
    /// the rows of $A$ over the field $\mathbb{Z}/m\mathbb{Z}$, and in which:
    /// - the nonzero rows come before the zero rows;
    /// - the first nonzero element of each nonzero row, its pivot, is 1, and lies to the right of
    ///   the pivot of the row above;
    /// - every other element of a column containing a pivot is 0.
    ///
    /// The number of nonzero rows is the rank of $A$, which
    /// [`ModRank`](malachite_base::num::arithmetic::traits::ModRank) computes. When $m$ fits in a
    /// limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 0, 1; 0, 1, 1; 0, 0, 0]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(m).to_string(),
    ///     "[1, 2; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rref` from `fmpz_mod_mat/rref.c`, FLINT 3.6.0, except
    /// that the rank is not returned, and that it panics if an element is not reduced or a pivot is
    /// not invertible, where FLINT aborts.
    #[inline]
    fn mod_reduced_row_echelon_form(self, m: Natural) -> NaturalMatrix {
        mod_reduced_row_echelon_form_ref_helper(self, &m)
    }
}

impl ModReducedRowEchelonForm<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the reduced row echelon form of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The reduced row echelon form
    /// of a matrix $A$ is the unique matrix $R$ of the same shape whose rows span the same space as
    /// the rows of $A$ over the field $\mathbb{Z}/m\mathbb{Z}$, and in which:
    /// - the nonzero rows come before the zero rows;
    /// - the first nonzero element of each nonzero row, its pivot, is 1, and lies to the right of
    ///   the pivot of the row above;
    /// - every other element of a column containing a pivot is 0.
    ///
    /// The number of nonzero rows is the rank of $A$, which
    /// [`ModRank`](malachite_base::num::arithmetic::traits::ModRank) computes. When $m$ fits in a
    /// limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(&Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 0, 1; 0, 1, 1; 0, 0, 0]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(&m).to_string(),
    ///     "[1, 2; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rref` from `fmpz_mod_mat/rref.c`, FLINT 3.6.0, except
    /// that the rank is not returned, and that it panics if an element is not reduced or a pivot is
    /// not invertible, where FLINT aborts.
    #[inline]
    fn mod_reduced_row_echelon_form(self, m: &Natural) -> NaturalMatrix {
        mod_reduced_row_echelon_form_ref_helper(self, m)
    }
}

impl ModReducedRowEchelonFormAssign<Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its reduced row echelon form modulo a prime $m$, taking
    /// the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The reduced row echelon form
    /// of a matrix $A$ is the unique matrix $R$ of the same shape whose rows span the same space as
    /// the rows of $A$ over the field $\mathbb{Z}/m\mathbb{Z}$, and in which:
    /// - the nonzero rows come before the zero rows;
    /// - the first nonzero element of each nonzero row, its pivot, is 1, and lies to the right of
    ///   the pivot of the row above;
    /// - every other element of a column containing a pivot is 0.
    ///
    /// The number of nonzero rows is the rank of $A$, which
    /// [`ModRank`](malachite_base::num::arithmetic::traits::ModRank) computes. When $m$ fits in a
    /// limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// a.mod_reduced_row_echelon_form_assign(Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[1, 0, 1; 0, 1, 1; 0, 0, 0]");
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let mut a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// a.mod_reduced_row_echelon_form_assign(m);
    /// assert_eq!(a.to_string(), "[1, 2; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rref` from `fmpz_mod_mat/rref.c`, FLINT 3.6.0, except
    /// that the rank is not returned, and that it panics if an element is not reduced or a pivot is
    /// not invertible, where FLINT aborts.
    #[inline]
    fn mod_reduced_row_echelon_form_assign(&mut self, m: Natural) {
        mod_reduced_row_echelon_form_assign_helper(self, &m);
    }
}

impl ModReducedRowEchelonFormAssign<&Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its reduced row echelon form modulo a prime $m$, taking
    /// the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The reduced row echelon form
    /// of a matrix $A$ is the unique matrix $R$ of the same shape whose rows span the same space as
    /// the rows of $A$ over the field $\mathbb{Z}/m\mathbb{Z}$, and in which:
    /// - the nonzero rows come before the zero rows;
    /// - the first nonzero element of each nonzero row, its pivot, is 1, and lies to the right of
    ///   the pivot of the row above;
    /// - every other element of a column containing a pivot is 0.
    ///
    /// The number of nonzero rows is the rank of $A$, which
    /// [`ModRank`](malachite_base::num::arithmetic::traits::ModRank) computes. When $m$ fits in a
    /// limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// a.mod_reduced_row_echelon_form_assign(&Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[1, 0, 1; 0, 1, 1; 0, 0, 0]");
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let mut a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// a.mod_reduced_row_echelon_form_assign(&m);
    /// assert_eq!(a.to_string(), "[1, 2; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rref` from `fmpz_mod_mat/rref.c`, FLINT 3.6.0, except
    /// that the rank is not returned, and that it panics if an element is not reduced or a pivot is
    /// not invertible, where FLINT aborts.
    #[inline]
    fn mod_reduced_row_echelon_form_assign(&mut self, m: &Natural) {
        mod_reduced_row_echelon_form_assign_helper(self, m);
    }
}
