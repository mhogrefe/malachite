// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use crate::integer::random::{
    RandomIntegers, random_integers, random_natural_integers, random_negative_integers,
    random_nonzero_integers,
};
use crate::natural::Natural;
use crate::natural::arithmetic::binomial_coefficient::BIN_GOETGHELUCK_THRESHOLD;
use crate::natural::arithmetic::binomial_coefficient::BIN_UIUI_RECURSIVE_SMALLDC;
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
use crate::natural::arithmetic::mul::limb::limbs_vec_mul_limb_in_place;
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::mul::mul_mod::limbs_mul_mod_base_pow_n_minus_1_next_size;
use crate::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
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
};
use crate::natural::arithmetic::square::{
    limbs_square_to_out_toom_3_input_size_valid, limbs_square_to_out_toom_4_input_size_valid,
    limbs_square_to_out_toom_6_input_size_valid, limbs_square_to_out_toom_8_input_size_valid,
};
use crate::natural::conversion::digits::general_digits::{
    GET_STR_PRECOMPUTE_THRESHOLD, limbs_digit_count, limbs_per_digit_in_base,
};
use crate::natural::random::{
    RandomNaturalRangeToInfinity, RandomNaturals, RandomNaturalsLessThan,
    get_random_natural_with_bits, get_random_natural_with_up_to_bits,
    random_natural_range_to_infinity, random_naturals, random_naturals_less_than,
    random_positive_naturals,
};
use crate::platform::{
    DoubleLimb, Limb, ODD_CENTRAL_BINOMIAL_OFFSET, ODD_CENTRAL_BINOMIAL_TABLE_LIMIT,
    ODD_FACTORIAL_EXTTABLE_LIMIT, ODD_FACTORIAL_TABLE_LIMIT, SQR_TOOM2_THRESHOLD,
};
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
use crate::test_util::generators::{T8, factors_of_limb_max, limbs_odd_factorial_valid};
use crate::test_util::natural::arithmetic::gcd::{OwnedHalfGcdMatrix, half_gcd_matrix_create};
use itertools::Itertools;
use malachite_base::bools::random::{RandomBools, random_bools};
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
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::string::options::random::random_to_sci_options;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, SaturatingFrom, ToSci, WrappingFrom,
};
use malachite_base::num::factorization::prime_sieve::n_to_bit;
use malachite_base::num::logic::traits::{
    BitAccess, BitConvertible, LeadingZeros, SignificantBits,
};
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, GeometricRandomSignedRange, GeometricRandomSigneds,
    geometric_random_positive_unsigneds, geometric_random_signed_range, geometric_random_signeds,
    geometric_random_unsigned_inclusive_range, geometric_random_unsigneds,
};
use malachite_base::num::random::{
    RandomPrimitiveInts, RandomUnsignedBitChunks, RandomUnsignedRange, RandomUnsignedsLessThan,
    VariableRangeGenerator, random_natural_signeds, random_positive_unsigneds,
    random_primitive_ints, random_unsigned_bit_chunks, random_unsigned_inclusive_range,
    random_unsigneds_less_than, special_random_primitive_floats,
};
use malachite_base::options::random::{RandomOptions, random_options};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::test_util::generators::common::{
    GenConfig, It, permute_1_3_2, reshape_1_3_to_4, reshape_2_1_to_3, reshape_2_2_to_4,
};
use malachite_base::test_util::generators::random::{
    PrimitiveIntVecPairLenGenerator1, PrimitiveIntVecPairLenGenerator2,
    PrimitiveIntVecQuadrupleLenGenerator1, PrimitiveIntVecTripleLenGenerator1,
    PrimitiveIntVecTripleXYYLenGenerator, get_two_highest,
    random_primitive_int_vec_unsigned_pair_gen_var_10,
};
use malachite_base::tuples::random::{
    random_ordered_unique_pairs, random_pairs, random_pairs_from_single,
};
use malachite_base::unions::Union2;
use malachite_base::unions::random::random_union2s;
use malachite_base::vecs::random::{
    RandomVecs, random_vecs, random_vecs_length_range, random_vecs_min_length,
};
use malachite_base::vecs::{RandomValuesFromVec, random_values_from_vec};
use num::{BigInt, BigUint};
use std::cmp::{Ordering::*, max};
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

#[allow(clippy::type_repetition_in_bounds)]
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

pub fn random_integer_gen_var_8(config: &GenConfig) -> It<Integer> {
    Box::new(
        random_natural_integers(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|n| (n << 1u32) | Integer::ONE),
    )
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

pub fn random_integer_pair_gen_var_4(config: &GenConfig) -> It<(Integer, Integer)> {
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
                random_natural_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(a, n)| (a, (n << 1u32) | Integer::ONE)),
    )
}

pub fn random_integer_pair_gen_var_5(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs_from_single(random_integers(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y)| x.unsigned_abs_ref().coprime_with(y.unsigned_abs_ref())),
    )
}

