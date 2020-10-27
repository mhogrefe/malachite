use std::str::FromStr;

use num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
use random::Seed;

/// Inserts several copies of a value at the left (beginning) of a `Vec`.
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

/// Deletes several values from the left (beginning) of a `Vec`.
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

/// Converts a `&str` to an `Vec<T>`, where `T` implements `FromStr`.
///
/// If the `&str` does not represent a valid `Option<T>`, `None` is returned.
///
/// If `T` does not implement `FromStr`, try using `vec_from_str_custom` instead.
///
/// Substrings representing `T`s may contain commas. Sometimes this may lead to ambiguities: for
/// example, the two `Vec<&str>`s `vec!["a, b"]` and `vec!["a", "b"]` both have the string
/// representation `"[a, b]"`. The parser is greedy, so it will interpet this string as
/// `vec!["a", "b"]`.
///
/// # Worst-case complexity
///
/// $T(n) = O(n + T^\prime(n))$
///
/// $M(n) = O(n + M^\prime(n))$
///
/// where $T$ is time, $M$ is additional memory, $n$ = `src.len()`, and $T^\prime$ and $M^\prime$
/// are the time and memory complexity functions of `T::from_str`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::vec_from_str;
/// use malachite_base::nevers::Never;
///
/// assert_eq!(vec_from_str::<Never>("[]"), Some(vec![]));
/// assert_eq!(vec_from_str::<u32>("[5, 6, 7]"), Some(vec![5, 6, 7]));
/// assert_eq!(vec_from_str::<bool>("[false, false, true]"), Some(vec![false, false, true]));
/// assert_eq!(vec_from_str::<bool>("[false, false, true"), None);
/// ```
#[inline]
pub fn vec_from_str<T: FromStr>(src: &str) -> Option<Vec<T>> {
    vec_from_str_custom(&(|t| t.parse().ok()), src)
}

/// Converts a `&str` to an `Vec<T>`, given a function to parse a `&str` into a `T`.
///
/// If the `&str` does not represent a valid `Option<T>`, `None` is returned.
///
/// If `f` just uses `T::from_str`, you can use `vec_from_str` instead.
///
/// Substrings representing `T`s may contain commas. Sometimes this may lead to ambiguities: for
/// example, the two `Vec<&str>`s `vec!["a, b"]` and `vec!["a", "b"]` both have the string
/// representation `"[a, b]"`. The parser is greedy, so it will interpet this string as
/// `vec!["a", "b"]`.
///
/// # Worst-case complexity
///
/// $T(n) = O(n + \max\sum_{p \in P}T^\prime(p))$, where the maximum is taken over all multisets $P$
/// that sum to $n$.
///
/// $M(n) = O(\max\sum_{p \in P}M^\prime(p))$, where the maximum is taken over all multisets $P$
/// that sum to $n$.
///
/// where $T$ is time, $M$ is additional memory, $n$ = `src.len()`, and $T^\prime$ and $M^\prime$
/// are the time and memory complexity functions of `f`.
///
/// # Examples
/// ```
/// use malachite_base::options::option_from_str;
/// use malachite_base::orderings::ordering_from_str;
/// use malachite_base::vecs::{vec_from_str, vec_from_str_custom};
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     vec_from_str_custom(&ordering_from_str, "[Less, Greater]"),
///     Some(vec![Ordering::Less, Ordering::Greater]),
/// );
/// assert_eq!(
///     vec_from_str_custom(&option_from_str, "[Some(false), None]"),
///     Some(vec![Some(false), None]),
/// );
/// assert_eq!(
///     vec_from_str_custom(&vec_from_str, "[[], [3], [2, 5]]"),
///     Some(vec![vec![], vec![3], vec![2, 5]]),
/// );
/// assert_eq!(vec_from_str_custom(&option_from_str::<bool>, "[Some(fals), None]"), None);
/// ```
pub fn vec_from_str_custom<T>(f: &dyn Fn(&str) -> Option<T>, src: &str) -> Option<Vec<T>> {
    if src.is_empty() {
        return None;
    }
    let mut tokens = src.split(", ").collect::<Vec<&str>>();
    let last_token_index = tokens.len() - 1;
    if tokens[0].is_empty() {
        return None;
    }
    let mut cleaned_first_token = String::from(tokens[0]);
    if cleaned_first_token.remove(0) != '[' {
        return None;
    }
    tokens[0] = &cleaned_first_token;
    let mut cleaned_last_token = String::from(tokens[last_token_index]);
    if cleaned_last_token.pop() != Some(']') {
        return None;
    }
    tokens[last_token_index] = &cleaned_last_token;
    let mut xs = Vec::new();
    let mut buffer = String::new();
    for token in &tokens {
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

/// Uniformly generates a random value from a nonempty `Vec`.
#[derive(Clone, Debug)]
pub struct RandomValuesFromVec<T: Clone> {
    xs: Vec<T>,
    indices: RandomUnsignedsLessThan<usize>,
}

impl<T: Clone> Iterator for RandomValuesFromVec<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(self.xs[self.indices.next().unwrap()].clone())
    }
}

