// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use crate::integer::random::{
    striped_random_integers, striped_random_natural_integers, striped_random_negative_integers,
    striped_random_nonzero_integers, StripedRandomIntegers,
};
use crate::integer::Integer;
use crate::natural::arithmetic::div_exact::{
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use crate::natural::arithmetic::div_mod::{
    limbs_div_mod_barrett_is_len, limbs_div_mod_barrett_scratch_len, limbs_invert_limb,
    limbs_two_limb_inverse_helper,
};
use crate::natural::arithmetic::eq_mod::limbs_eq_mod_ref_ref_ref;
use crate::natural::arithmetic::gcd::half_gcd::HalfGcdMatrix1;
use crate::natural::arithmetic::mod_power_of_2::limbs_slice_mod_power_of_2_in_place;
use crate::natural::arithmetic::mod_power_of_2_square::SQRLO_DC_THRESHOLD_LIMIT;
use crate::natural::arithmetic::mul::fft::{
    limbs_mul_greater_to_out_fft_is_valid, limbs_square_to_out_fft_is_valid,
};
use crate::natural::arithmetic::mul::limb::limbs_vec_mul_limb_in_place;
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::mul::mul_mod::limbs_mul_mod_base_pow_n_minus_1_next_size;
use crate::natural::arithmetic::mul::toom::{
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
use crate::natural::arithmetic::square::{
    limbs_square_to_out_toom_3_input_size_valid, limbs_square_to_out_toom_4_input_size_valid,
    limbs_square_to_out_toom_6_input_size_valid, limbs_square_to_out_toom_8_input_size_valid,
};
use crate::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_per_digit_in_base, GET_STR_PRECOMPUTE_THRESHOLD,
};
use crate::natural::random::{
    get_striped_random_natural_with_bits, get_striped_random_natural_with_up_to_bits,
    striped_random_natural_range, striped_random_natural_range_to_infinity,
    striped_random_naturals, striped_random_positive_naturals, StripedRandomNaturalInclusiveRange,
    StripedRandomNaturalRangeToInfinity, StripedRandomNaturals,
};
use crate::natural::Natural;
use crate::platform::{Limb, SQR_TOOM2_THRESHOLD};
use crate::test_util::extra_variadic::{
    random_quadruples_from_single, random_quadruples_xxxy, random_quadruples_xyxz,
    random_quadruples_xyyx, random_quadruples_xyyz, random_quintuples_xyyyz,
    random_sextuples_from_single, random_triples, random_triples_from_single, random_triples_xxy,
    random_triples_xyx, random_triples_xyy,
};
use crate::test_util::generators::exhaustive::{
    filter_helper_1, filter_helper_2, filter_helper_3, filter_helper_4, filter_helper_5,
    filter_helper_6, filter_map_helper_1, filter_map_helper_2, filter_map_helper_3,
    gcd_input_filter, large_type_filter_map_1, limbs_eq_mod_map, limbs_significant_bits_helper,
    map_helper_1, map_helper_2, map_helper_3, round_to_multiple_integer_filter_map,
    round_to_multiple_natural_filter_map,
};
use crate::test_util::generators::{factors_of_limb_max, T8};
use crate::test_util::natural::arithmetic::gcd::{half_gcd_matrix_create, OwnedHalfGcdMatrix};
use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::iterators::with_special_value;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, CeilingLogBase2, CoprimeWith, DivRound, DivisibleBy, DivisibleByPowerOf2,
    EqMod, EqModPowerOf2, Parity, PowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::options::random::random_to_sci_options;
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, SaturatingFrom, ToSci, WrappingFrom,
};
use malachite_base::num::logic::traits::{
    BitAccess, BitConvertible, LeadingZeros, SignificantBits,
};
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_signed_range, geometric_random_signeds,
    geometric_random_unsigned_inclusive_range, geometric_random_unsigneds,
    GeometricRandomNaturalValues, GeometricRandomSignedRange, GeometricRandomSigneds,
};
use malachite_base::num::random::striped::{
    get_striped_bool_vec, get_striped_unsigned_vec, striped_random_natural_signeds,
    striped_random_positive_unsigneds, striped_random_signeds, striped_random_unsigned_bit_chunks,
    striped_random_unsigned_inclusive_range, striped_random_unsigned_vecs,
    striped_random_unsigned_vecs_length_range, striped_random_unsigned_vecs_min_length,
    striped_random_unsigneds, StripedBitSource, StripedRandomUnsignedBitChunks,
    StripedRandomUnsignedVecs,
};
use malachite_base::num::random::{
    random_primitive_floats, random_unsigneds_less_than, RandomUnsignedRange,
    RandomUnsignedsLessThan,
};
use malachite_base::options::random::{random_options, RandomOptions};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::{
    permute_1_3_2, reshape_1_3_to_4, reshape_2_1_to_3, reshape_2_2_to_4, GenConfig, It,
};
use malachite_base::test_util::generators::random::get_two_highest;
use malachite_base::test_util::generators::special_random::{
    special_random_unsigned_vec_unsigned_pair_gen_var_17, UnsignedVecPairLenGenerator1,
    UnsignedVecPairLenGenerator2, UnsignedVecQuadrupleLenGenerator1,
    UnsignedVecTripleLenGenerator1, UnsignedVecTripleXYYLenGenerator,
};
use malachite_base::tuples::random::{
    random_ordered_unique_pairs, random_pairs, random_pairs_from_single,
};
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;
use malachite_base::vecs::random::random_vecs;
use malachite_base::vecs::{random_values_from_vec, RandomValuesFromVec};
use num::{BigInt, BigUint};
use std::cmp::{max, Ordering::*};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Shl, Shr};

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

pub fn special_random_integer_gen_var_1<T: PrimitiveFloat>(config: &GenConfig) -> It<Integer> {
    Box::new(with_special_value(
        EXAMPLE_SEED,
        Integer::ZERO,
        1,
        100,
        &|seed| {
            random_pairs(
                seed,
                &|seed_2| {
                    special_random_positive_float_naturals::<T>(
                        seed_2,
                        0,
                        config.get_or("exponent_mean_n", 8),
                        config.get_or("exponent_mean_d", 1),
                        config.get_or("mean_stripe_n", 16),
                        config.get_or("mean_stripe_d", 1),
                    )
                },
                &random_bools,
            )
            .map(|(n, b)| Integer::from_sign_and_abs(b, n))
        },
    ))
}

