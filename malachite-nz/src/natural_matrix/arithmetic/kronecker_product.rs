// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::kronecker_product::kronecker_product_elements;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{KroneckerProduct, KroneckerProductAssign};

fn kronecker_product(a: &NaturalMatrix, b: &NaturalMatrix) -> NaturalMatrix {
    NaturalMatrix {
        elements: kronecker_product_elements(&a.elements, a.width, &b.elements, b.width),
        height: a.height * b.height,
        width: a.width * b.width,
    }
}

impl KroneckerProduct<Self> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s, taking both by value.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.kronecker_product(b).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 20; 18, 21, 24, 28]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(a.kronecker_product(b).to_string(), "[1, 2; 2, 4]");
    ///
    /// let a = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 2]").unwrap();
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

impl KroneckerProduct<&Self> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s, taking the first by value and the
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.kronecker_product(&b).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 20; 18, 21, 24, 28]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(a.kronecker_product(&b).to_string(), "[1, 2; 2, 4]");
    ///
    /// let a = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 2]").unwrap();
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

impl KroneckerProduct<NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s, taking the first by reference and
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(b).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 20; 18, 21, 24, 28]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(b).to_string(), "[1, 2; 2, 4]");
    ///
    /// let a = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: NaturalMatrix) -> NaturalMatrix {
        kronecker_product(self, &other)
    }
}

impl KroneckerProduct<&NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s, taking both by reference.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).kronecker_product(&b).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 20; 18, 21, 24, 28]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(&b).to_string(), "[1, 2; 2, 4]");
    ///
    /// let a = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 2]").unwrap();
    /// assert_eq!((&a).kronecker_product(&b).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product(self, other: &NaturalMatrix) -> NaturalMatrix {
        kronecker_product(self, other)
    }
}

impl KroneckerProductAssign<Self> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`],
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.kronecker_product_assign(NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap());
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 20; 18, 21, 24, 28]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[2]").unwrap();
    /// a.kronecker_product_assign(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap());
    /// assert_eq!(a.to_string(), "[2, 4; 6, 8]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product_assign(&mut self, other: Self) {
        *self = kronecker_product(self, &other);
    }
}

impl KroneckerProductAssign<&Self> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`],
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.kronecker_product_assign(&NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap());
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 20; 18, 21, 24, 28]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[2]").unwrap();
    /// a.kronecker_product_assign(&NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap());
    /// assert_eq!(a.to_string(), "[2, 4; 6, 8]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_kronecker_product` from `fmpz_mat/kronecker_product.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn kronecker_product_assign(&mut self, other: &Self) {
        *self = kronecker_product(self, other);
    }
}
