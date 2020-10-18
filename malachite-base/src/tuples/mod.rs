/// Generates all singletons (1-element tuples) with values from a given iterator.
///
/// This `struct` is created by the `singletons` method. See its documentation for more.
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
///
/// Same as the time and additional memory complexity of iterating `xs`.
///
/// # Examples
/// ```
/// use malachite_base::tuples::singletons;
///
/// assert_eq!(singletons([1, 2, 3].iter().cloned()).collect::<Vec<_>>(), &[(1,), (2,), (3,)]);
/// ```
#[inline]
pub fn singletons<I: Iterator>(xs: I) -> Singletons<I> {
    Singletons { xs }
}

/// This module contains iterators that generate tuples without repetition.
///
/// Here are usage examples of the macro-generated functions:
///
/// # lex_[n]_tuples
/// ```
/// use malachite_base::tuples::exhaustive::lex_pairs;
///
/// assert_eq!(
///     lex_pairs('a'..'f', 0..3).collect::<Vec<_>>(),
///     &[
///         ('a', 0), ('a', 1), ('a', 2), ('b', 0), ('b', 1), ('b', 2), ('c', 0), ('c', 1),
///         ('c', 2), ('d', 0), ('d', 1), ('d', 2), ('e', 0), ('e', 1), ('e', 2)
///     ]
/// );
/// ```
///
/// # lex_[n]_tuples_from_single
/// ```
/// use malachite_base::tuples::exhaustive::lex_pairs_from_single;
///
/// assert_eq!(
///     lex_pairs_from_single(0..3).collect::<Vec<_>>(),
///     &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)]
/// );
/// ```
///
/// # lex_custom_tuples
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::tuples::exhaustive::lex_triples_xyx;
///
/// // We are generating triples of chars using two input iterators. The first iterator, `xs`,
/// // produces all ASCII chars, and the second, `ys`, produces the three chars `'x'`, `'y'`, and
/// // `'z'`. The function we're using is `lex_triples_xyx`, meaning that the first element of the
/// // output triples will be taken from `xs`, the second element from `ys`, and the third also from
/// // `xs`.
/// let ts = lex_triples_xyx(exhaustive_ascii_chars(), ['x', 'y', 'z'].iter().cloned());
/// assert_eq!(
///     ts.take(20).collect::<Vec<_>>(),
///     &[
///         ('a', 'x', 'a'), ('a', 'x', 'b'), ('a', 'x', 'c'), ('a', 'x', 'd'), ('a', 'x', 'e'),
///         ('a', 'x', 'f'), ('a', 'x', 'g'), ('a', 'x', 'h'), ('a', 'x', 'i'), ('a', 'x', 'j'),
///         ('a', 'x', 'k'), ('a', 'x', 'l'), ('a', 'x', 'm'), ('a', 'x', 'n'), ('a', 'x', 'o'),
///         ('a', 'x', 'p'), ('a', 'x', 'q'), ('a', 'x', 'r'), ('a', 'x', 's'), ('a', 'x', 't')
///     ]
/// );
/// ```
pub mod exhaustive;
/// This module contains iterators that generate tuples randomly.
pub mod random;
