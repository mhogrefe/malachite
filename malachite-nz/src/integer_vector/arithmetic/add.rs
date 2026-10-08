// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::IntegerVector;
use core::borrow::Borrow;
use core::ops::{Add, AddAssign};

// Sets each element of `out` to the sum of the elements of `xs` and `ys` at the same index.
//
// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0, with the output
// separate from the inputs.
pub(crate) fn vec_add<C: PolynomialCoefficient, T: Borrow<C>>(out: &mut [C], xs: &[T], ys: &[T]) {
    for ((o, x), y) in out.iter_mut().zip(xs).zip(ys) {
        *o = x.borrow().add_ref(y.borrow());
    }
}

// Adds each element of `ys` to the element of `xs` at the same index.
//
// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0, with the output the
// same as the first input.
pub(crate) fn vec_add_assign<C: PolynomialCoefficient>(xs: &mut [C], ys: &[C]) {
    for (x, y) in xs.iter_mut().zip(ys) {
        *x += y;
    }
}

fn assert_same_dimension(v: &IntegerVector, w: &IntegerVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add vectors of different dimensions"
    );
}

impl Add<Self> for IntegerVector {
    type Output = Self;

    /// Adds two [`IntegerVector`]s, taking both by value.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-1, 5, 0)").unwrap();
    /// assert_eq!((v + w).to_string(), "(0, 3, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Add<&Self> for IntegerVector {
    type Output = Self;

    /// Adds two [`IntegerVector`]s, taking the first by value and the second by reference.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-1, 5, 0)").unwrap();
    /// assert_eq!((v + &w).to_string(), "(0, 3, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<IntegerVector> for &IntegerVector {
    type Output = IntegerVector;

    /// Adds two [`IntegerVector`]s, taking the first by reference and the second by value.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-1, 5, 0)").unwrap();
    /// assert_eq!((&v + w).to_string(), "(0, 3, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(self, mut other: IntegerVector) -> IntegerVector {
        other += self;
        other
    }
}

impl Add<&IntegerVector> for &IntegerVector {
    type Output = IntegerVector;

    /// Adds two [`IntegerVector`]s, taking both by reference.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(-1, 5, 0)").unwrap();
    /// assert_eq!((&v + &w).to_string(), "(0, 3, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add(self, other: &IntegerVector) -> IntegerVector {
        assert_same_dimension(self, other);
        IntegerVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}

impl AddAssign<Self> for IntegerVector {
    /// Adds an [`IntegerVector`] to an [`IntegerVector`] in place, taking the [`IntegerVector`] on
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// v += IntegerVector::from_str("(-1, 5, 0)").unwrap();
    /// assert_eq!(v.to_string(), "(0, 3, 3)");
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

impl AddAssign<&Self> for IntegerVector {
    /// Adds an [`IntegerVector`] to an [`IntegerVector`] in place, taking the [`IntegerVector`] on
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// v += &IntegerVector::from_str("(-1, 5, 0)").unwrap();
    /// assert_eq!(v.to_string(), "(0, 3, 3)");
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
