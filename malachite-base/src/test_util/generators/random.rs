// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::random::{random_bools, RandomBools};
use crate::chars::constants::NUMBER_OF_CHARS;
use crate::chars::random::{
    random_ascii_chars, random_char_inclusive_range, random_char_range, random_chars,
};
use crate::iterators::with_special_value;
use crate::num::arithmetic::traits::CoprimeWith;
use crate::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, Parity, PowerOf2, ShrRound, UnsignedAbs,
};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::string::options::random::{
    random_from_sci_string_options, random_sci_size_options, random_to_sci_options,
    RandomFromSciStringOptions,
};
use crate::num::conversion::string::options::{FromSciStringOptions, SciSizeOptions, ToSciOptions};
use crate::num::conversion::traits::{
    ConvertibleFrom, Digits, ExactFrom, HasHalf, JoinHalves, RoundingFrom, SaturatingFrom,
    SplitInHalf, WrappingFrom, WrappingInto,
};
use crate::num::float::NiceFloat;
use crate::num::logic::traits::{BitBlockAccess, LeadingZeros};
use crate::num::random::geometric::{
    geometric_random_natural_signeds, geometric_random_negative_signeds,
    geometric_random_nonzero_signeds, geometric_random_positive_unsigneds,
    geometric_random_signed_inclusive_range, geometric_random_signed_range,
    geometric_random_signeds, geometric_random_unsigned_inclusive_range,
    geometric_random_unsigned_range, geometric_random_unsigneds, GeometricRandomNaturalValues,
    GeometricRandomSignedRange, GeometricRandomSigneds,
};
use crate::num::random::{
    random_highest_bit_set_unsigneds, random_natural_signeds, random_negative_signeds,
    random_nonzero_signeds, random_positive_signeds, random_positive_unsigneds,
    random_primitive_ints, random_signed_inclusive_range, random_signed_range,
    random_unsigned_bit_chunks, random_unsigned_inclusive_range, random_unsigned_range,
    random_unsigneds_less_than, special_random_finite_primitive_floats,
    special_random_nonzero_finite_primitive_floats,
    special_random_positive_finite_primitive_floats,
    special_random_primitive_float_inclusive_range, special_random_primitive_float_range,
    special_random_primitive_floats, RandomPrimitiveInts, RandomUnsignedBitChunks,
    RandomUnsignedInclusiveRange, RandomUnsignedRange, SpecialRandomNonzeroFiniteFloats,
    VariableRangeGenerator,
};
use crate::random::{Seed, EXAMPLE_SEED};
use crate::rational_sequences::random::random_rational_sequences;
use crate::rational_sequences::RationalSequence;
use crate::rounding_modes::random::{random_rounding_modes, RandomRoundingModes};
use crate::rounding_modes::RoundingMode::{self, *};
use crate::slices::slice_test_zero;
use crate::strings::random::{random_strings, random_strings_using_chars};
use crate::strings::strings_from_char_vecs;
use crate::test_util::extra_variadic::{
    random_duodecuples_from_single, random_octuples_from_single, random_quadruples_from_single,
    random_quadruples_xxxy, random_quadruples_xxyx, random_quadruples_xyxy, random_quadruples_xyyx,
    random_quadruples_xyyz, random_quadruples_xyzz, random_sextuples_from_single, random_triples,
    random_triples_from_single, random_triples_xxy, random_triples_xyx, random_triples_xyy,
    random_union3s, Union3,
};
use crate::test_util::generators::common::{
    reshape_1_2_to_3, reshape_2_1_to_3, reshape_2_2_to_4, reshape_3_1_to_4, GenConfig, It,
};
use crate::test_util::generators::exhaustive::{
    float_rounding_mode_filter_var_1, valid_digit_chars,
};
use crate::test_util::generators::{
    digits_valid, large_exponent, round_to_multiple_of_power_of_2_filter_map,
    round_to_multiple_signed_filter_map, round_to_multiple_unsigned_filter_map,
    signed_assign_bits_valid, smallest_invalid_value, unsigned_assign_bits_valid,
};
use crate::test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;
use crate::test_util::num::conversion::string::from_sci_string::DECIMAL_SCI_STRING_CHARS;
use crate::test_util::num::float::PRIMITIVE_FLOAT_CHARS;
use crate::test_util::rounding_modes::ROUNDING_MODE_CHARS;
use crate::tuples::random::{random_ordered_unique_pairs, random_pairs, random_pairs_from_single};
use crate::unions::random::random_union2s;
use crate::unions::Union2;
use crate::vecs::random::{
    random_vecs, random_vecs_fixed_length_from_single, random_vecs_length_inclusive_range,
    random_vecs_min_length,
};
use crate::vecs::random_values_from_vec;
use itertools::repeat_n;
use itertools::Itertools;
use std::cmp::{max, min, Ordering::*};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::swap;

// -- bool --

pub fn random_bool_gen(_config: &GenConfig) -> It<bool> {
    Box::new(random_bools(EXAMPLE_SEED))
}

// -- char --

pub fn random_char_gen(_config: &GenConfig) -> It<char> {
    Box::new(random_chars(EXAMPLE_SEED))
}

#[allow(unstable_name_collisions)]
pub fn random_char_gen_var_1(_config: &GenConfig) -> It<char> {
    Box::new(random_char_range(EXAMPLE_SEED, char::MIN, char::MAX))
}

#[allow(unstable_name_collisions)]
pub fn random_char_gen_var_2(_config: &GenConfig) -> It<char> {
    Box::new(random_char_inclusive_range(
        EXAMPLE_SEED,
        '\u{1}',
        char::MAX,
    ))
}

// -- (char, char) --

pub fn random_char_pair_gen(_config: &GenConfig) -> It<(char, char)> {
    Box::new(random_pairs_from_single(random_chars(EXAMPLE_SEED)))
}

// -- FromSciStringOptions --

pub fn random_from_sci_string_options_gen(_config: &GenConfig) -> It<FromSciStringOptions> {
    Box::new(random_from_sci_string_options(EXAMPLE_SEED))
}

// -- (FromSciStringOptions, PrimitiveUnsigned) --

pub fn random_from_sci_string_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(FromSciStringOptions, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_from_sci_string_options,
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::from(36u8)),
    ))
}

// -- (FromSciStringOptions, RoundingMode) --

pub fn random_from_sci_string_options_rounding_mode_pair_gen(
    _config: &GenConfig,
) -> It<(FromSciStringOptions, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_from_sci_string_options,
        &random_rounding_modes,
    ))
}

// -- PrimitiveFloat --

pub fn random_primitive_float_gen<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(special_random_primitive_floats(
        EXAMPLE_SEED,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
        config.get_or("special_p_mean_n", 1),
        config.get_or("special_p_mean_d", 64),
    ))
}

pub fn random_primitive_float_gen_var_1<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(special_random_primitive_float_range(
        EXAMPLE_SEED,
        T::NEGATIVE_ONE / T::TWO,
        T::INFINITY,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
        config.get_or("special_p_mean_n", 1),
        config.get_or("special_p_mean_d", 64),
    ))
}

struct RandomPositiveNaturalFloats<T: PrimitiveFloat> {
    exponents: GeometricRandomSignedRange<i64>,
    ranges: VariableRangeGenerator,
    phantom: PhantomData<T>,
}

impl<T: PrimitiveFloat> Iterator for RandomPositiveNaturalFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let exponent = self.exponents.next().unwrap();
        let a = if exponent == 0 {
            1
        } else {
            u64::power_of_2(T::MANTISSA_WIDTH)
        };
        let mantissa = self
            .ranges
            .next_in_range(a, u64::power_of_2(T::MANTISSA_WIDTH + 1));
        Some(T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap())
    }
}

fn random_positive_natural_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
) -> RandomPositiveNaturalFloats<T> {
    RandomPositiveNaturalFloats {
        exponents: geometric_random_signed_range(
            seed.fork("exponents"),
            0,
            i64::power_of_2(T::EXPONENT_WIDTH - 1) - i64::wrapping_from(T::MANTISSA_WIDTH) - 1,
            mean_exponent_numerator,
            mean_exponent_denominator,
        ),
        ranges: VariableRangeGenerator::new(seed.fork("mantissas")),
        phantom: PhantomData,
    }
}

pub fn random_primitive_float_gen_var_2<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(with_special_value(EXAMPLE_SEED, T::ZERO, 1, 100, &|seed| {
        random_positive_natural_floats(
            seed,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
        )
    }))
}

pub fn random_primitive_float_gen_var_3<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        special_random_positive_finite_primitive_floats::<T>(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
        )
        .filter(|f| !f.is_integer()),
    )
}

pub fn random_primitive_float_gen_var_4<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        special_random_primitive_float_inclusive_range::<T>(
            EXAMPLE_SEED,
            T::ONE,
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            1,
            100,
        )
        .map(|f| f.floor() - T::ONE / T::TWO),
    )
}

pub fn random_primitive_float_gen_var_5<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                with_special_value(seed, T::ZERO, 1, 100, &|seed_2| {
                    random_positive_natural_floats(
                        seed_2,
                        config.get_or("exponent_mean_n", 8),
                        config.get_or("exponent_mean_d", 1),
                    )
                })
            },
            &random_bools,
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn random_primitive_float_gen_var_6<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_positive_finite_primitive_floats::<T>(
                    seed,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                )
                .filter(|f| !f.is_integer())
            },
            &random_bools,
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn random_primitive_float_gen_var_7<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_primitive_float_inclusive_range::<T>(
                    seed,
                    T::ONE,
                    T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                        .unwrap(),
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    1,
                    100,
                )
                .map(|f| f.floor() - T::ONE / T::TWO)
            },
            &random_bools,
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn random_primitive_float_gen_var_8<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(special_random_finite_primitive_floats(
        EXAMPLE_SEED,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
        config.get_or("special_p_mean_n", 1),
        config.get_or("special_p_mean_d", 64),
    ))
}

pub fn random_primitive_float_gen_var_9<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        special_random_primitive_floats::<T>(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .filter(|&f| !f.is_nan() && f != T::INFINITY),
    )
}

pub fn random_primitive_float_gen_var_10<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        special_random_primitive_floats::<T>(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .filter(|&f| !f.is_nan() && f != T::NEGATIVE_INFINITY),
    )
}

pub fn random_primitive_float_gen_var_11<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        special_random_primitive_floats::<T>(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .filter(|&f| !f.is_nan()),
    )
}

pub fn random_primitive_float_gen_var_12<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(special_random_nonzero_finite_primitive_floats(
        EXAMPLE_SEED,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
    ))
}

pub fn random_primitive_float_gen_var_13<T: PrimitiveFloat + RoundingFrom<U>, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(random_primitive_ints::<U>(EXAMPLE_SEED).map(|n| T::rounding_from(n, Down).0))
}

pub fn random_primitive_float_gen_var_14<
    T: PrimitiveFloat,
    U: ConvertibleFrom<T> + PrimitiveInt,
>(
    config: &GenConfig,
) -> It<T> {
    Box::new(
        special_random_primitive_floats::<T>(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .filter(|&f| !U::convertible_from(f)),
    )
}

pub fn random_primitive_float_gen_var_15<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<T> {
    let limit = min(
        NiceFloat(T::rounding_from(U::MAX, Down).0),
        NiceFloat(
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        ),
    )
    .0;
    Box::new(
        special_random_primitive_float_inclusive_range::<T>(
            EXAMPLE_SEED,
            T::ONE,
            limit,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            1,
            100,
        )
        .map(|f| f.floor() - T::ONE / T::TWO),
    )
}

pub fn random_primitive_float_gen_var_16<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<T> {
    let min_limit = min(
        NiceFloat(-T::rounding_from(U::MIN, Down).0),
        NiceFloat(
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        ),
    )
    .0;
    let max_limit = min(
        NiceFloat(T::rounding_from(U::MAX, Down).0),
        NiceFloat(
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        ),
    )
    .0;
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                special_random_primitive_float_inclusive_range::<T>(
                    seed,
                    T::ONE,
                    max_limit,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    1,
                    100,
                )
                .map(|f| f.floor() - T::ONE / T::TWO)
            },
            &|seed| {
                special_random_primitive_float_inclusive_range::<T>(
                    seed,
                    T::ONE,
                    min_limit,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    1,
                    100,
                )
                .map(|f| T::ONE / T::TWO - f.floor())
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_primitive_float_gen_var_17<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(special_random_positive_finite_primitive_floats::<T>(
        EXAMPLE_SEED,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
    ))
}

pub fn random_primitive_float_gen_var_18<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(special_random_primitive_float_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ZERO,
        T::power_of_2(T::MAX_EXPONENT),
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
        1,
        100,
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat) --

pub fn random_primitive_float_pair_gen<T: PrimitiveFloat>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(special_random_primitive_floats(
        EXAMPLE_SEED,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
        config.get_or("special_p_mean_n", 1),
        config.get_or("special_p_mean_d", 64),
    )))
}

pub fn random_primitive_float_pair_gen_var_1<T: PrimitiveFloat>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(
        special_random_primitive_floats::<T>(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .filter(|&f| !f.is_nan()),
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat, PrimitiveFloat) --

pub fn random_primitive_float_triple_gen<T: PrimitiveFloat>(config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(special_random_primitive_floats(
        EXAMPLE_SEED,
        config.get_or("exponent_mean_n", 8),
        config.get_or("exponent_mean_d", 1),
        config.get_or("precision_mean_n", 8),
        config.get_or("precision_mean_d", 1),
        config.get_or("special_p_mean_n", 1),
        config.get_or("special_p_mean_d", 64),
    )))
}

// -- (PrimitiveFloat, PrimitiveSigned) --

pub fn random_primitive_float_signed_pair_gen<T: PrimitiveFloat, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
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
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
    ))
}

pub fn random_primitive_float_signed_pair_gen_var_1<T: PrimitiveFloat, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            special_random_positive_finite_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
            )
        },
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
    ))
}

pub fn random_primitive_float_signed_pair_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, i64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_primitive_float_range(
                    seed,
                    T::ONE,
                    T::TWO,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    config.get_or("special_p_mean_n", 1),
                    config.get_or("special_p_mean_d", 64),
                )
            },
            &|seed| {
                geometric_random_signed_inclusive_range(
                    seed,
                    T::MIN_EXPONENT,
                    T::MAX_EXPONENT,
                    config.get_or("mean_small_signed_n", 32),
                    config.get_or("mean_small_signed_d", 1),
                )
            },
        )
        .filter(|&(m, e)| m.precision() <= T::max_precision_for_sci_exponent(e)),
    )
}

pub fn random_primitive_float_signed_pair_gen_var_3<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, i64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            special_random_primitive_float_range(
                seed,
                T::ONE,
                T::TWO,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
    ))
}

pub fn random_primitive_float_signed_pair_gen_var_4<T: PrimitiveFloat, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
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
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
    ))
}

// -- (PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned) --

pub fn random_primitive_float_signed_unsigned_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
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
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned) --

