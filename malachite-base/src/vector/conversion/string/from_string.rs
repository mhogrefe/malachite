// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::vector::Vector;
use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

impl<T: FromStr> FromStr for Vector<T> {
    type Err = ();

    /// Converts a string to a [`Vector`].
    ///
    /// This reads back what [`Display`](core::fmt::Display) writes: the elements in order,
    /// separated by `", "` and enclosed in parentheses, with `()` for the 0-dimensional vector.
    /// Each element is read by `T`'s [`FromStr`]. Any other spacing, such as `(1,2)` or
    /// `( 1, 2 )`, is rejected, as is an empty element.
    ///
    /// An element's own string may contain `", "`, as a vector of vectors' does. The string is
    /// split at each `", "`, and the pieces are joined back together until they read as an
    /// element, so the parser takes the shortest element it can. This is ambiguous only when a
    /// string can be split into elements in more than one way.
    ///
    /// If the string does not represent a [`Vector`], an `Err` is returned.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     Vector::<u32>::from_str("(1, 2, 3)").unwrap().to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(Vector::<i32>::from_str("(-5)").unwrap().to_string(), "(-5)");
    /// assert_eq!(Vector::<u32>::from_str("()").unwrap().dimension(), 0);
    ///
    /// // A vector of vectors.
    /// assert_eq!(
    ///     Vector::<Vector<u32>>::from_str("((1, 2), (), (3))")
    ///         .unwrap()
    ///         .dimension(),
    ///     3
    /// );
    ///
    /// assert!(Vector::<u32>::from_str("").is_err());
    /// assert!(Vector::<u32>::from_str("1, 2").is_err());
    /// assert!(Vector::<u32>::from_str("(1,2)").is_err());
    /// assert!(Vector::<u32>::from_str("( 1, 2 )").is_err());
    /// assert!(Vector::<u32>::from_str("(1, )").is_err());
    /// assert!(Vector::<u32>::from_str("(-1)").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        let inner = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .ok_or(())?;
        let mut elements = Vec::new();
        if inner.is_empty() {
            return Ok(Self { elements });
        }
        // `pending` rather than `buffer.is_empty()`, so that an empty piece is not dropped
        let mut buffer = String::new();
        let mut pending = false;
        for piece in inner.split(", ") {
            if pending {
                buffer.push_str(", ");
            }
            buffer.push_str(piece);
            pending = true;
            if let Ok(x) = T::from_str(&buffer) {
                elements.push(x);
                buffer.clear();
                pending = false;
            }
        }
        if pending {
            Err(())
        } else {
            Ok(Self { elements })
        }
    }
}
