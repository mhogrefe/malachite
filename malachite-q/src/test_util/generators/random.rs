// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::random::{
    random_negative_rationals, random_non_negative_rationals, random_nonzero_rationals,
    random_positive_rationals, random_rationals, RandomRationalsFromDoubleAndSign,
};
use crate::test_util::extra_variadic::{
    random_ordered_unique_triples, random_quadruples_xxyz, random_triples,
    random_triples_from_single, random_triples_xxy, random_triples_xyy,
};
use crate::test_util::generators::round_to_multiple_rational_filter;
use crate::Rational;
use malachite_base::bools::random::random_bools;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::string::options::random::random_to_sci_options;
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsInteger, ToSci};
use malachite_base::num::random::geometric::{
    geometric_random_nonzero_signeds, geometric_random_positive_unsigneds,
    geometric_random_signeds, geometric_random_unsigneds,
};
use malachite_base::num::random::{
    random_primitive_ints, random_unsigned_inclusive_range, special_random_finite_primitive_floats,
    special_random_nonzero_finite_primitive_floats, special_random_primitive_floats,
};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::{GenConfig, It};
use malachite_base::tuples::random::{
    random_ordered_unique_pairs, random_pairs, random_pairs_from_single,
};
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;
use malachite_base::vecs::random::random_vecs;
use malachite_nz::integer::random::random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::random::{
    random_natural_range_to_infinity, random_naturals, random_positive_naturals,
};
use malachite_nz::natural::Natural;
use num::BigRational;
use std::cmp::Ordering::*;
use std::ops::Shr;

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

pub fn random_rational_gen_var_4<T: PrimitiveFloat>(config: &GenConfig) -> It<Rational>
where
    Rational: TryFrom<T>,
{
    Box::new(
        special_random_finite_primitive_floats(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
            config.get_or("special_p_mean_n", 1),
            config.get_or("special_p_mean_d", 64),
        )
        .map(Rational::exact_from),
    )
}

pub fn random_rational_gen_var_5<T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat>(
    config: &GenConfig,
) -> It<Rational> {
    Box::new(
        random_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .filter(|q| !T::convertible_from(q)),
    )
}

pub fn random_rational_gen_var_6<T: PrimitiveFloat>(config: &GenConfig) -> It<Rational>
where
    Rational: TryFrom<T>,
{
    Box::new(
        special_random_nonzero_finite_primitive_floats(
            EXAMPLE_SEED,
            config.get_or("exponent_mean_n", 8),
            config.get_or("exponent_mean_d", 1),
            config.get_or("precision_mean_n", 8),
            config.get_or("precision_mean_d", 1),
        )
        .map(|f| {
            let x = Rational::exact_from(f);
            let y = Rational::exact_from(if f > T::ZERO {
                f.next_lower()
            } else {
                f.next_higher()
            });
            (x + y) >> 1
        }),
    )
}

pub fn random_rational_gen_var_7(config: &GenConfig) -> It<Rational> {
    Box::new(RandomRationalsFromDoubleAndSign {
        bs: random_bools(EXAMPLE_SEED.fork("sign")),
        xs: geometric_random_unsigneds::<u32>(
            EXAMPLE_SEED.fork("numerator"),
            config.get_or("mean_small_n", 64),
            config.get_or("mean_small_d", 1),
        )
        .map(Natural::from),
        ys: geometric_random_positive_unsigneds::<u32>(
            EXAMPLE_SEED.fork("denominator"),
            config.get_or("mean_small_n", 64),
            config.get_or("mean_small_d", 1),
        )
        .map(Natural::from),
    })
}

pub fn random_rational_gen_var_8(config: &GenConfig) -> It<Rational> {
    Box::new(
        random_positive_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        )
        .filter(|x| *x != 1u32),
    )
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