pub fn special_random_integer_gen_var_2<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    config: &GenConfig,
) -> It<Integer>
where
    Natural: ExactFrom<T>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_positive_float_naturals::<T>(
                    seed,
                    1,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("mean_stripe_n", 16),
                    config.get_or("mean_stripe_d", 1),
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

pub fn special_random_integer_gen_var_3(config: &GenConfig) -> It<Integer> {
    Box::new(striped_random_natural_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn special_random_integer_gen_var_4<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Integer>
where
    Integer: From<T>,
{
    Box::new(
        striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(Integer::from),
    )
}

pub fn special_random_integer_gen_var_5<T: PrimitiveSigned>(config: &GenConfig) -> It<Integer>
where
    Integer: From<T>,
{
    Box::new(
        striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(Integer::from),
    )
}

pub fn special_random_integer_gen_var_6(config: &GenConfig) -> It<Integer> {
    Box::new(striped_random_negative_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn special_random_integer_gen_var_7(config: &GenConfig) -> It<Integer> {
    Box::new(striped_random_nonzero_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn special_random_integer_gen_var_8<T: PrimitiveFloat>(config: &GenConfig) -> It<Integer>
where
    for<'a> T: ConvertibleFrom<&'a Natural>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_natural_range_to_infinity(
                    seed,
                    Natural::power_of_2(T::MANTISSA_WIDTH + 1) | Natural::ONE,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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

pub fn special_random_integer_gen_var_9(config: &GenConfig) -> It<Integer> {
    Box::new(
        striped_random_natural_integers(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|n| (n << 1u32) | Integer::ONE),
    )
}

// -- (Integer, Integer) --

pub fn special_random_integer_pair_gen(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(random_pairs_from_single(striped_random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn special_random_integer_pair_gen_var_1(config: &GenConfig) -> It<(Integer, Integer)> {
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
            striped_random_nonzero_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_pair_gen_var_2(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs(
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
                striped_random_nonzero_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(x, y)| (x * &y, y)),
    )
}

pub fn special_random_integer_pair_gen_var_3(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs(
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
                striped_random_nonzero_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|(x, y)| !x.divisible_by(y)),
    )
}

pub fn special_random_integer_pair_gen_var_4(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs(
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
                striped_random_natural_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(a, n)| (a, (n << 1u32) | Integer::ONE)),
    )
}

pub fn special_random_integer_pair_gen_var_5(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs_from_single(striped_random_integers(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y)| x.unsigned_abs_ref().coprime_with(y.unsigned_abs_ref())),
    )
}

pub fn special_random_integer_pair_gen_var_6(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs_from_single(
            striped_random_natural_integers(
                EXAMPLE_SEED,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
            .map(|n| (n << 1u32) | Integer::ONE),
        )
        .filter(|(x, y)| x.unsigned_abs_ref().coprime_with(y.unsigned_abs_ref())),
    )
}

pub fn special_random_integer_pair_gen_var_7(config: &GenConfig) -> It<(Integer, Integer)> {
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
            geometric_random_unsigneds::<Limb>(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Integer::from)
        },
    ))
}

// -- (Integer, Integer, Integer) --

pub fn special_random_integer_triple_gen(config: &GenConfig) -> It<(Integer, Integer, Integer)> {
    Box::new(random_triples_from_single(striped_random_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn special_random_integer_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, Integer, Integer)> {
    Box::new(random_triples_from_single(striped_random_natural_integers(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn special_random_integer_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, Integer, Integer)> {
    Box::new(
        random_triples_xxy(
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
                striped_random_natural_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(a, b, n)| (a, b, (n << 1u32) | Integer::ONE)),
    )
}

pub fn special_random_integer_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Integer, Integer, Integer)> {
    Box::new(
        random_triples_xyy(
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
                striped_random_natural_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(a, m, n)| (a, (m << 1u32) | Integer::ONE, (n << 1u32) | Integer::ONE)),
    )
}

// -- (Integer, Integer, Integer, PrimitiveUnsigned) --

pub fn special_random_integer_integer_integer_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, Integer, T)> {
    Box::new(random_quadruples_xxxy(
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

// -- (Integer, Integer, Natural) --

pub fn special_random_integer_integer_natural_triple_gen(
    config: &GenConfig,
) -> It<(Integer, Integer, Natural)> {
    Box::new(random_triples_xxy(
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
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_integer_natural_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, Integer, Natural)> {
    Box::new(
        random_triples_xxy(
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
                striped_random_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(x, y, m)| (x * Integer::from(&m) + &y, y, m)),
    )
}

pub fn special_random_integer_integer_natural_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, Integer, Natural)> {
    Box::new(
        random_triples_xxy(
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
                striped_random_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|(x, y, m)| !x.eq_mod(y, m)),
    )
}

// -- (Integer, Integer, PrimitiveFloat) --

pub fn special_random_integer_integer_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(random_triples_xxy(
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
        &random_primitive_floats,
    ))
}

// -- (Integer, Integer, PrimitiveSigned) --

pub fn special_random_integer_integer_signed_triple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(random_triples_xxy(
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
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Integer, Integer, PrimitiveUnsigned) --

pub fn special_random_integer_integer_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(random_triples_xxy(
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
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_integer_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(random_triples_xxy(
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

pub fn special_random_integer_integer_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        random_triples_xxy(
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
        )
        .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

pub fn special_random_integer_integer_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, Integer, T)> {
    Box::new(
        random_triples_xxy(
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

pub fn special_random_integer_integer_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, Integer, RoundingMode)> {
    Box::new(
        random_triples(
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
                striped_random_nonzero_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(x, y, rm)| {
            if rm == Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub fn special_random_integer_integer_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, Integer, RoundingMode)> {
    Box::new(
        random_triples(
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
                striped_random_nonzero_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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

pub fn special_random_integer_natural_pair_gen(config: &GenConfig) -> It<(Integer, Natural)> {
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
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, Natural, Natural) --

pub fn special_random_integer_natural_natural_triple_gen(
    config: &GenConfig,
) -> It<(Integer, Natural, Natural)> {
    Box::new(random_triples_xyy(
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
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveFloat) --

pub fn special_random_integer_primitive_float_pair_gen<T: PrimitiveFloat>(
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
        &random_primitive_floats,
    ))
}

// -- (Integer, PrimitiveFloat, PrimitiveFloat) --

pub fn special_random_integer_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(random_triples_xyy(
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
        &random_primitive_floats,
    ))
}

// -- (Integer, PrimitiveSigned) --

pub fn special_random_integer_signed_pair_gen<T: PrimitiveSigned>(
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
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_signed_pair_gen_var_1<T: PrimitiveSigned>(
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
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveSigned, PrimitiveSigned) ---

pub fn special_random_integer_signed_signed_triple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(random_triples_xyy(
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
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveSigned, PrimitiveUnsigned) --

pub fn special_random_integer_signed_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
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
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
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

// -- (Integer, PrimitiveSigned, RoundingMode) --

pub fn special_random_integer_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shr<T, Output = Integer>,
{
    Box::new(
        random_triples(
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
                if i < T::ZERO && rm == Exact {
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

pub fn special_random_integer_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        random_triples(
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
                if i > T::ZERO && rm == Exact {
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

pub fn special_random_integer_unsigned_pair_gen<T: PrimitiveUnsigned>(
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
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
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
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::from(36u8),
                config.get_or("mean_bits_n", T::WIDTH >> 1),
                config.get_or("mean_bits_d", 1),
            )
        },
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

pub fn special_random_integer_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_natural_integers(
                            seed_2,
                            config.get_or("mean_stripe_n", 32),
                            config.get_or("mean_stripe_d", 1),
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
                        striped_random_negative_integers(
                            seed_2,
                            config.get_or("mean_stripe_n", 32),
                            config.get_or("mean_stripe_d", 1),
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
                        .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE))
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_integer_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed_2| {
                striped_random_integers(
                    seed_2,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), Down);
            (x, y)
        }),
    )
}

pub fn special_random_integer_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T)> {
    Box::new(
        random_pairs(
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

pub fn special_random_integer_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>(
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
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveUnsigned, bool) --

pub fn special_random_integer_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, bool)> {
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

// -- (Integer, PrimitiveUnsigned, Natural) --

pub fn special_random_integer_unsigned_natural_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, Natural)> {
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
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_integer_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(random_triples_xyy(
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
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::from(36u8),
                config.get_or("mean_bits_n", T::WIDTH >> 1),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(
        random_triples_xyy(
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
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, y, z)| if y <= z { (xs, y, z) } else { (xs, z, y) }),
    )
}

pub fn special_random_integer_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, T)> {
    Box::new(random_triples_xyy(
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

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

pub fn special_random_integer_unsigned_unsigned_natural_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Integer, T, T, Natural)> {
    Box::new(
        random_quadruples_xyyz(
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
                geometric_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                striped_random_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z, w)| match y.cmp(&z) {
            Less => Some((x, y, z, w)),
            Greater => Some((x, z, y, w)),
            Equal => None,
        }),
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_integer_unsigned_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_integers(
                    seed.fork("xs"),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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
            if rm == Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

pub fn special_random_integer_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        random_triples(
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
            &random_rounding_modes,
        )
        .map(|(n, u, rm)| (if rm == Exact { n << u } else { n }, u, rm)),
    )
}

// var 3 is in malachite-float.

// -- (Integer, RoundingMode) --

pub fn special_random_integer_rounding_mode_pair_gen(
    config: &GenConfig,
) -> It<(Integer, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_integers(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_integer_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Integer> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(Integer, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_integers(
                    seed.fork("xs"),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, rm)| rm != Exact || T::convertible_from(n)),
    )
}

pub fn special_random_integer_rounding_mode_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_nonzero_integers(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// -- (Integer, ToSciOptions) --

pub fn special_random_integer_to_sci_options_pair_gen(
    config: &GenConfig,
) -> It<(Integer, ToSciOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_integers(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("small_mean_n", 4),
                config.get_or("small_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_integer_to_sci_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, ToSciOptions)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_integers(
                    seed.fork("xs"),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_to_sci_options(
                    seed,
                    config.get_or("small_mean_n", 4),
                    config.get_or("small_mean_d", 1),
                )
            },
        )
        .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// --(Integer, Vec<bool>) --

struct IntegerBoolVecPairGenerator1 {
    xs: StripedRandomIntegers<GeometricRandomSigneds<i64>>,
    striped_bit_source: StripedBitSource,
}

impl Iterator for IntegerBoolVecPairGenerator1 {
    type Item = (Integer, Vec<bool>);

    fn next(&mut self) -> Option<(Integer, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = get_striped_bool_vec(
            &mut self.striped_bit_source,
            u64::exact_from(x.to_twos_complement_limbs_asc().len()),
        );
        Some((x, bs))
    }
}

pub fn special_random_integer_bool_vec_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, Vec<bool>)> {
    Box::new(IntegerBoolVecPairGenerator1 {
        xs: striped_random_integers(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct IntegerBoolVecPairGenerator2 {
    xs: StripedRandomIntegers<GeometricRandomSigneds<i64>>,
    striped_bit_source: StripedBitSource,
}

impl Iterator for IntegerBoolVecPairGenerator2 {
    type Item = (Integer, Vec<bool>);

    fn next(&mut self) -> Option<(Integer, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = get_striped_bool_vec(
            &mut self.striped_bit_source,
            u64::exact_from(x.to_bits_asc().len()),
        );
        Some((x, bs))
    }
}

pub fn special_random_integer_bool_vec_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Integer, Vec<bool>)> {
    Box::new(IntegerBoolVecPairGenerator2 {
        xs: striped_random_integers(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
    })
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

pub fn special_random_natural_gen_var_1(config: &GenConfig) -> It<Natural> {
    Box::new(striped_random_positive_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

struct SpecialRandomPositiveFloatNaturals<T: PrimitiveFloat> {
    exponents: GeometricRandomSignedRange<i64>,
    mantissas: StripedRandomUnsignedBitChunks<u64>,
    phantom: PhantomData<T>,
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomPositiveFloatNaturals<T> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let exponent = self.exponents.next().unwrap();
        let mut mantissa = self.mantissas.next().unwrap();
        if exponent != 0 {
            mantissa.set_bit(T::MANTISSA_WIDTH);
        } else if mantissa == 0 {
            mantissa = 1;
        }
        Some(Natural::from(mantissa) << exponent)
    }
}

fn special_random_positive_float_naturals<T: PrimitiveFloat>(
    seed: Seed,
    start_exponent: i64,
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> SpecialRandomPositiveFloatNaturals<T> {
    SpecialRandomPositiveFloatNaturals {
        exponents: geometric_random_signed_range(
            seed.fork("exponents"),
            start_exponent,
            i64::power_of_2(T::EXPONENT_WIDTH - 1) - i64::wrapping_from(T::MANTISSA_WIDTH) - 1,
            mean_exponent_numerator,
            mean_exponent_denominator,
        ),
        mantissas: striped_random_unsigned_bit_chunks(
            seed.fork("mantissas"),
            T::MANTISSA_WIDTH + 1,
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        phantom: PhantomData,
    }
}

pub fn special_random_natural_gen_var_2<T: PrimitiveFloat>(config: &GenConfig) -> It<Natural> {
    Box::new(with_special_value(
        EXAMPLE_SEED,
        Natural::ZERO,
        1,
        100,
        &|seed| {
            special_random_positive_float_naturals::<T>(
                seed,
                0,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("mean_stripe_n", 16),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_gen_var_3<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    config: &GenConfig,
) -> It<Natural>
where
    Natural: ExactFrom<T>,
{
    Box::new(
        special_random_positive_float_naturals::<T>(
            EXAMPLE_SEED,
            1,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("mean_stripe_n", 16),
            config.get_or("mean_stripe_d", 1),
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

pub fn special_random_natural_gen_var_4<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Natural>
where
    Natural: From<T>,
{
    Box::new(
        striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(Natural::from),
    )
}

pub fn special_random_natural_gen_var_5<T: PrimitiveSigned>(config: &GenConfig) -> It<Natural>
where
    Natural: ExactFrom<T>,
{
    Box::new(
        striped_random_natural_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(Natural::exact_from),
    )
}

pub fn special_random_natural_gen_var_6(config: &GenConfig) -> It<Natural> {
    Box::new(striped_random_natural_range_to_infinity(
        EXAMPLE_SEED,
        Natural::TWO,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn special_random_natural_gen_var_7<T: PrimitiveFloat>(config: &GenConfig) -> It<Natural>
where
    for<'a> T: ConvertibleFrom<&'a Natural>,
{
    Box::new(
        striped_random_natural_range_to_infinity(
            EXAMPLE_SEED,
            Natural::power_of_2(T::MANTISSA_WIDTH + 1) | Natural::ONE,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", Limb::WIDTH << 1),
            config.get_or("mean_bits_d", 1),
        )
        .filter(|n| !T::convertible_from(n)),
    )
}

pub fn special_random_natural_gen_var_8(config: &GenConfig) -> It<Natural> {
    Box::new(
        striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|n| (n << 1u32) | Natural::ONE),
    )
}

// -- (Natural, bool) --

pub fn special_random_natural_bool_pair_gen(config: &GenConfig) -> It<(Natural, bool)> {
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
        &random_bools,
    ))
}

// -- (Natural, Integer, Natural) --

pub fn special_random_natural_integer_natural_triple_gen(
    config: &GenConfig,
) -> It<(Natural, Integer, Natural)> {
    Box::new(random_triples_xyx(
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
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
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
            striped_random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_pair_gen_var_2(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            striped_random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_pair_gen_var_3(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| (x * &y, y * z)),
    )
}

pub fn special_random_natural_pair_gen_var_4(config: &GenConfig) -> It<(Natural, Natural)> {
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
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_pair_gen_var_5(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs(
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
                striped_random_positive_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(x, y)| (x * &y, y)),
    )
}

pub fn special_random_natural_pair_gen_var_6(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs(
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
                striped_random_positive_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|(x, y)| !x.divisible_by(y)),
    )
}

pub fn special_random_natural_pair_gen_var_7(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_ordered_unique_pairs(striped_random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn special_random_natural_pair_gen_var_8(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs_from_single(striped_random_positive_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn special_random_natural_pair_gen_var_9(config: &GenConfig) -> It<(Natural, Natural)> {
    // TODO
    Box::new(
        random_pairs_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y)| x >= y),
    )
}

pub fn special_random_natural_pair_gen_var_10(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_natural_range_to_infinity(
                seed,
                Natural::power_of_2(Limb::WIDTH),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64 + Limb::WIDTH),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            striped_random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_pair_gen_var_11(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_ordered_unique_pairs(
        striped_random_positive_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
    ))
}

pub fn special_random_natural_pair_gen_var_12(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(a, n)| (a, (n << 1u32) | Natural::ONE)),
    )
}

pub fn special_random_natural_pair_gen_var_13(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs_from_single(
            striped_random_naturals(
                EXAMPLE_SEED,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
            .map(|n| (n << 1u32) | Natural::ONE),
        )
        .filter(|(x, y)| x.coprime_with(y)),
    )
}

pub fn special_random_natural_pair_gen_var_14(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y)| x.coprime_with(y)),
    )
}

pub fn special_random_natural_pair_gen_var_15(config: &GenConfig) -> It<(Natural, Natural)> {
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
            geometric_random_unsigneds::<Limb>(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from)
        },
    ))
}

// -- (Natural, Natural, bool) --

pub fn special_random_natural_natural_bool_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, bool)> {
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
        &|seed| {
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_bools,
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

pub fn special_random_natural_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, m)| (x * &m + &y, y, m)),
    )
}

pub fn special_random_natural_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y, m)| !x.eq_mod(y, m)),
    )
}

pub fn special_random_natural_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| {
            let z = max(&x, &y) + z + Natural::ONE;
            (x, y, z)
        }),
    )
}

pub fn special_random_natural_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(random_triples_xxy(
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
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, mut z)| {
            z += &x;
            z += Natural::ONE;
            (x, y, z)
        }),
    )
}

pub fn special_random_natural_triple_gen_var_6(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(random_triples_from_single(
        striped_random_positive_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
    ))
}

pub fn special_random_natural_triple_gen_var_7(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| (x + &y * &z, y, z)),
    )
}

pub fn special_random_natural_triple_gen_var_8(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(a, b, n)| (a, b, (n << 1u32) | Natural::ONE)),
    )
}

pub fn special_random_natural_triple_gen_var_9(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(a, m, n)| (a, (m << 1u32) | Natural::ONE, (n << 1u32) | Natural::ONE)),
    )
}

// -- (Natural, Natural, Natural, Natural) --

pub fn special_random_natural_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        random_quadruples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter_map(|(x, y, z, w)| {
            let ranking = [(&x, 0), (&y, 1), (&z, 2), (&w, 3)];
            let (hi, next_hi) = get_two_highest(&ranking);
            if hi.0 == next_hi.0 {
                None
            } else {
                Some(match hi.1 {
                    0 => (y, z, w, x),
                    1 => (x, z, w, y),
                    2 => (x, y, w, z),
                    _ => (x, y, z, w),
                })
            }
        }),
    )
}

pub fn special_random_natural_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        random_quadruples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z, mut w)| {
            w += max!(&x, &y);
            w += Natural::ONE;
            (x, y, z, w)
        }),
    )
}

pub fn special_random_natural_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        random_quadruples_from_single(striped_random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z, mut w)| {
            w += &x;
            w += Natural::ONE;
            (x, y, z, w)
        }),
    )
}

// -- (Natural, Natural, Natural, PrimitiveUnsigned) --

pub fn special_random_natural_natural_natural_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, T)> {
    Box::new(random_quadruples_xxxy(
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

pub fn special_random_natural_natural_natural_unsigned_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        random_quadruples_xxxy(
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
        )
        .map(|(x, y, z, mut m)| {
            m += max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits()
            );
            (x, y, z, m)
        }),
    )
}

pub fn special_random_natural_natural_natural_unsigned_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        random_quadruples_xxxy(
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
        )
        .map(|(x, y, z, mut m)| {
            m += max(x.significant_bits(), y.significant_bits());
            (x, y, z, m)
        }),
    )
}

pub fn special_random_natural_natural_natural_unsigned_quadruple_gen_var_4(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        random_quadruples_xxxy(
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
        )
        .map(|(x, y, z, mut m)| {
            m += x.significant_bits();
            (x, y, z, m)
        }),
    )
}

// -- (Natural, Natural, PrimitiveFloat) --

pub fn special_random_natural_natural_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(random_triples_xxy(
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
        &random_primitive_floats,
    ))
}

// -- (Natural, Natural, PrimitiveUnsigned) --

pub fn special_random_natural_natural_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(random_triples_xxy(
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
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Natural, Natural, PrimitiveSigned) --

pub fn special_random_natural_natural_signed_triple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(random_triples_xxy(
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
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_natural_signed_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_ordered_unique_pairs(striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            ))
        },
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    )))
}

// -- (Natural, Natural, PrimitiveUnsigned) --

pub fn special_random_natural_natural_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(random_triples_xxy(
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

pub fn special_random_natural_natural_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        random_triples_xxy(
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
        )
        .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

pub fn special_random_natural_natural_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(
        random_triples_xxy(
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

pub fn special_random_natural_natural_unsigned_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Natural, Natural, u64)> {
    Box::new(
        random_triples_xxy(
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
        )
        .map(|(x, y, mut m)| {
            m += max(x.significant_bits(), y.significant_bits());
            (x, y, m)
        }),
    )
}

pub fn special_random_natural_natural_unsigned_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Natural, Natural, u64)> {
    Box::new(
        random_triples_xxy(
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
        )
        .map(|(x, y, mut m)| {
            m += x.significant_bits();
            (x, y, m)
        }),
    )
}

pub fn special_random_natural_natural_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_ordered_unique_pairs(striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            ))
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    )))
}

// -- (Natural, Natural, RoundingMode) --

pub fn special_random_natural_natural_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        random_triples(
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
                striped_random_positive_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(x, y, rm)| {
            if rm == Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub fn special_random_natural_natural_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        random_triples(
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
                striped_random_positive_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_natural_filter_map(x, y, rm)),
    )
}

// -- (Natural, PrimitiveFloat) --

pub fn special_random_natural_primitive_float_pair_gen<T: PrimitiveFloat>(
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
        &random_primitive_floats,
    ))
}

