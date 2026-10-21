// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2018 Martin Raum
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use alloc::vec::Vec;
use core::ops::Mul;
use malachite_base::num::arithmetic::traits::{KroneckerProduct, KroneckerProductAssign};

// The elements, in row-major order, of the Kronecker product of the matrix with elements `xs` and
// width `a_width` and the matrix with elements `ys` and width `b_width`. Row $ip + k$ of the
// product, where $p$ is the height of the second matrix, is row $i$ of the first matrix with each
// element $x$ replaced by $x$ times row $k$ of the second, so the product is written in the order
// in which it is stored.
//
// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`, FLINT
// 3.6.0, which writes the product one block at a time instead.
pub(crate) fn kronecker_product_elements<T>(
    xs: &[T],
    a_width: usize,
    ys: &[T],
    b_width: usize,
) -> Vec<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    let mut out = Vec::with_capacity(xs.len() * ys.len());
    if a_width == 0 || b_width == 0 {
        return out;
    }
    for a_row in xs.chunks_exact(a_width) {
        for b_row in ys.chunks_exact(b_width) {
            for x in a_row {
                for y in b_row {
                    out.push(x * y);
                }
            }
        }
    }
    out
}

fn kronecker_product(a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
    IntegerMatrix {
        elements: kronecker_product_elements(&a.elements, a.width, &b.elements, b.width),
        height: a.height * b.height,
        width: a.width * b.width,
    }
}

impl KroneckerProduct<Self> for IntegerMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`IntegerMatrix`]s, taking both by value.
    ///
    /// The Kronecker product $A \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix
    /// $B$ is the $mp \times nq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$:
    ///
    /// $$
    /// (A \otimes B)_{ip + k, jq + l} = A_{ij}B_{kl}.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $mp
    /// \times nq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is the largest number of significant bits of any element of either matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::KroneckerProduct;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = IntegerMatrix::from_str("[0, 5; -6, 7]").unwrap();
    /// assert_eq!(
    ///     a.kronecker_product(b).to_string(),
    ///     "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1; 1]").unwrap();
    /// assert_eq!(a.kronecker_product(b).to_string(), "[-1, -2, -3; 1, 2, 3]");
    ///
    /// let a = IntegerMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 2]").unwrap();
    /// assert_eq!(a.kronecker_product(b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: Self) -> Self {
        kronecker_product(&self, &other)
    }
}

impl KroneckerProduct<&Self> for IntegerMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`IntegerMatrix`]s, taking the first by value and the
    /// second by reference.
    ///
    /// The Kronecker product $A \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix
    /// $B$ is the $mp \times nq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$:
    ///
    /// $$
    /// (A \otimes B)_{ip + k, jq + l} = A_{ij}B_{kl}.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $mp
    /// \times nq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is the largest number of significant bits of any element of either matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::KroneckerProduct;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = IntegerMatrix::from_str("[0, 5; -6, 7]").unwrap();
    /// assert_eq!(
    ///     a.kronecker_product(&b).to_string(),
    ///     "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1; 1]").unwrap();
    /// assert_eq!(a.kronecker_product(&b).to_string(), "[-1, -2, -3; 1, 2, 3]");
    ///
    /// let a = IntegerMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 2]").unwrap();
    /// assert_eq!(a.kronecker_product(&b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: &Self) -> Self {
        kronecker_product(&self, other)
    }
}

impl KroneckerProduct<IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes the Kronecker product of two [`IntegerMatrix`]s, taking the first by reference and
    /// the second by value.
    ///
    /// The Kronecker product $A \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix
    /// $B$ is the $mp \times nq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$:
    ///
    /// $$
    /// (A \otimes B)_{ip + k, jq + l} = A_{ij}B_{kl}.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $mp
    /// \times nq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is the largest number of significant bits of any element of either matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::KroneckerProduct;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = IntegerMatrix::from_str("[0, 5; -6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(b).to_string(),
    ///     "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1; 1]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(b).to_string(),
    ///     "[-1, -2, -3; 1, 2, 3]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: IntegerMatrix) -> IntegerMatrix {
        kronecker_product(self, &other)
    }
}

impl KroneckerProduct<&IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes the Kronecker product of two [`IntegerMatrix`]s, taking both by reference.
    ///
    /// The Kronecker product $A \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix
    /// $B$ is the $mp \times nq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$:
    ///
    /// $$
    /// (A \otimes B)_{ip + k, jq + l} = A_{ij}B_{kl}.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $mp
    /// \times nq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is the largest number of significant bits of any element of either matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::KroneckerProduct;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = IntegerMatrix::from_str("[0, 5; -6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(&b).to_string(),
    ///     "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1; 1]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(&b).to_string(),
    ///     "[-1, -2, -3; 1, 2, 3]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(&b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: &IntegerMatrix) -> IntegerMatrix {
        kronecker_product(self, other)
    }
}

impl KroneckerProductAssign<Self> for IntegerMatrix {
    /// Replaces a [`IntegerMatrix`] with its Kronecker product with another [`IntegerMatrix`],
    /// taking the second by value.
    ///
    /// The Kronecker product $A \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix
    /// $B$ is the $mp \times nq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$:
    ///
    /// $$
    /// (A \otimes B)_{ip + k, jq + l} = A_{ij}B_{kl}.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $mp
    /// \times nq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is the largest number of significant bits of any element of either matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::KroneckerProductAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a.kronecker_product_assign(IntegerMatrix::from_str("[0, 5; -6, 7]").unwrap());
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]"
    /// );
    ///
    /// let mut a = IntegerMatrix::from_str("[-2]").unwrap();
    /// a.kronecker_product_assign(IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap());
    /// assert_eq!(a.to_string(), "[-2, -4; -6, -8]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product_assign(&mut self, other: Self) {
        *self = kronecker_product(self, &other);
    }
}

impl KroneckerProductAssign<&Self> for IntegerMatrix {
    /// Replaces a [`IntegerMatrix`] with its Kronecker product with another [`IntegerMatrix`],
    /// taking the second by reference.
    ///
    /// The Kronecker product $A \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix
    /// $B$ is the $mp \times nq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$:
    ///
    /// $$
    /// (A \otimes B)_{ip + k, jq + l} = A_{ij}B_{kl}.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $mp
    /// \times nq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is the largest number of significant bits of any element of either matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::KroneckerProductAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a.kronecker_product_assign(&IntegerMatrix::from_str("[0, 5; -6, 7]").unwrap());
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]"
    /// );
    ///
    /// let mut a = IntegerMatrix::from_str("[-2]").unwrap();
    /// a.kronecker_product_assign(&IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap());
    /// assert_eq!(a.to_string(), "[-2, -4; -6, -8]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product_assign(&mut self, other: &Self) {
        *self = kronecker_product(self, other);
    }
}