pub fn random_primitive_float_unsigned_pair_gen_var_1<T: PrimitiveFloat, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            special_random_positive_finite_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_float_unsigned_pair_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            special_random_primitive_float_range(
                seed,
                T::ONE,
                T::TWO,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_float_unsigned_pair_gen_var_3<T: PrimitiveFloat, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            special_random_positive_finite_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_float_unsigned_pair_gen_var_4<T: PrimitiveFloat, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
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
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

pub fn random_primitive_float_unsigned_rounding_mode_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            special_random_positive_finite_primitive_floats(
                seed,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn random_primitive_float_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            special_random_primitive_float_range(
                seed,
                T::ONE,
                T::TWO,
                config.get_or("exponent_mean_n", 8),
                config.get_or("exponent_mean_d", 1),
                config.get_or("precision_mean_n", 8),
                config.get_or("precision_mean_d", 1),
                config.get_or("special_p_mean_n", 1),
                config.get_or("special_p_mean_d", 64),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// var 3 is in malachite-float.

// -- (PrimitiveFloat, RoundingMode) --

pub fn random_primitive_float_rounding_mode_pair_gen_var_1<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_finite_primitive_floats(
                    seed,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    config.get_or("special_p_mean_n", 1),
                    config.get_or("special_p_mean_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(float_rounding_mode_filter_var_1),
    )
}

pub fn random_primitive_float_rounding_mode_pair_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_finite_primitive_floats::<T>(
                    seed,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    config.get_or("special_p_mean_n", 1),
                    config.get_or("special_p_mean_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(f, rm)| rm != Exact || f.is_integer()),
    )
}

pub fn random_primitive_float_rounding_mode_pair_gen_var_3<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: ConvertibleFrom<T> + PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    let f_min = T::rounding_from(U::MIN, Down).0;
    let f_max = T::rounding_from(U::MAX, Down).0;
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                special_random_primitive_floats::<T>(
                    seed,
                    config.get_or("exponent_mean_n", 8),
                    config.get_or("exponent_mean_d", 1),
                    config.get_or("precision_mean_n", 8),
                    config.get_or("precision_mean_d", 1),
                    config.get_or("special_p_mean_n", 1),
                    config.get_or("special_p_mean_d", 64),
                )
                .filter(|f| !f.is_nan())
            },
            &random_rounding_modes,
        )
        .filter(move |&(f, rm)| match rm {
            Up => f >= f_min && f <= f_max,
            Ceiling => f <= f_max,
            Floor => f >= f_min,
            Down | Nearest => true,
            Exact => U::convertible_from(f),
        }),
    )
}

// -- PrimitiveInt --

pub fn random_primitive_int_gen<T: PrimitiveInt>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_ints(EXAMPLE_SEED))
}

pub fn random_primitive_int_gen_var_1<
    T: PrimitiveInt + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    config: &GenConfig,
) -> It<T> {
    Box::new(
        special_random_primitive_float_range(
            EXAMPLE_SEED,
            U::rounding_from(T::MIN, Down).0,
            U::rounding_from(T::MAX, Down).0,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .map(|f| T::rounding_from(f, Down).0),
    )
}

// -- (PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_pair_gen<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &random_primitive_ints,
    ))
}

pub fn random_primitive_int_pair_gen_var_1<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

// TODO make better
pub fn random_primitive_int_pair_gen_var_2<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(random_primitive_ints(EXAMPLE_SEED)).map(|(x, y)| {
            if x <= y {
                (x, y)
            } else {
                (y, x)
            }
        }),
    )
}

pub fn random_primitive_int_pair_gen_var_3<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_ordered_unique_pairs(random_primitive_ints::<T>(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_triple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED))
            .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

// Returns (highest, second-highest)
pub fn get_two_highest<T: Ord>(xs: &[T]) -> (&T, &T) {
    assert!(xs.len() > 1);
    let (mut hi, mut next_hi) = (&xs[0], &xs[1]);
    if hi < next_hi {
        swap(&mut hi, &mut next_hi);
    }
    for x in &xs[2..] {
        if x > next_hi {
            if x > hi {
                hi = x;
                next_hi = hi;
            } else {
                next_hi = x;
            }
        }
    }
    (hi, next_hi)
}

pub fn random_primitive_int_triple_gen_var_2<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED)).filter_map(
            |(x, y, z)| {
                let ranking = [(x, 0), (y, 1), (z, 2)];
                let (hi, next_hi) = get_two_highest(&ranking);
                if hi.0 == next_hi.0 {
                    None
                } else {
                    Some(match hi.1 {
                        0 => (y, z, x),
                        1 => (x, z, y),
                        _ => (x, y, z),
                    })
                }
            },
        ),
    )
}

pub fn random_primitive_int_triple_gen_var_3<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(EXAMPLE_SEED, &random_primitive_ints, &random_primitive_ints)
            .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
                Equal => None,
                Less => Some((x, y, z)),
                Greater => Some((z, y, x)),
            }),
    )
}

pub fn random_primitive_int_triple_gen_var_4<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_quadruple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_quadruples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED)).filter_map(
            |(x, y, z, w)| {
                let ranking = [(x, 0), (y, 1), (z, 2), (w, 3)];
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
            },
        ),
    )
}

pub fn random_primitive_int_quadruple_gen_var_2<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, U, T)> {
    Box::new(
        random_quadruples_xxyx(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_primitive_ints::<U>,
        )
        .filter_map(|(x, y, z, w)| {
            let ranking = [(x, 0), (y, 1), (w, 2)];
            let (hi, next_hi) = get_two_highest(&ranking);
            if hi.0 == next_hi.0 {
                None
            } else {
                Some(match hi.1 {
                    0 => (y, w, z, x),
                    1 => (x, w, z, y),
                    _ => (x, y, z, w),
                })
            }
        }),
    )
}

pub fn random_primitive_int_quadruple_gen_var_3<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, U, U, T)> {
    Box::new(
        random_quadruples_xyyx(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_primitive_ints::<U>,
        )
        .filter_map(|(x, y, z, w)| match x.cmp(&w) {
            Equal => None,
            Less => Some((x, y, z, w)),
            Greater => Some((w, y, z, x)),
        }),
    )
}

pub fn random_primitive_int_quadruple_gen_var_4<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

pub fn random_primitive_int_quadruple_gen_var_5<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_quadruples_from_single(random_primitive_ints(EXAMPLE_SEED)).filter(
            |&(n1, n0, d1, d0)| {
                // conditions: D >= 2^W, N >= D, and N / D < 2^W
                d1 != T::ZERO && (n1 > d1 || n1 == d1 && n0 >= d0)
            },
        ),
    )
}

// -- (PrimitiveInt * 6) --

pub fn random_primitive_int_sextuple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T, T, T)> {
    Box::new(random_sextuples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveInt * 8) --

#[allow(clippy::type_complexity)]
pub fn random_primitive_int_octuple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T, T, T, T, T)> {
    Box::new(random_octuples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveInt * 9) --

#[allow(clippy::type_complexity)]
pub fn random_primitive_int_nonuple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T, T, T, T, T, T)> {
    Box::new(
        random_triples_from_single(random_triples_from_single(random_primitive_ints(
            EXAMPLE_SEED,
        )))
        .map(|((a, b, c), (d, e, f), (g, h, i))| (a, b, c, d, e, f, g, h, i)),
    )
}

// -- (PrimitiveInt * 12) --

#[allow(clippy::type_complexity)]
pub fn random_primitive_int_duodecuple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T, T, T, T, T, T, T, T, T)> {
    Box::new(random_duodecuples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveUnsigned) --

pub fn random_primitive_int_primitive_int_unsigned_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, T, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_primitive_int_unsigned_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveInt,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_primitive_int_unsigned_triple_gen_var_3<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, T, u64)> {
    Box::new(
        random_triples_xxy(EXAMPLE_SEED, &random_primitive_ints::<T>, &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        })
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveInt, PrimitiveUnsigned) --

pub fn random_primitive_int_primitive_int_primitive_int_unsigned_quadruple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, T, T, U)> {
    Box::new(random_quadruples_xxxy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (PrimitiveInt, PrimitiveSigned) --

pub fn random_primitive_int_signed_pair_gen_var_1<T: PrimitiveInt, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_signed_n", 32),
                config.get_or("mean_small_signed_d", 1),
            )
        },
    ))
}

// -- (PrimitiveInt, PrimitiveSigned, PrimitiveInt) --

pub fn random_primitive_int_signed_primitive_int_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveSigned,
>(
    _config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(EXAMPLE_SEED, &random_primitive_ints, &|seed| {
            random_signed_inclusive_range(
                seed,
                if U::WIDTH <= u64::WIDTH {
                    U::MIN
                } else {
                    -U::exact_from(u64::MAX)
                },
                U::saturating_from(u64::MAX),
            )
        })
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

// -- (PrimitiveInt, PrimitiveSigned, RoundingMode) --

pub fn random_primitive_int_signed_rounding_mode_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &|seed| {
                geometric_random_signeds::<U>(
                    seed,
                    config.get_or("mean_shift_n", 4),
                    config.get_or("mean_shift_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(x, pow, rm)| {
            rm != Exact || pow <= U::ZERO || x.divisible_by_power_of_2(pow.exact_into())
        }),
    )
}

pub fn random_primitive_int_signed_rounding_mode_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &|seed| {
                geometric_random_signeds::<U>(
                    seed,
                    config.get_or("mean_shift_n", 4),
                    config.get_or("mean_shift_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(x, pow, rm)| {
            let pow: i64 = pow.exact_into();
            rm != Exact || pow >= 0 || x.divisible_by_power_of_2(pow.unsigned_abs())
        }),
    )
}

// -- (PrimitiveInt, PrimitiveUnsigned) --

pub fn random_primitive_int_unsigned_pair_gen_var_1<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_2<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigneds_less_than(seed, T::WIDTH),
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_3<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigned_inclusive_range(seed, 1, U::WIDTH),
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_4<
    T: PrimitiveInt,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::saturating_from(T::MAX)),
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_5<
    T: PrimitiveInt,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_6<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_pairs(EXAMPLE_SEED, &random_primitive_ints::<T>, &|seed| {
            geometric_random_unsigneds::<U>(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        })
        .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

pub fn random_primitive_int_unsigned_pair_gen_var_7<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_pairs(EXAMPLE_SEED, &random_primitive_ints::<T>, &|seed| {
            geometric_random_unsigneds::<U>(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        })
        .map(|(mut x, y)| {
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), Down);
            (x, y)
        }),
    )
}

pub fn random_primitive_int_unsigned_pair_gen_var_8<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigneds_less_than(seed, U::WIDTH + 1),
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_9<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_unsigned_pair_gen_var_10<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (PrimitiveInt, PrimitiveUnsigned, bool) --

pub fn random_primitive_int_unsigned_bool_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, u64, bool)> {
    reshape_1_2_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_union2s(
                seed,
                &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                    .map(|x| (x, false))
                },
                &|seed_2| random_unsigneds_less_than(seed_2, T::WIDTH).map(|x| (x, true)),
            )
            .map(Union2::unwrap)
        },
    )))
}

pub fn random_primitive_int_unsigned_bool_triple_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, u64, bool)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(seed, &random_primitive_ints, &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                })
                .map(|(x, y)| (x, y, x < T::ZERO))
            },
            &|seed| {
                random_pairs(seed, &random_primitive_ints, &|seed_2| {
                    random_unsigneds_less_than(seed_2, T::WIDTH)
                })
                .map(|(x, y)| (x, y, x >= T::ZERO))
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_primitive_int_unsigned_bool_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    _config: &GenConfig,
) -> It<(T, U, bool)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_ints::<T>,
        &random_positive_unsigneds::<U>,
        &random_bools,
    ))
}

// -- (PrimitiveInt, PrimitiveUnsigned, PrimitiveInt) --

pub fn random_primitive_int_unsigned_primitive_int_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(EXAMPLE_SEED, &random_primitive_ints, &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        })
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

pub fn random_primitive_int_unsigned_primitive_int_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    _config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(EXAMPLE_SEED, &random_primitive_ints, &|seed| {
            random_unsigned_inclusive_range(seed, U::ZERO, U::saturating_from(u64::MAX))
        })
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

pub fn random_primitive_int_unsigned_primitive_int_triple_gen_var_3<
    T: PrimitiveInt,
    U: PrimitiveInt,
>(
    _config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(EXAMPLE_SEED, &random_primitive_ints, &random_primitive_ints)
            .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
                Equal => None,
                Less => Some((x, y, z)),
                Greater => Some((z, y, x)),
            }),
    )
}

// -- (PrimitiveInt, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_primitive_int_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveInt,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, u64, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigned_inclusive_range(seed, 1, U::WIDTH),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(
        random_triples_xyy(EXAMPLE_SEED, &random_primitive_ints, &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        })
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
}

pub fn random_primitive_int_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveInt,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 4),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// --(PrimitiveInt, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_primitive_int_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, u64, u64, U)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &random_primitive_ints,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_primitive_ints,
        )
        .filter_map(|(x, y, z, w)| {
            let (y, z) = if y <= z { (y, z) } else { (z, y) };
            if unsigned_assign_bits_valid(y, z, w) {
                Some((x, y, z, w))
            } else {
                None
            }
        }),
    )
}

// --(PrimitiveInt, PrimitiveUnsigned, RoundingMode) --

pub fn random_primitive_int_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_ints,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_shift_n", 4),
                    config.get_or("mean_shift_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, pow, rm)| round_to_multiple_of_power_of_2_filter_map(x, pow, rm)),
    )
}

pub fn random_primitive_int_unsigned_rounding_mode_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("mean_shift_n", 4),
                    config.get_or("mean_shift_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(x, y, rm)| rm != Exact || x.divisible_by_power_of_2(y.exact_into())),
    )
}

// --(PrimitiveInt, PrimitiveUnsigned, Vec<bool>) --

struct PrimitiveIntUnsignedBoolVecTripleGeneratorVar1<T: PrimitiveInt> {
    xs: RandomPrimitiveInts<T>,
    log_bases: RandomUnsignedInclusiveRange<u64>,
    bs: RandomBools,
}

impl<T: PrimitiveInt> Iterator for PrimitiveIntUnsignedBoolVecTripleGeneratorVar1<T> {
    type Item = (T, u64, Vec<bool>);

    fn next(&mut self) -> Option<(T, u64, Vec<bool>)> {
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

pub fn random_primitive_int_unsigned_bool_vec_triple_gen_var_1<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, u64, Vec<bool>)> {
    Box::new(PrimitiveIntUnsignedBoolVecTripleGeneratorVar1 {
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        log_bases: random_unsigned_inclusive_range(EXAMPLE_SEED.fork("log_bases"), 1, U::WIDTH),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

// -- (PrimitiveInt, RoundingMode) --

pub fn random_primitive_int_rounding_mode_pair_gen<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &random_rounding_modes,
    ))
}

pub fn random_primitive_int_rounding_mode_pair_gen_var_1<
    T: PrimitiveInt,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(EXAMPLE_SEED, &random_primitive_ints, &random_rounding_modes)
            .filter(move |&(n, rm)| rm != Exact || U::convertible_from(n)),
    )
}

// -- (PrimitiveInt, ToSciOptions) --

pub fn random_primitive_int_to_sci_options_pair_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, ToSciOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_to_sci_options_pair_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, ToSciOptions)> {
    Box::new(
        random_pairs(EXAMPLE_SEED, &random_primitive_ints::<T>, &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        })
        .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- PrimitiveSigned --

pub fn random_signed_gen_var_1<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_ints(EXAMPLE_SEED).filter(|&x| x != T::MIN))
}

pub fn random_signed_gen_var_2<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_natural_signeds(EXAMPLE_SEED))
}

