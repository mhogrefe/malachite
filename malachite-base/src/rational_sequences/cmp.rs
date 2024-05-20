// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_sequences::RationalSequence;
use core::cmp::Ordering::{self, *};

impl<T: Eq + Ord> PartialOrd for RationalSequence<T> {
    /// Compares a [`RationalSequence`] to another [`RationalSequence`].
    ///
    /// See [here](RationalSequence::cmp) for more information.
    #[inline]
    fn partial_cmp(&self, other: &RationalSequence<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Ord> Ord for RationalSequence<T> {
    /// Compares a [`RationalSequence`] to another [`RationalSequence`].
    ///
    /// The comparison is made lexicographically with respect to the element type's ordering.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.component_len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert!(
    ///     RationalSequence::from_slice(&[1, 2]) < RationalSequence::from_slices(&[1, 2], &[1])
    /// );
    /// assert!(
    ///     RationalSequence::from_slice(&[1, 2, 3])
    ///         < RationalSequence::from_slices(&[1, 2], &[3, 4])
    /// );
    /// ```
    fn cmp(&self, other: &RationalSequence<T>) -> Ordering {
        if self == other {
            Equal
        } else {
            Iterator::cmp(self.iter(), other.iter())
        }
    }
}