// -- (Natural, PrimitiveFloat, PrimitiveFloat) --

pub fn special_random_natural_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
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
        &random_primitive_floats,
    ))
}

// -- (Natural, PrimitiveSigned) --

pub fn special_random_natural_signed_pair_gen<T: PrimitiveSigned>(
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
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_signed_pair_gen_var_1<T: PrimitiveSigned>(
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
            striped_random_natural_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_signed_pair_gen_var_2<T: PrimitiveSigned>(
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
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_signed_pair_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

struct NaturalBitsMultipleOfLimbBitsGenerator {
    bit_source: StripedBitSource,
    limb_counts: GeometricRandomNaturalValues<u64>,
}

impl Iterator for NaturalBitsMultipleOfLimbBitsGenerator {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(get_striped_random_natural_with_bits(
            &mut self.bit_source,
            self.limb_counts.next().unwrap() << Limb::LOG_WIDTH,
        ))
    }
}

pub fn special_random_natural_signed_pair_gen_var_4<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| NaturalBitsMultipleOfLimbBitsGenerator {
            bit_source: StripedBitSource::new(
                seed.fork("bit_source"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            ),
            limb_counts: geometric_random_positive_unsigneds(
                seed.fork("limb_counts"),
                config.get_or("mean_small_n", 2),
                config.get_or("mean_small_d", 1),
            ),
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

// -- (Natural, PrimitiveSigned, PrimitiveSigned) ---

pub fn special_random_natural_signed_signed_triple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
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
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Natural, PrimitiveSigned, PrimitiveUnsigned) --

pub fn special_random_natural_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, u64)> {
    Box::new(
        random_triples(
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
                geometric_random_signeds(
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
        )
        .map(|(x, y, mut m)| {
            m += x.significant_bits();
            (x, y, m)
        }),
    )
}

pub fn special_random_natural_signed_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
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
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
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

// -- (Natural, PrimitiveSigned, RoundingMode) --

pub fn special_random_natural_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shr<T, Output = Natural>,
{
    Box::new(
        random_triples(
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
                if i < T::ZERO && rm == Exact {
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

pub fn special_random_natural_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        random_triples(
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
                if i > T::ZERO && rm == Exact {
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

pub fn special_random_natural_unsigned_pair_gen<T: PrimitiveUnsigned>(
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
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::saturating_from(U::MAX),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::MAX,
                config.get_or("mean_stripe_n", T::WIDTH),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
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
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::from(36u8),
                config.get_or("mean_stripe_n", T::WIDTH),
                config.get_or("mean_stripe_d", 1),
            )
        },
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

pub fn special_random_natural_unsigned_pair_gen_var_5<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, u64)> {
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
            striped_random_unsigned_inclusive_range(
                seed,
                1,
                T::WIDTH,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>(
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
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_natural_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed_2| {
                striped_random_naturals(
                    seed_2,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), Down);
            (x, y)
        }),
    )
}

pub fn special_random_natural_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(
        random_pairs(
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

pub fn special_random_natural_unsigned_pair_gen_var_10(config: &GenConfig) -> It<(Natural, u64)> {
    Box::new(
        random_pairs(
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
        )
        .map(|(x, mut m)| {
            m += x.significant_bits();
            (x, m)
        }),
    )
}

pub fn special_random_natural_unsigned_pair_gen_var_11<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_natural_range_to_infinity(
                seed,
                Natural::TWO,
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

pub fn special_random_natural_unsigned_pair_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_naturals(
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

pub fn special_random_natural_unsigned_pair_gen_var_14(config: &GenConfig) -> It<(Natural, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_naturals(
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
        )
        .map(|(x, mut m)| {
            m += x.significant_bits();
            (x, m)
        }),
    )
}

// -- (Natural, PrimitiveUnsigned, bool) --

pub fn special_random_natural_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, bool)> {
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

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) ---

pub fn special_random_natural_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
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
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::from(36u8),
                config.get_or("mean_stripe_n", T::WIDTH),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Natural, u64, T)> {
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
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
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(
        random_triples_xyy(
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
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, y, z)| if y <= z { (xs, y, z) } else { (xs, z, y) }),
    )
}

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_unsigned_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, u64)> {
    Box::new(
        random_triples(
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
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(x, y, mut m)| {
            m += x.significant_bits();
            (x, y, m)
        }),
    )
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

pub fn special_random_natural_unsigned_unsigned_natural_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Natural, T, T, Natural)> {
    Box::new(
        random_quadruples_xyyx::<_, _, T, _>(
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
        )
        .filter_map(|(x, y, z, w)| match y.cmp(&z) {
            Less => Some((x, y, z, w)),
            Greater => Some((x, z, y, w)),
            Equal => None,
        }),
    )
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_natural_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        random_triples(
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
                    config.get_or("mean_excess_len_n", 4),
                    config.get_or("mean_excess_len_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .map(|(n, u, rm)| {
            if rm == Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

// var 2 is in malachite-float

// --(Natural, PrimitiveUnsigned, Vec<bool>) --

struct NaturalUnsignedBoolVecTripleGenerator {
    xs: StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    log_bases: It<u64>,
    striped_bit_source: StripedBitSource,
}

impl Iterator for NaturalUnsignedBoolVecTripleGenerator {
    type Item = (Natural, u64, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, u64, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let log_base = self.log_bases.next().unwrap();
        let bs = get_striped_bool_vec(
            &mut self.striped_bit_source,
            x.significant_bits().div_round(log_base, Ceiling).0,
        );
        Some((x, log_base, bs))
    }
}

pub fn special_random_natural_unsigned_bool_vec_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, u64, Vec<bool>)> {
    Box::new(NaturalUnsignedBoolVecTripleGenerator {
        xs: striped_random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        log_bases: Box::new(geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("log_bases"),
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        )),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_natural_unsigned_bool_vec_triple_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, u64, Vec<bool>)> {
    Box::new(NaturalUnsignedBoolVecTripleGenerator {
        xs: striped_random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        log_bases: Box::new(striped_random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("log_bases"),
            1,
            T::WIDTH,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        )),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- (Natural, RoundingMode) --

pub fn special_random_natural_rounding_mode_pair_gen(
    config: &GenConfig,
) -> It<(Natural, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_natural_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(Natural, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_naturals(
                    seed.fork("xs"),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, rm)| rm != Exact || T::convertible_from(n)),
    )
}

pub fn special_random_natural_rounding_mode_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_naturals(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// -- (Natural, ToSciOptions) --

pub fn special_random_natural_to_sci_options_pair_gen(
    config: &GenConfig,
) -> It<(Natural, ToSciOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("small_mean_n", 4),
                config.get_or("small_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_natural_to_sci_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, ToSciOptions)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_naturals(
                    seed.fork("xs"),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                random_to_sci_options(
                    seed,
                    config.get_or("small_mean_n", 4),
                    config.get_or("small_mean_d", 1),
                )
            },
        )
        .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (Natural, Vec<bool>) --

struct NaturalBoolVecPairGenerator1 {
    xs: StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    striped_bit_source: StripedBitSource,
}

impl Iterator for NaturalBoolVecPairGenerator1 {
    type Item = (Natural, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = get_striped_bool_vec(&mut self.striped_bit_source, x.limb_count());
        Some((x, bs))
    }
}

pub fn special_random_natural_bool_vec_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Vec<bool>)> {
    Box::new(NaturalBoolVecPairGenerator1 {
        xs: striped_random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct NaturalBoolVecPairGenerator2 {
    xs: StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    striped_bit_source: StripedBitSource,
}

impl Iterator for NaturalBoolVecPairGenerator2 {
    type Item = (Natural, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = get_striped_bool_vec(&mut self.striped_bit_source, x.significant_bits());
        Some((x, bs))
    }
}

pub fn special_random_natural_bool_vec_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Vec<bool>)> {
    Box::new(NaturalBoolVecPairGenerator2 {
        xs: striped_random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- (PrimitiveUnsigned * 6) --

// var 1 is in malachite-base.

pub fn special_random_unsigned_sextuple_gen_var_2(
    config: &GenConfig,
) -> It<(Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_pairs_from_single(striped_random_unsigneds(
                            seed_2,
                            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        ))
                    },
                    &|seed_2| {
                        random_pairs(
                            seed_2,
                            &|seed_3| {
                                striped_random_unsigned_inclusive_range(
                                    seed_3,
                                    Limb::power_of_2(Limb::WIDTH - 1),
                                    Limb::MAX,
                                    config.get_or("mean_stripe_n", 32),
                                    config.get_or("mean_stripe_d", 1),
                                )
                            },
                            &|seed_3| {
                                striped_random_unsigneds(
                                    seed_3,
                                    config.get_or("mean_unsigned_stripe_n", Limb::WIDTH >> 1),
                                    config.get_or("mean_unsigned_stripe_d", 1),
                                )
                            },
                        )
                    },
                )
                .filter(|&((n_2, n_1), (d_1, d_0))| n_2 < d_1 || n_2 == d_1 && n_1 < d_0)
            },
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|(((n_2, n_1), (d_1, d_0)), n_0)| {
            (
                n_2,
                n_1,
                n_0,
                d_1,
                d_0,
                limbs_two_limb_inverse_helper(d_1, d_0),
            )
        }),
    )
}

// -- (String, String, String) --

pub fn special_random_string_triple_gen_var_1(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        striped_random_naturals(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|x| {
            (
                serde_json::to_string(&BigUint::from(&x)).unwrap(),
                serde_json::to_string(&rug::Integer::from(&x)).unwrap(),
                serde_json::to_string(&x).unwrap(),
            )
        }),
    )
}

pub fn special_random_string_triple_gen_var_2(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        striped_random_integers(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|x| {
            (
                serde_json::to_string(&BigInt::from(&x)).unwrap(),
                serde_json::to_string(&rug::Integer::from(&x)).unwrap(),
                serde_json::to_string(&x).unwrap(),
            )
        }),
    )
}

// -- Vec<Integer> --

pub fn special_random_integer_vec_gen(config: &GenConfig) -> It<Vec<Integer>> {
    Box::new(random_vecs(
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
        config.get_or("mean_len_n", 4),
        config.get_or("mean_len_d", 1),
    ))
}

// -- Vec<Natural> --

pub fn special_random_natural_vec_gen(config: &GenConfig) -> It<Vec<Natural>> {
    Box::new(random_vecs(
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
        config.get_or("mean_len_n", 4),
        config.get_or("mean_len_d", 1),
    ))
}

// -- (Vec<Natural>, Integer) --

pub fn special_random_natural_vec_integer_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Natural>, Integer)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    striped_random_positive_naturals(
                        seed_2,
                        config.get_or("mean_stripe_n", 32),
                        config.get_or("mean_stripe_d", 1),
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_len_n", 4),
                config.get_or("mean_len_d", 1),
            )
        },
        &|seed| {
            striped_random_integers(
                seed.fork("xs"),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Vec<Natural>, Natural) --

pub fn special_random_natural_vec_natural_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    striped_random_naturals(
                        seed_2,
                        config.get_or("mean_stripe_n", 32),
                        config.get_or("mean_stripe_d", 1),
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_digit_count_n", 4),
                config.get_or("mean_digit_count_d", 1),
            )
        },
        &|seed| {
            striped_random_natural_range_to_infinity(
                seed,
                Natural::power_of_2(Limb::WIDTH),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                Limb::WIDTH + 4,
                1,
            )
        },
    ))
}

pub fn special_random_natural_vec_natural_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    striped_random_naturals(
                        seed_2,
                        config.get_or("mean_stripe_n", 32),
                        config.get_or("mean_stripe_d", 1),
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_digit_count_n", 4),
                config.get_or("mean_digit_count_d", 1),
            )
        },
        &|seed| {
            striped_random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

struct LargeDigitsRandomGenerator {
    bases: StripedRandomNaturalRangeToInfinity,
    digit_counts: GeometricRandomNaturalValues<usize>,
    bit_source: StripedBitSource,
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
                let x = get_striped_random_natural_with_up_to_bits(&mut self.bit_source, bits);
                if x < base {
                    digits.push(x);
                    break;
                }
            }
        }
        Some((digits, base))
    }
}

