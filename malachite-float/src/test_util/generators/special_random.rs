// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::random::{
    striped_random_finite_floats, striped_random_floats, striped_random_non_negative_finite_floats,
    striped_random_nonzero_finite_floats, striped_random_positive_finite_floats,
    striped_random_positive_floats_with_precision, RandomPositiveFiniteFloats,
};
use crate::test_util::extra_variadic::{
    random_quadruples, random_quadruples_xxyz, random_triples, random_triples_from_single,
    random_triples_xxy, random_triples_xyy,
};
use crate::test_util::generators::{
    add_prec_round_rational_valid, add_prec_round_valid, add_round_rational_valid, add_round_valid,
    mul_prec_round_rational_valid, mul_prec_round_valid, mul_round_rational_valid, mul_round_valid,
    square_prec_round_valid, square_round_valid, sub_prec_round_rational_valid,
    sub_prec_round_valid, sub_round_rational_valid, sub_round_valid,
};
use crate::test_util::generators::{
    from_primitive_float_prec_round_valid, integer_rounding_from_float_valid,
    natural_rounding_from_float_valid, set_prec_round_valid, signed_rounding_from_float_valid,
    unsigned_rounding_from_float_valid,
};
use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_signeds,
    geometric_random_unsigned_inclusive_range, geometric_random_unsigneds,
};
use malachite_base::num::random::striped::{striped_random_signeds, striped_random_unsigneds};
use malachite_base::num::random::{random_primitive_floats, random_unsigned_inclusive_range};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::{reshape_2_1_to_3, GenConfig, It};
use malachite_base::tuples::random::{random_pairs, random_pairs_from_single};
use malachite_nz::integer::random::striped_random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::random::{striped_random_naturals, StripedRandomNaturalInclusiveRange};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::random::striped_random_rationals;
use malachite_q::Rational;
use std::collections::HashMap;

// -- Float --

pub fn special_random_float_gen(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
        config.get_or("mean_zero_p_n", 1),
        config.get_or("mean_zero_p_d", 64),
    ))
}

pub fn special_random_float_gen_var_1(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_positive_finite_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
    ))
}

pub fn special_random_float_gen_var_2(config: &GenConfig) -> It<Float> {
    Box::new(
        striped_random_floats(
            EXAMPLE_SEED,
            config.get_or("mean_exponent_n", 64),
            config.get_or("mean_exponent_d", 1),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            config.get_or("mean_precision_n", 64),
            config.get_or("mean_precision_d", 1),
            config.get_or("mean_zero_p_n", 1),
            config.get_or("mean_zero_p_d", 64),
        )
        .filter(|f| !f.is_nan()),
    )
}

pub fn special_random_float_gen_var_3(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_nonzero_finite_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
    ))
}

pub fn special_random_float_gen_var_4(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_finite_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
        config.get_or("mean_zero_p_n", 1),
        config.get_or("mean_zero_p_d", 64),
    ))
}

pub fn special_random_float_gen_var_5(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_non_negative_finite_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
        config.get_or("mean_zero_p_n", 1),
        config.get_or("mean_zero_p_d", 64),
    ))
}

pub fn special_random_float_gen_var_6(config: &GenConfig) -> It<Float> {
    striped_random_floats_with_precision_inclusive_range(EXAMPLE_SEED, config, 1, Limb::WIDTH - 1)
}

pub fn special_random_float_gen_var_7(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_positive_floats_with_precision(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        Limb::WIDTH,
    ))
}

pub fn special_random_float_gen_var_8(config: &GenConfig) -> It<Float> {
    striped_random_floats_with_precision_inclusive_range(
        EXAMPLE_SEED,
        config,
        Limb::WIDTH + 1,
        (Limb::WIDTH << 1) - 1,
    )
}

pub fn special_random_float_gen_var_9(config: &GenConfig) -> It<Float> {
    Box::new(striped_random_positive_floats_with_precision(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        Limb::WIDTH << 1,
    ))
}

pub fn special_random_float_gen_var_10(config: &GenConfig) -> It<Float> {
    striped_random_floats_with_precision_inclusive_range(
        EXAMPLE_SEED,
        config,
        (Limb::WIDTH << 1) + 1,
        Limb::WIDTH * 3 - 1,
    )
}