pub fn random_integer_pair_gen_var_6(config: &GenConfig) -> It<(Integer, Integer)> {
    Box::new(
        random_pairs_from_single(
            random_natural_integers(
                EXAMPLE_SEED,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
            .map(|n| (n << 1u32) | Integer::ONE),
        )
        .filter(|(x, y)| x.unsigned_abs_ref().coprime_with(y.unsigned_abs_ref())),
    )
}

pub fn random_integer_pair_gen_var_7(config: &GenConfig) -> It<(Integer, Integer)> {
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

pub fn random_integer_triple_gen_var_2(config: &GenConfig) -> It<(Integer, Integer, Integer)> {
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
                random_natural_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(a, b, n)| (a, b, (n << 1u32) | Integer::ONE)),
    )
}

pub fn random_integer_triple_gen_var_3(config: &GenConfig) -> It<(Integer, Integer, Integer)> {
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
                random_natural_integers(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .map(|(a, m, n)| (a, (m << 1u32) | Integer::ONE, (n << 1u32) | Integer::ONE)),
    )
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
        .filter(|(x, y, m)| !x.eq_mod(y, m)),
    )
}

// -- (Integer, Integer, PrimitiveFloat) --

pub fn random_integer_integer_primitive_float_triple_gen<T: PrimitiveFloat>(
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
            special_random_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
    ))
}

// -- (Integer, Integer, PrimitiveInt) --

pub fn random_integer_integer_primitive_int_triple_gen<T: PrimitiveInt>(
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
        &random_primitive_ints,
    ))
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
            if rm == Exact {
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

// -- (Integer, Natural, Natural) --

pub fn random_integer_natural_natural_triple_gen(
    config: &GenConfig,
) -> It<(Integer, Natural, Natural)> {
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
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Integer, PrimitiveFloat) --

pub fn random_integer_primitive_float_pair_gen<T: PrimitiveFloat>(
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
            special_random_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
    ))
}

// -- (Integer, PrimitiveFloat, PrimitiveFloat) --

pub fn random_integer_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
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
            special_random_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
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

// -- (Integer, PrimitiveInt, PrimitiveInt) --

pub fn random_integer_primitive_int_primitive_int_triple_gen<T: PrimitiveInt>(
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
        &random_primitive_ints,
    ))
}

// -- (Integer, PrimitiveSigned) --

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

// -- (Integer, PrimitiveSigned, PrimitiveUnsigned) --

pub fn random_integer_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
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

pub fn random_integer_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::from(36u8)),
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
                        .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE))
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
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), Down);
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

pub fn random_integer_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>(
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
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
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
    T: PrimitiveUnsigned,
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::from(36u8)),
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
            Less => Some((x, y, z, w)),
            Greater => Some((x, z, y, w)),
            Equal => None,
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
            if rm == Exact {
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
        .map(|(n, u, rm)| (if rm == Exact { n << u } else { n }, u, rm)),
    )
}

// vars 3 and 4 are in malachite-float.

pub fn random_integer_unsigned_rounding_mode_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Integer, T, RoundingMode)> {
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
            geometric_random_unsigneds::<T>(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
        &random_rounding_modes,
    ))
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
        .filter(|&(ref n, rm)| rm != Exact || T::convertible_from(n)),
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

// -- (Integer, ToSciOptions) --

pub fn random_integer_to_sci_options_pair_gen(config: &GenConfig) -> It<(Integer, ToSciOptions)> {
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
            random_to_sci_options(
                seed,
                config.get_or("small_mean_n", 4),
                config.get_or("small_mean_d", 1),
            )
        },
    ))
}

pub fn random_integer_to_sci_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Integer, ToSciOptions)> {
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
        ranges: VariableRangeGenerator::new(seed.fork("mantissas")),
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

#[allow(clippy::type_repetition_in_bounds)]
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

pub fn random_natural_gen_var_8(config: &GenConfig) -> It<Natural> {
    Box::new(
        random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|n| (n << 1u32) | Natural::ONE),
    )
}

pub fn random_natural_gen_var_9(config: &GenConfig) -> It<Natural> {
    Box::new(
        geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED,
            config.get_or("mean_n", 64),
            config.get_or("mean_d", 1),
        )
        .map(Natural::from),
    )
}

// -- (Natural, bool) --

pub fn random_natural_bool_pair_gen(config: &GenConfig) -> It<(Natural, bool)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_bools,
    ))
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

