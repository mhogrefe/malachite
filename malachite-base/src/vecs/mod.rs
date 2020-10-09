use num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
use random::Seed;

/// Inserts several copies of a value to the left (beginning) of a `Vec`. Using this function is
/// more efficient than inserting the values one by one.
///
/// Time: worst case O(n + m)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()` before the function is called and m = `pad_size`
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

/// Deletes several values from the left (beginning) of a `Vec`. Using this function is more
/// efficient than deleting the values one by one.
///
/// Time: worst case O(max(1, n - m))
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` before the function is called and m = `delete_size`
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

/// Uniformly generates a random value from a nonempty `Vec`. The iterator owns the data. It may be
/// more convenient for the iterator to return references to a pre-existing slice, in which case you
/// may  use `random_values_from_slice` instead.
///
/// Length is infinite.
///
/// Time per iteration: worst case O(1)
///
/// Additional memory per iteration: worst case O(1)
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
/// # lex_exhaustive_length_[n]_vecs
/// ```
/// use malachite_base::vecs::exhaustive::lex_exhaustive_length_2_vecs;
///
/// let xss = lex_exhaustive_length_2_vecs(
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
/// # lex_exhaustive_fixed_length_vecs_[m]_inputs
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::vecs::exhaustive::lex_exhaustive_fixed_length_vecs_2_inputs;
///
/// // We are generating length-3 `Vec`s of chars using two input iterators. The first iterator
/// // (with index 0) produces all ASCII chars, and the second (index 1) produces the three chars
/// // `'x'`, `'y'`, and `'z'`. The second elements of `output_types` are 0, 1, and 0, meaning that
/// // the first element of the output `Vec`s will be taken from iterator 0, the second element from
/// // iterator 1, and the third also from iterator 0.
/// let xss = lex_exhaustive_fixed_length_vecs_2_inputs(
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
/// // We are generating length-3 `Vec`s of chars using two input iterators. The first iterator
/// // (with index 0) produces all ASCII chars, and the second (index 1) produces the three chars
/// // `'x'`, `'y'`, and `'z'`. The second elements of `output_types` are 0, 1, and 0, meaning that
/// // the first element of the output `Vec`s will be taken from iterator 0, the second element from
/// // iterator 1, and the third also from iterator 0. The third element has a tiny output type, so
/// // it will grow more slowly than the other two elements (though it doesn't look that way from
/// // the first few `Vec`s).
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
pub mod from_str;