struct StripedRandomFloatsWithPrecisionUniformInclusiveRange {
    seed: Seed,
    mean_exponent_n: u64,
    mean_exponent_d: u64,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    precisions: Box<dyn Iterator<Item = u64>>,
    floats: HashMap<u64, RandomPositiveFiniteFloats<StripedRandomNaturalInclusiveRange>>,
}

impl Iterator for StripedRandomFloatsWithPrecisionUniformInclusiveRange {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        let prec = self.precisions.next().unwrap();
        let xs = self
            .floats
            .entry(prec)
            .or_insert(striped_random_positive_floats_with_precision(
                self.seed.fork(&prec.to_string()),
                self.mean_exponent_n,
                self.mean_exponent_d,
                self.mean_stripe_n,
                self.mean_stripe_d,
                prec,
            ));
        Some(xs.next().unwrap())
    }
}

fn striped_random_floats_with_precision_inclusive_range(
    seed: Seed,
    config: &GenConfig,
    prec_lo: u64,
    prec_hi: u64,
) -> It<Float> {
    Box::new(StripedRandomFloatsWithPrecisionUniformInclusiveRange {
        seed: seed.fork("floats"),
        mean_exponent_n: config.get_or("mean_exponent_n", 64),
        mean_exponent_d: config.get_or("mean_exponent_d", 1),
        mean_stripe_n: config.get_or("mean_stripe_n", 64),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        precisions: Box::new(random_unsigned_inclusive_range(
            seed.fork("precisions"),
            prec_lo,
            prec_hi,
        )),
        floats: HashMap::new(),
    })
}

struct StripedRandomFloatPairsWithPrecisionUniformInclusiveRange {
    seed: Seed,
    mean_exponent_n: u64,
    mean_exponent_d: u64,
    mean_stripe_n: u64,
    mean_stripe_d: u64,
    precisions: Box<dyn Iterator<Item = u64>>,
    floats: HashMap<u64, RandomPositiveFiniteFloats<StripedRandomNaturalInclusiveRange>>,
}

impl Iterator for StripedRandomFloatPairsWithPrecisionUniformInclusiveRange {
    type Item = (Float, Float);

    fn next(&mut self) -> Option<(Float, Float)> {
        let prec = self.precisions.next().unwrap();
        let xs = self
            .floats
            .entry(prec)
            .or_insert(striped_random_positive_floats_with_precision(
                self.seed.fork(&prec.to_string()),
                self.mean_exponent_n,
                self.mean_exponent_d,
                self.mean_stripe_n,
                self.mean_stripe_d,
                prec,
            ));
        Some((xs.next().unwrap(), xs.next().unwrap()))
    }
}

fn striped_random_float_pairs_with_precision_inclusive_range(
    seed: Seed,
    config: &GenConfig,
    prec_lo: u64,
    prec_hi: u64,
) -> It<(Float, Float)> {
    Box::new(StripedRandomFloatPairsWithPrecisionUniformInclusiveRange {
        seed: seed.fork("floats"),
        mean_exponent_n: config.get_or("mean_exponent_n", 64),
        mean_exponent_d: config.get_or("mean_exponent_d", 1),
        mean_stripe_n: config.get_or("mean_stripe_n", 64),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        precisions: Box::new(random_unsigned_inclusive_range(
            seed.fork("precisions"),
            prec_lo,
            prec_hi,
        )),
        floats: HashMap::new(),
    })
}

fn striped_random_float_pairs_with_precision_inclusive_range_to_infinity(
    seed: Seed,
    config: &GenConfig,
    prec_lo: u64,
) -> It<(Float, Float)> {
    let mean_precision = Rational::from_unsigneds(
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 64),
    ) + Rational::from(prec_lo);
    let (n, d) = mean_precision.into_numerator_and_denominator();
    Box::new(StripedRandomFloatPairsWithPrecisionUniformInclusiveRange {
        seed: seed.fork("floats"),
        mean_exponent_n: config.get_or("mean_exponent_n", 64),
        mean_exponent_d: config.get_or("mean_exponent_d", 1),
        mean_stripe_n: config.get_or("mean_stripe_n", 64),
        mean_stripe_d: config.get_or("mean_stripe_d", 1),
        precisions: Box::new(geometric_random_unsigned_inclusive_range(
            seed.fork("precisions"),
            prec_lo,
            u64::MAX,
            u64::exact_from(&n),
            u64::exact_from(&d),
        )),
        floats: HashMap::new(),
    })
}

