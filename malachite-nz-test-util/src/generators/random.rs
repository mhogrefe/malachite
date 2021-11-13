use crate::common::{
    integer_to_bigint, integer_to_rug_integer, natural_to_biguint, natural_to_rug_integer,
};
use crate::generators::exhaustive::{
    filter_helper_1, filter_helper_2, filter_helper_3, filter_map_helper_1, filter_map_helper_2,
    filter_map_helper_3, round_to_multiple_integer_filter_map,
    round_to_multiple_natural_filter_map,
};
use crate::generators::T8;
use crate::natural::arithmetic::gcd::half_gcd_matrix_create;
use itertools::Itertools;
use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::iterators::with_special_value;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, CeilingLogBase2, DivRound, DivisibleBy, DivisibleByPowerOf2, EqMod,
    EqModPowerOf2, Parity, PowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::{BitConvertible, SignificantBits};
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_signed_range, geometric_random_signeds,
    geometric_random_unsigneds, GeometricRandomNaturalValues, GeometricRandomSignedRange,
    GeometricRandomSigneds,
};
use malachite_base::num::random::{
    random_natural_signeds, random_positive_unsigneds, random_primitive_ints,
    random_unsigned_bit_chunks, random_unsigned_inclusive_range, random_unsigneds_less_than,
    variable_range_generator, RandomPrimitiveInts, RandomUnsignedBitChunks, RandomUnsignedRange,
    RandomUnsignedsLessThan, VariableRangeGenerator,
};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_quadruples_from_single, random_quadruples_xxxy,
    random_quadruples_xyyx, random_quadruples_xyyz, random_triples, random_triples_from_single,
    random_triples_xxy, random_triples_xyx, random_triples_xyy,
};
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;
use malachite_base::vecs::random::{
    random_vecs, random_vecs_length_range, random_vecs_min_length, RandomVecs,
};
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use malachite_base_test_util::generators::common::{
    permute_1_3_2, reshape_1_3_to_4, GenConfig, It,
};
use malachite_base_test_util::generators::random::{
    random_primitive_int_vec_unsigned_pair_gen_var_10, PrimitiveIntVecTripleLenGenerator1,
    PrimitiveIntVecTripleXYYLenGenerator,
};
use malachite_nz::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use malachite_nz::integer::random::{
    random_integers, random_natural_integers, random_negative_integers, random_nonzero_integers,
    RandomIntegers,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::gcd::half_gcd::{HalfGcdMatrix, HalfGcdMatrix1};
use malachite_nz::natural::arithmetic::mul::fft::*;
use malachite_nz::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
};
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_per_digit_in_base, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::random::{
    get_random_natural_with_up_to_bits, random_natural_range_to_infinity, random_naturals,
    random_positive_naturals, RandomNaturalRangeToInfinity, RandomNaturals,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Shl, Shr};

// -- Integer --

pub fn random_integer_gen(config: &GenConfig) -> It<Integer> {
    Box::new(random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_integer_gen_var_1<T: PrimitiveFloat>(config: &GenConfig) -> It<Integer> {
    Box::new(with_special_value(
        EXAMPLE_SEED,
        Integer::ZERO,
        1,
        100,
        &|seed| {
            random_pairs(
                seed,
                &|seed_2| {
                    random_positive_float_naturals::<T>(
                        seed_2,
                        0,
                        config.get_or("exponent_mean_n", 8),
                        config.get_or("exponent_mean_d", 1),
                    )
                },
                &random_bools,
            )
            .map(|(n, b)| Integer::from_sign_and_abs(b, n))
        },
    ))
}

pub fn random_integer_gen_var_2<T: PrimitiveFloat>(config: &GenConfig) -> It<Integer>
where
    for<'a> T: ConvertibleFrom<&'a Natural>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_natural_range_to_infinity(
                    seed,
                    Natural::power_of_2(T::MANTISSA_WIDTH + 1) | Natural::ONE,
                    config.get_or("mean_bits_n", Limb::WIDTH << 1),
                    config.get_or("mean_bits_d", 1),
                )
                .filter(|n| !T::convertible_from(n))
            },
            &random_bools,
        )
        .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
    )
}

