// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::random::random_bools;
use crate::chars::constants::NUMBER_OF_CHARS;
use crate::chars::random::{
    graphic_weighted_random_ascii_chars, graphic_weighted_random_char_inclusive_range,
    graphic_weighted_random_char_range, graphic_weighted_random_chars,
};
use crate::iterators::{with_special_value, NonzeroValues};
use crate::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, Parity, PowerOf2, ShrRound, UnsignedAbs,
};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::string::options::random::{
    random_from_sci_string_options, random_to_sci_options,
};
use crate::num::conversion::string::options::{FromSciStringOptions, ToSciOptions};
use crate::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, HasHalf, JoinHalves, RoundingFrom, SaturatingFrom, SplitInHalf,
    WrappingFrom, WrappingInto,
};
use crate::num::float::NiceFloat;
use crate::num::logic::traits::{BitAccess, BitBlockAccess, LeadingZeros};
use crate::num::random::geometric::{
    geometric_random_natural_signeds, geometric_random_nonzero_signeds,
    geometric_random_positive_unsigneds, geometric_random_signed_inclusive_range,
    geometric_random_signed_range, geometric_random_signeds,
    geometric_random_unsigned_inclusive_range, geometric_random_unsigned_range,
    geometric_random_unsigneds, GeometricRandomNaturalValues, GeometricRandomSignedRange,
    GeometricRandomSigneds,
};
use crate::num::random::striped::{
    get_striped_bool_vec, get_striped_unsigned_vec, striped_random_bool_vecs,
    striped_random_bool_vecs_length_inclusive_range, striped_random_bool_vecs_min_length,
    striped_random_fixed_length_bool_vecs, striped_random_natural_signeds,
    striped_random_negative_signeds, striped_random_nonzero_signeds,
    striped_random_positive_signeds, striped_random_positive_unsigneds,
    striped_random_signed_inclusive_range, striped_random_signed_range, striped_random_signeds,
    striped_random_unsigned_bit_chunks, striped_random_unsigned_inclusive_range,
    striped_random_unsigned_range, striped_random_unsigned_vecs,
    striped_random_unsigned_vecs_min_length, striped_random_unsigneds, StripedBitSource,
    StripedRandomSigneds, StripedRandomUnsignedBitChunks, StripedRandomUnsignedInclusiveRange,
};
use crate::num::random::{
    random_finite_primitive_floats, random_nonzero_finite_primitive_floats,
    random_positive_finite_primitive_floats, random_primitive_float_inclusive_range,
    random_primitive_float_range, random_primitive_floats, random_unsigneds_less_than,
    RandomPrimitiveFloatInclusiveRange, VariableRangeGenerator,
};
use crate::random::{Seed, EXAMPLE_SEED};
use crate::rational_sequences::random::random_rational_sequences;
use crate::rational_sequences::RationalSequence;
use crate::rounding_modes::random::{random_rounding_modes, RandomRoundingModes};
use crate::rounding_modes::RoundingMode::{self, *};
use crate::slices::slice_test_zero;
use crate::strings::random::random_strings_using_chars;
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
use crate::test_util::generators::{
    digits_valid, float_rounding_mode_filter_var_1, get_two_highest, large_exponent,
    reduce_to_fit_add_mul_signed, reduce_to_fit_add_mul_unsigned, reduce_to_fit_sub_mul_signed,
    reduce_to_fit_sub_mul_unsigned, round_to_multiple_of_power_of_2_filter_map,
    round_to_multiple_signed_filter_map, round_to_multiple_unsigned_filter_map,
    shift_integer_mantissa_and_exponent, signed_assign_bits_valid, smallest_invalid_value,
    unsigned_assign_bits_valid,
};
use crate::test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;
use crate::tuples::random::{random_ordered_unique_pairs, random_pairs, random_pairs_from_single};
use crate::unions::random::random_union2s;
use crate::unions::Union2;
use itertools::repeat_n;
use itertools::Itertools;
use std::cmp::{max, min, Ordering::*};
use std::collections::HashMap;
use std::marker::PhantomData;

// -- char --

pub fn special_random_char_gen(config: &GenConfig) -> It<char> {
    Box::new(graphic_weighted_random_chars(
        EXAMPLE_SEED,
        config.get_or("graphic_char_prob_n", 50),
        config.get_or("graphic_char_prob_d", 51),
    ))
}

#[allow(unstable_name_collisions)]
pub fn special_random_char_gen_var_1(config: &GenConfig) -> It<char> {
    Box::new(graphic_weighted_random_char_range(
        EXAMPLE_SEED,
        char::MIN,
        char::MAX,
        config.get_or("graphic_char_prob_n", 50),
        config.get_or("graphic_char_prob_d", 51),
    ))
}

#[allow(unstable_name_collisions)]
pub fn special_random_char_gen_var_2(config: &GenConfig) -> It<char> {
    Box::new(graphic_weighted_random_char_inclusive_range(
        EXAMPLE_SEED,
        '\u{1}',
        char::MAX,
        config.get_or("graphic_char_prob_n", 50),
        config.get_or("graphic_char_prob_d", 51),
    ))
}

// -- (char, char) --

pub fn special_random_char_pair_gen(config: &GenConfig) -> It<(char, char)> {
    Box::new(random_pairs_from_single(graphic_weighted_random_chars(
        EXAMPLE_SEED,
        config.get_or("graphic_char_prob_n", 50),
        config.get_or("graphic_char_prob_d", 51),
    )))
}

// -- (FromSciStringOptions, PrimitiveUnsigned> --

pub fn special_random_from_sci_string_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(FromSciStringOptions, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_from_sci_string_options,
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

// -- PrimitiveFloat --

pub fn special_random_primitive_float_gen<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_floats(EXAMPLE_SEED))
}

pub fn special_random_primitive_float_gen_var_1<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_float_range(
        EXAMPLE_SEED,
        T::NEGATIVE_ONE / T::TWO,
        T::INFINITY,
    ))
}

struct SpecialRandomPositiveNaturalFloats<T: PrimitiveFloat> {
    exponents: GeometricRandomSignedRange<i64>,
    mantissas: StripedRandomUnsignedBitChunks<u64>,
    phantom: PhantomData<T>,
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomPositiveNaturalFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let exponent = self.exponents.next().unwrap();
        let mut mantissa = self.mantissas.next().unwrap();
        if exponent != 0 {
            mantissa.set_bit(T::MANTISSA_WIDTH);
        } else if mantissa == 0 {
            mantissa = 1;
        }
        Some(T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap())
    }
}

fn special_random_positive_natural_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> SpecialRandomPositiveNaturalFloats<T> {
    SpecialRandomPositiveNaturalFloats {
        exponents: geometric_random_signed_range(
            seed.fork("exponents"),
            0,
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

pub fn special_random_primitive_float_gen_var_2<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(with_special_value(EXAMPLE_SEED, T::ZERO, 1, 100, &|seed| {
        special_random_positive_natural_floats(
            seed,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("mean_stripe_n", 16),
            config.get_or("mean_stripe_d", 1),
        )
    }))
}

pub fn special_random_primitive_float_gen_var_3<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_positive_finite_primitive_floats::<T>(EXAMPLE_SEED).filter(|f| !f.is_integer()))
}

pub fn special_random_primitive_float_gen_var_4<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_primitive_float_inclusive_range::<T>(
            EXAMPLE_SEED,
            T::ONE,
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        )
        .map(|f| f.floor() - T::ONE / T::TWO),
    )
}

pub fn special_random_primitive_float_gen_var_5<T: PrimitiveFloat>(config: &GenConfig) -> It<T> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                with_special_value(seed, T::ZERO, 1, 100, &|seed_2| {
                    special_random_positive_natural_floats(
                        seed_2,
                        config.get_or("exponent_mean_n", 8),
                        config.get_or("exponent_mean_d", 1),
                        config.get_or("mean_stripe_n", 16),
                        config.get_or("mean_stripe_d", 1),
                    )
                })
            },
            &random_bools,
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn special_random_primitive_float_gen_var_6<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| random_positive_finite_primitive_floats::<T>(seed).filter(|f| !f.is_integer()),
            &random_bools,
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn special_random_primitive_float_gen_var_7<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_primitive_float_inclusive_range::<T>(
                    seed,
                    T::ONE,
                    T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                        .unwrap(),
                )
                .map(|f| f.floor() - T::ONE / T::TWO)
            },
            &random_bools,
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn special_random_primitive_float_gen_var_8<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_finite_primitive_floats(EXAMPLE_SEED))
}

pub fn special_random_primitive_float_gen_var_9<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_primitive_floats::<T>(EXAMPLE_SEED).filter(|&f| !f.is_nan() && f != T::INFINITY),
    )
}

pub fn special_random_primitive_float_gen_var_10<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(
        random_primitive_floats::<T>(EXAMPLE_SEED)
            .filter(|&f| !f.is_nan() && f != T::NEGATIVE_INFINITY),
    )
}

pub fn special_random_primitive_float_gen_var_11<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_floats::<T>(EXAMPLE_SEED).filter(|&f| !f.is_nan()))
}

pub fn special_random_primitive_float_gen_var_12<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_nonzero_finite_primitive_floats(EXAMPLE_SEED))
}

pub fn special_random_primitive_float_gen_var_13<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<T> {
    Box::new(
        striped_random_unsigneds::<U>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", U::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(|n| T::rounding_from(n, Down).0),
    )
}

pub fn special_random_primitive_float_gen_var_14<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<T> {
    Box::new(
        striped_random_signeds::<U>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", U::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(|n| T::rounding_from(n, Down).0),
    )
}

pub fn special_random_primitive_float_gen_var_15<
    T: PrimitiveFloat,
    U: ConvertibleFrom<T> + PrimitiveInt,
>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(random_primitive_floats::<T>(EXAMPLE_SEED).filter(|&f| !U::convertible_from(f)))
}

pub fn special_random_primitive_float_gen_var_16<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned,
>(
    _config: &GenConfig,
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
        random_primitive_float_inclusive_range::<T>(EXAMPLE_SEED, T::ONE, limit)
            .map(|f| f.floor() - T::ONE / T::TWO),
    )
}

pub fn special_random_primitive_float_gen_var_17<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned,
>(
    _config: &GenConfig,
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
                random_primitive_float_inclusive_range::<T>(seed, T::ONE, max_limit)
                    .map(|f| f.floor() - T::ONE / T::TWO)
            },
            &|seed| {
                random_primitive_float_inclusive_range::<T>(seed, T::ONE, min_limit)
                    .map(|f| T::ONE / T::TWO - f.floor())
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_primitive_float_gen_var_18<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_positive_finite_primitive_floats::<T>(EXAMPLE_SEED))
}

