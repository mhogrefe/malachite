// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::vector::Vector;
use core::fmt::{Debug, Display, Formatter, Result};

impl<T: Display> Display for Vector<T> {
    /// Converts a [`Vector`] to a [`String`](alloc::string::String).
    ///
    /// The elements are written in order, as [`Display`] writes them, separated by `, ` and
    /// enclosed in parentheses, as a vector's coordinates are written by hand. The 0-dimensional
    /// vector is `()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![1u32, 2, 3]
    ///     }
    ///     .to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![-5i32]
    ///     }
    ///     .to_string(),
    ///     "(-5)"
    /// );
    /// assert_eq!(
    ///     Vector::<u32> {
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

impl<T: Display> Debug for Vector<T> {
    /// Converts a [`Vector`] to a [`String`](alloc::string::String).
    ///
    /// This is the same as the [`Display::fmt`] implementation, so that a collection of
    /// [`Vector`]s is written the same way its elements are displayed.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::Vector;
    ///
    /// let xs = vec![
    ///     Vector {
    ///         elements: vec![1u32, 2],
    ///     },
    ///     Vector {
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