pub fn random_signed_gen_var_3<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_ints(EXAMPLE_SEED).filter(|&x| x != T::ZERO && x != T::NEGATIVE_ONE))
}

pub fn random_signed_gen_var_4<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_negative_signeds(EXAMPLE_SEED))
}

pub fn random_signed_gen_var_5<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(geometric_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_small_signed_n", 32),
        config.get_or("mean_small_signed_d", 1),
    ))
}

pub fn random_signed_gen_var_6<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_nonzero_signeds(EXAMPLE_SEED))
}

pub fn random_signed_gen_var_7<T: PrimitiveSigned, U: ConvertibleFrom<T> + PrimitiveFloat>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_signed_inclusive_range(
                    seed,
                    T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT),
                    T::MAX,
                )
            },
            &|seed| {
                random_signed_inclusive_range(
                    seed,
                    T::MIN,
                    T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT).saturating_neg(),
                )
            },
        )
        .map(Union2::unwrap)
        .filter(|&x| !U::convertible_from(x)),
    )
}

pub fn random_signed_gen_var_8<
    T: TryFrom<NiceFloat<U>> + PrimitiveSigned,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_signed_inclusive_range(
                    seed,
                    T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT),
                    T::MAX,
                )
                .filter_map(|a| {
                    let f = U::rounding_from(a, Down).0;
                    let a = T::try_from(NiceFloat(f)).ok()?;
                    let b = T::try_from(NiceFloat(f.next_higher())).ok()?;
                    let diff = b - a;
                    if diff.even() {
                        // This happens almost always
                        Some(a + (diff >> 1))
                    } else {
                        None
                    }
                })
            },
            &|seed| {
                random_signed_inclusive_range(
                    seed,
                    T::MIN,
                    T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT)
                        .checked_neg()
                        .unwrap(),
                )
                .filter_map(|a| {
                    let f = U::rounding_from(a, Down).0;
                    let a = T::try_from(NiceFloat(f)).ok()?;
                    let b = T::try_from(NiceFloat(f.next_lower())).ok()?;
                    let diff = a - b;
                    if diff.even() {
                        // This happens almost always
                        Some(a - (diff >> 1))
                    } else {
                        None
                    }
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_gen_var_9<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    _config: &GenConfig,
) -> It<S> {
    let limit = S::wrapping_from(U::wrapping_from(S::MAX).floor_sqrt());
    Box::new(random_signed_inclusive_range(EXAMPLE_SEED, -limit, limit))
}

pub fn random_signed_gen_var_10<T: PrimitiveFloat>(_config: &GenConfig) -> It<i64> {
    Box::new(random_signed_inclusive_range(
        EXAMPLE_SEED,
        T::MIN_EXPONENT,
        T::MAX_EXPONENT,
    ))
}

pub fn random_signed_gen_var_11<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_ints(EXAMPLE_SEED).filter(|&x| x != T::ZERO && x != T::MIN))
}

pub fn random_signed_gen_var_12<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_signed_inclusive_range(EXAMPLE_SEED, T::ZERO, T::low_mask(T::WIDTH - 2))
            .map(|u| (u << 1) | T::ONE),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn random_signed_pair_gen_var_1<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_pairs_from_single(random_natural_signeds(seed)),
            &|seed| random_pairs_from_single(random_negative_signeds(seed)),
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_pair_gen_var_2<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_nonzero_signeds::<T>,
        )
        .filter_map(|(mut x, y)| {
            x.round_to_multiple_assign(y, Down);
            if x == T::MIN && y == T::NEGATIVE_ONE {
                None
            } else {
                Some((x, y))
            }
        }),
    )
}

pub fn random_signed_pair_gen_var_3<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_nonzero_signeds::<T>,
        )
        .filter(|&(x, y)| x != T::MIN || y != T::NEGATIVE_ONE),
    )
}

pub fn random_signed_pair_gen_var_4<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_nonzero_signeds::<T>,
        )
        .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn random_signed_pair_gen_var_5<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints::<T>,
        &random_nonzero_signeds::<T>,
    ))
}

pub fn random_signed_pair_gen_var_6<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(random_natural_signeds::<T>(
        EXAMPLE_SEED,
    )))
}

pub fn random_signed_pair_gen_var_7<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_signed_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 2))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn random_signed_pair_gen_var_8<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T)>
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    Box::new(
        random_pairs_from_single(
            random_signed_inclusive_range(EXAMPLE_SEED, T::ZERO, T::low_mask(T::WIDTH - 2))
                .map(|u| (u << 1) | T::ONE),
        )
        .filter(|&(a, b): &(T, T)| a.unsigned_abs().coprime_with(b.unsigned_abs())),
    )
}

pub fn random_signed_pair_gen_var_9<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    _config: &GenConfig,
) -> It<(S, S)> {
    Box::new(
        random_pairs_from_single(random_primitive_ints(EXAMPLE_SEED))
            .filter(|&(x, y): &(S, S)| x.unsigned_abs().coprime_with(y.unsigned_abs())),
    )
}

pub fn random_signed_pair_gen_var_10<T: PrimitiveSigned, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_signed_pair_gen_var_11<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(geometric_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_small_n", 32),
            config.get_or("mean_small_d", 1),
        ))
        .filter(|&(n, k)| T::checked_binomial_coefficient(n, k).is_some()),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

fn halve_bits<T: PrimitiveSigned>(x: T) -> T {
    let half_width = (T::WIDTH >> 1) - 1;
    let half_mask = T::low_mask(half_width);
    if x >= T::ZERO {
        x & half_mask
    } else {
        x | (T::NEGATIVE_ONE << half_width)
    }
}

pub(crate) fn reduce_to_fit_add_mul_signed<T: PrimitiveSigned>(x: T, y: T, z: T) -> (T, T, T) {
    if x.checked_add_mul(y, z).is_some() {
        (x, y, z)
    } else {
        (halve_bits(x), halve_bits(y), halve_bits(z))
    }
}

pub fn random_signed_triple_gen_var_1<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints(EXAMPLE_SEED))
            .map(|(x, y, z)| reduce_to_fit_add_mul_signed(x, y, z)),
    )
}

pub(crate) fn reduce_to_fit_sub_mul_signed<T: PrimitiveSigned>(x: T, y: T, z: T) -> (T, T, T) {
    if x.checked_sub_mul(y, z).is_some() {
        (x, y, z)
    } else {
        (halve_bits(x), halve_bits(y), halve_bits(z))
    }
}

pub fn random_signed_triple_gen_var_2<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints(EXAMPLE_SEED))
            .map(|(x, y, z)| reduce_to_fit_sub_mul_signed(x, y, z)),
    )
}

pub fn random_signed_triple_gen_var_3<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_triples_from_single(random_natural_signeds(seed)),
            &|seed| random_triples_from_single(random_negative_signeds(seed)),
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_triple_gen_var_4<
    U: PrimitiveUnsigned + WrappingFrom<S> + WrappingInto<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    _config: &GenConfig,
) -> It<(S, S, S)> {
    Box::new(
        random_triples_from_single(random_primitive_ints::<S>(EXAMPLE_SEED)).map(|(x, y, m)| {
            if m == S::ZERO {
                let min = min(x, y);
                (min, min, m)
            } else if x <= y {
                let adjusted_diff = U::wrapping_from(y.wrapping_sub(x))
                    .round_to_multiple(m.unsigned_abs(), Down)
                    .0;
                (
                    x,
                    (U::wrapping_from(x).wrapping_add(adjusted_diff)).wrapping_into(),
                    m,
                )
            } else {
                let adjusted_diff = U::wrapping_from(x.wrapping_sub(y))
                    .round_to_multiple(m.unsigned_abs(), Down)
                    .0;
                (
                    (U::wrapping_from(y).wrapping_add(adjusted_diff)).wrapping_into(),
                    y,
                    m,
                )
            }
        }),
    )
}

pub fn random_signed_triple_gen_var_5<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_signed_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 2))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn random_signed_triple_gen_var_6<T: PrimitiveSigned>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_signed_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 2))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

pub fn random_signed_signed_unsigned_triple_gen_var_1<
    U: PrimitiveUnsigned + WrappingFrom<S> + WrappingInto<S>,
    S: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(S, S, u64)> {
    Box::new(
        random_triples_xxy(EXAMPLE_SEED, &random_primitive_ints::<S>, &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_pow_n", 32),
                config.get_or("mean_pow_d", 1),
            )
        })
        .map(|(x, y, pow)| {
            if pow >= S::WIDTH {
                (x, x, pow)
            } else if x <= y {
                let adjusted_diff = U::wrapping_from(y.wrapping_sub(x))
                    .round_to_multiple_of_power_of_2(pow, Down)
                    .0;
                (
                    x,
                    (U::wrapping_from(x).wrapping_add(adjusted_diff)).wrapping_into(),
                    pow,
                )
            } else {
                let adjusted_diff = U::wrapping_from(x.wrapping_sub(y))
                    .round_to_multiple_of_power_of_2(pow, Down)
                    .0;
                (
                    (U::wrapping_from(y).wrapping_add(adjusted_diff)).wrapping_into(),
                    y,
                    pow,
                )
            }
        }),
    )
}

pub fn random_signed_signed_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, RoundingMode) --

struct SignedSignedRoundingModeTripleGenerator<T: PrimitiveSigned> {
    xs: RandomPrimitiveInts<T>,
    rms: RandomRoundingModes,
}

impl<T: PrimitiveSigned> Iterator for SignedSignedRoundingModeTripleGenerator<T> {
    type Item = (T, T, RoundingMode);

    fn next(&mut self) -> Option<(T, T, RoundingMode)> {
        let mut x;
        let mut y;
        loop {
            x = self.xs.next().unwrap();
            loop {
                y = self.xs.next().unwrap();
                if y != T::ZERO {
                    break;
                }
            }
            if x != T::MIN || y != T::NEGATIVE_ONE {
                break;
            }
        }
        let rm = self.rms.next().unwrap();
        if rm == Exact {
            x.round_to_multiple_assign(y, Down);
        }
        Some((x, y, rm))
    }
}

pub fn random_signed_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    _config: &GenConfig,
) -> It<(T, T, RoundingMode)> {
    Box::new(SignedSignedRoundingModeTripleGenerator {
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        rms: random_rounding_modes(EXAMPLE_SEED.fork("rms")),
    })
}

pub fn random_signed_signed_rounding_mode_triple_gen_var_2<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    _config: &GenConfig,
) -> It<(S, S, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_ints,
            &random_nonzero_signeds,
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_signed_filter_map(x, y, rm)),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

pub fn random_signed_unsigned_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(seed, &random_natural_signeds, &|seed_2| {
                    random_unsigned_range(seed_2, 0, T::WIDTH)
                })
            },
            &|seed| {
                random_pairs(seed, &random_negative_signeds, &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_unsigned_pair_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(seed, &random_negative_signeds, &|seed_2| {
                    random_unsigned_range(seed_2, 0, T::WIDTH)
                })
            },
            &|seed| {
                random_pairs(seed, &random_natural_signeds, &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_unsigned_pair_gen_var_3<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_natural_signeds,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_signed_unsigned_pair_gen_var_4<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_natural_signeds,
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
    ))
}

pub fn random_signed_unsigned_pair_gen_var_5<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(seed, &random_natural_signeds, &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                })
            },
            &|seed| {
                random_pairs(seed, &random_primitive_ints, &|seed_2| {
                    random_unsigned_inclusive_range(seed_2, 0, T::WIDTH)
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_unsigned_pair_gen_var_6<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| random_signed_range(seed_2, T::MIN + T::ONE, T::ONE),
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("mean_small_n", 32),
                            config.get_or("mean_small_d", 1),
                        )
                    },
                )
            },
            &|seed| {
                random_pairs(seed, &random_primitive_ints, &|seed_2| {
                    random_unsigned_range(seed_2, 0, T::WIDTH)
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_unsigned_pair_gen_var_7<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_signed_inclusive_range(
                seed,
                if T::WIDTH <= u64::WIDTH {
                    T::MIN
                } else {
                    -T::exact_from(u64::MAX)
                },
                T::saturating_from(u64::MAX),
            )
        },
        &random_positive_unsigneds,
    ))
}

pub fn random_signed_unsigned_pair_gen_var_8<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
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

pub fn random_signed_unsigned_pair_gen_var_9<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_signeds::<T>(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
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
        .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

pub fn random_signed_unsigned_pair_gen_var_10<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_signeds,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_signed_unsigned_pair_gen_var_11<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_signed_range(seed, T::MIN + T::ONE, T::ZERO),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_signed_unsigned_pair_gen_var_12<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(seed, &random_natural_signeds, &|seed_2| {
                    geometric_random_positive_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                })
            },
            &|seed| {
                random_pairs(seed, &random_negative_signeds, &|seed_2| {
                    geometric_random_unsigneds::<U>(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                    .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | U::ONE))
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_signed_unsigned_pair_gen_var_13<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_signed_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_triples_xyy(seed, &random_positive_signeds, &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("mean_small_n", 32),
                        config.get_or("mean_small_d", 1),
                    )
                })
                .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) })
            },
            &|seed| {
                random_triples(
                    seed,
                    &|seed_2| random_signed_range(seed_2, T::MIN, T::ZERO),
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("mean_small_n", 32),
                            config.get_or("mean_small_d", 1),
                        )
                    },
                    &|seed_2| random_unsigned_range(seed_2, U::ZERO, U::exact_from(T::WIDTH)),
                )
                .filter_map(|(x, y, z): (T, U, U)| y.checked_add(z).map(|new_z| (x, y, new_z)))
            },
        )
        .map(Union2::unwrap),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, u64, u64, U)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &random_primitive_ints,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_primitive_ints,
        )
        .filter_map(|(x, y, z, w)| {
            let (y, z) = if y <= z { (y, z) } else { (z, y) };
            if signed_assign_bits_valid(x, y, z, w) {
                Some((x, y, z, w))
            } else {
                None
            }
        }),
    )
}

pub fn random_signed_rounding_mode_pair_gen_var_1<T: PrimitiveSigned>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_nonzero_signeds,
        &random_rounding_modes,
    ))
}

pub fn random_signed_rounding_mode_pair_gen_var_2<T: PrimitiveSigned>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_signed_inclusive_range(seed, T::MIN + T::ONE, T::MAX),
        &random_rounding_modes,
    ))
}

pub fn random_signed_rounding_mode_pair_gen_var_3<T: PrimitiveSigned>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_nonzero_signeds(seed).filter(|&x| x != T::MIN),
        &random_rounding_modes,
    ))
}

pub fn random_signed_rounding_mode_pair_gen_var_4<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// --(PrimitiveSigned, Vec<bool>) --

struct SignedBoolVecPairGeneratorVar1<T: PrimitiveSigned> {
    xs: RandomPrimitiveInts<T>,
    bs: RandomBools,
}

impl<T: PrimitiveSigned> Iterator for SignedBoolVecPairGeneratorVar1<T> {
    type Item = (T, Vec<bool>);

    fn next(&mut self) -> Option<(T, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = (&mut self.bs)
            .take(usize::exact_from(u64::exact_from(x.to_bits_asc().len())))
            .collect();
        Some((x, bs))
    }
}

