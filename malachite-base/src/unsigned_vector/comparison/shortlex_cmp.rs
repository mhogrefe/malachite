// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::{ShortlexUnsignedVector, ShortlexUnsignedVectorRef};
use core::cmp::Ordering;

impl<T: PrimitiveUnsigned> Ord for ShortlexUnsignedVectorRef<'_, T> {
    /// Compares two [`ShortlexUnsignedVectorRef`]s.
    ///
    /// The order is shortlex: the vectors are compared first by dimension, and then, in case of a
    /// tie, lexicographically, by their elements from first to last. The 0-dimensional vector comes
    /// first. This is a total order, and its equality agrees with
    /// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector) equality.
    ///
    /// Where the dimensions differ this parts company with the lexicographic order of [`Vec`]s:
    /// here dimension decides outright, so $(5) < (0, 0)$, where lexicographic order has $(5) > (0,
    /// 0)$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::{ShortlexUnsignedVectorRef, UnsignedVector};
    ///
    /// let empty = UnsignedVector::<u32>::from_str("()").unwrap();
    /// let five = UnsignedVector::<u32>::from_str("(5)").unwrap();
    /// let zero_zero = UnsignedVector::<u32>::from_str("(0, 0)").unwrap();
    /// let zero_one = UnsignedVector::<u32>::from_str("(0, 1)").unwrap();
    ///
    /// // The 0-dimensional vector comes first.
    /// assert!(ShortlexUnsignedVectorRef(&empty) < ShortlexUnsignedVectorRef(&five));
    /// // Dimension decides, whatever the elements.
    /// assert!(ShortlexUnsignedVectorRef(&five) < ShortlexUnsignedVectorRef(&zero_zero));
    /// // At equal dimensions, the first differing element decides.
    /// assert!(ShortlexUnsignedVectorRef(&zero_zero) < ShortlexUnsignedVectorRef(&zero_one));
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

impl<T: PrimitiveUnsigned> PartialOrd for ShortlexUnsignedVectorRef<'_, T> {
    /// Compares two [`ShortlexUnsignedVectorRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PrimitiveUnsigned> Ord for ShortlexUnsignedVector<T> {
    /// Compares two [`ShortlexUnsignedVector`]s.
    ///
    /// The order is shortlex: the vectors are compared first by dimension, and then, in case of a
    /// tie, lexicographically. See the [`Ord`] implementation for [`ShortlexUnsignedVectorRef`] for
    /// details.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::{ShortlexUnsignedVector, UnsignedVector};
    /// use std::collections::BTreeSet;
    ///
    /// let five = UnsignedVector::<u32>::from_str("(5)").unwrap();
    /// let zero_zero = UnsignedVector::<u32>::from_str("(0, 0)").unwrap();
    ///
    /// // Dimension decides, whatever the elements.
    /// assert!(ShortlexUnsignedVector(five.clone()) < ShortlexUnsignedVector(zero_zero.clone()));
    ///
    /// // The wrapper makes vectors usable in ordered collections.
    /// let set: BTreeSet<_> = [zero_zero, five]
    ///     .into_iter()
    ///     .map(ShortlexUnsignedVector)
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

impl<T: PrimitiveUnsigned> PartialOrd for ShortlexUnsignedVector<T> {
    /// Compares two [`ShortlexUnsignedVector`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
