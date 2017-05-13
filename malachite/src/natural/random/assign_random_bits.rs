use natural::Natural;
use natural::random::assign_random_up_to_bits::assign_random_up_to_bits;
use rand::Rng;

/// Assigns a random number with up to `bits` bits to `n`; equivalently, assigns a random number
/// uniformly sampled from [0, 2^(`bits`)) to `n`.
///
/// # Example
/// ```
/// extern crate malachite;
/// extern crate rand;
///
/// use malachite::natural::Natural;
/// use malachite::natural::random::assign_random_bits::assign_random_bits;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     let mut x = Natural::new();
///     assign_random_bits(&mut rng, &mut x, 4);
///     assert_eq!(x.to_string(), "10");
///     assign_random_bits(&mut rng, &mut x, 10);
///     assert_eq!(x.to_string(), "717");
///     assign_random_bits(&mut rng, &mut x, 100);
///     assert_eq!(x.to_string(), "1147035045202790645135301334895");
/// }
/// ```
pub fn assign_random_bits<R: Rng>(rng: &mut R, n: &mut Natural, bits: u64) {
    assign_random_up_to_bits(rng, n, bits);
    if bits != 0 {
        n.set_bit(bits - 1);
    }
}
