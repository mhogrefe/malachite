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
/// # exhaustive_length_[n]_vecs
/// ```
/// use malachite_base::vecs::exhaustive::exhaustive_length_2_vecs;
///
/// let xss = exhaustive_length_2_vecs(
///     ['a', 'b', 'c'].iter().cloned(),
///     ['x', 'y', 'z'].iter().cloned()
/// )
///     .take(20)
///     .collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &['a', 'x'], &['a', 'y'], &['b', 'x'], &['b', 'y'], &['a', 'z'], &['b', 'z'],
///         &['c', 'x'], &['c', 'y'], &['c', 'z']
///     ]
/// );
/// ```
pub mod exhaustive;
pub mod from_str;
