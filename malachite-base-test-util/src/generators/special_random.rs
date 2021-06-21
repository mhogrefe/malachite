use generators::common::{reshape_1_2_to_3, reshape_2_1_to_3, reshape_2_2_to_4, GenConfig, It};
use generators::{
    digits_valid, float_rounding_mode_filter_var_1, get_two_highest, reduce_to_fit_add_mul_signed,
    reduce_to_fit_add_mul_unsigned, reduce_to_fit_sub_mul_signed, reduce_to_fit_sub_mul_unsigned,
    shift_integer_mantissa_and_exponent, signed_assign_bits_valid, unsigned_assign_bits_valid,
};
use itertools::repeat_n;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::chars::random::{
    graphic_weighted_random_ascii_chars, graphic_weighted_random_char_inclusive_range,
    graphic_weighted_random_char_range, graphic_weighted_random_chars,
};
use malachite_base::comparison::traits::Min;
use malachite_base::iterators::{with_special_value, NonzeroValues};
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, PowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ExactFrom, HasHalf, JoinHalves, SaturatingFrom, SplitInHalf, WrappingFrom,
    WrappingInto,
};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess, LeadingZeros};
use malachite_base::num::random::geometric::{
    geometric_random_natural_signeds, geometric_random_nonzero_signeds,
    geometric_random_positive_unsigneds, geometric_random_signed_inclusive_range,
    geometric_random_signed_range, geometric_random_signeds, geometric_random_unsigned_range,
    geometric_random_unsigneds, GeometricRandomNaturalValues, GeometricRandomSignedRange,
    GeometricRandomSigneds,
};
use malachite_base::num::random::striped::{
    get_striped_bool_vec, get_striped_unsigned_vec,
    striped_random_bool_vecs_length_inclusive_range, striped_random_fixed_length_bool_vecs,
    striped_random_natural_signeds, striped_random_negative_signeds,
    striped_random_nonzero_signeds, striped_random_positive_signeds,
    striped_random_positive_unsigneds, striped_random_signeds, striped_random_unsigned_bit_chunks,
    striped_random_unsigned_vecs, striped_random_unsigned_vecs_min_length,
    striped_random_unsigneds, StripedBitSource, StripedRandomSigneds,
    StripedRandomUnsignedBitChunks,
};
use malachite_base::num::random::{
    random_finite_primitive_floats, random_nonzero_finite_primitive_floats,
    random_positive_finite_primitive_floats, random_primitive_float_inclusive_range,
    random_primitive_float_range, random_primitive_floats, random_signed_inclusive_range,
    random_signed_range, random_unsigned_inclusive_range, random_unsigned_range,
    random_unsigneds_less_than, variable_range_generator, RandomPrimitiveFloatInclusiveRange,
    RandomUnsignedInclusiveRange, VariableRangeGenerator,
};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::rounding_modes::random::{random_rounding_modes, RandomRoundingModes};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::random::random_strings_using_chars;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_quadruples_from_single, random_quadruples_xxxy,
    random_quadruples_xxyx, random_quadruples_xyyx, random_quadruples_xyyz, random_triples,
    random_triples_from_single, random_triples_xxy, random_triples_xyx, random_triples_xyy,
};
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;
use num::arithmetic::mod_mul::limbs_invert_limb_naive;
use std::cmp::{min, Ordering};
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

// -- PrimitiveFloat --

pub fn special_random_primitive_float_gen<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_floats(EXAMPLE_SEED))
}

