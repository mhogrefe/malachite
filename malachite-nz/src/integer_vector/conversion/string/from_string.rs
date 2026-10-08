// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use alloc::vec::Vec;
use core::str::FromStr;

impl FromStr for IntegerVector {
    type Err = ();

    /// Converts a string to an [`IntegerVector`].
    ///
    /// This reads back what [`Display`](core::fmt::Display) writes: the elements in order,
    /// separated by `", "` and enclosed in parentheses, with `()` for the 0-dimensional vector.
    /// Each element is read the way [`Integer::from_str`] reads one, so it may have leading zeros
    /// and a single leading `'-'` or `'+'`. Any other spacing, such as `(1,2)` or `( 1, 2 )`, is
    /// rejected, as is an empty element.
    ///
    /// If the string does not represent an [`IntegerVector`], an `Err` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, 2, 3)").unwrap().to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(IntegerVector::from_str("(5)").unwrap().to_string(), "(5)");
    /// assert_eq!(IntegerVector::from_str("()").unwrap().dimension(), 0);
    ///
    /// // An element may be written as `Integer::from_str` accepts it.
    /// assert_eq!(
    ///     IntegerVector::from_str("(-1, -0, +2)").unwrap().to_string(),
    ///     "(-1, 0, 2)"
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(007, +8)").unwrap().to_string(),
    ///     "(7, 8)"
    /// );
    ///
    /// assert!(IntegerVector::from_str("").is_err());
    /// assert!(IntegerVector::from_str("1, 2").is_err());
    /// assert!(IntegerVector::from_str("(1,2)").is_err());
    /// assert!(IntegerVector::from_str("( 1, 2 )").is_err());
    /// assert!(IntegerVector::from_str("(1, )").is_err());
    /// assert!(IntegerVector::from_str("(--1)").is_err());
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
                .map(Integer::from_str)
                .collect::<Result<_, _>>()?,
        })
    }
}
