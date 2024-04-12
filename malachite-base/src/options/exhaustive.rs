// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::{once, Chain, Once};

/// Generates all [`Option`]s except `None`, with values from a given iterator.
///
/// This `struct` is created by [`exhaustive_somes`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveSomes<I: Iterator> {
    xs: I,
}

impl<I: Iterator> Iterator for ExhaustiveSomes<I> {
    type Item = Option<I::Item>;

    fn next(&mut self) -> Option<Option<I::Item>> {
        self.xs.next().map(Some)
    }
}

/// Generates all [`Option`]s except `None`, with values from a given iterator.
///
/// The elements of the given iterator are wrapped in `Some` and generated in the original order.
///
/// The output length is `xs.count()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::options::exhaustive::exhaustive_somes;
///
/// assert_eq!(
///     exhaustive_somes([1, 2, 3].iter().cloned()).collect_vec(),
///     &[Some(1), Some(2), Some(3)]
/// );
/// ```
#[inline]
pub const fn exhaustive_somes<I: Iterator>(xs: I) -> ExhaustiveSomes<I> {
    ExhaustiveSomes { xs }
}

/// Generates all [`Option`]s with values from a given iterator.
///
/// `None` comes first, followed by the elements of the given iterator wrapped in `Some`.
///
/// The output length is `xs.count()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::options::exhaustive::exhaustive_options;
///
/// assert_eq!(
///     exhaustive_options([1, 2, 3].iter().cloned()).collect_vec(),
///     &[None, Some(1), Some(2), Some(3)]
/// );
/// ```
#[inline]
pub fn exhaustive_options<I: Iterator>(xs: I) -> Chain<Once<Option<I::Item>>, ExhaustiveSomes<I>> {
    once(None).chain(exhaustive_somes(xs))
}
