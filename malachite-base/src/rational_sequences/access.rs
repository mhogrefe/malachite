// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_sequences::{rational_sequence_reduce, RationalSequence};
use core::ops::Index;

impl<T: Eq> Index<usize> for RationalSequence<T> {
    type Output = T;

    /// Gets a reference to an element of a [`RationalSequence`] at an index.
    ///
    /// If the index is greater than or equal to the length of the sequence, this function panics.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `index` is greater than or equal to the length of this sequence.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(RationalSequence::from_slices(&[1, 2], &[3, 4])[1], 2);
    /// assert_eq!(RationalSequence::from_slices(&[1, 2], &[3, 4])[10], 3);
    /// ```
    #[inline]
    fn index(&self, i: usize) -> &T {
        self.get(i).unwrap()
    }
}

impl<T: Eq> RationalSequence<T> {
    /// Gets a reference to an element of a [`RationalSequence`] at an index.
    ///
    /// If the index is greater than or equal to the length of the sequence, `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert_eq!(
    ///     RationalSequence::from_slices(&[1, 2], &[3, 4]).get(1),
    ///     Some(&2)
    /// );
    /// assert_eq!(
    ///     RationalSequence::from_slices(&[1, 2], &[3, 4]).get(10),
    ///     Some(&3)
    /// );
    /// ```
    pub fn get(&self, i: usize) -> Option<&T> {
        let non_repeating_len = self.non_repeating.len();
        if i < non_repeating_len {
            Some(&self.non_repeating[i])
        } else if self.repeating.is_empty() {
            None
        } else {
            Some(&self.repeating[(i - non_repeating_len) % self.repeating.len()])
        }
    }
}

impl<T: Clone + Eq> RationalSequence<T> {
    /// Mutates an element of a [`RationalSequence`] at an index using a provided closure, and then
    /// returns whatever the closure returns.
    ///
    /// If the index is greater than or equal to the length of the sequence, this function panics.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `index`.
    ///
    /// # Panics
    /// Panics if `index` is greater than or equal to the length of this sequence.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// let mut xs = RationalSequence::from_slices(&[1, 2], &[3, 4]);
    /// assert_eq!(
    ///     xs.mutate(1, |x| {
    ///         *x = 100;
    ///         25
    ///     }),
    ///     25
    /// );
    /// assert_eq!(xs, RationalSequence::from_slices(&[1, 100], &[3, 4]));
    ///
    /// let mut xs = RationalSequence::from_slices(&[1, 2], &[3, 4]);
    /// assert_eq!(
    ///     xs.mutate(6, |x| {
    ///         *x = 100;
    ///         25
    ///     }),
    ///     25
    /// );
    /// assert_eq!(
    ///     xs,
    ///     RationalSequence::from_slices(&[1, 2, 3, 4, 3, 4, 100], &[4, 3])
    /// );
    /// ```
    pub fn mutate<F: FnOnce(&mut T) -> U, U>(&mut self, i: usize, f: F) -> U {
        let non_repeating_len = self.non_repeating.len();
        let result = if i < non_repeating_len {
            f(&mut self.non_repeating[i])
        } else if self.repeating.is_empty() {
            panic!("index out of bounds");
        } else {
            let repeating_len = self.repeating.len();
            let extra = i - non_repeating_len + 1;
            self.non_repeating
                .extend(self.repeating.iter().cycle().take(extra).cloned());
            self.repeating.rotate_left(extra % repeating_len);
            f(&mut self.non_repeating[i])
        };
        rational_sequence_reduce(&mut self.non_repeating, &mut self.repeating);
        result
    }
}
