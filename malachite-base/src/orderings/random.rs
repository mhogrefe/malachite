use std::cmp::Ordering;

use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;

use orderings::ORDERINGS;
use random::StandardRand;

impl StandardRand for Ordering {
    /// Generates a random `Ordering` that has an equal probability of being `Less`, `Greater`, or
    /// `Equal`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::{standard_random_values, EXAMPLE_SEED};
    /// use std::cmp::Ordering::{self, Less, Greater, Equal};
    ///
    /// assert_eq!(
    ///     standard_random_values::<Ordering>(EXAMPLE_SEED).take(10).collect::<Vec<Ordering>>(),
    ///     &[Less, Less, Greater, Equal, Less, Less, Less, Greater, Less, Greater]
    /// )
    /// ```
    #[inline]
    fn standard_gen(rng: &mut ChaCha20Rng) -> Ordering {
        *ORDERINGS.choose(rng).unwrap()
    }
}
