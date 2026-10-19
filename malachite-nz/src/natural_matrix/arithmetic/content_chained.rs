// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::arithmetic::content_chained::content_chained_helper;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::ContentChained;

impl ContentChained<Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalMatrix`], taking the matrix by value
    /// and `x` by value.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!(a.clone().content_chained(Natural::from(4u32)), 2u32);
    /// assert_eq!(a.content_chained(Natural::ZERO), 2u32);
    /// let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.content_chained(Natural::from(6u32)), 6u32);
    /// ```
    #[inline]
    fn content_chained(self, x: Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x)
    }
}

impl ContentChained<&Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalMatrix`], taking the matrix by value
    /// and `x` by reference.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!(a.clone().content_chained(&Natural::from(4u32)), 2u32);
    /// assert_eq!(a.content_chained(&Natural::ZERO), 2u32);
    /// let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.content_chained(&Natural::from(6u32)), 6u32);
    /// ```
    #[inline]
    fn content_chained(self, x: &Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x.clone())
    }
}

impl ContentChained<Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalMatrix`], taking the matrix by
    /// reference and `x` by value.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!((&a).content_chained(Natural::from(4u32)), 2u32);
    /// assert_eq!((&a).content_chained(Natural::ZERO), 2u32);
    /// let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).content_chained(Natural::from(6u32)), 6u32);
    /// ```
    #[inline]
    fn content_chained(self, x: Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x)
    }
}

impl ContentChained<&Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalMatrix`], taking the matrix by
    /// reference and `x` by reference.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!((&a).content_chained(&Natural::from(4u32)), 2u32);
    /// assert_eq!((&a).content_chained(&Natural::ZERO), 2u32);
    /// let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).content_chained(&Natural::from(6u32)), 6u32);
    /// ```
    #[inline]
    fn content_chained(self, x: &Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x.clone())
    }
}
