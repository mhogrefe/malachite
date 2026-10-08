// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use core::fmt::{Debug, Display, Formatter, Result};

impl Display for IntegerVector {
    /// Converts an [`IntegerVector`] to a [`String`](alloc::string::String).
    ///
    /// The elements are written in order, separated by `, ` and enclosed in parentheses, as a
    /// vector's coordinates are written by hand. The 0-dimensional vector is `()`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector {
    ///     elements: vec![Integer::from(1u32), Integer::from(2u32), Integer::from(3u32)],
    /// };
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// assert_eq!(
    ///     IntegerVector {
    ///         elements: vec![Integer::from(5u32)]
    ///     }
    ///     .to_string(),
    ///     "(5)"
    /// );
    /// assert_eq!(
    ///     IntegerVector {
    ///         elements: Vec::new()
    ///     }
    ///     .to_string(),
    ///     "()"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("(")?;
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            Display::fmt(x, f)?;
        }
        f.write_str(")")
    }
}

impl Debug for IntegerVector {
    /// Converts an [`IntegerVector`] to a [`String`](alloc::string::String).
    ///
    /// This is the same as the [`Display::fmt`] implementation, so that a collection of
    /// [`IntegerVector`]s is written the same way its elements are displayed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let xs = vec![
    ///     IntegerVector {
    ///         elements: vec![Integer::from(1u32), Integer::from(2u32)],
    ///     },
    ///     IntegerVector {
    ///         elements: Vec::new(),
    ///     },
    /// ];
    /// assert_eq!(xs[0].to_debug_string(), "(1, 2)");
    /// assert_eq!(xs.to_debug_string(), "[(1, 2), ()]");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
