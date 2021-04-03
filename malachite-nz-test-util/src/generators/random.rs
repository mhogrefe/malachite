use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, CeilingLogTwo, PowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::{
    random_primitive_ints, random_unsigned_inclusive_range, random_unsigneds_less_than,
    RandomPrimitiveInts, RandomUnsignedRange, RandomUnsignedsLessThan,
};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_triples, random_triples_from_single,
};
use malachite_base::vecs::random::{
    random_vecs, random_vecs_length_range, random_vecs_min_length, RandomVecs,
};
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use malachite_base_test_util::generators::common::{GenConfig, It};
use malachite_base_test_util::generators::random::{
    PrimitiveIntVecTripleLenGenerator, PrimitiveIntVecTripleXYYLenGenerator,
};
use malachite_nz::integer::random::random_integers;
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
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_per_digit_in_base, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::random::{
    get_random_natural_with_up_to_bits, random_natural_range_to_infinity, random_naturals,
    RandomNaturalRangeToInfinity,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::collections::HashMap;
use std::marker::PhantomData;

// -- Integer --

pub fn random_integer_gen(config: &GenConfig) -> It<Integer> {
    Box::new(random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- (Integer, PrimitiveUnsigned) --

pub fn random_integer_unsigned_pair_gen_var_1<T: ExactFrom<u8> + PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::exact_from(36u8)),
    ))
}

pub fn random_integer_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
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

pub fn random_integer_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Integer, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
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

pub fn random_natural_pair_gen(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs_from_single(random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

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

// -- (Natural, Natural, Natural) --

pub fn random_natural_triple_gen(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(random_triples_from_single(random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
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

pub fn random_natural_unsigned_pair_gen_var_3<T: ExactFrom<u8> + PrimitiveUnsigned>(
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::exact_from(36u8)),
    ))
}

pub fn random_natural_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>(
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
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_natural_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_natural_range_to_infinity(
                seed,
                Natural::TWO,
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

pub fn random_natural_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Natural, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
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

// -- (Vec<Natural>, Natural> --

struct LargeDigitsRandomGenerator {
    bases: RandomNaturalRangeToInfinity,
    digit_counts: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<u64>,
}

impl Iterator for LargeDigitsRandomGenerator {
    type Item = (Vec<Natural>, Natural);

    fn next(&mut self) -> Option<(Vec<Natural>, Natural)> {
        let base = self.bases.next().unwrap();
        let bits = base.ceiling_log_two();
        let digit_count = self.digit_counts.next().unwrap();
        let mut digits = Vec::with_capacity(digit_count);
        for _ in 0..digit_count {
            loop {
                let x = get_random_natural_with_up_to_bits(&mut self.xs, bits);
                if x < base {
                    digits.push(x);
                    break;
                }
            }
        }
        Some((digits, base))
    }
}

pub fn random_natural_vec_natural_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(LargeDigitsRandomGenerator {
        bases: random_natural_range_to_infinity(
            EXAMPLE_SEED.fork("bases"),
            Natural::power_of_two(Limb::WIDTH),
            Limb::WIDTH + 4,
            1,
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("digit_counts_mean_n", 4),
            config.get_or("digit_counts_mean_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_natural_vec_natural_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(LargeDigitsRandomGenerator {
        bases: random_natural_range_to_infinity(
            EXAMPLE_SEED.fork("bases"),
            Natural::TWO,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("digit_counts_mean_n", 4),
            config.get_or("digit_counts_mean_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
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
        outs: random_primitive_ints(EXAMPLE_SEED.fork("outs")),
    })
}

// -- (Vec<PrimitiveInt>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsRandomGenerator2<T: PrimitiveUnsigned, U: PrimitiveInt> {
    bases: RandomValuesFromVec<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    base_to_digits: HashMap<u64, RandomUnsignedsLessThan<T>>,
    excess_limb_counts: GeometricRandomNaturalValues<usize>,
    outs: RandomPrimitiveInts<U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator for BasecaseDigitsRandomGenerator2<T, U> {
    type Item = (Vec<U>, Vec<T>, u64);

    fn next(&mut self) -> Option<(Vec<U>, Vec<T>, u64)> {
        let base = self.bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let ds = self.base_to_digits.entry(base).or_insert_with(move || {
            random_unsigneds_less_than(EXAMPLE_SEED.fork(&base.to_string()), T::wrapping_from(base))
        });
        let digits = ds.take(digit_count).collect();
        let min_limb_count = limbs_per_digit_in_base(digit_count, base);
        let out = (&mut self.outs)
            .take(usize::exact_from(min_limb_count) + self.excess_limb_counts.next().unwrap())
            .collect();
        Some((out, digits, base))
    }
}

pub fn random_primitive_int_vec_unsigned_vec_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(BasecaseDigitsRandomGenerator2 {
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
            config.get_or("excess_limb_count_n", 4),
            config.get_or("excess_limb_count_d", 1),
        ),
        outs: random_primitive_ints(EXAMPLE_SEED.fork("outs")),
        base_to_digits: HashMap::new(),
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

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>, Vec<PrimitiveInt>) --

// vars 1 through 3 are in malachite-base

fn random_mul_helper<T: PrimitiveInt, F: Fn(usize, usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: usize,
    min_y: usize,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_4<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_22_input_sizes_valid,
        2,
        2,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_5<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_32_input_sizes_valid,
        6,
        4,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_6<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        3,
        3,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_7<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_42_input_sizes_valid,
        4,
        2,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_8<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_43_input_sizes_valid,
        11,
        8,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_9<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_44_input_sizes_valid,
        4,
        4,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_10<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_52_input_sizes_valid,
        14,
        5,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_11<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_53_input_sizes_valid,
        5,
        3,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_12<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_54_input_sizes_valid,
        14,
        11,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_13<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_62_input_sizes_valid,
        6,
        2,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_14<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_63_input_sizes_valid,
        17,
        9,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_15<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
        42,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_16<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
        86,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_17<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &_limbs_mul_greater_to_out_fft_input_sizes_threshold,
        15,
        15,
    )
}

fn random_mul_same_length_helper<T: PrimitiveInt, F: Fn(usize, usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: usize,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
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
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_18<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &_limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        5,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_19<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &_limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_20<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &_limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_21<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len, ys_len)
        },
        86,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_22<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len, ys_len)
        },
        11,
        8,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_23<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
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
