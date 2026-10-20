// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2020 William Hart
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModMul, ModMulAssign};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::classical::matrix_mod_mul_classical_op;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::matrix_mod_mul_strassen;
use alloc::vec;
use alloc::vec::Vec;

/// The classical algorithm.
pub mod classical;
/// Strassen's algorithm.
pub mod strassen;

// The shape of a matrix product: the first factor is `ar` by `br`, and the second is `br` by `bc`.
crate_test_struct! {
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
ProductShape {
    pub ar: usize,
    pub br: usize,
    pub bc: usize,
}}

// What the classical algorithm does with a product: sets the output to it, or adds it to the
// elements already in the output.
crate_test_enum! {
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
ModMulOp {
    Set,
    Add,
}}

// The smallest dimension at which the product of two matrices modulo `m` uses Strassen's algorithm
// rather than the classical one: 400 for moduli below 2048 with 64-bit words, and 200 otherwise.
//
// This is the cutoff from `nmod_mat_mul` in `nmod_mat/mul.c`, FLINT 3.6.0.
pub(crate) fn classical_cutoff<T: PrimitiveUnsigned>(m: T) -> usize {
    if T::WIDTH == u64::WIDTH && m < T::wrapping_from(2048u16) {
        400
    } else {
        200
    }
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, all reduced
// modulo `m`, which must be positive, choosing between the classical algorithm and Strassen's.
//
// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
// without BLAS.
crate_test_fn! {matrix_mod_mul<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
) -> Vec<T> {
    let ProductShape { ar, br, bc } = shape;
    if ar.min(br).min(bc) < classical_cutoff(m) {
        let mut out = vec![T::ZERO; ar * bc];
        matrix_mod_mul_classical_op(&mut out, ModMulOp::Set, xs, ys, shape, m);
        out
    } else {
        matrix_mod_mul_strassen(xs, ys, shape, m)
    }
}}

// Checks the arguments of a matrix product modulo `m`: `m` is positive, the width of the first
// matrix is the height of the second, and the elements of both are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(
        a.width, b.height,
        "the width of the first matrix must equal the height of the second"
    );
    assert!(
        a.mod_is_reduced(&m),
        "the first matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(&m),
        "the second matrix must be reduced mod m, but {b} has an element >= {m}"
    );
}

const fn product_shape<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
) -> ProductShape {
    ProductShape {
        ar: a.height,
        br: b.height,
        bc: b.width,
    }
}

fn mod_mul_matrix<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    assert_arguments(a, b, m);
    UnsignedMatrix {
        elements: matrix_mod_mul(&a.elements, &b.elements, product_shape(a, b), m),
        height: a.height,
        width: b.width,
    }
}

fn mod_mul_matrix_assign<T: PrimitiveUnsigned>(
    a: &mut UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) {
    assert_arguments(a, b, m);
    let shape = product_shape(a, b);
    let product = matrix_mod_mul(&a.elements, &b.elements, shape, m);
    // Keep the storage of `a`.
    a.elements.clear();
    a.elements.extend_from_slice(&product);
    a.width = b.width;
}

impl<T: PrimitiveUnsigned> ModMul<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedMatrix`]es modulo a number $m$, taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{k-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(h, k, w) = O(hkw)$
    ///
    /// $M(h, k, w) = O(hk + kw + hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $k$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 0, 1]").unwrap();
    /// assert_eq!(a.mod_mul(b, 7).to_string(), "[5, 1; 1, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_mul(b, 10).to_string(), "[5, 1; 4, 3]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
    /// without BLAS, except that it panics if the dimensions are incompatible, which FLINT checks
    /// only in debug builds.
    #[inline]
    fn mod_mul(self, other: Self, m: T) -> Self {
        mod_mul_matrix(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedMatrix`]es modulo a number $m$, taking the first by value and the
    /// second by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{k-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(h, k, w) = O(hkw)$
    ///
    /// $M(h, k, w) = O(hk + kw + hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $k$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 0, 1]").unwrap();
    /// assert_eq!(a.mod_mul(&b, 7).to_string(), "[5, 1; 1, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_mul(&b, 10).to_string(), "[5, 1; 4, 3]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
    /// without BLAS, except that it panics if the dimensions are incompatible, which FLINT checks
    /// only in debug builds.
    #[inline]
    fn mod_mul(self, other: &Self, m: T) -> Self {
        mod_mul_matrix(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Multiplies two [`UnsignedMatrix`]es modulo a number $m$, taking the first by reference and
    /// the second by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{k-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(h, k, w) = O(hkw)$
    ///
    /// $M(h, k, w) = O(hk + kw + hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $k$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 0, 1]").unwrap();
    /// assert_eq!((&a).mod_mul(b, 7).to_string(), "[5, 1; 1, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_mul(b, 10).to_string(), "[5, 1; 4, 3]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
    /// without BLAS, except that it panics if the dimensions are incompatible, which FLINT checks
    /// only in debug builds.
    #[inline]
    fn mod_mul(self, other: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_mul_matrix(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Multiplies two [`UnsignedMatrix`]es modulo a number $m$, taking both by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{k-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(h, k, w) = O(hkw)$
    ///
    /// $M(h, k, w) = O(hk + kw + hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $k$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 0, 1]").unwrap();
    /// assert_eq!((&a).mod_mul(&b, 7).to_string(), "[5, 1; 1, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_mul(&b, 10).to_string(), "[5, 1; 4, 3]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
    /// without BLAS, except that it panics if the dimensions are incompatible, which FLINT checks
    /// only in debug builds.
    #[inline]
    fn mod_mul(self, other: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_mul_matrix(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulAssign<Self, T> for UnsignedMatrix<T> {
    /// Multiplies an [`UnsignedMatrix`] by another modulo a number $m$ in place, taking the second
    /// by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{k-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(h, k, w) = O(hkw)$
    ///
    /// $M(h, k, w) = O(hk + kw + hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $k$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_mul_assign(UnsignedMatrix::from_str("[0, 1; 1, 0]").unwrap(), 5);
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a.mod_mul_assign(UnsignedMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap(), 10);
    /// assert_eq!(a.to_string(), "[5, 1; 4, 3]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
    /// without BLAS, except that it panics if the dimensions are incompatible, which FLINT checks
    /// only in debug builds.
    #[inline]
    fn mod_mul_assign(&mut self, other: Self, m: T) {
        mod_mul_matrix_assign(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModMulAssign<&Self, T> for UnsignedMatrix<T> {
    /// Multiplies an [`UnsignedMatrix`] by another modulo a number $m$ in place, taking the second
    /// by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{k-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(h, k, w) = O(hkw)$
    ///
    /// $M(h, k, w) = O(hk + kw + hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $k$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_mul_assign(&UnsignedMatrix::from_str("[0, 1; 1, 0]").unwrap(), 5);
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a.mod_mul_assign(&UnsignedMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap(), 10);
    /// assert_eq!(a.to_string(), "[5, 1; 4, 3]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_mul` from `nmod_mat/mul.c`, FLINT 3.6.0, with one thread and
    /// without BLAS, except that it panics if the dimensions are incompatible, which FLINT checks
    /// only in debug builds.
    #[inline]
    fn mod_mul_assign(&mut self, other: &Self, m: T) {
        mod_mul_matrix_assign(self, other, m);
    }
}
