// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2KroneckerProduct, ModPowerOf2KroneckerProductAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;

// Checks the arguments of a Kronecker product modulo $2^k$: `pow` is at most the width of `T`, and
// the elements of both matrices are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
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

// The Kronecker product of `a` and `b` modulo $2^k$, after checking the arguments, written one row
// at a time, in storage order. Each element is a wrapping product with all but its lowest $k$ bits
// cleared, which is exact because $2^k$ divides the word modulus.
fn mod_power_of_2_kronecker_product<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    assert_arguments(a, b, pow);
    let mut elements = Vec::with_capacity(a.elements.len() * b.elements.len());
    if a.width != 0 && b.width != 0 {
        for a_row in a.elements.chunks_exact(a.width) {
            for b_row in b.elements.chunks_exact(b.width) {
                for &x in a_row {
                    for &y in b_row {
                        elements.push(x.wrapping_mul(y).mod_power_of_2(pow));
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

impl<T: PrimitiveUnsigned> ModPowerOf2KroneckerProduct<Self> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $2^k$, taking both by
    /// value.
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of either matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[255, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(b, 0).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(self, other: Self, pow: u64) -> Self {
        mod_power_of_2_kronecker_product(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2KroneckerProduct<&Self> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $2^k$, taking the first by
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of either matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(&b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[255, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(&b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_kronecker_product(&b, 0).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(self, other: &Self, pow: u64) -> Self {
        mod_power_of_2_kronecker_product(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2KroneckerProduct<UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $2^k$, taking the first by
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of either matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[255, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(b, 0).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(
        self,
        other: UnsignedMatrix<T>,
        pow: u64,
    ) -> UnsignedMatrix<T> {
        mod_power_of_2_kronecker_product(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2KroneckerProduct<&UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the Kronecker product of two [`UnsignedMatrix`]s modulo $2^k$, taking both by
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of either matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProduct;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(&b, 4).to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[255, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3; 129]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(&b, 8).to_string(),
    ///     "[253, 6; 127, 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0; 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_kronecker_product(&b, 0).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product(
        self,
        other: &UnsignedMatrix<T>,
        pow: u64,
    ) -> UnsignedMatrix<T> {
        mod_power_of_2_kronecker_product(self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2KroneckerProductAssign<Self> for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its Kronecker product with another [`UnsignedMatrix`]
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of either matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProductAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(b, 4);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[255]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3, 129]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(b, 8);
    /// assert_eq!(a.to_string(), "[253, 127]");
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product_assign(&mut self, other: Self, pow: u64) {
        *self = mod_power_of_2_kronecker_product(self, &other, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2KroneckerProductAssign<&Self> for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its Kronecker product with another [`UnsignedMatrix`]
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// product.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of either matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2KroneckerProductAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(&b, 4);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]"
    /// );
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[255]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[3, 129]").unwrap();
    /// a.mod_power_of_2_kronecker_product_assign(&b, 8);
    /// assert_eq!(a.to_string(), "[253, 127]");
    /// ```
    #[inline]
    fn mod_power_of_2_kronecker_product_assign(&mut self, other: &Self, pow: u64) {
        *self = mod_power_of_2_kronecker_product(self, other, pow);
    }
}
