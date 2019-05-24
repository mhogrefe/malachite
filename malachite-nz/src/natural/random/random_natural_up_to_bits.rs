use malachite_base::conversion::CheckedFrom;
#[cfg(feature = "64_bit_limbs")]
use malachite_base::num::traits::JoinHalves;
use malachite_base::num::traits::{ModPowerOfTwo, ShrRound, Zero};
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use malachite_base::round::RoundingMode;
use natural::Natural;
use rand::{Rand, Rng};

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
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(limbs_random_up_to_bits::<u32, _>(&mut rng, 4), &[2]);
///     assert_eq!(limbs_random_up_to_bits::<u32, _>(&mut rng, 10), &[205]);
///     assert_eq!(limbs_random_up_to_bits::<u32, _>(&mut rng, 100),
///         &[1930352495, 1261517434, 2051352252, 14]);
/// }
/// ```
pub fn limbs_random_up_to_bits<T: PrimitiveUnsigned + Rand, R: Rng>(
    rng: &mut R,
    bits: u64,
) -> Vec<T> {
    assert_ne!(bits, 0);
    let remainder_bits = bits.mod_power_of_two(u64::from(T::LOG_WIDTH));
    let limb_count = bits.shr_round(T::LOG_WIDTH, RoundingMode::Ceiling);
    let mut limbs: Vec<T> = Vec::with_capacity(usize::checked_from(limb_count).unwrap());
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
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(random_natural_up_to_bits(&mut rng, 4).to_string(), "2");
///     assert_eq!(random_natural_up_to_bits(&mut rng, 10).to_string(), "205");
///     assert_eq!(random_natural_up_to_bits(&mut rng, 100).to_string(),
///                "1147035045202790645135301334895");
/// }
/// ```
#[cfg(feature = "32_bit_limbs")]
pub fn random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(limbs_random_up_to_bits(rng, bits))
    }
}

#[cfg(feature = "64_bit_limbs")]
pub fn random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(_transform_32_to_64_bit_limbs(&limbs_random_up_to_bits(
            rng, bits,
        )))
    }
}

#[cfg(feature = "64_bit_limbs")]
pub fn _transform_32_to_64_bit_limbs(limbs: &Vec<u32>) -> Vec<u64> {
    let mut result_limbs = Vec::with_capacity(limbs.len() << 1);
    let mut iter = limbs.chunks_exact(2);
    for chunk in &mut iter {
        result_limbs.push(u64::join_halves(chunk[1], chunk[0]));
    }
    if let Some(&last) = iter.remainder().first() {
        result_limbs.push(u64::from(last));
    }
    result_limbs
}
