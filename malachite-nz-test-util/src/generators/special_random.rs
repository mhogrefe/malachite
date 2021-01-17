use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::random::geometric::{
    geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::striped::{
    get_striped_unsigned_vec, striped_random_unsigned_vecs,
    striped_random_unsigned_vecs_length_range, striped_random_unsigned_vecs_min_length,
    StripedBitSource, StripedRandomUnsignedVecs,
};
use malachite_base::num::random::{random_unsigned_inclusive_range, RandomUnsignedRange};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::tuples::random::random_pairs;
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use malachite_base_test_util::generators::common::{GenConfig, It};
use malachite_base_test_util::generators::exhaustive::unsigned_pair_gen_var_5_limit;
use malachite_nz::integer::random::striped_random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::random::striped_random_naturals;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// -- Integer --

pub fn special_random_integer_gen(config: &GenConfig) -> It<Integer> {
    Box::new(striped_random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- Natural --

pub fn special_random_natural_gen(config: &GenConfig) -> It<Natural> {
    Box::new(striped_random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)>
where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                2,
                config.get_or("mean_length_n", 6),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, 2, unsigned_pair_gen_var_5_limit::<T, U>()),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

struct DigitsSpecialRandomGenerator {
    bases: RandomValuesFromVec<u64>,
    xss: StripedRandomUnsignedVecs<Limb, GeometricRandomNaturalValues<u64>>,
    bit_source: StripedBitSource,
}

impl Iterator for DigitsSpecialRandomGenerator {
    type Item = (Vec<u8>, u64, Vec<Limb>);

    fn next(&mut self) -> Option<(Vec<u8>, u64, Vec<Limb>)> {
        let base = self.bases.next().unwrap();
        let xs = self.xss.next().unwrap();
        let out_len = usize::exact_from(limbs_digit_count(&xs, base));
        let out = get_striped_unsigned_vec(
            &mut self.bit_source,
            u64::exact_from(out_len) << u8::LOG_WIDTH,
        );
        Some((out, base, xs))
    }
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<u8>, u64, Vec<Limb>)> {
    Box::new(DigitsSpecialRandomGenerator {
        bases: random_values_from_vec(
            EXAMPLE_SEED.fork("bases"),
            (3u64..256).filter(|&b| !b.is_power_of_two()).collect(),
        ),
        xss: striped_random_unsigned_vecs(
            EXAMPLE_SEED.fork("xss"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsSpecialRandomGenerator {
    bases: RandomValuesFromVec<u64>,
    xss: StripedRandomUnsignedVecs<Limb, RandomUnsignedRange<u64>>,
    excess_lens: RandomOptions<GeometricRandomNaturalValues<usize>>,
    excess_out_lens: GeometricRandomNaturalValues<usize>,
    bit_source: StripedBitSource,
}

impl Iterator for BasecaseDigitsSpecialRandomGenerator {
    type Item = (Vec<u8>, usize, Vec<Limb>, u64);

    fn next(&mut self) -> Option<(Vec<u8>, usize, Vec<Limb>, u64)> {
        let base = self.bases.next().unwrap();
        let xs = self.xss.next().unwrap();
        let min_out_len = usize::exact_from(limbs_digit_count(&xs, base));
        let excess_out_len = self.excess_out_lens.next().unwrap();
        let (len, out_len) = if let Some(excess) = self.excess_lens.next().unwrap() {
            (min_out_len + excess, min_out_len + excess + excess_out_len)
        } else {
            (0, min_out_len + excess_out_len)
        };
        let out = get_striped_unsigned_vec(
            &mut self.bit_source,
            u64::exact_from(out_len) << u8::LOG_WIDTH,
        );
        Some((out, len, xs, base))
    }
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<u8>, usize, Vec<Limb>, u64)> {
    Box::new(BasecaseDigitsSpecialRandomGenerator {
        bases: random_values_from_vec(
            EXAMPLE_SEED.fork("bases"),
            (3u64..256).filter(|&b| !b.is_power_of_two()).collect(),
        ),
        xss: striped_random_unsigned_vecs_length_range(
            EXAMPLE_SEED.fork("xss"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            0,
            u64::exact_from(GET_STR_PRECOMPUTE_THRESHOLD),
        ),
        excess_lens: random_options(
            EXAMPLE_SEED.fork("excess_lens"),
            config.get_or("zero_len_weight_n", 1),
            config.get_or("zero_len_weight_d", 4),
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_excess_len_n", 4),
                    config.get_or("mean_excess_len_d", 1),
                )
            },
        ),
        excess_out_lens: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("excess_out_lens"),
            config.get_or("mean_excess_len_n", 4),
            config.get_or("mean_excess_len_d", 1),
        ),
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}
