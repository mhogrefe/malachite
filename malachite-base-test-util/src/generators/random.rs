use malachite_base::bools::random::random_bools;
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::random::{
    random_ascii_chars, random_char_inclusive_range, random_char_range, random_chars,
};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::{
    random_natural_signeds, random_negative_signeds, random_positive_unsigneds,
    random_primitive_ints, random_unsigned_range, random_unsigneds_less_than,
};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::random::{random_strings, random_strings_using_chars};
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_triples_from_single,
};
use malachite_base::unions::Union2;
use malachite_base::vecs::random_values_from_vec;

use generators::common::{reshape_1_2_to_3, GenConfig, It};
use malachite_base::unions::random::random_union2s;
use malachite_base::vecs::random::random_vecs;
use rounding_modes::ROUNDING_MODE_CHARS;

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

// -- PrimitiveInt --

pub fn random_primitive_int_gen<T: PrimitiveInt>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_ints(EXAMPLE_SEED))
}

// -- (PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_pair_gen<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
}

// -- (PrimitiveInt, PrimitiveInt, PrimitiveInt) --

pub fn random_primitive_int_triple_gen<T: PrimitiveInt>(_config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(random_primitive_ints(
        EXAMPLE_SEED,
    )))
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

// -- (PrimitiveSigned, PrimitiveUnsigned)

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

// -- PrimitiveUnsigned --

pub fn random_unsigned_gen_var_1<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_positive_unsigneds(EXAMPLE_SEED))
}

pub fn random_unsigned_gen_var_2(_config: &GenConfig) -> It<u32> {
    Box::new(random_unsigneds_less_than(EXAMPLE_SEED, NUMBER_OF_CHARS))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_unsigned_pair_gen_var_1(_config: &GenConfig) -> It<(u32, u32)> {
    Box::new(random_pairs_from_single(random_unsigneds_less_than(
        EXAMPLE_SEED,
        NUMBER_OF_CHARS,
    )))
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

// -- Vec<PrimitiveUnsigned> --

pub fn random_primitive_int_vec_gen<T: PrimitiveInt>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(random_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints,
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}
