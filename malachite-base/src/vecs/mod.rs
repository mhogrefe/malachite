// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(feature = "random")]
use crate::num::conversion::traits::ExactFrom;
#[cfg(feature = "random")]
use crate::num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
#[cfg(feature = "random")]
use crate::random::Seed;
use crate::slices::advance_indices;
use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;
#[cfg(feature = "random")]
use rand::prelude::SliceRandom;
#[cfg(feature = "random")]
use rand_chacha::ChaCha20Rng;

/// Inserts several copies of a value at the left (beginning) of a [`Vec`].
///
/// Using this function is more efficient than inserting the values one by one.
///
/// # Worst-case complexity
/// $T(n) = O(n + m)$
///
/// $M(n) = O(n + m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ = `xs.len()` before the function is called, and
/// $m$ = `pad_size`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::vec_pad_left;
///
/// let mut xs = vec![1, 2, 3];
/// vec_pad_left::<u32>(&mut xs, 5, 10);
/// assert_eq!(xs, [10, 10, 10, 10, 10, 1, 2, 3]);
/// ```
pub fn vec_pad_left<T: Clone>(xs: &mut Vec<T>, pad_size: usize, pad_value: T) {
    let old_len = xs.len();
    xs.resize(old_len + pad_size, pad_value);
    for i in (0..old_len).rev() {
        xs.swap(i, i + pad_size);
    }
}

/// Deletes several values from the left (beginning) of a [`Vec`].
///
/// Using this function is more efficient than deleting the values one by one.
///
/// # Worst-case complexity
/// $T(n) = O(\operatorname{max}(1, n - m))$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, $n$ = `xs.len()` before the function is called, and
/// $m$ = `delete_size`.
///
/// # Panics
/// Panics if `delete_size` is greater than `xs.len()`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::vec_delete_left;
///
/// let mut xs = vec![1, 2, 3, 4, 5];
/// vec_delete_left::<u32>(&mut xs, 3);
/// assert_eq!(xs, [4, 5]);
/// ```
pub fn vec_delete_left<T: Copy>(xs: &mut Vec<T>, delete_size: usize) {
    let old_len = xs.len();
    xs.copy_within(delete_size..old_len, 0);
    xs.truncate(old_len - delete_size);
}

/// Converts a string to an `Vec<T>`, where `T` implements [`FromStr`].
///
/// If the string does not represent a valid `Vec<T>`, `None` is returned.
///
/// If `T` does not implement [`FromStr`], try using [`vec_from_str_custom`] instead.
///
/// Substrings representing `T`s may contain commas. Sometimes this may lead to ambiguities: for
/// example, the two `Vec<&str>`s `vec!["a, b"]` and `vec!["a", "b"]` both have the string
/// representation `"[a, b]"`. The parser is greedy, so it will interpet this string as `vec!["a",
/// "b"]`.
///
/// # Examples
/// ```
/// use malachite_base::nevers::Never;
/// use malachite_base::vecs::vec_from_str;
///
/// assert_eq!(vec_from_str::<Never>("[]"), Some(vec![]));
/// assert_eq!(vec_from_str("[5, 6, 7]"), Some(vec![5, 6, 7]));
/// assert_eq!(
///     vec_from_str("[false, false, true]"),
///     Some(vec![false, false, true])
/// );
/// assert_eq!(vec_from_str::<bool>("[false, false, true"), None);
/// ```
#[inline]
pub fn vec_from_str<T: FromStr>(src: &str) -> Option<Vec<T>> {
    vec_from_str_custom(&(|t| t.parse().ok()), src)
}

