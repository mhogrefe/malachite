// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::vector_mul::vector_matrix_mul;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Mul, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;

// Checks the arguments of a vector-matrix product modulo $2^k$: the vector's dimension is the
// height of the matrix, and the elements of both are reduced.
fn assert_arguments(v: &NaturalVector, a: &NaturalMatrix, pow: u64) {
    assert_eq!(
        v.elements.len(),
        a.height,
        "the vector's dimension must equal the height of the matrix"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "the vector must be reduced mod 2^{pow}, but {v} has an element >= 2^{pow}"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// The product `v * a` modulo $2^k$, after checking the arguments. When $k$ is at most the width of
// a limb, the rows of the matrix, scaled by the matching elements of the vector, are accumulated in
// wrapping limb arithmetic, since $2^k$ divides the limb modulus; otherwise the exact product is
// reduced once.
fn vector_mod_power_of_2_mul(v: &NaturalVector, a: &NaturalMatrix, pow: u64) -> NaturalVector {
    assert_arguments(v, a, pow);
    let width = a.width;
    let elements = if pow <= Limb::WIDTH {
        let mut out: Vec<Limb> = vec![0; width];
        if width != 0 {
            for (x, row) in v.elements.iter().zip(a.elements.chunks_exact(width)) {
                let x = Limb::wrapping_from(x);
                if x != 0 {
                    for (o, y) in out.iter_mut().zip(row) {
                        o.wrapping_add_assign(x.wrapping_mul(Limb::wrapping_from(y)));
                    }
                }
            }
        }
        out.into_iter()
            .map(|o| Natural::from(o.mod_power_of_2(pow)))
            .collect()
    } else {
        let mut elements = vector_matrix_mul(&v.elements, &a.elements, width);
        for x in &mut elements {
            x.mod_power_of_2_assign(pow);
        }
        elements
    };
    NaturalVector { elements }
}

impl ModPowerOf2Mul<NaturalMatrix> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo $2^k$, taking the
    /// vector by value and the matrix by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. Element $j$ of the result is the
    /// dot product of the vector with column $j$ of the matrix, modulo $2^k$:
    ///
    /// $$
    /// f(v, A, k)_j = \sum_{i=0}^{n-1} v_iA_{ij} \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the height of $A$, which must be the dimension of $v$.
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
    /// Panics if the dimension of `self` is not the height of `other`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_power_of_2_mul(a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: NaturalMatrix, pow: u64) -> Self {
        vector_mod_power_of_2_mul(&self, &other, pow)
    }
}

impl ModPowerOf2Mul<&NaturalMatrix> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo $2^k$, taking the
    /// vector by value and the matrix by reference.
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
    /// Panics if the dimension of `self` is not the height of `other`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_power_of_2_mul(&a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &NaturalMatrix, pow: u64) -> Self {
        vector_mod_power_of_2_mul(&self, other, pow)
    }
}

impl ModPowerOf2Mul<NaturalMatrix> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo $2^k$, taking the
    /// vector by reference and the matrix by value.
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
    /// Panics if the dimension of `self` is not the height of `other`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v).mod_power_of_2_mul(a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: NaturalMatrix, pow: u64) -> NaturalVector {
        vector_mod_power_of_2_mul(self, &other, pow)
    }
}

impl ModPowerOf2Mul<&NaturalMatrix> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo $2^k$, taking the
    /// vector by reference and the matrix by reference.
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
    /// Panics if the dimension of `self` is not the height of `other`, or if any element of either
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v).mod_power_of_2_mul(&a, 3).to_string(), "(1, 4, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &NaturalMatrix, pow: u64) -> NaturalVector {
        vector_mod_power_of_2_mul(self, other, pow)
    }
}
