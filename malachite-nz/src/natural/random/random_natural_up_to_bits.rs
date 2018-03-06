use malachite_base::num::Zero;
use natural::{Natural, LIMB_BITS_MASK, LOG_LIMB_BITS};
use rand::Rng;

/// Returns a random `Natural` with up to `bits` bits; equivalently, returns a random `Natural`
/// uniformly sampled from [0, 2<sup>`bits`</sup>).
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(random_natural_up_to_bits(&mut rng, 4).to_string(), "2");
///     assert_eq!(random_natural_up_to_bits(&mut rng, 10).to_string(), "205");
///     assert_eq!(random_natural_up_to_bits(&mut rng, 100).to_string(),
///                "1147035045202790645135301334895");
/// }
/// ```
pub fn random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        return Natural::ZERO;
    }
    let remainder_bits = (bits & u64::from(LIMB_BITS_MASK)) as u32;
    let limb_count = if remainder_bits == 0 {
        bits >> LOG_LIMB_BITS
    } else {
        (bits >> LOG_LIMB_BITS) + 1
    };
    let mut limbs = Vec::with_capacity(limb_count as usize);
    for _ in 0..limb_count {
        limbs.push(rng.gen());
    }
    if remainder_bits != 0 {
        *limbs.last_mut().unwrap() &= (1 << remainder_bits) - 1;
    }
    Natural::from_owned_limbs_asc(limbs)
}