pub fn random_signed_bool_vec_pair_gen_var_1<T: PrimitiveSigned>(
    _config: &GenConfig,
) -> It<(T, Vec<bool>)> {
    Box::new(SignedBoolVecPairGeneratorVar1 {
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

// -- PrimitiveUnsigned --

pub fn random_unsigned_gen_var_1<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_positive_unsigneds(EXAMPLE_SEED))
}

pub fn random_unsigned_gen_var_2(_config: &GenConfig) -> It<u32> {
    Box::new(random_unsigneds_less_than(EXAMPLE_SEED, NUMBER_OF_CHARS))
}

pub fn random_unsigned_gen_var_3<T: PrimitiveInt>(_config: &GenConfig) -> It<u64> {
    Box::new(random_unsigned_inclusive_range(EXAMPLE_SEED, 1, T::WIDTH))
}

pub fn random_unsigned_gen_var_4<T: PrimitiveInt, U: PrimitiveUnsigned + SaturatingFrom<T>>(
    _config: &GenConfig,
) -> It<U> {
    Box::new(random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        U::TWO,
        U::saturating_from(T::MAX),
    ))
}

pub fn random_unsigned_gen_var_5<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(geometric_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_small_n", 32),
        config.get_or("mean_small_d", 1),
    ))
}

pub fn random_unsigned_gen_var_6<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::TWO,
        T::MAX,
    ))
}

pub fn random_unsigned_gen_var_7<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_unsigneds_less_than(EXAMPLE_SEED, T::exact_from(36)))
}

pub fn random_unsigned_gen_var_8<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::TWO,
        T::exact_from(36),
    ))
}

pub fn random_unsigned_gen_var_9<T: PrimitiveInt>(_config: &GenConfig) -> It<u64> {
    Box::new(random_unsigneds_less_than(EXAMPLE_SEED, T::WIDTH + 1))
}

pub fn random_unsigned_gen_var_10(_config: &GenConfig) -> It<u8> {
    Box::new(
        random_union3s(
            EXAMPLE_SEED,
            &|seed| random_unsigned_inclusive_range(seed, b'0', b'9'),
            &|seed| random_unsigned_inclusive_range(seed, b'a', b'z'),
            &|seed| random_unsigned_inclusive_range(seed, b'A', b'Z'),
        )
        .map(Union3::unwrap),
    )
}

pub fn random_unsigned_gen_var_11<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(geometric_random_positive_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_small_n", 32),
        config.get_or("mean_small_d", 1),
    ))
}

pub fn random_unsigned_gen_var_12<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_highest_bit_set_unsigneds(EXAMPLE_SEED))
}

pub fn random_unsigned_gen_var_13<T: PrimitiveFloat>(_config: &GenConfig) -> It<u64> {
    Box::new(random_unsigneds_less_than(
        EXAMPLE_SEED,
        T::LARGEST_ORDERED_REPRESENTATION,
    ))
}

pub fn random_unsigned_gen_var_14<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_unsigneds_less_than(
        EXAMPLE_SEED,
        T::power_of_2(T::WIDTH - 1) + T::ONE,
    ))
}

pub fn random_unsigned_gen_var_15<T: PrimitiveInt>(_config: &GenConfig) -> It<u64> {
    Box::new(random_unsigneds_less_than(EXAMPLE_SEED, T::WIDTH))
}

pub fn random_unsigned_gen_var_16<T: PrimitiveInt>(_config: &GenConfig) -> It<u64> {
    Box::new(random_unsigneds_less_than(EXAMPLE_SEED, T::WIDTH - 1))
}

pub fn random_unsigned_gen_var_17<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::power_of_2(T::WIDTH - 2),
        T::MAX,
    ))
}

pub fn random_unsigned_gen_var_18<T: PrimitiveUnsigned, U: ConvertibleFrom<T> + PrimitiveFloat>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(
        random_unsigned_inclusive_range(
            EXAMPLE_SEED,
            T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT),
            T::MAX,
        )
        .filter(|&x| !U::convertible_from(x)),
    )
}

pub fn random_unsigned_gen_var_19<
    T: TryFrom<NiceFloat<U>> + PrimitiveUnsigned,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(
        random_unsigned_inclusive_range(
            EXAMPLE_SEED,
            T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT),
            T::MAX,
        )
        .filter_map(|a| {
            let f = U::rounding_from(a, Down).0;
            let a = T::try_from(NiceFloat(f)).ok()?;
            let b = T::try_from(NiceFloat(f.next_higher())).ok()?;
            let diff = b - a;
            if diff.even() {
                // This happens almost always
                Some(a + (diff >> 1))
            } else {
                None
            }
        }),
    )
}

pub fn random_unsigned_gen_var_20<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::ZERO,
        T::MAX.floor_sqrt(),
    ))
}

pub fn random_unsigned_gen_var_21<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_unsigned_inclusive_range(EXAMPLE_SEED, T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    )
}

pub fn random_unsigned_gen_var_22<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_factorial);
    Box::new(random_unsigned_range(EXAMPLE_SEED, 0, limit))
}

pub fn random_unsigned_gen_var_23<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_double_factorial);
    Box::new(random_unsigned_range(EXAMPLE_SEED, 0, limit))
}

pub fn random_unsigned_gen_var_24<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_subfactorial);
    Box::new(random_unsigned_range(EXAMPLE_SEED, 0, limit))
}

pub fn random_unsigned_gen_var_25<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(geometric_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::wrapping_from(5u8),
        T::MAX,
        config.get_or("mean_n", 128),
        config.get_or("mean_d", 1),
    ))
}

pub fn random_unsigned_gen_var_26<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_primorial);
    Box::new(random_unsigned_range(EXAMPLE_SEED, 0, limit))
}

pub fn random_unsigned_gen_var_27<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_product_of_first_n_primes);
    Box::new(random_unsigned_range(EXAMPLE_SEED, 0, limit))
}

// -- (PrimitiveUnsigned, PrimitiveInt) --

pub fn random_unsigned_primitive_int_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range(seed, T::power_of_2(T::WIDTH - 2), T::MAX),
        &random_primitive_ints,
    ))
}

// -- (PrimitiveUnsigned, PrimitiveInt, PrimitiveInt, PrimitiveUnsigned) --

struct ModPowerOf2QuadrupleWithTwoExtraPrimitiveIntsGenerator<T: PrimitiveUnsigned, U: PrimitiveInt>
{
    ms: GeometricRandomNaturalValues<u64>,
    us: RandomPrimitiveInts<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator
    for ModPowerOf2QuadrupleWithTwoExtraPrimitiveIntsGenerator<T, U>
{
    type Item = (T, U, U, u64);

    fn next(&mut self) -> Option<(T, U, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), self.us.next().unwrap(), pow))
    }
}

pub fn random_unsigned_primitive_int_primitive_int_unsigned_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(T, U, U, u64)> {
    Box::new(ModPowerOf2QuadrupleWithTwoExtraPrimitiveIntsGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: random_primitive_ints(EXAMPLE_SEED.fork("us")),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

// -- (PrimitiveUnsigned, PrimitiveInt, PrimitiveUnsigned) --

struct ModPowerOf2TripleWithExtraPrimitiveIntGenerator<T: PrimitiveUnsigned, U: PrimitiveInt> {
    ms: GeometricRandomNaturalValues<u64>,
    us: RandomPrimitiveInts<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator
    for ModPowerOf2TripleWithExtraPrimitiveIntGenerator<T, U>
{
    type Item = (T, U, u64);

    fn next(&mut self) -> Option<(T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), pow))
    }
}

pub fn random_unsigned_primitive_int_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(T, U, u64)> {
    Box::new(ModPowerOf2TripleWithExtraPrimitiveIntGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: random_primitive_ints(EXAMPLE_SEED.fork("us")),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

// -- (PrimitiveUnsigned, PrimitiveSigned) --

struct IntegerMantissaAndExponentGenerator<T: PrimitiveFloat> {
    xs: SpecialRandomNonzeroFiniteFloats<T>,
    shifts: GeometricRandomNaturalValues<i64>,
}

#[inline]
pub(crate) fn shift_integer_mantissa_and_exponent(
    mantissa: u64,
    exponent: i64,
    shift: i64,
) -> Option<(u64, i64)> {
    Some((
        mantissa.arithmetic_checked_shl(shift)?,
        exponent.checked_sub(shift)?,
    ))
}

impl<T: PrimitiveFloat> Iterator for IntegerMantissaAndExponentGenerator<T> {
    type Item = (u64, i64);

    fn next(&mut self) -> Option<(u64, i64)> {
        loop {
            let (mantissa, exponent) = self.xs.next().unwrap().integer_mantissa_and_exponent();
            let shift = self.shifts.next().unwrap();
            let out = shift_integer_mantissa_and_exponent(mantissa, exponent, shift);
            if out.is_some() {
                return out;
            }
        }
    }
}

pub fn random_unsigned_signed_pair_gen_var_1<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(u64, i64)> {
    Box::new(IntegerMantissaAndExponentGenerator::<T> {
        xs: special_random_nonzero_finite_primitive_floats(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
        ),
        shifts: geometric_random_natural_signeds(
            EXAMPLE_SEED.fork("shifts"),
            config.get_or("shift_mean_n", 4),
            config.get_or("shift_mean_d", 1),
        ),
    })
}

// -- (PrimitiveUnsigned, PrimitiveSigned, PrimitiveUnsigned) --

struct ModPowerOf2TripleWithExtraSmallSignedGenerator<T: PrimitiveUnsigned, U: PrimitiveSigned> {
    ms: GeometricRandomNaturalValues<u64>,
    us: GeometricRandomSigneds<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveSigned> Iterator
    for ModPowerOf2TripleWithExtraSmallSignedGenerator<T, U>
{
    type Item = (T, U, u64);

    fn next(&mut self) -> Option<(T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), pow))
    }
}

pub fn random_unsigned_signed_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, u64)> {
    Box::new(ModPowerOf2TripleWithExtraSmallSignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: geometric_random_signeds(
            EXAMPLE_SEED.fork("us"),
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

pub fn random_unsigned_signed_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_unsigned_pair_gen_var_1(_config: &GenConfig) -> It<(u32, u32)> {
    Box::new(random_pairs_from_single(random_unsigneds_less_than(
        EXAMPLE_SEED,
        NUMBER_OF_CHARS,
    )))
}

pub fn random_unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, 1, U::WIDTH),
    ))
}

pub fn random_unsigned_pair_gen_var_3<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
    ))
}

pub fn random_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, V)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::saturating_from(U::MAX)),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_positive_unsigneds::<T>,
        )
        .map(|(x, y)| (x.round_to_multiple(y, Down).0, y)),
    )
}

pub fn random_unsigned_pair_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints::<T>,
        &random_positive_unsigneds::<U>,
    ))
}

pub fn random_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_positive_unsigneds::<T>,
        )
        .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

struct ModPowerOf2SingleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOf2SingleGenerator<T> {
    type Item = (T, u64);

    fn next(&mut self) -> Option<(T, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, pow))
    }
}

struct ModPowerOf2SingleGenerator2<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<Box<dyn Iterator<Item = T>>>>,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOf2SingleGenerator2<T> {
    type Item = (T, u64);

    fn next(&mut self) -> Option<(T, u64)> {
        let pow = self.ms.next().unwrap();
        assert_ne!(pow, 0);
        let xs = &mut self.xss[usize::wrapping_from(pow)];
        if xs.is_none() {
            *xs = Some(Box::new(
                random_unsigned_bit_chunks(EXAMPLE_SEED.fork(&pow.to_string()), pow)
                    .filter(|&x| x != T::ZERO),
            ));
        }
        let x = xs.as_mut().unwrap().next().unwrap();
        Some((x, pow))
    }
}

pub fn random_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, u64)> {
    Box::new(ModPowerOf2SingleGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

pub fn random_unsigned_pair_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_unsigned_pair_gen_var_10<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
    ))
}

pub fn random_unsigned_pair_gen_var_11<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
                .map(|x| (T::ZERO, x))
            },
            &|seed| {
                random_pairs(seed, &random_positive_unsigneds, &|seed_2| {
                    random_unsigneds_less_than(seed_2, T::WIDTH)
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_unsigned_pair_gen_var_12<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_unsigneds,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_unsigned_pair_gen_var_13<T: PrimitiveFloat>(_config: &GenConfig) -> It<(u64, u64)> {
    Box::new(random_pairs_from_single(random_unsigneds_less_than(
        EXAMPLE_SEED,
        T::LARGEST_ORDERED_REPRESENTATION,
    )))
}

pub fn random_unsigned_pair_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_unsigneds,
        &|seed| {
            geometric_random_unsigned_range(
                seed,
                U::TWO,
                U::MAX,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_unsigned_pair_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range(seed, T::ZERO, T::saturating_from(u64::MAX)),
        &random_positive_unsigneds::<U>,
    ))
}

pub fn random_unsigned_pair_gen_var_16<T: PrimitiveFloat>(_config: &GenConfig) -> It<(u64, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_bit_chunks(seed, T::MANTISSA_WIDTH),
        &|seed| random_unsigned_bit_chunks(seed, T::EXPONENT_WIDTH),
    ))
}

pub fn random_unsigned_pair_gen_var_17<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
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

pub fn random_unsigned_pair_gen_var_18<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, u64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                geometric_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
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
        .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

pub fn random_unsigned_pair_gen_var_19<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, u64)> {
    Box::new(random_unsigned_pair_gen_var_8(config).map(|(x, p)| (x, T::WIDTH - p)))
}

pub fn random_unsigned_pair_gen_var_20<T: PrimitiveInt, U: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| random_unsigned_inclusive_range(seed, U::power_of_2(U::WIDTH - 2), U::MAX),
    ))
}

struct LikelyMultiplyablePairs<T: PrimitiveUnsigned> {
    bits: GeometricRandomNaturalValues<u64>,
    ranges: VariableRangeGenerator,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for LikelyMultiplyablePairs<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        let x_bits = self.bits.next().unwrap();
        let x = if x_bits == 0 {
            T::ZERO
        } else {
            self.ranges.next_bit_chunk(x_bits)
        };
        let y_bits = self.bits.next().unwrap();
        let y = if y_bits == 0 {
            T::ZERO
        } else {
            self.ranges.next_bit_chunk(y_bits)
        };
        Some((x, y))
    }
}

pub fn random_unsigned_pair_gen_var_21<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(LikelyMultiplyablePairs {
        bits: geometric_random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("bits"),
            0,
            T::WIDTH,
            T::WIDTH >> 1,
            1,
        ),
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        phantom: PhantomData,
    })
}

pub fn random_unsigned_pair_gen_var_22<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_unsigned_pair_gen_var_21::<T>(config).filter(|&(x, y)| x.checked_lcm(y).is_some()),
    )
}

pub fn random_unsigned_pair_gen_var_23<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range(seed, T::power_of_2(T::WIDTH - 1), T::MAX),
        &random_primitive_ints,
    ))
}

pub fn random_unsigned_pair_gen_var_24<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_positive_unsigneds,
            &random_primitive_ints,
        )
        .filter(|&(x, y)| x != T::ONE || y != T::ZERO),
    )
}

pub fn random_unsigned_pair_gen_var_25<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_unsigneds,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_unsigned_pair_gen_var_26<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_ordered_unique_pairs(random_positive_unsigneds::<T>(
        EXAMPLE_SEED,
    )))
}

pub fn random_unsigned_pair_gen_var_27<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, u64)> {
    Box::new(ModPowerOf2SingleGenerator2 {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            1,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: {
            let len = usize::wrapping_from(T::WIDTH) + 1;
            let mut xss = Vec::with_capacity(len);
            xss.resize_with(len, || None);
            xss
        },
    })
}

