use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::striped::{
    get_striped_unsigned_vec, striped_random_unsigned_vecs,
    striped_random_unsigned_vecs_length_range, striped_random_unsigned_vecs_min_length,
    StripedBitSource, StripedRandomUnsignedVecs,
};
use malachite_base::num::random::{
    random_unsigned_inclusive_range, random_unsigneds_less_than, RandomUnsignedRange,
    RandomUnsignedsLessThan,
};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_triples, random_triples_from_single,
};
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use malachite_base_test_util::generators::common::{GenConfig, It};
use malachite_base_test_util::generators::special_random::{
    UnsignedVecTripleLenGenerator, UnsignedVecTripleXYYLenGenerator,
};
use malachite_nz::integer::random::striped_random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mul::fft::*;
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
};
use malachite_nz::natural::conversion::digits::general_digits::limbs_per_digit_in_base;
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::random::{random_natural_range_to_infinity, striped_random_naturals};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::collections::HashMap;
use std::marker::PhantomData;

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

// -- (Integer, PrimitiveUnsigned) --

pub fn special_random_integer_unsigned_pair_gen_var_3<T: ExactFrom<u8> + PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::exact_from(36u8)),
    ))
}

pub fn special_random_integer_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_integer_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Integer, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::exact_from(36u8)),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
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

// -- (Natural, Natural) --

pub fn special_random_natural_pair_gen(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs_from_single(striped_random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn special_random_natural_pair_gen_var_1(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
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

// -- (Natural, Natural, Natural) --

pub fn special_random_natural_triple_gen(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(random_triples_from_single(striped_random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

// -- (Natural, PrimitiveUnsigned) --

pub fn special_random_natural_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::saturating_from(U::MAX)),
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::MAX),
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_3<T: ExactFrom<u8> + PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::exact_from(36u8)),
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Natural, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::exact_from(36u8)),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// vars 1 through 3 are in malachite-base

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T)> {
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::saturating_from(U::MAX)),
    ))
}

// var 1 5 is in malachite-base

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

struct DigitsSpecialRandomGenerator<T: PrimitiveUnsigned> {
    bases: RandomValuesFromVec<u64>,
    xss: StripedRandomUnsignedVecs<Limb, GeometricRandomNaturalValues<u64>>,
    bit_source: StripedBitSource,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for DigitsSpecialRandomGenerator<T> {
    type Item = (Vec<T>, u64, Vec<Limb>);

    fn next(&mut self) -> Option<(Vec<T>, u64, Vec<Limb>)> {
        let base = self.bases.next().unwrap();
        let xs = self.xss.next().unwrap();
        let out_len = usize::exact_from(limbs_digit_count(&xs, base));
        let out = get_striped_unsigned_vec(
            &mut self.bit_source,
            u64::exact_from(out_len) << T::LOG_WIDTH,
        );
        Some((out, base, xs))
    }
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, u64, Vec<Limb>)> {
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
        phantom: PhantomData,
    })
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsSpecialRandomGenerator<T: PrimitiveUnsigned> {
    bases: RandomValuesFromVec<u64>,
    xss: StripedRandomUnsignedVecs<Limb, RandomUnsignedRange<u64>>,
    excess_lens: RandomOptions<GeometricRandomNaturalValues<usize>>,
    excess_out_lens: GeometricRandomNaturalValues<usize>,
    bit_source: StripedBitSource,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for BasecaseDigitsSpecialRandomGenerator<T> {
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
        let out = get_striped_unsigned_vec(
            &mut self.bit_source,
            u64::exact_from(out_len) << T::LOG_WIDTH,
        );
        Some((out, len, xs, base))
    }
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, usize, Vec<Limb>, u64)> {
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
        phantom: PhantomData,
    })
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsSpecialRandomGenerator2<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    bases: RandomValuesFromVec<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    base_to_digits: HashMap<u64, RandomUnsignedsLessThan<T>>,
    excess_limb_counts: GeometricRandomNaturalValues<u64>,
    bit_source: StripedBitSource,
    phantom: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for BasecaseDigitsSpecialRandomGenerator2<T, U>
{
    type Item = (Vec<U>, Vec<T>, u64);

    fn next(&mut self) -> Option<(Vec<U>, Vec<T>, u64)> {
        let base = self.bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let ds = self.base_to_digits.entry(base).or_insert_with(move || {
            random_unsigneds_less_than(EXAMPLE_SEED.fork(&base.to_string()), T::wrapping_from(base))
        });
        let digits = ds.take(digit_count).collect();
        let min_limb_count = limbs_per_digit_in_base(digit_count, base);
        let out = get_striped_unsigned_vec(
            &mut self.bit_source,
            (min_limb_count + self.excess_limb_counts.next().unwrap()) << U::LOG_WIDTH,
        );
        Some((out, digits, base))
    }
}

// var 1 is in malachite-base

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(BasecaseDigitsSpecialRandomGenerator2 {
        bases: random_values_from_vec(
            EXAMPLE_SEED.fork("bases"),
            (3u64..256).filter(|&b| !b.is_power_of_two()).collect(),
        ),
        digit_counts: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        excess_limb_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("excess_limb_count"),
            config.get_or("mean_excess_limb_count_n", 4),
            config.get_or("mean_excess_limb_count_d", 1),
        ),
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
        base_to_digits: HashMap::new(),
        phantom: PhantomData,
    })
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 3 are in malachite-base

fn special_random_mul_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: u64,
    min_y: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .flat_map(move |(o, x, y)| {
            let x = x.checked_add(min_x)?;
            let y = y.checked_add(min_y)?;
            if valid(usize::exact_from(x), usize::exact_from(y)) {
                let o = x.checked_add(y)?.checked_add(o)?;
                Some((o, x, y))
            } else {
                None
            }
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_22_input_sizes_valid,
        2,
        2,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_32_input_sizes_valid,
        6,
        4,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        3,
        3,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_42_input_sizes_valid,
        4,
        2,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_43_input_sizes_valid,
        11,
        8,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_44_input_sizes_valid,
        4,
        4,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_52_input_sizes_valid,
        14,
        5,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_53_input_sizes_valid,
        5,
        3,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_54_input_sizes_valid,
        14,
        11,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_62_input_sizes_valid,
        6,
        2,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_63_input_sizes_valid,
        17,
        9,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
        42,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
        86,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_17<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_fft_input_sizes_threshold,
        15,
        15,
    )
}

fn special_random_mul_same_length_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .flat_map(move |(o, x)| {
            let x = x.checked_add(min_x)?;
            let ux = usize::exact_from(x);
            if valid(ux, ux) {
                let o = x.arithmetic_checked_shl(1u64)?.checked_add(o)?;
                Some((o, x))
            } else {
                None
            }
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_18<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_same_length_helper(
        config,
        &_limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        5,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_19<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_same_length_helper(
        config,
        &_limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_20<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_same_length_helper(
        config,
        &_limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_21<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_same_length_helper(
        config,
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len, ys_len)
        },
        86,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_22<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len, ys_len)
        },
        11,
        8,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_23<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len, ys_len)
        },
        5,
        3,
    )
}

// vars 24 through 27 are in malachite-base
