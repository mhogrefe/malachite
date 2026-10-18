// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;

// Checks the arguments of a matrix-vector product modulo $2^k$: `pow` is at most the width of `T`,
// the vector's dimension is the width of the matrix, and the elements of both are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, v: &UnsignedVector<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
    );
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

// The product `a * v` modulo $2^k$, after checking the arguments. Each element is a wrapping sum of
// wrapping products with all but the lowest $k$ bits cleared, which is correct since $2^k$ divides
// $2^W$.
fn mod_power_of_2_mul_vector<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    v: &UnsignedVector<T>,
    pow: u64,
) -> UnsignedVector<T> {
    assert_arguments(a, v, pow);
    let width = a.width;
    UnsignedVector {
        elements: (0..a.height)
            .map(|i| {
                let mut sum = T::ZERO;
                for (&x, &y) in a.elements[i * width..(i + 1) * width]
                    .iter()
                    .zip(&v.elements)
                {
                    sum.wrapping_add_assign(x.wrapping_mul(y));
                }
                sum.mod_power_of_2(pow)
            })
            .collect(),
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<UnsignedVector<T>> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo $2^k$, taking
    /// the matrix by value and the vector by value.
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
    /// The products and sums are done in wrapping arithmetic, and all but the lowest $k$ bits of
    /// each result are cleared at the end, which is correct since $2^k$ divides $2^W$.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `other` is not the width of
    /// `self`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a).mod_power_of_2_mul(v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: UnsignedVector<T>, pow: u64) -> UnsignedVector<T> {
        mod_power_of_2_mul_vector(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&UnsignedVector<T>> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo $2^k$, taking
    /// the matrix by value and the vector by reference.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `other` is not the width of
    /// `self`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a).mod_power_of_2_mul(&v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &UnsignedVector<T>, pow: u64) -> UnsignedVector<T> {
        mod_power_of_2_mul_vector(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<UnsignedVector<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo $2^k$, taking
    /// the matrix by reference and the vector by value.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `other` is not the width of
    /// `self`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: UnsignedVector<T>, pow: u64) -> UnsignedVector<T> {
        mod_power_of_2_mul_vector(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&UnsignedVector<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedMatrix`] by an [`UnsignedVector`] on the right modulo $2^k$, taking
    /// the matrix by reference and the vector by reference.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the dimension of `other` is not the width of
    /// `self`, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let v = UnsignedVector::<u8>::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_mul(&v, 3).to_string(), "(7, 0)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, other: &UnsignedVector<T>, pow: u64) -> UnsignedVector<T> {
        mod_power_of_2_mul_vector(self, other, pow)
    }
}
