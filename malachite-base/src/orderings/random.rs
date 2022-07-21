use crate::orderings::ORDERINGS;
use crate::random::Seed;
use crate::slices::{random_values_from_slice, RandomValuesFromSlice};
use std::cmp::Ordering;
use std::iter::Cloned;

pub type RandomOrderings = Cloned<RandomValuesFromSlice<'static, Ordering>>;

/// Generates a random [`Ordering`] that has an equal probability of being `Less`, `Greater`, or
/// `Equal`.
///
/// $P(<) = P(=) = P(>) = \frac{1}{3}$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
/// use malachite_base::orderings::random::random_orderings;
/// use malachite_base::random::EXAMPLE_SEED;
/// use std::cmp::Ordering::{self, Equal, Greater, Less};
///
/// assert_eq!(
///     random_orderings(EXAMPLE_SEED).take(10).collect_vec(),
///     &[Less, Equal, Less, Greater, Less, Less, Equal, Less, Equal, Greater]
/// )
/// ```
#[inline]
pub fn random_orderings(seed: Seed) -> RandomOrderings {
    random_values_from_slice(seed, &ORDERINGS).cloned()
}
