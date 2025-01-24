// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::{Display, Formatter};
use core::iter::{empty, Empty};
use core::str::FromStr;

/// `Never` is a type that cannot be instantiated.
///
/// This is a [bottom type](https://en.wikipedia.org/wiki/Bottom_type).
///
/// # Examples
/// ```
/// use malachite_base::nevers::Never;
///
/// let _x: Option<Never> = None;
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Never {}

impl Display for Never {
    /// Would convert a [`Never`] to a [`String`].
    fn fmt(&self, _f: &mut Formatter) -> core::fmt::Result {
        unreachable!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NeverError;

impl FromStr for Never {
    type Err = NeverError;

    /// Would convert a [`String`] to a [`Never`].
    ///
    /// Since a [`Never`] can never be instantiated, `from_str` never succeeds.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::nevers::{Never, NeverError};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Never::from_str("abc"), Err(NeverError));
    /// ```
    #[inline]
    fn from_str(_: &str) -> Result<Never, NeverError> {
        Err(NeverError)
    }
}

/// Generates all (none) of the [`Never`]s.
///
/// The output length is 0.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::nevers::nevers;
///
/// assert_eq!(nevers().collect_vec(), &[]);
/// ```
pub const fn nevers() -> Empty<Never> {
    empty()
}