pub fn special_random_primitive_float_gen_var_1<T: PrimitiveFloat>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_float_range(
        EXAMPLE_SEED,
        T::NEGATIVE_ONE / T::TWO,
        T::POSITIVE_INFINITY,
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
        random_primitive_floats::<T>(EXAMPLE_SEED)
            .filter(|&f| !f.is_nan() && f != T::POSITIVE_INFINITY),
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

// -- (PrimitiveFloat, RoundingMode) --

pub fn special_random_primitive_float_rounding_mode_pair_gen_var_1<T: PrimitiveFloat>(
    _config: &GenConfig,
) -> It<(T, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &random_primitive_floats,
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
        .filter(|&(f, rm)| rm != RoundingMode::Exact || f.is_integer()),
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
            x.round_to_multiple_assign(y, RoundingMode::Down);
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
                    .round_to_multiple(m.unsigned_abs(), RoundingMode::Down);
                (
                    x,
                    (U::wrapping_from(x).wrapping_add(adjusted_diff)).wrapping_into(),
                    m,
                )
            } else {
                let adjusted_diff = U::wrapping_from(x.wrapping_sub(y))
                    .round_to_multiple(m.unsigned_abs(), RoundingMode::Down);
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
                    .round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
                (
                    x,
                    (U::wrapping_from(x).wrapping_add(adjusted_diff)).wrapping_into(),
                    pow,
                )
            } else {
                let adjusted_diff = U::wrapping_from(x.wrapping_sub(y))
                    .round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
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
        if rm == RoundingMode::Exact {
            x.round_to_multiple_assign(y, RoundingMode::Down);
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

// -- (PrimitiveSigned, PrimitiveUnsigned) --

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
                    &|seed_2| random_unsigned_range(seed_2, 0, T::WIDTH),
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
                    &|seed_2| random_unsigned_range(seed_2, 0, T::WIDTH),
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
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
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
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
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
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), RoundingMode::Down);
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
                            config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_large_unsigned_stripe_d", 1),
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
                            config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_large_unsigned_stripe_d", 1),
                        )
                    },
                    &|seed_2| random_unsigned_inclusive_range(seed_2, 0, T::WIDTH),
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_11<T: PrimitiveSigned>(
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
                            config.get_or("mean_large_unsigned_stripe_n", T::WIDTH >> 1),
                            config.get_or("mean_large_unsigned_stripe_d", 1),
                        )
                    },
                    &|seed_2| random_unsigned_range(seed_2, 0, T::WIDTH),
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn special_random_signed_unsigned_pair_gen_var_12<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    config: &GenConfig,
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
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(T, U, U)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_triples_xyy(
                    seed,
                    &|seed_2| {
                        striped_random_positive_signeds(
                            seed_2,
                            config.get_or("mean_large_stripe_n", T::WIDTH >> 1),
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
                    &|seed_2| random_signed_range(seed_2, T::MIN, T::ZERO),
                    &|seed_2| {
                        geometric_random_unsigneds(
                            seed_2,
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
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
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
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
            let out = shift_integer_mantissa_and_exponent::<T>(mantissa, exponent, shift);
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

struct ModPowerOfTwoTripleExtraSmallSignedGenerator<T: PrimitiveUnsigned, U: PrimitiveSigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: GeometricRandomSigneds<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveSigned> Iterator
    for ModPowerOfTwoTripleExtraSmallSignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoTripleExtraSmallSignedGenerator {
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
                random_signed_inclusive_range(
                    seed,
                    if U::WIDTH <= u64::WIDTH {
                        U::MIN
                    } else {
                        -U::exact_from(u64::MAX)
                    },
                    U::saturating_from(u64::MAX),
                )
            },
        )
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
        }),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_pair_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
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
        &|seed| random_unsigned_inclusive_range(seed, 1, U::WIDTH),
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
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::saturating_from(T::MAX)),
    ))
}

//TODO make better
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
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
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
        .map(|(x, y)| (x.round_to_multiple(y, RoundingMode::Down), y)),
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
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), RoundingMode::Down);
            (x, y)
        }),
    )
}

//TODO make better
pub fn special_random_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(striped_random_unsigneds::<T>(
            EXAMPLE_SEED,
            config.get_or("mean_stripe_n", T::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
        .flat_map(|(x, y)| match x.cmp(&y) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y)),
            Ordering::Greater => Some((y, x)),
        }),
    )
}

struct ModPowerOfTwoSingleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOfTwoSingleGenerator<T> {
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

pub fn special_random_unsigned_pair_gen_var_13<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(ModPowerOfTwoSingleGenerator {
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
        &|seed| random_unsigned_inclusive_range(seed, T::ZERO, T::saturating_from(u64::MAX)),
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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

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
        &|seed| random_unsigned_inclusive_range(seed, 1, U::WIDTH),
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
        &|seed| random_unsigned_inclusive_range(seed, U::TWO, U::exact_from(36u8)),
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
                let adjusted_diff = (y - x).round_to_multiple(m, RoundingMode::Down);
                (x, x + adjusted_diff, m)
            } else {
                let adjusted_diff = (x - y).round_to_multiple(m, RoundingMode::Down);
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
                let adjusted_diff =
                    (y - x).round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
                (x, x + adjusted_diff, pow)
            } else {
                let adjusted_diff =
                    (x - y).round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
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

struct ModPowerOfTwoPairGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOfTwoPairGenerator<T> {
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
    Box::new(ModPowerOfTwoPairGenerator {
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
        .flat_map(|(x, y, z)| {
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
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
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
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
        }),
    )
}

struct ModPowerOfTwoTripleExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: StripedRandomUnsignedBitChunks<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOfTwoTripleExtraUnsignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoTripleExtraUnsignedGenerator {
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

struct ModPowerOfTwoTripleExtraSmallUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: GeometricRandomNaturalValues<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOfTwoTripleExtraSmallUnsignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoTripleExtraSmallUnsignedGenerator {
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
            &|seed| random_unsigned_inclusive_range(seed, U::ZERO, U::saturating_from(u64::MAX)),
        )
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
        }),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_quadruple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
}

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

struct ModPowerOfTwoTripleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned> Iterator for ModPowerOfTwoTripleGenerator<T> {
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
    Box::new(ModPowerOfTwoTripleGenerator {
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
        .flat_map(|(x, y, z, w)| {
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
    T: CheckedFrom<DT> + PrimitiveUnsigned,
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
        .flat_map(|(x, y, z, w)| {
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
        .flat_map(|(x, y, z, w)| match x.cmp(&w) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z, w)),
            Ordering::Greater => Some((w, y, z, x)),
        }),
    )
}

struct ModPowerOfTwoQuadrupleWithExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
{
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: StripedRandomUnsignedBitChunks<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOfTwoQuadrupleWithExtraUnsignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoQuadrupleWithExtraUnsignedGenerator {
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

struct ModPowerOfTwoQuadrupleWithTwoExtraUnsignedGenerator<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<StripedRandomUnsignedBitChunks<T>>>,
    us: StripedRandomUnsignedBitChunks<U>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOfTwoQuadrupleWithTwoExtraUnsignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoQuadrupleWithTwoExtraUnsignedGenerator {
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
        if rm == RoundingMode::Exact {
            x.round_to_multiple_assign(y, RoundingMode::Down);
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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

struct UnsignedUnsignedBoolVecTripleGeneratorVar1<T: PrimitiveUnsigned> {
    xs: StripedRandomUnsignedBitChunks<T>,
    log_bases: RandomUnsignedInclusiveRange<u64>,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for UnsignedUnsignedBoolVecTripleGeneratorVar1<T> {
    type Item = (T, u64, Vec<bool>);

    fn next(&mut self) -> Option<(T, u64, Vec<bool>)> {
        let x = self.xs.next().unwrap();
        let log_base = self.log_bases.next().unwrap();
        let bs = get_striped_bool_vec(
            &mut self.striped_bit_source,
            x.significant_bits()
                .div_round(log_base, RoundingMode::Ceiling),
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
        log_bases: random_unsigned_inclusive_range(EXAMPLE_SEED.fork("log_bases"), 1, U::WIDTH),
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

// -- Vec<bool> --

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
                .map(|(bs, n)| repeat_n(false, n).chain(bs.into_iter()).collect())
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
                .map(|(bs, n)| {
                    repeat_n(n < 0, n.unsigned_abs())
                        .chain(bs.into_iter())
                        .collect()
                })
            },
        )
        .map(Union2::unwrap),
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
        let max_count = usize::exact_from(T::WIDTH.div_round(log_base, RoundingMode::Ceiling));
        loop {
            let digit_count = self.ranges.next_in_inclusive_range(0, max_count);
            let mut digits = Vec::with_capacity(digit_count);
            for _ in 0..digit_count {
                digits.push(U::from_bits_asc(
                    get_striped_bool_vec(&mut self.striped_bit_source, log_base).into_iter(),
                ))
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
        ranges: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
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
        &|seed| random_unsigned_inclusive_range(seed, T::TWO, T::MAX),
    ))
}

struct PowerOfTwoDigitsGenerator<T: PrimitiveUnsigned> {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    digit_map: HashMap<u64, StripedRandomUnsignedBitChunks<T>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for PowerOfTwoDigitsGenerator<T> {
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
    Box::new(PowerOfTwoDigitsGenerator::<T> {
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
            random_unsigned_inclusive_range(seed.fork("base"), U::TWO, U::saturating_from(T::MAX))
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

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

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

pub fn special_random_primitive_int_vec_unsigned_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
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

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

struct UnsignedVecPairLenGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64)>> Iterator
    for UnsignedVecPairLenGenerator<T, I>
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
    Box::new(UnsignedVecPairLenGenerator {
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
    Box::new(UnsignedVecPairLenGenerator {
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
        .filter(|&(ref xs, ref es)| {
            !xs.is_empty()
                && (es.len() > 1 || es.len() == 1 && es[0] > T::ONE)
                && *es.last().unwrap() != T::ZERO
        }),
    )
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

// var 2 is in malachite-nz

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
        .flat_map(|(x, y)| {
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

pub struct UnsignedVecTripleLenGenerator<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>>
{
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub striped_bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = (u64, u64, u64)>> Iterator
    for UnsignedVecTripleLenGenerator<T, I>
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

pub fn special_random_unsigned_vec_triple_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(UnsignedVecTripleLenGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .flat_map(|(o, x, y)| {
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
    Box::new(UnsignedVecTripleLenGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED.fork("lengths"),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
        .flat_map(|(o, x, y)| {
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
        .flat_map(|(x, y)| {
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
        .flat_map(|(x, y)| {
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
        .flat_map(|(x, y)| {
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
