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

use crate::num::arithmetic::traits::ModNullspace;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::unsigned_matrix::arithmetic::mod_reduced_row_echelon_form::rref_in_place;
use alloc::vec;
use alloc::vec::Vec;

// Returns the elements of a `width` by `nullity` matrix whose columns form a basis of the right
// nullspace of the `height` by `width` matrix with elements `xs` modulo the prime `m`, and the
// nullity. `xs` is replaced by its reduced row echelon form.
//
// This is equivalent to `nmod_mat_nullspace` from `nmod_mat/nullspace.c`, FLINT 3.6.0, returning
// only the first `nullity` columns of FLINT's `width` by `width` result.
pub(crate) fn nullspace_in_place<T: PrimitiveUnsigned>(
    xs: &mut [T],
    height: usize,
    width: usize,
    m: T,
) -> (Vec<T>, usize) {
    let rank = rref_in_place(xs, height, width, m);
    let nullity = width - rank;
    let mut x = vec![T::ZERO; width * nullity];
    if rank == 0 {
        // Modulo 1, every element is 0, so the basis vectors are 0 rather than columns of the
        // identity.
        if m != T::ONE {
            for i in 0..nullity {
                x[i * nullity + i] = T::ONE;
            }
        }
    } else if nullity != 0 {
        let mut pivots = Vec::with_capacity(rank);
        let mut nonpivots = Vec::with_capacity(nullity);
        let mut j = 0;
        for i in 0..rank {
            while xs[i * width + j] == T::ZERO {
                nonpivots.push(j);
                j += 1;
            }
            pivots.push(j);
            j += 1;
        }
        nonpivots.extend(j..width);
        for (i, &c) in nonpivots.iter().enumerate() {
            for (j, &p) in pivots.iter().enumerate() {
                x[p * nullity + i] = xs[j * width + c].mod_neg(m);
            }
            x[c * nullity + i] = T::ONE;
        }
    }
    (x, nullity)
}

fn mod_nullspace_helper<T: PrimitiveUnsigned>(mut a: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
    assert_reduced(&a, m);
    let width = a.width;
    let (elements, nullity) = nullspace_in_place(&mut a.elements, a.height, width, m);
    UnsignedMatrix {
        elements,
        height: width,
        width: nullity,
    }
}

impl<T: PrimitiveUnsigned> ModNullspace<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes a basis of the right nullspace of an [`UnsignedMatrix`] modulo a prime $m$, taking
    /// the matrix by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The right nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $w$ with $Ax = 0$
    /// modulo $m$; it has dimension $w - r$. The result is the $w \times (w - r)$ matrix whose
    /// columns are the basis vectors obtained from the reduced row echelon form $R$ of $A$: for
    /// each column $c$ of $R$ that does not contain a pivot, the vector with 1 in position $c$, the
    /// negation of $R_{i,c}$ in the position of the pivot of row $i$ of $R$, and 0 elsewhere.
    /// Modulo 1, where every element is 0, the basis vectors are 0.
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
    /// use malachite_base::num::arithmetic::traits::ModNullspace;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(a.mod_nullspace(7).to_string(), "[5; 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(a.mod_nullspace(7).to_string(), "[6; 6; 1]");
    ///
    /// // A matrix of full column rank has a trivial nullspace.
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(a.mod_nullspace(5).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_nullspace` from `nmod_mat/nullspace.c`, FLINT 3.6.0, except
    /// that only the $w - r$ basis vectors are returned rather than a $w \times w$ matrix and the
    /// nullity, that modulo 1 the basis vectors are 0 rather than having an element equal to 1, and
    /// that it panics if an element is not reduced.
    #[inline]
    fn mod_nullspace(self, m: T) -> Self {
        mod_nullspace_helper(self, m)
    }
}

impl<T: PrimitiveUnsigned> ModNullspace<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes a basis of the right nullspace of an [`UnsignedMatrix`] modulo a prime $m$, taking
    /// the matrix by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The right nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $w$ with $Ax = 0$
    /// modulo $m$; it has dimension $w - r$. The result is the $w \times (w - r)$ matrix whose
    /// columns are the basis vectors obtained from the reduced row echelon form $R$ of $A$: for
    /// each column $c$ of $R$ that does not contain a pivot, the vector with 1 in position $c$, the
    /// negation of $R_{i,c}$ in the position of the pivot of row $i$ of $R$, and 0 elsewhere.
    /// Modulo 1, where every element is 0, the basis vectors are 0.
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
    /// use malachite_base::num::arithmetic::traits::ModNullspace;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!((&a).mod_nullspace(7).to_string(), "[5; 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_nullspace(7).to_string(), "[6; 6; 1]");
    ///
    /// // A matrix of full column rank has a trivial nullspace.
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!((&a).mod_nullspace(5).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_nullspace` from `nmod_mat/nullspace.c`, FLINT 3.6.0, except
    /// that only the $w - r$ basis vectors are returned rather than a $w \times w$ matrix and the
    /// nullity, that modulo 1 the basis vectors are 0 rather than having an element equal to 1, and
    /// that it panics if an element is not reduced.
    #[inline]
    fn mod_nullspace(self, m: T) -> UnsignedMatrix<T> {
        mod_nullspace_helper(self.clone(), m)
    }
}