pub fn random_natural_pair_gen_var_8(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_ordered_unique_pairs(random_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_natural_pair_gen_var_9(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_pairs_from_single(random_positive_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_natural_pair_gen_var_10(config: &GenConfig) -> It<(Natural, Natural)> {
    // TODO
    Box::new(
        random_pairs_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y)| x >= y),
    )
}

pub fn random_natural_pair_gen_var_11(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(random_ordered_unique_pairs(random_positive_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_natural_pair_gen_var_12(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(a, n)| (a, (n << 1u32) | Natural::ONE)),
    )
}

pub fn random_natural_pair_gen_var_13(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs_from_single(
            random_naturals(
                EXAMPLE_SEED,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
            .map(|n| (n << 1u32) | Natural::ONE),
        )
        .filter(|(x, y)| x.coprime_with(y)),
    )
}

pub fn random_natural_pair_gen_var_14(config: &GenConfig) -> It<(Natural, Natural)> {
    Box::new(
        random_pairs_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y)| x.coprime_with(y)),
    )
}

pub fn random_natural_pair_gen_var_15(config: &GenConfig) -> It<(Natural, Natural)> {
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

pub fn random_natural_natural_bool_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, bool)> {
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
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_bools,
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

pub fn random_natural_triple_gen_var_1(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, m)| (x * &m + &y, y, m)),
    )
}

pub fn random_natural_triple_gen_var_2(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .filter(|(x, y, m)| !x.eq_mod(y, m)),
    )
}

pub fn random_natural_triple_gen_var_3(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| {
            let z = max(&x, &y) + z + Natural::ONE;
            (x, y, z)
        }),
    )
}

pub fn random_natural_triple_gen_var_4(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
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
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_natural_triple_gen_var_5(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
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

pub fn random_natural_triple_gen_var_6(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(random_triples_from_single(random_positive_naturals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_natural_triple_gen_var_7(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| (x + &y * &z, y, z)),
    )
}

pub fn random_natural_triple_gen_var_8(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(a, b, n)| (a, b, (n << 1u32) | Natural::ONE)),
    )
}

pub fn random_natural_triple_gen_var_9(config: &GenConfig) -> It<(Natural, Natural, Natural)> {
    Box::new(
        random_triples_from_single(random_naturals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(a, m, n)| (a, (m << 1u32) | Natural::ONE, (n << 1u32) | Natural::ONE)),
    )
}

// -- (Natural, Natural, Natural, Natural) --

pub fn random_natural_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        random_quadruples_from_single(random_naturals(
            EXAMPLE_SEED,
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

pub fn random_natural_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        random_quadruples_from_single(random_naturals(
            EXAMPLE_SEED,
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

pub fn random_natural_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        random_quadruples_from_single(random_naturals(
            EXAMPLE_SEED,
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

pub fn random_natural_natural_natural_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, T)> {
    Box::new(random_quadruples_xxxy(
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

pub fn random_natural_natural_natural_unsigned_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        random_quadruples_xxxy(
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

pub fn random_natural_natural_natural_unsigned_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        random_quadruples_xxxy(
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
        .map(|(x, y, z, mut m)| {
            m += max(x.significant_bits(), y.significant_bits());
            (x, y, z, m)
        }),
    )
}

pub fn random_natural_natural_natural_unsigned_quadruple_gen_var_4(
    config: &GenConfig,
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        random_quadruples_xxxy(
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
        .map(|(x, y, z, mut m)| {
            m += x.significant_bits();
            (x, y, z, m)
        }),
    )
}

// -- (Natural, Natural, PrimitiveFloat) --

pub fn random_natural_natural_primitive_float_triple_gen<T: PrimitiveFloat>(
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
            special_random_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
    ))
}

// -- (Natural, Natural, PrimitiveInt) --

pub fn random_natural_natural_primitive_int_triple_gen<T: PrimitiveInt>(
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
        &random_primitive_ints,
    ))
}

// -- (Natural, Natural, PrimitiveSigned) --

pub fn random_natural_natural_signed_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_ordered_unique_pairs(random_naturals(
                seed,
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

pub fn random_natural_natural_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        random_triples_xxy(
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
        .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

pub fn random_natural_natural_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_naturals(
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

pub fn random_natural_natural_unsigned_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Natural, Natural, u64)> {
    Box::new(
        random_triples_xxy(
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
        .map(|(x, y, mut m)| {
            m += max(x.significant_bits(), y.significant_bits());
            (x, y, m)
        }),
    )
}

pub fn random_natural_natural_unsigned_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Natural, Natural, u64)> {
    Box::new(
        random_triples_xxy(
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
        .map(|(x, y, mut m)| {
            m += x.significant_bits();
            (x, y, m)
        }),
    )
}

pub fn random_natural_natural_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, Natural, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_ordered_unique_pairs(random_naturals(
                seed,
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
            if rm == Exact {
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

// -- (Natural, PrimitiveFloat) --

pub fn random_natural_primitive_float_pair_gen<T: PrimitiveFloat>(
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
            special_random_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
    ))
}

// -- (Natural, PrimitiveFloat, PrimitiveFloat) --

pub fn random_natural_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            special_random_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
    ))
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

pub fn random_natural_primitive_int_pair_gen_var_1<T: PrimitiveInt>(
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
        &random_primitive_ints,
    ))
}

// -- (Natural, PrimitiveInt, PrimitiveInt) --

pub fn random_natural_primitive_int_primitive_int_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
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

struct NaturalBitsMultipleOfLimbBitsGenerator {
    limbs: RandomPrimitiveInts<u64>,
    limb_counts: GeometricRandomNaturalValues<u64>,
}

impl Iterator for NaturalBitsMultipleOfLimbBitsGenerator {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(get_random_natural_with_bits(
            &mut self.limbs,
            self.limb_counts.next().unwrap() << Limb::LOG_WIDTH,
        ))
    }
}

pub fn random_natural_signed_pair_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| NaturalBitsMultipleOfLimbBitsGenerator {
            limbs: random_primitive_ints(seed.fork("limbs")),
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

// -- (Natural, PrimitiveSigned, PrimitiveUnsigned) --

pub fn random_natural_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Natural, T, u64)> {
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

pub fn random_natural_signed_unsigned_triple_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
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

pub fn random_natural_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::from(36u8)),
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

pub fn random_natural_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
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

pub fn random_natural_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T)> {
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

pub fn random_natural_unsigned_pair_gen_var_11(config: &GenConfig) -> It<(Natural, u64)> {
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

pub fn random_natural_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>(
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
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_natural_unsigned_pair_gen_var_13(config: &GenConfig) -> It<(Natural, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_positive_naturals(
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
        .map(|(x, mut m)| {
            m += x.significant_bits();
            (x, m)
        }),
    )
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::from(36u8)),
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

pub fn random_natural_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, T)> {
    Box::new(random_triples_xyy(
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
    ))
}

pub fn random_natural_unsigned_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Natural, T, u64)> {
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
            Less => Some((x, y, z, w)),
            Greater => Some((x, z, y, w)),
            Equal => None,
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
                x.significant_bits().div_round(log_base, Ceiling).0,
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
        .filter(|&(ref n, rm)| rm != Exact || T::convertible_from(n)),
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

// -- (Natural, ToSciOptions) --

pub fn random_natural_to_sci_options_pair_gen(config: &GenConfig) -> It<(Natural, ToSciOptions)> {
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
            random_to_sci_options(
                seed,
                config.get_or("small_mean_n", 4),
                config.get_or("small_mean_d", 1),
            )
        },
    ))
}

pub fn random_natural_to_sci_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Natural, ToSciOptions)> {
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

// -- (Vec<PrimitiveUnsigned>, bool) --

pub fn random_unsigned_bool_pair_gen_var_1(config: &GenConfig) -> It<(usize, bool)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_n", 16),
                    config.get_or("mean_d", 1),
                )
            },
            &random_bools,
        )
        .filter(|&(n, b)| limbs_odd_factorial_valid(n, b)),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 32 are in malachite-base

