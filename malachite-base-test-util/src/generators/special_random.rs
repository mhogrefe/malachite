use malachite_base::chars::random::{
    graphic_weighted_random_ascii_chars, graphic_weighted_random_char_inclusive_range,
    graphic_weighted_random_char_range, graphic_weighted_random_chars,
};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::striped::{
    striped_random_natural_signeds, striped_random_negative_signeds,
    striped_random_positive_unsigneds, striped_random_signeds, striped_random_unsigned_vecs,
    striped_random_unsigneds,
};
use malachite_base::num::random::{random_unsigned_range, random_unsigneds_less_than};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::random::random_strings_using_chars;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_triples_from_single,
};
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;

use generators::common::{reshape_1_2_to_3, GenConfig, It};
use generators::{
    reduce_to_fit_add_mul_signed, reduce_to_fit_add_mul_unsigned, reduce_to_fit_sub_mul_signed,
    reduce_to_fit_sub_mul_unsigned,
};

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

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_signed_pair_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<(T, T)> {
    Box::new(random_pairs_from_single(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", T::WIDTH >> 1),
        config.get_or("mean_stripe_d", 1),
    )))
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
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
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
                    &|seed| {
                        striped_random_negative_signeds(
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
                        striped_random_natural_signeds(
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
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
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

// -- String --

pub fn special_random_string_gen(config: &GenConfig) -> It<String> {
    Box::new(random_strings_using_chars(
        EXAMPLE_SEED,
        &|seed| {
            graphic_weighted_random_chars(
                seed,
                config.get_or("graphic_char_weight_n", 50),
                config.get_or("graphic_char_weight_d", 1),
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
                config.get_or("graphic_char_weight_n", 50),
                config.get_or("graphic_char_weight_d", 1),
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
                config.get_or("graphic_char_weight_n", 50),
                config.get_or("graphic_char_weight_d", 1),
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
                config.get_or("graphic_char_weight_n", 50),
                config.get_or("graphic_char_weight_d", 1),
            )
        },
        config.get_or("mean_length_n", 32),
        config.get_or("mean_length_d", 1),
    )))
}

// -- Vec<PrimitiveUnsigned> --

pub fn special_random_unsigned_vec_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<Vec<T>> {
    Box::new(striped_random_unsigned_vecs(
        EXAMPLE_SEED,
        config.get_or("mean_stripe_n", 2 << T::LOG_WIDTH),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_length_n", 4),
        config.get_or("mean_length_d", 1),
    ))
}
