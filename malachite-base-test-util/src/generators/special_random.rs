use malachite_base::chars::random::{
    graphic_weighted_random_char_inclusive_range, graphic_weighted_random_char_range,
    graphic_weighted_random_chars,
};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_unsigneds_less_than;
use malachite_base::num::random::striped::{
    striped_random_natural_signeds, striped_random_positive_unsigneds, striped_random_signeds,
    striped_random_unsigneds,
};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_triples_from_single,
};

use generators::common::{GenConfig, It};

// -- char --

pub fn special_random_char_gen(config: &GenConfig) -> It<char> {
    Box::new(graphic_weighted_random_chars(
        EXAMPLE_SEED,
        config.get_or("graphic_char_weight_n", 50),
        config.get_or("graphic_char_weight_d", 1),
    ))
}

#[allow(unstable_name_collisions)]
pub fn special_random_char_gen_var_1(config: &GenConfig) -> It<char> {
    Box::new(graphic_weighted_random_char_range(
        EXAMPLE_SEED,
        char::MIN,
        char::MAX,
        config.get_or("graphic_char_weight_n", 50),
        config.get_or("graphic_char_weight_d", 1),
    ))
}

#[allow(unstable_name_collisions)]
pub fn special_random_char_gen_var_2(config: &GenConfig) -> It<char> {
    Box::new(graphic_weighted_random_char_inclusive_range(
        EXAMPLE_SEED,
        '\u{1}',
        char::MAX,
        config.get_or("graphic_char_weight_n", 50),
        config.get_or("graphic_char_weight_d", 1),
    ))
}

// -- (char, char) --

pub fn special_random_char_pair_gen(config: &GenConfig) -> It<(char, char)> {
    Box::new(random_pairs_from_single(graphic_weighted_random_chars(
        EXAMPLE_SEED,
        config.get_or("graphic_char_weight_n", 50),
        config.get_or("graphic_char_weight_d", 1),
    )))
}

// -- PrimitiveSigned --

pub fn special_random_signed_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}

pub fn special_random_signed_gen_var_1<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(special_random_signed_gen(config).filter(|&x| x != T::MIN))
}

pub fn special_random_signed_gen_var_2<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_natural_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}

pub fn special_random_signed_gen_var_3<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(
        striped_random_signeds(
            EXAMPLE_SEED,
            config.get_or("mean_run_length_n", T::WIDTH >> 1),
            config.get_or("mean_run_length_d", 1),
        )
        .filter(|&x| x != T::ZERO && x != T::NEGATIVE_ONE),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_signed_pair_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    )))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_signed_triple_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    )))
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
                config.get_or("large_unsigned_mean_run_length_n", T::WIDTH >> 1),
                config.get_or("large_unsigned_mean_run_length_d", 1),
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

pub fn special_random_signed_unsigned_pair_gen_var_1_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_run_length_n", T::WIDTH >> 1),
                config.get_or("mean_run_length_d", 1),
            )
        },
        &|seed| random_unsigneds_less_than(seed, T::WIDTH),
    ))
}

// -- PrimitiveUnsigned --

pub fn special_random_unsigned_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_pair_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
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
                config.get_or("large_unsigned_mean_run_length_n", T::WIDTH >> 1),
                config.get_or("large_unsigned_mean_run_length_d", 1),
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
                config.get_or("mean_run_length_n", T::WIDTH >> 1),
                config.get_or("mean_run_length_d", 1),
            )
        },
        &|seed| random_unsigneds_less_than(seed, T::WIDTH),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, T, T)> {
    Box::new(random_triples_from_single(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    )))
}

pub fn special_random_unsigned_gen_var_1<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_positive_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}