pub fn random_integer_gen_var_3<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    config: &GenConfig,
) -> It<Integer>
where
    Natural: ExactFrom<T>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_positive_float_naturals::<T>(
                    seed,
                    1,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                )
                .filter_map(|a| {
                    let b = Natural::exact_from(T::exact_from(&a).next_higher());
                    let diff = b - &a;
                    if diff.even() {
                        // This happens almost always
                        Some(a + (diff >> 1))
                    } else {
                        None
                    }
                })
            },
            &random_bools,
        )
        .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
    )
}

pub fn random_integer_gen_var_4(config: &GenConfig) -> It<Integer> {
    Box::new(random_natural_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_integer_gen_var_5<T: PrimitiveInt>(_config: &GenConfig) -> It<Integer>
where
    Integer: From<T>,
{
    Box::new(random_primitive_ints(EXAMPLE_SEED).map(Integer::from))
}

pub fn random_integer_gen_var_6(config: &GenConfig) -> It<Integer> {
    Box::new(random_negative_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_integer_gen_var_7(config: &GenConfig) -> It<Integer> {
    Box::new(random_nonzero_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- (Integer, Integer) --

pub fn random_integer_pair_gen(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(random_pairs_from_single(random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_integer_pair_gen_var_1(config: &GenConfig) -> It<(Integer, Integer)> {
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
            random_nonzero_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_integer_pair_gen_var_2(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_nonzero_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(x, y)| (x * &y, y)),
    )
}

pub fn random_integer_pair_gen_var_3(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_nonzero_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|(x, y)| !x.divisible_by(y)),
    )
}

// -- (Integer, Integer, Integer) --

pub fn random_integer_triple_gen(config: &GenConfig) -> It<(Integer, Integer, Integer)> {
    Box::new(random_triples_from_single(random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_integer_triple_gen_var_1(config: &GenConfig) -> It<(Integer, Integer, Integer)> {
    Box::new(random_triples_from_single(random_natural_integers(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

// -- (Integer, Integer, Integer, PrimitiveUnsigned) --

pub fn random_integer_integer_integer_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, Integer, T)> {
    Box::new(random_quadruples_xxxy(
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

// -- (Integer, Integer, Natural) --

pub fn random_integer_integer_natural_triple_gen(
    config: &GenConfig,
) -> It<(Integer, Integer, Natural)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_integer_integer_natural_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, Integer, Natural)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(x, y, m)| (x * Integer::from(&m) + &y, y, m)),
    )
}

pub fn random_integer_integer_natural_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, Integer, Natural)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|&(ref x, ref y, ref m)| !x.eq_mod(y, m)),
    )
}

// -- (Integer, Integer, PrimitiveUnsigned) --

pub fn random_integer_integer_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(random_triples_xxy(
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

pub fn random_integer_integer_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        random_triples_xxy(
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
        )
        .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

pub fn random_integer_integer_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_2(y, pow.exact_into())),
    )
}

// -- (Integer, Integer, RoundingMode) --

pub fn random_integer_integer_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, Integer, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_nonzero_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(x, y, rm)| {
            if rm == RoundingMode::Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub fn random_integer_integer_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, Integer, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_nonzero_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_integer_filter_map(x, y, rm)),
    )
}

// -- (Integer, Natural) --

pub fn random_integer_natural_pair_gen(config: &GenConfig) -> It<(Integer, Natural)> {
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
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, Natural, Integer) --

pub fn random_integer_natural_integer_triple_gen(
    config: &GenConfig,
) -> It<(Integer, Natural, Integer)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveInt) --

pub fn random_integer_primitive_int_pair_gen<T: PrimitiveInt>(
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
        &random_primitive_ints,
    ))
}

// -- (Integer, PrimitiveInt, Integer) --

pub fn random_integer_primitive_int_integer_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Integer, T, Integer)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// -- (Integer, PrimitiveInt, Natural) --

pub fn random_integer_primitive_int_natural_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Integer, T, Natural)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_primitive_ints,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveSigned, RoundingMode) --

