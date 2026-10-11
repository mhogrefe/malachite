// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use core::iter::Sum;

const EMPTY: &str = "cannot sum an empty iterator of vectors: the dimension is unknown";

impl Sum for IntegerVector {
    /// Adds up all the [`IntegerVector`]s in an iterator, entry by entry.
    ///
    /// The vectors must all have the same dimension, which is also the dimension of the sum. Since
    /// the dimension of the zero vector cannot be inferred from an empty iterator, an empty
    /// iterator panics rather than summing to zero.
    ///
    /// $$
    /// f((v_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of all the vectors.
    ///
    /// # Panics
    /// Panics if the iterator is empty or if the vectors do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let vs = ["(1, -2, 3)", "(-4, 5, -6)", "(7, 8, -9)"]
    ///     .map(|s| IntegerVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     IntegerVector::sum(vs.into_iter()).to_string(),
    ///     "(4, 11, -12)"
    /// );
    ///
    /// let vs = [
    ///     "(18446744073709551615, -1)",
    ///     "(-18446744073709551616, 1267650600228229401496703205376)",
    /// ]
    /// .map(|s| IntegerVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     IntegerVector::sum(vs.into_iter()).to_string(),
    ///     "(-1, 1267650600228229401496703205375)"
    /// );
    /// ```
    fn sum<I>(mut xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut sum = xs.next().expect(EMPTY);
        for v in xs {
            sum += v;
        }
        sum
    }
}

impl<'a> Sum<&'a Self> for IntegerVector {
    /// Adds up all the [`IntegerVector`]s in an iterator of references, entry by entry.
    ///
    /// The vectors must all have the same dimension, which is also the dimension of the sum. Since
    /// the dimension of the zero vector cannot be inferred from an empty iterator, an empty
    /// iterator panics rather than summing to zero.
    ///
    /// $$
    /// f((v_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of all the vectors.
    ///
    /// # Panics
    /// Panics if the iterator is empty or if the vectors do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let vs = ["(1, -2, 3)", "(-4, 5, -6)", "(7, 8, -9)"]
    ///     .map(|s| IntegerVector::from_str(s).unwrap());
    /// assert_eq!(IntegerVector::sum(vs.iter()).to_string(), "(4, 11, -12)");
    ///
    /// let vs = [
    ///     "(18446744073709551615, -1)",
    ///     "(-18446744073709551616, 1267650600228229401496703205376)",
    /// ]
    /// .map(|s| IntegerVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     IntegerVector::sum(vs.iter()).to_string(),
    ///     "(-1, 1267650600228229401496703205375)"
    /// );
    /// ```
    fn sum<I>(mut xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let mut sum = xs.next().expect(EMPTY).clone();
        for v in xs {
            sum += v;
        }
        sum
    }
}