pub fn random_unsigned_pair_gen_var_33<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    T::exact_from(ODD_FACTORIAL_TABLE_LIMIT + 1),
                    T::MAX,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|&(n, k)| n >= k),
    )
}

pub fn random_unsigned_pair_gen_var_34<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    T::TWO,
                    T::wrapping_from(ODD_FACTORIAL_TABLE_LIMIT),
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|&(n, k)| n >= k),
    )
}

pub fn random_unsigned_pair_gen_var_35<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    T::from(4u8),
                    T::wrapping_from(ODD_FACTORIAL_EXTTABLE_LIMIT),
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    T::TWO,
                    T::wrapping_from(ODD_FACTORIAL_EXTTABLE_LIMIT - 2),
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|&(n, k)| n >= k + T::TWO),
    )
}

pub fn random_unsigned_pair_gen_var_36<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    T::wrapping_from((ODD_CENTRAL_BINOMIAL_OFFSET << 1) + 1),
                    T::MAX,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    T::wrapping_from((ODD_CENTRAL_BINOMIAL_OFFSET << 1) - 1),
                    T::wrapping_from(if BIN_UIUI_RECURSIVE_SMALLDC {
                        ODD_CENTRAL_BINOMIAL_TABLE_LIMIT
                    } else {
                        ODD_FACTORIAL_TABLE_LIMIT
                    }) << 1,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|&(n, k)| n >= k + T::TWO),
    )
}

pub fn random_unsigned_pair_gen_var_37(config: &GenConfig) -> It<(Limb, Limb)> {
    Box::new(
        random_pairs_from_single(geometric_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_small_n", 32),
            config.get_or("mean_small_d", 1),
        ))
        .filter_map(|(mut n, mut k)| {
            n += u64::wrapping_from(BIN_GOETGHELUCK_THRESHOLD) << 1;
            k += u64::wrapping_from(BIN_GOETGHELUCK_THRESHOLD);
            if n >= k + 5 && k > (n >> 4) && n_to_bit(n - k) < n_to_bit(n) && k <= n - k {
                Some((Limb::exact_from(n), Limb::exact_from(k)))
            } else {
                None
            }
        }),
    )
}

// var 38 is in malachite-base.

