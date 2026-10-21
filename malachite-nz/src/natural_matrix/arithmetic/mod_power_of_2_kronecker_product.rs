// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2KroneckerProduct, ModPowerOf2KroneckerProductAssign,
    ModPowerOf2Mul,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a Kronecker product modulo $2^k$: the elements of both matrices are
// reduced.
fn assert_arguments(a: &NaturalMatrix, b: &NaturalMatrix, pow: u64) {
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the first matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    assert!(
        b.mod_power_of_2_is_reduced(pow),
        "the second matrix must be reduced mod 2^{pow}, but {b} has an element >= 2^{pow}"
    );
}

// The matrix with elements `xs`, which fit in limbs, as an `UnsignedMatrix` of limbs.
fn to_limb_matrix(a: &NaturalMatrix) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(Limb::wrapping_from).collect(),
    )
}

// The Kronecker product of `a` and `b` modulo $2^k$, after checking the arguments. When $k$ is at
// most the width of a limb, every element fits in a limb, and the product is that of the matrices
// as `UnsignedMatrix`es of limbs; otherwise each element is computed with `Natural`'s
// `ModPowerOf2Mul`, one row at a time, in storage order.
fn mod_power_of_2_kronecker_product(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    assert_arguments(a, b, pow);
    if pow <= Limb::WIDTH {
        return NaturalMatrix::from(
            to_limb_matrix(a).mod_power_of_2_kronecker_product(to_limb_matrix(b), pow),
        );
    }
    let mut elements = Vec::with_capacity(a.elements.len() * b.elements.len());
    if a.width != 0 && b.width != 0 {
        for a_row in a.elements.chunks_exact(a.width) {
            for b_row in b.elements.chunks_exact(b.width) {
                for x in a_row {
                    for y in b_row {
                        elements.push(x.mod_power_of_2_mul(y, pow));
                    }
                }
            }
        }
    }
    NaturalMatrix {
        elements,
        height: a.height * b.height,
        width: a.width * b.width,
    }
}

impl ModPowerOf2KroneckerProduct<Self> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $2^k$, taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The Kronecker product $A
    /// \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix $B$ is the $mp \times nq$
    /// block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that product with
    /// every element reduced modulo $2^k$:
    ///
    /// $$
    /// (A \otimes B \bmod 2^k)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod 2^k.
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
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of either matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[255, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[633825300114114700748351602689, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[316912650057057350374175801351; 5]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(b, 100).to_string(),
    ///     "[950737950171172051122527404039, \
    ///     950737950171172051122527404053; 633825300114114700748351602693, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(self, other: Self, pow: u64) -> Self {
        mod_power_of_2_kronecker_product(&self, &other, pow)
    }
}

impl ModPowerOf2KroneckerProduct<&Self> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $2^k$, taking the first by
    /// value and the second by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The Kronecker product $A
    /// \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix $B$ is the $mp \times nq$
    /// block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that product with
    /// every element reduced modulo $2^k$:
    ///
    /// $$
    /// (A \otimes B \bmod 2^k)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod 2^k.
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
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of either matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(&b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[255, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(&b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[633825300114114700748351602689, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[316912650057057350374175801351; 5]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(&b, 100).to_string(),
    ///     "[950737950171172051122527404039, \
    ///     950737950171172051122527404053; 633825300114114700748351602693, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(self, other: &Self, pow: u64) -> Self {
        mod_power_of_2_kronecker_product(&self, other, pow)
    }
}

impl ModPowerOf2KroneckerProduct<NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $2^k$, taking the first by
    /// reference and the second by value.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The Kronecker product $A
    /// \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix $B$ is the $mp \times nq$
    /// block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that product with
    /// every element reduced modulo $2^k$:
    ///
    /// $$
    /// (A \otimes B \bmod 2^k)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod 2^k.
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
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of either matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[255, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[633825300114114700748351602689, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[316912650057057350374175801351; 5]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(b, 100).to_string(),
    ///     "[950737950171172051122527404039, \
    ///     950737950171172051122527404053; 633825300114114700748351602693, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(self, other: NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_kronecker_product(self, &other, pow)
    }
}

impl ModPowerOf2KroneckerProduct<&NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $2^k$, taking both by
    /// reference.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The Kronecker product $A
    /// \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix $B$ is the $mp \times nq$
    /// block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that product with
    /// every element reduced modulo $2^k$:
    ///
    /// $$
    /// (A \otimes B \bmod 2^k)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod 2^k.
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
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of either matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(&b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[255, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(&b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[633825300114114700748351602689, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[316912650057057350374175801351; 5]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(&b, 100).to_string(),
    ///     "[950737950171172051122527404039, \
    ///     950737950171172051122527404053; 633825300114114700748351602693, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(self, other: &NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_kronecker_product(self, other, pow)
    }
}

impl ModPowerOf2KroneckerProductAssign<Self> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`]
    /// modulo $2^k$, taking the second by value.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The Kronecker product $A
    /// \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix $B$ is the $mp \times nq$
    /// block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that product with
    /// every element reduced modulo $2^k$:
    ///
    /// $$
    /// (A \otimes B \bmod 2^k)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod 2^k.
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
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of either matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProductAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(b, 4);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[633825300114114700748351602689]").unwrap();
    /// let b = NaturalMatrix::from_str("[316912650057057350374175801351, 5]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(b, 100);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[950737950171172051122527404039, 633825300114114700748351602693]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product_assign(&mut self, other: Self, pow: u64) {
        *self = mod_power_of_2_kronecker_product(self, &other, pow);
    }
}

impl ModPowerOf2KroneckerProductAssign<&Self> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`]
    /// modulo $2^k$, taking the second by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $2^k$. The Kronecker product $A
    /// \otimes B$ of an $m \times n$ matrix $A$ and a $p \times q$ matrix $B$ is the $mp \times nq$
    /// block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that product with
    /// every element reduced modulo $2^k$:
    ///
    /// $$
    /// (A \otimes B \bmod 2^k)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod 2^k.
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
    /// and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of either matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProductAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(&b, 4);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[633825300114114700748351602689]").unwrap();
    /// let b = NaturalMatrix::from_str("[316912650057057350374175801351, 5]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(&b, 100);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[950737950171172051122527404039, 633825300114114700748351602693]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product_assign(&mut self, other: &Self, pow: u64) {
        *self = mod_power_of_2_kronecker_product(self, other, pow);
    }
}
