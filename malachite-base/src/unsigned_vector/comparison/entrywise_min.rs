// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{EntrywiseMin, EntrywiseMinAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::cmp::min;

fn assert_same_dimension<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot take the entrywise minimum of vectors of different dimensions"
    );
}

impl<T: PrimitiveUnsigned> EntrywiseMin<Self> for UnsignedVector<T> {
    type Output = Self;

    /// Takes the entrywise minimum of two [`UnsignedVector`]s, taking the first by value and the
    /// second by value.
    ///
    /// Every element of the result is the smaller of the corresponding elements of the two vectors,
    /// so the result has the same dimension as both.
    ///
    /// $$
    /// f(v, w) = (\min(v_0, w_0), \min(v_1, w_1), \ldots, \min(v_{n-1}, w_{n-1})).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMin;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 5, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 2, 3)").unwrap();
    /// assert_eq!(v.entrywise_min(w).to_string(), "(1, 2, 3)");
    /// ```
    #[inline]
    fn entrywise_min(mut self, other: Self) -> Self {
        self.entrywise_min_assign(other);
        self
    }
}

impl<T: PrimitiveUnsigned> EntrywiseMin<&Self> for UnsignedVector<T> {
    type Output = Self;

    /// Takes the entrywise minimum of two [`UnsignedVector`]s, taking the first by value and the
    /// second by reference.
    ///
    /// See the documentation for the [`EntrywiseMin`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMin;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 5, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 2, 3)").unwrap();
    /// assert_eq!(v.entrywise_min(&w).to_string(), "(1, 2, 3)");
    /// ```
    #[inline]
    fn entrywise_min(mut self, other: &Self) -> Self {
        self.entrywise_min_assign(other);
        self
    }
}

impl<T: PrimitiveUnsigned> EntrywiseMin<UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Takes the entrywise minimum of two [`UnsignedVector`]s, taking the first by reference and
    /// the second by value.
    ///
    /// See the documentation for the [`EntrywiseMin`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMin;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 5, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 2, 3)").unwrap();
    /// assert_eq!((&v).entrywise_min(w).to_string(), "(1, 2, 3)");
    /// ```
    #[inline]
    fn entrywise_min(self, mut other: UnsignedVector<T>) -> UnsignedVector<T> {
        // The minimum is symmetric, so the owned vector can hold the result.
        other.entrywise_min_assign(self);
        other
    }
}

impl<T: PrimitiveUnsigned> EntrywiseMin<&UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Takes the entrywise minimum of two [`UnsignedVector`]s, taking the first by reference and
    /// the second by reference.
    ///
    /// See the documentation for the [`EntrywiseMin`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMin;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 5, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 2, 3)").unwrap();
    /// assert_eq!((&v).entrywise_min(&w).to_string(), "(1, 2, 3)");
    /// ```
    fn entrywise_min(self, other: &UnsignedVector<T>) -> UnsignedVector<T> {
        assert_same_dimension(self, other);
        UnsignedVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| min(x, y))
                .collect(),
        }
    }
}

impl<T: PrimitiveUnsigned> EntrywiseMinAssign<Self> for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] by its entrywise minimum with another [`UnsignedVector`],
    /// taking the other by value.
    ///
    /// Every element becomes the smaller of itself and the corresponding element of `other`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMinAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 5, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 2, 3)").unwrap();
    /// v.entrywise_min_assign(w);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    fn entrywise_min_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            if y < *x {
                *x = y;
            }
        }
    }
}

impl<T: PrimitiveUnsigned> EntrywiseMinAssign<&Self> for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] by its entrywise minimum with another [`UnsignedVector`],
    /// taking the other by reference.
    ///
    /// Every element becomes the smaller of itself and the corresponding element of `other`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMinAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 5, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 2, 3)").unwrap();
    /// v.entrywise_min_assign(&w);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    fn entrywise_min_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, &y) in self.elements.iter_mut().zip(&other.elements) {
            if y < *x {
                *x = y;
            }
        }
    }
}