// -- (Float, Float) --

pub fn special_random_float_pair_gen(config: &GenConfig) -> It<(Float, Float)> {
    Box::new(random_pairs_from_single(striped_random_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
        config.get_or("mean_zero_p_n", 1),
        config.get_or("mean_zero_p_d", 64),
    )))
}

pub fn special_random_float_pair_gen_var_1(config: &GenConfig) -> It<(Float, Float)> {
    Box::new(random_pairs_from_single(striped_random_finite_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
        config.get_or("mean_zero_p_n", 1),
        config.get_or("mean_zero_p_d", 64),
    )))
}

pub fn special_random_float_pair_gen_var_2(config: &GenConfig) -> It<(Float, Float)> {
    striped_random_float_pairs_with_precision_inclusive_range(
        EXAMPLE_SEED,
        config,
        1,
        Limb::WIDTH - 1,
    )
}

pub fn special_random_float_pair_gen_var_3(config: &GenConfig) -> It<(Float, Float)> {
    Box::new(random_pairs_from_single(
        striped_random_positive_floats_with_precision(
            EXAMPLE_SEED,
            config.get_or("mean_exponent_n", 64),
            config.get_or("mean_exponent_d", 1),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            Limb::WIDTH,
        ),
    ))
}

pub fn special_random_float_pair_gen_var_4(config: &GenConfig) -> It<(Float, Float)> {
    striped_random_float_pairs_with_precision_inclusive_range(
        EXAMPLE_SEED,
        config,
        Limb::WIDTH + 1,
        (Limb::WIDTH << 1) - 1,
    )
}

pub fn special_random_float_pair_gen_var_5(config: &GenConfig) -> It<(Float, Float)> {
    Box::new(random_pairs_from_single(
        striped_random_positive_floats_with_precision(
            EXAMPLE_SEED,
            config.get_or("mean_exponent_n", 64),
            config.get_or("mean_exponent_d", 1),
            config.get_or("mean_stripe_n", 32),
            config.get_or("mean_stripe_d", 1),
            Limb::WIDTH << 1,
        ),
    ))
}

pub fn special_random_float_pair_gen_var_6(config: &GenConfig) -> It<(Float, Float)> {
    striped_random_float_pairs_with_precision_inclusive_range(
        EXAMPLE_SEED,
        config,
        (Limb::WIDTH << 1) + 1,
        Limb::WIDTH * 3 - 1,
    )
}

pub fn special_random_float_pair_gen_var_7(config: &GenConfig) -> It<(Float, Float)> {
    striped_random_float_pairs_with_precision_inclusive_range_to_infinity(
        EXAMPLE_SEED,
        config,
        Limb::WIDTH * 3,
    )
}

// -- (Float, Float, Float) --

pub fn special_random_float_triple_gen(config: &GenConfig) -> It<(Float, Float, Float)> {
    Box::new(random_triples_from_single(striped_random_floats(
        EXAMPLE_SEED,
        config.get_or("mean_exponent_n", 64),
        config.get_or("mean_exponent_d", 1),
        config.get_or("mean_stripe_n", 32),
        config.get_or("mean_stripe_d", 1),
        config.get_or("mean_precision_n", 64),
        config.get_or("mean_precision_d", 1),
        config.get_or("mean_zero_p_n", 1),
        config.get_or("mean_zero_p_d", 64),
    )))
}

// -- (Float, Float, Integer) --

pub fn special_random_float_float_integer_triple_gen(
    config: &GenConfig,
) -> It<(Float, Float, Integer)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Float, Natural) --

pub fn special_random_float_float_natural_triple_gen(
    config: &GenConfig,
) -> It<(Float, Float, Natural)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, PrimitiveFloat, PrimitiveFloat) --

pub fn special_random_float_float_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Float, Float, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &random_primitive_floats,
    ))
}

// -- (Float, Float, PrimitiveSigned) --

pub fn special_random_float_float_signed_triple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Float, Float, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Float, Float, PrimitiveUnsigned) --

pub fn special_random_float_float_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, Float, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_float_float_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, Float, T)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
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

// -- (Float, Float, PrimitiveUnsigned, Rational) --

