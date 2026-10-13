// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{EntrywiseMul, EntrywiseMulAssign};

fn assert_same_dimension(v: &NaturalVector, w: &NaturalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot take the entrywise product of vectors of different dimensions"
    );
}

impl EntrywiseMul<Self> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s, taking the first by value and the
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(v.entrywise_mul(w).to_string(), "(4, 10, 18)");
    /// ```
    #[inline]
    fn entrywise_mul(mut self, other: Self) -> Self {
        self.entrywise_mul_assign(other);
        self
    }
}

impl EntrywiseMul<&Self> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s, taking the first by value and the
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(v.entrywise_mul(&w).to_string(), "(4, 10, 18)");
    /// ```
    #[inline]
    fn entrywise_mul(mut self, other: &Self) -> Self {
        self.entrywise_mul_assign(other);
        self
    }
}

impl EntrywiseMul<NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s, taking the first by reference and the
    /// second by value.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!((&v).entrywise_mul(w).to_string(), "(4, 10, 18)");
    /// ```
    #[inline]
    fn entrywise_mul(self, mut other: NaturalVector) -> NaturalVector {
        // The product is commutative, so the owned vector can hold the result.
        other.entrywise_mul_assign(self);
        other
    }
}

impl EntrywiseMul<&NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s, taking the first by reference and the
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!((&v).entrywise_mul(&w).to_string(), "(4, 10, 18)");
    /// ```
    fn entrywise_mul(self, other: &NaturalVector) -> NaturalVector {
        assert_same_dimension(self, other);
        NaturalVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x * y)
                .collect(),
        }
    }
}

impl EntrywiseMulAssign<Self> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`], taking
    /// the other by value.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// v.entrywise_mul_assign(w);
    /// assert_eq!(v.to_string(), "(4, 10, 18)");
    /// ```
    fn entrywise_mul_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x *= y;
        }
    }
}

impl EntrywiseMulAssign<&Self> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`], taking
    /// the other by reference.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// v.entrywise_mul_assign(&w);
    /// assert_eq!(v.to_string(), "(4, 10, 18)");
    /// ```
    fn entrywise_mul_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x *= y;
        }
    }
}
