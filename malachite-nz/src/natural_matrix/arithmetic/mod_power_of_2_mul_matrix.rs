// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul, matrix_mul_assign};
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a matrix product modulo $2^k$: the width of the first matrix is the
// height of the second, and the elements of both are reduced.
fn assert_arguments(a: &NaturalMatrix, b: &NaturalMatrix, pow: u64) {
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

const fn product_shape(a: &NaturalMatrix, b: &NaturalMatrix) -> ProductShape {
    ProductShape {
        ar: a.height,
        br: b.height,
        bc: b.width,
    }
}

// The matrix with elements `xs`, which fit in limbs, as an `UnsignedMatrix` of limbs.
fn limb_matrix(height: usize, width: usize, xs: &[Natural]) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        u64::exact_from(height),
        u64::exact_from(width),
        xs.iter().map(Limb::wrapping_from).collect(),
    )
}

// The elements of the product of `a` and `b` modulo $2^k$, which must be at most the width of a
// limb, so that every element fits in a limb: the product of the matrices as `UnsignedMatrix`es of
// limbs, which is summed in wrapping limb arithmetic, since $2^k$ divides the limb modulus.
fn limb_product(a: &NaturalMatrix, b: &NaturalMatrix, pow: u64) -> Vec<Natural> {
    limb_matrix(a.height, a.width, &a.elements)
        .mod_power_of_2_mul(limb_matrix(b.height, b.width, &b.elements), pow)
        .into_elements()
        .into_iter()
        .map(Natural::from)
        .collect()
}

fn mod_power_of_2_mul_matrix(a: &NaturalMatrix, b: &NaturalMatrix, pow: u64) -> NaturalMatrix {
    assert_arguments(a, b, pow);
    let elements = if pow <= Limb::WIDTH {
        limb_product(a, b, pow)
    } else {
        let mut elements = matrix_mul(&a.elements, &b.elements, product_shape(a, b));
        for x in &mut elements {
            x.mod_power_of_2_assign(pow);
        }
        elements
    };
    NaturalMatrix {
        elements,
        height: a.height,
        width: b.width,
    }
}

fn mod_power_of_2_mul_matrix_assign(a: &mut NaturalMatrix, b: &NaturalMatrix, pow: u64) {
    assert_arguments(a, b, pow);
    if pow <= Limb::WIDTH {
        a.elements = limb_product(a, b, pow);
    } else {
        let shape = product_shape(a, b);
        matrix_mul_assign(&mut a.elements, &b.elements, shape);
        for x in &mut a.elements {
            x.mod_power_of_2_assign(pow);
        }
    }
    a.width = b.width;
}

impl ModPowerOf2Mul<Self> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]es modulo $2^k$, taking both by value.
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
    /// When $n$ is 0, the product is the $h \times w$ zero matrix. When $k$ is at most the width of
    /// a limb, the products and sums are done in wrapping limb arithmetic, as for
    /// [`UnsignedMatrix`]es, since $2^k$ divides the limb modulus; otherwise the exact product is
    /// computed, as by [`Mul`](core::ops::Mul), and each of its elements reduced once.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second, or if any element
    /// of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(b, 100).to_string(), "[19, 22; 43, 50]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: Self, pow: u64) -> Self {
        mod_power_of_2_mul_matrix(&self, &other, pow)
    }
}

impl ModPowerOf2Mul<&Self> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]es modulo $2^k$, taking the first by value and the second by
    /// reference.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second, or if any element
    /// of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(&b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul(&b, 100).to_string(),
    ///     "[19, 22; 43, 50]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_power_of_2_mul(&b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &Self, pow: u64) -> Self {
        mod_power_of_2_mul_matrix(&self, other, pow)
    }
}

impl ModPowerOf2Mul<NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]es modulo $2^k$, taking the first by reference and the
    /// second by value.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second, or if any element
    /// of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul(b, 100).to_string(),
    ///     "[19, 22; 43, 50]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_mul_matrix(self, &other, pow)
    }
}

impl ModPowerOf2Mul<&NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]es modulo $2^k$, taking both by reference.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second, or if any element
    /// of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(&b, 4).to_string(), "[3, 6; 11, 2]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul(&b, 100).to_string(),
    ///     "[19, 22; 43, 50]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(&b, 4).to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_mul_matrix(self, other, pow)
    }
}

impl ModPowerOf2MulAssign<Self> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another modulo $2^k$ in place, taking the second by value.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second, or if any element
    /// of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_mul_assign(NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap(), 3);
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_mul_assign(NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap(), 100);
    /// assert_eq!(a.to_string(), "[19, 22; 43, 50]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a.mod_power_of_2_mul_assign(NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap(), 4);
    /// assert_eq!(a.to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, other: Self, pow: u64) {
        mod_power_of_2_mul_matrix_assign(self, &other, pow);
    }
}

impl ModPowerOf2MulAssign<&Self> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another modulo $2^k$ in place, taking the second by
    /// reference.
    ///
    /// See the documentation for the implementation that takes both matrices by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second, or if any element
    /// of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_mul_assign(&NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap(), 3);
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_mul_assign(&NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap(), 100);
    /// assert_eq!(a.to_string(), "[19, 22; 43, 50]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a.mod_power_of_2_mul_assign(&NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap(), 4);
    /// assert_eq!(a.to_string(), "[5, 11; 14, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64) {
        mod_power_of_2_mul_matrix_assign(self, other, pow);
    }
}
