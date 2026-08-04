// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::foer_sequences::FoerSequence;
use crate::num::random::geometric::GeometricRandomNaturalValues;
use crate::random::Seed;
use crate::vecs::random::{RandomVecs, random_vecs};

/// Generates random [`FoerSequence`]s, given an iterator of random elements.
///
/// This `struct` is created by [`random_foer_sequences`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomFoerSequences<I: Iterator>(
    RandomVecs<I::Item, GeometricRandomNaturalValues<u64>, I>,
)
where
    I::Item: Eq;

impl<I: Iterator> Iterator for RandomFoerSequences<I>
where
    I::Item: Eq,
{
    type Item = FoerSequence<I::Item>;

    fn next(&mut self) -> Option<FoerSequence<I::Item>> {
        Some(FoerSequence::from_vecs(
            self.0.next().unwrap(),
            self.0.next().unwrap(),
        ))
    }
}

/// Generates random [`FoerSequence`]s whose non-repeating and repeating components have a specified
/// mean length, with elements from a given iterator.
///
/// The input iterator must be infinite, but this is not enforced.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(m^{1+\varepsilon} T^\prime(i))$ for all $\varepsilon > 0$
///
/// $M(i) = O(m M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $m$ is
/// `mean_length_numerator / mean_length_denominator`, and $T^\prime$ and $M^\prime$ are the time
/// and memory functions of the iterators produced by `xs_gen`: the non-repeating and repeating
/// parts have geometrically-distributed lengths with mean $m$, and reducing the sequence finds the
/// repeating part's minimal period.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::foer_sequences::random::random_foer_sequences;
/// use malachite_base::foer_sequences::FoerSequence;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_foer_sequences(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1)
///         .take(10)
///         .map(|x| FoerSequence::to_string(&x))
///         .collect_vec(),
///     &[
///         "[[85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32]]",
///         "[166, 234, 30, 218, [90, 106, 9, 216]]",
///         "[204]",
///         "[151, 213, 97, 253, 78, [91, 39]]",
///         "[191, 175, 170, 232]",
///         "[233, 2, 35, 22, 217, 198]",
///         "[[114, 17, 32, 173, 114, 65, 121, 222, 173, 25, 144]]",
///         "[148, 79, 115, 52, 73, 69, 137, 91]",
///         "[153, 178, 112]",
///         "[34, 95, 106, 167, 197, [130, 168, 122, 207, 172, 177, 86, 150, 221]]"
///     ]
/// )
/// ```
pub fn random_foer_sequences<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomFoerSequences<I>
where
    I::Item: Eq,
{
    RandomFoerSequences(random_vecs(
        seed,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}
