use malachite_base::num::logic::traits::BitAccess;
use rand::Rng;

use natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
use natural::Natural;

/// Returns a random `Natural` with exactly `bits` bits; equivalently, returns a random `Natural`
/// uniformly sampled from [2<sup>`bits`-1</sup>, 2<sup>`bits`</sup>).
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `bits`
///
/// # Examples
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
/// use rand::{SeedableRng, StdRng};
///
/// let seed: &[_] = &[1, 2, 3, 4];
/// let mut rng: StdRng = SeedableRng::from_seed(seed);
/// assert_eq!(random_natural_with_bits(&mut rng, 4).to_string(), "10");
/// assert_eq!(random_natural_with_bits(&mut rng, 10).to_string(), "717");
/// assert_eq!(random_natural_with_bits(&mut rng, 100).to_string(),
///            "1147035045202790645135301334895");
/// ```
pub fn random_natural_with_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    let mut n = random_natural_up_to_bits(rng, bits);
    if bits != 0 {
        n.set_bit(bits - 1);
    }
    n
}
