use iterators::iterator_cache::IteratorCache;
use num::conversion::traits::ExactFrom;
use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;
use vecs::exhaustive::{
    LexFixedLengthOrderedUniqueCollections, LexOrderedUniqueCollections,
    ShortlexOrderedUniqueCollections,
};

/// Generates `HashSet`s of a given size with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty `HashSet`.
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_fixed_length_hash_sets;
///
/// fn main() {
///     let xss = lex_fixed_length_hash_sets(4, 1..=6).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2, 3, 4},
///             hashset!{1, 2, 3, 5},
///             hashset!{1, 2, 3, 6},
///             hashset!{1, 2, 4, 5},
///             hashset!{1, 2, 4, 6},
///             hashset!{1, 2, 5, 6},
///             hashset!{1, 3, 4, 5},
///             hashset!{1, 3, 4, 6},
///             hashset!{1, 3, 5, 6},
///             hashset!{1, 4, 5, 6},
///             hashset!{2, 3, 4, 5},
///             hashset!{2, 3, 4, 6},
///             hashset!{2, 3, 5, 6},
///             hashset!{2, 4, 5, 6},
///             hashset!{3, 4, 5, 6}
///         ]
///     );
/// }
/// ```
pub fn lex_fixed_length_hash_sets<I: Iterator>(
    k: u64,
    xs: I,
) -> LexFixedLengthOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    LexFixedLengthOrderedUniqueCollections {
        first: true,
        done: false,
        xs: IteratorCache::new(xs),
        indices: (0..usize::exact_from(k)).collect(),
        phantom_i: PhantomData,
        phantom_c: PhantomData,
    }
}

/// Generates `HashSet`s with elements from a single iterator.
///
/// The `HashSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `HashSet`s of length 2 and above will never
/// be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty `HashSet`.
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets;
///
/// fn main() {
///     let xss = shortlex_hash_sets(1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{},
///             hashset!{1},
///             hashset!{2},
///             hashset!{3},
///             hashset!{4},
///             hashset!{1, 2},
///             hashset!{1, 3},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 4},
///             hashset!{3, 4},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 4},
///             hashset!{1, 3, 4},
///             hashset!{2, 3, 4},
///             hashset!{1, 2, 3, 4}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn shortlex_hash_sets<I: Clone + Iterator>(
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    shortlex_hash_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates `HashSet`s with a mininum length, with elements from a single iterator.
///
/// The `HashSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `HashSet`s of length `\max(2, \ell + 1)` and
/// above will never be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets_min_length;
///
/// fn main() {
///     let xss = shortlex_hash_sets_min_length(2, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2},
///             hashset!{1, 3},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 4},
///             hashset!{3, 4},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 4},
///             hashset!{1, 3, 4},
///             hashset!{2, 3, 4},
///             hashset!{1, 2, 3, 4}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn shortlex_hash_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    shortlex_hash_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates `HashSet`s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The `HashSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `HashSet`s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty `HashSet`.
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets_length_range;
///
/// fn main() {
///     let xss = shortlex_hash_sets_length_range(2, 4, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2},
///             hashset!{1, 3},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 4},
///             hashset!{3, 4},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 4},
///             hashset!{1, 3, 4},
///             hashset!{2, 3, 4},
///         ]
///     );
/// }
/// ```
#[inline]
pub fn shortlex_hash_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    shortlex_hash_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates `HashSet`s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The `HashSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `HashSet`s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty `HashSet`.
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets_length_inclusive_range;
///
/// fn main() {
///     let xss = shortlex_hash_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2},
///             hashset!{1, 3},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 4},
///             hashset!{3, 4},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 4},
///             hashset!{1, 3, 4},
///             hashset!{2, 3, 4},
///         ]
///     );
/// }
/// ```
pub fn shortlex_hash_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    ShortlexOrderedUniqueCollections {
        current_len: a,
        max_len: b,
        xs: xs.clone(),
        current_xss: lex_fixed_length_hash_sets(a, xs),
    }
}

