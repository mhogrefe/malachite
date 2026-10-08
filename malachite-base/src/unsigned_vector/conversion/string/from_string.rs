// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;
use core::str::FromStr;

impl<T: PrimitiveUnsigned> FromStr for UnsignedVector<T> {
    type Err = ();

    /// Converts a string to an [`UnsignedVector`].
    ///
    /// This reads back what [`Display`](core::fmt::Display) writes: the elements in order,
    /// separated by `", "` and enclosed in parentheses, with `()` for the 0-dimensional vector.
    /// Each element is read the way `T`'s [`FromStr`] reads one, so it may have leading zeros or a
    /// single leading `'+'`. Any other spacing, such as `(1,2)` or `( 1, 2 )`, is rejected, as is
    /// an empty element or one that is out of `T`'s range.
    ///
    /// If the string does not represent an [`UnsignedVector`], an `Err` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(5)").unwrap().to_string(),
    ///     "(5)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("()").unwrap().dimension(),
    ///     0
    /// );
    ///
    /// // An element may be written as `u32::from_str` accepts it.
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(007, +8)")
    ///         .unwrap()
    ///         .to_string(),
    ///     "(7, 8)"
    /// );
    ///
    /// assert!(UnsignedVector::<u32>::from_str("").is_err());
    /// assert!(UnsignedVector::<u32>::from_str("1, 2").is_err());
    /// assert!(UnsignedVector::<u32>::from_str("(1,2)").is_err());
    /// assert!(UnsignedVector::<u32>::from_str("( 1, 2 )").is_err());
    /// assert!(UnsignedVector::<u32>::from_str("(1, )").is_err());
    /// assert!(UnsignedVector::<u32>::from_str("(-1)").is_err());
    /// assert!(UnsignedVector::<u8>::from_str("(256)").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        let inner = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .ok_or(())?;
        if inner.is_empty() {
            return Ok(Self {
                elements: Vec::new(),
            });
        }
        Ok(Self {
            elements: inner
                .split(", ")
                .map(|x| T::from_str(x).map_err(|_| ()))
                .collect::<Result<_, _>>()?,
        })
    }
}