pub fn random_rational_natural_pair_gen_var_1(config: &GenConfig) -> It<(Rational, Natural)> {
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
            random_natural_range_to_infinity(
                seed,
                Natural::TWO,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_rational_natural_pair_gen_var_2(config: &GenConfig) -> It<(Rational, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| RandomRationalsFromDoubleAndSign {
            bs: random_bools(seed.fork("sign")),
            xs: geometric_random_unsigneds::<u32>(
                seed.fork("numerator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
            ys: geometric_random_positive_unsigneds::<u32>(
                seed.fork("denominator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
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

pub fn random_rational_natural_pair_gen_var_3(config: &GenConfig) -> It<(Rational, Natural)> {
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
            random_positive_naturals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn random_rational_natural_pair_gen_var_4(config: &GenConfig) -> It<(Rational, Natural)> {
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
            geometric_random_positive_unsigneds::<u64>(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from)
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

pub fn random_rational_natural_natural_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, Natural, Natural)> {
    Box::new(
        random_triples_xyy(
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
        )
        .filter(|(_, x, y)| x < y),
    )
}

// -- (Rational, PrimitiveFloat) --

pub fn random_rational_primitive_float_pair_gen<T: PrimitiveFloat>(
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

// -- (Rational, PrimitiveFloat, PrimitiveFloat) --

pub fn random_rational_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
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

pub fn random_rational_signed_pair_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
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
            &|seed| {
                geometric_random_signeds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|(x, exp)| *exp >= T::ZERO || *x != 0u32),
    )
}

pub fn random_rational_signed_pair_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, T)>
where
    for<'a> &'a Rational: Shr<T, Output = Rational>,
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
            &|seed| {
                geometric_random_signeds::<T>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|(x, pow)| !x.denominator_ref().is_power_of_2() || !(x >> *pow).is_integer()),
    )
}

pub fn random_rational_signed_pair_gen_var_4<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_non_negative_rationals(
                            seed_2,
                            config.get_or("mean_bits_n", 64),
                            config.get_or("mean_bits_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_nonzero_signeds(
                            seed_2,
                            config.get_or("small_signed_mean_n", 32),
                            config.get_or("small_signed_mean_d", 1),
                        )
                    },
                )
                .filter(|(q, i)| *i > T::ZERO || *q != 0u32)
            },
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_negative_rationals(
                            seed_2,
                            config.get_or("mean_bits_n", 64),
                            config.get_or("mean_bits_d", 1),
                        )
                    },
                    &|seed_2| {
                        geometric_random_signeds::<T>(
                            seed_2,
                            config.get_or("small_signed_mean_n", 32),
                            config.get_or("small_signed_mean_d", 1),
                        )
                        .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE))
                    },
                )
            },
        )
        .map(Union2::unwrap),
    )
}

pub fn random_rational_signed_pair_gen_var_5<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_positive_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_nonzero_signeds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Rational, PrimitiveSigned) --

pub fn random_rational_signed_signed_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, T, T)> {
    Box::new(
        random_triples_xyy(
            EXAMPLE_SEED,
            &|seed| {
                random_rationals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_signeds::<T>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
        )
        .filter(|(x, e, f)| *e >= T::ZERO && *f >= T::ZERO || *x != 0),
    )
}

// -- (Rational, PrimitiveSigned, PrimitiveUnsigned) --

pub fn random_rational_signed_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    config: &GenConfig,
) -> It<(Rational, T, U)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_signeds::<T>(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds::<U>(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

// -- (Rational, PrimitiveSigned, RoundingMode) --

pub fn random_rational_signed_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, i64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                random_rationals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_signeds::<i64>(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, i, rm)| {
            *rm != Exact || x.denominator_ref().is_power_of_2() && (x >> *i).is_integer()
        }),
    )
}

// var 2 is in malachite-float.

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

