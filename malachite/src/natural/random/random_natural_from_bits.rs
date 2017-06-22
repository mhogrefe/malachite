use natural::Natural;
use natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
use rand::Rng;

/// Returns random `Natural` with exactly `bits` bits; equivalently, returns a random `Natural`
/// uniformly sampled from [2^(`bits`-1), 2^(`bits`)).
///
/// # Example
/// ```
/// extern crate malachite;
/// extern crate rand;
///
/// use malachite::natural::random::random_natural_from_bits::random_natural_from_bits;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(random_natural_from_bits(&mut rng, 4).to_string(), "10");
///     assert_eq!(random_natural_from_bits(&mut rng, 10).to_string(), "717");
///     assert_eq!(random_natural_from_bits(&mut rng, 100).to_string(),
///                "1147035045202790645135301334895");
/// }
/// ```
pub fn random_natural_from_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    let mut n = random_natural_up_to_bits(rng, bits);
    if bits != 0 {
        n.set_bit(bits - 1);
    }
    n
}
