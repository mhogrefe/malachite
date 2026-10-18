// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModMul};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, dot_sum};
use alloc::vec;

// Checks the arguments of a matrix-vector product modulo `m`: `m` is positive, the vector's
// dimension is the width of the matrix, and the elements of both are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, v: &UnsignedVector<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(
        v.elements.len(),
        a.width,
        "the vector's dimension must equal the width of the matrix"
    );
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        v.mod_is_reduced(&m),
        "the vector must be reduced mod m, but {v} has an element >= {m}"
    );
}

// The product `a * v` modulo `m`, after checking the arguments. As in FLINT's
// `nmod_mat_mul_nmod_vec`, each element is a dot product whose products are summed exactly, in one,
// two, or three words, and reduced once; the data for this is computed once for all the rows.
fn mod_mul_vector<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    v: &UnsignedVector<T>,
    m: T,
) -> UnsignedVector<T> {
    assert_arguments(a, v, m);
    let width = a.width;
    if width == 0 {
        return UnsignedVector {
            elements: vec![T::ZERO; a.height],
        };
    }
    let d = ModData::new(m, width);
    UnsignedVector {
        elements: a
            .elements
            .chunks_exact(width)
            .map(|row| d.reduce_sum(dot_sum(row, &v.elements, &d)))
            .collect(),
    }
}

impl<T: PrimitiveUnsigned> ModMul<UnsignedVector<T>, T> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo a number $m$,
    /// taking the matrix by value and the vector by value.
    ///
    /// The elements of both must already be reduced modulo $m$. Element $i$ of the result is the
    /// dot product of row $i$ of the matrix with the vector, modulo $m$:
    ///
    /// $$
    /// f(A, v, m)_i = \sum_{j=0}^{n-1} A_{ij}v_j \bmod m,
    /// $$
    ///
    /// where $n$ is the width of $A$, which must be the dimension of $v$.
    ///
    /// The products are summed exactly, in one, two, or three words depending on how large the sums
    /// can get, and each sum is reduced once.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `other` is not the width of `self`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a).mod_mul(v, 7).to_string(), "(0, 2)");
    /// ```
    #[inline]
    fn mod_mul(self, other: UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        mod_mul_vector(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&UnsignedVector<T>, T> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo a number $m$,
    /// taking the matrix by value and the vector by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `other` is not the width of `self`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a).mod_mul(&v, 7).to_string(), "(0, 2)");
    /// ```
    #[inline]
    fn mod_mul(self, other: &UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        mod_mul_vector(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<UnsignedVector<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo a number $m$,
    /// taking the matrix by reference and the vector by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `other` is not the width of `self`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a).mod_mul(v, 7).to_string(), "(0, 2)");
    /// ```
    #[inline]
    fn mod_mul(self, other: UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        mod_mul_vector(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&UnsignedVector<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo a number $m$,
    /// taking the matrix by reference and the vector by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `other` is not the width of `self`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a).mod_mul(&v, 7).to_string(), "(0, 2)");
    /// ```
    #[inline]
    fn mod_mul(self, other: &UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        mod_mul_vector(self, other, m)
    }
}