pub fn random_unsigned_pair_gen_var_28<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_unsigned_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn random_unsigned_pair_gen_var_29<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(
            random_unsigned_inclusive_range(EXAMPLE_SEED, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE),
        )
        .filter(|&(a, b): &(T, T)| a.coprime_with(b)),
    )
}

pub fn random_unsigned_pair_gen_var_30<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(random_primitive_ints(EXAMPLE_SEED))
            .filter(|&(x, y): &(T, T)| x.coprime_with(y)),
    )
}

struct MultifactorialNGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    ranges: VariableRangeGenerator,
    ms_to_n_limit: HashMap<u64, u64>,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for MultifactorialNGenerator<T> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<(u64, u64)> {
        let m = self.ms.next().unwrap();
        let smallest_invalid_n = self
            .ms_to_n_limit
            .entry(m)
            .or_insert_with(|| smallest_invalid_value(|n| T::checked_multifactorial(n, m)));
        let n = self.ranges.next_less_than(*smallest_invalid_n);
        Some((n, m))
    }
}

pub fn random_unsigned_pair_gen_var_31<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(u64, u64)> {
    Box::new(MultifactorialNGenerator {
        ms: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("mean_m_n", 4),
            config.get_or("mean_m_d", 1),
        ),
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        ms_to_n_limit: HashMap::new(),
        phantom: PhantomData::<*const T>,
    })
}

pub fn random_unsigned_pair_gen_var_32<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(geometric_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_small_n", 32),
            config.get_or("mean_small_d", 1),
        ))
        .filter(|&(n, k)| T::checked_binomial_coefficient(n, k).is_some()),
    )
}

// vars 33 through 37 are in malachite-nz

pub fn random_unsigned_pair_gen_var_38<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::TWO,
        T::MAX,
    )))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveInt, PrimitiveUnsigned) --

struct ModPowerOf2QuadrupleWithExtraPrimitiveIntGenerator<T: PrimitiveUnsigned, U: PrimitiveInt> {
    ms: GeometricRandomNaturalValues<u64>,
    us: RandomPrimitiveInts<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator
    for ModPowerOf2QuadrupleWithExtraPrimitiveIntGenerator<T, U>
{
    type Item = (T, T, U, u64);

    fn next(&mut self) -> Option<(T, T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let (x, y) = if pow == 0 {
            (T::ZERO, T::ZERO)
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            let xs = xs.as_mut().unwrap();
            (xs.next().unwrap(), xs.next().unwrap())
        };
        Some((x, y, self.us.next().unwrap(), pow))
    }
}

pub fn random_unsigned_unsigned_primitive_int_unsigned_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(T, T, U, u64)> {
    Box::new(ModPowerOf2QuadrupleWithExtraPrimitiveIntGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: random_primitive_ints(EXAMPLE_SEED.fork("us")),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

fn wrapping_shr<T: PrimitiveInt>(x: T, bits: u64) -> T {
    if bits >= x.significant_bits() {
        T::ZERO
    } else {
        x >> bits
    }
}

pub(crate) fn reduce_to_fit_add_mul_unsigned<T: PrimitiveUnsigned>(x: T, y: T, z: T) -> (T, T, T) {
    let (p_hi, p_lo) = T::x_mul_y_to_zz(y, z);
    let r_hi = T::xx_add_yy_to_zz(T::ZERO, x, p_hi, p_lo).0;
    if r_hi == T::ZERO {
        (x, y, z)
    } else {
        let excess_x: u64 = r_hi.significant_bits();
        let excess_yz = excess_x.shr_round(1, Ceiling).0;
        (
            wrapping_shr(x, excess_x),
            wrapping_shr(y, excess_yz),
            wrapping_shr(z, excess_yz),
        )
    }
}

pub fn random_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints(EXAMPLE_SEED))
            .map(|(x, y, z)| reduce_to_fit_add_mul_unsigned(x, y, z)),
    )
}

pub(crate) fn reduce_to_fit_sub_mul_unsigned<T: PrimitiveUnsigned>(x: T, y: T, z: T) -> (T, T, T) {
    let x_bits = x.significant_bits();
    let (p_hi, p_lo) = T::x_mul_y_to_zz(y, z);
    let product_bits = if p_hi == T::ZERO {
        p_lo.significant_bits()
    } else {
        p_hi.significant_bits() + T::WIDTH
    };
    if x_bits > product_bits {
        (x, y, z)
    } else {
        let excess = (product_bits - x_bits + 1).shr_round(1, Ceiling).0;
        (x, wrapping_shr(y, excess), wrapping_shr(z, excess))
    }
}

pub fn random_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints(EXAMPLE_SEED))
            .map(|(x, y, z)| reduce_to_fit_sub_mul_unsigned(x, y, z)),
    )
}

pub fn random_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED)).map(|(x, y, m)| {
            if m == T::ZERO {
                let min = min(x, y);
                (min, min, m)
            } else if x <= y {
                let adjusted_diff = (y - x).round_to_multiple(m, Down).0;
                (x, x + adjusted_diff, m)
            } else {
                let adjusted_diff = (x - y).round_to_multiple(m, Down).0;
                (y + adjusted_diff, y, m)
            }
        }),
    )
}

pub fn random_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, u64)> {
    Box::new(
        random_triples_xxy(EXAMPLE_SEED, &random_primitive_ints::<T>, &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_pow_n", 32),
                config.get_or("mean_pow_d", 1),
            )
        })
        .map(|(x, y, pow)| {
            if pow >= T::WIDTH {
                (x, x, pow)
            } else if x <= y {
                let adjusted_diff = (y - x).round_to_multiple_of_power_of_2(pow, Down).0;
                (x, x + adjusted_diff, pow)
            } else {
                let adjusted_diff = (x - y).round_to_multiple_of_power_of_2(pow, Down).0;
                (y + adjusted_diff, y, pow)
            }
        }),
    )
}

struct ModPowerOf2PairGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOf2PairGenerator<T> {
    type Item = (T, T, u64);

    fn next(&mut self) -> Option<(T, T, u64)> {
        let pow = self.ms.next().unwrap();
        let (x, y) = if pow == 0 {
            (T::ZERO, T::ZERO)
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            let xs = xs.as_mut().unwrap();
            (xs.next().unwrap(), xs.next().unwrap())
        };
        Some((x, y, pow))
    }
}

pub fn random_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, u64)> {
    Box::new(ModPowerOf2PairGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

struct ModPowerOf2TripleWithExtraSmallUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
{
    ms: GeometricRandomNaturalValues<u64>,
    us: GeometricRandomNaturalValues<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOf2TripleWithExtraSmallUnsignedGenerator<T, U>
{
    type Item = (T, U, u64);

    fn next(&mut self) -> Option<(T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), pow))
    }
}

pub fn random_unsigned_triple_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, u64)> {
    Box::new(ModPowerOf2TripleWithExtraSmallUnsignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("us"),
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

pub fn random_unsigned_triple_gen_var_7<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(
        random_triples_xyy(EXAMPLE_SEED, &random_positive_unsigneds, &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        })
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
}

pub fn random_unsigned_triple_gen_var_8<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_unsigned_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn random_unsigned_triple_gen_var_9<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_unsigned_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn random_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_unsigned_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

struct ModPowerOf2TripleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOf2TripleGenerator<T> {
    type Item = (T, T, T, u64);

    fn next(&mut self) -> Option<(T, T, T, u64)> {
        let pow = self.ms.next().unwrap();
        let (x, y, z) = if pow == 0 {
            (T::ZERO, T::ZERO, T::ZERO)
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                ));
            }
            let xs = xs.as_mut().unwrap();
            (xs.next().unwrap(), xs.next().unwrap(), xs.next().unwrap())
        };
        Some((x, y, z, pow))
    }
}

pub fn random_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, u64)> {
    Box::new(ModPowerOf2TripleGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
    })
}

pub fn random_unsigned_quadruple_gen_var_2<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &random_primitive_ints,
            &random_positive_unsigneds,
        )
        .map(|(x_1, x_0, d)| {
            let inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
            (x_1, x_0, d, inv)
        }),
    )
}

pub fn random_unsigned_quadruple_gen_var_3<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_xxxy(
        EXAMPLE_SEED,
        &random_primitive_ints,
        &|seed| {
            random_unsigned_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE)
        },
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

struct UnsignedUnsignedRoundingModeTripleGenerator<T: PrimitiveUnsigned> {
    xs: RandomPrimitiveInts<T>,
    rms: RandomRoundingModes,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedUnsignedRoundingModeTripleGenerator<T> {
    type Item = (T, T, RoundingMode);

    fn next(&mut self) -> Option<(T, T, RoundingMode)> {
        let mut x = self.xs.next().unwrap();
        let mut y;
        loop {
            y = self.xs.next().unwrap();
            if y != T::ZERO {
                break;
            }
        }
        let rm = self.rms.next().unwrap();
        if rm == Exact {
            x.round_to_multiple_assign(y, Down);
        }
        Some((x, y, rm))
    }
}

pub fn random_unsigned_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, RoundingMode)> {
    Box::new(UnsignedUnsignedRoundingModeTripleGenerator {
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        rms: random_rounding_modes(EXAMPLE_SEED.fork("rms")),
    })
}

pub fn random_unsigned_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, T, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_ints,
            &random_positive_unsigneds,
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_unsigned_filter_map(x, y, rm)),
    )
}

// var 3 is in malachite-float.

// -- (PrimitiveUnsigned, RoundingMode) --

pub fn random_unsigned_rounding_mode_pair_gen_var_1<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_unsigneds,
        &random_rounding_modes,
    ))
}

pub fn random_unsigned_rounding_mode_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn random_unsigned_rounding_mode_pair_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|&rm| rm != Exact),
    ))
}

pub fn random_unsigned_rounding_mode_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// -- (PrimitiveUnsigned, String) --

struct DigitStringGenerator {
    ranges: VariableRangeGenerator,
    digit_map: HashMap<u8, Vec<char>>,
    digit_counts: GeometricRandomNaturalValues<usize>,
}

impl Iterator for DigitStringGenerator {
    type Item = (u8, String);

    fn next(&mut self) -> Option<(u8, String)> {
        let base = self.ranges.next_in_inclusive_range(2, 36);
        let digits = self
            .digit_map
            .entry(base)
            .or_insert_with(|| valid_digit_chars(base));
        let digit_count = self.digit_counts.next().unwrap();
        let mut s = String::with_capacity(digit_count);
        for _ in 0..digit_count {
            let index = self.ranges.next_less_than(digits.len());
            s.push(digits[index]);
        }
        Some((base, s))
    }
}

pub fn random_unsigned_string_pair_gen_var_1(config: &GenConfig) -> It<(u8, String)> {
    Box::new(DigitStringGenerator {
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        digit_map: HashMap::new(),
        digit_counts: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("digit_counts"),
            1,
            usize::MAX,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
    })
}

pub fn random_unsigned_string_pair_gen_var_2(config: &GenConfig) -> It<(u8, String)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_inclusive_range(seed, 2, 36),
        &|seed| {
            random_strings(
                seed,
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
        },
    ))
}

struct TargetedIntegerFromStringBaseInputs {
    uss: It<(u8, String)>,
    negs: RandomBools,
}

impl Iterator for TargetedIntegerFromStringBaseInputs {
    type Item = (u8, String);

    fn next(&mut self) -> Option<(u8, String)> {
        if self.negs.next().unwrap() {
            let (u, s) = self.uss.next().unwrap();
            let mut out = '-'.to_string();
            out.push_str(&s);
            Some((u, out))
        } else {
            self.uss.next()
        }
    }
}

