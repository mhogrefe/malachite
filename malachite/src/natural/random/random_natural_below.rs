use natural::Natural;
use natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
use rand::Rng;

/// Returns a random `Natural` uniformly sampled from [0, `n`).
///
/// # Example
/// ```
/// extern crate malachite;
/// extern crate rand;
///
/// use malachite::natural::Natural;
/// use malachite::natural::random::random_natural_below::random_natural_below;
/// use rand::{SeedableRng, StdRng};
/// use std::str::FromStr;
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(random_natural_below(&mut rng, &Natural::from(10u32)).to_string(), "2");
///     assert_eq!(random_natural_below(&mut rng,
///                &Natural::from(1000000u32)).to_string(), "293069");
///     assert_eq!(random_natural_below(&mut rng,
///                                     &Natural::from_str("1000000000000").unwrap()).to_string(),
///                "525916362607");
/// }
/// ```
pub fn random_natural_below<R: Rng>(rng: &mut R, n: &Natural) -> Natural {
    assert_ne!(*n, 0, "Cannot generate a Natural below 0");
    if n.is_power_of_two() {
        random_natural_up_to_bits(rng, n.significant_bits())
    } else {
        let bits = n.significant_bits();
        loop {
            let m = random_natural_up_to_bits(rng, bits);
            if m < *n {
                return m;
            }
        }
    }
}