pub fn random_integer_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shr<T, Output = Integer>,
{
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_signeds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, i, rm)| {
            (
                if i < T::ZERO && rm == RoundingMode::Exact {
                    n >> i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

pub fn random_integer_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_signeds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, i, rm)| {
            (
                if i > T::ZERO && rm == RoundingMode::Exact {
                    n << i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

// -- (Integer, PrimitiveUnsigned) --

pub fn random_integer_signed_pair_gen_var_1<T: PrimitiveSigned>(
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
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
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

pub fn random_integer_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_natural_integers(
                            seed_2,
                            config.get_or("mean_bits_n", 64),
                            config.get_or("mean_bits_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_positive_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                    },
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_negative_integers(
                            seed_2,
                            config.get_or("mean_bits_n", 64),
                            config.get_or("mean_bits_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds::<T>(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                        .flat_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE))
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_integer_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(mut x, y)| {
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), RoundingMode::Down);
            (x, y)
        }),
    )
}

pub fn random_integer_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

// -- (Integer, PrimitiveUnsigned, bool) --

pub fn random_integer_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, bool)> {
    Box::new(random_triples(
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
        &random_bools,
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

pub fn random_integer_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(
        random_triples_xyy(
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
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
}

pub fn random_integer_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(random_triples_xyy(
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

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

pub fn random_integer_unsigned_unsigned_natural_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, T, Natural)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z, w)| match y.cmp(&z) {
            Ordering::Less => Some((x, y, z, w)),
            Ordering::Greater => Some((x, z, y, w)),
            Ordering::Equal => None,
        }),
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

pub fn random_integer_unsigned_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<u64>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, u, rm)| {
            if rm == RoundingMode::Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

pub fn random_integer_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        random_triples(
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
            &random_rounding_modes,
        )
        .map(|(n, u, rm)| (if rm == RoundingMode::Exact { n << u } else { n }, u, rm)),
    )
}

// -- (Integer, RoundingMode) --

pub fn random_integer_rounding_mode_pair_gen(config: &GenConfig) -> It<(Integer, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn random_integer_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Integer> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(Integer, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, rm)| rm != RoundingMode::Exact || T::convertible_from(n)),
    )
}

pub fn random_integer_rounding_mode_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_nonzero_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// --(Integer, Vec<bool>) --

struct IntegerBoolVecPairGenerator1 {
    xs: RandomIntegers<GeometricRandomSigneds<i64>>,
    bs: RandomBools,
}

impl Iterator for IntegerBoolVecPairGenerator1 {
    type Item = (Integer, Vec<bool>);

    fn next(&mut self) -> Option<(Integer, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = (&mut self.bs)
            .take(x.to_twos_complement_limbs_asc().len())
            .collect();
        Some((x, bs))
    }
}

pub fn random_integer_bool_vec_pair_gen_var_1(config: &GenConfig) -> It<(Integer, Vec<bool>)> {
    Box::new(IntegerBoolVecPairGenerator1 {
        xs: random_integers(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

struct IntegerBoolVecPairGenerator2 {
    xs: RandomIntegers<GeometricRandomSigneds<i64>>,
    bs: RandomBools,
}

impl Iterator for IntegerBoolVecPairGenerator2 {
    type Item = (Integer, Vec<bool>);

    fn next(&mut self) -> Option<(Integer, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = (&mut self.bs).take(x.to_bits_asc().len()).collect();
        Some((x, bs))
    }
}

pub fn random_integer_bool_vec_pair_gen_var_2(config: &GenConfig) -> It<(Integer, Vec<bool>)> {
    Box::new(IntegerBoolVecPairGenerator2 {
        xs: random_integers(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
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

pub fn random_natural_gen_var_2(config: &GenConfig) -> It<Natural> {
    Box::new(random_positive_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

struct RandomPositiveFloatNaturals<T: PrimitiveFloat> {
    exponents: GeometricRandomSignedRange<i64>,
    ranges: VariableRangeGenerator,
    phantom: PhantomData<T>,
}

impl<T: PrimitiveFloat> Iterator for RandomPositiveFloatNaturals<T> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let exponent = self.exponents.next().unwrap();
        let a = if exponent == 0 {
            1
        } else {
            u64::power_of_2(T::MANTISSA_WIDTH)
        };
        let mantissa = self
            .ranges
            .next_in_range(a, u64::power_of_2(T::MANTISSA_WIDTH + 1));
        Some(Natural::from(mantissa) << exponent)
    }
}

fn random_positive_float_naturals<T: PrimitiveFloat>(
    seed: Seed,
    start_exponent: i64,
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
) -> RandomPositiveFloatNaturals<T> {
    RandomPositiveFloatNaturals {
        exponents: geometric_random_signed_range(
            seed.fork("exponents"),
            start_exponent,
            i64::power_of_2(T::EXPONENT_WIDTH - 1) - i64::wrapping_from(T::MANTISSA_WIDTH) - 1,
            mean_exponent_numerator,
            mean_exponent_denominator,
        ),
        ranges: variable_range_generator(seed.fork("mantissas")),
        phantom: PhantomData,
    }
}

pub fn random_natural_gen_var_3<T: PrimitiveFloat>(config: &GenConfig) -> It<Natural> {
    Box::new(with_special_value(
        EXAMPLE_SEED,
        Natural::ZERO,
        1,
        100,
        &|seed| {
            random_positive_float_naturals::<T>(
                seed,
                0,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
            )
        },
    ))
}

pub fn random_natural_gen_var_4<T: PrimitiveFloat>(config: &GenConfig) -> It<Natural>
where
    for<'a> T: ConvertibleFrom<&'a Natural>,
{
    Box::new(
        random_natural_range_to_infinity(
            EXAMPLE_SEED,
            Natural::power_of_2(T::MANTISSA_WIDTH + 1) | Natural::ONE,
            config.get_or("mean_bits_n", Limb::WIDTH << 1),
            config.get_or("mean_bits_d", 1),
        )
        .filter(|n| !T::convertible_from(n)),
    )
}

pub fn random_natural_gen_var_5<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    config: &GenConfig,
) -> It<Natural>
where
    Natural: ExactFrom<T>,
{
    Box::new(
        random_positive_float_naturals::<T>(
            EXAMPLE_SEED,
            1,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
        )
        .filter_map(|a| {
            let b = Natural::exact_from(T::exact_from(&a).next_higher());
            let diff = b - &a;
            if diff.even() {
                // This happens almost always
                Some(a + (diff >> 1))
            } else {
                None
            }
        }),
    )
}

pub fn random_natural_gen_var_6<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<Natural>
where
    Natural: From<T>,
{
    Box::new(random_primitive_ints(EXAMPLE_SEED).map(Natural::from))
}

pub fn random_natural_gen_var_7<T: PrimitiveSigned>(_config: &GenConfig) -> It<Natural>
where
    Natural: ExactFrom<T>,
{
    Box::new(random_natural_signeds(EXAMPLE_SEED).map(Natural::exact_from))
}

// -- (Natural, Integer, Natural) --

pub fn random_natural_integer_natural_triple_gen(
    config: &GenConfig,
) -> It<(Natural, Integer, Natural)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
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
                Natural::power_of_2(Limb::WIDTH),
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

pub fn random_natural_pair_gen_var_3(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_positive_naturals(
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

pub fn random_natural_pair_gen_var_4(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| (x * &y, y * z)),
    )
}

pub fn random_natural_pair_gen_var_5(config: &GenConfig) -> It<(Natural, Natural)> {
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
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_natural_pair_gen_var_6(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_positive_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(x, y)| (x * &y, y)),
    )
}

pub fn random_natural_pair_gen_var_7(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_positive_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|(x, y)| !x.divisible_by(y)),
    )
}

// -- (Natural, Natural, Natural) --

pub fn random_natural_triple_gen(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(random_triples_from_single(random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

// -- (Natural, Natural, PrimitiveUnsigned) --

pub fn random_natural_natural_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(random_triples_xxy(
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

// -- (Natural, Natural, RoundingMode) --

pub fn random_natural_natural_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_positive_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(x, y, rm)| {
            if rm == RoundingMode::Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub fn random_natural_natural_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_positive_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_natural_filter_map(x, y, rm)),
    )
}

// -- (Natural, PrimitiveInt) --

pub fn random_natural_primitive_int_pair_gen<T: PrimitiveInt>(
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
        &random_primitive_ints,
    ))
}

// -- (Natural, PrimitiveInt, Natural) --

pub fn random_natural_primitive_int_natural_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, T, Natural)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// -- (Natural, PrimitiveSigned) --

pub fn random_natural_signed_pair_gen_var_1<T: PrimitiveSigned>(
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
        &random_natural_signeds,
    ))
}

pub fn random_natural_signed_pair_gen_var_2<T: PrimitiveSigned>(
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
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Natural, PrimitiveSigned, RoundingMode) --

pub fn random_natural_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shr<T, Output = Natural>,
{
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_signeds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, i, rm)| {
            (
                if i < T::ZERO && rm == RoundingMode::Exact {
                    n >> i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

pub fn random_natural_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_signeds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, i, rm)| {
            (
                if i > T::ZERO && rm == RoundingMode::Exact {
                    n << i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
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

pub fn random_natural_unsigned_pair_gen_var_6<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, 1, T::WIDTH),
    ))
}

pub fn random_natural_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>(
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
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_natural_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Natural, PrimitiveUnsigned, bool) --

pub fn random_natural_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, bool)> {
    Box::new(random_triples(
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
        &random_bools,
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

pub fn random_natural_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, u64, T)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, 1, U::WIDTH),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_natural_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
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
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
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

pub fn random_natural_unsigned_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(
        random_triples_xyy(
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
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

pub fn random_natural_unsigned_unsigned_natural_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, T, Natural)> {
    Box::new(
        random_quadruples_xyyx::<_, _, T, _>(
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
        )
        .filter_map(|(x, y, z, w)| match y.cmp(&z) {
            Ordering::Less => Some((x, y, z, w)),
            Ordering::Greater => Some((x, z, y, w)),
            Ordering::Equal => None,
        }),
    )
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

pub fn random_natural_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        random_triples(
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
                    config.get_or("mean_excess_len_n", 4),
                    config.get_or("mean_excess_len_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, u, rm)| {
            if rm == RoundingMode::Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

// --(Natural, PrimitiveUnsigned, Vec<bool>) --

struct NaturalUnsignedBoolVecTripleGenerator {
    xs: RandomNaturals<GeometricRandomNaturalValues<u64>>,
    log_bases: It<u64>,
    bs: RandomBools,
}

impl Iterator for NaturalUnsignedBoolVecTripleGenerator {
    type Item = (Natural, u64, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, u64, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let log_base = self.log_bases.next().unwrap();
        let bs = (&mut self.bs)
            .take(usize::exact_from(
                x.significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling),
            ))
            .collect();
        Some((x, log_base, bs))
    }
}

pub fn random_natural_unsigned_bool_vec_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, u64, Vec<bool>)> {
    Box::new(NaturalUnsignedBoolVecTripleGenerator {
        xs: random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        log_bases: Box::new(geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("log_bases"),
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        )),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

pub fn random_natural_unsigned_bool_vec_triple_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, u64, Vec<bool>)> {
    Box::new(NaturalUnsignedBoolVecTripleGenerator {
        xs: random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        log_bases: Box::new(random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("log_bases"),
            1,
            T::WIDTH,
        )),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

// -- (Natural, RoundingMode) --

pub fn random_natural_rounding_mode_pair_gen(config: &GenConfig) -> It<(Natural, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn random_natural_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(Natural, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, rm)| rm != RoundingMode::Exact || T::convertible_from(n)),
    )
}

pub fn random_natural_rounding_mode_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// --(Natural, Vec<bool>) --

struct NaturalBoolVecPairGenerator1 {
    xs: RandomNaturals<GeometricRandomNaturalValues<u64>>,
    bs: RandomBools,
}

impl Iterator for NaturalBoolVecPairGenerator1 {
    type Item = (Natural, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = (&mut self.bs)
            .take(usize::exact_from(x.limb_count()))
            .collect();
        Some((x, bs))
    }
}

pub fn random_natural_bool_vec_pair_gen_var_1(config: &GenConfig) -> It<(Natural, Vec<bool>)> {
    Box::new(NaturalBoolVecPairGenerator1 {
        xs: random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

struct NaturalBoolVecPairGenerator2 {
    xs: RandomNaturals<GeometricRandomNaturalValues<u64>>,
    bs: RandomBools,
}

impl Iterator for NaturalBoolVecPairGenerator2 {
    type Item = (Natural, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = (&mut self.bs)
            .take(usize::exact_from(x.significant_bits()))
            .collect();
        Some((x, bs))
    }
}

pub fn random_natural_bool_vec_pair_gen_var_2(config: &GenConfig) -> It<(Natural, Vec<bool>)> {
    Box::new(NaturalBoolVecPairGenerator2 {
        xs: random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

// -- (PrimitiveInt, Integer, PrimitiveInt) --

pub fn random_primitive_int_integer_primitive_int_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, Integer, T)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (PrimitiveInt, Natural, PrimitiveInt) --

pub fn random_primitive_int_natural_primitive_int_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, Natural, T)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (String, String, String) --

pub fn random_string_triple_gen_var_1(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|x| {
            (
                serde_json::to_string(&natural_to_biguint(&x)).unwrap(),
                serde_json::to_string(&natural_to_rug_integer(&x)).unwrap(),
                serde_json::to_string(&x).unwrap(),
            )
        }),
    )
}

pub fn random_string_triple_gen_var_2(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        random_integers(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|x| {
            (
                serde_json::to_string(&integer_to_bigint(&x)).unwrap(),
                serde_json::to_string(&integer_to_rug_integer(&x)).unwrap(),
                serde_json::to_string(&x).unwrap(),
            )
        }),
    )
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
        let bits = base.ceiling_log_base_2();
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
            Natural::power_of_2(Limb::WIDTH),
            Limb::WIDTH + 4,
            1,
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
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
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_natural_vec_natural_pair_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    random_naturals(
                        seed_2,
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_digit_count_n", 4),
                config.get_or("mean_digit_count_d", 1),
            )
        },
        &|seed| {
            random_natural_range_to_infinity(
                seed,
                Natural::power_of_2(Limb::WIDTH),
                Limb::WIDTH + 4,
                1,
            )
        },
    ))
}

pub fn random_natural_vec_natural_pair_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    random_naturals(
                        seed_2,
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_digit_count_n", 4),
                config.get_or("mean_digit_count_d", 1),
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

// -- (Vec<Natural>, PrimitiveUnsigned) --

struct PowerOfTwoDigitsGenerator {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<u64>,
}

impl Iterator for PowerOfTwoDigitsGenerator {
    type Item = (Vec<Natural>, u64);

    fn next(&mut self) -> Option<(Vec<Natural>, u64)> {
        let log_base = self.log_bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let mut digits = Vec::with_capacity(digit_count);
        for _ in 0..digit_count {
            digits.push(get_random_natural_with_up_to_bits(&mut self.xs, log_base));
        }
        Some((digits, log_base))
    }
}

pub fn random_natural_vec_unsigned_pair_gen_var_1(config: &GenConfig) -> It<(Vec<Natural>, u64)> {
    Box::new(PowerOfTwoDigitsGenerator {
        log_bases: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("log_bases"),
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_count"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_natural_vec_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<Natural>, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    random_naturals(
                        seed_2,
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
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
            config.get_or("zero_len_prob_n", 1),
            config.get_or("zero_len_prob_d", 5),
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

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>, Vec<PrimitiveInt>) --

// vars 1 through 3 are in malachite-base

fn random_mul_helper<T: PrimitiveInt, F: Fn(usize, usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: usize,
    min_y: usize,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator1 {
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
        &limbs_mul_greater_to_out_toom_22_input_sizes_valid,
        2,
        2,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_5<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_32_input_sizes_valid,
        6,
        4,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_6<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        3,
        3,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_7<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_42_input_sizes_valid,
        4,
        2,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_8<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_43_input_sizes_valid,
        11,
        8,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_9<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_44_input_sizes_valid,
        4,
        4,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_10<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_52_input_sizes_valid,
        14,
        5,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_11<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_53_input_sizes_valid,
        5,
        3,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_12<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_54_input_sizes_valid,
        14,
        11,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_13<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_62_input_sizes_valid,
        6,
        2,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_14<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_63_input_sizes_valid,
        17,
        9,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_15<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
        42,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_16<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
        86,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_17<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &limbs_mul_greater_to_out_fft_input_sizes_threshold,
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
        &limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        5,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_19<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_20<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
    )
}

pub fn random_primitive_int_vec_triple_gen_var_21<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_same_length_helper(
        config,
        &|xs_len, ys_len| {
            limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len, ys_len)
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
            limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len, ys_len)
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
            limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len, ys_len)
        },
        5,
        3,
    )
}

// vars 24 through 35 are in malachite-base

pub fn random_primitive_int_vec_triple_gen_var_36(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_triples_from_single(
            random_vecs_min_length(
                EXAMPLE_SEED,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter_map(filter_map_helper_3),
    )
}

pub fn random_primitive_int_vec_triple_gen_var_37(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_triples_from_single(
            random_vecs_min_length(
                EXAMPLE_SEED,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter(filter_helper_3),
    )
}

// var 38 is in malachite-base.

// -- (Vec<PrimitiveInt>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct BasecaseDigitsRandomGenerator1<T: PrimitiveUnsigned, U: PrimitiveInt> {
    bases: RandomValuesFromVec<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    base_to_digits: HashMap<u64, RandomUnsignedsLessThan<T>>,
    excess_limb_counts: GeometricRandomNaturalValues<usize>,
    outs: RandomPrimitiveInts<U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator for BasecaseDigitsRandomGenerator1<T, U> {
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
    Box::new(BasecaseDigitsRandomGenerator1 {
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

struct BasecaseDigitsRandomGenerator2<T: PrimitiveUnsigned, U: PrimitiveInt> {
    bases: RandomValuesFromVec<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    base_to_digits: HashMap<u64, RandomPrimitiveInts<T>>,
    excess_limb_counts: GeometricRandomNaturalValues<usize>,
    outs: RandomPrimitiveInts<U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator for BasecaseDigitsRandomGenerator2<T, U> {
    type Item = (Vec<U>, Vec<T>, u64);

    fn next(&mut self) -> Option<(Vec<U>, Vec<T>, u64)> {
        let base = self.bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let ds = self
            .base_to_digits
            .entry(base)
            .or_insert_with(move || random_primitive_ints(EXAMPLE_SEED.fork(&base.to_string())));
        let digits = ds.take(digit_count).collect();
        let min_limb_count = limbs_per_digit_in_base(digit_count, base);
        let out = (&mut self.outs)
            .take(usize::exact_from(min_limb_count) + self.excess_limb_counts.next().unwrap())
            .collect();
        Some((out, digits, base))
    }
}

pub fn random_primitive_int_vec_unsigned_vec_unsigned_triple_gen_var_2<
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

// -- (Vec<PrimitiveInt>, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    2,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &random_positive_unsigneds,
        )
        .filter(|(m, x, y)| {
            !Integer::from(Natural::from(*x)).eq_mod(-Natural::from(*y), Natural::from_limbs_asc(m))
        }),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// vars 1 through 7 are in malachite-base

pub fn random_unsigned_vec_unsigned_pair_gen_var_8<
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

pub fn random_unsigned_vec_unsigned_pair_gen_var_9(config: &GenConfig) -> It<(Vec<Limb>, u64)> {
    Box::new(
        random_primitive_int_vec_unsigned_pair_gen_var_10(config).filter(|(xs, index)| {
            let mut mut_xs = xs.clone();
            limbs_vec_clear_bit_neg(&mut mut_xs, *index);
            mut_xs.len() == xs.len()
        }),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
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

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    2,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &random_positive_unsigneds,
        )))
        .filter_map(filter_map_helper_1),
    )
}

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    2,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &random_positive_unsigneds,
        )))
        .filter(filter_helper_1),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    2,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &random_positive_unsigneds,
        )
        .filter_map(filter_map_helper_2),
    )
}

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    2,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &random_positive_unsigneds,
        )
        .filter(filter_helper_2),
    )
}