// -- (PrimitiveUnsigned * 6) --

pub fn random_unsigned_sextuple_gen_var_1(
    _config: &GenConfig,
) -> It<(Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| random_pairs_from_single(random_primitive_ints(seed_2)),
                    &|seed_2| {
                        random_pairs(
                            seed_2,
                            &|seed_3| {
                                random_unsigned_inclusive_range(
                                    seed_3,
                                    Limb::power_of_2(Limb::WIDTH - 1),
                                    Limb::MAX,
                                )
                            },
                            &random_primitive_ints,
                        )
                    },
                )
                .filter(|&((n_2, n_1), (d_1, d_0))| n_2 < d_1 || n_2 == d_1 && n_1 < d_0)
            },
            &random_primitive_ints,
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

pub fn random_string_triple_gen_var_1(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        random_naturals(
            EXAMPLE_SEED.fork("xs"),
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

pub fn random_string_triple_gen_var_2(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        random_integers(
            EXAMPLE_SEED.fork("xs"),
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

pub fn random_integer_vec_gen(config: &GenConfig) -> It<Vec<Integer>> {
    Box::new(random_vecs(
        EXAMPLE_SEED,
        &|seed| {
            random_integers(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        config.get_or("mean_len_n", 4),
        config.get_or("mean_len_d", 1),
    ))
}

// -- Vec<Natural> --

pub fn random_natural_vec_gen(config: &GenConfig) -> It<Vec<Natural>> {
    Box::new(random_vecs(
        EXAMPLE_SEED,
        &|seed| {
            random_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        config.get_or("mean_len_n", 4),
        config.get_or("mean_len_d", 1),
    ))
}

// -- (Vec<Natural>, Integer> --

pub fn random_natural_vec_integer_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Natural>, Integer)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &|seed_2| {
                    random_positive_naturals(
                        seed_2,
                        config.get_or("mean_bits_n", 64),
                        config.get_or("mean_bits_d", 1),
                    )
                },
                config.get_or("mean_len_n", 4),
                config.get_or("mean_len_d", 1),
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

struct PowerOf2DigitsGenerator {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<u64>,
}

impl Iterator for PowerOf2DigitsGenerator {
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
    Box::new(PowerOf2DigitsGenerator {
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

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>) --

// vars 1 through 8 are in malachite-base.

pub fn random_primitive_int_vec_pair_gen_var_9(config: &GenConfig) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecPairLenGenerator2 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            xs: random_primitive_ints::<Limb>(EXAMPLE_SEED.fork("xs")),
        }
        .filter(|(xs, ys)| gcd_input_filter(xs, ys)),
    )
}

// vars 10 through 13 are in malachite-base.

pub fn random_primitive_int_vec_pair_gen_var_14(config: &GenConfig) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs_from_single(
            random_vecs_min_length(
                EXAMPLE_SEED,
                2,
                &random_primitive_ints,
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

// var 15 is in malachite-base.

pub fn random_primitive_int_vec_pair_gen_var_16<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs(
            EXAMPLE_SEED.fork("lengths"),
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    1,
                    usize::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigned_inclusive_range(
                    seed,
                    1,
                    SQRLO_DC_THRESHOLD_LIMIT,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
        )
        .filter(|(x, y)| x >= y),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

fn random_square_helper<T: PrimitiveInt, F: Fn(usize) -> bool>(
    config: &GenConfig,
    valid: &'static F,
    min_x: usize,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
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
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_pair_gen_var_17<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &|x| x <= SQR_TOOM2_THRESHOLD, 1)
}

pub fn random_primitive_int_vec_pair_gen_var_18<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &|_| true, 2)
}

pub fn random_primitive_int_vec_pair_gen_var_19<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &limbs_square_to_out_toom_3_input_size_valid, 3)
}

pub fn random_primitive_int_vec_pair_gen_var_20<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &limbs_square_to_out_toom_4_input_size_valid, 4)
}

pub fn random_primitive_int_vec_pair_gen_var_21<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &|x| x == 7 || x == 8 || x > 9, 7)
}

pub fn random_primitive_int_vec_pair_gen_var_22<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &limbs_square_to_out_toom_6_input_size_valid, 18)
}

pub fn random_primitive_int_vec_pair_gen_var_23<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_square_helper(config, &limbs_square_to_out_toom_8_input_size_valid, 40)
}

// vars 26 to 27 are in malachite-base.

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

// var 39 through 44 are in malachite-base.

pub fn random_primitive_int_vec_triple_gen_var_45<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    random_mul_helper(
        config,
        &|x, y| {
            limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
                && limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
        },
        7,
        7,
    )
}

// var 46 is in malachite-base.

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

// -- Vec<PrimitiveUnsigned> --

