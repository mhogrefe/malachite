// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
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
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use malachite_base::num::arithmetic::traits::ModRank;
use malachite_base::num::conversion::traits::ExactFrom;

// This is equivalent to `fmpz_mod_mat_rank` from `fmpz_mod_mat/rank.c`, FLINT 3.6.0, which calls
// `gr_mat_rank_lu` from `gr_mat/rank.c`.
fn mod_rank_helper(mut a: NaturalMatrix, m: &Natural) -> u64 {
    assert_reduced(&a, m);
    if let Natural(Small(small_m)) = *m {
        return to_limb_matrix(&a).mod_rank(small_m);
    }
    if a.height == 0 || a.width == 0 {
        return 0;
    }
    u64::exact_from(
        lu_in_place(&mut a, m, false)
            .unwrap_or_else(|| non_invertible_pivot(m))
            .1,
    )
}

impl ModRank<Natural> for NaturalMatrix {
    /// Computes the rank of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by value and
    /// the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The rank is the dimension of
    /// the space spanned by the rows (or, equivalently, the columns) of the matrix over the field
    /// $\mathbb{Z}/m\mathbb{Z}$, and is the number of rows of $U$ in the matrix's LU decomposition.
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is the number of pivots.
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
    /// use malachite_base::num::arithmetic::traits::ModRank;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(a.mod_rank(Natural::from(5u32)), 2);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(a.mod_rank(Natural::from(7u32)), 2);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rank` from `fmpz_mod_mat/rank.c`, FLINT 3.6.0, except
    /// that it panics if an element is not reduced or a pivot is not invertible, where FLINT
    /// aborts.
    #[inline]
    fn mod_rank(self, m: Natural) -> u64 {
        mod_rank_helper(self, &m)
    }
}

impl ModRank<&Natural> for NaturalMatrix {
    /// Computes the rank of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by value and
    /// the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The rank is the dimension of
    /// the space spanned by the rows (or, equivalently, the columns) of the matrix over the field
    /// $\mathbb{Z}/m\mathbb{Z}$, and is the number of rows of $U$ in the matrix's LU decomposition.
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is the number of pivots.
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
    /// use malachite_base::num::arithmetic::traits::ModRank;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(a.mod_rank(&Natural::from(5u32)), 2);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(a.mod_rank(&Natural::from(7u32)), 2);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rank` from `fmpz_mod_mat/rank.c`, FLINT 3.6.0, except
    /// that it panics if an element is not reduced or a pivot is not invertible, where FLINT
    /// aborts.
    #[inline]
    fn mod_rank(self, m: &Natural) -> u64 {
        mod_rank_helper(self, m)
    }
}

impl ModRank<Natural> for &NaturalMatrix {
    /// Computes the rank of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by reference
    /// and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The rank is the dimension of
    /// the space spanned by the rows (or, equivalently, the columns) of the matrix over the field
    /// $\mathbb{Z}/m\mathbb{Z}$, and is the number of rows of $U$ in the matrix's LU decomposition.
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is the number of pivots.
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
    /// use malachite_base::num::arithmetic::traits::ModRank;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!((&a).mod_rank(Natural::from(5u32)), 2);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_rank(Natural::from(7u32)), 2);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rank` from `fmpz_mod_mat/rank.c`, FLINT 3.6.0, except
    /// that it panics if an element is not reduced or a pivot is not invertible, where FLINT
    /// aborts.
    #[inline]
    fn mod_rank(self, m: Natural) -> u64 {
        mod_rank_helper(self.clone(), &m)
    }
}

impl ModRank<&Natural> for &NaturalMatrix {
    /// Computes the rank of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by reference
    /// and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The rank is the dimension of
    /// the space spanned by the rows (or, equivalently, the columns) of the matrix over the field
    /// $\mathbb{Z}/m\mathbb{Z}$, and is the number of rows of $U$ in the matrix's LU decomposition.
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is the number of pivots.
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
    /// use malachite_base::num::arithmetic::traits::ModRank;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!((&a).mod_rank(&Natural::from(5u32)), 2);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_rank(&Natural::from(7u32)), 2);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_rank` from `fmpz_mod_mat/rank.c`, FLINT 3.6.0, except
    /// that it panics if an element is not reduced or a pivot is not invertible, where FLINT
    /// aborts.
    #[inline]
    fn mod_rank(self, m: &Natural) -> u64 {
        mod_rank_helper(self.clone(), m)
    }
}
