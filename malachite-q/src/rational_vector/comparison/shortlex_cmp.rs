// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::{ShortlexRationalVector, ShortlexRationalVectorRef};
use core::cmp::Ordering;

impl Ord for ShortlexRationalVectorRef<'_> {
    /// Compares two [`ShortlexRationalVectorRef`]s.
    ///
    /// The order is shortlex: the vectors are compared first by dimension, and then, in case of a
    /// tie, lexicographically, by their elements from first to last. The 0-dimensional vector comes
    /// first. This is a total order, and its equality agrees with
    /// [`RationalVector`](crate::rational_vector::RationalVector) equality.
    ///
    /// Where the dimensions differ this parts company with the lexicographic order of [`Vec`]s:
    /// here dimension decides outright, so $(5) < (0, 0)$, where lexicographic order has $(5) > (0,
    /// 0)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two vectors'
    /// total number of bits, summed over their elements. Vectors of different dimensions are
    /// compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::{RationalVector, ShortlexRationalVectorRef};
    ///
    /// let empty = RationalVector::from_str("()").unwrap();
    /// let five = RationalVector::from_str("(5)").unwrap();
    /// let zero_zero = RationalVector::from_str("(0, 0)").unwrap();
    /// let zero_one = RationalVector::from_str("(0, 1)").unwrap();
    ///
    /// // The 0-dimensional vector comes first.
    /// assert!(ShortlexRationalVectorRef(&empty) < ShortlexRationalVectorRef(&five));
    /// // Dimension decides, whatever the elements.
    /// assert!(ShortlexRationalVectorRef(&five) < ShortlexRationalVectorRef(&zero_zero));
    /// // At equal dimensions, the first differing element decides.
    /// assert!(ShortlexRationalVectorRef(&zero_zero) < ShortlexRationalVectorRef(&zero_one));
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Once the lengths agree, comparing the `Vec`s is the comparison at the first element where
        // the two differ.
        self.0
            .elements
            .len()
            .cmp(&other.0.elements.len())
            .then_with(|| self.0.elements.cmp(&other.0.elements))
    }
}

impl PartialOrd for ShortlexRationalVectorRef<'_> {
    /// Compares two [`ShortlexRationalVectorRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &ShortlexRationalVectorRef) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortlexRationalVector {
    /// Compares two [`ShortlexRationalVector`]s.
    ///
    /// The order is shortlex: the vectors are compared first by dimension, and then, in case of a
    /// tie, lexicographically. See the [`Ord`] implementation for [`ShortlexRationalVectorRef`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two vectors'
    /// total number of bits, summed over their elements. Vectors of different dimensions are
    /// compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::{RationalVector, ShortlexRationalVector};
    /// use std::collections::BTreeSet;
    ///
    /// let five = RationalVector::from_str("(5)").unwrap();
    /// let zero_zero = RationalVector::from_str("(0, 0)").unwrap();
    ///
    /// // Dimension decides, whatever the elements.
    /// assert!(ShortlexRationalVector(five.clone()) < ShortlexRationalVector(zero_zero.clone()));
    ///
    /// // The wrapper makes vectors usable in ordered collections.
    /// let set: BTreeSet<_> = [zero_zero, five]
    ///     .into_iter()
    ///     .map(ShortlexRationalVector)
    ///     .collect();
    /// assert_eq!(
    ///     set.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
    ///     ["(5)", "(0, 0)"]
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl PartialOrd for ShortlexRationalVector {
    /// Compares two [`ShortlexRationalVector`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