pub fn random_unsigned_vec_gen_var_1(config: &GenConfig) -> It<Vec<Limb>> {
    Box::new(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .map(|mut xs| {
            limbs_vec_mul_limb_in_place(&mut xs, 3);
            xs
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

pub fn random_unsigned_vec_unsigned_pair_gen_var_10(config: &GenConfig) -> It<(Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    1,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &random_positive_unsigneds,
        )
        .map(|(mut xs, y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y);
            (xs, y)
        }),
    )
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_11(config: &GenConfig) -> It<(Vec<Limb>, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    1,
                    &random_primitive_ints::<Limb>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
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

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

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

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| random_values_from_vec(seed, factors_of_limb_max()),
        &random_primitive_ints,
    ))
}

// var 3 is in malachite-base.

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        random_triples(
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
            &random_primitive_ints,
            &random_positive_unsigneds,
        )
        .map(map_helper_3),
    )
}

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        random_triples(
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
            &random_primitive_ints,
            &random_positive_unsigneds,
        )
        .filter(filter_helper_6),
    )
}

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_6<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints::<Limb>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &random_primitive_ints::<T>,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            (xs, y, pow)
        }),
    )
}

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_7<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    1,
                    &random_primitive_ints::<Limb>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &random_primitive_ints::<T>,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            (xs, y, pow)
        }),
    )
}

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_8<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints::<Limb>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &random_primitive_ints::<T>,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
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

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4(
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
        .map(map_helper_1),
    )
}

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5(
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
        .filter(filter_helper_4),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 2 are in malachite-base.

pub fn random_unsigned_vec_pair_gen_var_3(config: &GenConfig) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
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

pub fn random_unsigned_vec_pair_gen_var_4(config: &GenConfig) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs_from_single(random_vecs_min_length(
            EXAMPLE_SEED,
            2,
            &random_primitive_ints,
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

// var 5 is in malachite-base.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// var 1 is in malachite-base.

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
        .filter_map(filter_map_helper_2),
    )
}

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3(
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

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
                    seed.fork("lengths"),
                    2,
                    usize::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &random_positive_unsigneds,
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

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5(
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
        .map(map_helper_2),
    )
}

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6(
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
        .filter(filter_helper_5),
    )
}

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
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
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_unsigned_inclusive_range(seed, Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX)
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

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_7(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints::<Limb>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
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

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_8(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigneds(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
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

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_9(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    1,
                    &random_primitive_ints::<Limb>,
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
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
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

pub fn random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_10(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    1,
                    &random_primitive_ints::<Limb>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 4),
                    config.get_or("mean_small_d", 1),
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

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub fn random_unsigned_vec_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_unsigned_inclusive_range(seed, Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX)
            },
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

pub fn random_unsigned_vec_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_unsigned_inclusive_range(seed, Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX)
            },
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

pub fn random_unsigned_vec_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter(|(_, _, d)| *d.last().unwrap() != 0),
    )
}

pub fn random_unsigned_vec_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter(|(_, _, d)| *d.last().unwrap() != 0),
    )
}

pub fn random_unsigned_vec_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(
                geometric_random_unsigned_inclusive_range::<usize>(
                    EXAMPLE_SEED.fork("lengths"),
                    2,
                    usize::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ),
            )
            .filter_map(|(q_len, n_len, d_len)| {
                if q_len >= n_len && n_len >= d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            (q, n, d)
        }),
    )
}

pub fn random_unsigned_vec_triple_gen_var_6(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(
                geometric_random_unsigned_inclusive_range::<usize>(
                    EXAMPLE_SEED.fork("lengths"),
                    1,
                    usize::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ),
            )
            .filter_map(|(q_len, n_len, d_len)| {
                if q_len >= n_len && n_len >= d_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            (q, n, d)
        }),
    )
}

