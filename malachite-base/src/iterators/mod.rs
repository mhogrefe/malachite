#[cfg(feature = "random")]
use crate::bools::random::{weighted_random_bools, WeightedRandomBools};
use crate::num::basic::traits::Zero;
#[cfg(feature = "random")]
use crate::random::Seed;
#[cfg(feature = "random")]
use crate::vecs::{random_values_from_vec, RandomValuesFromVec};
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::hash::Hash;
use hashbrown::HashSet;
use itertools::Itertools;

/// Generates all the nonzero values of a provided iterator.
///
/// This `struct` is created by [`nonzero_values`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct NonzeroValues<I: Iterator>(I)
where
    I::Item: PartialEq<I::Item> + Zero;

impl<I: Iterator> Iterator for NonzeroValues<I>
where
    I::Item: PartialEq<I::Item> + Zero,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let x = self.0.next();
            if x != Some(I::Item::ZERO) {
                return x;
            }
        }
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for NonzeroValues<I>
where
    I::Item: PartialEq<I::Item> + Zero,
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> {
        loop {
            let x = self.0.next_back();
            if x != Some(I::Item::ZERO) {
                return x;
            }
        }
    }
}

/// Returns an iterator that generates all the nonzero values of a provided iterator.
///
/// `nonzero_values(xs)` generates the same values as `xs.filter(|x| x != I::Item::ZERO)`, but its
/// type is easier to work with.
///
/// This iterator will hang if given an iterator that produces an infinite suffix of zeros.
///
/// The output length is the number of nonzero values produced by `xs`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::nonzero_values;
///
/// assert_eq!(
///     nonzero_values([-3i8, -2, -1, 0, 1, 2, 3].iter().cloned()).collect_vec(),
///     &[-3, -2, -1, 1, 2, 3]
/// )
/// ```
#[inline]
pub const fn nonzero_values<I: Iterator>(xs: I) -> NonzeroValues<I>
where
    I::Item: PartialEq<I::Item> + Zero,
{
    NonzeroValues(xs)
}

/// Returns whether all of the values generated by an iterator are equal.
///
/// `is_constant(xs)` is equivalent to `xs.unique().count() == 1` for finite nonempty iterators, but
/// is more efficient, doesn't require [`Clone`] or [`Hash`] implementations, and doesn't hang if
/// provided an infinite non-constant iterator.
///
/// This function will hang if given an infinite constant iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::is_constant;
///
/// assert_eq!(is_constant([1; 4].iter()), true);
/// assert_eq!(is_constant([1, 2, 3, 4].iter()), false);
/// assert_eq!(is_constant(0..), false);
/// ```
pub fn is_constant<I: Iterator>(xs: I) -> bool
where
    I::Item: Eq,
{
    let mut first = None;
    for x in xs {
        if let Some(ref first) = first {
            if x != *first {
                return false;
            }
        } else {
            first = Some(x);
        }
    }
    true
}

/// Returns whether an iterator returns at least some number of values.
///
/// `count_is_at_least(xs, n)` is equivalent to `xs.count() >= n` for finite iterators, but doesn't
/// hang if provided an infinite iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::count_is_at_least;
///
/// assert_eq!(count_is_at_least([1, 2, 3, 4].iter(), 3), true);
/// assert_eq!(count_is_at_least([1, 2, 3, 4].iter(), 4), true);
/// assert_eq!(count_is_at_least([1, 2, 3, 4].iter(), 5), false);
/// assert_eq!(count_is_at_least(0.., 5), true);
/// ```
#[inline]
pub fn count_is_at_least<I: Iterator>(xs: I, n: usize) -> bool {
    xs.take(n).count() == n
}

/// Returns whether an iterator returns at most some number of values.
///
/// `count_is_at_most(xs, n)` is equivalent to `xs.count() <= n` for finite iterators, but doesn't
/// hang if provided an infinite iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::count_is_at_most;
///
/// assert_eq!(count_is_at_most([1, 2, 3, 4].iter(), 3), false);
/// assert_eq!(count_is_at_most([1, 2, 3, 4].iter(), 4), true);
/// assert_eq!(count_is_at_most([1, 2, 3, 4].iter(), 5), true);
/// assert_eq!(count_is_at_most(0.., 5), false);
/// ```
#[inline]
pub fn count_is_at_most<I: Iterator>(xs: I, n: usize) -> bool {
    xs.take(n + 1).count() <= n
}

/// Returns whether an iterator never returns the same value twice.
///
/// `is_unique(xs)` is equivalent to `xs.unique().count() <= 1` for finite iterators, but is more
/// efficient and doesn't hang if provided a non-unique infinite iterator.
///
/// This iterator will hang if given an infinite unique iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::is_unique;
///
/// let empty: [u32; 0] = [];
/// assert_eq!(is_unique(empty.iter()), true);
/// assert_eq!(is_unique([1, 2, 3, 4].iter()), true);
/// assert_eq!(is_unique([1, 2, 3, 1].iter()), false);
/// assert_eq!(is_unique((0..).map(|i| i / 2)), false);
/// ```
#[inline]
pub fn is_unique<I: Iterator>(xs: I) -> bool
where
    I::Item: Eq + Hash,
{
    let mut set = HashSet::new();
    for x in xs {
        if !set.insert(x) {
            return false;
        }
    }
    true
}

