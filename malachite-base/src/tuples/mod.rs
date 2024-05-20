// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Generates all singletons (1-element tuples) with values from a given iterator.
///
/// This `struct` is created by [`singletons`]; see its documentation for more.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Singletons<I: Iterator> {
    xs: I,
}

impl<I: Iterator> Iterator for Singletons<I> {
    type Item = (I::Item,);

    #[inline]
    fn next(&mut self) -> Option<(I::Item,)> {
        self.xs.next().map(|x| (x,))
    }
}

/// Generates all singletons (1-element tuples) with values from a given iterator.
///
/// The elements appear in the same order as they do in the given iterator, but wrapped in `(_,)`.
///
/// The output length is `xs.count()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::singletons;
///
/// assert_eq!(
///     singletons([1, 2, 3].iter().cloned()).collect_vec(),
///     &[(1,), (2,), (3,)]
/// );
/// ```
#[inline]
pub const fn singletons<I: Iterator>(xs: I) -> Singletons<I> {
    Singletons { xs }
}

/// Iterators that generate tuples without repetition.
///
/// To reduce binary size and lower compilation time, many of the functions described here are not
/// actually defined in Malachite, but may be created in your program using macros exported from
/// Malachite. To do this, see the documentation for `lex_tuples` and `lex_custom_tuples`.
///
/// # lex_pairs
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::lex_pairs;
///
/// assert_eq!(
///     lex_pairs('a'..'f', 0..3).collect_vec(),
///     &[
///         ('a', 0),
///         ('a', 1),
///         ('a', 2),
///         ('b', 0),
///         ('b', 1),
///         ('b', 2),
///         ('c', 0),
///         ('c', 1),
///         ('c', 2),
///         ('d', 0),
///         ('d', 1),
///         ('d', 2),
///         ('e', 0),
///         ('e', 1),
///         ('e', 2)
///     ]
/// );
/// ```
///
/// # lex_pairs_from_single
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::lex_pairs_from_single;
///
/// assert_eq!(
///     lex_pairs_from_single(0..3).collect_vec(),
///     &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)]
/// );
/// ```
///
/// # lex_triples_xyx
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_custom_tuples;
///
/// fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
///     (a.unwrap(), b.unwrap(), c.unwrap())
/// }
///
/// lex_custom_tuples!(
///     (pub(crate)),
///     LexTriplesXYX,
///     (X, Y, X),
///     (None, None, None),
///     unwrap_triple,
///     lex_triples_xyx,
///     [X, I, xs, [0, x_0], [2, x_2]],
///     [Y, J, ys, [1, y_1]]
/// );
///
/// // We are generating triples of `char`, `i8`, and `char` using two input iterators. The first
/// // iterator, `xs`, the chars 'a' through 'c', and the second, `ys`, produces the three numbers
/// // 0, 1, and 2. The function we're using is `lex_triples_xyx`, meaning that the first element of
/// // the output triples will be taken from `xs`, the second element from `ys`, and the third also
/// // from `xs`.
/// let ts = lex_triples_xyx('a'..='c', 0..3);
/// assert_eq!(
///     ts.collect_vec(),
///     &[
///         ('a', 0, 'a'),
///         ('a', 0, 'b'),
///         ('a', 0, 'c'),
///         ('a', 1, 'a'),
///         ('a', 1, 'b'),
///         ('a', 1, 'c'),
///         ('a', 2, 'a'),
///         ('a', 2, 'b'),
///         ('a', 2, 'c'),
///         ('b', 0, 'a'),
///         ('b', 0, 'b'),
///         ('b', 0, 'c'),
///         ('b', 1, 'a'),
///         ('b', 1, 'b'),
///         ('b', 1, 'c'),
///         ('b', 2, 'a'),
///         ('b', 2, 'b'),
///         ('b', 2, 'c'),
///         ('c', 0, 'a'),
///         ('c', 0, 'b'),
///         ('c', 0, 'c'),
///         ('c', 1, 'a'),
///         ('c', 1, 'b'),
///         ('c', 1, 'c'),
///         ('c', 2, 'a'),
///         ('c', 2, 'b'),
///         ('c', 2, 'c')
///     ]
/// );
/// ```
///
/// # exhaustive_pairs_from_single
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::exhaustive_pairs_from_single;
///
/// assert_eq!(
///     exhaustive_pairs_from_single(0..4).collect_vec(),
///     &[
///         (0, 0),
///         (0, 1),
///         (1, 0),
///         (1, 1),
///         (0, 2),
///         (0, 3),
///         (1, 2),
///         (1, 3),
///         (2, 0),
///         (2, 1),
///         (3, 0),
///         (3, 1),
///         (2, 2),
///         (2, 3),
///         (3, 2),
///         (3, 3)
///     ]
/// );
/// ```
///
/// # exhaustive_pairs_1_input
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::exhaustive_tuples_1_input;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::arithmetic::traits::CheckedPow;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use std::cmp::max;
/// use std::marker::PhantomData;
///
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveTriples1Input,
///     exhaustive_triples_1_input,
///     exhaustive_triples_from_single,
///     (I::Item, I::Item, I::Item),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z]
/// );
///
/// // We are generating triples of `char`s using one input iterator, which produces all ASCII
/// // `char`s. The third element has a tiny output type, so it will grow more slowly than the other
/// // two elements (though it doesn't look that way from the first few tuples).
/// let ts = exhaustive_triples_1_input(
///     exhaustive_ascii_chars(),
///     BitDistributorOutputType::normal(1),
///     BitDistributorOutputType::normal(1),
///     BitDistributorOutputType::tiny(),
/// );
/// assert_eq!(
///     ts.take(20).collect_vec(),
///     &[
///         ('a', 'a', 'a'),
///         ('a', 'a', 'b'),
///         ('a', 'a', 'c'),
///         ('a', 'a', 'd'),
///         ('a', 'b', 'a'),
///         ('a', 'b', 'b'),
///         ('a', 'b', 'c'),
///         ('a', 'b', 'd'),
///         ('a', 'a', 'e'),
///         ('a', 'a', 'f'),
///         ('a', 'a', 'g'),
///         ('a', 'a', 'h'),
///         ('a', 'b', 'e'),
///         ('a', 'b', 'f'),
///         ('a', 'b', 'g'),
///         ('a', 'b', 'h'),
///         ('b', 'a', 'a'),
///         ('b', 'a', 'b'),
///         ('b', 'a', 'c'),
///         ('b', 'a', 'd')
///     ]
/// );
/// ```
///
/// # exhaustive_pairs
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::exhaustive_pairs;
///
/// let xss = exhaustive_pairs(['a', 'b', 'c'].iter().cloned(), 0..3).collect_vec();
/// assert_eq!(
///     xss,
///     &[('a', 0), ('a', 1), ('b', 0), ('b', 1), ('a', 2), ('b', 2), ('c', 0), ('c', 1), ('c', 2)]
/// );
/// ```
///
/// # exhaustive_pairs_custom_output
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::tuples::exhaustive::exhaustive_pairs_custom_output;
///
/// let xss = exhaustive_pairs_custom_output(
///     ['a', 'b', 'c'].iter().cloned(),
///     0..3,
///     BitDistributorOutputType::normal(1),
///     BitDistributorOutputType::tiny(),
/// )
/// .collect_vec();
/// assert_eq!(
///     xss,
///     &[('a', 0), ('a', 1), ('a', 2), ('b', 0), ('b', 1), ('b', 2), ('c', 0), ('c', 1), ('c', 2)]
/// );
/// ```
///
/// # exhaustive_triples_xyx
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::custom_tuples;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use std::cmp::max;
///
/// #[allow(clippy::missing_const_for_fn)]
/// fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
///     (a.unwrap(), b.unwrap(), c.unwrap())
/// }
///
/// custom_tuples!(
///     (pub(crate)),
///     ExhaustiveTriplesXYX,
///     (X, Y, X),
///     (None, None, None),
///     unwrap_triple,
///     exhaustive_triples_xyx,
///     exhaustive_triples_xyx_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [2, output_type_ys_1]],
///     [Y, J, ys, ys_done, [1, output_type_xs_2]]
/// );
///
/// // We are generating triples of `char`, `i8`, and `char` using two input iterators. The first
/// // iterator, `xs`, produces all ASCII `char`s, and the second, `ys`, produces the three numbers
/// // 0, 1, and 2. The function we're using is `exhaustive_triples_xyx`, meaning that the first
/// // element of the output triples will be taken from `xs`, the second element from `ys`, and the
/// // third also from `xs`.
/// let ts = exhaustive_triples_xyx(exhaustive_ascii_chars(), 0..3);
/// assert_eq!(
///     ts.take(20).collect_vec(),
///     &[
///         ('a', 0, 'a'),
///         ('a', 0, 'b'),
///         ('a', 1, 'a'),
///         ('a', 1, 'b'),
///         ('b', 0, 'a'),
///         ('b', 0, 'b'),
///         ('b', 1, 'a'),
///         ('b', 1, 'b'),
///         ('a', 0, 'c'),
///         ('a', 0, 'd'),
///         ('a', 1, 'c'),
///         ('a', 1, 'd'),
///         ('b', 0, 'c'),
///         ('b', 0, 'd'),
///         ('b', 1, 'c'),
///         ('b', 1, 'd'),
///         ('a', 2, 'a'),
///         ('a', 2, 'b'),
///         ('b', 2, 'a'),
///         ('b', 2, 'b')
///     ]
/// );
/// ```
///
/// # exhaustive_triples_xyx_custom_output
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::custom_tuples;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use std::cmp::max;
///
/// #[allow(clippy::missing_const_for_fn)]
/// fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
///     (a.unwrap(), b.unwrap(), c.unwrap())
/// }
///
/// custom_tuples!(
///     (pub(crate)),
///     ExhaustiveTriplesXYX,
///     (X, Y, X),
///     (None, None, None),
///     unwrap_triple,
///     exhaustive_triples_xyx,
///     exhaustive_triples_xyx_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [2, output_type_ys_1]],
///     [Y, J, ys, ys_done, [1, output_type_xs_2]]
/// );
///
/// // We are generating triples of `char`, `i8`, and `char` using two input iterators. The first
/// // iterator, `xs`, produces all ASCII `char`s, and the second, `ys`, produces the three numbers
/// // 0, 1, and 2. The function we're using is `exhaustive_triples_xyx_custom_output`, meaning that
/// // the first element of the output triples will be taken from `xs`, the second element from
/// // `ys`, and the third also from `xs`.
/// //
/// // The third element has a tiny output type, so it will grow more slowly than the other two
/// // elements (though it doesn't look that way from the first few tuples).
/// let ts = exhaustive_triples_xyx_custom_output(
///     exhaustive_ascii_chars(),
///     0..3,
///     BitDistributorOutputType::normal(1),
///     BitDistributorOutputType::normal(1),
///     BitDistributorOutputType::tiny(),
/// );
/// assert_eq!(
///     ts.take(20).collect_vec(),
///     &[
///         ('a', 0, 'a'),
///         ('a', 0, 'b'),
///         ('a', 0, 'c'),
///         ('a', 0, 'd'),
///         ('a', 1, 'a'),
///         ('a', 1, 'b'),
///         ('a', 1, 'c'),
///         ('a', 1, 'd'),
///         ('a', 0, 'e'),
///         ('a', 0, 'f'),
///         ('a', 0, 'g'),
///         ('a', 0, 'h'),
///         ('a', 1, 'e'),
///         ('a', 1, 'f'),
///         ('a', 1, 'g'),
///         ('a', 1, 'h'),
///         ('b', 0, 'a'),
///         ('b', 0, 'b'),
///         ('b', 0, 'c'),
///         ('b', 0, 'd')
///     ]
/// );
/// ```
///
/// # lex_ordered_unique_quadruples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_ordered_unique_tuples;
/// use malachite_base::vecs::exhaustive::fixed_length_ordered_unique_indices_helper;
/// use std::marker::PhantomData;
///
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     lex_ordered_unique_quadruples,
///     [0, 1, 2, 3]
/// );
///
/// let xss = lex_ordered_unique_quadruples(1..=6).collect_vec();
/// assert_eq!(
///     xss.into_iter().collect_vec().as_slice(),
///     &[
///         (1, 2, 3, 4),
///         (1, 2, 3, 5),
///         (1, 2, 3, 6),
///         (1, 2, 4, 5),
///         (1, 2, 4, 6),
///         (1, 2, 5, 6),
///         (1, 3, 4, 5),
///         (1, 3, 4, 6),
///         (1, 3, 5, 6),
///         (1, 4, 5, 6),
///         (2, 3, 4, 5),
///         (2, 3, 4, 6),
///         (2, 3, 5, 6),
///         (2, 4, 5, 6),
///         (3, 4, 5, 6)
///     ]
/// );
/// ```
///
/// # exhaustive_ordered_unique_quadruples
/// ```
/// use itertools::Itertools;
/// use malachite_base::exhaustive_ordered_unique_tuples;
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::vecs::exhaustive::next_bit_pattern;
///
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     exhaustive_ordered_unique_quadruples,
///     [0, 1, 2, 3]
/// );
///
/// let xss = exhaustive_ordered_unique_quadruples(1..=6).collect_vec();
/// assert_eq!(
///     xss.into_iter().collect_vec().as_slice(),
///     &[
///         (1, 2, 3, 4),
///         (1, 2, 3, 5),
///         (1, 2, 4, 5),
///         (1, 3, 4, 5),
///         (2, 3, 4, 5),
///         (1, 2, 3, 6),
///         (1, 2, 4, 6),
///         (1, 3, 4, 6),
///         (2, 3, 4, 6),
///         (1, 2, 5, 6),
///         (1, 3, 5, 6),
///         (2, 3, 5, 6),
///         (1, 4, 5, 6),
///         (2, 4, 5, 6),
///         (3, 4, 5, 6)
///     ]
/// );
/// ```
///
/// # lex_unique_quadruples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_unique_tuples;
/// use malachite_base::vecs::exhaustive::{unique_indices, UniqueIndices};
///
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     lex_unique_quadruples,
///     [0, 1, 2, 3]
/// );
///
/// let xss = lex_unique_quadruples(1..=6).take(20).collect_vec();
/// assert_eq!(
///     xss.into_iter().collect_vec().as_slice(),
///     &[
///         (1, 2, 3, 4),
///         (1, 2, 3, 5),
///         (1, 2, 3, 6),
///         (1, 2, 4, 3),
///         (1, 2, 4, 5),
///         (1, 2, 4, 6),
///         (1, 2, 5, 3),
///         (1, 2, 5, 4),
///         (1, 2, 5, 6),
///         (1, 2, 6, 3),
///         (1, 2, 6, 4),
///         (1, 2, 6, 5),
///         (1, 3, 2, 4),
///         (1, 3, 2, 5),
///         (1, 3, 2, 6),
///         (1, 3, 4, 2),
///         (1, 3, 4, 5),
///         (1, 3, 4, 6),
///         (1, 3, 5, 2),
///         (1, 3, 5, 4)
///     ]
/// );
/// ```
///
/// # exhaustive_unique_quadruples
/// ```
/// use itertools::Itertools;
/// use malachite_base::exhaustive_unique_tuples;
/// use malachite_base::num::iterators::{ruler_sequence, RulerSequence};
/// use malachite_base::tuples::exhaustive::{
///     exhaustive_dependent_pairs, ExhaustiveDependentPairs,
/// };
/// use malachite_base::vecs::exhaustive::{
///     exhaustive_ordered_unique_vecs_fixed_length, ExhaustiveOrderedUniqueCollections,
///     ExhaustiveUniqueVecsGenerator,
/// };
/// use malachite_base::vecs::ExhaustiveVecPermutations;
///
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     exhaustive_unique_quadruples,
///     [0, 1, 2, 3]
/// );
///
/// let xss = exhaustive_unique_quadruples(1..=6).take(20).collect_vec();
/// assert_eq!(
///     xss.into_iter().collect_vec().as_slice(),
///     &[
///         (1, 2, 3, 4),
///         (1, 2, 3, 5),
///         (1, 2, 4, 3),
///         (1, 2, 4, 5),
///         (1, 3, 2, 4),
///         (1, 2, 5, 3),
///         (1, 3, 4, 2),
///         (1, 3, 4, 5),
///         (1, 4, 2, 3),
///         (1, 3, 2, 5),
///         (1, 4, 3, 2),
///         (1, 2, 5, 4),
///         (2, 1, 3, 4),
///         (1, 3, 5, 2),
///         (2, 1, 4, 3),
///         (2, 3, 4, 5),
///         (2, 3, 1, 4),
///         (1, 5, 2, 3),
///         (2, 3, 4, 1),
///         (1, 4, 2, 5)
///     ]
/// );
/// ```
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate tuples randomly.
///
/// # random_pairs
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::tuples::random::random_pairs;
///
/// let ps = random_pairs(
///     EXAMPLE_SEED,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     &|seed| random_char_inclusive_range(seed, 'x', 'z'),
/// );
/// assert_eq!(
///     ps.take(20).collect_vec().as_slice(),
///     &[
///         (1, 'z'),
///         (1, 'x'),
///         (1, 'z'),
///         (1, 'y'),
///         (2, 'x'),
///         (0, 'z'),
///         (0, 'z'),
///         (0, 'z'),
///         (2, 'z'),
///         (0, 'y'),
///         (2, 'x'),
///         (0, 'x'),
///         (2, 'z'),
///         (0, 'z'),
///         (2, 'x'),
///         (2, 'x'),
///         (2, 'y'),
///         (1, 'y'),
///         (0, 'x'),
///         (2, 'x')
///     ]
/// );
/// ```
///
/// # random_pairs_from_single
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::tuples::random::random_pairs_from_single;
///
/// let ps = random_pairs_from_single(random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 0, 2));
/// assert_eq!(
///     ps.take(20).collect_vec().as_slice(),
///     &[
///         (1, 0),
///         (1, 2),
///         (1, 1),
///         (0, 1),
///         (0, 2),
///         (1, 0),
///         (1, 2),
///         (2, 0),
///         (1, 0),
///         (2, 2),
///         (2, 1),
///         (0, 2),
///         (2, 1),
///         (1, 1),
///         (0, 0),
///         (2, 0),
///         (2, 2),
///         (1, 0),
///         (1, 1),
///         (0, 2)
///     ]
/// );
/// ```
///
/// # random_triples_xyx
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::{Seed, EXAMPLE_SEED};
/// use malachite_base::random_custom_tuples;
///
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomTriplesXYX,
///     (X, Y, X),
///     random_triples_xyx,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
///     [Y, J, ys, ys_gen, [y_1, x_2]]
/// );
///
/// // We are generating triples of `char`s using two input iterators. The first iterator, `xs`,
/// // produces all ASCII `char`s, and the second, `ys`, produces the three numbers 0, 1, and 2. The
/// // function we're using is `random_triples_xyx`, meaning that the first element of the
/// // output triples will be taken from `xs`, the second element from `ys`, and the third also from
/// // `xs`.
/// let ts = random_triples_xyx(
///     EXAMPLE_SEED,
///     &|seed| random_char_inclusive_range(seed, 'x', 'z'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
/// );
/// assert_eq!(
///     ts.take(20).collect_vec().as_slice(),
///     &[
///         ('y', 2, 'y'),
///         ('y', 0, 'y'),
///         ('z', 2, 'x'),
///         ('x', 1, 'x'),
///         ('z', 0, 'x'),
///         ('z', 2, 'x'),
///         ('z', 2, 'x'),
///         ('z', 2, 'z'),
///         ('z', 2, 'y'),
///         ('x', 1, 'z'),
///         ('z', 0, 'x'),
///         ('y', 0, 'z'),
///         ('y', 2, 'z'),
///         ('x', 2, 'z'),
///         ('z', 0, 'y'),
///         ('z', 0, 'y'),
///         ('y', 1, 'x'),
///         ('z', 1, 'z'),
///         ('x', 0, 'z'),
///         ('z', 0, 'x')
///     ]
/// );
/// ```
///
/// # random_ordered_unique_quadruples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::random_ordered_unique_tuples;
/// use malachite_base::sets::random::{
///     random_b_tree_sets_fixed_length, RandomBTreeSetsFixedLength,
/// };
///
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     random_ordered_unique_quadruples,
///     [0, 1, 2, 3]
/// );
///
/// let qs = random_ordered_unique_quadruples(random_unsigned_inclusive_range::<u8>(
///     EXAMPLE_SEED,
///     1,
///     10,
/// ));
/// assert_eq!(
///     qs.take(20).collect_vec().as_slice(),
///     &[
///         (2, 5, 6, 8),
///         (3, 5, 7, 9),
///         (1, 2, 6, 8),
///         (3, 4, 6, 7),
///         (3, 6, 9, 10),
///         (4, 6, 8, 10),
///         (3, 6, 8, 10),
///         (2, 5, 9, 10),
///         (2, 3, 8, 10),
///         (1, 3, 7, 8),
///         (1, 2, 6, 10),
///         (2, 5, 8, 9),
///         (1, 8, 9, 10),
///         (1, 3, 7, 8),
///         (2, 3, 4, 5),
///         (1, 3, 4, 8),
///         (3, 6, 7, 9),
///         (5, 6, 7, 8),
///         (3, 4, 5, 9),
///         (4, 6, 9, 10)
///     ]
/// );
/// ```
///
/// # random_unique_quadruples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::random_unique_tuples;
/// use std::collections::HashMap;
/// use std::hash::Hash;
///
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     random_unique_quadruples,
///     [0, 1, 2, 3]
/// );
///
/// let qs = random_unique_quadruples(random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 1, 10));
/// assert_eq!(
///     qs.take(20).collect_vec().as_slice(),
///     &[
///         (2, 8, 6, 5),
///         (7, 5, 3, 9),
///         (2, 8, 6, 1),
///         (3, 7, 4, 6),
///         (3, 10, 6, 9),
///         (6, 10, 4, 8),
///         (6, 10, 8, 3),
///         (10, 2, 9, 5),
///         (8, 10, 2, 3),
///         (8, 1, 7, 3),
///         (2, 6, 1, 10),
///         (9, 5, 8, 2),
///         (8, 1, 9, 10),
///         (7, 3, 8, 1),
///         (3, 2, 5, 4),
///         (3, 8, 4, 1),
///         (9, 7, 6, 3),
///         (5, 7, 8, 6),
///         (5, 3, 9, 4),
///         (9, 10, 4, 6)
///     ]
/// );
/// ```
pub mod random;