pub fn random_unsigned_vec_triple_gen_var_7(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(
                geometric_random_unsigned_inclusive_range::<usize>(
                    EXAMPLE_SEED.fork("lengths"),
                    1,
                    usize::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ),
            )
            .filter_map(|(q_len, n_len, d_len)| {
                if q_len + 1 >= n_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter_map(|(q, n, mut d): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
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

pub fn random_unsigned_vec_triple_gen_var_8(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(
                geometric_random_unsigned_inclusive_range::<usize>(
                    EXAMPLE_SEED.fork("lengths"),
                    1,
                    usize::MAX,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ),
            )
            .filter_map(|(q_len, n_len, mut d_len)| {
                d_len = d_len.checked_add(1)?;
                if q_len + 1 >= n_len {
                    Some((q_len, n_len, d_len))
                } else {
                    None
                }
            }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter_map(|(q, n, mut d): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
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

// vars 9 through 10 are in malachite-base.

pub fn random_unsigned_vec_triple_gen_var_11(
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
        .map(|(xs, ys, m)| limbs_eq_mod_map(&xs, ys, m)),
    )
}

pub fn random_unsigned_vec_triple_gen_var_12(
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
        .filter(|(xs, ys, m)| !limbs_eq_mod_ref_ref_ref(xs, ys, m)),
    )
}

pub fn random_unsigned_vec_triple_gen_var_13(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter_map(|(q_len, n_len): (usize, usize)| {
                    Some((q_len.checked_add(n_len)?, n_len.checked_add(2)?))
                }),
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_unsigned_inclusive_range(
                            seed_2,
                            Limb::power_of_2(Limb::WIDTH - 1),
                            Limb::MAX,
                        )
                    },
                    &random_primitive_ints,
                )
            },
        )
        .map(|((q, n), (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// -- (Vec<PrimitiveUnsigned> * 4) --

#[allow(clippy::type_complexity)]
pub fn random_unsigned_vec_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
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
pub fn random_unsigned_vec_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, r, mut d)| {
            d[0] |= 1;
            (q, n, r, d)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_unsigned_vec_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, r, mut d)| {
            d[0] |= 1;
            (q, n, r, d)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_unsigned_vec_quadruple_gen_var_4(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
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
pub fn random_unsigned_vec_quadruple_gen_var_5(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
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
pub fn random_unsigned_vec_quadruple_gen_var_6(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        PrimitiveIntVecQuadrupleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_quadruples_from_single(geometric_random_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
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
pub fn random_unsigned_vec_quadruple_gen_var_7(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        random_unsigned_vec_quadruple_gen_var_6(config).map(|(out, bs, es, mut ms)| {
            ms[0] |= 1;
            (out, bs, es, ms)
        }),
    )
}

// -- large types --

// vars 1 through 4 are in malachite-base

fn random_half_gcd_matrix(
    s: usize,
    n: usize,
    xs: &mut RandomPrimitiveInts<Limb>,
) -> OwnedHalfGcdMatrix {
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
            let m = random_half_gcd_matrix(m_s, m_n, &mut self.xs);
            let qs = (&mut self.xs).take(qs_len).collect_vec();
            let column = u8::from(self.bs.next().unwrap());
            return Some((m, qs, column));
        }
    }
}

pub fn random_large_type_gen_var_5(config: &GenConfig) -> It<(OwnedHalfGcdMatrix, Vec<Limb>, u8)> {
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
                .filter_map(|(x, y, z)| {
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

pub fn random_large_type_gen_var_7(config: &GenConfig) -> It<(OwnedHalfGcdMatrix, HalfGcdMatrix1)> {
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

// var 9 is in malachite-base.

pub fn random_large_type_gen_var_10(config: &GenConfig) -> It<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    reshape_2_2_to_4(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| PrimitiveIntVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            xs: random_primitive_ints(seed.fork("xs")),
        },
        &|seed| {
            random_pairs(
                seed,
                &|seed_2| random_values_from_vec(seed_2, factors_of_limb_max()),
                &random_primitive_ints,
            )
        },
    )))
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_11(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_unsigned_inclusive_range(seed, Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX)
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
pub fn random_large_type_gen_var_12(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecTripleLenGenerator1 {
                phantom: PhantomData,
                lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_unsigned_inclusive_range(seed, Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX)
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
pub fn random_large_type_gen_var_13(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_positive_unsigneds::<usize>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter(|&(q_len, n_len, d_len)| q_len >= n_len && n_len >= d_len),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb::<Limb>(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_14(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_positive_unsigneds::<usize>(
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb::<Limb>(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_15(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                usize::MAX,
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
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb::<Limb>(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_16(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter(|&(q_len, n_len, d_len)| q_len >= n_len && n_len >= d_len),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(q, n, mut d)| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb::<Limb>(d[0]).wrapping_neg();
            (q, n, d, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_17(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .map(|mut d| {
            d[0] |= 1;
            let inverse = limbs_modular_invert_limb::<Limb>(d[0]).wrapping_neg();
            let is = vec![0; d.len()];
            let scratch = vec![0; limbs_modular_invert_scratch_len(d.len())];
            (is, scratch, d, inverse)
        }),
    )
}

pub fn random_large_type_gen_var_18(config: &GenConfig) -> It<(Vec<Limb>, usize, Limb, Limb, u64)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints,
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
            &random_positive_unsigneds,
        )
        .filter_map(|(ns, fraction_len, d)| {
            if ns.len() <= fraction_len {
                None
            } else {
                let shift = LeadingZeros::leading_zeros(d);
                let d_inv = limbs_invert_limb::<DoubleLimb, Limb>(d << shift);
                Some((ns, fraction_len, d, d_inv, shift))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_19(
    config: &GenConfig,
) -> It<(Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)> {
    Box::new(
        random_quadruples_xyxz(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints,
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
            &random_positive_unsigneds,
        )
        .filter_map(|(out, fraction_len, ns, d)| {
            if ns.is_empty() || out.len() < ns.len() + fraction_len {
                None
            } else {
                let shift = LeadingZeros::leading_zeros(d);
                let d_inv = limbs_invert_limb::<DoubleLimb, Limb>(d << shift);
                Some((out, fraction_len, ns, d, d_inv, shift))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_20(
    config: &GenConfig,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    Box::new(
        random_quintuples_xyyyz(
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
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints,
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
pub fn random_large_type_gen_var_21(
    _config: &GenConfig,
) -> It<(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        random_sextuples_from_single(random_primitive_ints(EXAMPLE_SEED))
            .filter_map(large_type_filter_map_1),
    )
}

// var 22 is in malachite-base.

struct RationalFromPowerOf2DigitsGenerator {
    log_bases: GeometricRandomNaturalValues<u64>,
    sizes: GeometricRandomNaturalValues<usize>,
    xs_map: HashMap<u64, RandomNaturalsLessThan>,
}

impl Iterator for RationalFromPowerOf2DigitsGenerator {
    type Item = (u64, Vec<Natural>, RationalSequence<Natural>);

    fn next(&mut self) -> Option<(u64, Vec<Natural>, RationalSequence<Natural>)> {
        let log_base = self.log_bases.next().unwrap();
        let xs = self.xs_map.entry(log_base).or_insert_with(|| {
            let seed = EXAMPLE_SEED.fork(&log_base.to_string());
            random_naturals_less_than(seed, Natural::power_of_2(log_base))
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

pub fn random_large_type_gen_var_23(
    config: &GenConfig,
) -> It<(u64, Vec<Natural>, RationalSequence<Natural>)> {
    Box::new(RationalFromPowerOf2DigitsGenerator {
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

struct RationalFromPowerOf2DigitsBinaryGenerator {
    sizes: GeometricRandomNaturalValues<usize>,
    bits: RandomBools,
}

impl Iterator for RationalFromPowerOf2DigitsBinaryGenerator {
    type Item = (Vec<Natural>, RationalSequence<Natural>);

    fn next(&mut self) -> Option<(Vec<Natural>, RationalSequence<Natural>)> {
        let before_point = (&mut self.bits)
            .map(Natural::from)
            .take(self.sizes.next().unwrap())
            .collect();
        let non_repeating = (&mut self.bits)
            .map(Natural::from)
            .take(self.sizes.next().unwrap())
            .collect();
        let repeating = (&mut self.bits)
            .map(Natural::from)
            .take(self.sizes.next().unwrap())
            .collect();
        Some((
            before_point,
            RationalSequence::from_vecs(non_repeating, repeating),
        ))
    }
}

pub fn random_large_type_gen_var_24(
    config: &GenConfig,
) -> It<(Vec<Natural>, RationalSequence<Natural>)> {
    Box::new(RationalFromPowerOf2DigitsBinaryGenerator {
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        bits: random_bools(EXAMPLE_SEED.fork("bits")),
    })
}

struct RationalFromDigitsGenerator {
    bases: RandomNaturalRangeToInfinity,
    sizes: GeometricRandomNaturalValues<usize>,
    xs_map: HashMap<Natural, RandomNaturalsLessThan>,
}

impl Iterator for RationalFromDigitsGenerator {
    type Item = (Natural, Vec<Natural>, RationalSequence<Natural>);

    fn next(&mut self) -> Option<(Natural, Vec<Natural>, RationalSequence<Natural>)> {
        let base = self.bases.next().unwrap();
        let xs = self.xs_map.entry(base.clone()).or_insert_with(|| {
            let seed = EXAMPLE_SEED.fork(&base.to_string());
            random_naturals_less_than(seed, base.clone())
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

pub fn random_large_type_gen_var_25(
    config: &GenConfig,
) -> It<(Natural, Vec<Natural>, RationalSequence<Natural>)> {
    Box::new(RationalFromDigitsGenerator {
        bases: random_natural_range_to_infinity(
            EXAMPLE_SEED,
            Natural::TWO,
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

struct RationalFromDigitsDecimalGenerator {
    sizes: GeometricRandomNaturalValues<usize>,
    digits: RandomNaturalsLessThan,
}

impl Iterator for RationalFromDigitsDecimalGenerator {
    type Item = (Vec<Natural>, RationalSequence<Natural>);

    fn next(&mut self) -> Option<(Vec<Natural>, RationalSequence<Natural>)> {
        let before_point = (&mut self.digits)
            .take(self.sizes.next().unwrap())
            .collect();
        let non_repeating = (&mut self.digits)
            .take(self.sizes.next().unwrap())
            .collect();
        let repeating = (&mut self.digits)
            .take(self.sizes.next().unwrap())
            .collect();
        Some((
            before_point,
            RationalSequence::from_vecs(non_repeating, repeating),
        ))
    }
}

pub fn random_large_type_gen_var_26(
    config: &GenConfig,
) -> It<(Vec<Natural>, RationalSequence<Natural>)> {
    Box::new(RationalFromDigitsDecimalGenerator {
        sizes: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("sizes"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        digits: random_naturals_less_than(EXAMPLE_SEED.fork("digits"), Natural::from(10u32)),
    })
}
