// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::slices::min_repeating_len;
use alloc::vec::Vec;
use core::iter::{Chain, Cycle};

fn foer_sequence_reduce<T: Eq>(non_repeating: &mut Vec<T>, repeating: &mut Vec<T>) {
    if repeating.is_empty() {
        return;
    }
    repeating.truncate(min_repeating_len(repeating));
    if non_repeating.is_empty() {
        return;
    }
    let extra_non_repeating = non_repeating
        .iter()
        .rev()
        .zip(repeating.iter().rev().cycle())
        .take_while(|(x, y)| x == y)
        .count();
    if extra_non_repeating != 0 {
        non_repeating.truncate(non_repeating.len() - extra_non_repeating);
        let len = repeating.len();
        repeating.rotate_right(extra_non_repeating % len);
    }
}

private_test_fn! {foer_sequence_is_reduced<T: Eq>(non_repeating: &[T], repeating: &[T]) -> bool {
    if repeating.is_empty() {
        return true;
    }
    if min_repeating_len(repeating) != repeating.len() {
        return false;
    }
    if non_repeating.is_empty() {
        return true;
    }
    non_repeating
        .iter()
        .rev()
        .zip(repeating.iter().rev().cycle())
        .take_while(|(x, y)| x == y)
        .count()
        == 0
}}

/// A `FoerSequence` is a sequence that is either finite or eventually repeating, just like the
/// digits of a rational number. "Foer" abbreviates "finite or eventually repeating".
///
/// In testing, the set of foer sequences may be used as a proxy for the set of all sequences, which
/// is too large to work with.
#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct FoerSequence<T: Eq> {
    non_repeating: Vec<T>,
    repeating: Vec<T>,
}

impl<T: Eq> FoerSequence<T> {
    /// Returns whether this `FoerSequence` is empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::foer_sequences::FoerSequence;
    ///
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[]).is_empty(), true);
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[1, 2, 3]).is_empty(), false);
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[], &[3, 4]).is_empty(),
    ///     false
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[1, 2], &[3, 4]).is_empty(),
    ///     false
    /// );
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.non_repeating.is_empty() && self.repeating.is_empty()
    }

    /// Returns whether this `FoerSequence` is finite (has no repeating part).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::foer_sequences::FoerSequence;
    ///
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[]).is_finite(), true);
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[1, 2, 3]).is_finite(), true);
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[], &[3, 4]).is_finite(),
    ///     false
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[1, 2], &[3, 4]).is_finite(),
    ///     false
    /// );
    /// ```
    #[inline]
    pub const fn is_finite(&self) -> bool {
        self.repeating.is_empty()
    }

    /// Returns the length of this `FoerSequence`. If the sequence is infinite, `None` is returned.
    ///
    /// For a measure of length that always exists, try [`component_len`](Self::component_len).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::foer_sequences::FoerSequence;
    ///
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[]).len(), Some(0));
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[1, 2, 3]).len(), Some(3));
    /// assert_eq!(FoerSequence::<u8>::from_slices(&[], &[3, 4]).len(), None);
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[1, 2], &[3, 4]).len(),
    ///     None
    /// );
    /// ```
    pub const fn len(&self) -> Option<usize> {
        if self.repeating.is_empty() {
            Some(self.non_repeating.len())
        } else {
            None
        }
    }

    /// Returns the sum of the lengths of the non-repeating and repeating parts of this
    /// `FoerSequence`.
    ///
    /// This is often a more useful way of measuring the complexity of a sequence than
    /// [`len`](Self::len).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::foer_sequences::FoerSequence;
    ///
    /// assert_eq!(FoerSequence::<u8>::from_slice(&[]).component_len(), 0);
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slice(&[1, 2, 3]).component_len(),
    ///     3
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[], &[3, 4]).component_len(),
    ///     2
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[1, 2], &[3, 4]).component_len(),
    ///     4
    /// );
    /// ```
    pub const fn component_len(&self) -> usize {
        self.non_repeating.len() + self.repeating.len()
    }

    /// Returns an iterator of references to the elements of this sequence.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::foer_sequences::FoerSequence;
    ///
    /// let empty: &[u8] = &[];
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slice(empty)
    ///         .iter()
    ///         .cloned()
    ///         .collect_vec(),
    ///     empty
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slice(&[1, 2, 3])
    ///         .iter()
    ///         .cloned()
    ///         .collect_vec(),
    ///     &[1, 2, 3]
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[], &[3, 4])
    ///         .iter()
    ///         .cloned()
    ///         .take(10)
    ///         .collect_vec(),
    ///     &[3, 4, 3, 4, 3, 4, 3, 4, 3, 4]
    /// );
    /// assert_eq!(
    ///     FoerSequence::<u8>::from_slices(&[1, 2], &[3, 4])
    ///         .iter()
    ///         .cloned()
    ///         .take(10)
    ///         .collect_vec(),
    ///     &[1, 2, 3, 4, 3, 4, 3, 4, 3, 4]
    /// );
    /// ```
    #[inline]
    pub fn iter(&self) -> Chain<core::slice::Iter<'_, T>, Cycle<core::slice::Iter<'_, T>>> {
        self.non_repeating
            .iter()
            .chain(self.repeating.iter().cycle())
    }
}

impl<T: Clone + Eq> FoerSequence<T> {
    // Returns true iff `self` is valid.
    //
    // To be valid, the non-repeating and repeating parts must be reduced. For example, `[1, 2]` and
    // `[3, 4]` is a reduced pair. On the other hand, `[1, 2]` and `[3, 4, 3, 4]` is a non-reduced
    // pair representing the same sequence, as is `[1, 2, 3]` and `[4, 3]`. All `FoerSequence`s must
    // be valid.
    #[cfg(feature = "test_build")]
    #[inline]
    pub fn is_valid(&self) -> bool {
        foer_sequence_is_reduced(&self.non_repeating, &self.repeating)
    }
}

/// Functions for getting and setting elements in a [`FoerSequence`].
pub mod access;
/// Functions for comparing [`FoerSequence`]s.
pub mod cmp;
/// Functions for converting a [`FoerSequence`]s to and from a [`Vec`] or a slice.
pub mod conversion;
/// Functions for generating all [`FoerSequence`]s over a set of elements.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Functions for generating random [`FoerSequence`]s from a set of elements.
pub mod random;
/// Functions for displaying a [`FoerSequence`].
pub mod to_string;