pub fn special_random_primitive_float_gen_var_19<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_float_inclusive_range(
        EXAMPLE_SEED,
        T::ZERO,
        T::power_of_2(T::MAX_EXPONENT),
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat) --

pub fn special_random_primitive_float_pair_gen<T: PrimitiveFloat>(
    _config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_pairs_from_single(random_primitive_floats(
        EXAMPLE_SEED,
    )))
}

pub fn special_random_primitive_float_pair_gen_var_1<T: PrimitiveFloat>(
    _config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_pairs_from_single(
        random_primitive_floats::<T>(EXAMPLE_SEED).filter(|&f| !f.is_nan()),
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat, PrimitiveFloat) --

pub fn special_random_primitive_float_triple_gen<T: PrimitiveFloat>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(random_primitive_floats(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveFloat, PrimitiveSigned) --

pub fn special_random_primitive_float_signed_pair_gen<T: PrimitiveFloat, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_floats,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_primitive_float_signed_pair_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_finite_primitive_floats,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_primitive_float_signed_pair_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, i64)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| random_primitive_float_range(seed, T::ONE, T::TWO),
            &|seed| {
                geometric_random_signed_inclusive_range(
                    seed,
                    T::MIN_EXPONENT,
                    T::MAX_EXPONENT,
                    config.get_or("small_signed_mean_n", 32),
                    config.get_or("small_signed_mean_d", 1),
                )
            },
        )
        .filter(|&(m, e)| m.precision() <= T::max_precision_for_sci_exponent(e)),
    )
}

pub fn special_random_primitive_float_signed_pair_gen_var_3<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, i64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_float_range(seed, T::ONE, T::TWO),
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_primitive_float_signed_pair_gen_var_4<
    T: PrimitiveFloat,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_floats,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
    ))
}

// -- (PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned) --

pub fn special_random_primitive_float_signed_unsigned_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_floats,
        &|seed| {
            geometric_random_signeds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned) --

pub fn special_random_primitive_float_unsigned_pair_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_finite_primitive_floats,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_primitive_float_unsigned_pair_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| random_primitive_float_range(seed, T::ONE, T::TWO),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_primitive_float_unsigned_pair_gen_var_3<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_positive_finite_primitive_floats,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_primitive_float_unsigned_pair_gen_var_4<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &random_primitive_floats,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_positive_finite_primitive_floats,
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| random_primitive_float_range(seed, T::ONE, T::TWO),
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

// var 3 is in malachite-float.

// -- (PrimitiveFloat, RoundingMode) --

pub fn special_random_primitive_float_rounding_mode_pair_gen_var_1<T: PrimitiveFloat>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_finite_primitive_floats,
            &random_rounding_modes,
        )
        .filter(float_rounding_mode_filter_var_1),
    )
}

pub fn special_random_primitive_float_rounding_mode_pair_gen_var_2<T: PrimitiveFloat>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_floats::<T>,
            &random_rounding_modes,
        )
        .filter(|&(f, rm)| rm != Exact || f.is_integer()),
    )
}

pub fn special_random_primitive_float_rounding_mode_pair_gen_var_3<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: ConvertibleFrom<T> + PrimitiveInt,
>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    let f_min = T::rounding_from(U::MIN, Down).0;
    let f_max = T::rounding_from(U::MAX, Down).0;
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| random_primitive_floats::<T>(seed).filter(|f| !f.is_nan()),
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

pub fn special_random_primitive_int_gen_var_1<
    T: PrimitiveInt + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    _config: &GenConfig,
) -> It<T> {
    Box::new(
        random_primitive_float_range(
            EXAMPLE_SEED,
            U::rounding_from(T::MIN, Down).0,
            U::rounding_from(T::MAX, Down).0,
        )
        .map(|f| T::rounding_from(f, Down).0),
    )
}

// -- PrimitiveSigned --

pub fn special_random_signed_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_signed_gen_var_1<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(special_random_signed_gen(config).filter(|&x| x != T::MIN))
}

pub fn special_random_signed_gen_var_2<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_natural_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_signed_gen_var_3<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(
        striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .filter(|&x| x != T::ZERO && x != T::NEGATIVE_ONE),
    )
}

pub fn special_random_signed_gen_var_4<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_negative_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_signed_gen_var_5<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_nonzero_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_signed_gen_var_6<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: ConvertibleFrom<S> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<S> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signed_inclusive_range::<U, S>(
                    seed,
                    S::saturating_from(V::SMALLEST_UNREPRESENTABLE_UINT),
                    S::MAX,
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_signed_inclusive_range::<U, S>(
                    seed,
                    S::MIN,
                    S::saturating_from(V::SMALLEST_UNREPRESENTABLE_UINT).saturating_neg(),
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(Union2::unwrap)
        .filter(|&x| !V::convertible_from(x)),
    )
}

pub fn special_random_signed_gen_var_7<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: TryFrom<NiceFloat<V>> + PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveFloat + RoundingFrom<S>,
>(
    config: &GenConfig,
) -> It<S> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signed_inclusive_range::<U, S>(
                    seed,
                    S::exact_from(V::SMALLEST_UNREPRESENTABLE_UINT),
                    S::MAX,
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
                .filter_map(|a| {
                    let f = V::rounding_from(a, Down).0;
                    let a = S::try_from(NiceFloat(f)).ok()?;
                    let b = S::try_from(NiceFloat(f.next_higher())).ok()?;
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
                striped_random_signed_inclusive_range::<U, S>(
                    seed,
                    S::MIN,
                    S::exact_from(V::SMALLEST_UNREPRESENTABLE_UINT)
                        .checked_neg()
                        .unwrap(),
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
                .filter_map(|a| {
                    let f = V::rounding_from(a, Down).0;
                    let a = S::try_from(NiceFloat(f)).ok()?;
                    let b = S::try_from(NiceFloat(f.next_lower())).ok()?;
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

pub fn special_random_signed_gen_var_8<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<S> {
    let limit = S::wrapping_from(U::wrapping_from(S::MAX).floor_sqrt());
    Box::new(striped_random_signed_inclusive_range::<U, S>(
        EXAMPLE_SEED,
        -limit,
        limit,
        config.get_or("mean_stripe_n", S::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_signed_gen_var_9<T: PrimitiveFloat>(config: &GenConfig) -> It<i64> {
    Box::new(striped_random_signed_inclusive_range::<u64, i64>(
        EXAMPLE_SEED,
        T::MIN_EXPONENT,
        T::MAX_EXPONENT,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_signed_gen_var_10<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(
        striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .filter(|&x| x != T::ZERO && x != T::MIN),
    )
}

pub fn special_random_signed_gen_var_11<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<S> {
    Box::new(
        striped_random_signed_inclusive_range::<U, S>(
            EXAMPLE_SEED,
            S::ZERO,
            S::low_mask(S::WIDTH - 2),
            config.get_or("mean_stripe_n", S::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(|u| (u << 1) | S::ONE),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_signed_pair_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_signed_pair_gen_var_1<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_natural_signeds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ))
            },
            &|seed| {
                random_pairs_from_single(striped_random_negative_signeds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ))
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_pair_gen_var_2<T: PrimitiveSigned, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_signed_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_signed_stripe_d", 1),
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

pub fn special_random_signed_pair_gen_var_3<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_nonzero_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

pub fn special_random_signed_pair_gen_var_4<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_nonzero_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(|&(x, y)| x != T::MIN || y != T::NEGATIVE_ONE),
    )
}

pub fn special_random_signed_pair_gen_var_5<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_nonzero_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn special_random_signed_pair_gen_var_6<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_nonzero_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_signed_pair_gen_var_7<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_natural_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_signed_pair_gen_var_8<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<(S, S)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_signed_inclusive_range::<U, S>(
                seed,
                S::ZERO,
                S::low_mask(S::WIDTH - 2),
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | S::ONE)
        },
    ))
}

pub fn special_random_signed_pair_gen_var_9<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<(S, S)> {
    Box::new(
        random_pairs_from_single(
            striped_random_signed_inclusive_range::<U, S>(
                EXAMPLE_SEED,
                S::ZERO,
                S::low_mask(S::WIDTH - 2),
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | S::ONE),
        )
        .filter(|&(a, b): &(S, S)| a.unsigned_abs().coprime_with(b.unsigned_abs())),
    )
}

pub fn special_random_signed_pair_gen_var_10<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    config: &GenConfig,
) -> It<(S, S)> {
    Box::new(
        random_pairs_from_single(striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", S::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter(|&(x, y): &(S, S)| x.unsigned_abs().coprime_with(y.unsigned_abs())),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_signed_triple_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_signed_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y, z)| reduce_to_fit_add_mul_signed(x, y, z)),
    )
}

pub fn special_random_signed_triple_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y, z)| reduce_to_fit_sub_mul_signed(x, y, z)),
    )
}

pub fn special_random_signed_triple_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_triples_from_single(striped_random_natural_signeds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ))
            },
            &|seed| {
                random_triples_from_single(striped_random_negative_signeds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ))
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_triple_gen_var_4<
    U: PrimitiveUnsigned + WrappingFrom<S> + WrappingInto<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    config: &GenConfig,
) -> It<(S, S, S)> {
    Box::new(
        random_triples_from_single(striped_random_signeds::<S>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", S::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y, m)| {
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

pub fn special_random_signed_triple_gen_var_5<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_signeds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

pub fn special_random_signed_triple_gen_var_6<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<(S, S, S)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_signed_inclusive_range::<U, S>(
                seed,
                S::ZERO,
                S::low_mask(S::WIDTH - 2),
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | S::ONE)
        },
    ))
}

pub fn special_random_signed_triple_gen_var_7<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<(S, S, S)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_signed_inclusive_range::<U, S>(
                seed,
                S::ZERO,
                S::low_mask(S::WIDTH - 2),
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | S::ONE)
        },
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_signed_quadruple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_from_single(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

pub fn special_random_signed_signed_signed_unsigned_quadruple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, T, T, U)> {
    Box::new(random_quadruples_xxxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

pub fn special_random_signed_signed_unsigned_triple_gen_var_1<
    U: PrimitiveUnsigned + WrappingFrom<S> + WrappingInto<S>,
    S: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(S, S, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<S>(
                    seed,
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_pow_n", 32),
                    config.get_or("mean_pow_d", 1),
                )
            },
        )
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

pub fn special_random_signed_signed_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_stripe_d", 1),
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

pub fn special_random_signed_signed_unsigned_triple_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
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
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

pub fn special_random_signed_signed_unsigned_triple_gen_var_4<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
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
    xs: StripedRandomSigneds<T>,
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

pub fn special_random_signed_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, T, RoundingMode)> {
    Box::new(SignedSignedRoundingModeTripleGenerator {
        xs: striped_random_signeds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        rms: random_rounding_modes(EXAMPLE_SEED.fork("rms")),
    })
}