pub fn random_unsigned_string_pair_gen_var_3(config: &GenConfig) -> It<(u8, String)> {
    Box::new(TargetedIntegerFromStringBaseInputs {
        uss: Box::new(DigitStringGenerator {
            ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
            digit_map: HashMap::new(),
            digit_counts: geometric_random_unsigned_range(
                EXAMPLE_SEED.fork("digit_counts"),
                1,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
        }),
        negs: random_bools(EXAMPLE_SEED.fork("negs")),
    })
}

// --(PrimitiveUnsigned, Vec<bool>) --

struct UnsignedBoolVecPairGeneratorVar1<T: PrimitiveUnsigned> {
    xs: RandomPrimitiveInts<T>,
    bs: RandomBools,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedBoolVecPairGeneratorVar1<T> {
    type Item = (T, Vec<bool>);

    fn next(&mut self) -> Option<(T, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = (&mut self.bs)
            .take(usize::exact_from(x.significant_bits()))
            .collect();
        Some((x, bs))
    }
}

pub fn random_unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>(
    _config: &GenConfig,
) -> It<(T, Vec<bool>)> {
    Box::new(UnsignedBoolVecPairGeneratorVar1 {
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        bs: random_bools(EXAMPLE_SEED.fork("bs")),
    })
}

// -- RationalSequence<PrimitiveInt> --

pub fn random_primitive_int_rational_sequence_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<RationalSequence<T>> {
    Box::new(random_rational_sequences(
        EXAMPLE_SEED,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

// -- (RationalSequence<PrimitiveInt>, PrimitiveUnsigned) --

pub fn random_primitive_int_rational_sequence_unsigned_pair_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rational_sequences(
                seed,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_rational_sequence_unsigned_pair_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, usize)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_rational_sequences(
                    seed,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_unsigned_n", 32),
                    config.get_or("mean_small_unsigned_d", 1),
                )
            },
        )
        .filter(|&(ref xs, i)| {
            if let Some(len) = xs.len() {
                i < len
            } else {
                true
            }
        }),
    )
}

// -- (RationalSequence<PrimitiveInt>, RationalSequence<PrimitiveInt>) --

pub fn random_primitive_int_rational_sequence_pair_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, RationalSequence<T>)> {
    Box::new(random_pairs_from_single(random_rational_sequences(
        EXAMPLE_SEED,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

// -- RationalSequence<PrimitiveInt> * 3 --

pub fn random_primitive_int_rational_sequence_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(
    RationalSequence<T>,
    RationalSequence<T>,
    RationalSequence<T>,
)> {
    Box::new(random_triples_from_single(random_rational_sequences(
        EXAMPLE_SEED,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

// -- RoundingMode --

pub fn random_rounding_mode_gen(_config: &GenConfig) -> It<RoundingMode> {
    Box::new(random_rounding_modes(EXAMPLE_SEED))
}

// -- (RoundingMode, RoundingMode) --

pub fn random_rounding_mode_pair_gen(_config: &GenConfig) -> It<(RoundingMode, RoundingMode)> {
    Box::new(random_pairs_from_single(random_rounding_modes(
        EXAMPLE_SEED,
    )))
}

// -- (RoundingMode, RoundingMode, RoundingMode) --

pub fn random_rounding_mode_triple_gen(
    _config: &GenConfig,
) -> It<(RoundingMode, RoundingMode, RoundingMode)> {
    Box::new(random_triples_from_single(random_rounding_modes(
        EXAMPLE_SEED,
    )))
}

// -- SciSizeOptions --

pub fn random_sci_size_options_gen(config: &GenConfig) -> It<SciSizeOptions> {
    Box::new(random_sci_size_options(
        EXAMPLE_SEED,
        config.get_or("mean_size_n", 32),
        config.get_or("mean_size_d", 1),
    ))
}

// -- String --

pub fn random_string_gen(config: &GenConfig) -> It<String> {
    Box::new(random_strings(
        EXAMPLE_SEED,
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_string_gen_var_1(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &random_ascii_chars,
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_string_gen_var_2(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| random_values_from_vec(seed, ROUNDING_MODE_CHARS.chars().collect()),
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_string_gen_var_3(config: &GenConfig) -> It<String> {
    Box::new(strings_from_char_vecs(random_vecs_min_length(
        EXAMPLE_SEED,
        1,
        &|seed| random_values_from_vec(seed, ('0'..='9').collect()),
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

struct TargetedIntegerFromStrStringsVar1 {
    ss: It<String>,
    negs: RandomBools,
}

impl Iterator for TargetedIntegerFromStrStringsVar1 {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.negs.next().unwrap() {
            Some(format!("-{}", self.ss.next().unwrap()))
        } else {
            self.ss.next()
        }
    }
}

pub fn random_string_gen_var_4(config: &GenConfig) -> It<String> {
    Box::new(TargetedIntegerFromStrStringsVar1 {
        ss: Box::new(strings_from_char_vecs(random_vecs_min_length(
            EXAMPLE_SEED.fork("ss"),
            1,
            &|seed| random_values_from_vec(seed, ('0'..='9').collect()),
            config.get_or("mean_length_n", 32),
            config.get_or("mean_length_d", 1),
        ))),
        negs: random_bools(EXAMPLE_SEED.fork("negs")),
    })
}

pub fn random_string_gen_var_5(config: &GenConfig) -> It<String> {
    Box::new(strings_from_char_vecs(random_vecs_min_length(
        EXAMPLE_SEED,
        1,
        &|seed| random_values_from_vec(seed, ('0'..='1').collect()),
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

pub fn random_string_gen_var_6(config: &GenConfig) -> It<String> {
    Box::new(strings_from_char_vecs(random_vecs_min_length(
        EXAMPLE_SEED,
        1,
        &|seed| random_values_from_vec(seed, ('0'..='7').collect()),
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

pub fn random_string_gen_var_7(config: &GenConfig) -> It<String> {
    Box::new(strings_from_char_vecs(random_vecs_min_length(
        EXAMPLE_SEED,
        1,
        &|seed| {
            random_union3s(
                seed,
                &|seed_2| random_values_from_vec(seed_2, ('0'..='9').collect()),
                &|seed_2| random_values_from_vec(seed_2, ('a'..='f').collect()),
                &|seed_2| random_values_from_vec(seed_2, ('A'..='F').collect()),
            )
            .map(Union3::unwrap)
        },
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

pub fn random_string_gen_var_8(config: &GenConfig) -> It<String> {
    Box::new(
        strings_from_char_vecs(random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &|seed| {
                random_union3s(
                    seed,
                    &|seed_2| random_values_from_vec(seed_2, ('0'..='9').collect()),
                    &|seed_2| random_values_from_vec(seed_2, ('a'..='f').collect()),
                    &|seed_2| random_values_from_vec(seed_2, ('A'..='F').collect()),
                )
                .map(Union3::unwrap)
            },
            config.get_or("mean_length_n", 32),
            config.get_or("mean_length_d", 1),
        ))
        .map(|s| format!("\"0x{s}\"")),
    )
}

struct TargetedIntegerFromStrStringsVar2 {
    ss: It<String>,
    negs: RandomBools,
}

impl Iterator for TargetedIntegerFromStrStringsVar2 {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(if self.negs.next().unwrap() {
            format!("\"-0x{}\"", self.ss.next().unwrap())
        } else {
            format!("\"0x{}\"", self.ss.next().unwrap())
        })
    }
}

pub fn random_string_gen_var_9(config: &GenConfig) -> It<String> {
    Box::new(TargetedIntegerFromStrStringsVar2 {
        ss: Box::new(strings_from_char_vecs(random_vecs_min_length(
            EXAMPLE_SEED.fork("ss"),
            1,
            &|seed| {
                random_union3s(
                    seed,
                    &|seed_2| random_values_from_vec(seed_2, ('0'..='9').collect()),
                    &|seed_2| random_values_from_vec(seed_2, ('a'..='f').collect()),
                    &|seed_2| random_values_from_vec(seed_2, ('A'..='F').collect()),
                )
                .map(Union3::unwrap)
            },
            config.get_or("mean_length_n", 32),
            config.get_or("mean_length_d", 1),
        ))),
        negs: random_bools(EXAMPLE_SEED.fork("negs")),
    })
}

pub fn random_string_gen_var_10(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| random_values_from_vec(seed, PRIMITIVE_FLOAT_CHARS.chars().collect()),
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

// vars 11 through 12 are in malachite-q.

pub fn random_string_gen_var_13(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| random_values_from_vec(seed, DECIMAL_SCI_STRING_CHARS.chars().collect()),
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_string_gen_var_14(config: &GenConfig) -> It<String> {
    Box::new(
        random_strings(
            EXAMPLE_SEED,
            config.get_or("mean_length_n", 32),
            config.get_or("mean_length_d", 1),
        )
        .filter(|s| !large_exponent(s)),
    )
}

pub fn random_string_gen_var_15(config: &GenConfig) -> It<String> {
    Box::new(
        random_strings_using_chars(
            EXAMPLE_SEED,
            &|seed| random_values_from_vec(seed, DECIMAL_SCI_STRING_CHARS.chars().collect()),
            config.get_or("mean_length_n", 32),
            config.get_or("mean_length_d", 1),
        )
        .filter(|s| !large_exponent(s)),
    )
}

// -- (String, FromSciStringOptions) --

pub fn random_string_from_sci_string_options_pair_gen(
    config: &GenConfig,
) -> It<(String, FromSciStringOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_strings(
                seed,
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
        },
        &random_from_sci_string_options,
    ))
}

struct SciDigitStringGenerator {
    options: RandomFromSciStringOptions,
    ranges: VariableRangeGenerator,
    digit_map: HashMap<u8, Vec<char>>,
    digit_counts: GeometricRandomNaturalValues<usize>,
}

impl Iterator for SciDigitStringGenerator {
    type Item = (String, FromSciStringOptions);

    fn next(&mut self) -> Option<(String, FromSciStringOptions)> {
        let options = self.options.next().unwrap();
        let base = options.get_base();
        let digits = self.digit_map.entry(base).or_insert_with(|| {
            let mut cs = vec!['+', '-', '.'];
            if base < 15 {
                cs.push('e');
                cs.push('E');
            }
            cs.extend(valid_digit_chars(base));
            cs
        });
        let digit_count = self.digit_counts.next().unwrap();
        let mut s = String::with_capacity(digit_count);
        for _ in 0..digit_count {
            let index = self.ranges.next_less_than(digits.len());
            s.push(digits[index]);
        }
        Some((s, options))
    }
}

struct SciDigitStringGenerator2 {
    ranges: VariableRangeGenerator,
    digit_map: HashMap<u8, Vec<char>>,
    digit_counts: GeometricRandomNaturalValues<usize>,
}

impl Iterator for SciDigitStringGenerator2 {
    type Item = (String, u8);

    fn next(&mut self) -> Option<(String, u8)> {
        let base = self.ranges.next_in_inclusive_range(2, 36);
        let digits = self.digit_map.entry(base).or_insert_with(|| {
            let mut cs = vec!['+', '-', '.'];
            if base < 15 {
                cs.push('e');
                cs.push('E');
            }
            cs.extend(valid_digit_chars(base));
            cs
        });
        let digit_count = self.digit_counts.next().unwrap();
        let mut s = String::with_capacity(digit_count);
        for _ in 0..digit_count {
            let index = self.ranges.next_less_than(digits.len());
            s.push(digits[index]);
        }
        Some((s, base))
    }
}

pub fn random_string_from_sci_string_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(String, FromSciStringOptions)> {
    Box::new(SciDigitStringGenerator {
        options: random_from_sci_string_options(EXAMPLE_SEED.fork("options")),
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        digit_map: HashMap::new(),
        digit_counts: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("digit_counts"),
            1,
            usize::MAX,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
    })
}

pub fn random_string_from_sci_string_options_pair_gen_var_2(
    config: &GenConfig,
) -> It<(String, FromSciStringOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_strings(
                seed,
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
            .filter(|s| !large_exponent(s))
        },
        &random_from_sci_string_options,
    ))
}

pub fn random_string_from_sci_string_options_pair_gen_var_3(
    config: &GenConfig,
) -> It<(String, FromSciStringOptions)> {
    Box::new(
        SciDigitStringGenerator {
            options: random_from_sci_string_options(EXAMPLE_SEED.fork("options")),
            ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
            digit_map: HashMap::new(),
            digit_counts: geometric_random_unsigned_range(
                EXAMPLE_SEED.fork("digit_counts"),
                1,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
        }
        .filter(|(s, _)| !large_exponent(s)),
    )
}

// -- (String, PrimitiveUnsigned) --

pub fn random_string_unsigned_pair_gen_var_1(config: &GenConfig) -> It<(String, u8)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_strings(
                seed,
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
            .filter(|s| !large_exponent(s))
        },
        &|seed| random_unsigned_inclusive_range(seed, 2, 36),
    ))
}

pub fn random_string_unsigned_pair_gen_var_2(config: &GenConfig) -> It<(String, u8)> {
    Box::new(
        SciDigitStringGenerator2 {
            ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
            digit_map: HashMap::new(),
            digit_counts: geometric_random_unsigned_range(
                EXAMPLE_SEED.fork("digit_counts"),
                1,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
        }
        .filter(|(s, _)| !large_exponent(s)),
    )
}

// -- (String, String) --

pub fn random_string_pair_gen(config: &GenConfig) -> It<(String, String)> {
    Box::new(random_pairs_from_single(random_strings(
        EXAMPLE_SEED,
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

pub fn random_string_pair_gen_var_1(config: &GenConfig) -> It<(String, String)> {
    Box::new(random_pairs_from_single(random_strings_using_chars(
        EXAMPLE_SEED,
        &random_ascii_chars,
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

// -- ToSciOptions --

pub fn random_to_sci_options_gen(config: &GenConfig) -> It<ToSciOptions> {
    Box::new(random_to_sci_options(
        EXAMPLE_SEED,
        config.get_or("mean_size_n", 32),
        config.get_or("mean_size_d", 1),
    ))
}

// -- (ToSciOptions, bool) --

pub fn random_to_sci_options_bool_pair_gen(config: &GenConfig) -> It<(ToSciOptions, bool)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
        &random_bools,
    ))
}

// -- (ToSciOptions, PrimitiveSigned) --

pub fn random_to_sci_options_signed_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(ToSciOptions, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
        &|seed| {
            geometric_random_negative_signeds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (ToSciOptions, PrimitiveUnsigned) --

pub fn random_to_sci_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(ToSciOptions, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::from(36u8)),
    ))
}

pub fn random_to_sci_options_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(ToSciOptions, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
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

pub fn random_to_sci_options_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(ToSciOptions, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (ToSciOptions, RoundingMode) --

pub fn random_to_sci_options_rounding_mode_pair_gen(
    config: &GenConfig,
) -> It<(ToSciOptions, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// -- Vec<bool> --

pub fn random_bool_vec_gen(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(random_vecs(
        EXAMPLE_SEED,
        &random_bools,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_bool_vec_gen_var_1<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_vecs_length_inclusive_range(seed, 0, T::WIDTH, &random_bools),
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| random_vecs_fixed_length_from_single(T::WIDTH, random_bools(seed_2)),
                    &|seed_2| {
                        geometric_random_positive_unsigneds(
                            seed_2,
                            config.get_or("mean_excess_length_n", 32),
                            config.get_or("mean_excess_length_d", 1),
                        )
                    },
                )
                .map(|(bs, n)| bs.into_iter().chain(repeat_n(false, n)).collect())
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_bool_vec_gen_var_2<T: PrimitiveSigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_vecs_length_inclusive_range(seed, 0, T::WIDTH - 1, &random_bools),
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_vecs_fixed_length_from_single(T::WIDTH - 1, random_bools(seed_2))
                    },
                    &|seed_2| {
                        geometric_random_nonzero_signeds::<isize>(
                            seed_2,
                            config.get_or("mean_excess_length_n", 32),
                            config.get_or("mean_excess_length_d", 1),
                        )
                    },
                )
                .map(|(bs, n)| {
                    bs.into_iter()
                        .chain(repeat_n(n < 0, n.unsigned_abs()))
                        .collect()
                })
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_bool_vec_gen_var_3<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_vecs_length_inclusive_range(seed, 0, T::WIDTH, &random_bools),
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| random_vecs_fixed_length_from_single(T::WIDTH, random_bools(seed_2)),
                    &|seed_2| {
                        geometric_random_positive_unsigneds(
                            seed_2,
                            config.get_or("mean_excess_length_n", 32),
                            config.get_or("mean_excess_length_d", 1),
                        )
                    },
                )
                .map(|(bs, n)| repeat_n(false, n).chain(bs).collect())
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_bool_vec_gen_var_4<T: PrimitiveSigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_vecs_length_inclusive_range(seed, 0, T::WIDTH - 1, &random_bools),
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_vecs_fixed_length_from_single(T::WIDTH - 1, random_bools(seed_2))
                    },
                    &|seed_2| {
                        geometric_random_nonzero_signeds::<isize>(
                            seed_2,
                            config.get_or("mean_excess_length_n", 32),
                            config.get_or("mean_excess_length_d", 1),
                        )
                    },
                )
                .map(|(bs, n)| repeat_n(n < 0, n.unsigned_abs()).chain(bs).collect())
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_bool_vec_gen_var_5(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_bools,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|bs| bs.iter().any(|&b| b)),
    )
}

// -- Vec<PrimitiveInt> --

pub fn random_primitive_int_vec_gen<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(random_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_primitive_int_vec_gen_var_1<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| *xs.last().unwrap() != T::ZERO),
    )
}

pub fn random_primitive_int_vec_gen_var_2<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| !slice_test_zero(xs)),
    )
}

pub fn random_primitive_int_vec_gen_var_3<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(
        random_vecs(
            EXAMPLE_SEED,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| xs.last() != Some(&T::ZERO)),
    )
}

pub fn random_primitive_int_vec_gen_var_4<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(random_vecs_min_length(
        EXAMPLE_SEED,
        1,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn random_primitive_int_vec_gen_var_5<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(random_vecs_min_length(
        EXAMPLE_SEED,
        2,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveInt) --

pub fn random_primitive_int_vec_primitive_int_pair_gen<T: PrimitiveInt, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

pub fn random_primitive_int_vec_primitive_int_pair_gen_var_1<T: PrimitiveInt, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
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
        &random_primitive_ints,
    ))
}

pub fn random_primitive_int_vec_primitive_int_pair_gen_var_2<T: PrimitiveInt, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
        },
        &random_primitive_ints,
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveUnsigned) --

struct PrimitiveIntVecUnsignedPairGeneratorVar1<T: PrimitiveInt> {
    xs: GeometricRandomNaturalValues<usize>,
    ys: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt> Iterator for PrimitiveIntVecUnsignedPairGeneratorVar1<T> {
    type Item = (Vec<T>, usize);

    fn next(&mut self) -> Option<(Vec<T>, usize)> {
        let x_1 = self.xs.next().unwrap();
        let x_2 = self.xs.next().unwrap();
        let (len, i) = if x_1 <= x_2 { (x_2, x_1) } else { (x_1, x_2) };
        Some(((&mut self.ys).take(len).collect(), i))
    }
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, usize)> {
    Box::new(PrimitiveIntVecUnsignedPairGeneratorVar1 {
        xs: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        ys: random_primitive_ints(EXAMPLE_SEED.fork("ys")),
    })
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
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
            geometric_random_unsigned_range(
                seed,
                1,
                T::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_3<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
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
            geometric_random_unsigned_inclusive_range(
                seed,
                1,
                T::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_4<
    T: PrimitiveInt,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
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
            random_unsigned_inclusive_range(seed.fork("base"), U::TWO, U::saturating_from(T::MAX))
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_5<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 4),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_6<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            geometric_random_unsigned_inclusive_range(
                seed,
                U::exact_from(3),
                U::MAX,
                config.get_or("mean_small_unsigned_n", 4),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_7<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
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
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_8<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(
        random_pairs(
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
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|(xs, y)| *y < U::exact_from(xs.len() << T::LOG_WIDTH)),
    )
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_9<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
        },
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_10<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
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

pub fn random_primitive_int_vec_unsigned_pair_gen_var_11<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
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
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_12<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
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
            .filter(|xs| !slice_test_zero(xs))
        },
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_13<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
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
        &|seed| random_unsigned_inclusive_range(seed, U::power_of_2(U::WIDTH - 1), U::MAX),
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_14<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
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
        &|seed| random_unsigned_range(seed, U::ONE, U::power_of_2(U::WIDTH - 1)),
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_15<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
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
        &|seed| random_unsigned_inclusive_range(seed, U::power_of_2(U::WIDTH - 1), U::MAX),
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_16<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
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
        &|seed| random_unsigned_range(seed, U::ONE, U::power_of_2(U::WIDTH - 1)),
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_17<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
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
        &|seed| random_unsigned_range(seed, U::ONE, U::power_of_2(U::WIDTH - 2)),
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_18<T: PrimitiveInt, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            geometric_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::MAX,
                config.get_or("mean_small_unsigned_n", 4),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_19<T: PrimitiveInt, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
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
            geometric_random_unsigned_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_pair_gen_var_20<T: PrimitiveInt, U: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
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
        &|seed| {
            geometric_random_unsigned_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveUnsigned, PrimitiveInt) --

pub fn random_primitive_int_vec_unsigned_primitive_int_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
    V: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, V)> {
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
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 4),
                config.get_or("mean_small_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_vec_primitive_int_primitive_int_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveInt, RoundingMode) --

pub fn random_primitive_int_vec_primitive_int_rounding_mode_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, T, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_primitive_ints,
        &random_rounding_modes,
    ))
}

pub fn random_primitive_int_vec_primitive_int_rounding_mode_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 4),
                config.get_or("mean_small_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveUnsigned, PrimitiveUnsigned) --

struct PrimitiveIntVecUnsignedUnsignedTripleGeneratorVar1<T: PrimitiveInt> {
    is: GeometricRandomNaturalValues<usize>,
    xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt> Iterator for PrimitiveIntVecUnsignedUnsignedTripleGeneratorVar1<T> {
    type Item = (Vec<T>, usize, usize);

    fn next(&mut self) -> Option<(Vec<T>, usize, usize)> {
        let i = self.is.next().unwrap();
        let j = self.is.next().unwrap();
        let excess = self.is.next().unwrap();
        let xs = (&mut self.xs).take(i * j + excess).collect();
        Some((xs, i, j))
    }
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, usize, usize)> {
    Box::new(PrimitiveIntVecUnsignedUnsignedTripleGeneratorVar1 {
        is: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("is"),
            config.get_or("mean_small_n", 2),
            config.get_or("mean_small_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U)> {
    Box::new(
        random_triples_xyy(
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
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(xs, y, z)| if y <= z { (xs, y, z) } else { (xs, z, y) }),
    )
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs_min_length(
                    seed,
                    1,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| !slice_test_zero(xs))
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .map(|(xs, y, z)| if y <= z { (xs, y, z) } else { (xs, z, y) }),
    )
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_5<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_primitive_ints,
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_6<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_primitive_int_vec_unsigned_unsigned_triple_gen_var_7<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_primitive_ints,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 32),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>) --

pub fn random_primitive_int_vec_pair_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(random_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

pub struct PrimitiveIntVecPairLenGenerator1<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> Iterator
    for PrimitiveIntVecPairLenGenerator1<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(j).collect(),
        ))
    }
}

fn random_primitive_int_vec_pair_gen_var_1_helper<T: PrimitiveInt>(
    config: &GenConfig,
    seed: Seed,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds(
            seed.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(seed.fork("xs")),
    })
}

pub fn random_primitive_int_vec_pair_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    random_primitive_int_vec_pair_gen_var_1_helper(config, EXAMPLE_SEED)
}

pub fn random_primitive_int_vec_pair_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_pair_gen_var_3<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(random_vecs_min_length(
        EXAMPLE_SEED,
        1,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

pub fn random_primitive_int_vec_pair_gen_var_4<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        random_pairs_from_single(random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter(|(ref xs, ref es)| {
            !xs.is_empty()
                && (es.len() > 1 || es.len() == 1 && es[0] > T::ONE)
                && *es.last().unwrap() != T::ZERO
        }),
    )
}

struct PrimitiveIntVecPairSameLenGenerator<T: PrimitiveInt, I: Iterator<Item = usize>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = usize>> Iterator
    for PrimitiveIntVecPairSameLenGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let len = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(len).collect(),
            (&mut self.xs).take(len).collect(),
        ))
    }
}

pub fn random_primitive_int_vec_pair_gen_var_5<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairSameLenGenerator {
        phantom: PhantomData,
        lengths: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_pair_gen_var_6<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        random_vecs(
            EXAMPLE_SEED,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| xs.last() != Some(&T::ZERO)),
    ))
}

pub fn random_primitive_int_vec_pair_gen_var_7<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| !slice_test_zero(xs)),
    ))
}

pub struct PrimitiveIntVecPairLenGenerator2<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> Iterator
    for PrimitiveIntVecPairLenGenerator2<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        let mut xs;
        loop {
            xs = (&mut self.xs).take(i).collect_vec();
            if !slice_test_zero(&xs) {
                break;
            }
        }
        let mut ys;
        loop {
            ys = (&mut self.xs).take(j).collect_vec();
            if !slice_test_zero(&ys) {
                break;
            }
        }
        Some((xs, ys))
    }
}

pub fn random_primitive_int_vec_pair_gen_var_8<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator2 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

// var 9 is in malachite-nz.

pub fn random_primitive_int_vec_pair_gen_var_10<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter(|(_, ds)| *ds.last().unwrap() != T::ZERO),
    )
}

