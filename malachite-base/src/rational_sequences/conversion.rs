// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_sequences::{rational_sequence_reduce, RationalSequence};
use alloc::vec;
use alloc::vec::Vec;

impl<T: Eq> RationalSequence<T> {
    /// Converts a [`Vec`] to a finite [`RationalSequence`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(RationalSequence::<u8>::from_vec(vec![]).to_string(), "[]");
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vec(vec![1, 2]).to_string(),
    ///     "[1, 2]"
    /// );
    /// ```
    pub const fn from_vec(non_repeating: Vec<T>) -> RationalSequence<T> {
        RationalSequence {
            non_repeating,
            repeating: vec![],
        }
    }

    /// Converts two [`Vec`]s to a finite [`RationalSequence`]. The first [`Vec`] is the
    /// nonrepeating part and the second is the repeating part.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m^{1+\varepsilon})$ for all $\varepsilon > 0$
    ///
    /// $M(n, m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `non_repeating.len()`, and $m$ is
    /// `repeating.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![], vec![]).to_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![], vec![1, 2]).to_string(),
    ///     "[[1, 2]]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![1, 2], vec![]).to_string(),
    ///     "[1, 2]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![1, 2], vec![3, 4]).to_string(),
    ///     "[1, 2, [3, 4]]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![1, 2, 3], vec![4, 3]).to_string(),
    ///     "[1, 2, [3, 4]]"
    /// );
    /// ```
    pub fn from_vecs(mut non_repeating: Vec<T>, mut repeating: Vec<T>) -> RationalSequence<T> {
        rational_sequence_reduce(&mut non_repeating, &mut repeating);
        RationalSequence {
            non_repeating,
            repeating,
        }
    }

    /// Converts a [`RationalSequence`] to a pair of [`Vec`]s containing the non-repeating and
    /// repeating parts, taking the [`RationalSequence`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(
    ///     RationalSequence::from_slices(&[1, 2], &[3, 4]).into_vecs(),
    ///     (vec![1, 2], vec![3, 4])
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)] // can't be const because of destructors
    pub fn into_vecs(self) -> (Vec<T>, Vec<T>) {
        (self.non_repeating, self.repeating)
    }

    /// Returns references to the non-repeating and repeating parts of a [`RationalSequence`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(
    ///     RationalSequence::from_slices(&[1u8, 2], &[3, 4]).slices_ref(),
    ///     (&[1u8, 2][..], &[3u8, 4][..])
    /// );
    /// ```
    pub fn slices_ref(&self) -> (&[T], &[T]) {
        (&self.non_repeating, &self.repeating)
    }
}

impl<T: Clone + Eq> RationalSequence<T> {
    /// Converts a slice to a finite [`RationalSequence`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(RationalSequence::<u8>::from_slice(&[]).to_string(), "[]");
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_slice(&[1, 2]).to_string(),
    ///     "[1, 2]"
    /// );
    /// ```
    pub fn from_slice(non_repeating: &[T]) -> RationalSequence<T> {
        RationalSequence {
            non_repeating: non_repeating.to_vec(),
            repeating: vec![],
        }
    }

    /// Converts two slices to a finite [`RationalSequence`]. The first slice is the nonrepeating
    /// part and the second is the repeating part.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m^{1+\varepsilon})$ for all $\varepsilon > 0$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `non_repeating.len()`, and $m$ is
    /// `repeating.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_slices(&[], &[]).to_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_slices(&[], &[1, 2]).to_string(),
    ///     "[[1, 2]]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_slices(&[1, 2], &[]).to_string(),
    ///     "[1, 2]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_slices(&[1, 2], &[3, 4]).to_string(),
    ///     "[1, 2, [3, 4]]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_slices(&[1, 2, 3], &[4, 3]).to_string(),
    ///     "[1, 2, [3, 4]]"
    /// );
    /// ```
    pub fn from_slices(non_repeating: &[T], repeating: &[T]) -> RationalSequence<T> {
        let mut non_repeating = non_repeating.to_vec();
        let mut repeating = repeating.to_vec();
        rational_sequence_reduce(&mut non_repeating, &mut repeating);
        RationalSequence {
            non_repeating,
            repeating,
        }
    }

    /// Converts a [`RationalSequence`] to a pair of [`Vec`]s containing the non-repeating and
    /// repeating parts, taking the [`RationalSequence`] by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.component_len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(
    ///     RationalSequence::from_slices(&[1, 2], &[3, 4]).to_vecs(),
    ///     (vec![1, 2], vec![3, 4])
    /// );
    /// ```
    pub fn to_vecs(&self) -> (Vec<T>, Vec<T>) {
        (self.non_repeating.clone(), self.repeating.clone())
    }
}
