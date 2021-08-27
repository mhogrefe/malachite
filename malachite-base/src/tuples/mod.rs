/// Generates all singletons (1-element tuples) with values from a given iterator.
///
/// This `struct` is created by the `singletons` function. See its documentation for more.
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
/// # Complexity per iteration
/// Same as the time and additional memory complexity of iterating `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::tuples::singletons;
///
/// assert_eq!(
///     singletons([1, 2, 3].iter().cloned()).collect_vec(),
///     &[(1,), (2,), (3,)]
/// );
/// ```
#[inline]
pub fn singletons<I: Iterator>(xs: I) -> Singletons<I> {
    Singletons { xs }
}

/// This module contains iterators that generate tuples without repetition.
///
/// Here are usage examples of the macro-generated functions:
///
/// # lex_\[n\]_tuples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
/// # lex_\[n\]_tuples_from_single
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::tuples::exhaustive::lex_pairs_from_single;
///
/// assert_eq!(
///     lex_pairs_from_single(0..3).collect_vec(),
///     &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)]
/// );
/// ```
///
/// # lex_custom_tuples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::tuples::exhaustive::lex_triples_xyx;
///
/// // We are generating triples of `char`, `i8`, and `char` using two input iterators. The first
/// // iterator, `xs`, produces all ASCII `char`s, and the second, `ys`, produces the three numbers
/// // 0, 1, and 2. The function we're using is `lex_triples_xyx`, meaning that the first element of
/// // the output triples will be taken from `xs`, the second element from `ys`, and the third also
/// // from `xs`.
/// let ts = lex_triples_xyx(exhaustive_ascii_chars(), 0..3);
/// assert_eq!(
///     ts.take(20).collect_vec(),
///     &[
///         ('a', 0, 'a'),
///         ('a', 0, 'b'),
///         ('a', 0, 'c'),
///         ('a', 0, 'd'),
///         ('a', 0, 'e'),
///         ('a', 0, 'f'),
///         ('a', 0, 'g'),
///         ('a', 0, 'h'),
///         ('a', 0, 'i'),
///         ('a', 0, 'j'),
///         ('a', 0, 'k'),
///         ('a', 0, 'l'),
///         ('a', 0, 'm'),
///         ('a', 0, 'n'),
///         ('a', 0, 'o'),
///         ('a', 0, 'p'),
///         ('a', 0, 'q'),
///         ('a', 0, 'r'),
///         ('a', 0, 's'),
///         ('a', 0, 't')
///     ]
/// );
/// ```
///
/// # exhaustive_[n-tuples]
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::tuples::exhaustive::exhaustive_pairs;
///
/// let xss = exhaustive_pairs(['a', 'b', 'c'].iter().cloned(), 0..3).collect_vec();
/// assert_eq!(
///     xss,
///     &[('a', 0), ('a', 1), ('b', 0), ('b', 1), ('a', 2), ('b', 2), ('c', 0), ('c', 1), ('c', 2)]
/// );
/// ```
///
/// # exhaustive_[n-tuples]_custom_output
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
/// # exhaustive_[n-tuples]_from_single
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
/// # exhaustive_[n-tuples]_1_input
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::tuples::exhaustive::exhaustive_triples_1_input;
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
/// # exhaustive_custom_tuples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::tuples::exhaustive::exhaustive_triples_xyx;
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
/// # exhaustive_custom_tuples_custom_output
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::tuples::exhaustive::exhaustive_triples_xyx_custom_output;
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
pub mod exhaustive;
/// This module contains iterators that generate tuples randomly.
///
/// Here are usage examples of the macro-generated functions:
///
/// # random_[n-tuples]
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
/// # random_[n-tuples]_from_single
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::chars::random::random_char_inclusive_range;
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
/// # random_custom_tuples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::tuples::random::random_triples_xyx;
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
pub mod random;
