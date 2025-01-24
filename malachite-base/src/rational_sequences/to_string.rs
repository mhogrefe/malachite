// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_sequences::RationalSequence;
use core::fmt::{Debug, Display, Formatter, Result, Write};

impl<T: Display + Eq> Display for RationalSequence<T> {
    /// Converts a [`RationalSequence`] to a [`String`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.component_len()`.
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
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_char('[')?;
        let mut first = true;
        for x in &self.non_repeating {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            Display::fmt(x, f)?;
        }
        if !self.repeating.is_empty() {
            if !self.non_repeating.is_empty() {
                f.write_str(", ")?;
            }
            f.write_char('[')?;
            let mut first = true;
            for x in &self.repeating {
                if first {
                    first = false;
                } else {
                    f.write_str(", ")?;
                }
                Display::fmt(x, f)?;
            }
            f.write_char(']')?;
        }
        f.write_char(']')
    }
}

impl<T: Display + Eq> Debug for RationalSequence<T> {
    /// Converts a [`RationalSequence`] to a [`String`].
    ///
    /// This is the same implementation as for [`Display`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.component_len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    /// use malachite_base::strings::ToDebugString;
    ///
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![], vec![]).to_debug_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![], vec![1, 2]).to_debug_string(),
    ///     "[[1, 2]]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![1, 2], vec![]).to_debug_string(),
    ///     "[1, 2]"
    /// );
    /// assert_eq!(
    ///     RationalSequence::<u8>::from_vecs(vec![1, 2], vec![3, 4]).to_string(),
    ///     "[1, 2, [3, 4]]"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
