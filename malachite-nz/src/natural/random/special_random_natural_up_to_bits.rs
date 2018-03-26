use malachite_base::num::{BitAccess, PrimitiveInteger};
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::Natural;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::cmp::max;
use std::u32;

//TODO document and test
pub fn limbs_special_random_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Vec<u32> {
    if bits == 0 {
        return Vec::new();
    }
    let remainder_bits = (bits & u64::from(u32::WIDTH_MASK)) as u32;
    let limb_count = if remainder_bits == 0 {
        bits >> u32::LOG_WIDTH
    } else {
        (bits >> u32::LOG_WIDTH) + 1
    };
    // Initialize the value to all binary 1s; later we'll remove chunks to create blocks of 0s.
    let mut limbs = vec![u32::MAX; limb_count as usize];
    // max_chunk_size may be as low as max(1, bits / 4) or as high as bits. The actual chunk size
    // will be between 1 and max_chunk_size, inclusive.
    let max_chunk_size = max(1, (bits / (rng.gen_range(0, 4) + 1)) as u32);
    let chunk_size_range = Range::new(1, max_chunk_size + 1);
    // Start i at a random position in the highest limb.
    let mut i = ((limb_count as u32) << u32::LOG_WIDTH) - rng.gen_range(0, u32::WIDTH);
    loop {
        let mut chunk_size = chunk_size_range.ind_sample(rng);
        i = if i < chunk_size { 0 } else { i - chunk_size };
        if i == 0 {
            break;
        }
        limbs[(i >> u32::LOG_WIDTH) as usize].clear_bit(u64::from(i & u32::WIDTH_MASK));
        chunk_size = chunk_size_range.ind_sample(rng);
        i = if i < chunk_size { 0 } else { i - chunk_size };
        mpn_add_1_in_place(
            &mut limbs[(i >> u32::LOG_WIDTH) as usize..],
            1 << (i & u32::WIDTH_MASK),
        );
        if i == 0 {
            break;
        }
    }
    if remainder_bits != 0 {
        *limbs.last_mut().unwrap() &= (1 << remainder_bits) - 1;
    }
    limbs
}

/// Returns a random `Natural` with up to `bits` bits; equivalently, returns a random `Natural`
/// sampled from [0, 2<sup>`bits`</sup>). The `Natural` will typically have long runs of 0s and 1s
/// in its binary expansion, to help trigger edge cases for testing.
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
///     assert_eq!(format!("{:b}", special_random_natural_up_to_bits(&mut rng, 10)), "1111111011");
///     assert_eq!(format!("{:b}", special_random_natural_up_to_bits(&mut rng, 80)),
///         "11111111100000000000000000000000000000000000000000000000000000000000000000000000");
/// }
/// ```
pub fn special_random_natural_up_to_bits<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    Natural::from_owned_limbs_asc(limbs_special_random_up_to_bits(rng, bits))
}
