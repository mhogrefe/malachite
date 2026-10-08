// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use core::str::FromStr;

impl FromStr for NaturalVector {
    type Err = ();

    /// Converts a string to a [`NaturalVector`].
    ///
    /// This reads back what [`Display`](core::fmt::Display) writes: the elements in order,
    /// separated by `", "` and enclosed in parentheses, with `()` for the 0-dimensional vector.
    /// Each element is read the way [`Natural::from_str`] reads one, so it may have leading zeros
    /// or a single leading `'+'`. Any other spacing, such as `(1,2)` or `( 1, 2 )`, is rejected, as
    /// is an empty element.
    ///
    /// If the string does not represent a [`NaturalVector`], an `Err` is returned.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)").unwrap().to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(NaturalVector::from_str("(5)").unwrap().to_string(), "(5)");
    /// assert_eq!(NaturalVector::from_str("()").unwrap().dimension(), 0);
    ///
    /// // An element may be written as `Natural::from_str` accepts it.
    /// assert_eq!(
    ///     NaturalVector::from_str("(007, +8)").unwrap().to_string(),
    ///     "(7, 8)"
    /// );
    ///
    /// assert!(NaturalVector::from_str("").is_err());
    /// assert!(NaturalVector::from_str("1, 2").is_err());
    /// assert!(NaturalVector::from_str("(1,2)").is_err());
    /// assert!(NaturalVector::from_str("( 1, 2 )").is_err());
    /// assert!(NaturalVector::from_str("(1, )").is_err());
    /// assert!(NaturalVector::from_str("(-1)").is_err());
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
                .map(Natural::from_str)
                .collect::<Result<_, _>>()?,
        })
    }
}
