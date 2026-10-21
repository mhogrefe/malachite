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

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{KroneckerProduct, KroneckerProductAssign};

// The elements, in row-major order, of the Kronecker product of `a` and `b`. Row $ip + k$ of the
// product, where $p$ is the height of `b`, is row $i$ of `a` with each element $x$ replaced by $x$
// times row $k$ of `b`, so the product is written in the order in which it is stored.
//
// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`, FLINT
// 3.6.0, which writes the product one block at a time instead.
fn kronecker_product_elements(a: &RationalMatrix, b: &RationalMatrix) -> Vec<Rational> {
    let mut out = Vec::with_capacity(a.elements.len() * b.elements.len());
    if a.width == 0 || b.width == 0 {
        return out;
    }
    for a_row in a.elements.chunks_exact(a.width) {
        for b_row in b.elements.chunks_exact(b.width) {
            for x in a_row {
                for y in b_row {
                    out.push(x * y);
                }
            }
        }
    }
    out
}

fn kronecker_product(a: &RationalMatrix, b: &RationalMatrix) -> RationalMatrix {
    RationalMatrix {
        elements: kronecker_product_elements(a, b),
        height: a.height * b.height,
        width: a.width * b.width,
    }
}

impl KroneckerProduct<Self> for RationalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`RationalMatrix`]s, taking both by value.
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2; 3, 1/4]").unwrap();
    /// let b = RationalMatrix::from_str("[0, 2/3; -6, 1]").unwrap();
    /// assert_eq!(
    ///     a.kronecker_product(b).to_string(),
    ///     "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3]").unwrap();
    /// let b = RationalMatrix::from_str("[3; -2]").unwrap();
    /// assert_eq!(a.kronecker_product(b).to_string(), "[3/2, 1; -1, -2/3]");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 2]").unwrap();
    /// assert_eq!(a.kronecker_product(b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: Self) -> Self {
        kronecker_product(&self, &other)
    }
}

impl KroneckerProduct<&Self> for RationalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`RationalMatrix`]s, taking the first by value and the
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2; 3, 1/4]").unwrap();
    /// let b = RationalMatrix::from_str("[0, 2/3; -6, 1]").unwrap();
    /// assert_eq!(
    ///     a.kronecker_product(&b).to_string(),
    ///     "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3]").unwrap();
    /// let b = RationalMatrix::from_str("[3; -2]").unwrap();
    /// assert_eq!(a.kronecker_product(&b).to_string(), "[3/2, 1; -1, -2/3]");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 2]").unwrap();
    /// assert_eq!(a.kronecker_product(&b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: &Self) -> Self {
        kronecker_product(&self, other)
    }
}

impl KroneckerProduct<RationalMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Computes the Kronecker product of two [`RationalMatrix`]s, taking the first by reference and
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2; 3, 1/4]").unwrap();
    /// let b = RationalMatrix::from_str("[0, 2/3; -6, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(b).to_string(),
    ///     "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3]").unwrap();
    /// let b = RationalMatrix::from_str("[3; -2]").unwrap();
    /// assert_eq!((&a).kronecker_product(b).to_string(), "[3/2, 1; -1, -2/3]");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: RationalMatrix) -> RationalMatrix {
        kronecker_product(self, &other)
    }
}

impl KroneckerProduct<&RationalMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Computes the Kronecker product of two [`RationalMatrix`]s, taking both by reference.
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2; 3, 1/4]").unwrap();
    /// let b = RationalMatrix::from_str("[0, 2/3; -6, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(&b).to_string(),
    ///     "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3]").unwrap();
    /// let b = RationalMatrix::from_str("[3; -2]").unwrap();
    /// assert_eq!((&a).kronecker_product(&b).to_string(), "[3/2, 1; -1, -2/3]");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(&b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: &RationalMatrix) -> RationalMatrix {
        kronecker_product(self, other)
    }
}

impl KroneckerProductAssign<Self> for RationalMatrix {
    /// Replaces a [`RationalMatrix`] with its Kronecker product with another [`RationalMatrix`],
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -2; 3, 1/4]").unwrap();
    /// a.kronecker_product_assign(RationalMatrix::from_str("[0, 2/3; -6, 1]").unwrap());
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]"
    /// );
    ///
    /// let mut a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// a.kronecker_product_assign(RationalMatrix::from_str("[1, 1/2; 3, 3/4]").unwrap());
    /// assert_eq!(a.to_string(), "[-2/3, -1/3; -2, -1/2]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product_assign(&mut self, other: Self) {
        *self = kronecker_product(self, &other);
    }
}

impl KroneckerProductAssign<&Self> for RationalMatrix {
    /// Replaces a [`RationalMatrix`] with its Kronecker product with another [`RationalMatrix`],
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -2; 3, 1/4]").unwrap();
    /// a.kronecker_product_assign(&RationalMatrix::from_str("[0, 2/3; -6, 1]").unwrap());
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]"
    /// );
    ///
    /// let mut a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// a.kronecker_product_assign(&RationalMatrix::from_str("[1, 1/2; 3, 3/4]").unwrap());
    /// assert_eq!(a.to_string(), "[-2/3, -1/3; -2, -1/2]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_kronecker_product` from `fmpq_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product_assign(&mut self, other: &Self) {
        *self = kronecker_product(self, other);
    }
}
