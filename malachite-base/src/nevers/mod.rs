// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use crate::strings::typst::ToTypst;
use core::fmt::{Display, Formatter};
use core::iter::{Empty, empty};
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

impl ToLatex for Never {
    /// Would write a [`Never`] as a LaTeX math-mode fragment.
    ///
    /// A [`Never`] cannot be instantiated, so this can never be called. The implementation exists
    /// so that a type parameter bounded by [`ToLatex`] may be [`Never`], which is what lets
    /// `Option<Never>` have a fragment of its own.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::nevers::Never;
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// // The only `Option<Never>` there is.
    /// assert_eq!(None::<Never>.to_latex().to_string(), r"\bot");
    /// ```
    fn fmt_latex(&self, _f: &mut Formatter) -> core::fmt::Result {
        unreachable!()
    }
}

impl ToTypst for Never {
    /// Would write a [`Never`] as a Typst math-mode fragment.
    ///
    /// A [`Never`] cannot be instantiated, so this can never be called. The implementation exists
    /// so that a type parameter bounded by [`ToTypst`] may be [`Never`], which is what lets
    /// `Option<Never>` have a fragment of its own.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::nevers::Never;
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// // The only `Option<Never>` there is.
    /// assert_eq!(None::<Never>.to_typst().to_string(), "bot");
    /// ```
    fn fmt_typst(&self, _f: &mut Formatter) -> core::fmt::Result {
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
    fn from_str(_: &str) -> Result<Self, NeverError> {
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
#[inline]
pub const fn nevers() -> Empty<Never> {
    empty()
}
