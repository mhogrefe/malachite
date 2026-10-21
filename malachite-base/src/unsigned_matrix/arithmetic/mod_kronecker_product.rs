// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModIsReduced, ModKroneckerProduct, ModKroneckerProductAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;

// Checks the arguments of a Kronecker product modulo `m`: `m` is positive, and the elements of both
// matrices are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert!(
        a.mod_is_reduced(&m),
        "the first matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(&m),
        "the second matrix must be reduced mod m, but {b} has an element >= {m}"
    );
}

// The Kronecker product of `a` and `b` modulo `m`, after checking the arguments, written one row at
// a time, in storage order. The data for multiplying modulo `m` is precomputed once and shared by
// all the elements.
fn mod_kronecker_product<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    assert_arguments(a, b, m);
    let data = T::precompute_mod_mul_data(&m);
    let mut elements = Vec::with_capacity(a.elements.len() * b.elements.len());
    if a.width != 0 && b.width != 0 {
        for a_row in a.elements.chunks_exact(a.width) {
            for b_row in b.elements.chunks_exact(b.width) {
                for &x in a_row {
                    for &y in b_row {
                        elements.push(x.mod_mul_precomputed(y, m, &data));
                    }
                }
            }
        }
    }
    UnsignedMatrix {
        elements,
        height: a.height * b.height,
        width: a.width * b.width,
    }
}

impl<T: PrimitiveUnsigned> ModKroneckerProduct<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $m$, taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The Kronecker product $A
    /// \otimes B$ of an $h_A \times w_A$ matrix $A$ and a $p \times q$ matrix $B$ is the $h_Ap
    /// \times w_Aq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that
    /// product with every element reduced modulo $m$:
    ///
    /// $$
    /// (A \otimes B \bmod m)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod m.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $h_Ap
    /// \times w_Aq$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, 9u8).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[254, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, 255u8).to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(a.mod_kronecker_product(b, 1u8).to_string(), "[0, 0; 0, 0]");
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: Self, m: T) -> Self {
        mod_kronecker_product(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModKroneckerProduct<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $m$, taking the first by
    /// value and the second by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, 9u8).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[254, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, 255u8).to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(a.mod_kronecker_product(&b, 1u8).to_string(), "[0, 0; 0, 0]");
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: &Self, m: T) -> Self {
        mod_kronecker_product(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModKroneckerProduct<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $m$, taking the first by
    /// reference and the second by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, 9u8).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[254, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, 255u8).to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, 1u8).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_kronecker_product(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModKroneckerProduct<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $m$, taking both by
    /// reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, 9u8).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[254, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, 255u8).to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, 1u8).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_kronecker_product(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModKroneckerProductAssign<Self, T> for UnsignedMatrix<T> {
    /// Replaces a [`UnsignedMatrix`] with its Kronecker product with another [`UnsignedMatrix`]
    /// modulo $m$, taking the second by value.
    ///
    /// See the documentation for the [`ModKroneckerProduct`] implementation that takes all
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProductAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_kronecker_product_assign(b, 9u8);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[254]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3, 128]").unwrap();
    /// a.mod_kronecker_product_assign(b, 255u8);
    /// assert_eq!(a.to_string(), "[252, 127]");
    /// ```
    #[inline]
    fn mod_kronecker_product_assign(&mut self, other: Self, m: T) {
        *self = mod_kronecker_product(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModKroneckerProductAssign<&Self, T> for UnsignedMatrix<T> {
    /// Replaces a [`UnsignedMatrix`] with its Kronecker product with another [`UnsignedMatrix`]
    /// modulo $m$, taking the second by reference.
    ///
    /// See the documentation for the [`ModKroneckerProduct`] implementation that takes all
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProductAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_kronecker_product_assign(&b, 9u8);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[254]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3, 128]").unwrap();
    /// a.mod_kronecker_product_assign(&b, 255u8);
    /// assert_eq!(a.to_string(), "[252, 127]");
    /// ```
    #[inline]
    fn mod_kronecker_product_assign(&mut self, other: &Self, m: T) {
        *self = mod_kronecker_product(self, other, m);
    }
}
