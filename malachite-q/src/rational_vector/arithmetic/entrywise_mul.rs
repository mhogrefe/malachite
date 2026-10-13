// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{EntrywiseMul, EntrywiseMulAssign};

fn assert_same_dimension(v: &RationalVector, w: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot take the entrywise product of vectors of different dimensions"
    );
}

impl EntrywiseMul<Self> for RationalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`RationalVector`]s, taking the first by value and the
    /// second by value.
    ///
    /// Every element of the result is the product of the corresponding elements of the two vectors,
    /// so the result has the same dimension as both. This is also called the Hadamard or Schur
    /// product.
    ///
    /// $$
    /// f(v, w) = (v_0w_0, v_1w_1, \ldots, v_{n-1}w_{n-1}).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseMul;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -1/6)").unwrap();
    /// assert_eq!(v.entrywise_mul(w).to_string(), "(2, -2/3, -1/2)");
    /// ```
    #[inline]
    fn entrywise_mul(mut self, other: Self) -> Self {
        self.entrywise_mul_assign(other);
        self
    }
}

impl EntrywiseMul<&Self> for RationalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`RationalVector`]s, taking the first by value and the
    /// second by reference.
    ///
    /// See the documentation for the [`EntrywiseMul`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseMul;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -1/6)").unwrap();
    /// assert_eq!(v.entrywise_mul(&w).to_string(), "(2, -2/3, -1/2)");
    /// ```
    #[inline]
    fn entrywise_mul(mut self, other: &Self) -> Self {
        self.entrywise_mul_assign(other);
        self
    }
}

impl EntrywiseMul<RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Takes the entrywise product of two [`RationalVector`]s, taking the first by reference and
    /// the second by value.
    ///
    /// See the documentation for the [`EntrywiseMul`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseMul;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -1/6)").unwrap();
    /// assert_eq!((&v).entrywise_mul(w).to_string(), "(2, -2/3, -1/2)");
    /// ```
    #[inline]
    fn entrywise_mul(self, mut other: RationalVector) -> RationalVector {
        // The product is commutative, so the owned vector can hold the result.
        other.entrywise_mul_assign(self);
        other
    }
}

impl EntrywiseMul<&RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Takes the entrywise product of two [`RationalVector`]s, taking the first by reference and
    /// the second by reference.
    ///
    /// See the documentation for the [`EntrywiseMul`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseMul;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -1/6)").unwrap();
    /// assert_eq!((&v).entrywise_mul(&w).to_string(), "(2, -2/3, -1/2)");
    /// ```
    fn entrywise_mul(self, other: &RationalVector) -> RationalVector {
        assert_same_dimension(self, other);
        RationalVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x * y)
                .collect(),
        }
    }
}

impl EntrywiseMulAssign<Self> for RationalVector {
    /// Replaces a [`RationalVector`] by its entrywise product with another [`RationalVector`],
    /// taking the other by value.
    ///
    /// Every element is multiplied by the corresponding element of `other`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseMulAssign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -1/6)").unwrap();
    /// v.entrywise_mul_assign(w);
    /// assert_eq!(v.to_string(), "(2, -2/3, -1/2)");
    /// ```
    fn entrywise_mul_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x *= y;
        }
    }
}

impl EntrywiseMulAssign<&Self> for RationalVector {
    /// Replaces a [`RationalVector`] by its entrywise product with another [`RationalVector`],
    /// taking the other by reference.
    ///
    /// Every element is multiplied by the corresponding element of `other`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseMulAssign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -1/6)").unwrap();
    /// v.entrywise_mul_assign(&w);
    /// assert_eq!(v.to_string(), "(2, -2/3, -1/2)");
    /// ```
    fn entrywise_mul_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x *= y;
        }
    }
}