/// Uniformly generates a random value from a nonempty `Vec`.
///
/// The iterator owns the data. It may be more convenient for the iterator to return references to a
/// pre-existing slice, in which case you may use `random_values_from_slice` instead.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let xs = vec![2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_vec(EXAMPLE_SEED, xs).take(10).collect::<Vec<_>>(),
///     &[3, 7, 3, 5, 11, 3, 5, 11, 2, 2]
/// );
/// ```
#[inline]
pub fn random_values_from_vec<T: Clone>(seed: Seed, xs: Vec<T>) -> RandomValuesFromVec<T> {
    if xs.is_empty() {
        panic!("empty Vec");
    }
    let indices = random_unsigneds_less_than(seed, xs.len());
    RandomValuesFromVec { xs, indices }
}

/// This module contains iterators that generate `Vec`s without repetition.
///
/// Here are usage examples of the macro-generated functions:
///
/// # lex_length_[n]_vecs
/// ```
/// use malachite_base::vecs::exhaustive::lex_length_2_vecs;
///
/// let xss = lex_length_2_vecs(
///     ['a', 'b', 'c'].iter().cloned(),
///     ['x', 'y', 'z'].iter().cloned()
/// ).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &['a', 'x'], &['a', 'y'], &['a', 'z'], &['b', 'x'], &['b', 'y'], &['b', 'z'],
///         &['c', 'x'], &['c', 'y'], &['c', 'z']
///     ]
/// );
/// ```
///
/// # lex_fixed_length_vecs_[m]_inputs
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::vecs::exhaustive::lex_fixed_length_vecs_2_inputs;
///
/// // We are generating length-3 `Vec`s of `char`s using two input iterators. The first iterator
/// // (with index 0) produces all ASCII `char`s, and the second (index 1) produces the three
/// // `char`s `'x'`, `'y'`, and `'z'`. The elements of `output_types` are 0, 1, and 0, meaning that
/// // the first element of the output `Vec`s will be taken from iterator 0, the second element from
/// // iterator 1, and the third also from iterator 0.
/// let xss = lex_fixed_length_vecs_2_inputs(
///     exhaustive_ascii_chars(),
///     ['x', 'y', 'z'].iter().cloned(),
///     &[0, 1, 0],
/// );
/// let xss_prefix = xss.take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss_prefix.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &['a', 'x', 'a'], &['a', 'x', 'b'], &['a', 'x', 'c'], &['a', 'x', 'd'],
///         &['a', 'x', 'e'], &['a', 'x', 'f'], &['a', 'x', 'g'], &['a', 'x', 'h'],
///         &['a', 'x', 'i'], &['a', 'x', 'j'], &['a', 'x', 'k'], &['a', 'x', 'l'],
///         &['a', 'x', 'm'], &['a', 'x', 'n'], &['a', 'x', 'o'], &['a', 'x', 'p'],
///         &['a', 'x', 'q'], &['a', 'x', 'r'], &['a', 'x', 's'], &['a', 'x', 't']
///     ]
/// );
/// ```
///
/// # exhaustive_length_[n]_vecs
/// ```
/// use malachite_base::vecs::exhaustive::exhaustive_length_2_vecs;
///
/// let xss = exhaustive_length_2_vecs(
///     ['a', 'b', 'c'].iter().cloned(),
///     ['x', 'y', 'z'].iter().cloned()
/// ).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &['a', 'x'], &['a', 'y'], &['b', 'x'], &['b', 'y'], &['a', 'z'], &['b', 'z'],
///         &['c', 'x'], &['c', 'y'], &['c', 'z']
///     ]
/// );
/// ```
///
/// # exhaustive_fixed_length_vecs_[m]_inputs
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_2_inputs;
///
/// // We are generating length-3 `Vec`s of `char`s using two input iterators. The first iterator
/// // (with index 0) produces all ASCII `char`s, and the second (index 1) produces the three
/// // `char`s `'x'`, `'y'`, and `'z'`. The elements of `output_types` have the indices 0, 1, and 0,
/// // meaning that the first element of the output `Vec`s will be taken from iterator 0, the second
/// // element from iterator 1, and the third also from iterator 0. The third element has a tiny
/// // output type, so it will grow more slowly than the other two elements (though it doesn't look
/// // that way from the first few `Vec`s).
/// let xss = exhaustive_fixed_length_vecs_2_inputs(
///     exhaustive_ascii_chars(),
///     ['x', 'y', 'z'].iter().cloned(),
///     &[
///         (BitDistributorOutputType::normal(1), 0),
///         (BitDistributorOutputType::normal(1), 1),
///         (BitDistributorOutputType::tiny(), 0),
///     ],
/// );
/// let xss_prefix = xss.take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss_prefix.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &['a', 'x', 'a'], &['a', 'x', 'b'], &['a', 'x', 'c'], &['a', 'x', 'd'],
///         &['a', 'y', 'a'], &['a', 'y', 'b'], &['a', 'y', 'c'], &['a', 'y', 'd'],
///         &['a', 'x', 'e'], &['a', 'x', 'f'], &['a', 'x', 'g'], &['a', 'x', 'h'],
///         &['a', 'y', 'e'], &['a', 'y', 'f'], &['a', 'y', 'g'], &['a', 'y', 'h'],
///         &['b', 'x', 'a'], &['b', 'x', 'b'], &['b', 'x', 'c'], &['b', 'x', 'd']
///     ]
/// );
/// ```
pub mod exhaustive;