// -- large types --

// vars 1 through 4 are in malachite-base

fn random_half_gcd_matrix(s: usize, n: usize, xs: &mut RandomPrimitiveInts<Limb>) -> HalfGcdMatrix {
    assert!(s >= n);
    let mut m00 = xs.take(n).collect_vec();
    let m01 = xs.take(n).collect_vec();
    let m10 = xs.take(n).collect_vec();
    let m11 = xs.take(n).collect_vec();
    m00.resize(s << 2, 0);
    m00[s..s + n].copy_from_slice(&m01);
    m00[s << 1..(s << 1) + n].copy_from_slice(&m10);
    m00[s * 3..s * 3 + n].copy_from_slice(&m11);
    half_gcd_matrix_create(s, n, m00)
}

struct HalfGcdMatrixAndVecGenerator {
    sizes: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<Limb>,
    bs: RandomBools,
}

impl Iterator for HalfGcdMatrixAndVecGenerator {
    type Item = (HalfGcdMatrix, Vec<Limb>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.sizes.next().unwrap();
            let qs_len = x.checked_add(1);
            let qs_len = if let Some(qs_len) = qs_len {
                qs_len
            } else {
                continue;
            };
            let y = self.sizes.next().unwrap();
            let m_n = qs_len.checked_add(y);
            let m_n = if let Some(m_n) = m_n { m_n } else { continue };
            let z = self.sizes.next().unwrap();
            let m_s = m_n.checked_add(z);
            let m_s = if let Some(m_s) = m_s { m_s } else { continue };
            let m_s_1 = m_s.checked_add(2);
            let m_s_1 = if let Some(m_s_1) = m_s_1 {
                m_s_1
            } else {
                continue;
            };
            let m_s_2 = m_s.checked_add(qs_len);
            let m_s_2 = if let Some(m_s_2) = m_s_2 {
                m_s_2
            } else {
                continue;
            };
            let m_s = max(m_s_1, m_s_2);
            let m = random_half_gcd_matrix(m_s, m_n, &mut self.xs);
            let qs = (&mut self.xs).take(qs_len).collect_vec();
            let column = if self.bs.next().unwrap() { 1 } else { 0 };
            return Some((m, qs, column));
        }
    }
}

