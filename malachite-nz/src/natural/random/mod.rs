use malachite_base::traits::Zero;
use natural::Natural;
use rand::Rng;

/// Returns a random `Natural` with up to `bits` bits; equivalently, returns a random `Natural`
/// uniformly sampled from [0, 2^(`bits`)).
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::random_natural_up_to_bits;
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
    Natural::from_limbs_le(&limbs)
}

/// Returns random `Natural` with exactly `bits` bits; equivalently, returns a random `Natural`
/// uniformly sampled from [2^(`bits`-1), 2^(`bits`)).
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::random_natural_from_bits;
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

/// Returns a random `Natural` uniformly sampled from [0, `n`).
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::random_natural_below;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(random_natural_below(&mut rng, &Natural::from(10u32)).to_string(), "2");
///     assert_eq!(random_natural_below(&mut rng,
///         &Natural::from(1000000u32)).to_string(), "293069");
///     assert_eq!(random_natural_below(&mut rng, &Natural::trillion()).to_string(),
///         "525916362607");
/// }
/// ```
pub fn random_natural_below<R: Rng>(rng: &mut R, n: &Natural) -> Natural {
    assert_ne!(*n, 0, "Cannot generate a Natural below 0");
    if n.is_power_of_2() {
        random_natural_up_to_bits(rng, n.significant_bits() - 1)
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