pub fn special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        random_quadruples_xxyz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, prec, rm)| add_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        random_quadruples_xxyz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, prec, rm)| sub_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        random_quadruples_xxyz(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, prec, rm)| mul_prec_round_valid(x, y, *prec, *rm)),
    )
}

// -- (Float, Float, Rational) --

pub fn special_random_float_float_rational_triple_gen(
    config: &GenConfig,
) -> It<(Float, Float, Rational)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Float, RoundingMode) --

pub fn special_random_float_float_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    Box::new(random_triples_xxy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|&rm| rm != Exact),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    1,
                    Limb::WIDTH - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| add_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_5(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH,
                ))
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| add_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_6(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    Limb::WIDTH + 1,
                    (Limb::WIDTH << 1) - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| add_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_7(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH << 1,
                ))
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| add_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_8(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    (Limb::WIDTH << 1) + 1,
                    Limb::WIDTH * 3 - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| add_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_9(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range_to_infinity(
                    seed,
                    config,
                    Limb::WIDTH * 3,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| add_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_10(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    1,
                    Limb::WIDTH - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| sub_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_11(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH,
                ))
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| sub_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_12(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    Limb::WIDTH + 1,
                    (Limb::WIDTH << 1) - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| sub_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_13(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH << 1,
                ))
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| sub_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_14(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    (Limb::WIDTH << 1) + 1,
                    Limb::WIDTH * 3 - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| sub_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_15(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range_to_infinity(
                    seed,
                    config,
                    Limb::WIDTH * 3,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| sub_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_16(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    Box::new(
        random_triples_xxy(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_17(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    1,
                    Limb::WIDTH - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| mul_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_18(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH,
                ))
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| mul_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_19(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    Limb::WIDTH + 1,
                    (Limb::WIDTH << 1) - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| mul_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_20(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                random_pairs_from_single(striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH << 1,
                ))
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| mul_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_21(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range(
                    seed,
                    config,
                    (Limb::WIDTH << 1) + 1,
                    Limb::WIDTH * 3 - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| mul_round_valid(x, y, *rm)),
    ))
}

pub fn special_random_float_float_rounding_mode_triple_gen_var_22(
    config: &GenConfig,
) -> It<(Float, Float, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_float_pairs_with_precision_inclusive_range_to_infinity(
                    seed,
                    config,
                    Limb::WIDTH * 3,
                )
            },
            &random_rounding_modes,
        )
        .filter(|((x, y), rm)| mul_round_valid(x, y, *rm)),
    ))
}

// -- (Float, Integer) --

pub fn special_random_float_integer_pair_gen(config: &GenConfig) -> It<(Float, Integer)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_float_integer_pair_gen_var_1(config: &GenConfig) -> It<(Float, Integer)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_finite_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Integer, Integer) --

pub fn special_random_float_integer_integer_triple_gen(
    config: &GenConfig,
) -> It<(Float, Integer, Integer)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Natural) --

pub fn special_random_float_natural_pair_gen(config: &GenConfig) -> It<(Float, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_float_natural_pair_gen_var_1(config: &GenConfig) -> It<(Float, Natural)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_finite_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Natural, Natural) --

pub fn special_random_float_natural_natural_triple_gen(
    config: &GenConfig,
) -> It<(Float, Natural, Natural)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, PrimitiveFloat) --

pub fn special_random_float_primitive_float_pair_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &random_primitive_floats,
    ))
}

// -- (Float, PrimitiveFloat, PrimitiveFloat) --

pub fn special_random_float_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Float, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &random_primitive_floats,
    ))
}

// -- (Float, PrimitiveSigned) --

pub fn special_random_float_signed_pair_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// TODO use ranges
pub fn special_random_float_signed_pair_gen_var_1<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_positive_finite_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
            )
            .map(|mut x| {
                if let Float(Finite { exponent, .. }) = &mut x {
                    *exponent = 1;
                }
                x
            })
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

pub fn special_random_float_signed_pair_gen_var_2<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
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

// -- (Float, PrimitiveSigned, PrimitiveSigned) --

pub fn special_random_float_signed_signed_triple_gen<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Float, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_signeds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Float, PrimitiveUnsigned) --

pub fn special_random_float_unsigned_pair_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

pub fn special_random_float_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
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

pub fn special_random_float_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, T)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
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

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn special_random_float_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, T, T)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_unsigneds(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
    ))
}

