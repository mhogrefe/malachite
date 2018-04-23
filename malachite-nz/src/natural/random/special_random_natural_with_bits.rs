use malachite_base::num::BitAccess;
use natural::random::special_random_natural_up_to_bits::special_random_natural_up_to_bits;
use natural::Natural;
use rand::Rng;

/// Returns a random `Natural` with exactly `bits` bits; equivalently, returns a random `Natural`
/// sampled from [2<sup>`bits`-1</sup>, 2<sup>`bits`</sup>). The `Natural` will typically have long
/// runs of 0s and 1s in its binary expansion, to help trigger edge cases for testing.
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::special_random_natural_with_bits::*;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(format!("{:b}", special_random_natural_with_bits(&mut rng, 4)), "1101");
///     assert_eq!(format!("{:b}", special_random_natural_with_bits(&mut rng, 10)), "1111111011");
///     assert_eq!(format!("{:b}", special_random_natural_with_bits(&mut rng, 80)),
///         "11111111100000000000000000000000000000000000000000000000000000000000000000000000");
/// }
/// ```
pub fn special_random_natural_with_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    let mut n = special_random_natural_up_to_bits(rng, bits);
    if bits != 0 {
        n.set_bit(bits - 1);
    }
    n
}
