use malachite_base::num::traits::{IsPowerOfTwo, SignificantBits};
use natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
use natural::Natural;
use platform::Limb;
use rand::Rng;

/// Returns a random `Natural` uniformly sampled from [0, `n`).
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
/// use malachite_nz::natural::random::random_natural_below::random_natural_below;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(random_natural_below(&mut rng, &Natural::from(10u32)).to_string(), "2");
///     assert_eq!(random_natural_below(&mut rng,
///         &Natural::from(1_000_000u32)).to_string(), "293069");
///     assert_eq!(random_natural_below(&mut rng, &Natural::trillion()).to_string(),
///         "525916362607");
/// }
/// ```
pub fn random_natural_below<R: Rng>(rng: &mut R, n: &Natural) -> Natural {
    assert_ne!(*n, 0 as Limb, "Cannot generate a Natural below 0");
    if n.is_power_of_two() {
        random_natural_up_to_bits(rng, n.significant_bits() - 1)
    } else {
        let bits = n.significant_bits();
        // Loop loops <= 2 times on average.
        loop {
            let m = random_natural_up_to_bits(rng, bits);
            if m < *n {
                return m;
            }
        }
    }
}
