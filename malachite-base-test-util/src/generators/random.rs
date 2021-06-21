use generators::common::{reshape_1_2_to_3, reshape_2_1_to_3, reshape_2_2_to_4, GenConfig, It};
use generators::exhaustive::{float_rounding_mode_filter_var_1, valid_digit_chars};
use generators::{digits_valid, signed_assign_bits_valid, unsigned_assign_bits_valid};
use itertools::repeat_n;
use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::random::{
    random_ascii_chars, random_char_inclusive_range, random_char_range, random_chars,
};
use malachite_base::comparison::traits::Min;
use malachite_base::iterators::with_special_value;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, PowerOf2, ShrRound, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, Digits, ExactFrom, HasHalf, JoinHalves, SaturatingFrom, SplitInHalf, WrappingFrom,
    WrappingInto,
};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros};
use malachite_base::num::random::geometric::{
    geometric_random_natural_signeds, geometric_random_nonzero_signeds,
    geometric_random_positive_unsigneds, geometric_random_signed_inclusive_range,
    geometric_random_signed_range, geometric_random_signeds, geometric_random_unsigned_range,
    geometric_random_unsigneds, GeometricRandomNaturalValues, GeometricRandomSignedRange,
    GeometricRandomSigneds,
};
use malachite_base::num::random::{
    random_highest_bit_set_unsigneds, random_natural_signeds, random_negative_signeds,
    random_nonzero_signeds, random_positive_signeds, random_positive_unsigneds,
    random_primitive_ints, random_signed_inclusive_range, random_signed_range,
    random_unsigned_bit_chunks, random_unsigned_inclusive_range, random_unsigned_range,
    random_unsigneds_less_than, special_random_finite_primitive_floats,
    special_random_nonzero_finite_primitive_floats,
    special_random_positive_finite_primitive_floats,
    special_random_primitive_float_inclusive_range, special_random_primitive_float_range,
    special_random_primitive_floats, variable_range_generator, RandomPrimitiveInts,
    RandomUnsignedBitChunks, RandomUnsignedInclusiveRange, SpecialRandomNonzeroFiniteFloats,
    VariableRangeGenerator,
};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::rounding_modes::random::{random_rounding_modes, RandomRoundingModes};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::random::{random_strings, random_strings_using_chars};
use malachite_base::strings::strings_from_char_vecs;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_quadruples_from_single, random_quadruples_xxxy,
    random_quadruples_xxyx, random_quadruples_xyyx, random_quadruples_xyyz, random_triples,
    random_triples_from_single, random_triples_xxy, random_triples_xyx, random_triples_xyy,
};
use malachite_base::unions::random::{random_union2s, random_union3s};
use malachite_base::unions::{Union2, Union3};
use malachite_base::vecs::random::{
    random_fixed_length_vecs_from_single, random_vecs, random_vecs_length_inclusive_range,
    random_vecs_min_length,
};
use malachite_base::vecs::random_values_from_vec;
use num::arithmetic::mod_mul::limbs_invert_limb_naive;
use num::float::PRIMITIVE_FLOAT_CHARS;
use rounding_modes::ROUNDING_MODE_CHARS;
use std::cmp::{min, Ordering};
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
        T::POSITIVE_INFINITY,
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
        ranges: variable_range_generator(seed.fork("mantissas")),
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
        .filter(|&f| !f.is_nan() && f != T::POSITIVE_INFINITY),
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
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
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
                    config.get_or("small_signed_mean_n", 32),
                    config.get_or("small_signed_mean_d", 1),
                )
            },
        )
        .filter(|&(m, e)| m.precision() <= T::max_precision_for_sci_exponent(e)),
    )
}

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
        .filter(|&(f, rm)| rm != RoundingMode::Exact || f.is_integer()),
    )
}

// -- PrimitiveInt --

pub fn random_primitive_int_gen<T: PrimitiveInt>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_ints(EXAMPLE_SEED))
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

//TODO make better
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

//TODO make better
pub fn random_primitive_int_pair_gen_var_3<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(
        random_pairs_from_single(random_primitive_ints::<T>(EXAMPLE_SEED)).flat_map(
            |(x, y)| match x.cmp(&y) {
                Ordering::Equal => None,
                Ordering::Less => Some((x, y)),
                Ordering::Greater => Some((y, x)),
            },
        ),
    )
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_triple_gen<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

