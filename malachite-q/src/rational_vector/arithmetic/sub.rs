// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use core::ops::{Sub, SubAssign};

fn assert_same_dimension(v: &RationalVector, w: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot subtract vectors of different dimensions"
    );
}

impl Sub<Self> for RationalVector {
    type Output = Self;

    /// Subtracts a [`RationalVector`] from another [`RationalVector`], taking both by value.
    ///
    /// The difference is taken element by element, so the result has the same dimension as the
    /// operands.
    ///
    /// $$
    /// f(v, w) = v - w.
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
    /// assert_eq!((v - w).to_string(), "(1/6, -7, 6)");
    /// ```
    #[inline]
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl Sub<&Self> for RationalVector {
    type Output = Self;

    /// Subtracts a [`RationalVector`] from another [`RationalVector`], taking the first by value
    /// and the second by reference.
    ///
    /// The difference is taken element by element, so the result has the same dimension as the
    /// operands.
    ///
    /// $$
    /// f(v, w) = v - w.
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
    /// assert_eq!((v - &w).to_string(), "(1/6, -7, 6)");
    /// ```
    #[inline]
    fn sub(mut self, other: &Self) -> Self {
        self -= other;
        self
    }
}

impl Sub<RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Subtracts a [`RationalVector`] from another [`RationalVector`], taking the first by
    /// reference and the second by value.
    ///
    /// The difference is taken element by element, so the result has the same dimension as the
    /// operands.
    ///
    /// $$
    /// f(v, w) = v - w.
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
    /// assert_eq!((&v - w).to_string(), "(1/6, -7, 6)");
    /// ```
    #[inline]
    fn sub(self, mut other: RationalVector) -> RationalVector {
        other -= self;
        -other
    }
}

impl Sub<&RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Subtracts a [`RationalVector`] from another [`RationalVector`], taking both by reference.
    ///
    /// The difference is taken element by element, so the result has the same dimension as the
    /// operands.
    ///
    /// $$
    /// f(v, w) = v - w.
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
    /// assert_eq!((&v - &w).to_string(), "(1/6, -7, 6)");
    /// ```
    fn sub(self, other: &RationalVector) -> RationalVector {
        assert_same_dimension(self, other);
        RationalVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x - y)
                .collect(),
        }
    }
}

impl SubAssign<Self> for RationalVector {
    /// Subtracts a [`RationalVector`] from a [`RationalVector`] in place, taking the
    /// [`RationalVector`] on the right-hand side by value.
    ///
    /// The difference is taken element by element, so the result has the same dimension as the
    /// operands.
    ///
    /// $$
    /// f(v, w) = v - w.
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
    /// v -= RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!(v.to_string(), "(1/6, -7, 6)");
    /// ```
    fn sub_assign(&mut self, other: Self) {
        assert_same_dimension(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x -= y;
        }
    }
}

impl SubAssign<&Self> for RationalVector {
    /// Subtracts a [`RationalVector`] from a [`RationalVector`] in place, taking the
    /// [`RationalVector`] on the right-hand side by reference.
    ///
    /// The difference is taken element by element, so the result has the same dimension as the
    /// operands.
    ///
    /// $$
    /// f(v, w) = v - w.
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
    /// v -= &RationalVector::from_str("(1/3, 5, -3)").unwrap();
    /// assert_eq!(v.to_string(), "(1/6, -7, 6)");
    /// ```
    fn sub_assign(&mut self, other: &Self) {
        assert_same_dimension(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x -= y;
        }
    }
}
