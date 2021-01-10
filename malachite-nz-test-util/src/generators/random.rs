use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::random::geometric::{
    geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::{
    random_primitive_ints, RandomPrimitiveInts, RandomUnsignedRange,
};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::vecs::random::{random_vecs, random_vecs_length_range, RandomVecs};
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use malachite_base_test_util::generators::common::{GenConfig, It};
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::random::random_naturals;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// -- Natural --

pub fn random_natural_gen(config: &GenConfig) -> It<Natural> {
    Box::new(random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

struct DigitsRandomGenerator {
    bases: RandomValuesFromVec<u64>,
    xss: RandomVecs<Limb, GeometricRandomNaturalValues<u64>, RandomPrimitiveInts<Limb>>,
    bytes: RandomPrimitiveInts<u8>,
}

impl Iterator for DigitsRandomGenerator {
    type Item = (Vec<u8>, u64, Vec<Limb>);

    fn next(&mut self) -> Option<(Vec<u8>, u64, Vec<Limb>)> {
        let base = self.bases.next().unwrap();
        let xs = self.xss.next().unwrap();
        let out_len = usize::exact_from(limbs_digit_count(&xs, base));
        let out = (&mut self.bytes).take(out_len).collect();
        Some((out, base, xs))
    }
}

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<u8>, u64, Vec<Limb>)> {
    Box::new(DigitsRandomGenerator {
        bases: random_values_from_vec(
            EXAMPLE_SEED.fork("bases"),
            (3u64..256).filter(|&b| !b.is_power_of_two()).collect(),
        ),
        xss: random_vecs(
            EXAMPLE_SEED.fork("xss"),
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        bytes: random_primitive_ints(EXAMPLE_SEED.fork("bytes")),
    })
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsRandomGenerator {
    bases: RandomValuesFromVec<u64>,
    xss: RandomVecs<Limb, RandomUnsignedRange<u64>, RandomPrimitiveInts<Limb>>,
    excess_lens: RandomOptions<GeometricRandomNaturalValues<usize>>,
    excess_out_lens: GeometricRandomNaturalValues<usize>,
    bytes: RandomPrimitiveInts<u8>,
}

impl Iterator for BasecaseDigitsRandomGenerator {
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
        let out = (&mut self.bytes).take(out_len).collect();
        Some((out, len, xs, base))
    }
}

pub fn random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<u8>, usize, Vec<Limb>, u64)> {
    Box::new(BasecaseDigitsRandomGenerator {
        bases: random_values_from_vec(
            EXAMPLE_SEED.fork("bases"),
            (3u64..256).filter(|&b| !b.is_power_of_two()).collect(),
        ),
        xss: random_vecs_length_range(
            EXAMPLE_SEED.fork("xss"),
            0,
            u64::exact_from(GET_STR_PRECOMPUTE_THRESHOLD),
            &random_primitive_ints,
        ),
        excess_lens: random_options(
            EXAMPLE_SEED.fork("excess_lens"),
            config.get_or("zero_len_weight_n", 1),
            config.get_or("zero_len_weight_n", 4),
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
        bytes: random_primitive_ints(EXAMPLE_SEED.fork("bytes")),
    })
}