pub fn special_random_natural_vec_natural_pair_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(LargeDigitsRandomGenerator {
        bases: striped_random_natural_range_to_infinity(
            EXAMPLE_SEED.fork("bases"),
            Natural::power_of_2(Limb::WIDTH),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            Limb::WIDTH + 4,
            1,
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn striped_random_natural_vec_natural_pair_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Natural>, Natural)> {
    Box::new(LargeDigitsRandomGenerator {
        bases: striped_random_natural_range_to_infinity(
            EXAMPLE_SEED.fork("bases"),
            Natural::TWO,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- (Vec<Natural>, PrimitiveUnsigned) --

struct PowerOfTwoDigitsGenerator {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    bit_source: StripedBitSource,
}

impl Iterator for PowerOfTwoDigitsGenerator {
    type Item = (Vec<Natural>, u64);

    fn next(&mut self) -> Option<(Vec<Natural>, u64)> {
        let log_base = self.log_bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let mut digits = Vec::with_capacity(digit_count);
        for _ in 0..digit_count {
            digits.push(get_striped_random_natural_with_up_to_bits(
                &mut self.bit_source,
                log_base,
            ));
        }
        Some((digits, log_base))
    }
}

pub fn special_random_natural_vec_unsigned_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Natural>, u64)> {
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
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_natural_vec_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<Natural>, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    striped_random_naturals(
                        seed_2,
                        config.get_or("mean_stripe_n", 32),
                        config.get_or("mean_stripe_d", 1),
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

// -- Vec<PrimitiveUnsigned> --

// vars 1 through 4 are in malachite-base.

pub fn special_random_unsigned_vec_gen_var_5(config: &GenConfig) -> It<Vec<Limb>> {
    Box::new(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", Limb::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .map(|mut xs| {
            limbs_vec_mul_limb_in_place(&mut xs, 3);
            xs
        }),
    )
}

// var 6 is in malachite-base.

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
                2,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 6),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::saturating_from(U::MAX),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// vars 5 through 17 are in malachite-base.

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_18(
    config: &GenConfig,
) -> It<(Vec<Limb>, u64)> {
    Box::new(
        special_random_unsigned_vec_unsigned_pair_gen_var_17(config).filter(|(xs, index)| {
            let mut mut_xs = xs.clone();
            limbs_vec_clear_bit_neg(&mut mut_xs, *index);
            mut_xs.len() == xs.len()
        }),
    )
}

// vars 19 through 25 are in malachite-base.

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_26(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|(mut xs, y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y);
            (xs, y)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_27(
    config: &GenConfig,
) -> It<(Vec<Limb>, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length::<Limb>(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_unsigned_n", 4),
                    config.get_or("mean_small_unsigned_d", 1),
                )
            },
        )
        .filter_map(|(mut xs, mut pow)| {
            let xs_last = xs.last_mut().unwrap();
            *xs_last = xs_last.checked_add(1)?;
            pow += limbs_significant_bits_helper(&xs);
            Some((xs, pow))
        }),
    )
}

// var 28 is in malachite-base.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 5 are in malachite-base

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_6(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(|(m, x, y)| {
            !Integer::from(Natural::from(*x)).eq_mod(-Natural::from(*y), Natural::from_limbs_asc(m))
        }),
    )
}

// vars 7 through 8 are in malachite-base.

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_9(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| random_values_from_vec(seed, factors_of_limb_max()),
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// var 10 is in malachite-base.

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_11(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(map_helper_3),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_12(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(filter_helper_6),
    )
}

// var 13 is in malachite-base.

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_14<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length::<Limb>(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            (xs, y, pow)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_15<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length::<Limb>(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            (xs, y, pow)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_16<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length::<Limb>(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            if pow == 0 {
                pow = 1;
            }
            (xs, y, pow)
        }),
    )
}

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

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )))
        .filter_map(filter_map_helper_1),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )))
        .filter(filter_helper_1),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )))
        .map(map_helper_1),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )))
        .filter(filter_helper_4),
    )
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
            0,
            u64::exact_from(GET_STR_PRECOMPUTE_THRESHOLD),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
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
        bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("bit_source"),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
        ),
        phantom: PhantomData,
    })
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 10 are in malachite-base.