pub fn special_random_signed_signed_rounding_mode_triple_gen_var_2<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    config: &GenConfig,
) -> It<(S, S, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds(
                    seed,
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_nonzero_signeds(
                    seed,
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_signed_filter_map(x, y, rm)),
    )
}

pub fn special_random_signed_signed_rounding_mode_triple_gen_var_3<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

pub fn special_random_signed_signed_rounding_mode_triple_gen_var_4<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

// -- (PrimitiveSigned, PrimitiveUnsigned) --

pub fn special_random_signed_unsigned_pair_gen<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_signed_unsigned_pair_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_signed_unsigned_pair_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| random_unsigneds_less_than(seed, T::WIDTH),
    ))
}

pub fn special_random_signed_unsigned_pair_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed| {
                        striped_random_natural_signeds(
                            seed,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_unsigned_range(
                            seed_2,
                            0,
                            T::WIDTH,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed| {
                        striped_random_negative_signeds(
                            seed,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_4<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_negative_signeds(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_unsigned_range(
                            seed_2,
                            0,
                            T::WIDTH,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_natural_signeds(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_5<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_signed_unsigned_pair_gen_var_6<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_natural_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_signed_unsigned_pair_gen_var_7<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_natural_signeds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_signed_unsigned_pair_gen_var_8<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_large_signed_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_signed_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_9<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(mut x, y)| {
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), Down);
            (x, y)
        }),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_10<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_natural_signeds::<T>(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
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
                        striped_random_signeds::<T>(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_unsigned_inclusive_range(
                            seed_2,
                            0,
                            T::WIDTH,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_11<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<(S, u64)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_signed_range::<U, S>(
                            seed_2,
                            S::MIN + S::ONE,
                            S::ONE,
                            config.get_or("mean_stripe_n", S::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
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
                        striped_random_signeds::<S>(
                            seed_2,
                            config.get_or("mean_large_unsigned_stripe_n", S::WIDTH >> 1),
                            config.get_or("mean_large_unsigned_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_unsigned_range(
                            seed_2,
                            0,
                            S::WIDTH,
                            config.get_or("mean_stripe_n", S::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_12<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(S, V)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signed_inclusive_range::<U, S>(
                seed,
                if S::WIDTH <= u64::WIDTH {
                    S::MIN
                } else {
                    -S::exact_from(u64::MAX)
                },
                S::saturating_from(u64::MAX),
                config.get_or("mean_stripe_n", S::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_signed_unsigned_pair_gen_var_13<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_signed_unsigned_pair_gen_var_14<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed| {
                        striped_random_natural_signeds(
                            seed,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
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
                    &|seed| {
                        striped_random_negative_signeds(
                            seed,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds::<U>(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                        .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | U::ONE))
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_15<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(S, V)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signed_range::<U, S>(
                seed,
                S::MIN + S::ONE,
                S::ZERO,
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

pub fn special_random_signed_unsigned_pair_gen_var_16<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, bool) --

pub fn random_signed_unsigned_bool_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64, bool)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_signeds(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                    },
                )
                .map(|(x, y)| (x, y, x < T::ZERO))
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_signeds(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| random_unsigneds_less_than(seed_2, T::WIDTH),
                )
                .map(|(x, y)| (x, y, x >= T::ZERO))
            },
        )
        .map(Union2::unwrap),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_signed_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_stripe_d", 1),
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

pub fn special_random_signed_unsigned_unsigned_triple_gen_var_2<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(S, V, V)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_triples_xyy(
                    seed,
                    &|seed_2| {
                        striped_random_positive_signeds(
                            seed_2,
                            config.get_or("mean_large_stripe_n", S::WIDTH >> 1),
                            config.get_or("mean_large_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                    },
                )
                .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) })
            },
            &|seed| {
                random_triples(
                    seed,
                    &|seed_2| {
                        striped_random_signed_range::<U, S>(
                            seed_2,
                            S::MIN,
                            S::ZERO,
                            config.get_or("mean_stripe_n", S::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_unsigned_range(
                            seed_2,
                            V::ZERO,
                            V::exact_from(S::WIDTH),
                            config.get_or("mean_stripe_n", S::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
                .filter_map(|(x, y, z): (S, V, V)| y.checked_add(z).map(|new_z| (x, y, new_z)))
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
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

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, u64, u64, U)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &|seed_2| {
                striped_random_signeds(
                    seed_2,
                    config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &|seed_2| {
                striped_random_unsigneds(
                    seed_2,
                    config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_stripe_d", 1),
                )
            },
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

// -- (PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_signed_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

pub fn special_random_signed_unsigned_rounding_mode_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

// var 3 is in malachite-float.

// -- (PrimitiveSigned, RoundingMode) --

pub fn special_random_signed_rounding_mode_pair_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_signed_rounding_mode_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_nonzero_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_signed_rounding_mode_pair_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .filter(|&x| x != T::MIN)
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_signed_rounding_mode_pair_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_nonzero_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .filter(|&x| x != T::MIN)
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_signed_rounding_mode_pair_gen_var_4<
    T: PrimitiveSigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(move |&(n, rm)| rm != Exact || U::convertible_from(n)),
    )
}

// -- (PrimitiveUnsigned, ToSciOptions) --

pub fn special_random_signed_to_sci_options_pair_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, ToSciOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
    ))
}

pub fn special_random_signed_to_sci_options_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, ToSciOptions)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                random_to_sci_options(
                    seed,
                    config.get_or("mean_size_n", 32),
                    config.get_or("mean_size_d", 1),
                )
            },
        )
        .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (PrimitiveSigned, Vec<bool>) --

struct SignedBoolVecPairGeneratorVar1<T: PrimitiveSigned> {
    xs: StripedRandomSigneds<T>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveSigned> Iterator for SignedBoolVecPairGeneratorVar1<T> {
    type Item = (T, Vec<bool>);

    fn next(&mut self) -> Option<(T, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = get_striped_bool_vec(
            &mut self.striped_bit_source,
            u64::exact_from(x.to_bits_asc().len()),
        );
        Some((x, bs))
    }
}

pub fn special_random_signed_bool_vec_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, Vec<bool>)> {
    Box::new(SignedBoolVecPairGeneratorVar1 {
        xs: striped_random_signeds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- PrimitiveUnsigned --

pub fn special_random_unsigned_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_1<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_positive_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_2(config: &GenConfig) -> It<u32> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        NUMBER_OF_CHARS,
        config.get_or("mean_stripe_n", 16),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_3<T: PrimitiveInt>(config: &GenConfig) -> It<u64> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        1,
        T::WIDTH,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_4<
    T: PrimitiveInt,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    config: &GenConfig,
) -> It<U> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        U::TWO,
        U::saturating_from(T::MAX),
        config.get_or("mean_stripe_n", U::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_5<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::TWO,
        T::MAX,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_6<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        T::ZERO,
        T::exact_from(36),
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_7<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::TWO,
        T::exact_from(36),
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_8<T: PrimitiveInt>(config: &GenConfig) -> It<u64> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        T::WIDTH + 1,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_9(config: &GenConfig) -> It<u8> {
    Box::new(
        random_union3s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    b'0',
                    b'9',
                    config.get_or("mean_stripe_n", 4),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    b'a',
                    b'z',
                    config.get_or("mean_stripe_n", 4),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    b'A',
                    b'Z',
                    config.get_or("mean_stripe_n", 4),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(Union3::unwrap),
    )
}

pub fn special_random_unsigned_gen_var_10<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::power_of_2(T::WIDTH - 1),
        T::MAX,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_11<T: PrimitiveFloat>(config: &GenConfig) -> It<u64> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        T::LARGEST_ORDERED_REPRESENTATION,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_12<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        T::ZERO,
        T::power_of_2(T::WIDTH - 1) + T::ONE,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_13<T: PrimitiveInt>(config: &GenConfig) -> It<u64> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        T::WIDTH,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_14<T: PrimitiveInt>(config: &GenConfig) -> It<u64> {
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        T::WIDTH - 1,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_15<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::power_of_2(T::WIDTH - 2),
        T::MAX,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_16<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<T> {
    Box::new(
        striped_random_unsigned_inclusive_range(
            EXAMPLE_SEED,
            T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT),
            T::MAX,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .filter(|&x| !U::convertible_from(x)),
    )
}

pub fn special_random_unsigned_gen_var_17<
    T: TryFrom<NiceFloat<U>> + PrimitiveUnsigned,
    U: PrimitiveFloat + RoundingFrom<T>,
>(
    config: &GenConfig,
) -> It<T> {
    Box::new(
        striped_random_unsigned_inclusive_range(
            EXAMPLE_SEED,
            T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT),
            T::MAX,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_gen_var_18<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigned_inclusive_range(
        EXAMPLE_SEED,
        T::ZERO,
        T::MAX.floor_sqrt(),
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_19<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(
        striped_random_unsigned_inclusive_range(
            EXAMPLE_SEED,
            T::ZERO,
            T::low_mask(T::WIDTH - 1),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )
        .map(|u| (u << 1) | T::ONE),
    )
}

pub fn special_random_unsigned_gen_var_20<T: PrimitiveUnsigned>(config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_factorial);
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        limit,
        config.get_or("mean_stripe_n", 4),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_21<T: PrimitiveUnsigned>(config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_double_factorial);
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        limit,
        config.get_or("mean_stripe_n", 4),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_22<T: PrimitiveUnsigned>(config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_subfactorial);
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        limit,
        config.get_or("mean_stripe_n", 4),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_23<T: PrimitiveUnsigned>(config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_primorial);
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        limit,
        config.get_or("mean_stripe_n", 4),
        config.get_or("mean_stripe_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_24<T: PrimitiveUnsigned>(config: &GenConfig) -> It<u64> {
    let limit = smallest_invalid_value(T::checked_product_of_first_n_primes);
    Box::new(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        limit,
        config.get_or("mean_stripe_n", 4),
        config.get_or("mean_stripe_d", 1),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveSigned) --

pub fn special_random_unsigned_signed_pair_gen<T: PrimitiveUnsigned, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_signed_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

struct IntegerMantissaAndExponentGenerator<T: PrimitiveFloat> {
    xs: NonzeroValues<RandomPrimitiveFloatInclusiveRange<T>>,
    shifts: GeometricRandomNaturalValues<i64>,
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

pub fn special_random_unsigned_signed_pair_gen_var_2<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(u64, i64)> {
    Box::new(IntegerMantissaAndExponentGenerator::<T> {
        xs: random_nonzero_finite_primitive_floats(EXAMPLE_SEED.fork("xs")),
        shifts: geometric_random_natural_signeds(
            EXAMPLE_SEED.fork("shifts"),
            config.get_or("shift_mean_n", 4),
            config.get_or("shift_mean_d", 1),
        ),
    })
}

// -- (PrimitiveUnsigned, PrimitiveSigned, PrimitiveUnsigned) --

struct ModPowerOf2TripleExtraSmallSignedGenerator<T: PrimitiveUnsigned, U: PrimitiveSigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: GeometricRandomSigneds<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveSigned> Iterator
    for ModPowerOf2TripleExtraSmallSignedGenerator<T, U>
{
    type Item = (T, U, u64);

    fn next(&mut self) -> Option<(T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), pow))
    }
}

pub fn special_random_unsigned_signed_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, u64)> {
    Box::new(ModPowerOf2TripleExtraSmallSignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: geometric_random_signeds(
            EXAMPLE_SEED.fork("ms"),
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_signed_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    config: &GenConfig,
) -> It<(T, S, T)> {
    Box::new(
        random_triples_xyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_signed_inclusive_range::<U, S>(
                    seed,
                    if S::WIDTH <= u64::WIDTH {
                        S::MIN
                    } else {
                        -S::exact_from(u64::MAX)
                    },
                    S::saturating_from(u64::MAX),
                    config.get_or("mean_stripe_n", S::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z): (T, S, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

pub fn special_random_unsigned_signed_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_signeds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", U::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

pub fn special_random_unsigned_signed_unsigned_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
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

// -- (PrimitiveUnsigned, PrimitiveSigned, RoundingMode) --

pub fn special_random_unsigned_signed_rounding_mode_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

pub fn special_random_unsigned_signed_rounding_mode_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| random_unsigneds_less_than(seed, T::WIDTH),
    ))
}

pub fn special_random_unsigned_pair_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::saturating_from(T::MAX),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// TODO make better
pub fn special_random_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y)| if x <= y { (x, y) } else { (y, x) }),
    )
}

pub fn special_random_unsigned_pair_gen_var_6<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|(x, y)| (x.round_to_multiple(y, Down).0, y)),
    )
}

pub fn special_random_unsigned_pair_gen_var_8<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_positive_unsigneds::<U>(
                seed,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn special_random_unsigned_pair_gen_var_10<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

pub fn special_random_unsigned_pair_gen_var_11<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
        )
        .map(|(mut x, y)| {
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), Down);
            (x, y)
        }),
    )
}

pub fn special_random_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_ordered_unique_pairs(striped_random_unsigneds::<T>(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

struct ModPowerOf2SingleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
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
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
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
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOf2SingleGenerator2<T> {
    type Item = (T, u64);

    fn next(&mut self) -> Option<(T, u64)> {
        let pow = self.ms.next().unwrap();
        assert_ne!(pow, 0);
        let xs = &mut self.xss[usize::wrapping_from(pow)];
        if xs.is_none() {
            *xs = Some(Box::new(
                striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                )
                .filter(|&x| x != T::ZERO),
            ));
        }
        let x = xs.as_mut().unwrap().next().unwrap();
        Some((x, pow))
    }
}

pub fn special_random_unsigned_pair_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(ModPowerOf2SingleGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_pair_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| random_unsigneds_less_than(seed, U::WIDTH + 1),
    ))
}

pub fn special_random_unsigned_pair_gen_var_16<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigned_range(
                seed,
                U::TWO,
                U::MAX,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_17<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| random_unsigneds_less_than(seed, U::WIDTH + 1),
    ))
}

pub fn special_random_unsigned_pair_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::saturating_from(u64::MAX),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_positive_unsigneds::<U>(
                seed,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_19<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(u64, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_bit_chunks(
                seed,
                T::MANTISSA_WIDTH,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_bit_chunks(
                seed,
                T::EXPONENT_WIDTH,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_20<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(special_random_unsigned_pair_gen_var_13(config).map(|(x, p)| (x, T::WIDTH - p)))
}

pub fn special_random_unsigned_pair_gen_var_21<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
    ))
}

struct LikelyMultiplyablePairs<T: PrimitiveUnsigned> {
    bits: GeometricRandomNaturalValues<u64>,
    striped_bit_source: StripedBitSource,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for LikelyMultiplyablePairs<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        let x = T::from_bits_asc(
            get_striped_bool_vec(&mut self.striped_bit_source, self.bits.next().unwrap())
                .into_iter(),
        );
        let y = T::from_bits_asc(
            get_striped_bool_vec(&mut self.striped_bit_source, self.bits.next().unwrap())
                .into_iter(),
        );
        Some((x, y))
    }
}

pub fn special_random_unsigned_pair_gen_var_22<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(LikelyMultiplyablePairs {
        bits: geometric_random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("bits"),
            0,
            T::WIDTH,
            T::WIDTH >> 1,
            1,
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        phantom: PhantomData,
    })
}

pub fn special_random_unsigned_pair_gen_var_23<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        special_random_unsigned_pair_gen_var_22::<T>(config)
            .filter(|&(x, y)| x.checked_lcm(y).is_some()),
    )
}

pub fn special_random_unsigned_pair_gen_var_24<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::power_of_2(T::WIDTH - 1),
                T::MAX,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_unsigned_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_25<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter(|&(x, y)| x != T::ONE || y != T::ZERO),
    )
}

