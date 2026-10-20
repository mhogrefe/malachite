// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::classical::*;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::strassen::*;
use alloc::vec::Vec;

// Checks the arguments of a matrix product modulo $2^k$: `pow` is at most the width of `T`, the
// width of the first matrix is the height of the second, and the elements of both are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
    );
    assert_eq!(
        a.width, b.height,
        "the width of the first matrix must equal the height of the second"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the first matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    assert!(
        b.mod_power_of_2_is_reduced(pow),
        "the second matrix must be reduced mod 2^{pow}, but {b} has an element >= 2^{pow}"
    );
}

/// The classical algorithm.
pub mod classical;
/// Strassen's algorithm.
pub mod strassen;

// The smallest dimension at which a product in wrapping arithmetic uses Strassen's algorithm rather
// than the classical one. The classical algorithm's inner loop vectorizes for narrow types, which
// pushes the cutoff up; these values were measured on aarch64.
const fn strassen_cutoff<T: PrimitiveUnsigned>() -> usize {
    match T::WIDTH {
        8 => 512,
        16 => 384,
        32 => 192,
        _ => 48,
    }
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, both in
// row-major order, in wrapping arithmetic, choosing between the classical algorithm and Strassen's.
crate_test_fn! {matrix_wrapping_mul<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
) -> Vec<T> {
    let ProductShape { ar, br, bc } = shape;
    if ar.min(br).min(bc) < strassen_cutoff::<T>() {
        matrix_wrapping_mul_classical(xs, ys, shape)
    } else {
        matrix_wrapping_mul_strassen(xs, ys, shape)
    }
}}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, both in
// row-major order and reduced modulo $2^k$, modulo $2^k$. The product is computed in wrapping
// arithmetic, and all but the lowest $k$ bits of each element are cleared at the end, which is
// correct since $2^k$ divides $2^W$.
crate_test_fn! {matrix_mod_power_of_2_mul<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    pow: u64,
) -> Vec<T> {
    let mut out = matrix_wrapping_mul(xs, ys, shape);
    for z in &mut out {
        z.mod_power_of_2_assign(pow);
    }
    out
}}

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

fn mod_power_of_2_mul_matrix<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    assert_arguments(a, b, pow);
    UnsignedMatrix {
        elements: matrix_mod_power_of_2_mul(&a.elements, &b.elements, product_shape(a, b), pow),
        height: a.height,
        width: b.width,
    }
}

fn mod_power_of_2_mul_matrix_assign<T: PrimitiveUnsigned>(
    a: &mut UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
) {
    assert_arguments(a, b, pow);
    let product = matrix_mod_power_of_2_mul(&a.elements, &b.elements, product_shape(a, b), pow);
    // Keep the storage of `a`.
    a.elements.clear();
    a.elements.extend_from_slice(&product);
    a.width = b.width;
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<Self> for UnsignedMatrix<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedMatrix`]es modulo $2^k$, taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The product $AB$ of an
    /// $h \times n$ matrix $A$ and an $n \times w$ matrix $B$ is the $h \times w$ matrix whose
    /// element in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of
    /// $B$, reduced modulo $2^k$:
    ///
    /// $$
    /// f(A, B, k)_{ij} = \left ( \sum_{l=0}^{n-1} A_{il}B_{lj} \right ) \bmod 2^k.
    /// $$
    ///
    /// When $n$ is 0, the product is the $h \times w$ zero matrix. The products and sums are done
    /// in wrapping arithmetic, and all but the lowest $k$ bits of each result are cleared at the
    /// end, which is correct since $2^k$ divides $2^W$. Large matrices are multiplied with
    /// Strassen's algorithm, which needs only ring operations and so also works in wrapping
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w) = O(hnw)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the width of the first matrix is not the
    /// height of the second, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: Self, pow: u64) -> Self {
        mod_power_of_2_mul_matrix(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&Self> for UnsignedMatrix<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedMatrix`]es modulo $2^k$, taking the first by value and the second
    /// by reference.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w) = O(hnw)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the width of the first matrix is not the
    /// height of the second, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(&b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(&b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &Self, pow: u64) -> Self {
        mod_power_of_2_mul_matrix(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Multiplies two [`UnsignedMatrix`]es modulo $2^k$, taking the first by reference and the
    /// second by value.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w) = O(hnw)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the width of the first matrix is not the
    /// height of the second, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: UnsignedMatrix<T>, pow: u64) -> UnsignedMatrix<T> {
        mod_power_of_2_mul_matrix(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Multiplies two [`UnsignedMatrix`]es modulo $2^k$, taking both by reference.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w) = O(hnw)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the width of the first matrix is not the
    /// height of the second, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(&b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(&b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &UnsignedMatrix<T>, pow: u64) -> UnsignedMatrix<T> {
        mod_power_of_2_mul_matrix(self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulAssign<Self> for UnsignedMatrix<T> {
    /// Multiplies an [`UnsignedMatrix`] by another modulo $2^k$ in place, taking the second by
    /// value.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w) = O(hnw)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the width of the first matrix is not the
    /// height of the second, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_mul_assign(UnsignedMatrix::from_str("[0, 1; 1, 0]").unwrap(), 3);
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a.mod_power_of_2_mul_assign(UnsignedMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap(), 4);
    /// assert_eq!(a.to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, other: Self, pow: u64) {
        mod_power_of_2_mul_matrix_assign(self, &other, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulAssign<&Self> for UnsignedMatrix<T> {
    /// Multiplies an [`UnsignedMatrix`] by another modulo $2^k$ in place, taking the second by
    /// reference.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w) = O(hnw)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, and $w$ is the width of the second matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the width of the first matrix is not the
    /// height of the second, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_mul_assign(&UnsignedMatrix::from_str("[0, 1; 1, 0]").unwrap(), 3);
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a.mod_power_of_2_mul_assign(&UnsignedMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap(), 4);
    /// assert_eq!(a.to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64) {
        mod_power_of_2_mul_matrix_assign(self, other, pow);
    }
}