pub fn special_random_unsigned_vec_pair_gen_var_11(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecPairLenGenerator2 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(xs, ys)| gcd_input_filter(xs, ys)),
    )
}

// vars 12 through 13 are in malachite-base.

pub fn special_random_unsigned_vec_pair_gen_var_14(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter_map(|(out, mut xs)| {
            limbs_vec_mul_limb_in_place(&mut xs, 3);
            if out.len() >= xs.len() {
                Some((out, xs))
            } else {
                None
            }
        }),
    )
}

pub fn special_random_unsigned_vec_pair_gen_var_15(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs_from_single(striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            2,
            config.get_or("mean_stripe_n", Limb::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(ns, mut ds)| {
            let d_last = ds.last_mut().unwrap();
            if *d_last == 0 {
                *d_last = 1;
            }
            let mut new_ns = limbs_mul(&ns, &ds);
            if *new_ns.last().unwrap() == 0 {
                new_ns.pop();
            }
            (new_ns, ds)
        }),
    )
}

// vars 16 through 17 are in malachite-base.

pub fn special_random_unsigned_vec_pair_gen_var_18(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs_from_single(
            striped_random_unsigned_vecs_min_length(
                EXAMPLE_SEED,
                1,
                config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter_map(|mut xs| {
                let x_last = xs.last_mut().unwrap();
                if *x_last == Limb::MAX {
                    None
                } else {
                    *x_last += 1;
                    Some(xs)
                }
            }),
        )
        .filter_map(|(ns, ds)| {
            let mut ns = limbs_mul(&ns, &ds);
            if *ns.last().unwrap() == 0 {
                ns.pop();
            }
            if *ns.last().unwrap() == 0 {
                None
            } else {
                Some((ns, ds))
            }
        }),
    )
}

// vars 19 through 21 are in malachite-base.