pub fn special_random_unsigned_pair_gen_var_26(config: &GenConfig) -> It<(u32, u32)> {
    Box::new(random_pairs_from_single(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        NUMBER_OF_CHARS,
        config.get_or("mean_stripe_n", 16),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_unsigned_pair_gen_var_27<T: PrimitiveUnsigned, U: PrimitiveInt>(
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_stripe_n", 4),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_28<
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
                config.get_or("mean_stripe_n", 4),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_29<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, V)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::saturating_from(U::MAX),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_pair_gen_var_30<
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
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
                config.get_or("mean_stripe_n", 4),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_31<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
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
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_positive_unsigneds::<T>(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed_2| {
                        striped_random_unsigned_range(
                            seed_2,
                            0,
                            T::WIDTH,
                            config.get_or("mean_stripe_n", 4),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_unsigned_pair_gen_var_32<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(u64, u64)> {
    Box::new(random_pairs_from_single(striped_random_unsigned_range(
        EXAMPLE_SEED,
        0,
        T::LARGEST_ORDERED_REPRESENTATION,
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_unsigned_pair_gen_var_33<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::power_of_2(T::WIDTH - 2),
                T::MAX,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_34<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_unsigned_pair_gen_var_35<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_unsigned_pair_gen_var_36<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_ordered_unique_pairs(
        striped_random_positive_unsigneds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    ))
}

pub fn special_random_unsigned_pair_gen_var_37<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
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
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_pair_gen_var_38<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn special_random_unsigned_pair_gen_var_39<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(
            striped_random_unsigned_inclusive_range(
                EXAMPLE_SEED,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE),
        )
        .filter(|&(a, b): &(T, T)| a.coprime_with(b)),
    )
}

pub fn special_random_unsigned_pair_gen_var_40<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter(|&(x, y): &(T, T)| x.coprime_with(y)),
    )
}

pub fn special_random_unsigned_pair_gen_var_41<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(random_pairs_from_single(
        striped_random_unsigned_inclusive_range(
            EXAMPLE_SEED,
            T::TWO,
            T::MAX,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, bool) --

pub fn special_random_unsigned_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64, bool)> {
    reshape_1_2_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            random_union2s(
                seed,
                &|seed_2| {
                    geometric_random_unsigneds(
                        seed_2,
                        config.get_or("small_unsigned_mean_n", 32),
                        config.get_or("small_unsigned_mean_d", 1),
                    )
                    .map(|x| (x, false))
                },
                &|seed_2| random_unsigneds_less_than(seed_2, T::WIDTH).map(|x| (x, true)),
            )
            .map(Union2::unwrap)
        },
    )))
}

pub fn special_random_unsigned_unsigned_bool_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, bool)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_positive_unsigneds::<U>(
                seed,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &random_bools,
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y, z)| reduce_to_fit_add_mul_unsigned(x, y, z)),
    )
}

pub fn special_random_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y, z)| reduce_to_fit_sub_mul_unsigned(x, y, z)),
    )
}

pub fn special_random_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, u64, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_triple_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_unsigned_triple_gen_var_5<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
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
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
}

pub fn special_random_unsigned_triple_gen_var_6<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::TWO,
                U::exact_from(36u8),
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
    ))
}

pub fn special_random_unsigned_triple_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_unsigneds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .map(|(x, y, m)| {
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

pub fn special_random_unsigned_triple_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_unsigneds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

pub fn special_random_unsigned_triple_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
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
                    config.get_or("mean_pow_n", 32),
                    config.get_or("mean_pow_d", 1),
                )
            },
        )
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

pub fn special_random_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, u64)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
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
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

struct ModPowerOf2PairGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
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
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            let xs = xs.as_mut().unwrap();
            (xs.next().unwrap(), xs.next().unwrap())
        };
        Some((x, y, pow))
    }
}

pub fn special_random_unsigned_triple_gen_var_11<T: PrimitiveUnsigned>(
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
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_triple_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(striped_random_unsigneds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter_map(|(x, y, z)| {
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
        }),
    )
}

pub fn special_random_unsigned_triple_gen_var_13<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

pub fn special_random_unsigned_triple_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
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
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

pub fn special_random_unsigned_triple_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", U::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

struct ModPowerOf2TripleExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: StripedRandomUnsignedBitChunks<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOf2TripleExtraUnsignedGenerator<T, U>
{
    type Item = (T, U, u64);

    fn next(&mut self) -> Option<(T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), pow))
    }
}

pub fn special_random_unsigned_triple_gen_var_16<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, u64)> {
    Box::new(ModPowerOf2TripleExtraUnsignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: striped_random_unsigneds::<U>(
            EXAMPLE_SEED.fork("us"),
            config.get_or("mean_stripe_n", U::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

struct ModPowerOf2TripleExtraSmallUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: GeometricRandomNaturalValues<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOf2TripleExtraSmallUnsignedGenerator<T, U>
{
    type Item = (T, U, u64);

    fn next(&mut self) -> Option<(T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), pow))
    }
}

pub fn special_random_unsigned_triple_gen_var_17<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, u64)> {
    Box::new(ModPowerOf2TripleExtraSmallUnsignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("ms"),
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_triple_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, T)> {
    Box::new(
        random_triples_xyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    U::ZERO,
                    U::saturating_from(u64::MAX),
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Equal => None,
            Less => Some((x, y, z)),
            Greater => Some((z, y, x)),
        }),
    )
}

pub fn special_random_unsigned_triple_gen_var_19<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_unsigned_triple_gen_var_20<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
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
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
}