/// Converts a string to an `Vec<T>`, given a function to parse a string into a `T`.
///
/// If the string does not represent a valid `Option<T>`, `None` is returned.
///
/// If `f` just uses [`FromStr::from_str`], you can use [`vec_from_str`] instead.
///
/// Substrings representing `T`s may contain commas. Sometimes this may lead to ambiguities: for
/// example, the two `Vec<&str>`s `vec!["a, b"]` and `vec!["a", "b"]` both have the string
/// representation `"[a, b]"`. The parser is greedy, so it will interpet this string as `vec!["a",
/// "b"]`.
///
/// # Examples
/// ```
/// use malachite_base::options::option_from_str;
/// use malachite_base::orderings::ordering_from_str;
/// use malachite_base::vecs::{vec_from_str, vec_from_str_custom};
/// use std::cmp::Ordering::*;
///
/// assert_eq!(
///     vec_from_str_custom(&ordering_from_str, "[Less, Greater]"),
///     Some(vec![Less, Greater]),
/// );
/// assert_eq!(
///     vec_from_str_custom(&option_from_str, "[Some(false), None]"),
///     Some(vec![Some(false), None]),
/// );
/// assert_eq!(
///     vec_from_str_custom(&vec_from_str, "[[], [3], [2, 5]]"),
///     Some(vec![vec![], vec![3], vec![2, 5]]),
/// );
/// assert_eq!(
///     vec_from_str_custom(&option_from_str::<bool>, "[Some(fals), None]"),
///     None
/// );
/// ```
pub fn vec_from_str_custom<T>(f: &dyn Fn(&str) -> Option<T>, src: &str) -> Option<Vec<T>> {
    if !src.starts_with('[') || !src.ends_with(']') {
        return None;
    }
    let mut xs = Vec::new();
    let mut buffer = String::new();
    for token in src[1..src.len() - 1].split(", ") {
        if !buffer.is_empty() {
            buffer.push_str(", ");
        }
        buffer.push_str(token);
        if let Some(x) = f(&buffer) {
            xs.push(x);
            buffer.clear();
        }
    }
    if buffer.is_empty() {
        Some(xs)
    } else {
        None
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random value from a nonempty [`Vec`].
///
/// This `struct` is created by [`random_values_from_vec`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomValuesFromVec<T: Clone> {
    xs: Vec<T>,
    indices: RandomUnsignedsLessThan<u64>,
}

#[cfg(feature = "random")]
impl<T: Clone> Iterator for RandomValuesFromVec<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(self.xs[usize::exact_from(self.indices.next().unwrap())].clone())
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random value from a nonempty [`Vec`].
///
/// The iterator owns the data. It may be more convenient for the iterator to return references to a
/// pre-existing slice, in which case you may use
/// [`random_values_from_slice`](crate::slices::random_values_from_slice) instead.
///
/// The output length is infinite.
///
/// $P(x) = 1/n$, where $n$ is `xs.len()`.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let xs = vec![2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_vec(EXAMPLE_SEED, xs)
///         .take(10)
///         .collect_vec(),
///     &[3, 7, 3, 5, 11, 3, 5, 11, 2, 2]
/// );
/// ```
#[inline]
pub fn random_values_from_vec<T: Clone>(seed: Seed, xs: Vec<T>) -> RandomValuesFromVec<T> {
    assert!(!xs.is_empty(), "empty Vec");
    let indices = random_unsigneds_less_than(seed, u64::exact_from(xs.len()));
    RandomValuesFromVec { xs, indices }
}

/// Generates every permutation of a [`Vec`].
///
/// This `struct` is created by [`exhaustive_vec_permutations`]; see its documentation for more.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExhaustiveVecPermutations<T: Clone> {
    xs: Vec<T>,
    indices: Vec<usize>,
    done: bool,
}

impl<T: Clone> Iterator for ExhaustiveVecPermutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.done {
            None
        } else {
            let out = Some(self.indices.iter().map(|&i| self.xs[i].clone()).collect());
            self.done = advance_indices(&mut self.indices);
            out
        }
    }
}

