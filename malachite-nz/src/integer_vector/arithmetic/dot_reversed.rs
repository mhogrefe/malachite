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
use malachite_base::num::arithmetic::traits::DotReversed;

// The dot product of `v` and the reverse of `w`.
fn dot_reversed(v: &IntegerVector, w: &IntegerVector) -> Integer {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    vec_dot_general(None, false, &v.elements, &w.elements, true)
}

impl DotReversed<Self> for IntegerVector {
    type Output = Integer;

    /// Computes the dot product of a [`IntegerVector`] and the reverse of another, taking the first
    /// by value and the second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(v.clone().dot_reversed(w.clone()), -4i32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with a
    /// null `initial`, `subtract` set to 0, and `reverse` set to 1.
    #[inline]
    fn dot_reversed(self, other: Self) -> Integer {
        dot_reversed(&self, &other)
    }
}

impl DotReversed<&Self> for IntegerVector {
    type Output = Integer;

    /// Computes the dot product of a [`IntegerVector`] and the reverse of another, taking the first
    /// by value and the second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(v.clone().dot_reversed(&w), -4i32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with a
    /// null `initial`, `subtract` set to 0, and `reverse` set to 1.
    #[inline]
    fn dot_reversed(self, other: &Self) -> Integer {
        dot_reversed(&self, other)
    }
}

impl DotReversed<IntegerVector> for &IntegerVector {
    type Output = Integer;

    /// Computes the dot product of a [`IntegerVector`] and the reverse of another, taking the first
    /// by reference and the second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!((&v).dot_reversed(w.clone()), -4i32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with a
    /// null `initial`, `subtract` set to 0, and `reverse` set to 1.
    #[inline]
    fn dot_reversed(self, other: IntegerVector) -> Integer {
        dot_reversed(self, &other)
    }
}

impl DotReversed<&IntegerVector> for &IntegerVector {
    type Output = Integer;

    /// Computes the dot product of a [`IntegerVector`] and the reverse of another, taking the first
    /// by reference and the second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!((&v).dot_reversed(&w), -4i32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with a
    /// null `initial`, `subtract` set to 0, and `reverse` set to 1.
    #[inline]
    fn dot_reversed(self, other: &IntegerVector) -> Integer {
        dot_reversed(self, other)
    }
}