/// Returns the first and last elements of an iterator, or `None` if it is empty.
///
/// The iterator's elements must be cloneable, since if the iterator consists of a single element
/// `x`, the result will be `(x, x)`.
///
/// This iterator will hang if given an infinite iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::first_and_last;
///
/// let empty: [u32; 0] = [];
/// assert_eq!(first_and_last(&mut empty.iter()), None);
/// assert_eq!(first_and_last(&mut [1].iter().cloned()), Some((1, 1)));
/// assert_eq!(first_and_last(&mut [1, 2, 3].iter().cloned()), Some((1, 3)));
/// ```
pub fn first_and_last<I: Iterator>(xs: &mut I) -> Option<(I::Item, I::Item)>
where
    I::Item: Clone,
{
    xs.next().map(|first| {
        if let Some(last) = xs.last() {
            (first, last)
        } else {
            (first.clone(), first)
        }
    })
}

/// Groups elements of an iterator into intervals of adjacent elements that match a predicate. The
/// endpoints of each interval are returned.
///
/// The intervals are inclusive.
///
/// This iterator will hang if given an infinite iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::matching_intervals_in_iterator;
///
/// let xs = &[1, 2, 10, 11, 12, 7, 8, 16, 5];
/// assert_eq!(
///     matching_intervals_in_iterator(xs.iter().cloned(), |&x| x >= 10).as_slice(),
///     &[(10, 12), (16, 16)]
/// );
/// assert_eq!(
///     matching_intervals_in_iterator(xs.iter().cloned(), |&x| x < 10).as_slice(),
///     &[(1, 2), (7, 8), (5, 5)]
/// );
/// ```
pub fn matching_intervals_in_iterator<I: Iterator, F: Fn(&I::Item) -> bool>(
    xs: I,
    predicate: F,
) -> Vec<(I::Item, I::Item)>
where
    I::Item: Clone,
{
    xs.group_by(predicate)
        .into_iter()
        .filter_map(|(b, mut group)| if b { first_and_last(&mut group) } else { None })
        .collect()
}

#[cfg(feature = "random")]
/// An iterator that randomly produces another iterator's values, or produces a special value.
///
/// This `struct` is created by [`with_special_value`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct WithSpecialValue<I: Iterator>
where
    I::Item: Clone,
{
    bs: WeightedRandomBools,
    special_value: I::Item,
    xs: I,
}

#[cfg(feature = "random")]
impl<I: Iterator> Iterator for WithSpecialValue<I>
where
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.bs.next().unwrap() {
            Some(self.special_value.clone())
        } else {
            self.xs.next()
        }
    }
}

#[cfg(feature = "random")]
/// An iterator that randomly produces another iterator's values, or produces a special value.
///
/// Let $n_p$ be `p_numerator`, $d_p$ be `p_denominator`, and let $p=n_p/d_p$.
///
/// Every time a value is to be generated, the iterator returns the special value with probability
/// $p$, or else returns a value from the inner iterator.
///
/// If $p > 0$, the output length is infinite. Otherwise, it is the same as the length of `xs`.
///
/// # Panics
/// Panics if `p_denominator` is 0 or `p_numerator` is greater than `p_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::{prefix_to_string, with_special_value};
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         with_special_value(EXAMPLE_SEED, -1i16, 1, 2, &random_primitive_ints::<i16>),
///         20
///     ),
///     "[-1, -1, -1, 2901, -1, -14200, -1, -1, -1, -30997, -8245, -5338, -1, -1, -20007, -1, -1, \
///     -1, -1, -1, ...]"
/// );
/// ```
pub fn with_special_value<I: Iterator>(
    seed: Seed,
    special_value: I::Item,
    p_numerator: u64,
    p_denominator: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> WithSpecialValue<I>
where
    I::Item: Clone,
{
    WithSpecialValue {
        bs: weighted_random_bools(seed.fork("bs"), p_numerator, p_denominator),
        special_value,
        xs: xs_gen(seed.fork("xs")),
    }
}

#[cfg(feature = "random")]
/// An iterator that randomly produces another iterator's values, or samples from a [`Vec`] of
/// special values.
///
/// This `struct` is created by [`with_special_values`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct WithSpecialValues<I: Iterator>
where
    I::Item: Clone,
{
    bs: WeightedRandomBools,
    special_values: RandomValuesFromVec<I::Item>,
    xs: I,
}

#[cfg(feature = "random")]
impl<I: Iterator> Iterator for WithSpecialValues<I>
where
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.bs.next().unwrap() {
            self.special_values.next()
        } else {
            self.xs.next()
        }
    }
}

