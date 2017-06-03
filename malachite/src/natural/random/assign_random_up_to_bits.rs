use natural::Natural;
use traits::Assign;
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
/// use malachite::natural::random::assign_random_up_to_bits::assign_random_up_to_bits;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     let mut x = Natural::new();
///     assign_random_up_to_bits(&mut rng, &mut x, 4);
///     assert_eq!(x.to_string(), "2");
///     assign_random_up_to_bits(&mut rng, &mut x, 10);
///     assert_eq!(x.to_string(), "205");
///     assign_random_up_to_bits(&mut rng, &mut x, 100);
///     assert_eq!(x.to_string(), "1147035045202790645135301334895");
/// }
/// ```
pub fn assign_random_up_to_bits<R: Rng>(rng: &mut R, n: &mut Natural, bits: u64) {
    if bits == 0 {
        n.assign(0u32);
        return;
    }
    let remainder_bits = bits & 0x1f;
    let limb_count = if remainder_bits == 0 {
        bits >> 5
    } else {
        (bits >> 5) + 1
    };
    let mut limbs = Vec::with_capacity(limb_count as usize);
    for _ in 0..limb_count {
        limbs.push(rng.gen());
    }
    if remainder_bits != 0 {
        *limbs.last_mut().unwrap() &= (1 << remainder_bits) - 1;
    }
    n.assign_limbs_le(&limbs);
}
