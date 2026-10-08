// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use core::ops::{Add, AddAssign};

fn assert_same_dimension(v: &NaturalVector, w: &NaturalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add vectors of different dimensions"
    );
}

impl Add<Self> for NaturalVector {
    type Output = Self;

    /// Adds two [`NaturalVector`]s, taking both by value.
    ///
    /// The sum is taken element by element, so the result has the same dimension as the summands.
    ///
    /// $$
    /// f(v, w) = v + w.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(10, 0, 5)").unwrap();
    /// assert_eq!((v + w).to_string(), "(11, 2, 8)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Add<&Self> for NaturalVector {
    type Output = Self;

    /// Adds two [`NaturalVector`]s, taking the first by value and the second by reference.
    ///
    /// The sum is taken element by element, so the result has the same dimension as the summands.
    ///
    /// $$
    /// f(v, w) = v + w.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(10, 0, 5)").unwrap();
    /// assert_eq!((v + &w).to_string(), "(11, 2, 8)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Adds two [`NaturalVector`]s, taking the first by reference and the second by value.
    ///
    /// The sum is taken element by element, so the result has the same dimension as the summands.
    ///
    /// $$
    /// f(v, w) = v + w.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(10, 0, 5)").unwrap();
    /// assert_eq!((&v + w).to_string(), "(11, 2, 8)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(self, mut other: NaturalVector) -> NaturalVector {
        other += self;
        other
    }
}

impl Add<&NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Adds two [`NaturalVector`]s, taking both by reference.
    ///
    /// The sum is taken element by element, so the result has the same dimension as the summands.
    ///
    /// $$
    /// f(v, w) = v + w.
    /// $$
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(10, 0, 5)").unwrap();
    /// assert_eq!((&v + &w).to_string(), "(11, 2, 8)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add(self, other: &NaturalVector) -> NaturalVector {
        assert_same_dimension(self, other);
        NaturalVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}

impl AddAssign<Self> for NaturalVector {
    /// Adds a [`NaturalVector`] to a [`NaturalVector`] in place, taking the [`NaturalVector`] on
    /// the right-hand side by value.
    ///
    /// The sum is taken element by element, so the result has the same dimension as the summands.
    ///
    /// $$
    /// f(v, w) = v + w.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v += NaturalVector::from_str("(10, 0, 5)").unwrap();
    /// assert_eq!(v.to_string(), "(11, 2, 8)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x += y;
        }
    }
}

impl AddAssign<&Self> for NaturalVector {
    /// Adds a [`NaturalVector`] to a [`NaturalVector`] in place, taking the [`NaturalVector`] on
    /// the right-hand side by reference.
    ///
    /// The sum is taken element by element, so the result has the same dimension as the summands.
    ///
    /// $$
    /// f(v, w) = v + w.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v += &NaturalVector::from_str("(10, 0, 5)").unwrap();
    /// assert_eq!(v.to_string(), "(11, 2, 8)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x += y;
        }
    }
}