#[cfg(feature = "random")]
/// An iterator that randomly produces another iterator's values, or produces a random special value
/// from a [`Vec`].
///
/// Let $n_p$ be `p_numerator`, $d_p$ be `p_denominator`, and let $p=n_p/d_p$.
///
/// Every time a value is to be generated, the iterator uniformly samples the special values [`Vec`]
/// with probability $p$, or else returns a value from the inner iterator.
///
/// If $p > 0$, the output length is infinite. Otherwise, it is the same as the length of `xs`.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `special_values` is empty, `p_denominator` is 0, or if `p_numerator` is greater than
/// `p_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::{prefix_to_string, with_special_values};
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         with_special_values(EXAMPLE_SEED, vec![1, 2, 3], 1, 2, &random_primitive_ints::<i16>),
///         20,
///     ),
///     "[3, 1, 3, 2901, 1, -14200, 2, 3, 1, -30997, -8245, -5338, 1, 1, -20007, 3, 1, 1, 1, 1, \
///     ...]"
/// );
/// ```
pub fn with_special_values<I: Iterator>(
    seed: Seed,
    special_values: Vec<I::Item>,
    p_numerator: u64,
    p_denominator: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> WithSpecialValues<I>
where
    I::Item: Clone,
{
    WithSpecialValues {
        bs: weighted_random_bools(seed.fork("bs"), p_numerator, p_denominator),
        special_values: random_values_from_vec(seed.fork("special_values"), special_values),
        xs: xs_gen(seed.fork("xs")),
    }
}

/// Generates sliding windows of elements from an iterator.
///
/// This `struct` is created by [`iter_windows`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct IterWindows<I: Iterator>
where
    I::Item: Clone,
{
    xs: I,
    window: VecDeque<I::Item>,
    window_size: usize,
}

impl<I: Iterator> Iterator for IterWindows<I>
where
    I::Item: Clone,
{
    type Item = VecDeque<I::Item>;

    fn next(&mut self) -> Option<VecDeque<I::Item>> {
        if self.window.len() < self.window_size {
            self.window = (&mut self.xs).take(self.window_size).collect();
            if self.window.len() < self.window_size {
                None
            } else {
                Some(self.window.clone())
            }
        } else {
            let x = self.xs.next()?;
            self.window.pop_front();
            self.window.push_back(x);
            Some(self.window.clone())
        }
    }
}

/// Returns windows of $n$ adjacent elements of an iterator, advancing the window by 1 in each
/// iteration. The values are cloned each time a new window is generated.
///
/// The output length is $n - k + 1$, where $n$ is `xs.count()` and $k$ is `window_size`.
///
/// # Panics
/// Panics if `window_size` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::iter_windows;
///
/// let xs = 0..=5;
/// let windows = iter_windows(3, xs).map(|ws| ws.iter().cloned().collect_vec()).collect_vec();
/// assert_eq!(
///     windows.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]]
/// );
/// ```
pub fn iter_windows<I: Iterator>(window_size: usize, xs: I) -> IterWindows<I>
where
    I::Item: Clone,
{
    assert_ne!(window_size, 0);
    IterWindows {
        xs,
        window: VecDeque::with_capacity(window_size),
        window_size,
    }
}

/// Converts a prefix of an iterator to a string.
///
/// Suppose the iterator generates $(a, b, c, d)$. If `max_len` is 3, this function will return the
/// string `"[a, b, c, ...]"`. If `max_len` is 4 or more, this function will return `[a, b, c, d]`.
///
/// This function will attempt to advance the iterator `max_len + 1` times. The extra time is used
/// determine whether the output string should contain an ellipsis.
///
/// # Panics
/// Panics if `max_len` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::prefix_to_string;
///
/// assert_eq!(prefix_to_string(0..10, 3), "[0, 1, 2, ...]");
/// assert_eq!(prefix_to_string(0..4, 5), "[0, 1, 2, 3]");
/// ```
pub fn prefix_to_string<I: Iterator>(mut xs: I, max_len: usize) -> String
where
    I::Item: Display,
{
    assert_ne!(max_len, 0);
    let mut s = String::new();
    s.push('[');
    let mut first = true;
    let mut done = false;
    for _ in 0..max_len {
        if let Some(x) = xs.next() {
            if first {
                first = false;
            } else {
                s.push_str(", ");
            }
            s.push_str(&x.to_string());
        } else {
            done = true;
            break;
        }
    }
    if !done && xs.next().is_some() {
        s.push_str(", ...");
    }
    s.push(']');
    s
}

/// Contains [`BitDistributor`](bit_distributor::BitDistributor), which helps generate tuples
/// exhaustively.
pub mod bit_distributor;
/// Functions that compare adjacent iterator elements.
pub mod comparison;
/// Contains [`IteratorCache`](iterator_cache::IteratorCache), which remembers values produced by an
/// iterator.
pub mod iterator_cache;