pub fn special_random_unsigned_triple_gen_var_21<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_xyx(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn special_random_unsigned_triple_gen_var_22<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE)
        },
    ))
}

pub fn special_random_unsigned_triple_gen_var_23<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE)
        },
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64, u64, U)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_large_unsigned_stripe_d", 1),
                )
            },
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

pub fn special_random_unsigned_quadruple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, U)> {
    Box::new(random_quadruples_xxxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_large_unsigned_stripe_d", 1),
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

struct ModPowerOf2TripleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
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
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            let xs = xs.as_mut().unwrap();
            (xs.next().unwrap(), xs.next().unwrap(), xs.next().unwrap())
        };
        Some((x, y, z, pow))
    }
}

pub fn special_random_unsigned_quadruple_gen_var_3<T: PrimitiveUnsigned>(
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
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_quadruple_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_quadruples_from_single(striped_random_unsigneds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter_map(|(x, y, z, w)| {
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
        }),
    )
}

pub fn special_random_unsigned_quadruple_gen_var_5<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .map(|(x_1, x_0, d)| {
            let inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
            (x_1, x_0, d, inv)
        }),
    )
}

pub fn special_random_unsigned_quadruple_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, U, T)> {
    Box::new(
        random_quadruples_xxyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds::<U>(
                    seed,
                    config.get_or("mean_stripe_n", U::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

pub fn special_random_unsigned_quadruple_gen_var_7<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, U, T)> {
    Box::new(
        random_quadruples_xyyx(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds::<U>(
                    seed,
                    config.get_or("mean_stripe_n", U::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
        )
        .filter_map(|(x, y, z, w)| match x.cmp(&w) {
            Equal => None,
            Less => Some((x, y, z, w)),
            Greater => Some((w, y, z, x)),
        }),
    )
}

struct ModPowerOf2QuadrupleWithExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: StripedRandomUnsignedBitChunks<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOf2QuadrupleWithExtraUnsignedGenerator<T, U>
{
    type Item = (T, T, U, u64);

    fn next(&mut self) -> Option<(T, T, U, u64)> {
        let pow = self.ms.next().unwrap();
        let (x, y) = if pow == 0 {
            (T::ZERO, T::ZERO)
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            let xs = xs.as_mut().unwrap();
            (xs.next().unwrap(), xs.next().unwrap())
        };
        Some((x, y, self.us.next().unwrap(), pow))
    }
}

pub fn special_random_unsigned_quadruple_gen_var_8<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, U, u64)> {
    Box::new(ModPowerOf2QuadrupleWithExtraUnsignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: striped_random_unsigneds::<U>(
            EXAMPLE_SEED.fork("us"),
            config.get_or("mean_stripe_n", U::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

struct ModPowerOf2QuadrupleWithTwoExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
{
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: StripedRandomUnsignedBitChunks<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOf2QuadrupleWithTwoExtraUnsignedGenerator<T, U>
{
    type Item = (T, U, U, u64);

    fn next(&mut self) -> Option<(T, U, U, u64)> {
        let pow = self.ms.next().unwrap();
        let x = if pow == 0 {
            T::ZERO
        } else {
            let xs = &mut self.xss[usize::wrapping_from(pow)];
            if xs.is_none() {
                *xs = Some(striped_random_unsigned_bit_chunks(
                    EXAMPLE_SEED.fork(&pow.to_string()),
                    pow,
                    self.mean_stripe_n,
                    self.mean_stripe_d,
                ));
            }
            xs.as_mut().unwrap().next().unwrap()
        };
        Some((x, self.us.next().unwrap(), self.us.next().unwrap(), pow))
    }
}

pub fn special_random_unsigned_quadruple_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, U, U, u64)> {
    Box::new(ModPowerOf2QuadrupleWithTwoExtraUnsignedGenerator {
        ms: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("ms"),
            0,
            T::WIDTH,
            config.get_or("mean_pow_n", T::WIDTH >> 1),
            config.get_or("mean_pow_d", 1),
        ),
        us: striped_random_unsigneds::<U>(
            EXAMPLE_SEED.fork("us"),
            config.get_or("mean_stripe_n", U::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        xss: vec![None; usize::wrapping_from(T::WIDTH) + 1],
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
    })
}

pub fn special_random_unsigned_quadruple_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

pub fn special_random_unsigned_quadruple_gen_var_11<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_quadruples_from_single(striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .filter(|&(n1, n0, d1, d0)| {
            // conditions: D >= 2^W, N >= D, and N / D < 2^W
            d1 != T::ZERO && (n1 > d1 || n1 == d1 && n0 >= d0)
        }),
    )
}

pub fn special_random_unsigned_quadruple_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_xxxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE)
        },
    ))
}

// -- (PrimitiveUnsigned * 6) --

pub fn special_random_unsigned_sextuple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T, T, T)> {
    Box::new(random_sextuples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

// var 2 is in malachite-nz.

// -- (PrimitiveUnsigned * 8) --

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_octuple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T, T, T, T, T)> {
    Box::new(random_octuples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

// -- (PrimitiveUnsigned * 9) --

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_nonuple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T, T, T, T, T, T)> {
    Box::new(
        random_triples_from_single(random_triples_from_single(striped_random_unsigneds(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        )))
        .map(|((a, b, c), (d, e, f), (g, h, i))| (a, b, c, d, e, f, g, h, i)),
    )
}

// -- (PrimitiveUnsigned * 12) --

#[allow(clippy::type_complexity)]
pub fn special_random_unsigned_duodecuple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T, T, T, T, T, T, T, T, T)> {
    Box::new(random_duodecuples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

struct UnsignedUnsignedRoundingModeTripleGenerator<T: PrimitiveUnsigned> {
    xs: StripedRandomUnsignedBitChunks<T>,
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

pub fn special_random_unsigned_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, RoundingMode)> {
    Box::new(UnsignedUnsignedRoundingModeTripleGenerator {
        xs: striped_random_unsigneds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        rms: random_rounding_modes(EXAMPLE_SEED.fork("rms")),
    })
}

pub fn special_random_unsigned_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, y, rm)| round_to_multiple_unsigned_filter_map(x, y, rm)),
    )
}

pub fn special_random_unsigned_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds(
                    seed,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter_map(|(x, pow, rm)| round_to_multiple_of_power_of_2_filter_map(x, pow, rm)),
    )
}

pub fn special_random_unsigned_unsigned_rounding_mode_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_unsigneds::<U>(
                    seed,
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(x, y, rm)| rm != Exact || x.divisible_by_power_of_2(y.exact_into())),
    )
}

// var 5 is in malachite-float.

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

struct UnsignedUnsignedBoolVecTripleGeneratorVar1<T: PrimitiveUnsigned> {
    xs: StripedRandomUnsignedBitChunks<T>,
    log_bases: StripedRandomUnsignedInclusiveRange<u64>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedUnsignedBoolVecTripleGeneratorVar1<T> {
    type Item = (T, u64, Vec<bool>);

    fn next(&mut self) -> Option<(T, u64, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let log_base = self.log_bases.next().unwrap();
        let bs = get_striped_bool_vec(
            &mut self.striped_bit_source,
            x.significant_bits().div_round(log_base, Ceiling).0,
        );
        Some((x, log_base, bs))
    }
}

pub fn special_random_unsigned_unsigned_bool_vec_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(T, u64, Vec<bool>)> {
    Box::new(UnsignedUnsignedBoolVecTripleGeneratorVar1 {
        xs: striped_random_unsigneds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        log_bases: striped_random_unsigned_inclusive_range(
            EXAMPLE_SEED.fork("log_bases"),
            1,
            U::WIDTH,
            config.get_or("mean_stripe_n", U::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- (PrimitiveUnsigned, RoundingMode) --

pub fn special_random_unsigned_rounding_mode_pair_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_unsigned_rounding_mode_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_unsigned_rounding_mode_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(move |&(n, rm)| rm != Exact || U::convertible_from(n)),
    )
}

// -- (PrimitiveUnsigned, String) --

pub fn special_random_unsigned_string_pair_gen_var_1(config: &GenConfig) -> It<(u8, String)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                2,
                36,
                config.get_or("mean_stripe_n", 4),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            random_strings_using_chars(
                seed,
                &|seed_2| {
                    graphic_weighted_random_chars(
                        seed_2,
                        config.get_or("graphic_char_prob_n", 50),
                        config.get_or("graphic_char_prob_d", 51),
                    )
                },
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
        },
    ))
}

// -- (PrimitiveUnsigned, ToSciOptions) --

pub fn special_random_unsigned_to_sci_options_pair_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, ToSciOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            random_to_sci_options(
                seed,
                config.get_or("mean_size_n", 32),
                config.get_or("mean_size_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_to_sci_options_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, ToSciOptions)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                random_to_sci_options(
                    seed,
                    config.get_or("mean_size_n", 32),
                    config.get_or("mean_size_d", 1),
                )
            },
        )
        .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (PrimitiveUnsigned, Vec<bool>) --