pub fn special_random_unsigned_vec_pair_gen_var_22<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs(
            EXAMPLE_SEED.fork("lengths"),
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    1,
                    u64::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    1,
                    u64::exact_from(SQRLO_DC_THRESHOLD_LIMIT),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
        )
        .filter(|(x, y)| x >= y),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

fn special_random_square_helper<T: PrimitiveUnsigned, F: Fn(usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: u64,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(move |(o, x)| {
            let x = x.checked_add(min_x)?;
            let ux = usize::exact_from(x);
            if valid(ux) {
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

pub fn special_random_unsigned_vec_pair_gen_var_23<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &|x| x <= SQR_TOOM2_THRESHOLD, 1)
}

pub fn special_random_unsigned_vec_pair_gen_var_24<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &|_| true, 2)
}

pub fn special_random_unsigned_vec_pair_gen_var_25<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &limbs_square_to_out_toom_3_input_size_valid, 3)
}

pub fn special_random_unsigned_vec_pair_gen_var_26<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &limbs_square_to_out_toom_4_input_size_valid, 4)
}

pub fn special_random_unsigned_vec_pair_gen_var_27<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &|x| x == 7 || x == 8 || x > 9, 7)
}

pub fn special_random_unsigned_vec_pair_gen_var_28<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &limbs_square_to_out_toom_6_input_size_valid, 18)
}

pub fn special_random_unsigned_vec_pair_gen_var_29<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_square_helper(config, &limbs_square_to_out_toom_8_input_size_valid, 40)
}

// vars 32 to 33 are in malachite-base.

pub fn special_random_unsigned_vec_pair_gen_var_34<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    #[cfg(feature = "32_bit_limbs")]
    let limit = 56;
    #[cfg(not(feature = "32_bit_limbs"))]
    let limit = 28;
    special_random_square_helper(config, &limbs_square_to_out_fft_is_valid, limit)
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// var 1 is in malachite-base

struct BasecaseDigitsSpecialRandomGenerator1<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    bases: RandomValuesFromVec<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    base_to_digits: HashMap<u64, RandomUnsignedsLessThan<T>>,
    excess_limb_counts: GeometricRandomNaturalValues<u64>,
    bit_source: StripedBitSource,
    phantom: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for BasecaseDigitsSpecialRandomGenerator1<T, U>
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

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(BasecaseDigitsSpecialRandomGenerator1 {
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

struct BasecaseDigitsSpecialRandomGenerator2<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    bases: RandomValuesFromVec<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    base_to_digits: HashMap<u64, StripedRandomUnsignedBitChunks<T>>,
    excess_limb_counts: GeometricRandomNaturalValues<u64>,
    bit_source: StripedBitSource,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    phantom: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for BasecaseDigitsSpecialRandomGenerator2<T, U>
{
    type Item = (Vec<U>, Vec<T>, u64);

    fn next(&mut self) -> Option<(Vec<U>, Vec<T>, u64)> {
        let base = self.bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let mean_stripe_n = self.mean_stripe_n;
        let mean_stripe_d = self.mean_stripe_d;
        let ds = self.base_to_digits.entry(base).or_insert_with(move || {
            striped_random_unsigneds(
                EXAMPLE_SEED.fork(&base.to_string()),
                mean_stripe_n,
                mean_stripe_d,
            )
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

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3<
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
        mean_stripe_n: config.get_or("mean_stripe_n", 32),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        phantom: PhantomData,
    })
}

// vars 4 through 6 are in malachite-base.

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(filter_map_helper_2),
    )
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(filter_helper_2),
    )
}

