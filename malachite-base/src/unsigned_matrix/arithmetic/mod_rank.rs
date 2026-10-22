// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModRank;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::{assert_reduced, lu_in_place};

// This is equivalent to `nmod_mat_rank` from `nmod_mat/rank.c`, FLINT 3.6.0.
fn mod_rank_helper<T: PrimitiveUnsigned>(mut a: UnsignedMatrix<T>, m: T) -> u64 {
    assert_reduced(&a, m);
    if a.height == 0 || a.width == 0 {
        return 0;
    }
    u64::exact_from(lu_in_place(&mut a, m, false).1)
}

impl<T: PrimitiveUnsigned> ModRank<T> for UnsignedMatrix<T> {
    /// Computes the rank of an [`UnsignedMatrix`] modulo a prime $m$, taking the matrix by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The rank is the dimension of
    /// the space spanned by the rows (or, equivalently, the columns) of the matrix over the field
    /// $\mathbb{Z}/m\mathbb{Z}$, and is the number of rows of $U$ in the matrix's LU decomposition.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is the number of pivots.
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
    /// use malachite_base::num::arithmetic::traits::ModRank;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// assert_eq!(a.mod_rank(5), 0);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(a.mod_rank(5), 2);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(a.mod_rank(7), 2);
    /// ```
    ///
    /// This is equivalent to `nmod_mat_rank` from `nmod_mat/rank.c`, FLINT 3.6.0, except that it
    /// panics if an element is not reduced.
    #[inline]
    fn mod_rank(self, m: T) -> u64 {
        mod_rank_helper(self, m)
    }
}

impl<T: PrimitiveUnsigned> ModRank<T> for &UnsignedMatrix<T> {
    /// Computes the rank of an [`UnsignedMatrix`] modulo a prime $m$, taking the matrix by
    /// reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The rank is the dimension of
    /// the space spanned by the rows (or, equivalently, the columns) of the matrix over the field
    /// $\mathbb{Z}/m\mathbb{Z}$, and is the number of rows of $U$ in the matrix's LU decomposition.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is the number of pivots.
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
    /// use malachite_base::num::arithmetic::traits::ModRank;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// assert_eq!((&a).mod_rank(5), 0);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!((&a).mod_rank(5), 2);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_rank(7), 2);
    /// ```
    ///
    /// This is equivalent to `nmod_mat_rank` from `nmod_mat/rank.c`, FLINT 3.6.0, except that it
    /// panics if an element is not reduced.
    #[inline]
    fn mod_rank(self, m: T) -> u64 {
        mod_rank_helper(self.clone(), m)
    }
}
