use rand::Rng;
use rand_chacha::ChaCha20Rng;

use random::StandardRand;

macro_rules! impl_standard_rand {
    ($t:ident) => {
        impl StandardRand for $t {
            /// Generates a random primitive integer, from a uniform distribution across all
            /// possible values.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::random::{standard_random_values, EXAMPLE_SEED};
            ///
            /// assert_eq!(
            ///     standard_random_values::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<u8>>(),
            ///     &[113, 228, 87, 188, 93, 189, 117, 151, 7, 72]
            /// )
            /// ```
            #[inline]
            fn standard_gen(rng: &mut ChaCha20Rng) -> $t {
                rng.gen()
            }
        }
    };
}
apply_to_primitive_ints!(impl_standard_rand);