pub fn random_large_type_gen_var_5(config: &GenConfig) -> It<(HalfGcdMatrix, Vec<Limb>, u8)> {
    Box::new(HalfGcdMatrixAndVecGenerator {
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_6(
    config: &GenConfig,
) -> It<(HalfGcdMatrix1, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    reshape_1_3_to_4(Box::new(
        random_quadruples_from_single(random_primitive_ints(EXAMPLE_SEED.fork("m")))
            .map(|(m00, m01, m10, m11)| HalfGcdMatrix1 {
                data: [[m00, m01], [m10, m11]],
            })
            .zip(PrimitiveIntVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
                    EXAMPLE_SEED.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .flat_map(|(x, y, z)| {
                    let xs_len = x;
                    let ys_len = x.checked_add(1)?.checked_add(y)?;
                    let out_len = x.checked_add(1)?.checked_add(z)?;
                    Some((out_len, xs_len, ys_len))
                }),
                xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
            }),
    ))
}

struct HalfGcdMatrixAndHalfGcdMatrix1Generator {
    sizes: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<Limb>,
    small_xs: RandomUnsignedBitChunks<Limb>,
}

impl Iterator for HalfGcdMatrixAndHalfGcdMatrix1Generator {
    type Item = (HalfGcdMatrix, HalfGcdMatrix1);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.sizes.next().unwrap();
            let n = x.checked_add(1);
            let n = if let Some(n) = n { n } else { continue };
            let y = self.sizes.next().unwrap();
            let s = n.checked_add(y);
            let s = if let Some(s) = s { s } else { continue };
            let s = s.checked_add(1);
            let s = if let Some(s) = s { s } else { continue };
            let m = random_half_gcd_matrix(s, n, &mut self.xs);
            let m_1 = HalfGcdMatrix1 {
                data: [
                    [self.small_xs.next().unwrap(), self.small_xs.next().unwrap()],
                    [self.small_xs.next().unwrap(), self.small_xs.next().unwrap()],
                ],
            };
            return Some((m, m_1));
        }
    }
}