pub fn random_primitive_int_vec_pair_gen_var_11<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            xs: random_primitive_ints::<T>(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(xs, mut ys)| {
            ys[0] |= T::ONE;
            (xs, ys)
        }),
    )
}

pub fn random_primitive_int_vec_pair_gen_var_12<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
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
        .filter(|(xs, ys)| *xs.last().unwrap() != T::ZERO && *ys.last().unwrap() != T::ZERO),
    )
}

pub fn random_primitive_int_vec_pair_gen_var_13<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecPairLenGenerator1 {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
                EXAMPLE_SEED.fork("lengths"),
                2,
                usize::MAX,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter(|(xs, ys)| *xs.last().unwrap() != T::ZERO && *ys.last().unwrap() != T::ZERO),
    )
}

pub fn random_primitive_int_vec_pair_gen_var_14<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints::<T>,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter_map(|mut xs| {
            let last = xs.last_mut().unwrap();
            *last = last.checked_add(T::ONE)?;
            Some(xs)
        }),
    ))
}

pub fn random_primitive_int_vec_pair_gen_var_15<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("lengths"),
            2,
            usize::MAX,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

// vars 16 through 25 are in malachite-nz.

pub fn random_primitive_int_vec_pair_gen_var_26<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_pair_gen_var_27<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecPairSameLenGenerator {
            phantom: PhantomData,
            lengths: geometric_random_positive_unsigneds(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter(|(ref xs, ref ys): &(Vec<T>, Vec<T>)| {
            (*xs.last().unwrap() != T::ZERO || *ys.last().unwrap() != T::ZERO) && ys[0].odd()
        }),
    )
}

// var 28 is in malachite-nz.

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>, bool) --

pub fn random_primitive_int_vec_primitive_int_vec_bool_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, bool)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| PrimitiveIntVecPairSameLenGenerator {
            phantom: PhantomData,
            lengths: geometric_random_positive_unsigneds(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
            xs: random_primitive_ints(seed.fork("xs")),
        },
        &random_bools,
    )))
}

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>, PrimitiveInt) --

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_1<
    T: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_int_vec_pair_gen_var_1_helper(config, seed),
        &random_primitive_ints,
    )))
}

fn random_primitive_int_vec_pair_gen_var_2_helper<T: PrimitiveInt>(
    config: &GenConfig,
    seed: Seed,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            seed.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(seed.fork("xs")),
    })
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_2<
    T: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_int_vec_pair_gen_var_2_helper(config, seed),
        &random_primitive_ints,
    )))
}

struct PrimitiveIntVecPairLenGenerator3<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> Iterator
    for PrimitiveIntVecPairLenGenerator3<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        let xs = (&mut self.xs).take(i).collect();
        let mut ys;
        loop {
            ys = (&mut self.xs).take(j).collect_vec();
            if !slice_test_zero(&ys) {
                break;
            }
        }
        Some((xs, ys))
    }
}

fn random_primitive_int_vec_pair_gen_var_3_helper<T: PrimitiveInt>(
    config: &GenConfig,
    seed: Seed,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecPairLenGenerator3 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            seed.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        xs: random_primitive_ints(seed.fork("xs")),
    })
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_3<
    T: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_int_vec_pair_gen_var_3_helper(config, seed),
        &random_primitive_ints,
    )))
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_4<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_5<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
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

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_6<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &random_positive_unsigneds,
    ))
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_7<
    T: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| PrimitiveIntVecPairSameLenGenerator {
            phantom: PhantomData,
            lengths: geometric_random_positive_unsigneds(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
            xs: random_primitive_ints(seed.fork("xs")),
        },
        &random_primitive_ints,
    )))
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_8<
    T: PrimitiveInt,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, u64)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_int_vec_pair_gen_var_1_helper(config, seed),
        &|seed| random_unsigned_range(seed, 1, U::WIDTH),
    )))
}

pub fn random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_9<
    T: PrimitiveInt,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, u64)> {
    reshape_2_1_to_3(Box::new(random_pairs(
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
        &|seed| random_unsigned_range(seed, 1, U::WIDTH),
    )))
}

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>, PrimitiveUnsigned) --

pub struct PrimitiveIntVecPairLenAndIndexGenerator<
    T: PrimitiveInt,
    I: Iterator<Item = (usize, usize, usize)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize, usize)>> Iterator
    for PrimitiveIntVecPairLenAndIndexGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>, usize);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, usize)> {
        let (i, j, k) = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(j).collect(),
            k,
        ))
    }
}

pub fn random_primitive_int_vec_primitive_int_vec_unsigned_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, usize)> {
    Box::new(PrimitiveIntVecPairLenAndIndexGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(y)?;
            let o = o.checked_add(x)?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_primitive_int_vec_unsigned_triple_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, usize)> {
    Box::new(PrimitiveIntVecPairLenAndIndexGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, i)| {
            let x = x.checked_add(i)?;
            Some((x, x, i))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>, Vec<PrimitiveInt>) --

pub struct PrimitiveIntVecTripleXYYLenGenerator<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>>
{
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> Iterator
    for PrimitiveIntVecTripleXYYLenGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(j).collect(),
            (&mut self.xs).take(j).collect(),
        ))
    }
}

pub fn random_primitive_int_vec_triple_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
            Some((x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub struct PrimitiveIntVecTripleLenGenerator1<
    T: PrimitiveInt,
    I: Iterator<Item = (usize, usize, usize)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize, usize)>> Iterator
    for PrimitiveIntVecTripleLenGenerator1<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (i, j, k) = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(j).collect(),
            (&mut self.xs).take(k).collect(),
        ))
    }
}

pub struct PrimitiveIntVecQuadrupleLenGenerator1<
    T: PrimitiveInt,
    I: Iterator<Item = (usize, usize, usize, usize)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize, usize, usize)>> Iterator
    for PrimitiveIntVecQuadrupleLenGenerator1<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, k, l) = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(j).collect(),
            (&mut self.xs).take(k).collect(),
            (&mut self.xs).take(l).collect(),
        ))
    }
}

pub fn random_primitive_int_vec_triple_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(y)?;
            let o = x.checked_add(y)?.checked_add(o)?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_3<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(1)?;
            let o = x.checked_add(y)?.checked_add(o)?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

// vars 4 through 23 are in malachite-nz

pub fn random_primitive_int_vec_triple_gen_var_24<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(y)?;
            Some((x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_25<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(2)?;
            let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
            Some((x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_26<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(2)?;
            let x = x.checked_add(y)?;
            Some((x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

struct PrimitiveIntVecTripleXXXLenGenerator<T: PrimitiveInt, I: Iterator<Item = usize>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = usize>> Iterator
    for PrimitiveIntVecTripleXXXLenGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let i = self.lengths.next().unwrap();
        Some((
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(i).collect(),
            (&mut self.xs).take(i).collect(),
        ))
    }
}

pub fn random_primitive_int_vec_triple_gen_var_27<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXXXLenGenerator {
        phantom: PhantomData,
        lengths: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("lengths"),
            2,
            usize::MAX,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_28<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXXXLenGenerator {
        phantom: PhantomData,
        lengths: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_29<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_from_single(
        random_vecs(
            EXAMPLE_SEED,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| xs.last() != Some(&T::ZERO)),
    ))
}

pub fn random_primitive_int_vec_triple_gen_var_30<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let x = x.checked_add(y)?;
            Some((x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_31<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let o = max(x, y).checked_add(o)?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

struct PrimitiveIntVecTripleLenGenerator2<
    T: PrimitiveInt,
    I: Iterator<Item = (usize, usize, usize)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize, usize)>> Iterator
    for PrimitiveIntVecTripleLenGenerator2<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (i, j, k) = self.lengths.next().unwrap();
        let out = (&mut self.xs).take(i).collect();
        let mut xs;
        loop {
            xs = (&mut self.xs).take(j).collect_vec();
            if !slice_test_zero(&xs) {
                break;
            }
        }
        let mut ys;
        loop {
            ys = (&mut self.xs).take(k).collect_vec();
            if !slice_test_zero(&ys) {
                break;
            }
        }
        Some((out, xs, ys))
    }
}

pub fn random_primitive_int_vec_triple_gen_var_32<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator2 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(1)?;
            let y = y.checked_add(1)?;
            let o = o.checked_add(x)?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_33<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator2 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(1)?;
            let y = y.checked_add(1)?;
            let o = o.checked_add(max(x, y))?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_34<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator2 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(1)?;
            let y = y.checked_add(1)?;
            let o = o.checked_add(min(x, y))?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_35<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_from_single(
        random_vecs_min_length(
            EXAMPLE_SEED,
            2,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

// vars 36 through 37 are in malachite-nz.

pub fn random_primitive_int_vec_triple_gen_var_38<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_from_single(
        random_vecs_min_length(
            EXAMPLE_SEED,
            1,
            &random_primitive_ints,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

pub fn random_primitive_int_vec_triple_gen_var_39<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(y)?;
            let o = o.checked_add(x)?;
            Some((o, x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_40<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs_min_length(
                seed,
                1,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            random_vecs_min_length(
                seed,
                2,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
    ))
}

pub fn random_primitive_int_vec_triple_gen_var_41<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(x, y, z)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y)?;
                let z = z.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y, z))
            }),
            xs: random_primitive_ints::<T>(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(x, mut y, z)| {
            y.last_mut().unwrap().set_bit(T::WIDTH - 1);
            (x, y, z)
        }),
    )
}

pub fn random_primitive_int_vec_triple_gen_var_42<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(x, y, z)| {
                let y = y.checked_add(5)?;
                let x = x.checked_add(y)?;
                let z = z.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y, z))
            }),
            xs: random_primitive_ints::<T>(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(x, mut y, z)| {
            y.last_mut().unwrap().set_bit(T::WIDTH - 1);
            (x, y, z)
        }),
    )
}

