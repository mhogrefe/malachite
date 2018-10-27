use malachite_base::num::{IsPowerOfTwo, SignificantBits};
use natural::random::special_random_natural_up_to_bits::special_random_natural_up_to_bits;
use natural::Natural;
use rand::Rng;

/// Returns a random `Natural` sampled from [0, `n`). The `Natural` will typically have long runs of
/// 0s and 1s in its binary expansion, to help trigger edge cases for testing.
///
/// Time: worst case expected O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `n` is 0.
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::special_random_natural_below::special_random_natural_below;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(format!("{:b}", special_random_natural_below(&mut rng, &Natural::from(10u32))),
///         "101");
///     assert_eq!(
///         format!("{:b}", special_random_natural_below(&mut rng, &Natural::from(1_000_000u32))),
///         "10111110000011110111"
///     );
///     assert_eq!(format!("{:b}", special_random_natural_below(&mut rng, &Natural::trillion())),
///         "1000001111111111111111100000000000000000");
/// }
/// ```
pub fn special_random_natural_below<R: Rng>(rng: &mut R, n: &Natural) -> Natural {
    assert_ne!(*n, 0, "Cannot generate a Natural below 0");
    if n.is_power_of_two() {
        special_random_natural_up_to_bits(rng, n.significant_bits() - 1)
    } else {
        let bits = n.significant_bits();
        // Loop loops <= 2 times on average.
        loop {
            let m = special_random_natural_up_to_bits(rng, bits);
            if m < *n {
                return m;
            }
        }
    }
}
