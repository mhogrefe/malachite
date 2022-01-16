use crate::common::{rational_to_bigrational, rational_to_rug_rational};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, IsInteger};
use malachite_base::num::random::geometric::{
    geometric_random_signeds, geometric_random_unsigneds,
};
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::random::{
    random_pairs, random_pairs_from_single, random_quadruples_xxyz, random_triples_from_single,
    random_triples_xxy, random_triples_xyy,
};
use malachite_base_test_util::generators::common::{GenConfig, It};
use malachite_nz::integer::random::random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::random::{random_naturals, random_positive_naturals};
use malachite_nz::natural::Natural;
use malachite_q::random::{
    random_non_negative_rationals, random_nonzero_rationals, random_positive_rationals,
    random_rationals,
};
use malachite_q::Rational;

// -- Rational --

pub fn random_rational_gen(config: &GenConfig) -> It<Rational> {
    Box::new(random_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_rational_gen_var_1(config: &GenConfig) -> It<Rational> {
    Box::new(random_nonzero_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_rational_gen_var_2(config: &GenConfig) -> It<Rational> {
    Box::new(random_positive_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

pub fn random_rational_gen_var_3(config: &GenConfig) -> It<Rational> {
    Box::new(random_non_negative_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    ))
}

// -- (Rational, Integer) --

pub fn random_rational_integer_pair_gen(config: &GenConfig) -> It<(Rational, Integer)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, Integer, Integer) --

pub fn random_rational_integer_integer_triple_gen(
    config: &GenConfig,
) -> It<(Rational, Integer, Integer)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, Natural) --

pub fn random_rational_natural_pair_gen(config: &GenConfig) -> It<(Rational, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, Natural, Natural) --

pub fn random_rational_natural_natural_triple_gen(
    config: &GenConfig,
) -> It<(Rational, Natural, Natural)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, PrimitiveInt) --

pub fn random_rational_primitive_int_pair_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// -- (Rational, PrimitiveInt, PrimitiveInt) --

pub fn random_rational_primitive_int_primitive_int_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Rational, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// -- (Rational, PrimitiveSigned) --

pub fn random_rational_signed_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, PrimitiveUnsigned) --

pub fn random_rational_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, Rational) --

pub fn random_rational_pair_gen(config: &GenConfig) -> It<(Rational, Rational)> {
    Box::new(random_pairs_from_single(random_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_rational_pair_gen_var_1(config: &GenConfig) -> It<(Rational, Rational)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_nonzero_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Rational, Rational, Integer) --

pub fn random_rational_rational_integer_triple_gen(
    config: &GenConfig,
) -> It<(Rational, Rational, Integer)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, Rational, Natural) --

pub fn random_rational_rational_natural_triple_gen(
    config: &GenConfig,
) -> It<(Rational, Rational, Natural)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

pub fn random_rational_rational_natural_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, Rational, Natural)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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

// -- (Rational, Rational, Natural, Natural) --

pub fn random_rational_rational_natural_natural_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, Rational, Natural, Natural)> {
    Box::new(random_quadruples_xxyz(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
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
        &|seed| {
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Rational, Rational, PrimitiveInt) --

pub fn random_rational_rational_primitive_int_triple_gen<T: PrimitiveInt>(
    config: &GenConfig,
) -> It<(Rational, Rational, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_primitive_ints,
    ))
}

// -- (Rational, Rational, Rational) --

pub fn random_rational_triple_gen(config: &GenConfig) -> It<(Rational, Rational, Rational)> {
    Box::new(random_triples_from_single(random_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_rational_triple_gen_var_1(config: &GenConfig) -> It<(Rational, Rational, Rational)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_nonzero_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Rational, RoundingMode) --

pub fn random_rational_rounding_mode_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_rationals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, rm)| match rm {
            RoundingMode::Floor | RoundingMode::Up => *x >= 0u32,
            RoundingMode::Exact => Natural::convertible_from(x),
            _ => true,
        }),
    )
}

pub fn random_rational_rounding_mode_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Rational, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_rationals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, rm)| *rm != RoundingMode::Exact || x.is_integer()),
    )
}

pub fn random_rational_rounding_mode_pair_gen_var_3<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>(
    config: &GenConfig,
) -> It<(Rational, RoundingMode)>
where
    Rational: PartialOrd<T>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_rationals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, rm)| match rm {
            RoundingMode::Floor => *x >= T::MIN,
            RoundingMode::Ceiling => *x <= T::MAX,
            RoundingMode::Up => *x >= T::MIN && *x <= T::MAX,
            RoundingMode::Exact => T::convertible_from(x),
            _ => true,
        }),
    )
}

// -- String --

// vars 1 through 10 are in malachite-base.

pub fn random_string_gen_var_11(config: &GenConfig) -> It<String> {
    Box::new(
        random_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|r| serde_json::to_string(&r).unwrap()),
    )
}

pub fn random_string_gen_var_12(config: &GenConfig) -> It<String> {
    Box::new(
        random_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|x| x.to_string()),
    )
}

// -- (String, String, String) --

// vars 1 through 2 are in malachite-nz.

pub fn random_string_triple_gen_var_3(config: &GenConfig) -> It<(String, String, String)> {
    Box::new(
        random_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .map(|x| {
            (
                serde_json::to_string(&rational_to_bigrational(&x)).unwrap(),
                serde_json::to_string(&rational_to_rug_rational(&x)).unwrap(),
                serde_json::to_string(&x).unwrap(),
            )
        }),
    )
}