pub fn random_primitive_int_vec_triple_gen_var_43<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(x, y, z)| {
                let q = x.checked_add(3)?;
                let n = y.checked_add(9)?;
                let d = z.checked_add(6)?;
                if n < d {
                    return None;
                }
                let q_alt = n - d + 1;
                if (q_alt << 1) > n || q_alt > d {
                    return None;
                }
                let n_alt = q_alt << 1;
                let d_alt = q_alt;
                if q >= q_alt && d_alt >= 6 && n_alt >= d_alt + 3 && d_alt >= q_alt {
                    Some((q, n, d))
                } else {
                    None
                }
            }),
            xs: random_primitive_ints::<T>(EXAMPLE_SEED.fork("xs")),
        }
        .map(|(x, y, mut z)| {
            z.last_mut().unwrap().set_bit(T::WIDTH - 1);
            (x, y, z)
        }),
    )
}

pub fn random_primitive_int_vec_triple_gen_var_44<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(r_len, n_len, d_len)| {
                let d_len = d_len.checked_add(2)?;
                let r_len = r_len.checked_add(d_len)?;
                let n_len = n_len.checked_add(d_len)?;
                Some((r_len, n_len, d_len))
            }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter_map(|(r, n, mut d): (Vec<T>, Vec<T>, Vec<T>)| {
            let last_d = d.last_mut().unwrap();
            *last_d = last_d.checked_add(T::ONE)?;
            Some((r, n, d))
        }),
    )
}

// var 45 is in malachite-nz.

pub fn random_primitive_int_vec_triple_gen_var_46<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        PrimitiveIntVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(xs_len, ys_len, zs_len)| {
                let ys_len = ys_len.checked_add(2)?;
                let zs_len = zs_len.checked_add(2)?;
                let xs_len = xs_len.checked_add(ys_len + zs_len - 1)?;
                Some((xs_len, ys_len, zs_len))
            }),
            xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        }
        .filter_map(|(mut xs, mut ys, mut zs): (Vec<T>, Vec<T>, Vec<T>)| {
            let last_x = xs.last_mut().unwrap();
            *last_x = last_x.checked_add(T::ONE)?;
            let last_y = ys.last_mut().unwrap();
            *last_y = last_y.checked_add(T::ONE)?;
            let last_z = zs.last_mut().unwrap();
            *last_z = last_z.checked_add(T::ONE)?;
            Some((xs, ys, zs))
        }),
    )
}

// var 47 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct UnsignedVecUnsignedPairGeneratorVar1<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    log_bases: GeometricRandomNaturalValues<u64>,
    ranges: VariableRangeGenerator,
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for UnsignedVecUnsignedPairGeneratorVar1<T, U>
{
    type Item = (Vec<U>, u64);

    fn next(&mut self) -> Option<(Vec<U>, u64)> {
        let log_base = self.log_bases.next().unwrap();
        let max_count = usize::exact_from(T::WIDTH.div_round(log_base, Ceiling).0);
        loop {
            let digit_count = self.ranges.next_in_inclusive_range(0, max_count);
            let mut digits = Vec::with_capacity(digit_count);
            for _ in 0..digit_count {
                digits.push(self.ranges.next_bit_chunk(log_base));
            }
            if digits_valid::<T, U>(log_base, &digits) {
                return Some((digits, log_base));
            }
        }
    }
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<U>, u64)> {
    Box::new(UnsignedVecUnsignedPairGeneratorVar1::<T, U> {
        log_bases: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("log_bases"),
            1,
            U::WIDTH,
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        ),
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        phantom_t: PhantomData,
        phantom_u: PhantomData,
    })
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<U>, u64)> {
    Box::new(
        random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>(config)
            .map(|(xs, y)| (xs.into_iter().rev().collect(), y)),
    )
}

struct BasecaseDigitsRandomGenerator<
    T: ExactFrom<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
> {
    bases: RandomUnsignedInclusiveRange<U>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    digits: VariableRangeGenerator,
    phantom: PhantomData<*const T>,
}

impl<T: ExactFrom<U> + PrimitiveUnsigned, U: PrimitiveUnsigned + SaturatingFrom<T>> Iterator
    for BasecaseDigitsRandomGenerator<T, U>
{
    type Item = (Vec<T>, U);

    fn next(&mut self) -> Option<(Vec<T>, U)> {
        let base = self.bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let mut digits = Vec::with_capacity(digit_count);
        let t_base = T::exact_from(base);
        for _ in 0..digit_count {
            digits.push(self.digits.next_less_than(t_base));
        }
        Some((digits, base))
    }
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_3<
    T: ExactFrom<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(BasecaseDigitsRandomGenerator {
        bases: random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("bases"),
            U::TWO,
            U::saturating_from(T::MAX),
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_counts"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        digits: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        phantom: PhantomData,
    })
}

struct DigitsDesc<T: PrimitiveUnsigned, U: Digits<T> + PrimitiveUnsigned> {
    ranges: VariableRangeGenerator,
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned + SaturatingFrom<U>, U: Digits<T> + PrimitiveUnsigned> Iterator
    for DigitsDesc<T, U>
{
    type Item = (Vec<T>, T);

    fn next(&mut self) -> Option<(Vec<T>, T)> {
        let base = self
            .ranges
            .next_in_inclusive_range(T::TWO, T::saturating_from(U::MAX));
        let max_digits = U::MAX.to_digits_desc(&base);
        let max_digits_len = max_digits.len();
        loop {
            let digit_count = self.ranges.next_in_inclusive_range(0, max_digits_len);
            let mut ds = Vec::with_capacity(digit_count);
            for _ in 0..digit_count {
                ds.push(self.ranges.next_less_than(base));
            }
            if digit_count < max_digits_len || ds <= max_digits {
                return Some((ds, base));
            }
        }
    }
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    _config: &GenConfig,
) -> It<(Vec<T>, T)> {
    Box::new(DigitsDesc::<T, U> {
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        phantom_t: PhantomData,
        phantom_u: PhantomData,
    })
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_5<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T)> {
    Box::new(
        random_unsigned_vec_unsigned_pair_gen_var_4::<T, U>(config).map(|(mut xs, base)| {
            xs.reverse();
            (xs, base)
        }),
    )
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_vecs(
                seed,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::MAX),
    ))
}

struct PowerOf2DigitsGenerator<T: PrimitiveUnsigned> {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    ranges: VariableRangeGenerator,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for PowerOf2DigitsGenerator<T> {
    type Item = (Vec<T>, u64);

    fn next(&mut self) -> Option<(Vec<T>, u64)> {
        let log_base = self.log_bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let mut digits = Vec::with_capacity(digit_count);
        for _ in 0..digit_count {
            digits.push(self.ranges.next_bit_chunk(log_base));
        }
        Some((digits, log_base))
    }
}

pub fn random_unsigned_vec_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(PowerOf2DigitsGenerator::<T> {
        log_bases: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("log_bases"),
            1,
            T::WIDTH,
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        ),
        digit_counts: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("digit_count"),
            config.get_or("mean_digit_count_n", 4),
            config.get_or("mean_digit_count_d", 1),
        ),
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        phantom: PhantomData,
    })
}

// vars 8 through 11 are in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 2 are in malachite-nz.

pub fn random_unsigned_vec_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T, T)> {
    Box::new(random_triples(
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
        &|seed| {
            random_unsigned_inclusive_range(seed, T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE)
        },
        &random_primitive_ints,
    ))
}

// vars 4 through 6 are in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

struct UnsignedVecSqrtRemGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (usize, usize)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
    hi_n_bits: RandomUnsignedRange<T>,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (usize, usize)>> Iterator
    for UnsignedVecSqrtRemGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (out_len, len) = self.lengths.next().unwrap();
        let out = (&mut self.xs).take(out_len).collect();
        let mut ns: Vec<T> = (&mut self.xs).take(len).collect();
        let n_hi = &mut ns[(usize::exact_from(out_len) << 1) - 1];
        n_hi.mod_power_of_2_assign(T::WIDTH - 2);
        *n_hi |= self.hi_n_bits.next().unwrap() << (T::WIDTH - 2);
        Some((out, ns))
    }
}

pub fn random_unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecSqrtRemGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let out_len = x.checked_add(2)?;
            let len: usize = out_len.arithmetic_checked_shl(1)?;
            let len = len.checked_add(y)?;
            Some((out_len, len))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
        hi_n_bits: random_unsigned_range(EXAMPLE_SEED.fork("hi_n_bits"), T::ONE, T::exact_from(4)),
    })
}

struct UnsignedVecSqrtGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (usize, usize)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (usize, usize)>> Iterator
    for UnsignedVecSqrtGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (out_len, len) = self.lengths.next().unwrap();
        let out = (&mut self.xs).take(out_len).collect();
        let mut ns: Vec<T> = (&mut self.xs).take(len).collect();
        let hi_n = ns.last_mut().unwrap();
        if *hi_n == T::ZERO {
            *hi_n = T::ONE;
        }
        Some((out, ns))
    }
}

pub fn random_unsigned_vec_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecSqrtGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let in_len = x.checked_add(1)?;
            let mut out_len: usize = in_len.shr_round(1, Ceiling).0;
            out_len = out_len.checked_add(y)?;
            Some((out_len, in_len))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

// vars 3 through 4 are in malachite-nz.

pub fn random_unsigned_vec_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
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
            },
            &|seed| random_unsigned_inclusive_range(seed, T::power_of_2(T::WIDTH - 1), T::MAX),
            &random_primitive_ints,
        )
        .map(|(n, d_1, d_0)| (n, vec![d_0, d_1])),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
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
    )))
}

// var 2 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 8 are in malachite-nz.

struct UnsignedVecSqrtRemGenerator3<T: PrimitiveUnsigned, I: Iterator<Item = (usize, usize, usize)>>
{
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (usize, usize, usize)>> Iterator
    for UnsignedVecSqrtRemGenerator3<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (out_len, rs_len, len) = self.lengths.next().unwrap();
        let out = (&mut self.xs).take(out_len).collect();
        let rs = (&mut self.xs).take(rs_len).collect();
        let mut ns: Vec<T> = (&mut self.xs).take(len).collect();
        let hi_n = ns.last_mut().unwrap();
        if *hi_n == T::ZERO {
            *hi_n = T::ONE;
        }
        Some((out, rs, ns))
    }
}

pub fn random_unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecSqrtRemGenerator3 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y, z)| {
            let in_len = x.checked_add(1)?;
            let mut out_len: usize = in_len.shr_round(1, Ceiling).0;
            out_len = out_len.checked_add(y)?;
            let rem_len = in_len.checked_add(z)?;
            Some((out_len, rem_len, in_len))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| PrimitiveIntVecPairLenGenerator1 {
                phantom: PhantomData,
                lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
                    seed.fork("lengths"),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                ))
                .filter(|&(x, y)| x >= y - 2),
                xs: random_primitive_ints(seed.fork("xs")),
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_unsigned_inclusive_range(seed_2, T::power_of_2(T::WIDTH - 1), T::MAX)
                    },
                    &random_primitive_ints,
                )
            },
        )
        .map(|((q, n), (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// var 11 is in malachite-nz.

// -- large types --

pub fn random_large_type_gen_var_1<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_int_vec_pair_gen_var_1_helper(config, seed),
        &|seed| random_pairs_from_single(random_primitive_ints(seed)),
    )))
}

struct UnsignedVecSqrtRemGenerator2<T: PrimitiveUnsigned> {
    pub phantom: PhantomData<*const T>,
    pub lengths: GeometricRandomNaturalValues<usize>,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedVecSqrtRemGenerator2<T> {
    type Item = (Vec<T>, Vec<T>, u64, bool);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, u64, bool)> {
        let len = self.lengths.next().unwrap();
        let n = len.shr_round(1, Ceiling).0;
        let out = (&mut self.xs).take(n).collect();
        let mut ns: Vec<T> = (&mut self.xs).take(len).collect();
        let last = ns.last_mut().unwrap();
        if *last == T::ZERO {
            *last = T::ONE;
        }
        let shift = last.leading_zeros() >> 1;
        Some((out, ns, shift, len.odd()))
    }
}

pub fn random_large_type_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, u64, bool)> {
    Box::new(UnsignedVecSqrtRemGenerator2 {
        phantom: PhantomData,
        lengths: geometric_random_unsigned_range::<usize>(
            EXAMPLE_SEED.fork("lengths"),
            9,
            usize::MAX,
            config.get_or("mean_length_n", 12),
            config.get_or("mean_length_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_large_type_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        random_quadruples_xyyx::<_, _, U, _>(
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
                    config.get_or("mean_small_n", 32),
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

pub fn random_large_type_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints::<T>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| !slice_test_zero(xs))
            },
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("mean_small_n", 32),
                    config.get_or("mean_small_d", 1),
                )
            },
            &|seed| {
                random_vecs(
                    seed,
                    &random_primitive_ints::<T>,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
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

// vars 5 through 8 are in malachite-nz

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_9<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>, bool)> {
    reshape_3_1_to_4(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| PrimitiveIntVecTripleXYYLenGenerator {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_unsigneds::<usize>(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(x, y)| {
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            xs: random_primitive_ints(seed.fork("xs")),
        },
        &random_bools,
    )))
}

// vars 10 through 21 are in malachite-nz.

pub fn random_large_type_gen_var_22<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, usize, T, T)> {
    Box::new(
        random_quadruples_xyzz(
            EXAMPLE_SEED,
            &|seed| {
                random_rational_sequences(
                    seed,
                    &random_primitive_ints,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_small_unsigned_n", 32),
                    config.get_or("mean_small_unsigned_d", 1),
                )
            },
            &random_primitive_ints,
        )
        .filter(|&(ref xs, index, _, _)| {
            if let Some(len) = xs.len() {
                index < len
            } else {
                true
            }
        }),
    )
}

// vars 23 through 26 are in malachite-nz.

pub fn random_large_type_gen_var_27<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(bool, Vec<T>, bool, Vec<T>)> {
    Box::new(random_quadruples_xyxy(
        EXAMPLE_SEED,
        &random_bools,
        &|seed| {
            random_vecs(
                seed,
                &random_primitive_ints,
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| xs.last() != Some(&T::ZERO))
        },
    ))
}
