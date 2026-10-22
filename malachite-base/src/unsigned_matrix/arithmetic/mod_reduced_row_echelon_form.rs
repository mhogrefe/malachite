// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModReducedRowEchelonForm, ModReducedRowEchelonFormAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::{
    LuBlock, assert_reduced, lu, pivot_inverse,
};
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::BlockRef;
use crate::unsigned_matrix::arithmetic::mod_solve_upper_triangular::solve_upper;
use alloc::vec;
use alloc::vec::Vec;

// Computes the LU decomposition of `a` in place, and returns the rank and the pivot columns of U in
// increasing order, followed by its other columns in increasing order. When the rank is 0 or the
// width of `a`, the columns are returned in their original order.
//
// This is equivalent to `nmod_mat_lu_with_pivots` from `nmod_mat/lu_with_pivots.c`, FLINT 3.6.0.
pub(crate) fn lu_with_pivots<T: PrimitiveUnsigned>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
) -> (usize, Vec<usize>) {
    let n = a.width;
    let rank = lu(perm, a, m, false);
    if rank == 0 || rank == n {
        return (rank, (0..n).collect());
    }
    let mut pivots_nonpivots = vec![0; n];
    let (pivots, nonpivots) = pivots_nonpivots.split_at_mut(rank);
    let mut j = 0;
    let mut k = 0;
    for (i, pivot) in pivots.iter_mut().enumerate() {
        while a.xs[i * a.stride + j] == T::ZERO {
            nonpivots[k] = j;
            k += 1;
            j += 1;
        }
        *pivot = j;
        j += 1;
    }
    while k < n - rank {
        nonpivots[k] = j;
        k += 1;
        j += 1;
    }
    (rank, pivots_nonpivots)
}

// Replaces the `height` by `width` matrix with elements `xs` with its reduced row echelon form
// modulo the prime `m`, and returns its rank.
//
// This is equivalent to `nmod_mat_rref` and `_nmod_mat_rref` from `nmod_mat/rref.c`, FLINT 3.6.0.
pub(crate) fn rref_in_place<T: PrimitiveUnsigned>(
    xs: &mut [T],
    height: usize,
    width: usize,
    m: T,
) -> usize {
    if height == 0 || width == 0 {
        return 0;
    }
    if height == 1 {
        let mut r = 0;
        for i in 0..width {
            let c = xs[i];
            if c != T::ZERO {
                r = 1;
                if c == T::ONE {
                    break;
                }
                let inverse = pivot_inverse(c, m);
                xs[i] = T::ONE;
                for x in &mut xs[i + 1..] {
                    x.mod_mul_assign(inverse, m);
                }
                break;
            }
        }
        return r;
    }
    let mut perm = vec![0; height];
    let (rank, pivots_nonpivots) = lu_with_pivots(
        &mut perm,
        &mut LuBlock {
            xs: &mut *xs,
            stride: width,
            height,
            width,
        },
        m,
    );
    if rank == 0 {
        return rank;
    }
    let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
    // Clear L.
    for i in 0..height {
        for x in &mut xs[i * width..i * width + i.min(rank)] {
            *x = T::ZERO;
        }
    }
    if rank != width {
        // Reorder U into upper triangular form U | V, with U of full rank and triangular, set V to
        // U^-1 V, and put the columns back in their original order.
        let nullity = width - rank;
        let mut u = vec![T::ZERO; rank * rank];
        for j in 0..rank {
            for i in j..rank {
                u[j * rank + i] = xs[j * width + pivots[i]];
            }
        }
        let mut v = vec![T::ZERO; rank * nullity];
        for j in 0..rank {
            for (i, &c) in nonpivots.iter().enumerate() {
                v[j * nullity + i] = xs[j * width + c];
            }
        }
        solve_upper(
            &mut v,
            BlockRef {
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
        for j in 0..rank {
            for (i, &c) in nonpivots.iter().enumerate() {
                xs[j * width + c] = v[j * nullity + i];
            }
        }
    }
    // Clear the pivot columns.
    for j in 0..rank {
        for i in j..rank {
            xs[j * width + pivots[i]] = if i == j { T::ONE } else { T::ZERO };
        }
    }
    rank
}

impl<T: PrimitiveUnsigned> ModReducedRowEchelonForm<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the reduced row echelon form of an [`UnsignedMatrix`] modulo a prime $m$, taking it
    /// by value.
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
    /// [`ModRank`](crate::num::arithmetic::traits::ModRank) computes.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_reduced_row_echelon_form(7).to_string(),
    ///     "[1, 2; 0, 0]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_reduced_row_echelon_form(7).to_string(),
    ///     "[1, 0, 1; 0, 1, 1; 0, 0, 0]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 3, 6]").unwrap();
    /// assert_eq!(a.mod_reduced_row_echelon_form(7).to_string(), "[0, 1, 2]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_rref` from `nmod_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned, and that it panics if an element is not reduced.
    #[inline]
    fn mod_reduced_row_echelon_form(mut self, m: T) -> Self {
        self.mod_reduced_row_echelon_form_assign(m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModReducedRowEchelonForm<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the reduced row echelon form of an [`UnsignedMatrix`] modulo a prime $m$, taking it
    /// by reference.
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
    /// [`ModRank`](crate::num::arithmetic::traits::ModRank) computes.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(7).to_string(),
    ///     "[1, 2; 0, 0]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(7).to_string(),
    ///     "[1, 0, 1; 0, 1, 1; 0, 0, 0]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 3, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_reduced_row_echelon_form(7).to_string(),
    ///     "[0, 1, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_rref` from `nmod_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned, and that it panics if an element is not reduced.
    #[inline]
    fn mod_reduced_row_echelon_form(self, m: T) -> UnsignedMatrix<T> {
        self.clone().mod_reduced_row_echelon_form(m)
    }
}

impl<T: PrimitiveUnsigned> ModReducedRowEchelonFormAssign<T> for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its reduced row echelon form modulo a prime $m$.
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
    /// [`ModRank`](crate::num::arithmetic::traits::ModRank) computes.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModReducedRowEchelonFormAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// a.mod_reduced_row_echelon_form_assign(7);
    /// assert_eq!(a.to_string(), "[1, 2; 0, 0]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// a.mod_reduced_row_echelon_form_assign(7);
    /// assert_eq!(a.to_string(), "[1, 0, 1; 0, 1, 1; 0, 0, 0]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_rref` from `nmod_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned, and that it panics if an element is not reduced.
    fn mod_reduced_row_echelon_form_assign(&mut self, m: T) {
        assert_reduced(self, m);
        rref_in_place(&mut self.elements, self.height, self.width, m);
    }
}