pub fn random_large_type_gen_var_7(config: &GenConfig) -> It<(HalfGcdMatrix, HalfGcdMatrix1)> {
    Box::new(HalfGcdMatrixAndHalfGcdMatrix1Generator {
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        small_xs: random_unsigned_bit_chunks(EXAMPLE_SEED.fork("small_xs"), Limb::WIDTH - 1),
    })
}

struct MatrixMul22Generator {
    sizes: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<Limb>,
}

impl Iterator for MatrixMul22Generator {
    type Item = T8;

    fn next(&mut self) -> Option<Self::Item> {
        let ys_len = self.sizes.next().unwrap();
        let xs_len = self.sizes.next().unwrap();
        let sum = ys_len + xs_len + 1;
        Some((
            (&mut self.xs).take(sum).collect_vec(),
            (&mut self.xs).take(sum).collect_vec(),
            (&mut self.xs).take(sum).collect_vec(),
            (&mut self.xs).take(sum).collect_vec(),
            xs_len,
            (&mut self.xs).take(ys_len).collect_vec(),
            (&mut self.xs).take(ys_len).collect_vec(),
            (&mut self.xs).take(ys_len).collect_vec(),
            (&mut self.xs).take(ys_len).collect_vec(),
        ))
    }
}

pub fn random_large_type_gen_var_8(config: &GenConfig) -> It<T8> {
    Box::new(MatrixMul22Generator {
        sizes: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}