struct UnsignedBoolVecPairGeneratorVar1<T: PrimitiveUnsigned> {
    xs: StripedRandomUnsignedBitChunks<T>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedBoolVecPairGeneratorVar1<T> {
    type Item = (T, Vec<bool>);

    fn next(&mut self) -> Option<(T, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let bs = get_striped_bool_vec(&mut self.striped_bit_source, x.significant_bits());
        Some((x, bs))
    }
}

pub fn special_random_unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, Vec<bool>)> {
    Box::new(UnsignedBoolVecPairGeneratorVar1 {
        xs: striped_random_unsigneds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- RationalSequence<PrimitiveUnsigned> --

pub fn special_random_unsigned_rational_sequence_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<RationalSequence<T>> {
    Box::new(random_rational_sequences(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

// -- (RationalSequence<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn special_random_unsigned_rational_sequence_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rational_sequences(
                seed,
                &|seed_2| {
                    striped_random_unsigneds(
                        seed_2,
                        config.get_or("mean_stripe_n", T::WIDTH >> 1),
                        config.get_or("mean_stripe_d", 1),
                    )
                },
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

pub fn special_random_unsigned_rational_sequence_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, usize)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_rational_sequences(
                    seed,
                    &|seed_2| {
                        striped_random_unsigneds(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
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

// -- (RationalSequence<PrimitiveUnsigned>, RationalSequence<PrimitiveUnsigned>) --

pub fn special_random_unsigned_rational_sequence_pair_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, RationalSequence<T>)> {
    Box::new(random_pairs_from_single(random_rational_sequences(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

// -- RationalSequence<PrimitiveUnsigned> * 3 --

pub fn special_random_unsigned_rational_sequence_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(
    RationalSequence<T>,
    RationalSequence<T>,
    RationalSequence<T>,
)> {
    Box::new(random_triples_from_single(random_rational_sequences(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

// -- String --

pub fn special_random_string_gen(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| {
            graphic_weighted_random_chars(
                seed,
                config.get_or("graphic_char_prob_n", 50),
                config.get_or("graphic_char_prob_d", 51),
            )
        },
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn special_random_string_gen_var_1(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| {
            graphic_weighted_random_ascii_chars(
                seed,
                config.get_or("graphic_char_prob_n", 50),
                config.get_or("graphic_char_prob_d", 51),
            )
        },
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    ))
}

// vars 2 and 3 are in malachite-q.

pub fn special_random_string_gen_var_4(config: &GenConfig) -> It<String> {
    Box::new(
        random_strings_using_chars(
            EXAMPLE_SEED,
            &|seed| {
                graphic_weighted_random_chars(
                    seed,
                    config.get_or("graphic_char_prob_n", 50),
                    config.get_or("graphic_char_prob_d", 51),
                )
            },
            config.get_or("mean_length_n", 32),
            config.get_or("mean_length_d", 1),
        )
        .filter(|s| !large_exponent(s)),
    )
}

// -- (String, FromSciStringOptions) --

pub fn special_random_string_from_sci_string_options_pair_gen(
    config: &GenConfig,
) -> It<(String, FromSciStringOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_strings_using_chars(
                seed,
                &|seed_2| {
                    graphic_weighted_random_chars(
                        seed_2,
                        config.get_or("graphic_char_prob_n", 50),
                        config.get_or("graphic_char_prob_d", 51),
                    )
                },
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
        },
        &random_from_sci_string_options,
    ))
}

pub fn special_random_string_from_sci_string_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(String, FromSciStringOptions)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_strings_using_chars(
                seed,
                &|seed_2| {
                    graphic_weighted_random_chars(
                        seed_2,
                        config.get_or("graphic_char_prob_n", 50),
                        config.get_or("graphic_char_prob_d", 51),
                    )
                },
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
            .filter(|s| !large_exponent(s))
        },
        &random_from_sci_string_options,
    ))
}

// -- (String, PrimitiveUnsigned) --

pub fn special_random_string_unsigned_pair_gen_var_1(config: &GenConfig) -> It<(String, u8)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_strings_using_chars(
                seed,
                &|seed_2| {
                    graphic_weighted_random_chars(
                        seed_2,
                        config.get_or("graphic_char_prob_n", 50),
                        config.get_or("graphic_char_prob_d", 51),
                    )
                },
                config.get_or("mean_length_n", 32),
                config.get_or("mean_length_d", 1),
            )
            .filter(|s| !large_exponent(s))
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                2,
                36,
                config.get_or("mean_stripe_n", 4),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (String, String) --

pub fn special_random_string_pair_gen(config: &GenConfig) -> It<(String, String)> {
    Box::new(random_pairs_from_single(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| {
            graphic_weighted_random_chars(
                seed,
                config.get_or("graphic_char_prob_n", 50),
                config.get_or("graphic_char_prob_d", 51),
            )
        },
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

pub fn special_random_string_pair_gen_var_1(config: &GenConfig) -> It<(String, String)> {
    Box::new(random_pairs_from_single(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| {
            graphic_weighted_random_ascii_chars(
                seed,
                config.get_or("graphic_char_prob_n", 50),
                config.get_or("graphic_char_prob_d", 51),
            )
        },
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

// -- (ToSciOptions, PrimitiveUnsigned> --

pub fn special_random_to_sci_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
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

// -- Vec<bool> --

pub fn special_random_bool_vec_gen(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(striped_random_bool_vecs(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 8),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn special_random_bool_vec_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_bool_vecs_length_inclusive_range(
                    seed,
                    0,
                    T::WIDTH,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_fixed_length_bool_vecs(
                            seed_2,
                            T::WIDTH,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
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

pub fn special_random_bool_vec_gen_var_2<T: PrimitiveSigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_bool_vecs_length_inclusive_range(
                    seed,
                    0,
                    T::WIDTH,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_fixed_length_bool_vecs(
                            seed_2,
                            T::WIDTH - 1,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
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

pub fn special_random_bool_vec_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_bool_vecs_length_inclusive_range(
                    seed,
                    0,
                    T::WIDTH,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_fixed_length_bool_vecs(
                            seed_2,
                            T::WIDTH,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
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

pub fn special_random_bool_vec_gen_var_4<T: PrimitiveSigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_bool_vecs_length_inclusive_range(
                    seed,
                    0,
                    T::WIDTH - 1,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_fixed_length_bool_vecs(
                            seed_2,
                            T::WIDTH - 1,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
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

pub fn special_random_bool_vec_gen_var_5(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        striped_random_bool_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", 8),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|bs| bs.iter().any(|&b| b)),
    )
}

// -- Vec<PrimitiveUnsigned> --

pub fn special_random_unsigned_vec_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(striped_random_unsigned_vecs(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH << 1),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

pub fn special_random_unsigned_vec_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<T>> {
    Box::new(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| *xs.last().unwrap() != T::ZERO),
    )
}

pub fn special_random_unsigned_vec_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<T>> {
    Box::new(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| !slice_test_zero(xs)),
    )
}

pub fn special_random_unsigned_vec_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<T>> {
    Box::new(
        striped_random_unsigned_vecs(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| xs.last() != Some(&T::ZERO)),
    )
}

pub fn special_random_unsigned_vec_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<T>> {
    Box::new(striped_random_unsigned_vecs_min_length(
        EXAMPLE_SEED,
        1,
        config.get_or("mean_stripe_n", T::WIDTH << 1),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

// var 5 is in malachite-nz.

pub fn special_random_unsigned_vec_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<Vec<T>> {
    Box::new(striped_random_unsigned_vecs_min_length(
        EXAMPLE_SEED,
        2,
        config.get_or("mean_stripe_n", T::WIDTH << 1),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn special_random_unsigned_vec_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

struct UnsignedVecUnsignedPairGeneratorVar1<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    log_bases: GeometricRandomNaturalValues<u64>,
    ranges: VariableRangeGenerator,
    striped_bit_source: StripedBitSource,
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
                digits.push(U::from_bits_asc(
                    get_striped_bool_vec(&mut self.striped_bit_source, log_base).into_iter(),
                ));
            }
            if digits_valid::<T, U>(log_base, &digits) {
                return Some((digits, log_base));
            }
        }
    }
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
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
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        ranges: VariableRangeGenerator::new(EXAMPLE_SEED.fork("ranges")),
        phantom_t: PhantomData,
        phantom_u: PhantomData,
    })
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<U>, u64)> {
    Box::new(
        special_random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>(config)
            .map(|(xs, y)| (xs.into_iter().rev().collect(), y)),
    )
}

struct UnsignedVecUnsignedPairGeneratorVar3<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
    xs: GeometricRandomNaturalValues<usize>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedVecUnsignedPairGeneratorVar3<T> {
    type Item = (Vec<T>, usize);

    fn next(&mut self) -> Option<(Vec<T>, usize)> {
        let x_1 = self.xs.next().unwrap();
        let x_2 = self.xs.next().unwrap();
        let (len, i) = if x_1 <= x_2 { (x_2, x_1) } else { (x_1, x_2) };
        Some((
            get_striped_unsigned_vec(
                &mut self.striped_bit_source,
                u64::exact_from(len) << T::LOG_WIDTH,
            ),
            i,
        ))
    }
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, usize)> {
    Box::new(UnsignedVecUnsignedPairGeneratorVar3 {
        phantom: PhantomData,
        xs: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("xs"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// var 4 is in malachite-nz

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigned_range(
                seed.fork("log_bases"),
                1,
                T::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::TWO,
                T::MAX,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

struct PowerOf2DigitsGenerator<T: PrimitiveUnsigned> {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    digit_map: HashMap<u64, StripedRandomUnsignedBitChunks<T>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for PowerOf2DigitsGenerator<T> {
    type Item = (Vec<T>, u64);

    fn next(&mut self) -> Option<(Vec<T>, u64)> {
        let log_base = self.log_bases.next().unwrap();
        let digit_count = self.digit_counts.next().unwrap();
        let mean_stripe_n = self.mean_stripe_n;
        let mean_stripe_d = self.mean_stripe_d;
        let digits = self.digit_map.entry(log_base).or_insert_with(|| {
            striped_random_unsigned_bit_chunks(
                EXAMPLE_SEED.fork(&log_base.to_string()),
                log_base,
                mean_stripe_n,
                mean_stripe_d,
            )
        });
        let digits = digits.take(digit_count).collect_vec();
        Some((digits, log_base))
    }
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>(
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
        digit_map: HashMap::new(),
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH << 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        phantom: PhantomData,
    })
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigned_inclusive_range(
                seed.fork("log_bases"),
                1,
                T::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_9<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed.fork("base"),
                U::TWO,
                U::saturating_from(T::MAX),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_10<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_11<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_12<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_13<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
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
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_14<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
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
        .filter(|(xs, y)| *y < U::exact_from(xs.len() << T::LOG_WIDTH)),
    )
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_15<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_16<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_17<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
        },
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_unsigned_n", 4),
                config.get_or("mean_small_unsigned_d", 1),
            )
        },
    ))
}

// var 18 is in malachite-nz

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_19<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_20<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_21<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::power_of_2(U::WIDTH - 1),
                U::MAX,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_22<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_range(
                seed,
                U::ONE,
                U::power_of_2(U::WIDTH - 1),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_23<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                U::power_of_2(U::WIDTH - 1),
                U::MAX,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_24<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_range(
                seed,
                U::ONE,
                U::power_of_2(U::WIDTH - 1),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_25<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_range(
                seed,
                U::ONE,
                U::power_of_2(U::WIDTH - 2),
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// vars 26 through 27 are in malachite-nz.

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_28<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_29<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigned_range(
                seed.fork("log_bases"),
                1,
                U::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_30<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            geometric_random_unsigned_range(
                seed.fork("log_bases"),
                1,
                U::WIDTH,
                config.get_or("mean_log_base_n", 4),
                config.get_or("mean_log_base_d", 1),
            )
        },
    ))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, V)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
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
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

struct UnsignedVecUnsignedUnsignedTripleGeneratorVar1<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
    is: GeometricRandomNaturalValues<usize>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedVecUnsignedUnsignedTripleGeneratorVar1<T> {
    type Item = (Vec<T>, usize, usize);

    fn next(&mut self) -> Option<(Vec<T>, usize, usize)> {
        let i = self.is.next().unwrap();
        let j = self.is.next().unwrap();
        let excess = self.is.next().unwrap();
        let xs = get_striped_unsigned_vec(
            &mut self.striped_bit_source,
            u64::exact_from(i * j + excess) << T::LOG_WIDTH,
        );
        Some((xs, i, j))
    }
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, usize, usize)> {
    Box::new(UnsignedVecUnsignedUnsignedTripleGeneratorVar1 {
        phantom: PhantomData,
        is: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("is"),
            config.get_or("small_unsigned_mean_n", 2),
            config.get_or("small_unsigned_mean_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
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

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    1,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
                .filter(|xs| !slice_test_zero(xs))
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

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// var 6 is in malachite-nz.

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T, T)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_8<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
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

// var 9 is in malachite-nz.

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T, T)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
        },
        &|seed| {
            striped_random_unsigned_inclusive_range(
                seed,
                T::ZERO,
                T::low_mask(T::WIDTH - 1),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
            .map(|u| (u << 1) | T::ONE)
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// vars 11 through 12 are in malachite-nz.

pub fn special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_13<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
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

// var 14 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_unsigned_vec_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, T, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_unsigned_vec_unsigned_rounding_mode_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, U, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| !slice_test_zero(xs))
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

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub fn special_random_unsigned_vec_pair_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(striped_random_unsigned_vecs(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH << 1),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    )))
}

pub struct UnsignedVecPairLenGenerator1<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecPairLenGenerator1<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH),
        ))
    }
}

fn special_random_unsigned_vec_pair_gen_var_1_helper<T: PrimitiveUnsigned>(
    config: &GenConfig,
    seed: Seed,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds(
            seed.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        striped_bit_source: StripedBitSource::new(
            seed.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    special_random_unsigned_vec_pair_gen_var_1_helper(config, EXAMPLE_SEED)
}

pub fn special_random_unsigned_vec_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_pair_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
    ))
}

pub fn special_random_unsigned_vec_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        random_pairs_from_single(striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
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

struct UnsignedVecSqrtRemGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    striped_bit_source: StripedBitSource,
    hi_n_bits: StripedRandomUnsignedInclusiveRange<T>,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecSqrtRemGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (out_len, len) = self.lengths.next().unwrap();
        let out = get_striped_unsigned_vec(&mut self.striped_bit_source, out_len << T::LOG_WIDTH);
        let mut ns: Vec<T> =
            get_striped_unsigned_vec(&mut self.striped_bit_source, len << T::LOG_WIDTH);
        let n_hi = &mut ns[(usize::exact_from(out_len) << 1) - 1];
        n_hi.mod_power_of_2_assign(T::WIDTH - 2);
        *n_hi |= self.hi_n_bits.next().unwrap() << (T::WIDTH - 2);
        Some((out, ns))
    }
}

pub fn special_random_unsigned_vec_pair_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecSqrtRemGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let out_len = x.checked_add(2)?;
            let len: u64 = out_len.arithmetic_checked_shl(1)?;
            let len = len.checked_add(y)?;
            Some((out_len, len))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
        hi_n_bits: striped_random_unsigned_range(
            EXAMPLE_SEED.fork("hi_n_bits"),
            T::ONE,
            T::exact_from(4),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct UnsignedVecSqrtGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecSqrtGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (out_len, len) = self.lengths.next().unwrap();
        let out = get_striped_unsigned_vec(&mut self.striped_bit_source, out_len << T::LOG_WIDTH);
        let mut ns: Vec<T> =
            get_striped_unsigned_vec(&mut self.striped_bit_source, len << T::LOG_WIDTH);
        let hi_n = ns.last_mut().unwrap();
        if *hi_n == T::ZERO {
            *hi_n = T::ONE;
        }
        Some((out, ns))
    }
}

pub fn special_random_unsigned_vec_pair_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecSqrtGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let in_len = x.checked_add(1)?;
            let mut out_len: u64 = in_len.shr_round(1, Ceiling).0;
            out_len = out_len.checked_add(y)?;
            Some((out_len, in_len))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct UnsignedVecPairSameLenGenerator<T: PrimitiveUnsigned, I: Iterator<Item = u64>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = u64>> Iterator
    for UnsignedVecPairSameLenGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let len = self.lengths.next().unwrap() << T::LOG_WIDTH;
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, len),
            get_striped_unsigned_vec(&mut self.striped_bit_source, len),
        ))
    }
}

pub fn special_random_unsigned_vec_pair_gen_var_7<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairSameLenGenerator {
        phantom: PhantomData,
        lengths: geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_pair_gen_var_8<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        striped_random_unsigned_vecs(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| xs.last() != Some(&T::ZERO)),
    ))
}

pub fn special_random_unsigned_vec_pair_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| !slice_test_zero(xs)),
    ))
}

pub struct UnsignedVecPairLenGenerator2<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecPairLenGenerator2<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        let mut xs;
        loop {
            xs = get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH);
            if !slice_test_zero(&xs) {
                break;
            }
        }
        let mut ys;
        loop {
            ys = get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH);
            if !slice_test_zero(&ys) {
                break;
            }
        }
        Some((xs, ys))
    }
}

