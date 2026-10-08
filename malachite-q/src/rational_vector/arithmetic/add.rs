// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use core::ops::{Add, AddAssign};

fn assert_same_dimension(v: &RationalVector, w: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add vectors of different dimensions"
    );
}

impl Add<Self> for RationalVector {
    type Output = Self;

    /// Adds two [`RationalVector`]s, taking both by value.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!((v + w).to_string(), "(5/6, 3, 0)");
    /// ```
    #[inline]
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Add<&Self> for RationalVector {
    type Output = Self;

    /// Adds two [`RationalVector`]s, taking the first by value and the second by reference.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!((v + &w).to_string(), "(5/6, 3, 0)");
    /// ```
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Adds two [`RationalVector`]s, taking the first by reference and the second by value.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!((&v + w).to_string(), "(5/6, 3, 0)");
    /// ```
    #[inline]
    fn add(self, mut other: RationalVector) -> RationalVector {
        other += self;
        other
    }
}

impl Add<&RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Adds two [`RationalVector`]s, taking both by reference.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!((&v + &w).to_string(), "(5/6, 3, 0)");
    /// ```
    fn add(self, other: &RationalVector) -> RationalVector {
        assert_same_dimension(self, other);
        RationalVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}

impl AddAssign<Self> for RationalVector {
    /// Adds a [`RationalVector`] to a [`RationalVector`] in place, taking the [`RationalVector`] on
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// v += RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!(v.to_string(), "(5/6, 3, 0)");
    /// ```
    fn add_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x += y;
        }
    }
}

impl AddAssign<&Self> for RationalVector {
    /// Adds a [`RationalVector`] to a [`RationalVector`] in place, taking the [`RationalVector`] on
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// v += &RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!(v.to_string(), "(5/6, 3, 0)");
    /// ```
    fn add_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x += y;
        }
    }
}
