// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{EntrywiseMax, EntrywiseMaxAssign};

fn assert_same_dimension(v: &IntegerVector, w: &IntegerVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot take the entrywise maximum of vectors of different dimensions"
    );
}

impl EntrywiseMax<Self> for IntegerVector {
    type Output = Self;

    /// Takes the entrywise maximum of two [`IntegerVector`]s, taking the first by value and the
    /// second by value.
    ///
    /// Every element of the result is the larger of the corresponding elements of the two vectors,
    /// so the result has the same dimension as both.
    ///
    /// $$
    /// f(v, w) = (\max(v_0, w_0), \max(v_1, w_1), \ldots, \max(v_{n-1}, w_{n-1})).
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMax;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -5, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-4, 2, 3)").unwrap();
    /// assert_eq!(v.entrywise_max(w).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max` from `fmpz_vec/max.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_max(mut self, other: Self) -> Self {
        self.entrywise_max_assign(other);
        self
    }
}

impl EntrywiseMax<&Self> for IntegerVector {
    type Output = Self;

    /// Takes the entrywise maximum of two [`IntegerVector`]s, taking the first by value and the
    /// second by reference.
    ///
    /// See the documentation for the [`EntrywiseMax`] implementation that takes both vectors by
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMax;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -5, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-4, 2, 3)").unwrap();
    /// assert_eq!(v.entrywise_max(&w).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max` from `fmpz_vec/max.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_max(mut self, other: &Self) -> Self {
        self.entrywise_max_assign(other);
        self
    }
}

impl EntrywiseMax<IntegerVector> for &IntegerVector {
    type Output = IntegerVector;

    /// Takes the entrywise maximum of two [`IntegerVector`]s, taking the first by reference and the
    /// second by value.
    ///
    /// See the documentation for the [`EntrywiseMax`] implementation that takes both vectors by
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMax;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -5, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-4, 2, 3)").unwrap();
    /// assert_eq!((&v).entrywise_max(w).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max` from `fmpz_vec/max.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_max(self, mut other: IntegerVector) -> IntegerVector {
        // The maximum is symmetric, so the owned vector can hold the result.
        other.entrywise_max_assign(self);
        other
    }
}

impl EntrywiseMax<&IntegerVector> for &IntegerVector {
    type Output = IntegerVector;

    /// Takes the entrywise maximum of two [`IntegerVector`]s, taking the first by reference and the
    /// second by reference.
    ///
    /// See the documentation for the [`EntrywiseMax`] implementation that takes both vectors by
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMax;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -5, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-4, 2, 3)").unwrap();
    /// assert_eq!((&v).entrywise_max(&w).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max` from `fmpz_vec/max.c`, FLINT 3.6.0.
    fn entrywise_max(self, other: &IntegerVector) -> IntegerVector {
        assert_same_dimension(self, other);
        IntegerVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| max(x, y).clone())
                .collect(),
        }
    }
}

impl EntrywiseMaxAssign<Self> for IntegerVector {
    /// Replaces an [`IntegerVector`] by its entrywise maximum with another [`IntegerVector`],
    /// taking the other by value.
    ///
    /// Every element becomes the larger of itself and the corresponding element of `other`.
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMaxAssign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -5, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-4, 2, 3)").unwrap();
    /// v.entrywise_max_assign(w);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max_inplace` from `fmpz_vec/max.c`, FLINT 3.6.0.
    fn entrywise_max_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            if y > *x {
                *x = y;
            }
        }
    }
}

impl EntrywiseMaxAssign<&Self> for IntegerVector {
    /// Replaces an [`IntegerVector`] by its entrywise maximum with another [`IntegerVector`],
    /// taking the other by reference.
    ///
    /// Every element becomes the larger of itself and the corresponding element of `other`.
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
    /// use malachite_base::num::arithmetic::traits::EntrywiseMaxAssign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -5, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-4, 2, 3)").unwrap();
    /// v.entrywise_max_assign(&w);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max_inplace` from `fmpz_vec/max.c`, FLINT 3.6.0.
    fn entrywise_max_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            if y > x {
                x.clone_from(y);
            }
        }
    }
}