pub fn special_random_unsigned_vec_pair_gen_var_10<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator2 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// var 11 is in malachite-nz.

pub fn special_random_unsigned_vec_pair_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecPairLenGenerator1 {
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
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(_, ds)| *ds.last().unwrap() != T::ZERO),
    )
}

pub fn special_random_unsigned_vec_pair_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
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
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(xs, mut ys)| {
            ys[0] |= T::ONE;
            (xs, ys)
        }),
    )
}

// vars 14 through 15 are in malachite-nz.

pub fn special_random_unsigned_vec_pair_gen_var_16<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
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
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(xs, ys)| *xs.last().unwrap() != T::ZERO && *ys.last().unwrap() != T::ZERO),
    )
}

pub fn special_random_unsigned_vec_pair_gen_var_17<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecPairLenGenerator1 {
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
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(xs, ys)| *xs.last().unwrap() != T::ZERO && *ys.last().unwrap() != T::ZERO),
    )
}

// var 18 is in malachite-nz.

pub fn special_random_unsigned_vec_pair_gen_var_19<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs_min_length(
                    seed,
                    2,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_length_n", 4),
                    config.get_or("mean_length_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigned_inclusive_range(
                    seed,
                    T::power_of_2(T::WIDTH - 1),
                    T::MAX,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_unsigned_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_unsigned_stripe_d", 1),
                )
            },
        )
        .map(|(n, d_1, d_0)| (n, vec![d_0, d_1])),
    )
}

pub fn special_random_unsigned_vec_pair_gen_var_20<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(random_pairs_from_single(
        striped_random_unsigned_vecs_min_length::<T>(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_vec_pair_gen_var_21<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
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
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// vars 22 through 31 are in malachite-nz.

pub fn special_random_unsigned_vec_pair_gen_var_32<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_pair_gen_var_33<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecPairSameLenGenerator {
            phantom: PhantomData,
            lengths: geometric_random_positive_unsigneds(
                EXAMPLE_SEED.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter(|(ref xs, ref ys): &(Vec<T>, Vec<T>)| {
            (*xs.last().unwrap() != T::ZERO || *ys.last().unwrap() != T::ZERO) && ys[0].odd()
        }),
    )
}

// var 34 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, bool) --

pub fn special_random_unsigned_vec_unsigned_vec_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, bool)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| UnsignedVecPairSameLenGenerator {
            phantom: PhantomData,
            lengths: geometric_random_positive_unsigneds(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
            striped_bit_source: StripedBitSource::new(
                seed.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        },
        &random_bools,
    )))
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| special_random_unsigned_vec_pair_gen_var_1_helper(config, seed),
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

// var 2 and 3 are in malachite-nz

fn special_random_unsigned_vec_pair_gen_var_2_helper<T: PrimitiveUnsigned>(
    config: &GenConfig,
    seed: Seed,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator1 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            seed.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        striped_bit_source: StripedBitSource::new(
            seed.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| special_random_unsigned_vec_pair_gen_var_2_helper(config, seed),
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

struct UnsignedVecPairLenGenerator3<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecPairLenGenerator3<T, I>
{
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        let xs = get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH);
        let mut ys;
        loop {
            ys = get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH);
            if !slice_test_zero(&ys) {
                break;
            }
        }
        Some((xs, ys))
    }
}

fn special_random_unsigned_vec_pair_gen_var_3_helper<T: PrimitiveUnsigned>(
    config: &GenConfig,
    seed: Seed,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecPairLenGenerator3 {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_positive_unsigneds(
            seed.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .map(|(x, y)| if x >= y { (x, y) } else { (y, x) }),
        striped_bit_source: StripedBitSource::new(
            seed.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| special_random_unsigned_vec_pair_gen_var_3_helper(config, seed),
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// vars 7 through 8 are in malachite-nz.

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
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

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub struct PrimitiveIntVecPairLenAndIndexGenerator<
    T: PrimitiveUnsigned,
    I: Iterator<Item = (u64, u64, u64)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> Iterator
    for PrimitiveIntVecPairLenAndIndexGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>, usize);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, usize)> {
        let (i, j, k) = self.lengths.next().unwrap();
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH),
            usize::exact_from(k),
        ))
    }
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, usize)> {
    Box::new(PrimitiveIntVecPairLenAndIndexGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(y)?;
            let o = o.checked_add(x)?;
            Some((o, x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| UnsignedVecPairSameLenGenerator {
            phantom: PhantomData,
            lengths: geometric_random_positive_unsigneds(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ),
            striped_bit_source: StripedBitSource::new(
                seed.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(random_pairs(
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
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        },
        &|seed| {
            striped_random_positive_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

// vars 14 through 21 are in malachite-nz.

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, u64)> {
    reshape_2_1_to_3(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| special_random_unsigned_vec_pair_gen_var_1_helper(config, seed),
        &|seed| {
            striped_random_unsigned_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, u64)> {
    reshape_2_1_to_3(Box::new(random_pairs(
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
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        },
        &|seed| {
            striped_random_unsigned_range(
                seed,
                1,
                U::WIDTH,
                config.get_or("mean_stripe_n", U::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            )
        },
    )))
}

pub fn special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, usize)> {
    Box::new(PrimitiveIntVecPairLenAndIndexGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, i)| {
            let x = x.checked_add(i)?;
            Some((x, x, i))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub struct UnsignedVecTripleXYYLenGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecTripleXYYLenGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (i, j) = self.lengths.next().unwrap();
        let shifted_j = j << T::LOG_WIDTH;
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, shifted_j),
            get_striped_unsigned_vec(&mut self.striped_bit_source, shifted_j),
        ))
    }
}

pub fn special_random_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
            Some((x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub struct UnsignedVecTripleLenGenerator1<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>>
{
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> Iterator
    for UnsignedVecTripleLenGenerator1<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (i, j, k) = self.lengths.next().unwrap();
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, k << T::LOG_WIDTH),
        ))
    }
}

pub struct UnsignedVecQuadrupleLenGenerator1<
    T: PrimitiveUnsigned,
    I: Iterator<Item = (u64, u64, u64, u64)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64, u64)>> Iterator
    for UnsignedVecQuadrupleLenGenerator1<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, k, l) = self.lengths.next().unwrap();
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, k << T::LOG_WIDTH),
            get_striped_unsigned_vec(&mut self.striped_bit_source, l << T::LOG_WIDTH),
        ))
    }
}

