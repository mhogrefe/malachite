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

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::ModLeftNullspace;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::unsigned_matrix::arithmetic::mod_nullspace::nullspace_in_place;

// The left nullspace of A is the right nullspace of the transpose of A, with the basis vectors
// written as rows.
//
// This is equivalent to `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`, FLINT 3.6.0,
// returning only the first `nullity` rows of FLINT's `height` by `height` result.
fn mod_left_nullspace_helper<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    assert_reduced(a, m);
    let height = a.height;
    let mut at = a.transpose();
    let (elements, nullity) = nullspace_in_place(&mut at.elements, a.width, height, m);
    UnsignedMatrix {
        elements,
        height,
        width: nullity,
    }
    .transpose()
}

impl<T: PrimitiveUnsigned> ModLeftNullspace<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes a basis of the left nullspace of an [`UnsignedMatrix`] modulo a prime $m$, taking
    /// the matrix by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The left nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $h$ with $xA = 0$
    /// modulo $m$; it has dimension $h - r$. The result is the $(h - r) \times h$ matrix whose rows
    /// are the basis vectors of the right nullspace of the transpose of $A$, as computed by
    /// [`ModNullspace`](crate::num::arithmetic::traits::ModNullspace). Modulo 1, where every
    /// element is 0, the basis vectors are 0.
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
    /// use malachite_base::num::arithmetic::traits::ModLeftNullspace;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(a.mod_left_nullspace(7).to_string(), "[5, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1; 2; 3]").unwrap();
    /// assert_eq!(a.mod_left_nullspace(5).to_string(), "[3, 1, 0; 2, 0, 1]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`, FLINT
    /// 3.6.0, except that only the $h - r$ basis vectors are returned rather than an $h \times h$
    /// matrix and the nullity, that modulo 1 the basis vectors are 0 rather than having an element
    /// equal to 1, and that it panics if an element is not reduced.
    #[inline]
    fn mod_left_nullspace(self, m: T) -> Self {
        mod_left_nullspace_helper(&self, m)
    }
}

impl<T: PrimitiveUnsigned> ModLeftNullspace<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes a basis of the left nullspace of an [`UnsignedMatrix`] modulo a prime $m$, taking
    /// the matrix by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The left nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $h$ with $xA = 0$
    /// modulo $m$; it has dimension $h - r$. The result is the $(h - r) \times h$ matrix whose rows
    /// are the basis vectors of the right nullspace of the transpose of $A$, as computed by
    /// [`ModNullspace`](crate::num::arithmetic::traits::ModNullspace). Modulo 1, where every
    /// element is 0, the basis vectors are 0.
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
    /// use malachite_base::num::arithmetic::traits::ModLeftNullspace;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!((&a).mod_left_nullspace(7).to_string(), "[5, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1; 2; 3]").unwrap();
    /// assert_eq!((&a).mod_left_nullspace(5).to_string(), "[3, 1, 0; 2, 0, 1]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`, FLINT
    /// 3.6.0, except that only the $h - r$ basis vectors are returned rather than an $h \times h$
    /// matrix and the nullity, that modulo 1 the basis vectors are 0 rather than having an element
    /// equal to 1, and that it panics if an element is not reduced.
    #[inline]
    fn mod_left_nullspace(self, m: T) -> UnsignedMatrix<T> {
        mod_left_nullspace_helper(self, m)
    }
}