pub fn random_primitive_int_triple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(
        random_triples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED))
            .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

// Returns (highest, second-highest)
pub(crate) fn get_two_highest<T: Ord>(xs: &[T]) -> (&T, &T) {
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
        random_triples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED)).flat_map(
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
        random_triples_xyx(EXAMPLE_SEED, &random_primitive_ints, &random_primitive_ints).flat_map(
            |(x, y, z): (T, U, T)| match x.cmp(&z) {
                Ordering::Equal => None,
                Ordering::Less => Some((x, y, z)),
                Ordering::Greater => Some((z, y, x)),
            },
        ),
    )
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_quadruple_gen<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(random_quadruples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

pub fn random_primitive_int_quadruple_gen_var_1<T: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, T, T, T)> {
    Box::new(
        random_quadruples_from_single(random_primitive_ints::<T>(EXAMPLE_SEED)).flat_map(
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

pub fn random_primitive_int_quadruple_gen_var_3<T: PrimitiveInt, U: PrimitiveInt>(
    _config: &GenConfig,
) -> It<(T, U, U, T)> {
    Box::new(
        random_quadruples_xyyx(
            EXAMPLE_SEED,
            &random_primitive_ints::<T>,
            &random_primitive_ints::<U>,
        )
        .flat_map(|(x, y, z, w)| match x.cmp(&w) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z, w)),
            Ordering::Greater => Some((w, y, z, x)),
        }),
    )
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_signed_mean_n", 32),
                config.get_or("small_signed_mean_d", 1),
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
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        })
        .map(|(mut x, y)| {
            x.round_to_multiple_of_power_of_2_assign(y.exact_into(), RoundingMode::Down);
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
                        config.get_or("small_unsigned_mean_n", 32),
                        config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
            )
        })
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
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
        .flat_map(|(x, y, z): (T, U, T)| match x.cmp(&z) {
            Ordering::Equal => None,
            Ordering::Less => Some((x, y, z)),
            Ordering::Greater => Some((z, y, x)),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
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
                x.significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling),
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
        config.get_or("small_signed_mean_n", 32),
        config.get_or("small_signed_mean_d", 1),
    ))
}