pub fn random_rational_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| RandomRationalsFromDoubleAndSign {
            bs: random_bools(seed.fork("sign")),
            xs: geometric_random_unsigneds::<u32>(
                seed.fork("numerator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
            ys: geometric_random_positive_unsigneds::<u32>(
                seed.fork("denominator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
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

pub fn random_rational_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>(
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
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
    ))
}

pub fn random_rational_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Rational, T)> {
    Box::new(
        random_union2s(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs(
                    seed,
                    &|seed_2| {
                        random_non_negative_rationals(
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
                        random_negative_rationals(
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

pub fn random_rational_unsigned_pair_gen_var_5(config: &GenConfig) -> It<(Rational, u8)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_unsigned_inclusive_range(seed, 2, 36),
    ))
}

pub fn random_rational_unsigned_pair_gen_var_6(config: &GenConfig) -> It<(Rational, u8)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| RandomRationalsFromDoubleAndSign {
            bs: random_bools(seed.fork("sign")),
            xs: geometric_random_unsigneds::<u32>(
                seed.fork("numerator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
            ys: geometric_random_positive_unsigneds::<u32>(
                seed.fork("denominator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
        },
        &|seed| random_unsigned_inclusive_range(seed, 2, 36),
    ))
}

// -- (Rational, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn random_rational_unsigned_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
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

pub fn random_rational_pair_gen_var_2(config: &GenConfig) -> It<(Rational, Rational)> {
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
            &|seed| {
                random_nonzero_rationals(
                    seed,
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
        )
        .filter(|(x, y)| !(x / y).is_integer()),
    )
}

pub fn random_rational_pair_gen_var_3(config: &GenConfig) -> It<(Rational, Rational)> {
    Box::new(random_ordered_unique_pairs(random_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_rational_pair_gen_var_4(config: &GenConfig) -> It<(Rational, Rational)> {
    // TODO
    Box::new(
        random_pairs_from_single(random_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y)| if x <= y { (x, y) } else { (y, x) }),
    )
}

pub fn random_rational_pair_gen_var_5(config: &GenConfig) -> It<(Rational, Rational)> {
    // TODO
    Box::new(
        random_pairs_from_single(RandomRationalsFromDoubleAndSign {
            bs: random_bools(EXAMPLE_SEED.fork("sign")),
            xs: geometric_random_unsigneds::<u32>(
                EXAMPLE_SEED.fork("numerator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
            ys: geometric_random_positive_unsigneds::<u32>(
                EXAMPLE_SEED.fork("denominator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
        })
        .filter_map(|(x, y)| match x.cmp(&y) {
            Equal => None,
            Less => Some((x, y)),
            Greater => Some((y, x)),
        }),
    )
}

pub fn random_rational_pair_gen_var_6(config: &GenConfig) -> It<(Rational, Rational)> {
    // TODO
    Box::new(
        random_pairs_from_single(RandomRationalsFromDoubleAndSign {
            bs: random_bools(EXAMPLE_SEED.fork("sign")),
            xs: geometric_random_unsigneds::<u32>(
                EXAMPLE_SEED.fork("numerator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
            ys: geometric_random_positive_unsigneds::<u32>(
                EXAMPLE_SEED.fork("denominator"),
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
            .map(Natural::from),
        })
        .map(|(x, y)| if x <= y { (x, y) } else { (y, x) }),
    )
}

pub fn random_rational_pair_gen_var_7(config: &GenConfig) -> It<(Rational, Rational)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_positive_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            random_positive_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
            .filter(move |q| (q - Rational::ONE).gt_abs(&Rational::from_signeds(1, 1000)))
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

// -- (Rational, Rational, PrimitiveFloat) --

pub fn random_rational_rational_primitive_float_triple_gen<T: PrimitiveFloat>(
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

// -- (Rational, Rational, PrimitiveSigned) --

pub fn random_rational_rational_signed_triple_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Rational, Rational, T)> {
    Box::new(
        random_triples_xxy(
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
        )
        .filter(|(x, y, exp)| *exp >= T::ZERO || *x != 0 && *y != 0),
    )
}

// -- (Rational, Rational, PrimitiveUnsigned) --

pub fn random_rational_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
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
        &|seed| {
            geometric_random_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
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

pub fn random_rational_triple_gen_var_2(config: &GenConfig) -> It<(Rational, Rational, Rational)> {
    Box::new(random_ordered_unique_triples(random_rationals(
        EXAMPLE_SEED,
        config.get_or("mean_bits_n", 64),
        config.get_or("mean_bits_d", 1),
    )))
}

pub fn random_rational_triple_gen_var_3(config: &GenConfig) -> It<(Rational, Rational, Rational)> {
    // TODO
    Box::new(
        random_triples_from_single(random_rationals(
            EXAMPLE_SEED,
            config.get_or("mean_bits_n", 64),
            config.get_or("mean_bits_d", 1),
        ))
        .map(|(x, y, z)| {
            let mut xs = vec![x, y, z];
            xs.sort_unstable();
            let mut xs = xs.into_iter();
            (xs.next().unwrap(), xs.next().unwrap(), xs.next().unwrap())
        }),
    )
}

// -- (Rational, Rational, RoundingMode) --

pub fn random_rational_rational_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, Rational, RoundingMode)> {
    Box::new(
        random_triples_xxy(
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
        .filter(round_to_multiple_rational_filter),
    )
}

// -- (Rational, RoundingMode) --

pub fn random_rational_rounding_mode_pair_gen(config: &GenConfig) -> It<(Rational, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

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
            Floor | Up => *x >= 0u32,
            Exact => Natural::convertible_from(x),
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
        .filter(|(x, rm)| *rm != Exact || x.is_integer()),
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
            Floor => *x >= T::MIN,
            Ceiling => *x <= T::MAX,
            Up => *x >= T::MIN && *x <= T::MAX,
            Exact => T::convertible_from(x),
            _ => true,
        }),
    )
}

pub fn random_rational_rounding_mode_pair_gen_var_4(
    config: &GenConfig,
) -> It<(Rational, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            random_nonzero_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn random_rational_rounding_mode_pair_gen_var_5<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat,
>(
    config: &GenConfig,
) -> It<(Rational, RoundingMode)>
where
    Rational: TryFrom<T>,
{
    let max = Rational::exact_from(T::MAX_FINITE);
    let min = -&max;
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
        .filter(move |(x, rm)| match rm {
            Floor => *x >= min,
            Ceiling => *x <= max,
            Up => *x >= min && *x <= max,
            Exact => T::convertible_from(x),
            _ => true,
        }),
    )
}

// -- (Rational, ToSciOptions) --

pub fn random_rational_to_sci_options_pair_gen(config: &GenConfig) -> It<(Rational, ToSciOptions)> {
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
            random_to_sci_options(
                seed,
                config.get_or("small_mean_n", 4),
                config.get_or("small_mean_d", 1),
            )
        },
    ))
}

pub fn random_rational_to_sci_options_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Rational, ToSciOptions)> {
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
                serde_json::to_string(&BigRational::from(&x)).unwrap(),
                serde_json::to_string(&rug::Rational::from(&x)).unwrap(),
                serde_json::to_string(&x).unwrap(),
            )
        }),
    )
}

// -- Vec<Rational> --

pub fn random_rational_vec_gen(config: &GenConfig) -> It<Vec<Rational>> {
    Box::new(random_vecs(
        EXAMPLE_SEED,
        &|seed| {
            random_rationals(
                seed,
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        config.get_or("mean_len_n", 4),
        config.get_or("mean_len_d", 1),
    ))
}