// vars 9 through 13 are in malachite-base.

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
                    seed.fork("lengths"),
                    2,
                    u64::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(|((out, mut xs), y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y);
            if out.len() >= xs.len() {
                Some((out, xs, y))
            } else {
                None
            }
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_15(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(map_helper_2),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_16(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| *xs.last().unwrap() != 0)
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(filter_helper_5),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_17(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(mut n_len, mut d_init_len)| {
                    n_len = n_len.checked_add(3)?;
                    d_init_len = d_init_len.checked_add(2)?;
                    if n_len > d_init_len {
                        Some((n_len, d_init_len))
                    } else {
                        None
                    }
                }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    Limb::power_of_2(Limb::WIDTH - 1),
                    Limb::MAX,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|((n, mut d_init), d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (n, d_init, inverse)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_18(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs::<Limb>(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, ys, mut pow)| {
            pow += max(
                limbs_significant_bits_helper(&xs),
                limbs_significant_bits_helper(&ys),
            );
            (xs, ys, pow)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_19(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigneds(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|((xs, ys), mut pow)| {
            pow += max(
                limbs_significant_bits_helper(&xs),
                limbs_significant_bits_helper(&ys),
            );
            (xs, ys, pow)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_20(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length::<Limb>(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter_map(|mut xs| {
                    let last_x = xs.last_mut().unwrap();
                    *last_x = last_x.checked_add(1)?;
                    Some(xs)
                })
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(xs, ys, mut pow)| {
            pow += max(
                limbs_significant_bits_helper(&xs),
                limbs_significant_bits_helper(&ys),
            );
            (xs, ys, pow)
        }),
    )
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_21(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length::<Limb>(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .filter_map(|(mut xs, mut es, pow)| {
            let last_e = es.last_mut().unwrap();
            *last_e = last_e.checked_add(1)?;
            if es == [1] {
                return None;
            }
            limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
            if *xs.last().unwrap() == 0 {
                None
            } else {
                Some((xs, es, pow))
            }
        }),
    )
}

// var 22 is in malachite-base.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 3 are in malachite-base

fn special_random_mul_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: u64,
    min_y: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(move |(o, x, y)| {
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
        &limbs_mul_greater_to_out_toom_22_input_sizes_valid,
        2,
        2,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_32_input_sizes_valid,
        6,
        4,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        3,
        3,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_42_input_sizes_valid,
        4,
        2,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_43_input_sizes_valid,
        11,
        8,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_44_input_sizes_valid,
        4,
        4,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_52_input_sizes_valid,
        14,
        5,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_53_input_sizes_valid,
        5,
        3,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_54_input_sizes_valid,
        14,
        11,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_62_input_sizes_valid,
        6,
        2,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_63_input_sizes_valid,
        17,
        9,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
        42,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
        86,
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
        .filter_map(move |(o, x)| {
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
        &limbs_mul_greater_to_out_toom_33_input_sizes_valid,
        5,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_19<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_same_length_helper(
        config,
        &limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
        42,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_20<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_same_length_helper(
        config,
        &limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
        86,
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_22<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &|xs_len, ys_len| {
            limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len, ys_len)
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
            limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len, ys_len)
        },
        5,
        3,
    )
}

// vars 24 through 36 are in malachite-base

pub fn special_random_unsigned_vec_triple_gen_var_37(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_triples_from_single(
            striped_random_unsigned_vecs_min_length(
                EXAMPLE_SEED,
                2,
                config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter_map(filter_map_helper_3),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_38(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_triples_from_single(
            striped_random_unsigned_vecs_min_length(
                EXAMPLE_SEED,
                2,
                config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter(filter_helper_3),
    )
}

// vars 39 through 41 is in malachite-base.

pub fn special_random_unsigned_vec_triple_gen_var_42(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(q_len, mut n_len, mut d_init_len)| {
                    n_len = n_len.checked_add(2)?;
                    d_init_len = d_init_len.checked_add(1)?;
                    let d_len = d_init_len + 1;
                    if n_len >= d_len && q_len >= n_len - d_len {
                        Some((q_len, n_len, d_init_len))
                    } else {
                        None
                    }
                }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    Limb::power_of_2(Limb::WIDTH - 1),
                    Limb::MAX,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_43(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(q_len, mut n_len, mut d_init_len)| {
                    n_len = n_len.checked_add(3)?;
                    d_init_len = d_init_len.checked_add(1)?;
                    let d_len = d_init_len + 1;
                    if n_len > d_len && q_len >= n_len - d_len {
                        Some((q_len, n_len, d_init_len))
                    } else {
                        None
                    }
                }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    Limb::power_of_2(Limb::WIDTH - 1),
                    Limb::MAX,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_44(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(1)?;
                n_len = n_len.checked_add(2)?;
                d_len = d_len.checked_add(2)?;
                if n_len >= d_len && q_len > n_len - d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(_, _, d)| *d.last().unwrap() != 0),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_45(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(1)?;
                n_len = n_len.checked_add(2)?;
                d_len = d_len.checked_add(2)?;
                if n_len >= d_len && q_len > n_len - d_len && n_len < (d_len - 1) << 1 {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(_, _, d)| *d.last().unwrap() != 0),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_46(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                2,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(q_len, n_len, d_len)| {
                if q_len >= n_len && n_len >= d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            (q, n, d)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_47(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                1,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(q_len, n_len, d_len)| {
                if q_len >= n_len && n_len >= d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            (q, n, d)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_48(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                1,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(q_len, n_len, d_len)| {
                if q_len + 1 >= n_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter_map(|(q, n, mut d)| {
            let d_last = d.last_mut().unwrap();
            if *d_last == 0 {
                *d_last = 1;
            }
            let mut new_n = limbs_mul(&n, &d);
            if *new_n.last().unwrap() == 0 {
                new_n.pop();
            }
            if q.len() + d.len() >= new_n.len() + 1 {
                Some((q, new_n, d))
            } else {
                None
            }
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_49(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                1,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(q_len, n_len, mut d_len)| {
                d_len = d_len.checked_add(1)?;
                if q_len + 1 >= n_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter_map(|(q, n, mut d)| {
            let d_last = d.last_mut().unwrap();
            if *d_last == 0 {
                *d_last = 1;
            }
            let mut new_n = limbs_mul(&n, &d);
            if *new_n.last().unwrap() == 0 {
                new_n.pop();
            }
            if q.len() > new_n.len() - d.len() {
                Some((q, n, d))
            } else {
                None
            }
        }),
    )
}

// vars 50 through 53 are in malachite-base.

pub fn special_random_unsigned_vec_triple_gen_var_54(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_triples_from_single(
            striped_random_unsigned_vecs_min_length(
                EXAMPLE_SEED,
                2,
                config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != 0),
        )
        .map(|(xs, ys, m)| limbs_eq_mod_map(&xs, ys, m)),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_55(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_triples_from_single(
            striped_random_unsigned_vecs_min_length(
                EXAMPLE_SEED,
                2,
                config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter(|(xs, ys, m)| !limbs_eq_mod_ref_ref_ref(xs, ys, m)),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_56(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(q_len, n_len): (u64, u64)| {
                    Some((q_len.checked_add(n_len)?, n_len.checked_add(2)?))
                }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_unsigned_inclusive_range(
                            seed_2,
                            Limb::power_of_2(Limb::WIDTH - 1),
                            Limb::MAX,
                            config.get_or("mean_stripe_n", 32),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_positive_unsigneds(
                            seed_2,
                            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
        )
        .map(|((q, n), (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// var 57 is in malachite-base.

pub fn special_random_unsigned_vec_triple_gen_var_58<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    special_random_mul_helper(
        config,
        &|x, y| {
            limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
                && limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
        },
        7,
        7,
    )
}

// var 59 is in malachite-base.

pub fn special_random_unsigned_vec_triple_gen_var_60<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    #[cfg(feature = "32_bit_limbs")]
    let limit = 112;
    #[cfg(not(feature = "32_bit_limbs"))]
    let limit = 56;
    Box::new(UnsignedVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_positive_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(move |(o, mut x, mut y)| {
            let sum = x + y;
            if sum <= limit {
                if o.odd() {
                    x += limit - sum + 1;
                } else {
                    y += limit - sum + 1;
                }
            }
            if limbs_mul_greater_to_out_fft_is_valid(usize::exact_from(x), usize::exact_from(y)) {
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

// -- (Vec<PrimitiveUnsigned> * 4) --

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut r_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(1)?;
                r_len = r_len.checked_add(2)?;
                n_len = n_len.checked_add(2)?;
                d_len = d_len.checked_add(2)?;
                if r_len >= d_len && n_len >= d_len && q_len > n_len - d_len {
                    Some((q_len, r_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, r, mut d)| {
            let last_d = d.last_mut().unwrap();
            if *last_d == 0 {
                *last_d = 1;
            }
            (q, n, r, d)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut r_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(2)?;
                r_len = r_len.checked_add(2)?;
                n_len = n_len.checked_add(4)?;
                d_len = d_len.checked_add(2)?;
                if n_len >= d_len + 2 && q_len >= n_len - d_len && r_len >= d_len {
                    Some((q_len, r_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, r, mut d)| {
            d[0] |= 1;
            (q, n, r, d)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut r_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(4)?;
                r_len = r_len.checked_add(2)?;
                n_len = n_len.checked_add(4)?;
                d_len = d_len.checked_add(2)?;
                if n_len >= d_len + 2 && q_len >= n_len - d_len && r_len >= d_len {
                    Some((q_len, r_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, r, mut d)| {
            d[0] |= 1;
            (q, n, r, d)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut r_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(1)?;
                r_len = r_len.checked_add(2)?;
                n_len = n_len.checked_add(2)?;
                d_len = d_len.checked_add(2)?;
                if r_len >= d_len
                    && n_len >= d_len
                    && q_len > n_len - d_len
                    && (d_len << 1) > n_len + 1
                {
                    Some((q_len, r_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, r, mut d)| {
            let last_d = d.last_mut().unwrap();
            if *last_d == 0 {
                *last_d = 1;
            }
            (q, n, r, d)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_5(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut r_len, mut n_len, mut d_len)| {
                q_len = q_len.checked_add(1)?;
                r_len = r_len.checked_add(2)?;
                n_len = n_len.checked_add(3)?;
                d_len = d_len.checked_add(2)?;
                if r_len >= d_len && n_len > d_len && q_len + d_len >= n_len {
                    Some((q_len, r_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(
            |(q, n, r, mut d): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
                d.last_mut().unwrap().set_bit(Limb::WIDTH - 1);
                (q, n, r, d)
            },
        ),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_6(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        UnsignedVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(out_len, mut b_len, e_len, mut m_len)| {
                b_len = b_len.checked_add(1)?;
                m_len = m_len.checked_add(1)?;
                if out_len >= m_len {
                    Some((out_len, b_len, e_len, m_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(_, bs, es, ms)| {
            (es.len() > 1 || es.len() == 1 && es[0] > 1)
                && *bs.last().unwrap() != 0
                && *es.last().unwrap() != 0
                && *ms.last().unwrap() != 0
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_vec_quadruple_gen_var_7(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(special_random_unsigned_vec_quadruple_gen_var_6(config).map(
        |(out, bs, es, mut ms)| {
            ms[0] |= 1;
            (out, bs, es, ms)
        },
    ))
}

// -- large types --

// vars 1 through 4 are in malachite-base

fn special_random_half_gcd_matrix(
    s: usize,
    n: usize,
    bit_source: &mut StripedBitSource,
) -> OwnedHalfGcdMatrix {
    assert!(n <= s);
    let bits = u64::exact_from(n) << Limb::LOG_WIDTH;
    let mut m00 = get_striped_unsigned_vec(bit_source, bits);
    let m01 = get_striped_unsigned_vec(bit_source, bits);
    let m10 = get_striped_unsigned_vec(bit_source, bits);
    let m11 = get_striped_unsigned_vec(bit_source, bits);
    m00.resize(s << 2, 0);
    m00[s..s + n].copy_from_slice(&m01);
    m00[s << 1..(s << 1) + n].copy_from_slice(&m10);
    m00[s * 3..s * 3 + n].copy_from_slice(&m11);
    half_gcd_matrix_create(s, n, m00)
}

struct HalfGcdMatrixAndVecGenerator {
    sizes: GeometricRandomNaturalValues<usize>,
    striped_bit_source: StripedBitSource,
    bs: RandomBools,
}

impl Iterator for HalfGcdMatrixAndVecGenerator {
    type Item = (OwnedHalfGcdMatrix, Vec<Limb>, u8);

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
            let m = special_random_half_gcd_matrix(m_s, m_n, &mut self.striped_bit_source);
            let qs = get_striped_unsigned_vec(
                &mut self.striped_bit_source,
                u64::exact_from(qs_len) << Limb::LOG_WIDTH,
            );
            let column = u8::from(self.bs.next().unwrap());
            return Some((m, qs, column));
        }
    }
}

pub fn special_random_large_type_gen_var_5(
    config: &GenConfig,
) -> It<(OwnedHalfGcdMatrix, Vec<Limb>, u8)> {
    Box::new(HalfGcdMatrixAndVecGenerator {
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", 4),
            config.get_or("mean_stripe_d", 1),
        ),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_6(
    config: &GenConfig,
) -> It<(HalfGcdMatrix1, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    reshape_1_3_to_4(Box::new(
        random_quadruples_from_single(striped_random_unsigneds(
            EXAMPLE_SEED.fork("m"),
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(m00, m01, m10, m11)| HalfGcdMatrix1 {
            data: [[m00, m01], [m10, m11]],
        })
        .zip(UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(x, y, z)| {
                let xs_len = x;
                let ys_len = x.checked_add(1)?.checked_add(y)?;
                let out_len = x.checked_add(1)?.checked_add(z)?;
                Some((out_len, xs_len, ys_len))
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }),
    ))
}

struct HalfGcdMatrixAndHalfGcdMatrix1Generator {
    sizes: GeometricRandomNaturalValues<usize>,
    striped_bit_source: StripedBitSource,
    bit_chunks: StripedRandomUnsignedBitChunks<Limb>,
}

impl Iterator for HalfGcdMatrixAndHalfGcdMatrix1Generator {
    type Item = (OwnedHalfGcdMatrix, HalfGcdMatrix1);

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
            let m = special_random_half_gcd_matrix(s, n, &mut self.striped_bit_source);
            let m_1 = HalfGcdMatrix1 {
                data: [
                    [self.bit_chunks.next().unwrap(), self.bit_chunks.next().unwrap()],
                    [self.bit_chunks.next().unwrap(), self.bit_chunks.next().unwrap()],
                ],
            };
            return Some((m, m_1));
        }
    }
}

pub fn special_random_large_type_gen_var_7(
    config: &GenConfig,
) -> It<(OwnedHalfGcdMatrix, HalfGcdMatrix1)> {
    Box::new(HalfGcdMatrixAndHalfGcdMatrix1Generator {
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        bit_chunks: striped_random_unsigned_bit_chunks(
            EXAMPLE_SEED.fork("bit_chunks"),
            Limb::WIDTH - 1,
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct MatrixMul22Generator {
    sizes: GeometricRandomNaturalValues<u64>,
    striped_bit_source: StripedBitSource,
}

impl Iterator for MatrixMul22Generator {
    type Item = T8;

    fn next(&mut self) -> Option<Self::Item> {
        let ys_len = self.sizes.next().unwrap();
        let xs_len = self.sizes.next().unwrap();
        let sum_bits = (ys_len + xs_len + 1) << Limb::LOG_WIDTH;
        let ys_bits = ys_len << Limb::LOG_WIDTH;
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, sum_bits),
            get_striped_unsigned_vec(&mut self.striped_bit_source, sum_bits),
            get_striped_unsigned_vec(&mut self.striped_bit_source, sum_bits),
            get_striped_unsigned_vec(&mut self.striped_bit_source, sum_bits),
            usize::exact_from(xs_len),
            get_striped_unsigned_vec(&mut self.striped_bit_source, ys_bits),
            get_striped_unsigned_vec(&mut self.striped_bit_source, ys_bits),
            get_striped_unsigned_vec(&mut self.striped_bit_source, ys_bits),
            get_striped_unsigned_vec(&mut self.striped_bit_source, ys_bits),
        ))
    }
}

pub fn special_random_large_type_gen_var_8(config: &GenConfig) -> It<T8> {
    Box::new(MatrixMul22Generator {
        sizes: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// var 9 is in malachite-base.

pub fn special_random_large_type_gen_var_10(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    reshape_2_2_to_4(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| UnsignedVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            striped_bit_source: StripedBitSource::new(
                seed.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        },
        &|seed| {
            random_pairs(
                seed,
                &|seed_2| random_values_from_vec(seed_2, factors_of_limb_max()),
                &|seed_2| {
                    striped_random_unsigneds(
                        seed_2,
                        config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                        config.get_or("mean_stripe_d", 1),
                    )
                },
            )
        },
    )))
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_11(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(q_len, mut n_len, mut d_init_len)| {
                    n_len = n_len.checked_add(3)?;
                    d_init_len = d_init_len.checked_add(2)?;
                    let d_len = d_init_len + 1;
                    if n_len >= d_len && q_len >= n_len - d_len {
                        Some((q_len, n_len, d_init_len))
                    } else {
                        None
                    }
                }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    Limb::power_of_2(Limb::WIDTH - 1),
                    Limb::MAX,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (q, n, d_init, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_12(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| UnsignedVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(mut q_len, mut n_len, mut d_init_len)| {
                    q_len = q_len.checked_add(3)?;
                    n_len = n_len.checked_add(9)?;
                    d_init_len = d_init_len.checked_add(5)?;
                    let d_len = d_init_len + 1;
                    if n_len >= d_len + 3 && q_len >= n_len - d_len {
                        Some((q_len, n_len, d_init_len))
                    } else {
                        None
                    }
                }),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    Limb::power_of_2(Limb::WIDTH - 1),
                    Limb::MAX,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (q, n, d_init, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_13(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_positive_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter(|&(q_len, n_len, d_len)| q_len >= n_len && n_len >= d_len),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_14(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_positive_unsigneds::<u64>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut n_len, d_len)| {
                q_len = q_len.checked_add(1)?;
                n_len = n_len.checked_add(1)?;
                if q_len >= n_len && n_len > d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_15(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(mut q_len, mut n_len, d_len)| {
                q_len = q_len.checked_add(1)?;
                n_len = n_len.checked_add(1)?;
                if q_len >= n_len && n_len > d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_16(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                u64::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter(|&(q_len, n_len, d_len)| q_len >= n_len && n_len >= d_len),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_17(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", Limb::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .map(|mut d| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
            let is = vec![0; d.len()];
            let scratch = vec![0; limbs_modular_invert_scratch_len(d.len())];
            (is, scratch, d, inverse)
        }),
    )
}

pub fn special_random_large_type_gen_var_18(
    config: &GenConfig,
) -> It<(Vec<Limb>, usize, Limb, Limb, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(|(ns, fraction_len, d)| {
            if ns.len() <= fraction_len {
                None
            } else {
                let shift = LeadingZeros::leading_zeros(d);
                let d_inv = limbs_invert_limb(d << shift);
                Some((ns, fraction_len, d, d_inv, shift))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_19(
    config: &GenConfig,
) -> It<(Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)> {
    Box::new(
        random_quadruples_xyxz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(|(out, fraction_len, ns, d)| {
            if ns.is_empty() || out.len() < ns.len() + fraction_len {
                None
            } else {
                let shift = LeadingZeros::leading_zeros(d);
                let d_inv = limbs_invert_limb(d << shift);
                Some((out, fraction_len, ns, d, d_inv, shift))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_20(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    Box::new(
        random_quintuples_xyyyz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigned_vecs(
                    seed,
                    config.get_or("mean_stripe_n", Limb::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    3,
                    u32::MAX,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter_map(|(ds, mut scratch, mut qs, mut rs_hi, n_len)| {
            let n_len = usize::wrapping_from(n_len);
            let d_len = ds.len();
            if n_len < d_len {
                return None;
            }
            let i_len = limbs_div_mod_barrett_is_len(n_len - d_len, d_len);
            if i_len == 0 || qs.len() < i_len {
                return None;
            }
            qs.truncate(i_len);
            if rs_hi.len() < i_len {
                return None;
            }
            rs_hi.truncate(i_len);
            let scratch_len = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
            let x = limbs_div_mod_barrett_scratch_len(n_len, d_len);
            if x < i_len {
                return None;
            }
            let actual_scratch_len = x - i_len;
            if actual_scratch_len < d_len + i_len {
                return None;
            }
            if scratch.len() < actual_scratch_len {
                return None;
            }
            scratch.truncate(actual_scratch_len);
            Some((scratch, ds, qs, rs_hi, scratch_len, i_len))
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_21(
    config: &GenConfig,
) -> It<(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        random_sextuples_from_single(striped_random_positive_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter_map(large_type_filter_map_1),
    )
}

// var 22 is in malachite-base.

struct RationalFromPowerOf2DigitsGenerator {
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    log_bases: GeometricRandomNaturalValues<u64>,
    sizes: GeometricRandomNaturalValues<usize>,
    xs_map: HashMap<u64, StripedRandomNaturalInclusiveRange>,
}

impl Iterator for RationalFromPowerOf2DigitsGenerator {
    type Item = (u64, Vec<Natural>, RationalSequence<Natural>);

    fn next(&mut self) -> Option<(u64, Vec<Natural>, RationalSequence<Natural>)> {
        let log_base = self.log_bases.next().unwrap();
        let mean_stripe_n = self.mean_stripe_n;
        let mean_stripe_d = self.mean_stripe_d;
        let xs = self.xs_map.entry(log_base).or_insert_with(|| {
            let seed = EXAMPLE_SEED.fork(&log_base.to_string());
            striped_random_natural_range(
                seed,
                Natural::ZERO,
                Natural::power_of_2(log_base),
                mean_stripe_n,
                mean_stripe_d,
            )
        });
        let before_point = xs.take(self.sizes.next().unwrap()).collect();
        let non_repeating = xs.take(self.sizes.next().unwrap()).collect();
        let repeating = xs.take(self.sizes.next().unwrap()).collect();
        Some((
            log_base,
            before_point,
            RationalSequence::from_vecs(non_repeating, repeating),
        ))
    }
}

pub fn special_random_large_type_gen_var_23(
    config: &GenConfig,
) -> It<(u64, Vec<Natural>, RationalSequence<Natural>)> {
    Box::new(RationalFromPowerOf2DigitsGenerator {
        mean_stripe_n: config.get_or("mean_stripe_n", 32),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        log_bases: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("log_bases"),
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        ),
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs_map: HashMap::new(),
    })
}

struct RationalFromDigitsGenerator {
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    bases: StripedRandomNaturalRangeToInfinity,
    sizes: GeometricRandomNaturalValues<usize>,
    xs_map: HashMap<Natural, StripedRandomNaturalInclusiveRange>,
}

impl Iterator for RationalFromDigitsGenerator {
    type Item = (Natural, Vec<Natural>, RationalSequence<Natural>);

    fn next(&mut self) -> Option<(Natural, Vec<Natural>, RationalSequence<Natural>)> {
        let base = self.bases.next().unwrap();
        let mean_stripe_n = self.mean_stripe_n;
        let mean_stripe_d = self.mean_stripe_d;
        let xs = self.xs_map.entry(base.clone()).or_insert_with(|| {
            let seed = EXAMPLE_SEED.fork(&base.to_string());
            striped_random_natural_range(
                seed,
                Natural::ZERO,
                base.clone(),
                mean_stripe_n,
                mean_stripe_d,
            )
        });
        let before_point = xs.take(self.sizes.next().unwrap()).collect();
        let non_repeating = xs.take(self.sizes.next().unwrap()).collect();
        let repeating = xs.take(self.sizes.next().unwrap()).collect();
        Some((
            base,
            before_point,
            RationalSequence::from_vecs(non_repeating, repeating),
        ))
    }
}

pub fn special_random_large_type_gen_var_24(
    config: &GenConfig,
) -> It<(Natural, Vec<Natural>, RationalSequence<Natural>)> {
    Box::new(RationalFromDigitsGenerator {
        mean_stripe_n: config.get_or("mean_stripe_n", 32),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        bases: striped_random_natural_range_to_infinity(
            EXAMPLE_SEED,
            Natural::TWO,
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ),
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs_map: HashMap::new(),
    })
}