// -- (Float, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_float_unsigned_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Float, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref x, p, rm)| set_prec_round_valid(x, p, rm)),
    )
}

pub fn special_random_float_unsigned_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Float, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref x, p, rm)| square_prec_round_valid(x, p, rm)),
    )
}

// -- (Float, Rational) --

pub fn special_random_float_rational_pair_gen(config: &GenConfig) -> It<(Float, Rational)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

pub fn special_random_float_rational_pair_gen_var_1(config: &GenConfig) -> It<(Float, Rational)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_finite_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Rational, PrimitiveUnsigned) --

pub fn special_random_float_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, Rational, T)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
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

// -- (Float, Rational, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_1(
    config: &GenConfig,
) -> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        random_quadruples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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
            &random_rounding_modes,
        )
        .filter(|(x, y, prec, rm)| add_prec_round_rational_valid(x, y, *prec, *rm)),
    )
}

pub fn special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_2(
    config: &GenConfig,
) -> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        random_quadruples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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
            &random_rounding_modes,
        )
        .filter(|(x, y, prec, rm)| sub_prec_round_rational_valid(x, y, *prec, *rm)),
    )
}

pub fn special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_3(
    config: &GenConfig,
) -> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        random_quadruples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
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
            &random_rounding_modes,
        )
        .filter(|(x, y, prec, rm)| mul_prec_round_rational_valid(x, y, *prec, *rm)),
    )
}

// -- (Float, Rational, Rational) --

pub fn special_random_float_rational_rational_triple_gen(
    config: &GenConfig,
) -> It<(Float, Rational, Rational)> {
    Box::new(random_triples_xyy(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
    ))
}

// -- (Float, Rational, RoundingMode) --

pub fn special_random_float_rational_rounding_mode_triple_gen_var_1(
    config: &GenConfig,
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, rm)| add_round_rational_valid(x, y, *rm)),
    )
}

pub fn special_random_float_rational_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, rm)| sub_round_rational_valid(x, y, *rm)),
    )
}

pub fn special_random_float_rational_rounding_mode_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|&rm| rm != Exact),
    ))
}

pub fn special_random_float_rational_rounding_mode_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(x, y, rm)| mul_round_rational_valid(x, y, *rm)),
    )
}

// -- (Float, RoundingMode) --

pub fn special_random_float_rounding_mode_pair_gen(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
                config.get_or("mean_zero_p_n", 1),
                config.get_or("mean_zero_p_d", 64),
            )
        },
        &random_rounding_modes,
    ))
}

pub fn special_random_float_rounding_mode_pair_gen_var_1(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| natural_rounding_from_float_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_2(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| integer_rounding_from_float_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_3(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(random_pairs(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_nonzero_finite_floats(
                seed,
                config.get_or("mean_exponent_n", 64),
                config.get_or("mean_exponent_d", 1),
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_precision_n", 64),
                config.get_or("mean_precision_d", 1),
            )
        },
        &random_rounding_modes,
    ))
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn special_random_float_rounding_mode_pair_gen_var_4<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| unsigned_rounding_from_float_valid::<T>(f, *rm)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn special_random_float_rounding_mode_pair_gen_var_5<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| signed_rounding_from_float_valid::<T>(f, *rm)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn special_random_float_rounding_mode_pair_gen_var_6<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(Float, RoundingMode)>
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| *rm != Exact || T::convertible_from(f)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_7(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_precision_n", 64),
                    config.get_or("mean_precision_d", 1),
                    config.get_or("mean_zero_p_n", 1),
                    config.get_or("mean_zero_p_d", 64),
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_8(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats_with_precision_inclusive_range(
                    seed,
                    config,
                    1,
                    Limb::WIDTH - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_9(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH,
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_10(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats_with_precision_inclusive_range(
                    seed,
                    config,
                    Limb::WIDTH + 1,
                    (Limb::WIDTH << 1) - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_11(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_positive_floats_with_precision(
                    seed,
                    config.get_or("mean_exponent_n", 64),
                    config.get_or("mean_exponent_d", 1),
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    Limb::WIDTH << 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn special_random_float_rounding_mode_pair_gen_var_12(
    config: &GenConfig,
) -> It<(Float, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_floats_with_precision_inclusive_range(
                    seed,
                    config,
                    (Limb::WIDTH << 1) + 1,
                    Limb::WIDTH * 3 - 1,
                )
            },
            &random_rounding_modes,
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-nz.

pub fn special_random_integer_unsigned_rounding_mode_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Integer, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_integers(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, prec, rm)| {
            rm != Exact || *n == 0u32 || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
        }),
    )
}

pub fn special_random_integer_unsigned_rounding_mode_triple_gen_var_4(
    config: &GenConfig,
) -> It<(Integer, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_integers(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != Exact),
    ))
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

// var 1 is in malachite-nz

pub fn special_random_natural_unsigned_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Natural, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_naturals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, prec, rm)| {
            rm != Exact || *n == 0u32 || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
        }),
    )
}

pub fn special_random_natural_unsigned_rounding_mode_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Natural, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_naturals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|&rm| rm != Exact),
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-base.

pub fn special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)>
where
    Float: From<T>,
{
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &random_primitive_floats,
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("mean_small_n", 64),
                    config.get_or("mean_small_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(x, p, rm)| from_primitive_float_prec_round_valid(x, p, rm)),
    )
}