pub fn special_random_unsigned_vec_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_3<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

// vars 4 through 23 are in malachite-nz

pub fn special_random_unsigned_vec_triple_gen_var_24<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(y)?;
            Some((x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_25<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(2)?;
            let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
            Some((x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_26<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let y = y.checked_add(2)?;
            let x = x.checked_add(y)?;
            Some((x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct UnsignedVecTripleXXXLenGenerator<T: PrimitiveUnsigned, I: Iterator<Item = u64>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = u64>> Iterator
    for UnsignedVecTripleXXXLenGenerator<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let i = self.lengths.next().unwrap() << T::LOG_WIDTH;
        Some((
            get_striped_unsigned_vec(&mut self.striped_bit_source, i),
            get_striped_unsigned_vec(&mut self.striped_bit_source, i),
            get_striped_unsigned_vec(&mut self.striped_bit_source, i),
        ))
    }
}

pub fn special_random_unsigned_vec_triple_gen_var_27<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXXXLenGenerator {
        phantom: PhantomData,
        lengths: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("lengths"),
            2,
            u64::MAX,
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct UnsignedVecSqrtRemGenerator3<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> Iterator
    for UnsignedVecSqrtRemGenerator3<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (out_len, rs_len, len) = self.lengths.next().unwrap();
        let out = get_striped_unsigned_vec(&mut self.striped_bit_source, out_len << T::LOG_WIDTH);
        let rs = get_striped_unsigned_vec(&mut self.striped_bit_source, rs_len << T::LOG_WIDTH);
        let mut ns: Vec<T> =
            get_striped_unsigned_vec(&mut self.striped_bit_source, len << T::LOG_WIDTH);
        let hi_n = ns.last_mut().unwrap();
        if *hi_n == T::ZERO {
            *hi_n = T::ONE;
        }
        Some((out, rs, ns))
    }
}

pub fn special_random_unsigned_vec_triple_gen_var_28<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecSqrtRemGenerator3 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y, z)| {
            let in_len = x.checked_add(1)?;
            let mut out_len: u64 = in_len.shr_round(1, Ceiling).0;
            out_len = out_len.checked_add(y)?;
            let rem_len = in_len.checked_add(z)?;
            Some((out_len, rem_len, in_len))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_29<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXXXLenGenerator {
        phantom: PhantomData,
        lengths: geometric_random_unsigneds(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_30<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_from_single(
        striped_random_unsigned_vecs(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| xs.last() != Some(&T::ZERO)),
    ))
}

pub fn special_random_unsigned_vec_triple_gen_var_31<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleXYYLenGenerator {
        phantom: PhantomData,
        lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(x, y)| {
            let x = x.checked_add(y)?;
            Some((x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_32<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let o = max(x, y).checked_add(o)?;
            Some((o, x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

struct UnsignedVecTripleLenGenerator2<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> Iterator
    for UnsignedVecTripleLenGenerator2<T, I>
{
    type Item = (Vec<T>, Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, Vec<T>)> {
        let (i, j, k) = self.lengths.next().unwrap();
        let out = get_striped_unsigned_vec(&mut self.striped_bit_source, i << T::LOG_WIDTH);
        let mut xs;
        loop {
            xs = get_striped_unsigned_vec(&mut self.striped_bit_source, j << T::LOG_WIDTH);
            if !slice_test_zero(&xs) {
                break;
            }
        }
        let mut ys;
        loop {
            ys = get_striped_unsigned_vec(&mut self.striped_bit_source, k << T::LOG_WIDTH);
            if !slice_test_zero(&ys) {
                break;
            }
        }
        Some((out, xs, ys))
    }
}

pub fn special_random_unsigned_vec_triple_gen_var_33<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator2 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_34<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator2 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_35<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator2 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_36<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_from_single(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            2,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

// vars 37 through 38 are in malachite-nz.

pub fn special_random_unsigned_vec_triple_gen_var_39<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_from_single(
        striped_random_unsigned_vecs_min_length(
            EXAMPLE_SEED,
            1,
            config.get_or("mean_stripe_n", T::WIDTH << 1),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        )
        .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

pub fn special_random_unsigned_vec_triple_gen_var_40<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator1 {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .filter_map(|(o, x, y)| {
            let x = x.checked_add(y)?;
            let o = o.checked_add(x)?;
            Some((o, x, y))
        }),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_unsigned_vec_triple_gen_var_41<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                1,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
        &|seed| {
            striped_random_unsigned_vecs_min_length(
                seed,
                2,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| *xs.last().unwrap() != T::ZERO)
        },
    ))
}

// vars 42 through 49 are in malachite-nz.

pub fn special_random_unsigned_vec_triple_gen_var_50<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(x, mut y, z): (Vec<T>, Vec<T>, Vec<T>)| {
            y.last_mut().unwrap().set_bit(T::WIDTH - 1);
            (x, y, z)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_51<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(x, mut y, z): (Vec<T>, Vec<T>, Vec<T>)| {
            y.last_mut().unwrap().set_bit(T::WIDTH - 1);
            (x, y, z)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_52<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .map(|(x, y, mut z): (Vec<T>, Vec<T>, Vec<T>)| {
            z.last_mut().unwrap().set_bit(T::WIDTH - 1);
            (x, y, z)
        }),
    )
}

pub fn special_random_unsigned_vec_triple_gen_var_53<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
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
                .filter(|&(x, y)| x >= y - 2),
                striped_bit_source: StripedBitSource::new(
                    seed.fork("striped_bit_source"),
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                ),
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        striped_random_unsigned_inclusive_range(
                            seed_2,
                            T::power_of_2(T::WIDTH - 1),
                            T::MAX,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                    &|seed| {
                        striped_random_unsigneds(
                            seed,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
                )
            },
        )
        .map(|((q, n), (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// vars 54 through 56 are in malachite-nz.

pub fn special_random_unsigned_vec_triple_gen_var_57<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        }
        .filter_map(|(r, n, mut d): (Vec<T>, Vec<T>, Vec<T>)| {
            let last_d = d.last_mut().unwrap();
            *last_d = last_d.checked_add(T::ONE)?;
            Some((r, n, d))
        }),
    )
}

// var 58 is in malachite-nz.

pub fn special_random_unsigned_vec_triple_gen_var_59<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        UnsignedVecTripleLenGenerator1 {
            phantom: PhantomData,
            lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
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
            striped_bit_source: StripedBitSource::new(
                EXAMPLE_SEED.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
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

// var 60 is in malachite-nz.

// -- large types --

pub fn special_random_large_type_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| special_random_unsigned_vec_pair_gen_var_1_helper(config, seed),
        &|seed| {
            random_pairs_from_single(striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ))
        },
    )))
}

struct UnsignedVecSqrtRemGenerator2<T: PrimitiveUnsigned> {
    pub phantom: PhantomData<*const T>,
    pub lengths: GeometricRandomNaturalValues<u64>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedVecSqrtRemGenerator2<T> {
    type Item = (Vec<T>, Vec<T>, u64, bool);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>, u64, bool)> {
        let len = self.lengths.next().unwrap();
        let n = len.shr_round(1, Ceiling).0;
        let out = get_striped_unsigned_vec(&mut self.striped_bit_source, n << T::LOG_WIDTH);
        let mut ns: Vec<T> =
            get_striped_unsigned_vec(&mut self.striped_bit_source, len << T::LOG_WIDTH);
        let last = ns.last_mut().unwrap();
        if *last == T::ZERO {
            *last = T::ONE;
        }
        let shift = last.leading_zeros() >> 1;
        Some((out, ns, shift, len.odd()))
    }
}

pub fn special_random_large_type_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, u64, bool)> {
    Box::new(UnsignedVecSqrtRemGenerator2 {
        phantom: PhantomData,
        lengths: geometric_random_unsigned_range::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            9,
            u64::MAX,
            config.get_or("mean_length_n", 12),
            config.get_or("mean_length_d", 1),
        ),
        striped_bit_source: StripedBitSource::new(
            EXAMPLE_SEED.fork("striped_bit_source"),
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ),
    })
}

pub fn special_random_large_type_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        random_quadruples_xyyx::<_, _, U, _>(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
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

pub fn special_random_large_type_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        random_quadruples_xyyz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigned_vecs::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
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
                striped_random_unsigned_vecs::<T>(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH << 1),
                    config.get_or("mean_stripe_d", 1),
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
pub fn special_random_large_type_gen_var_9<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>, bool)> {
    reshape_3_1_to_4(Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| UnsignedVecTripleXYYLenGenerator {
            phantom: PhantomData,
            lengths: random_pairs_from_single(geometric_random_unsigneds::<u64>(
                seed.fork("lengths"),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            ))
            .filter_map(|(x, y)| {
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            striped_bit_source: StripedBitSource::new(
                seed.fork("striped_bit_source"),
                config.get_or("mean_stripe_n", T::WIDTH >> 1),
                config.get_or("mean_stripe_d", 1),
            ),
        },
        &random_bools,
    )))
}

// vars 10 through 21 are in malachite-nz.

pub fn special_random_large_type_gen_var_22<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(RationalSequence<T>, usize, T, T)> {
    Box::new(
        random_quadruples_xyzz(
            EXAMPLE_SEED,
            &|seed| {
                random_rational_sequences(
                    seed,
                    &|seed_2| {
                        striped_random_unsigneds(
                            seed_2,
                            config.get_or("mean_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_stripe_d", 1),
                        )
                    },
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
            &|seed| {
                striped_random_unsigneds(
                    seed,
                    config.get_or("mean_stripe_n", T::WIDTH >> 1),
                    config.get_or("mean_stripe_d", 1),
                )
            },
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

// vars 23 through 24 are in malachite-nz.

pub fn special_random_large_type_gen_var_25<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(bool, Vec<T>, bool, Vec<T>)> {
    Box::new(random_quadruples_xyxy(
        EXAMPLE_SEED,
        &random_bools,
        &|seed| {
            striped_random_unsigned_vecs(
                seed,
                config.get_or("mean_stripe_n", T::WIDTH << 1),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_length_n", 4),
                config.get_or("mean_length_d", 1),
            )
            .filter(|xs| xs.last() != Some(&T::ZERO))
        },
    ))
}