/// Generates `HashSet`s with elements from a single iterator.
///
/// The `HashSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty `HashSet`.
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets;
///
/// fn main() {
///     let xss = lex_hash_sets(1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{},
///             hashset!{1},
///             hashset!{1, 2},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 3, 4},
///             hashset!{1, 2, 4},
///             hashset!{1, 3},
///             hashset!{1, 3, 4},
///             hashset!{1, 4},
///             hashset!{2},
///             hashset!{2, 3},
///             hashset!{2, 3, 4},
///             hashset!{2, 4},
///             hashset!{3},
///             hashset!{3, 4},
///             hashset!{4}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn lex_hash_sets<I: Clone + Iterator>(xs: I) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    lex_hash_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates `HashSet`s with a mininum length, with elements from a single iterator.
///
/// The `HashSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_min_length;
///
/// fn main() {
///     let xss = lex_hash_sets_min_length(2, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 3, 4},
///             hashset!{1, 2, 4},
///             hashset!{1, 3},
///             hashset!{1, 3, 4},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 3, 4},
///             hashset!{2, 4},
///             hashset!{3, 4},
///         ]
///     );
/// }
/// ```
#[inline]
pub fn lex_hash_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    lex_hash_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates `HashSet`s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The `HashSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty `HashSet`.
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_length_range;
///
/// fn main() {
///     let xss = lex_hash_sets_length_range(2, 4, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 4},
///             hashset!{1, 3},
///             hashset!{1, 3, 4},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 3, 4},
///             hashset!{2, 4},
///             hashset!{3, 4},
///         ]
///     );
/// }
/// ```
#[inline]
pub fn lex_hash_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    lex_hash_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates `HashSet`s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The `HashSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty `HashSet`.
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_length_inclusive_range;
///
/// fn main() {
///     let xss = lex_hash_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             hashset!{1, 2},
///             hashset!{1, 2, 3},
///             hashset!{1, 2, 4},
///             hashset!{1, 3},
///             hashset!{1, 3, 4},
///             hashset!{1, 4},
///             hashset!{2, 3},
///             hashset!{2, 3, 4},
///             hashset!{2, 4},
///             hashset!{3, 4},
///         ]
///     );
/// }
/// ```
pub fn lex_hash_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    LexOrderedUniqueCollections {
        done: a > b,
        first: true,
        min_len: usize::exact_from(a),
        max_len: usize::exact_from(b),
        xs: IteratorCache::new(xs),
        indices: (0..usize::exact_from(a)).collect(),
        phantom_i: PhantomData,
        phantom_c: PhantomData,
    }
}

/// Generates `BTreeSet`s of a given size with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty `BTreeSet`.
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_fixed_length_b_tree_sets;
///
/// fn main() {
///     let xss = lex_fixed_length_b_tree_sets(4, 1..=6).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2, 3, 4},
///             btreeset!{1, 2, 3, 5},
///             btreeset!{1, 2, 3, 6},
///             btreeset!{1, 2, 4, 5},
///             btreeset!{1, 2, 4, 6},
///             btreeset!{1, 2, 5, 6},
///             btreeset!{1, 3, 4, 5},
///             btreeset!{1, 3, 4, 6},
///             btreeset!{1, 3, 5, 6},
///             btreeset!{1, 4, 5, 6},
///             btreeset!{2, 3, 4, 5},
///             btreeset!{2, 3, 4, 6},
///             btreeset!{2, 3, 5, 6},
///             btreeset!{2, 4, 5, 6},
///             btreeset!{3, 4, 5, 6}
///         ]
///     );
/// }
/// ```
pub fn lex_fixed_length_b_tree_sets<I: Iterator>(
    k: u64,
    xs: I,
) -> LexFixedLengthOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    LexFixedLengthOrderedUniqueCollections {
        first: true,
        done: false,
        xs: IteratorCache::new(xs),
        indices: (0..usize::exact_from(k)).collect(),
        phantom_i: PhantomData,
        phantom_c: PhantomData,
    }
}