/// Generates every permutation of a [`Vec`].
///
/// The permutations are [`Vec`]s of cloned items. It may be more convenient for the iterator to
/// return references to a slice, in which case you may use
/// [`exhaustive_slice_permutations`](crate::slices::exhaustive_slice_permutations) instead.
///
/// The permutations are generated in lexicographic order with respect to the ordering in the
/// [`Vec`].
///
/// The output length is $n!$, where $n$ is `xs.len()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive_vec_permutations;
///
/// let css: Vec<String> = exhaustive_vec_permutations(vec!['a', 'b', 'c', 'd'])
///     .map(|ds| ds.into_iter().collect())
///     .collect();
/// assert_eq!(
///     css.iter().map(String::as_str).collect_vec().as_slice(),
///     [
///         "abcd", "abdc", "acbd", "acdb", "adbc", "adcb", "bacd", "badc", "bcad", "bcda", "bdac",
///         "bdca", "cabd", "cadb", "cbad", "cbda", "cdab", "cdba", "dabc", "dacb", "dbac", "dbca",
///         "dcab", "dcba"
///     ]
/// );
/// ```
pub fn exhaustive_vec_permutations<T: Clone>(xs: Vec<T>) -> ExhaustiveVecPermutations<T> {
    let len = xs.len();
    ExhaustiveVecPermutations {
        xs,
        indices: (0..len).collect(),
        done: false,
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random [`Vec`] of values cloned from an original [`Vec`].
///
/// This `struct` is created by [`random_vec_permutations`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomVecPermutations<T: Clone> {
    xs: Vec<T>,
    indices: Vec<usize>,
    rng: ChaCha20Rng,
}

#[cfg(feature = "random")]
impl<T: Clone> Iterator for RandomVecPermutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        self.indices.shuffle(&mut self.rng);
        Some(self.indices.iter().map(|&i| self.xs[i].clone()).collect())
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random [`Vec`] of values cloned from an original [`Vec`].
///
/// The permutations are [`Vec`]s of cloned items. It may be more convenient for the iterator to
/// return references to a slice, in which case you may use
/// [`random_slice_permutations`](crate::slices::random_slice_permutations) instead.
///
/// The output length is infinite.
///
/// $P(p) = 1/n!$, where $n$ is `xs.len()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random_vec_permutations;
///
/// let css: Vec<String> = random_vec_permutations(EXAMPLE_SEED, vec!['a', 'b', 'c', 'd'])
///     .take(20)
///     .map(|ds| ds.into_iter().collect())
///     .collect();
/// assert_eq!(
///     css.iter().map(String::as_str).collect_vec().as_slice(),
///     [
///         "cadb", "cbad", "cadb", "badc", "acdb", "cbad", "dabc", "dbca", "cdba", "cdab", "bacd",
///         "cabd", "adbc", "cdab", "dcab", "abcd", "abcd", "dacb", "bcad", "adcb"
///     ]
/// );
/// ```
pub fn random_vec_permutations<T: Clone>(seed: Seed, xs: Vec<T>) -> RandomVecPermutations<T> {
    let len = xs.len();
    RandomVecPermutations {
        xs,
        indices: (0..len).collect(),
        rng: seed.get_rng(),
    }
}

/// Iterators that generate [`Vec`]s without repetition.
///
/// # lex_vecs_length_2
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_vecs_length_2;
///
/// let xss = lex_vecs_length_2(
///     ['a', 'b', 'c'].iter().cloned(),
///     ['x', 'y', 'z'].iter().cloned(),
/// )
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &['a', 'x'],
///         &['a', 'y'],
///         &['a', 'z'],
///         &['b', 'x'],
///         &['b', 'y'],
///         &['b', 'z'],
///         &['c', 'x'],
///         &['c', 'y'],
///         &['c', 'z']
///     ]
/// );
/// ```
///
/// # lex_vecs_fixed_length_2_inputs
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::vecs::exhaustive::lex_vecs_fixed_length_2_inputs;
///
/// // We are generating length-3 `Vec`s of `char`s using two input iterators. The first iterator
/// // (with index 0) produces all ASCII `char`s, and the second (index 1) produces the three
/// // `char`s `'x'`, `'y'`, and `'z'`. The elements of `output_types` are 0, 1, and 0, meaning that
/// // the first element of the output `Vec`s will be taken from iterator 0, the second element from
/// // iterator 1, and the third also from iterator 0.
/// let xss = lex_vecs_fixed_length_2_inputs(
///     exhaustive_ascii_chars(),
///     ['x', 'y', 'z'].iter().cloned(),
///     &[0, 1, 0],
/// );
/// let xss_prefix = xss.take(20).collect_vec();
/// assert_eq!(
///     xss_prefix
///         .iter()
///         .map(Vec::as_slice)
///         .collect_vec()
///         .as_slice(),
///     &[
///         &['a', 'x', 'a'],
///         &['a', 'x', 'b'],
///         &['a', 'x', 'c'],
///         &['a', 'x', 'd'],
///         &['a', 'x', 'e'],
///         &['a', 'x', 'f'],
///         &['a', 'x', 'g'],
///         &['a', 'x', 'h'],
///         &['a', 'x', 'i'],
///         &['a', 'x', 'j'],
///         &['a', 'x', 'k'],
///         &['a', 'x', 'l'],
///         &['a', 'x', 'm'],
///         &['a', 'x', 'n'],
///         &['a', 'x', 'o'],
///         &['a', 'x', 'p'],
///         &['a', 'x', 'q'],
///         &['a', 'x', 'r'],
///         &['a', 'x', 's'],
///         &['a', 'x', 't']
///     ]
/// );
/// ```
///
/// # exhaustive_vecs_length_2
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_length_2;
///
/// let xss = exhaustive_vecs_length_2(
///     ['a', 'b', 'c'].iter().cloned(),
///     ['x', 'y', 'z'].iter().cloned(),
/// )
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &['a', 'x'],
///         &['a', 'y'],
///         &['b', 'x'],
///         &['b', 'y'],
///         &['a', 'z'],
///         &['b', 'z'],
///         &['c', 'x'],
///         &['c', 'y'],
///         &['c', 'z']
///     ]
/// );
/// ```
///
/// # exhaustive_vecs_fixed_length_2_inputs
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_fixed_length_2_inputs;
///
/// // We are generating length-3 `Vec`s of `char`s using two input iterators. The first iterator
/// // (with index 0) produces all ASCII `char`s, and the second (index 1) produces the three
/// // `char`s `'x'`, `'y'`, and `'z'`. The elements of `output_types` have the indices 0, 1, and 0,
/// // meaning that the first element of the output `Vec`s will be taken from iterator 0, the second
/// // element from iterator 1, and the third also from iterator 0. The third element has a tiny
/// // output type, so it will grow more slowly than the other two elements (though it doesn't look
/// // that way from the first few `Vec`s).
/// let xss = exhaustive_vecs_fixed_length_2_inputs(
///     exhaustive_ascii_chars(),
///     ['x', 'y', 'z'].iter().cloned(),
///     &[
///         (BitDistributorOutputType::normal(1), 0),
///         (BitDistributorOutputType::normal(1), 1),
///         (BitDistributorOutputType::tiny(), 0),
///     ],
/// );
/// let xss_prefix = xss.take(20).collect_vec();
/// assert_eq!(
///     xss_prefix
///         .iter()
///         .map(Vec::as_slice)
///         .collect_vec()
///         .as_slice(),
///     &[
///         &['a', 'x', 'a'],
///         &['a', 'x', 'b'],
///         &['a', 'x', 'c'],
///         &['a', 'x', 'd'],
///         &['a', 'y', 'a'],
///         &['a', 'y', 'b'],
///         &['a', 'y', 'c'],
///         &['a', 'y', 'd'],
///         &['a', 'x', 'e'],
///         &['a', 'x', 'f'],
///         &['a', 'x', 'g'],
///         &['a', 'x', 'h'],
///         &['a', 'y', 'e'],
///         &['a', 'y', 'f'],
///         &['a', 'y', 'g'],
///         &['a', 'y', 'h'],
///         &['b', 'x', 'a'],
///         &['b', 'x', 'b'],
///         &['b', 'x', 'c'],
///         &['b', 'x', 'd']
///     ]
/// );
/// ```
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Vec`]s randomly.
///
/// # random_vecs_length_2
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_length_2;
///
/// let xss = random_vecs_length_2(
///     EXAMPLE_SEED,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_char_inclusive_range(seed, 'x', 'z'),
/// )
/// .take(20)
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &['b', 'z'],
///         &['b', 'x'],
///         &['b', 'z'],
///         &['b', 'y'],
///         &['c', 'x'],
///         &['a', 'z'],
///         &['a', 'z'],
///         &['a', 'z'],
///         &['c', 'z'],
///         &['a', 'y'],
///         &['c', 'x'],
///         &['a', 'x'],
///         &['c', 'z'],
///         &['a', 'z'],
///         &['c', 'x'],
///         &['c', 'x'],
///         &['c', 'y'],
///         &['b', 'y'],
///         &['a', 'x'],
///         &['c', 'x']
///     ]
/// );
/// ```
///
/// # random_vecs_fixed_length_2_inputs
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::{random_ascii_chars, random_char_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_fixed_length_2_inputs;
///
/// // We are generating length-3 `Vec`s of `char`s using two input iterators. The first iterator
/// // (with index 0) produces random ASCII `char`s, and the second (index 1) produces the three
/// // `char`s `'x'`, `'y'`, and `'z'`, uniformly at random. The elements of `output_types` are 0,
/// // 1, and 0, meaning that the first element of the output `Vec`s will be taken from iterator 0,
/// // the second element from iterator 1, and the third also from iterator 0.
/// let xss = random_vecs_fixed_length_2_inputs(
///     EXAMPLE_SEED,
///     &random_ascii_chars,
///     &|seed| random_char_inclusive_range(seed, 'x', 'z'),
///     &[0, 1, 0],
/// )
/// .take(20)
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &['U', 'z', '\u{16}'],
///         &[' ', 'x', 'D'],
///         &['<', 'z', ']'],
///         &['a', 'y', 'e'],
///         &['_', 'x', 'M'],
///         &[',', 'z', 'O'],
///         &['\u{1d}', 'z', 'V'],
///         &['(', 'z', '\u{10}'],
///         &['&', 'z', 'U'],
///         &['{', 'y', 'P'],
///         &['-', 'x', 'K'],
///         &['Z', 'x', '\u{4}'],
///         &['X', 'z', '\u{19}'],
///         &['_', 'z', ','],
///         &['\u{1d}', 'x', ','],
///         &['?', 'x', '\''],
///         &['[', 'y', 'N'],
///         &['|', 'y', '}'],
///         &['*', 'x', '\u{15}'],
///         &['z', 'x', 't']
///     ]
/// );
/// ```
pub mod random;