pub fn special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveFloat>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)>
where
    Float: From<T>,
{
    Box::new(random_triples(
        EXAMPLE_SEED,
        &random_primitive_floats,
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("mean_small_n", 64),
                config.get_or("mean_small_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != Exact),
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-base.

pub fn special_random_signed_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_signeds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, prec, rm)| {
            rm != Exact || *n == T::ZERO || n.significant_bits() - n.trailing_zeros() <= prec
        }),
    )
}

pub fn special_random_signed_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_signeds::<T>(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != Exact),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 4 are in malachite-base.

pub fn special_random_unsigned_unsigned_rounding_mode_triple_gen_var_5<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_unsigneds::<T>(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, prec, rm)| {
            rm != Exact || *n == T::ZERO || n.significant_bits() - n.trailing_zeros() <= prec
        }),
    )
}

pub fn special_random_unsigned_unsigned_rounding_mode_triple_gen_var_6<T: PrimitiveUnsigned>(
    config: &GenConfig,
) -> It<(T, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_unsigneds::<T>(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != Exact),
    ))
}

// -- (Rational, PrimitiveUnsigned, RoundingMode) --

pub fn special_random_rational_unsigned_rounding_mode_triple_gen_var_2(
    config: &GenConfig,
) -> It<(Rational, u64, RoundingMode)> {
    Box::new(
        random_triples(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &|seed| {
                geometric_random_positive_unsigneds(
                    seed,
                    config.get_or("small_unsigned_mean_n", 4),
                    config.get_or("small_unsigned_mean_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, prec, rm)| {
            rm != Exact
                || n.denominator_ref().is_power_of_2()
                    && n.numerator_ref().significant_bits() <= prec
        }),
    )
}

pub fn special_random_rational_unsigned_rounding_mode_triple_gen_var_3(
    config: &GenConfig,
) -> It<(Rational, u64, RoundingMode)> {
    Box::new(random_triples(
        EXAMPLE_SEED,
        &|seed| {
            striped_random_rationals(
                seed,
                config.get_or("mean_stripe_n", 32),
                config.get_or("mean_stripe_d", 1),
                config.get_or("mean_bits_n", 64),
                config.get_or("mean_bits_d", 1),
            )
        },
        &|seed| {
            geometric_random_positive_unsigneds(
                seed,
                config.get_or("small_unsigned_mean_n", 4),
                config.get_or("small_unsigned_mean_d", 1),
            )
        },
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != Exact),
    ))
}

// -- (Rational, RoundingMode) --

// vars 1 through 5 are in malachite-q.

pub fn special_random_rational_rounding_mode_pair_gen_var_6(
    config: &GenConfig,
) -> It<(Rational, RoundingMode)> {
    Box::new(
        random_pairs(
            EXAMPLE_SEED,
            &|seed| {
                striped_random_rationals(
                    seed,
                    config.get_or("mean_stripe_n", 32),
                    config.get_or("mean_stripe_d", 1),
                    config.get_or("mean_bits_n", 64),
                    config.get_or("mean_bits_d", 1),
                )
            },
            &random_rounding_modes,
        )
        .filter(|&(ref n, rm)| {
            rm != Exact
                || n.denominator_ref().is_power_of_2() && n.numerator_ref().significant_bits() <= 1
        }),
    )
}
