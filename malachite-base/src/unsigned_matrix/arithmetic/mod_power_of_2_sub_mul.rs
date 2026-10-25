// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2SubMul, ModPowerOf2SubMulAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

fn assert_reduced<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    c: T,
    pow: u64,
) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert_eq!(
        (a.height, a.width),
        (b.height, b.width),
        "cannot sub-multiply matrices of different shapes"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {a} has an element >= 2^{pow}"
    );
    assert!(
        b.mod_power_of_2_is_reduced(pow),
        "b must be reduced mod 2^pow, but {b} has an element >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubMul<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo
    /// $2^k$, taking both matrices by value. The elements and the scalar must already be reduced
    /// modulo $2^k$.
    ///
    /// The operation is taken element by element, so the result has the same shape as the matrices.
    ///
    /// $$
    /// f(A, B, c, k) = (A - cB) \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `b` have different shapes, or if
    /// any element of `self` or `b`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 7, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_sub_mul(b, 3, 3).to_string(), "[7, 4, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul(mut self, b: Self, c: T, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_assign(&b, c, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubMul<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo
    /// $2^k$, taking the first matrix by value and the second by reference. The elements and the
    /// scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMul`] implementation that takes both matrices
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `b` have different shapes, or if
    /// any element of `self` or `b`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 7, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_sub_mul(&b, 3, 3).to_string(), "[7, 4, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul(mut self, b: &Self, c: T, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_assign(b, c, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubMul<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo
    /// $2^k$, taking both matrices by reference. The elements and the scalar must already be
    /// reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMul`] implementation that takes both matrices
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `b` have different shapes, or if
    /// any element of `self` or `b`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 7, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_sub_mul(&b, 3, 3).to_string(),
    ///     "[7, 4, 7]"
    /// );
    /// ```
    fn mod_power_of_2_sub_mul(self, b: &UnsignedMatrix<T>, c: T, pow: u64) -> UnsignedMatrix<T> {
        assert_reduced(self, b, c, pow);
        UnsignedMatrix {
            height: self.height,
            width: self.width,
            elements: self
                .elements
                .iter()
                .zip(&b.elements)
                .map(|(&x, &y)| x.mod_power_of_2_sub_mul(y, c, pow))
                .collect(),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubMulAssign<Self, T> for UnsignedMatrix<T> {
    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo
    /// $2^k$, in place, taking the matrix on the right-hand side by value. The elements and the
    /// scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMul`] implementation that takes both matrices
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `b` have different shapes, or if
    /// any element of `self` or `b`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 7, 4]").unwrap();
    /// a.mod_power_of_2_sub_mul_assign(b, 3, 3);
    /// assert_eq!(a.to_string(), "[7, 4, 7]");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_assign(&mut self, b: Self, c: T, pow: u64) {
        self.mod_power_of_2_sub_mul_assign(&b, c, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubMulAssign<&Self, T> for UnsignedMatrix<T> {
    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo
    /// $2^k$, in place, taking the matrix on the right-hand side by reference. The elements and the
    /// scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMul`] implementation that takes both matrices
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `b` have different shapes, or if
    /// any element of `self` or `b`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 7, 4]").unwrap();
    /// a.mod_power_of_2_sub_mul_assign(&b, 3, 3);
    /// assert_eq!(a.to_string(), "[7, 4, 7]");
    /// ```
    fn mod_power_of_2_sub_mul_assign(&mut self, b: &Self, c: T, pow: u64) {
        assert_reduced(self, b, c, pow);
        if c == T::ZERO {
            return;
        }
        for (x, &y) in self.elements.iter_mut().zip(&b.elements) {
            x.mod_power_of_2_sub_mul_assign(y, c, pow);
        }
    }
}
