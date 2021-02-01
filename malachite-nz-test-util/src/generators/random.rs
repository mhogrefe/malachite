use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::random::geometric::{
    geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::{
    random_primitive_ints, random_unsigned_inclusive_range, RandomPrimitiveInts,
    RandomUnsignedRange,
};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::tuples::random::random_pairs;
use malachite_base::vecs::random::{
    random_vecs, random_vecs_length_range, random_vecs_min_length, RandomVecs,
};
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use malachite_base_test_util::generators::common::{GenConfig, It};
use malachite_nz::integer::random::random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::random::{random_natural_range_to_infinity, random_naturals};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// -- Integer --

pub fn random_integer_gen(config: &GenConfig) -> It<Integer> {
    Box::new(random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- Natural --

pub fn random_natural_gen(config: &GenConfig) -> It<Natural> {
    Box::new(random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_natural_gen_var_1(config: &GenConfig) -> It<Natural> {
    Box::new(random_natural_range_to_infinity(
        EXAMPLE_SEED,
        Natural::TWO,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- (Natural, Natural) --

pub fn random_natural_pair_gen_var_1(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_natural_range_to_infinity(
                seed,
                Natural::power_of_two(Limb::WIDTH),
                config.get_or("mean_bits_n", 64 + Limb::WIDTH),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_natural_pair_gen_var_2(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Natural, PrimitiveUnsigned) --

pub fn random_natural_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::saturating_from(U::MAX)),
    ))
}

pub fn random_natural_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::MAX),
    ))
}

// -- (Vec<PrimitiveInt>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

struct DigitsRandomGenerator<T: PrimitiveInt> {
    bases: RandomValuesFromVec<u64>,
    xss: RandomVecs<Limb, GeometricRandomNaturalValues<u64>, RandomPrimitiveInts<Limb>>,
    outs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt> Iterator for DigitsRandomGenerator<T> {
    type Item = (Vec<T>, u64, Vec<Limb>);

    fn next(&mut self) -> Option<(Vec<T>, u64, Vec<Limb>)> {
        let base = self.bases.next().unwrap();
        let xs = self.xss.next().unwrap();
        let out_len = usize::exact_from(limbs_digit_count(&xs, base));
        let out = (&mut self.outs).take(out_len).collect();
        Some((out, base, xs))
    }
}

pub fn random_primitive_int_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, u64, Vec<Limb>)> {
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
        outs: random_primitive_ints(EXAMPLE_SEED.fork("bytes")),
    })
}

// -- (Vec<PrimitiveInt>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsRandomGenerator<T: PrimitiveInt> {
    bases: RandomValuesFromVec<u64>,
    xss: RandomVecs<Limb, RandomUnsignedRange<u64>, RandomPrimitiveInts<Limb>>,
    excess_lens: RandomOptions<GeometricRandomNaturalValues<usize>>,
    excess_out_lens: GeometricRandomNaturalValues<usize>,
    outs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt> Iterator for BasecaseDigitsRandomGenerator<T> {
    type Item = (Vec<T>, usize, Vec<Limb>, u64);

    fn next(&mut self) -> Option<(Vec<T>, usize, Vec<Limb>, u64)> {
        let base = self.bases.next().unwrap();
        let xs = self.xss.next().unwrap();
        let min_out_len = usize::exact_from(limbs_digit_count(&xs, base));
        let excess_out_len = self.excess_out_lens.next().unwrap();
        let (len, out_len) = if let Some(excess) = self.excess_lens.next().unwrap() {
            (min_out_len + excess, min_out_len + excess + excess_out_len)
        } else {
            (0, min_out_len + excess_out_len)
        };
        let out = (&mut self.outs).take(out_len).collect();
        Some((out, len, xs, base))
    }
}

pub fn random_primitive_int_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<
    T: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, usize, Vec<Limb>, u64)> {
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
        outs: random_primitive_ints(EXAMPLE_SEED.fork("bytes")),
    })
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn random_unsigned_vec_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::saturating_from(U::MAX)),
    ))
}
