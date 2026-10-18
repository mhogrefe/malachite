// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;

// Checks the arguments of a vector-matrix product modulo $2^k$: `pow` is at most the width of `T`,
// the vector's dimension is the height of the matrix, and the elements of both are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
    );
    assert_eq!(
        v.elements.len(),
        a.height,
        "the vector's dimension must equal the height of the matrix"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "the vector must be reduced mod 2^{pow}, but {v} has an element >= 2^{pow}"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// The product `v * a` modulo $2^k$, after checking the arguments. Each row of the matrix, scaled by
// the matching element of the vector, is accumulated in wrapping arithmetic, so the matrix is read
// in storage order; all but the lowest $k$ bits of each sum are cleared at the end.
fn vector_mod_power_of_2_mul<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedVector<T> {
    assert_arguments(v, a, pow);
    let width = a.width;
    let mut out = vec![T::ZERO; width];
    if width != 0 {
        for (&x, row) in v.elements.iter().zip(a.elements.chunks_exact(width)) {
            if x != T::ZERO {
                for (o, &y) in out.iter_mut().zip(row) {
                    o.wrapping_add_assign(x.wrapping_mul(y));
                }
            }
        }
    }
    for o in &mut out {
        o.mod_power_of_2_assign(pow);
    }
    UnsignedVector { elements: out }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<UnsignedMatrix<T>> for UnsignedVector<T> {
    type Output = Self;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo $2^k$, taking
    /// the vector by value and the matrix by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. Element $j$ of the result is the
    /// dot product of the vector with column $j$ of the matrix, modulo $2^k$:
    ///
    /// $$
    /// f(v, A, k)_j = \sum_{i=0}^{n-1} v_iA_{ij} \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the height of $A$, which must be the dimension of $v$.
    ///
    /// The products and sums are done in wrapping arithmetic, and all but the lowest $k$ bits of
    /// each result are cleared at the end, which is correct since $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `self` is not the height of
    /// `other`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_power_of_2_mul(a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: UnsignedMatrix<T>, pow: u64) -> Self {
        vector_mod_power_of_2_mul(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&UnsignedMatrix<T>> for UnsignedVector<T> {
    type Output = Self;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo $2^k$, taking
    /// the vector by value and the matrix by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `self` is not the height of
    /// `other`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_power_of_2_mul(&a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &UnsignedMatrix<T>, pow: u64) -> Self {
        vector_mod_power_of_2_mul(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<UnsignedMatrix<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo $2^k$, taking
    /// the vector by reference and the matrix by value.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `self` is not the height of
    /// `other`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v).mod_power_of_2_mul(a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: UnsignedMatrix<T>, pow: u64) -> UnsignedVector<T> {
        vector_mod_power_of_2_mul(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&UnsignedMatrix<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo $2^k$, taking
    /// the vector by reference and the matrix by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `self` is not the height of
    /// `other`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v).mod_power_of_2_mul(&a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &UnsignedMatrix<T>, pow: u64) -> UnsignedVector<T> {
        vector_mod_power_of_2_mul(self, other, pow)
    }
}
