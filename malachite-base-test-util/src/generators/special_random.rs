use generators::common::{reshape_1_2_to_3, GenConfig, It};
use generators::{
    digits_valid, reduce_to_fit_add_mul_signed, reduce_to_fit_add_mul_unsigned,
    reduce_to_fit_sub_mul_signed, reduce_to_fit_sub_mul_unsigned, signed_assign_bits_valid,
    unsigned_assign_bits_valid,
};
use malachite_base::chars::random::{
    graphic_weighted_random_ascii_chars, graphic_weighted_random_char_inclusive_range,
    graphic_weighted_random_char_range, graphic_weighted_random_chars,
};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::arithmetic::traits::{DivRound, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::num::random::geometric::{
    geometric_random_unsigned_range, geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::striped::{
    get_striped_bool_vec, get_striped_unsigned_vec, striped_random_fixed_length_bool_vecs,
    striped_random_natural_signeds, striped_random_negative_signeds,
    striped_random_positive_signeds, striped_random_positive_unsigneds, striped_random_signeds,
    striped_random_unsigned_vecs, striped_random_unsigneds, StripedBitSource,
    StripedRandomUnsignedBitChunks,
};
use malachite_base::num::random::{
    random_signed_range, random_unsigned_inclusive_range, random_unsigned_range,
    random_unsigneds_less_than, RandomUnsignedInclusiveRange,
};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::random::random_strings_using_chars;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_quadruples_xyyz, random_triples,
    random_triples_from_single, random_triples_xxy, random_triples_xyy,
};
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;
use malachite_base::vecs::random::random_vecs_length_inclusive_range;
use std::collections::HashMap;
use std::marker::PhantomData;

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
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
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
                config.get_or("mean_small_unsigned_n", 32),
                config.get_or("mean_small_unsigned_d", 1),
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
                    config.get_or("mean_small_unsigned_n", 32),
                    config.get_or("mean_small_unsigned_d", 1),
                )
            },
        )
        .map(|(x, y, z)| if y <= z { (x, y, z) } else { (x, z, y) }),
    )
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
    phantom: PhantomData<*const T>,
    log_bases: GeometricRandomNaturalValues<u64>,
    log_base_to_xs: HashMap<u64, It<Vec<U>>>,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned> Iterator
    for UnsignedVecUnsignedPairGeneratorVar1<T, U>
{
    type Item = (Vec<U>, u64);

    fn next(&mut self) -> Option<(Vec<U>, u64)> {
        let log_base = self.log_bases.next().unwrap();
        let mean_stripe_n = self.mean_stripe_n;
        let mean_stripe_d = self.mean_stripe_d;
        let xs = self.log_base_to_xs.entry(log_base).or_insert_with(move || {
            Box::new(
                random_vecs_length_inclusive_range(
                    EXAMPLE_SEED.fork(&log_base.to_string()),
                    0,
                    T::WIDTH.div_round(log_base, RoundingMode::Ceiling),
                    &|seed| {
                        striped_random_fixed_length_bool_vecs(
                            seed,
                            mean_stripe_n,
                            mean_stripe_d,
                            log_base,
                        )
                        .map(|bs| U::from_bits_asc(bs.into_iter()))
                    },
                )
                .filter(move |xs| digits_valid::<T, U>(log_base, xs)),
            )
        });
        Some((xs.next().unwrap(), log_base))
    }
}

pub fn special_random_unsigned_vec_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Vec<U>, u64)> {
    Box::new(UnsignedVecUnsignedPairGeneratorVar1::<T, U> {
        phantom: PhantomData,
        log_bases: geometric_random_unsigned_range(
            EXAMPLE_SEED.fork("log_bases"),
            1,
            U::WIDTH,
            config.get_or("mean_log_base_n", 4),
            config.get_or("mean_log_base_d", 1),
        ),
        log_base_to_xs: HashMap::new(),
        mean_stripe_n: config.get_or("mean_stripe_n", T::WIDTH >> 1),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
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