pub fn random_signed_gen_var_6<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_nonzero_signeds(EXAMPLE_SEED))
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
            x.round_to_multiple_assign(y, RoundingMode::Down);
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
        if rm == RoundingMode::Exact {
            x.round_to_multiple_assign(y, RoundingMode::Down);
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
                        config.get_or("small_unsigned_mean_n", 32),
                        config.get_or("small_unsigned_mean_d", 1),
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
                        config.get_or("small_unsigned_mean_n", 32),
                        config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                        config.get_or("small_unsigned_mean_n", 32),
                        config.get_or("small_unsigned_mean_d", 1),
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
                            config.get_or("small_unsigned_mean_n", 32),
                            config.get_or("small_unsigned_mean_d", 1),
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
                        config.get_or("small_unsigned_mean_n", 32),
                        config.get_or("small_unsigned_mean_d", 1),
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
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
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

// -- (PrimitiveSigned, RoundingMode) --

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
        config.get_or("small_unsigned_mean_n", 32),
        config.get_or("small_unsigned_mean_d", 1),
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
        config.get_or("small_unsigned_mean_n", 32),
        config.get_or("small_unsigned_mean_d", 1),
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

// -- (PrimitiveUnsigned, PrimitiveInt, PrimitiveInt, PrimitiveUnsigned) --

struct ModPowerOfTwoQuadrupleWithTwoExtraPrimitiveIntsGenerator<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
> {
    ms: GeometricRandomNaturalValues<u64>,
    us: RandomPrimitiveInts<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator
    for ModPowerOfTwoQuadrupleWithTwoExtraPrimitiveIntsGenerator<T, U>
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
    Box::new(ModPowerOfTwoQuadrupleWithTwoExtraPrimitiveIntsGenerator {
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

struct ModPowerOfTwoTripleWithExtraPrimitiveIntGenerator<T: PrimitiveUnsigned, U: PrimitiveInt> {
    ms: GeometricRandomNaturalValues<u64>,
    us: RandomPrimitiveInts<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator
    for ModPowerOfTwoTripleWithExtraPrimitiveIntGenerator<T, U>
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
    Box::new(ModPowerOfTwoTripleWithExtraPrimitiveIntGenerator {
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
pub(crate) fn shift_integer_mantissa_and_exponent<T: PrimitiveFloat>(
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
            let out = shift_integer_mantissa_and_exponent::<T>(mantissa, exponent, shift);
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

struct ModPowerOfTwoTripleWithExtraSmallSignedGenerator<T: PrimitiveUnsigned, U: PrimitiveSigned> {
    ms: GeometricRandomNaturalValues<u64>,
    us: GeometricRandomSigneds<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveSigned> Iterator
    for ModPowerOfTwoTripleWithExtraSmallSignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoTripleWithExtraSmallSignedGenerator {
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
        .map(|(x, y)| (x.round_to_multiple(y, RoundingMode::Down), y)),
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

struct ModPowerOfTwoSingleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
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

pub fn random_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, u64)> {
    Box::new(ModPowerOfTwoSingleGenerator {
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
                    config.get_or("small_unsigned_mean_n", 32),
                    config.get_or("small_unsigned_mean_d", 1),
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
                config.get_or("small_unsigned_mean_n", 32),
                config.get_or("small_unsigned_mean_d", 1),
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
        &|seed| random_positive_unsigneds(seed),
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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveInt, PrimitiveUnsigned) --

struct ModPowerOfTwoQuadrupleWithExtraPrimitiveIntGenerator<T: PrimitiveUnsigned, U: PrimitiveInt> {
    ms: GeometricRandomNaturalValues<u64>,
    us: RandomPrimitiveInts<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveInt> Iterator
    for ModPowerOfTwoQuadrupleWithExtraPrimitiveIntGenerator<T, U>
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
    Box::new(ModPowerOfTwoQuadrupleWithExtraPrimitiveIntGenerator {
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
    let (p_hi, p_lo) = T::x_mul_y_is_zz(y, z);
    let (r_hi, _) = T::xx_add_yy_is_zz(T::ZERO, x, p_hi, p_lo);
    if r_hi == T::ZERO {
        (x, y, z)
    } else {
        let excess_x: u64 = r_hi.significant_bits();
        let excess_yz = excess_x.shr_round(1, RoundingMode::Ceiling);
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
    let (p_hi, p_lo) = T::x_mul_y_is_zz(y, z);
    let product_bits = if p_hi == T::ZERO {
        p_lo.significant_bits()
    } else {
        p_hi.significant_bits() + T::WIDTH
    };
    if x_bits > product_bits {
        (x, y, z)
    } else {
        let excess = (product_bits - x_bits + 1).shr_round(1, RoundingMode::Ceiling);
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
                let adjusted_diff = (y - x).round_to_multiple(m, RoundingMode::Down);
                (x, x + adjusted_diff, m)
            } else {
                let adjusted_diff = (x - y).round_to_multiple(m, RoundingMode::Down);
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

struct ModPowerOfTwoPairGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
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
    Box::new(ModPowerOfTwoPairGenerator {
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

struct ModPowerOfTwoTripleWithExtraSmallUnsignedGenerator<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    ms: GeometricRandomNaturalValues<u64>,
    us: GeometricRandomNaturalValues<U>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for ModPowerOfTwoTripleWithExtraSmallUnsignedGenerator<T, U>
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
    Box::new(ModPowerOfTwoTripleWithExtraSmallUnsignedGenerator {
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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

struct ModPowerOfTwoTripleGenerator<T: PrimitiveUnsigned> {
    ms: GeometricRandomNaturalValues<u64>,
    xss: Vec<Option<RandomUnsignedBitChunks<T>>>,
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
    Box::new(ModPowerOfTwoTripleGenerator {
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
    T: CheckedFrom<DT> + PrimitiveUnsigned,
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
        if rm == RoundingMode::Exact {
            x.round_to_multiple_assign(y, RoundingMode::Down);
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

// -- (PrimitiveUnsigned, String) --

struct DigitStringGenerator {
    ranges: VariableRangeGenerator,
    digit_map: HashMap<u64, Vec<char>>,
    digit_counts: GeometricRandomNaturalValues<usize>,
}

impl Iterator for DigitStringGenerator {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
        let base = self.ranges.next_in_inclusive_range(2, 36);
        let digits = self
            .digit_map
            .entry(base)
            .or_insert_with(|| valid_digit_chars(u8::wrapping_from(base)));
        let digit_count = self.digit_counts.next().unwrap();
        let mut s = String::with_capacity(digit_count);
        for _ in 0..digit_count {
            let index = self.ranges.next_less_than(digits.len());
            s.push(digits[index]);
        }
        Some((base, s))
    }
}

pub fn random_unsigned_string_pair_gen_var_1(config: &GenConfig) -> It<(u64, String)> {
    Box::new(DigitStringGenerator {
        ranges: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
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

pub fn random_unsigned_string_pair_gen_var_2(config: &GenConfig) -> It<(u64, String)> {
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
    uss: It<(u64, String)>,
    negs: RandomBools,
}

impl Iterator for TargetedIntegerFromStringBaseInputs {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
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

pub fn random_unsigned_string_pair_gen_var_3(config: &GenConfig) -> It<(u64, String)> {
    Box::new(TargetedIntegerFromStringBaseInputs {
        uss: Box::new(DigitStringGenerator {
            ranges: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
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
        .map(|s| format!("\"0x{}\"", s)),
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

// -- Vec<bool> --

pub fn random_bool_vec_gen_var_1<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_vecs_length_inclusive_range(seed, 0, T::WIDTH, &random_bools),
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| random_fixed_length_vecs_from_single(T::WIDTH, random_bools(seed_2)),
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
                        random_fixed_length_vecs_from_single(T::WIDTH - 1, random_bools(seed_2))
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
                    &|seed_2| random_fixed_length_vecs_from_single(T::WIDTH, random_bools(seed_2)),
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

pub fn random_bool_vec_gen_var_4<T: PrimitiveSigned>(config: &GenConfig) -> It<Vec<bool>> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| random_vecs_length_inclusive_range(seed, 0, T::WIDTH - 1, &random_bools),
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_fixed_length_vecs_from_single(T::WIDTH - 1, random_bools(seed_2))
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
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// --(Vec<PrimitiveInt>, PrimitiveUnsigned, PrimitiveUnsigned) --

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
            config.get_or("small_unsigned_mean_n", 2),
            config.get_or("small_unsigned_mean_d", 1),
        ),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

// -- (Vec<PrimitiveInt>, Vec<PrimitiveInt>) --

struct PrimitiveIntVecPairLenGenerator<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize)>> Iterator
    for PrimitiveIntVecPairLenGenerator<T, I>
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
    Box::new(PrimitiveIntVecPairLenGenerator {
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
    Box::new(PrimitiveIntVecPairLenGenerator {
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
        .filter(|&(ref xs, ref es)| {
            !xs.is_empty()
                && (es.len() > 1 || es.len() == 1 && es[0] > T::ONE)
                && *es.last().unwrap() != T::ZERO
        }),
    )
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
        .flat_map(|(x, y)| {
            let y = y.checked_add(1)?;
            let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
            Some((x, y))
        }),
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub struct PrimitiveIntVecTripleLenGenerator<
    T: PrimitiveInt,
    I: Iterator<Item = (usize, usize, usize)>,
> {
    pub phantom: PhantomData<*const T>,
    pub lengths: I,
    pub xs: RandomPrimitiveInts<T>,
}

impl<T: PrimitiveInt, I: Iterator<Item = (usize, usize, usize)>> Iterator
    for PrimitiveIntVecTripleLenGenerator<T, I>
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

pub fn random_primitive_int_vec_triple_gen_var_2<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
        xs: random_primitive_ints(EXAMPLE_SEED.fork("xs")),
    })
}

pub fn random_primitive_int_vec_triple_gen_var_3<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(PrimitiveIntVecTripleLenGenerator {
        phantom: PhantomData,
        lengths: random_triples_from_single(geometric_random_unsigneds::<usize>(
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
        .flat_map(|(x, y)| {
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
        .flat_map(|(x, y)| {
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
        .flat_map(|(x, y)| {
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
        let max_count = usize::exact_from(T::WIDTH.div_round(log_base, RoundingMode::Ceiling));
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
        ranges: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
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
        digits: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
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
        ranges: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
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

struct PowerOfTwoDigitsGenerator<T: PrimitiveUnsigned> {
    log_bases: GeometricRandomNaturalValues<u64>,
    digit_counts: GeometricRandomNaturalValues<usize>,
    ranges: VariableRangeGenerator,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for PowerOfTwoDigitsGenerator<T> {
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
        ranges: variable_range_generator(EXAMPLE_SEED.fork("ranges")),
        phantom: PhantomData,
    })
}

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