/// Generates `BTreeSet`s with elements from a single iterator.
///
/// The `BTreeSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `BTreeSet`s of length 2 and above will never
/// be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty `HashSet`.
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets;
///
/// fn main() {
///     let xss = shortlex_b_tree_sets(1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{},
///             btreeset!{1},
///             btreeset!{2},
///             btreeset!{3},
///             btreeset!{4},
///             btreeset!{1, 2},
///             btreeset!{1, 3},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3, 4},
///             btreeset!{2, 3, 4},
///             btreeset!{1, 2, 3, 4}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn shortlex_b_tree_sets<I: Clone + Iterator>(
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    shortlex_b_tree_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates `BTreeSet`s with a mininum length, with elements from a single iterator.
///
/// The `BTreeSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `BTreeSet`s of length `\max(2, \ell + 1)` and
/// above will never be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets_min_length;
///
/// fn main() {
///     let xss = shortlex_b_tree_sets_min_length(2, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2},
///             btreeset!{1, 3},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3, 4},
///             btreeset!{2, 3, 4},
///             btreeset!{1, 2, 3, 4}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn shortlex_b_tree_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    shortlex_b_tree_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates `BTreeSet`s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The `BTreeSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `BTreeSet`s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty `BTreeSet`.
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets_length_range;
///
/// fn main() {
///     let xss = shortlex_b_tree_sets_length_range(2, 4, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2},
///             btreeset!{1, 3},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3, 4},
///             btreeset!{2, 3, 4},
///         ]
///     );
/// }
/// ```
#[inline]
pub fn shortlex_b_tree_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    shortlex_b_tree_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates `BTreeSet`s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The `BTreeSet`s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, `BTreeSet`s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty `BTreeSet`.
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets_length_inclusive_range;
///
/// fn main() {
///     let xss = shortlex_b_tree_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2},
///             btreeset!{1, 3},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3, 4},
///             btreeset!{2, 3, 4},
///         ]
///     );
/// }
/// ```
pub fn shortlex_b_tree_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    ShortlexOrderedUniqueCollections {
        current_len: a,
        max_len: b,
        xs: xs.clone(),
        current_xss: lex_fixed_length_b_tree_sets(a, xs),
    }
}

/// Generates `BTreeSet`s with elements from a single iterator.
///
/// The `BTreeSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty `BTreeSet`.
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets;
///
/// fn main() {
///     let xss = lex_b_tree_sets(1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{},
///             btreeset!{1},
///             btreeset!{1, 2},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 3, 4},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3},
///             btreeset!{1, 3, 4},
///             btreeset!{1, 4},
///             btreeset!{2},
///             btreeset!{2, 3},
///             btreeset!{2, 3, 4},
///             btreeset!{2, 4},
///             btreeset!{3},
///             btreeset!{3, 4},
///             btreeset!{4}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn lex_b_tree_sets<I: Clone + Iterator>(
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    lex_b_tree_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates `BTreeSet`s with a mininum length, with elements from a single iterator.
///
/// The `BTreeSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_min_length;
///
/// fn main() {
///     let xss = lex_b_tree_sets_min_length(2, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 3, 4},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3},
///             btreeset!{1, 3, 4},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 3, 4},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///         ]
///     );
/// }
/// ```
#[inline]
pub fn lex_b_tree_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    lex_b_tree_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates `BTreeSet`s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The `BTreeSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty `BTreeSet`.
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_length_range;
///
/// fn main() {
///     let xss = lex_b_tree_sets_length_range(2, 4, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3},
///             btreeset!{1, 3, 4},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 3, 4},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///         ]
///     );
/// }
/// ```
#[inline]
pub fn lex_b_tree_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    lex_b_tree_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates `BTreeSet`s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The `BTreeSet`s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty `BTreeSet`.
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Complexity per iteration
/// $$
/// T(i, k) = O(k + T^\prime (i))
/// $$
///
/// $$
/// M(i, k) = O(k + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $T^\prime$ and $M^\prime$ are the time and
/// additional memory functions of `xs`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_length_inclusive_range;
///
/// fn main() {
///     let xss = lex_b_tree_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
///     assert_eq!(
///         xss.into_iter().collect_vec().as_slice(),
///         &[
///             btreeset!{1, 2},
///             btreeset!{1, 2, 3},
///             btreeset!{1, 2, 4},
///             btreeset!{1, 3},
///             btreeset!{1, 3, 4},
///             btreeset!{1, 4},
///             btreeset!{2, 3},
///             btreeset!{2, 3, 4},
///             btreeset!{2, 4},
///             btreeset!{3, 4},
///         ]
///     );
/// }
/// ```
pub fn lex_b_tree_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    LexOrderedUniqueCollections {
        done: a > b,
        first: true,
        min_len: usize::exact_from(a),
        max_len: usize::exact_from(b),
        xs: IteratorCache::new(xs),
        indices: (0..usize::exact_from(a)).collect(),
        phantom_i: PhantomData,
        phantom_c: PhantomData,
    }
}
