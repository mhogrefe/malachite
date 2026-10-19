// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ContentChained;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::arithmetic::content_chained::content_chained;

impl<T: PrimitiveUnsigned> ContentChained<T> for UnsignedMatrix<T> {
    type Output = T;

    /// Computes the GCD of `x` and the elements of an [`UnsignedMatrix`], taking the matrix by
    /// value.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!(a.clone().content_chained(4), 2);
    /// assert_eq!(a.content_chained(0), 2);
    /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.content_chained(6), 6);
    /// ```
    #[inline]
    fn content_chained(self, x: T) -> T {
        content_chained(&self.elements, x)
    }
}

impl<T: PrimitiveUnsigned> ContentChained<T> for &UnsignedMatrix<T> {
    type Output = T;

    /// Computes the GCD of `x` and the elements of an [`UnsignedMatrix`], taking the matrix by
    /// reference.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!((&a).content_chained(4), 2);
    /// assert_eq!((&a).content_chained(0), 2);
    /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).content_chained(6), 6);
    /// ```
    #[inline]
    fn content_chained(self, x: T) -> T {
        content_chained(&self.elements, x)
    }
}
