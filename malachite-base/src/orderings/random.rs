use std::cmp::Ordering;
use std::iter::Cloned;

use orderings::ORDERINGS;
use random::Seed;
use slices::{random_values_from_slice, RandomValuesFromSlice};

/// Generates a random `Ordering` that has an equal probability of being `Less`, `Greater`, or
/// `Equal`.
///
/// $P(<) = P(=) = P(>) = \frac{1}{3}$.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::orderings::random::random_orderings;
/// use malachite_base::random::EXAMPLE_SEED;
/// use std::cmp::Ordering::{self, Less, Greater, Equal};
///
/// assert_eq!(
///     random_orderings(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[Less, Equal, Less, Greater, Less, Less, Equal, Less, Equal, Greater]
/// )
/// ```
#[inline]
pub fn random_orderings(seed: Seed) -> Cloned<RandomValuesFromSlice<'static, Ordering>> {
    random_values_from_slice(seed, &ORDERINGS).cloned()
}
