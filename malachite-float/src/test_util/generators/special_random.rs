use crate::random::{
    striped_random_finite_floats, striped_random_floats, striped_random_non_negative_finite_floats,
    striped_random_nonzero_finite_floats, striped_random_positive_finite_floats,
};
use crate::test_util::extra_variadic::{
    random_triples, random_triples_from_single, random_triples_xxy, random_triples_xyy,
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
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_signeds,
};
use malachite_base::num::random::random_primitive_floats;
use malachite_base::num::random::striped::{striped_random_signeds, striped_random_unsigneds};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::{GenConfig, It};
use malachite_base::tuples::random::{random_pairs, random_pairs_from_single};
use malachite_nz::integer::random::striped_random_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::random::striped_random_naturals;
use malachite_nz::natural::Natural;
use malachite_q::random::striped_random_rationals;
use malachite_q::Rational;

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
        .filter(|(f, rm)| *rm != RoundingMode::Exact || T::convertible_from(f)),
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
            rm != RoundingMode::Exact
                || *n == 0u32
                || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
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
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || *n == 0u32
                || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
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
        &|seed| random_rounding_modes(seed).filter(|&rm| rm != RoundingMode::Exact),
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
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || *n == T::ZERO
                || n.significant_bits() - n.trailing_zeros() <= prec
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
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || *n == T::ZERO
                || n.significant_bits() - n.trailing_zeros() <= prec
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
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
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
        &|seed| random_rounding_modes(seed).filter(|rm| *rm != RoundingMode::Exact),
    ))
}
