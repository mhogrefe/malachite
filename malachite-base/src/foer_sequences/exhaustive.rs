// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::foer_sequences::{FoerSequence, foer_sequence_is_reduced};
use crate::num::exhaustive::PrimitiveIntIncreasingRange;
use crate::tuples::exhaustive::{ExhaustivePairs1Input, exhaustive_pairs_from_single};
use crate::vecs::exhaustive::{ExhaustiveVecs, exhaustive_vecs};

/// Generates all [`FoerSequence`]s containing elements from an iterator.
///
/// This `struct` is created by [`exhaustive_foer_sequences`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveFoerSequences<I: Clone + Iterator>(
    ExhaustivePairs1Input<ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>>,
)
where
    I::Item: Clone;

impl<I: Clone + Iterator> Iterator for ExhaustiveFoerSequences<I>
where
    I::Item: Clone + Eq,
{
    type Item = FoerSequence<I::Item>;

    fn next(&mut self) -> Option<FoerSequence<I::Item>> {
        loop {
            let (non_repeating, repeating) = self.0.next()?;
            if foer_sequence_is_reduced(&non_repeating, &repeating) {
                return Some(FoerSequence {
                    non_repeating,
                    repeating,
                });
            }
        }
    }
}

/// Generates all [`FoerSequence`]s containing elements from a given iterator.
///
/// The input iterator should contain no repetitions, but this is not enforced.
///
/// The output length is 1 if the input iterator is empty, and infinite otherwise.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(T^\prime(i) + (\log i)^{1+\varepsilon})$ for all $\varepsilon > 0$
///
/// $M(i) = O((\log i) M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the input iterator.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::foer_sequences::exhaustive::exhaustive_foer_sequences;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::strings::ToDebugString;
///
/// assert_eq!(
///     exhaustive_foer_sequences(exhaustive_unsigneds::<u8>())
///         .take(10)
///         .collect_vec()
///         .to_debug_string(),
///     "[[], [[0]], [0], [[1]], [0, [1]], [1], [1, [0]], [0, 0, 0], [0, 0, 0, [1]], [[2]]]"
/// )
/// ```
pub fn exhaustive_foer_sequences<I: Clone + Iterator>(xs: I) -> ExhaustiveFoerSequences<I>
where
    I::Item: Clone + Eq,
{
    ExhaustiveFoerSequences(exhaustive_pairs_from_single(exhaustive_vecs(xs)))
}
