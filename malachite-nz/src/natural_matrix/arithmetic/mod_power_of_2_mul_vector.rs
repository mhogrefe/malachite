// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul_vector::matrix_vector_mul;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Mul, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;

// Checks the arguments of a matrix-vector product modulo $2^k$: the vector's dimension is the width
// of the matrix, and the elements of both are reduced.
fn assert_arguments(a: &NaturalMatrix, v: &NaturalVector, pow: u64) {
    assert_eq!(
        v.elements.len(),
        a.width,
        "the vector's dimension must equal the width of the matrix"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "the vector must be reduced mod 2^{pow}, but {v} has an element >= 2^{pow}"
    );
}

// The product `a * v` modulo $2^k$, after checking the arguments. When $k$ is at most the width of
// a limb, every element fits in a limb, and each dot product is summed in wrapping limb arithmetic,
// since $2^k$ divides the limb modulus; otherwise the exact product is reduced once.
fn mod_power_of_2_mul_vector(a: &NaturalMatrix, v: &NaturalVector, pow: u64) -> NaturalVector {
    assert_arguments(a, v, pow);
    let width = a.width;
    let elements = if pow <= Limb::WIDTH {
        (0..a.height)
            .map(|i| {
                let mut sum: Limb = 0;
                for (x, y) in a.elements[i * width..(i + 1) * width]
                    .iter()
                    .zip(&v.elements)
                {
                    sum.wrapping_add_assign(
                        Limb::wrapping_from(x).wrapping_mul(Limb::wrapping_from(y)),
                    );
                }
                Natural::from(sum.mod_power_of_2(pow))
            })
            .collect()
    } else {
        let mut elements = matrix_vector_mul(&a.elements, a.height, width, &v.elements);
        for x in &mut elements {
            x.mod_power_of_2_assign(pow);
        }
        elements
    };
    NaturalVector { elements }
}

impl ModPowerOf2Mul<NaturalVector> for NaturalMatrix {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalMatrix`] by a [`NaturalVector`] on the right modulo $2^k$, taking the
    /// matrix by value and the vector by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. Element $i$ of the result is the
    /// dot product of row $i$ of the matrix with the vector, modulo $2^k$:
    ///
    /// $$
    /// f(A, v, k)_i = \sum_{j=0}^{n-1} A_{ij}v_j \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the width of $A$, which must be the dimension of $v$.
    ///
    /// When $k$ is at most the width of a limb, the products and sums are done in wrapping limb
    /// arithmetic, since $2^k$ divides the limb modulus; otherwise the exact product is computed
    /// and each of its elements reduced once.
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
    /// Panics if the dimension of `other` is not the width of `self`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = NaturalVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a).mod_power_of_2_mul(v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: NaturalVector, pow: u64) -> NaturalVector {
        mod_power_of_2_mul_vector(&self, &other, pow)
    }
}

impl ModPowerOf2Mul<&NaturalVector> for NaturalMatrix {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalMatrix`] by a [`NaturalVector`] on the right modulo $2^k$, taking the
    /// matrix by value and the vector by reference.
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
    /// Panics if the dimension of `other` is not the width of `self`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = NaturalVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a).mod_power_of_2_mul(&v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &NaturalVector, pow: u64) -> NaturalVector {
        mod_power_of_2_mul_vector(&self, other, pow)
    }
}

impl ModPowerOf2Mul<NaturalVector> for &NaturalMatrix {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalMatrix`] by a [`NaturalVector`] on the right modulo $2^k$, taking the
    /// matrix by reference and the vector by value.
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
    /// Panics if the dimension of `other` is not the width of `self`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = NaturalVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: NaturalVector, pow: u64) -> NaturalVector {
        mod_power_of_2_mul_vector(self, &other, pow)
    }
}

impl ModPowerOf2Mul<&NaturalVector> for &NaturalMatrix {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalMatrix`] by a [`NaturalVector`] on the right modulo $2^k$, taking the
    /// matrix by reference and the vector by reference.
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
    /// Panics if the dimension of `other` is not the width of `self`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = NaturalVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(&v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &NaturalVector, pow: u64) -> NaturalVector {
        mod_power_of_2_mul_vector(self, other, pow)
    }
}
