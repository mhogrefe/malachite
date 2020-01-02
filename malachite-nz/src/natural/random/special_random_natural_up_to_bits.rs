use std::cmp::max;

use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, SaturatingSubAssign, ShrRound};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base::round::RoundingMode;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

use natural::arithmetic::add::limbs_slice_add_limb_in_place;
use natural::Natural;

/// Returns a slice of `T`s representing a random `Natural` with up to `bits` bits; equivalently,
/// returns the limbs of a random `Natural` sampled from [0, 2<sup>`bits`</sup>). The `Natural` will
/// typically have long runs of 0s and 1s in its binary expansion, to help trigger edge cases for
/// testing. There may be trailing zero limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `bits`
///
/// This is mpn_random2 from mpn/generic/random2.c.
///
/// # Panics
/// Panics if `bits` is zero.
///
/// # Example
/// ```
/// extern crate malachite_nz;
/// extern crate rand;
///
/// use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
/// use rand::{SeedableRng, StdRng};
///
/// let seed: &[_] = &[1, 2, 3, 4];
/// let mut rng: StdRng = SeedableRng::from_seed(seed);
/// assert_eq!(limbs_special_random_up_to_bits::<u32, _>(&mut rng, 4), &[5]);
/// assert_eq!(limbs_special_random_up_to_bits::<u32, _>(&mut rng, 10), &[1020]);
/// assert_eq!(limbs_special_random_up_to_bits::<u32, _>(&mut rng, 100),
///     &[3940351, 4294965248, 4292870144, 15]);
/// ```
pub fn limbs_special_random_up_to_bits<T: PrimitiveUnsigned, R: Rng>(
    rng: &mut R,
    bits: u64,
) -> Vec<T> {
    assert_ne!(bits, 0);
    let remainder_bits = bits.mod_power_of_two(u64::from(T::LOG_WIDTH));
    let limb_count = bits.shr_round(T::LOG_WIDTH, RoundingMode::Ceiling);
    // Initialize the value to all binary 1s; later we'll remove chunks to create blocks of 0s.
    let mut limbs = vec![T::MAX; usize::exact_from(limb_count)];
    // max_chunk_size may be as low as max(1, bits / 4) or as high as bits. The actual chunk size
    // will be between 1 and max_chunk_size, inclusive.
    let max_chunk_size = max(1, bits / (rng.gen_range(0, 4) + 1));
    let chunk_size_range = Range::new(1, max_chunk_size + 1);
    // Start i at a random position in the highest limb.
    let mut i = (limb_count << T::LOG_WIDTH) - u64::from(rng.gen_range(0, T::WIDTH));
    loop {
        let mut chunk_size = chunk_size_range.ind_sample(rng);
        i.saturating_sub_assign(chunk_size);
        if i == 0 {
            break;
        }
        limbs[usize::exact_from(i >> T::LOG_WIDTH)].clear_bit(i & u64::from(T::WIDTH_MASK));
        chunk_size = chunk_size_range.ind_sample(rng);
        i.saturating_sub_assign(chunk_size);
        limbs_slice_add_limb_in_place(
            &mut limbs[usize::exact_from(i >> T::LOG_WIDTH)..],
            T::ONE << (i & u64::from(T::WIDTH_MASK)),
        );
        if i == 0 {
            break;
        }
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
/// sampled from [0, 2<sup>`bits`</sup>). The `Natural` will typically have long runs of 0s and 1s
/// in its binary expansion, to help trigger edge cases for testing.
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
/// use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
/// use rand::{SeedableRng, StdRng};
///
/// fn main() {
///     let seed: &[_] = &[1, 2, 3, 4];
///     let mut rng: StdRng = SeedableRng::from_seed(seed);
///     assert_eq!(format!("{:b}", special_random_natural_up_to_bits(&mut rng, 4)), "101");
///     assert_eq!(format!("{:b}", special_random_natural_up_to_bits(&mut rng, 10)), "1111111100");
///     assert_eq!(format!("{:b}", special_random_natural_up_to_bits(&mut rng, 80)),
///         "10000000000000000000010000000000000000000001111111111111111111100011111111111111");
/// }
/// ```
#[cfg(feature = "32_bit_limbs")]
pub fn special_random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(limbs_special_random_up_to_bits(rng, bits))
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
pub fn special_random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        let limbs: Vec<u32> = limbs_special_random_up_to_bits(rng, bits);
        Natural::from_owned_limbs_asc(u64::vec_from_other_type_slice(&limbs))
    }
}
