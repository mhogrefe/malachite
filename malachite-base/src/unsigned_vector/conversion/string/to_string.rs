// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::fmt::{Debug, Display, Formatter, Result};

impl<T: PrimitiveUnsigned> Display for UnsignedVector<T> {
    /// Converts an [`UnsignedVector`] to a [`String`](alloc::string::String).
    ///
    /// The elements are written in order, as [`Display`] writes them, separated by `, ` and
    /// enclosed in parentheses, as a vector's coordinates are written by hand. The 0-dimensional
    /// vector is `()`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector {
    ///         elements: vec![1u32, 2, 3]
    ///     }
    ///     .to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector {
    ///         elements: vec![5u8]
    ///     }
    ///     .to_string(),
    ///     "(5)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32> {
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

impl<T: PrimitiveUnsigned> Debug for UnsignedVector<T> {
    /// Converts an [`UnsignedVector`] to a [`String`](alloc::string::String).
    ///
    /// This is the same as the [`Display::fmt`] implementation, so that a collection of
    /// [`UnsignedVector`]s is written the same way its elements are displayed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let xs = vec![
    ///     UnsignedVector {
    ///         elements: vec![1u32, 2],
    ///     },
    ///     UnsignedVector {
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
