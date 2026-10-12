// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::integer_vector::arithmetic::dot_general::vec_dot_general;
use malachite_base::num::arithmetic::traits::Dot;

impl Dot<Self> for IntegerVector {
    type Output = Integer;

    /// Computes the dot product of two [`IntegerVector`]s, taking the first by value and the second
    /// by value.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(v.clone().dot(w.clone()), -24);
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .dot(IntegerVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot` from `fmpz_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn dot(mut self, mut other: Self) -> Integer {
        if self.elements.len() == 1 && other.elements.len() == 1 {
            // A single product can reuse the allocation of an owned element.
            self.elements.pop().unwrap() * other.elements.pop().unwrap()
        } else {
            (&self).dot(&other)
        }
    }
}

impl Dot<&Self> for IntegerVector {
    type Output = Integer;

    /// Computes the dot product of two [`IntegerVector`]s, taking the first by value and the second
    /// by reference.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(v.clone().dot(&w), -24);
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .dot(IntegerVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot` from `fmpz_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn dot(mut self, other: &Self) -> Integer {
        if self.elements.len() == 1 && other.elements.len() == 1 {
            // A single product can reuse the allocation of an owned element.
            self.elements.pop().unwrap() * &other.elements[0]
        } else {
            (&self).dot(other)
        }
    }
}

impl Dot<IntegerVector> for &IntegerVector {
    type Output = Integer;

    /// Computes the dot product of two [`IntegerVector`]s, taking the first by reference and the
    /// second by value.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!((&v).dot(w.clone()), -24);
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .dot(IntegerVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot` from `fmpz_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn dot(self, mut other: IntegerVector) -> Integer {
        if self.elements.len() == 1 && other.elements.len() == 1 {
            // A single product can reuse the allocation of an owned element.
            &self.elements[0] * other.elements.pop().unwrap()
        } else {
            self.dot(&other)
        }
    }
}

impl Dot<&IntegerVector> for &IntegerVector {
    type Output = Integer;

    /// Computes the dot product of two [`IntegerVector`]s, taking the first by reference and the
    /// second by reference.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!((&v).dot(&w), -24);
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .dot(IntegerVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot` from `fmpz_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn dot(self, other: &IntegerVector) -> Integer {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "the vectors must have the same dimension"
        );
        vec_dot_general(None, false, &self.elements, &other.elements, false)
    }
}
