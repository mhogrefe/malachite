use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ShrRound};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base::round::RoundingMode;
use rand::{Rand, Rng};

use natural::Natural;

/// Returns a slice of `Limb`s representing a random `Natural` with up to `bits` bits; equivalently,
/// returns the limbs of a random `Natural` uniformly sampled from [0, 2<sup>`bits`</sup>). There
/// may be trailing zero limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `bits`
///
/// This is mpn_random from mpn/generic/random.c.
///
/// # Panics
/// Panics if `bits` is zero.
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::random_natural_up_to_bits::limbs_random_up_to_bits;
/// use rand::{SeedableRng, StdRng};
///
/// let seed: &[_] = &[1, 2, 3, 4];
/// let mut rng: StdRng = SeedableRng::from_seed(seed);
/// assert_eq!(limbs_random_up_to_bits::<u32, _>(&mut rng, 4), &[2]);
/// assert_eq!(limbs_random_up_to_bits::<u32, _>(&mut rng, 10), &[205]);
/// assert_eq!(limbs_random_up_to_bits::<u32, _>(&mut rng, 100),
///     &[1930352495, 1261517434, 2051352252, 14]);
/// ```
pub fn limbs_random_up_to_bits<T: PrimitiveUnsigned + Rand, R: Rng>(
    rng: &mut R,
    bits: u64,
) -> Vec<T> {
    assert_ne!(bits, 0);
    let remainder_bits = bits.mod_power_of_two(u64::from(T::LOG_WIDTH));
    let limb_count = bits.shr_round(T::LOG_WIDTH, RoundingMode::Ceiling);
    let mut limbs: Vec<T> = Vec::with_capacity(usize::exact_from(limb_count));
    for _ in 0..limb_count {
        limbs.push(rng.gen());
    }
    if remainder_bits != 0 {
        limbs
            .last_mut()
            .unwrap()
            .mod_power_of_two_assign(remainder_bits);
    }
    limbs
}

/// Returns a random `Natural` with up to `bits` bits; equivalently, returns a random `Natural`
/// uniformly sampled from [0, 2<sup>`bits`</sup>).
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `bits`
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
/// use rand::{SeedableRng, StdRng};
///
/// let seed: &[_] = &[1, 2, 3, 4];
/// let mut rng: StdRng = SeedableRng::from_seed(seed);
/// assert_eq!(random_natural_up_to_bits(&mut rng, 4).to_string(), "2");
/// assert_eq!(random_natural_up_to_bits(&mut rng, 10).to_string(), "205");
/// assert_eq!(random_natural_up_to_bits(&mut rng, 100).to_string(),
///            "1147035045202790645135301334895");
/// ```
#[cfg(feature = "32_bit_limbs")]
pub fn random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(limbs_random_up_to_bits(rng, bits))
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
pub fn random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        let limbs: Vec<u32> = limbs_random_up_to_bits(rng, bits);
        Natural::from_owned_limbs_asc(u64::vec_from_other_type_slice(&limbs))
    }
}
