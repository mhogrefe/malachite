// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::generators::common::{
    float_float_anything_rounding_mode_quadruple_rm, float_float_anything_triple_rm,
    float_float_rounding_mode_triple_rm, float_integer_pair_rm, float_natural_pair_rm,
    float_pair_rm, float_primitive_float_pair_rm, float_primitive_int_pair_rm,
    float_rational_anything_rounding_mode_quadruple_rm, float_rational_anything_triple_rm,
    float_rational_pair_rm, float_rational_rounding_mode_triple_rm, float_rm,
    float_rounding_mode_pair_rm, float_t_rounding_mode_triple_rm,
};
use crate::test_util::generators::exhaustive::*;
use crate::test_util::generators::random::*;
use crate::test_util::generators::special_random::*;
use crate::Float;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::Generator;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// -- Float --

pub fn float_gen() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen,
        &random_float_gen,
        &special_random_float_gen,
    )
}

pub fn float_gen_rm() -> Generator<(rug::Float, Float)> {
    Generator::new(
        &|| float_rm(exhaustive_float_gen()),
        &|config| float_rm(random_float_gen(config)),
        &|config| float_rm(special_random_float_gen(config)),
    )
}

// All positive finite floats (not including positive zero).
pub fn float_gen_var_1() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_1,
        &random_float_gen_var_1,
        &special_random_float_gen_var_1,
    )
}

// All floats except NaN.
pub fn float_gen_var_2() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_2,
        &random_float_gen_var_2,
        &special_random_float_gen_var_2,
    )
}

pub fn float_gen_var_2_rm() -> Generator<(rug::Float, Float)> {
    Generator::new(
        &|| float_rm(exhaustive_float_gen_var_2()),
        &|config| float_rm(random_float_gen_var_2(config)),
        &|config| float_rm(special_random_float_gen_var_2(config)),
    )
}

// All nonzero finite floats.
pub fn float_gen_var_3() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_3,
        &random_float_gen_var_3,
        &special_random_float_gen_var_3,
    )
}

// All finite floats.
pub fn float_gen_var_4() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_4,
        &random_float_gen_var_4,
        &special_random_float_gen_var_4,
    )
}

// All non-negative finite floats.
pub fn float_gen_var_5() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_5,
        &random_float_gen_var_5,
        &special_random_float_gen_var_5,
    )
}

// All floats with a precision less than `Limb::WIDTH`.
pub fn float_gen_var_6() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_6,
        &random_float_gen_var_6,
        &special_random_float_gen_var_6,
    )
}

// All floats with precision `Limb::WIDTH`.
pub fn float_gen_var_7() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_7,
        &random_float_gen_var_7,
        &special_random_float_gen_var_7,
    )
}

// All floats with a precision greater than `Limb::WIDTH` and less than `Limb::WIDTH` * 2.
pub fn float_gen_var_8() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_8,
        &random_float_gen_var_8,
        &special_random_float_gen_var_8,
    )
}

// All floats with precision `Limb::WIDTH` * 2.
pub fn float_gen_var_9() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_9,
        &random_float_gen_var_9,
        &special_random_float_gen_var_9,
    )
}

// All floats with a precision greater than `Limb::WIDTH` * 2 and less than `Limb::WIDTH` * 3.
pub fn float_gen_var_10() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_10,
        &random_float_gen_var_10,
        &special_random_float_gen_var_10,
    )
}

// All floats with a precision greater than `Limb::WIDTH` * 2.
pub fn float_gen_var_11() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_11,
        &random_float_gen_var_11,
        &special_random_float_gen_var_11,
    )
}

// Extreme floats.
pub fn float_gen_var_12() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_12,
        &random_float_gen_var_12,
        &special_random_float_gen_var_12,
    )
}

// Extreme nonzero finite floats.
pub fn float_gen_var_13() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_13,
        &random_float_gen_var_13,
        &special_random_float_gen_var_13,
    )
}

// All extreme floats except NaN.
pub fn float_gen_var_14() -> Generator<Float> {
    Generator::new(
        &exhaustive_float_gen_var_14,
        &random_float_gen_var_14,
        &special_random_float_gen_var_14,
    )
}

// -- (Float, Float) --

pub fn float_pair_gen() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen,
        &random_float_pair_gen,
        &special_random_float_pair_gen,
    )
}

pub fn float_pair_gen_rm() -> Generator<((rug::Float, rug::Float), (Float, Float))> {
    Generator::new(
        &|| float_pair_rm(exhaustive_float_pair_gen()),
        &|config| float_pair_rm(random_float_pair_gen(config)),
        &|config| float_pair_rm(special_random_float_pair_gen(config)),
    )
}

// All pairs of finite floats.
pub fn float_pair_gen_var_1() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_1,
        &random_float_pair_gen_var_1,
        &special_random_float_pair_gen_var_1,
    )
}

// All pairs of positive floats with the same precision, which is greater than zero and less than
// `Limb::WIDTH`.
pub fn float_pair_gen_var_2() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_2,
        &random_float_pair_gen_var_2,
        &special_random_float_pair_gen_var_2,
    )
}

// All pairs of positive floats with the same precision, which is `Limb::WIDTH`.
pub fn float_pair_gen_var_3() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_3,
        &random_float_pair_gen_var_3,
        &special_random_float_pair_gen_var_3,
    )
}

// All pairs of positive floats with the same precision, which is greater than `Limb::WIDTH` and
// less than twice `Limb::WIDTH`.
pub fn float_pair_gen_var_4() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_4,
        &random_float_pair_gen_var_4,
        &special_random_float_pair_gen_var_4,
    )
}

// All pairs of positive floats with the same precision, which is twice `Limb::WIDTH`.
pub fn float_pair_gen_var_5() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_5,
        &random_float_pair_gen_var_5,
        &special_random_float_pair_gen_var_5,
    )
}

// All pairs of positive floats with the same precision, which is greater than twice `Limb::WIDTH`
// and less than three times `Limb::WIDTH`.
pub fn float_pair_gen_var_6() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_6,
        &random_float_pair_gen_var_6,
        &special_random_float_pair_gen_var_6,
    )
}

// All pairs of positive floats with the same precision, which is greater than or equal to three
// times `Limb::WIDTH`.
pub fn float_pair_gen_var_7() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_7,
        &random_float_pair_gen_var_7,
        &special_random_float_pair_gen_var_7,
    )
}

// All pairs of positive floats with different precisions, such that the precision of the second
// float is less than or equal to `Limb::WIDTH`.
pub fn float_pair_gen_var_8() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_8,
        &random_float_pair_gen_var_8,
        &special_random_float_pair_gen_var_8,
    )
}

// All pairs of positive floats with different precisions, such that the precision of the second
// float is greater than `Limb::WIDTH`.
pub fn float_pair_gen_var_9() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_9,
        &random_float_pair_gen_var_9,
        &special_random_float_pair_gen_var_9,
    )
}

// All pairs of floats, some of which are extreme.
pub fn float_pair_gen_var_10() -> Generator<(Float, Float)> {
    Generator::new(
        &exhaustive_float_pair_gen_var_10,
        &random_float_pair_gen_var_10,
        &special_random_float_pair_gen,
    )
}

// -- (Float, Float, Float) --

pub fn float_triple_gen() -> Generator<(Float, Float, Float)> {
    Generator::new(
        &exhaustive_float_triple_gen,
        &random_float_triple_gen,
        &special_random_float_triple_gen,
    )
}

// -- (Float, Float, Integer) --

pub fn float_float_integer_triple_gen() -> Generator<(Float, Float, Integer)> {
    Generator::new(
        &exhaustive_float_float_integer_triple_gen,
        &random_float_float_integer_triple_gen,
        &special_random_float_float_integer_triple_gen,
    )
}

// -- (Float, Float, Natural) --

pub fn float_float_natural_triple_gen() -> Generator<(Float, Float, Natural)> {
    Generator::new(
        &exhaustive_float_float_natural_triple_gen,
        &random_float_float_natural_triple_gen,
        &special_random_float_float_natural_triple_gen,
    )
}

// -- (Float, Float, PrimitiveFloat) --

pub fn float_float_primitive_float_triple_gen<T: PrimitiveFloat>() -> Generator<(Float, Float, T)> {
    Generator::new(
        &exhaustive_float_float_primitive_float_triple_gen,
        &random_float_float_primitive_float_triple_gen,
        &special_random_float_float_primitive_float_triple_gen,
    )
}

// -- (Float, Float, PrimitiveSigned) --

pub fn float_float_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Float, Float, T)> {
    Generator::new(
        &exhaustive_float_float_signed_triple_gen,
        &random_float_float_primitive_int_triple_gen,
        &special_random_float_float_signed_triple_gen,
    )
}

// -- (Float, Float, PrimitiveUnsigned) --

pub fn float_float_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Float, Float, T)> {
    Generator::new(
        &exhaustive_float_float_unsigned_triple_gen,
        &random_float_float_primitive_int_triple_gen,
        &special_random_float_float_unsigned_triple_gen,
    )
}

// All triples of `(Float, Float, T)`, where `T` is unsigned, small, and positive.
pub fn float_float_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Float, Float, T)>
{
    Generator::new(
        &exhaustive_float_float_unsigned_triple_gen_var_1,
        &random_float_float_unsigned_triple_gen_var_1,
        &special_random_float_float_unsigned_triple_gen_var_1,
    )
}

pub fn float_float_unsigned_triple_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Float, rug::Float, T), (Float, Float, T))> {
    Generator::new(
        &|| float_float_anything_triple_rm(exhaustive_float_float_unsigned_triple_gen_var_1()),
        &|config| {
            float_float_anything_triple_rm(random_float_float_unsigned_triple_gen_var_1(config))
        },
        &|config| {
            float_float_anything_triple_rm(special_random_float_float_unsigned_triple_gen_var_1(
                config,
            ))
        },
    )
}

// All triples of `(Float, Float, T)`, where the `Float`s may be extreme and the `T` is unsigned,
// small, and positive.
pub fn float_float_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Float, Float, T)>
{
    Generator::new(
        &exhaustive_float_float_unsigned_triple_gen_var_2,
        &random_float_float_unsigned_triple_gen_var_2,
        &special_random_float_float_unsigned_triple_gen_var_2,
    )
}

// -- (Float, Float, PrimitiveUnsigned, RoundingMode) --

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::add_prec_round`.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_1(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_1,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_1,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_1,
    )
}

pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_1_rm() -> Generator<(
    (rug::Float, rug::Float, u64, rug::float::Round),
    (Float, Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_anything_rounding_mode_quadruple_rm(
                exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_1(),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                random_float_float_unsigned_rounding_mode_quadruple_gen_var_1(config),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_1(config),
            )
        },
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::sub_prec_round`.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_2(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_2,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_2,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_2,
    )
}

pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_2_rm() -> Generator<(
    (rug::Float, rug::Float, u64, rug::float::Round),
    (Float, Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_anything_rounding_mode_quadruple_rm(
                exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_2(),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                random_float_float_unsigned_rounding_mode_quadruple_gen_var_2(config),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_2(config),
            )
        },
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::mul_prec_round`.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_3(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_3,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_3,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_3,
    )
}

pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_3_rm() -> Generator<(
    (rug::Float, rug::Float, u64, rug::float::Round),
    (Float, Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_anything_rounding_mode_quadruple_rm(
                exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_3(),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                random_float_float_unsigned_rounding_mode_quadruple_gen_var_3(config),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::div_prec_round`.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_4(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_4,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_4,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_4,
    )
}

pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_4_rm() -> Generator<(
    (rug::Float, rug::Float, u64, rug::float::Round),
    (Float, Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_anything_rounding_mode_quadruple_rm(
                exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_4(),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                random_float_float_unsigned_rounding_mode_quadruple_gen_var_4(config),
            )
        },
        &|config| {
            float_float_anything_rounding_mode_quadruple_rm(
                special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_4(config),
            )
        },
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::add_prec_round`, where
// the `Float`s may be extreme.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_5(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_5,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_5,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_5,
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::sub_prec_round`, where
// the `Float`s may be extreme.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_6(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_6,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_6,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_6,
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::mul_prec_round`, where
// the `Float`s may be extreme.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_7(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_7,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_7,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_7,
    )
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to `Float::div_prec_round`, where
// the `Float`s may be extreme.
pub fn float_float_unsigned_rounding_mode_quadruple_gen_var_8(
) -> Generator<(Float, Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_8,
        &random_float_float_unsigned_rounding_mode_quadruple_gen_var_8,
        &special_random_float_float_unsigned_rounding_mode_quadruple_gen_var_8,
    )
}

// -- (Float, Float, Rational) --

pub fn float_float_rational_triple_gen() -> Generator<(Float, Float, Rational)> {
    Generator::new(
        &exhaustive_float_float_rational_triple_gen,
        &random_float_float_rational_triple_gen,
        &special_random_float_float_rational_triple_gen,
    )
}

// -- (Float, Float, RoundingMode) --

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`.
pub fn float_float_rounding_mode_triple_gen_var_1() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_1,
        &random_float_float_rounding_mode_triple_gen_var_1,
        &special_random_float_float_rounding_mode_triple_gen_var_1,
    )
}

pub fn float_float_rounding_mode_triple_gen_var_1_rm() -> Generator<(
    (rug::Float, rug::Float, rug::float::Round),
    (Float, Float, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_rounding_mode_triple_rm(
                exhaustive_float_float_rounding_mode_triple_gen_var_3(),
            )
        },
        &|config| {
            float_float_rounding_mode_triple_rm(random_float_float_rounding_mode_triple_gen_var_3(
                config,
            ))
        },
        &|config| {
            float_float_rounding_mode_triple_rm(
                special_random_float_float_rounding_mode_triple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`.
pub fn float_float_rounding_mode_triple_gen_var_2() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_2,
        &random_float_float_rounding_mode_triple_gen_var_2,
        &special_random_float_float_rounding_mode_triple_gen_var_2,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round` or
// `Float::sub_round`, excluding those with Exact.
pub fn float_float_rounding_mode_triple_gen_var_3_rm() -> Generator<(
    (rug::Float, rug::Float, rug::float::Round),
    (Float, Float, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_rounding_mode_triple_rm(
                exhaustive_float_float_rounding_mode_triple_gen_var_3(),
            )
        },
        &|config| {
            float_float_rounding_mode_triple_rm(random_float_float_rounding_mode_triple_gen_var_3(
                config,
            ))
        },
        &|config| {
            float_float_rounding_mode_triple_rm(
                special_random_float_float_rounding_mode_triple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, where the
// `Float`s have the same precision, which is greater than zero and less than `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_4() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_4,
        &random_float_float_rounding_mode_triple_gen_var_4,
        &special_random_float_float_rounding_mode_triple_gen_var_4,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, where the
// `Float`s have the same precision, which is `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_5() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_5,
        &random_float_float_rounding_mode_triple_gen_var_5,
        &special_random_float_float_rounding_mode_triple_gen_var_5,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, where the
// `Float`s have the same precision, which is greater than `Limb::WIDTH` and less than twice
// `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_6() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_6,
        &random_float_float_rounding_mode_triple_gen_var_6,
        &special_random_float_float_rounding_mode_triple_gen_var_6,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, where the
// `Float`s have the same precision, which is twice `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_7() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_7,
        &random_float_float_rounding_mode_triple_gen_var_7,
        &special_random_float_float_rounding_mode_triple_gen_var_7,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, where the
// `Float`s have the same precision, which is greater than twice `Limb::WIDTH` and less than three
// times `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_8() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_8,
        &random_float_float_rounding_mode_triple_gen_var_8,
        &special_random_float_float_rounding_mode_triple_gen_var_8,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, where the
// `Float`s have the same precision, which is greater than or equal to three times `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_9() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_9,
        &random_float_float_rounding_mode_triple_gen_var_9,
        &special_random_float_float_rounding_mode_triple_gen_var_9,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, where the
// `Float`s have the same precision, which is greater than zero and less than `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_10() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_10,
        &random_float_float_rounding_mode_triple_gen_var_10,
        &special_random_float_float_rounding_mode_triple_gen_var_10,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, where the
// `Float`s have the same precision, which is `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_11() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_11,
        &random_float_float_rounding_mode_triple_gen_var_11,
        &special_random_float_float_rounding_mode_triple_gen_var_11,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, where the
// `Float`s have the same precision, which is greater than `Limb::WIDTH` and less than twice
// `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_12() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_12,
        &random_float_float_rounding_mode_triple_gen_var_12,
        &special_random_float_float_rounding_mode_triple_gen_var_12,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, where the
// `Float`s have the same precision, which is twice `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_13() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_13,
        &random_float_float_rounding_mode_triple_gen_var_13,
        &special_random_float_float_rounding_mode_triple_gen_var_13,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, where the
// `Float`s have the same precision, which is greater than twice `Limb::WIDTH` and less than three
// times `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_14() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_14,
        &random_float_float_rounding_mode_triple_gen_var_14,
        &special_random_float_float_rounding_mode_triple_gen_var_14,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, where the
// `Float`s have the same precision, which is greater than or equal to three times `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_15() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_15,
        &random_float_float_rounding_mode_triple_gen_var_15,
        &special_random_float_float_rounding_mode_triple_gen_var_15,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`.
pub fn float_float_rounding_mode_triple_gen_var_16() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_16,
        &random_float_float_rounding_mode_triple_gen_var_16,
        &special_random_float_float_rounding_mode_triple_gen_var_16,
    )
}

pub fn float_float_rounding_mode_triple_gen_var_16_rm() -> Generator<(
    (rug::Float, rug::Float, rug::float::Round),
    (Float, Float, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_rounding_mode_triple_rm(
                exhaustive_float_float_rounding_mode_triple_gen_var_16(),
            )
        },
        &|config| {
            float_float_rounding_mode_triple_rm(random_float_float_rounding_mode_triple_gen_var_16(
                config,
            ))
        },
        &|config| {
            float_float_rounding_mode_triple_rm(
                special_random_float_float_rounding_mode_triple_gen_var_16(config),
            )
        },
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, where the
// `Float`s have the same precision, which is greater than zero and less than `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_17() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_17,
        &random_float_float_rounding_mode_triple_gen_var_17,
        &special_random_float_float_rounding_mode_triple_gen_var_17,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, where the
// `Float`s have the same precision, which is `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_18() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_18,
        &random_float_float_rounding_mode_triple_gen_var_18,
        &special_random_float_float_rounding_mode_triple_gen_var_18,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, where the
// `Float`s have the same precision, which is greater than `Limb::WIDTH` and less than twice
// `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_19() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_19,
        &random_float_float_rounding_mode_triple_gen_var_19,
        &special_random_float_float_rounding_mode_triple_gen_var_19,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, where the
// `Float`s have the same precision, which is twice `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_20() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_20,
        &random_float_float_rounding_mode_triple_gen_var_20,
        &special_random_float_float_rounding_mode_triple_gen_var_20,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, where the
// `Float`s have the same precision, which is greater than twice `Limb::WIDTH` and less than three
// times `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_21() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_21,
        &random_float_float_rounding_mode_triple_gen_var_21,
        &special_random_float_float_rounding_mode_triple_gen_var_21,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, where the
// `Float`s have the same precision, which is greater than or equal to three times `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_22() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_22,
        &random_float_float_rounding_mode_triple_gen_var_22,
        &special_random_float_float_rounding_mode_triple_gen_var_22,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`.
pub fn float_float_rounding_mode_triple_gen_var_23() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_23,
        &random_float_float_rounding_mode_triple_gen_var_23,
        &special_random_float_float_rounding_mode_triple_gen_var_23,
    )
}

pub fn float_float_rounding_mode_triple_gen_var_23_rm() -> Generator<(
    (rug::Float, rug::Float, rug::float::Round),
    (Float, Float, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_float_rounding_mode_triple_rm(
                exhaustive_float_float_rounding_mode_triple_gen_var_23(),
            )
        },
        &|config| {
            float_float_rounding_mode_triple_rm(random_float_float_rounding_mode_triple_gen_var_23(
                config,
            ))
        },
        &|config| {
            float_float_rounding_mode_triple_rm(
                special_random_float_float_rounding_mode_triple_gen_var_23(config),
            )
        },
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`, where the
// `Float`s have the same precision, which is greater than zero and less than `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_24() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_24,
        &random_float_float_rounding_mode_triple_gen_var_24,
        &special_random_float_float_rounding_mode_triple_gen_var_24,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`, where the
// `Float`s have the same precision, which is `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_25() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_25,
        &random_float_float_rounding_mode_triple_gen_var_25,
        &special_random_float_float_rounding_mode_triple_gen_var_25,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`, where the
// `Float`s have the same precision, which is greater than `Limb::WIDTH` and less than twice
// `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_26() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_26,
        &random_float_float_rounding_mode_triple_gen_var_26,
        &special_random_float_float_rounding_mode_triple_gen_var_26,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`, where the
// `Float`s have different precisions and that the precision of the second float is less than or
// equal to `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_27() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_27,
        &random_float_float_rounding_mode_triple_gen_var_27,
        &special_random_float_float_rounding_mode_triple_gen_var_27,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`, where the
// `Float`s have different precisions and that the precision of the second float is greater than
// `Limb::WIDTH`.
pub fn float_float_rounding_mode_triple_gen_var_28() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_28,
        &random_float_float_rounding_mode_triple_gen_var_28,
        &special_random_float_float_rounding_mode_triple_gen_var_28,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::add_round`, and the `Float`s
// may be extreme.
pub fn float_float_rounding_mode_triple_gen_var_29() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_29,
        &random_float_float_rounding_mode_triple_gen_var_29,
        &special_random_float_float_rounding_mode_triple_gen_var_29,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::sub_round`, and the `Float`s
// may be extreme.
pub fn float_float_rounding_mode_triple_gen_var_30() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_30,
        &random_float_float_rounding_mode_triple_gen_var_30,
        &special_random_float_float_rounding_mode_triple_gen_var_30,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::mul_round`, and the `Float`s
// may be extreme.
pub fn float_float_rounding_mode_triple_gen_var_31() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_31,
        &random_float_float_rounding_mode_triple_gen_var_31,
        &special_random_float_float_rounding_mode_triple_gen_var_31,
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float::div_round`, and the `Float`s
// may be extreme.
pub fn float_float_rounding_mode_triple_gen_var_32() -> Generator<(Float, Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_float_rounding_mode_triple_gen_var_32,
        &random_float_float_rounding_mode_triple_gen_var_32,
        &special_random_float_float_rounding_mode_triple_gen_var_32,
    )
}

// -- (Float, Integer) --

pub fn float_integer_pair_gen() -> Generator<(Float, Integer)> {
    Generator::new(
        &exhaustive_float_integer_pair_gen,
        &random_float_integer_pair_gen,
        &special_random_float_integer_pair_gen,
    )
}

pub fn float_integer_pair_gen_rm() -> Generator<((rug::Float, rug::Integer), (Float, Integer))> {
    Generator::new(
        &|| float_integer_pair_rm(exhaustive_float_integer_pair_gen()),
        &|config| float_integer_pair_rm(random_float_integer_pair_gen(config)),
        &|config| float_integer_pair_rm(special_random_float_integer_pair_gen(config)),
    )
}

// All pairs of finite Floats and Integers.
pub fn float_integer_pair_gen_var_1() -> Generator<(Float, Integer)> {
    Generator::new(
        &exhaustive_float_integer_pair_gen_var_1,
        &random_float_integer_pair_gen_var_1,
        &special_random_float_integer_pair_gen_var_1,
    )
}

// All pairs of Floats and Integers, where the Float is extreme.
pub fn float_integer_pair_gen_var_2() -> Generator<(Float, Integer)> {
    Generator::new(
        &exhaustive_float_integer_pair_gen_var_2,
        &random_float_integer_pair_gen_var_2,
        &special_random_float_integer_pair_gen_var_2,
    )
}

// -- (Float, Integer, Integer) --

pub fn float_integer_integer_triple_gen() -> Generator<(Float, Integer, Integer)> {
    Generator::new(
        &exhaustive_float_integer_integer_triple_gen,
        &random_float_integer_integer_triple_gen,
        &special_random_float_integer_integer_triple_gen,
    )
}

// -- (Float, Natural) --

pub fn float_natural_pair_gen() -> Generator<(Float, Natural)> {
    Generator::new(
        &exhaustive_float_natural_pair_gen,
        &random_float_natural_pair_gen,
        &special_random_float_natural_pair_gen,
    )
}

pub fn float_natural_pair_gen_rm() -> Generator<((rug::Float, rug::Integer), (Float, Natural))> {
    Generator::new(
        &|| float_natural_pair_rm(exhaustive_float_natural_pair_gen()),
        &|config| float_natural_pair_rm(random_float_natural_pair_gen(config)),
        &|config| float_natural_pair_rm(special_random_float_natural_pair_gen(config)),
    )
}

// All pairs of finite Floats and Naturals.
pub fn float_natural_pair_gen_var_1() -> Generator<(Float, Natural)> {
    Generator::new(
        &exhaustive_float_natural_pair_gen_var_1,
        &random_float_natural_pair_gen_var_1,
        &special_random_float_natural_pair_gen_var_1,
    )
}

// All pairs of Floats and Naturals, where the Float is extreme.
pub fn float_natural_pair_gen_var_2() -> Generator<(Float, Natural)> {
    Generator::new(
        &exhaustive_float_natural_pair_gen_var_2,
        &random_float_natural_pair_gen_var_2,
        &special_random_float_natural_pair_gen_var_2,
    )
}

// -- (Float, Natural, Natural) --

pub fn float_natural_natural_triple_gen() -> Generator<(Float, Natural, Natural)> {
    Generator::new(
        &exhaustive_float_natural_natural_triple_gen,
        &random_float_natural_natural_triple_gen,
        &special_random_float_natural_natural_triple_gen,
    )
}

// -- (Float, PrimitiveFloat) --

pub fn float_primitive_float_pair_gen<T: PrimitiveFloat>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_primitive_float_pair_gen,
        &random_float_primitive_float_pair_gen,
        &special_random_float_primitive_float_pair_gen,
    )
}

pub fn float_primitive_float_pair_gen_rm<T: PrimitiveFloat>(
) -> Generator<((rug::Float, T), (Float, T))> {
    Generator::new(
        &|| float_primitive_float_pair_rm(exhaustive_float_primitive_float_pair_gen()),
        &|config| float_primitive_float_pair_rm(random_float_primitive_float_pair_gen(config)),
        &|config| {
            float_primitive_float_pair_rm(special_random_float_primitive_float_pair_gen(config))
        },
    )
}

// All `(Float, T)` where the Float is extreme and `T` is a primitive float.
pub fn float_primitive_float_pair_gen_var_1<T: PrimitiveFloat>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_primitive_float_pair_gen_var_1,
        &random_float_primitive_float_pair_gen_var_1,
        &special_random_float_primitive_float_pair_gen_var_1,
    )
}

// -- (Float, PrimitiveFloat, PrimitiveFloat) --

pub fn float_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Float, T, T)> {
    Generator::new(
        &exhaustive_float_primitive_float_primitive_float_triple_gen,
        &random_float_primitive_float_primitive_float_triple_gen,
        &special_random_float_primitive_float_primitive_float_triple_gen,
    )
}

// -- (Float, PrimitiveSigned) --

pub fn float_signed_pair_gen<T: PrimitiveSigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_signed_pair_gen,
        &random_float_signed_pair_gen,
        &special_random_float_signed_pair_gen,
    )
}

pub fn float_signed_pair_gen_rm<T: PrimitiveSigned>() -> Generator<((rug::Float, T), (Float, T))> {
    Generator::new(
        &|| float_primitive_int_pair_rm(exhaustive_float_signed_pair_gen()),
        &|config| float_primitive_int_pair_rm(random_float_signed_pair_gen(config)),
        &|config| float_primitive_int_pair_rm(special_random_float_signed_pair_gen(config)),
    )
}

// All `(Float, T)` where `T` is signed and small and the `Float` is in the range [1.0, 2.0).
pub fn float_signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_signed_pair_gen_var_1,
        &random_float_signed_pair_gen_var_1,
        &special_random_float_signed_pair_gen_var_1,
    )
}

// All `(Float, T)` where `T` is small and signed.
pub fn float_signed_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_signed_pair_gen_var_2,
        &random_float_signed_pair_gen_var_2,
        &special_random_float_signed_pair_gen_var_2,
    )
}

pub fn float_signed_pair_gen_var_2_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Float, T), (Float, T))> {
    Generator::new(
        &|| float_primitive_int_pair_rm(exhaustive_float_signed_pair_gen_var_2()),
        &|config| float_primitive_int_pair_rm(random_float_signed_pair_gen_var_2(config)),
        &|config| float_primitive_int_pair_rm(special_random_float_signed_pair_gen_var_2(config)),
    )
}

// All `(Float, T)` where the Float is extreme and `T` is small and signed.
pub fn float_signed_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_signed_pair_gen_var_3,
        &random_float_signed_pair_gen_var_3,
        &special_random_float_signed_pair_gen_var_3,
    )
}

// All `(Float, T)` where the Float is extreme and `T` is signed.
pub fn float_signed_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_signed_pair_gen_var_4,
        &random_float_signed_pair_gen_var_4,
        &special_random_float_signed_pair_gen_var_4,
    )
}

// -- (Float, PrimitiveSigned, PrimitiveSigned) --

pub fn float_signed_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Float, T, T)> {
    Generator::new(
        &exhaustive_float_signed_signed_triple_gen,
        &random_float_primitive_int_primitive_int_triple_gen,
        &special_random_float_signed_signed_triple_gen,
    )
}

// -- (Float, PrimitiveSigned, RoundingMode) --

// All `(Float, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `Float::shl_round`.
pub fn float_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_signed_rounding_mode_triple_gen_var_1,
        &random_float_signed_rounding_mode_triple_gen_var_1,
        &special_random_float_signed_rounding_mode_triple_gen_var_1,
    )
}

// All `(Float, T, RoundingMode)` where `T` is signed, the `Float` is extreme, and the triple is a
// valid input to `Float::shl_round`.
pub fn float_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_signed_rounding_mode_triple_gen_var_2,
        &random_float_signed_rounding_mode_triple_gen_var_2,
        &special_random_float_signed_rounding_mode_triple_gen_var_2,
    )
}

// All `(Float, T, RoundingMode)` where `T` is signed, the triple is a valid input to
// `Float::shl_round`, and the `RoundingMode` is not `Exact`.
pub fn float_signed_rounding_mode_triple_gen_var_3<T: PrimitiveSigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_signed_rounding_mode_triple_gen_var_3,
        &random_float_signed_rounding_mode_triple_gen_var_3,
        &special_random_float_signed_rounding_mode_triple_gen_var_3,
    )
}

pub fn float_signed_rounding_mode_triple_gen_var_3_rm() -> Generator<(
    (rug::Float, i32, rug::float::Round),
    (Float, i32, RoundingMode),
)> {
    Generator::new(
        &|| {
            let ts = exhaustive_float_signed_rounding_mode_triple_gen_var_3();
            float_t_rounding_mode_triple_rm(ts)
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_signed_rounding_mode_triple_gen_var_3(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_signed_rounding_mode_triple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `Float::shr_round`.
pub fn float_signed_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_signed_rounding_mode_triple_gen_var_4,
        &random_float_signed_rounding_mode_triple_gen_var_4,
        &special_random_float_signed_rounding_mode_triple_gen_var_4,
    )
}

// All `(Float, T, RoundingMode)` where `T` is signed, the `Float` is extreme, and the triple is a
// valid input to `Float::shr_round`.
pub fn float_signed_rounding_mode_triple_gen_var_5<T: PrimitiveSigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_signed_rounding_mode_triple_gen_var_5,
        &random_float_signed_rounding_mode_triple_gen_var_5,
        &special_random_float_signed_rounding_mode_triple_gen_var_5,
    )
}

// All `(Float, T, RoundingMode)` where `T` is signed, the triple is a valid input to
// `Float::shr_round`, and the `RoundingMode` is not `Exact`.
pub fn float_signed_rounding_mode_triple_gen_var_6<T: PrimitiveSigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_signed_rounding_mode_triple_gen_var_6,
        &random_float_signed_rounding_mode_triple_gen_var_6,
        &special_random_float_signed_rounding_mode_triple_gen_var_6,
    )
}

pub fn float_signed_rounding_mode_triple_gen_var_6_rm() -> Generator<(
    (rug::Float, i32, rug::float::Round),
    (Float, i32, RoundingMode),
)> {
    Generator::new(
        &|| {
            let ts = exhaustive_float_signed_rounding_mode_triple_gen_var_6();
            float_t_rounding_mode_triple_rm(ts)
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_signed_rounding_mode_triple_gen_var_6(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_signed_rounding_mode_triple_gen_var_6(config),
            )
        },
    )
}

// -- (Float, PrimitiveUnsigned) --

pub fn float_unsigned_pair_gen<T: PrimitiveUnsigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_unsigned_pair_gen,
        &random_float_unsigned_pair_gen,
        &special_random_float_unsigned_pair_gen,
    )
}

type GT<T> = Generator<((rug::Float, T), (Float, T))>;
pub fn float_unsigned_pair_gen_rm<T: PrimitiveUnsigned>() -> GT<T> {
    Generator::new(
        &|| float_primitive_int_pair_rm(exhaustive_float_unsigned_pair_gen()),
        &|config| float_primitive_int_pair_rm(random_float_unsigned_pair_gen(config)),
        &|config| float_primitive_int_pair_rm(special_random_float_unsigned_pair_gen(config)),
    )
}

// All `(Float, T)` where `T` is unsigned, small, and positive.
pub fn float_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_unsigned_pair_gen_var_1,
        &random_float_unsigned_pair_gen_var_1,
        &special_random_float_unsigned_pair_gen_var_1,
    )
}

pub fn float_unsigned_pair_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Float, T), (Float, T))> {
    Generator::new(
        &|| float_primitive_int_pair_rm(exhaustive_float_unsigned_pair_gen_var_1()),
        &|config| float_primitive_int_pair_rm(random_float_unsigned_pair_gen_var_1(config)),
        &|config| float_primitive_int_pair_rm(special_random_float_unsigned_pair_gen_var_1(config)),
    )
}

// All `(Float, T)` where `T` is small and unsigned.
pub fn float_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_unsigned_pair_gen_var_2,
        &random_float_unsigned_pair_gen_var_2,
        &special_random_float_unsigned_pair_gen_var_2,
    )
}

pub fn float_unsigned_pair_gen_var_2_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Float, T), (Float, T))> {
    Generator::new(
        &|| float_primitive_int_pair_rm(exhaustive_float_unsigned_pair_gen_var_2()),
        &|config| float_primitive_int_pair_rm(random_float_unsigned_pair_gen_var_2(config)),
        &|config| float_primitive_int_pair_rm(special_random_float_unsigned_pair_gen_var_2(config)),
    )
}

// All `(Float, T)` where the Float is extreme and `T` is small and unsigned.
pub fn float_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_unsigned_pair_gen_var_3,
        &random_float_unsigned_pair_gen_var_3,
        &special_random_float_unsigned_pair_gen_var_3,
    )
}

// All `(Float, T)` where the `Float` is extreme and the `T` is unsigned, small, and positive.
pub fn float_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_unsigned_pair_gen_var_4,
        &random_float_unsigned_pair_gen_var_4,
        &special_random_float_unsigned_pair_gen_var_4,
    )
}

// All `(Float, T)` where the Float is extreme and `T` is unsigned.
pub fn float_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Float, T)> {
    Generator::new(
        &exhaustive_float_unsigned_pair_gen_var_5,
        &random_float_unsigned_pair_gen_var_5,
        &special_random_float_unsigned_pair_gen_var_5,
    )
}

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn float_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Float, T, T)> {
    Generator::new(
        &exhaustive_float_unsigned_unsigned_triple_gen,
        &random_float_primitive_int_primitive_int_triple_gen,
        &special_random_float_unsigned_unsigned_triple_gen,
    )
}

// -- (Float, PrimitiveUnsigned, RoundingMode) --

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.set_prec_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_1() -> Generator<(Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_1,
        &random_float_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_1,
    )
}

pub fn float_unsigned_rounding_mode_triple_gen_var_1_rm() -> Generator<(
    (rug::Float, u64, rug::float::Round),
    (Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_t_rounding_mode_triple_rm(
                exhaustive_float_unsigned_rounding_mode_triple_gen_var_1(),
            )
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_unsigned_rounding_mode_triple_gen_var_1(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_unsigned_rounding_mode_triple_gen_var_1(config),
            )
        },
    )
}

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.square_prec_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_2() -> Generator<(Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_2,
        &random_float_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_2,
    )
}

pub fn float_unsigned_rounding_mode_triple_gen_var_2_rm() -> Generator<(
    (rug::Float, u64, rug::float::Round),
    (Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_t_rounding_mode_triple_rm(
                exhaustive_float_unsigned_rounding_mode_triple_gen_var_2(),
            )
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_unsigned_rounding_mode_triple_gen_var_2(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_unsigned_rounding_mode_triple_gen_var_2(config),
            )
        },
    )
}

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.reciprocal_prec_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_3() -> Generator<(Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_3,
        &random_float_unsigned_rounding_mode_triple_gen_var_3,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_3,
    )
}

pub fn float_unsigned_rounding_mode_triple_gen_var_3_rm() -> Generator<(
    (rug::Float, u64, rug::float::Round),
    (Float, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_t_rounding_mode_triple_rm(
                exhaustive_float_unsigned_rounding_mode_triple_gen_var_3(),
            )
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_unsigned_rounding_mode_triple_gen_var_3(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_unsigned_rounding_mode_triple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.set_prec_round`, and the
// `Float`s are extreme.
pub fn float_unsigned_rounding_mode_triple_gen_var_4() -> Generator<(Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_4,
        &random_float_unsigned_rounding_mode_triple_gen_var_4,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_4,
    )
}

// All `(Float, T, RoundingMode)` where `T` is unsigned and the triple is a valid input to
// `Float::shl_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_5,
        &random_float_unsigned_rounding_mode_triple_gen_var_5,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_5,
    )
}

// All `(Float, T, RoundingMode)` where `T` is unsigned, the `Float` is extreme, and the triple is a
// valid input to `Float::shl_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_6,
        &random_float_unsigned_rounding_mode_triple_gen_var_6,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_6,
    )
}

// All `(Float, T, RoundingMode)` where `T` is unsigned, the triple is a valid input to
// `Float::shl_round`, and the `RoundingMode` is not `Exact`.
pub fn float_unsigned_rounding_mode_triple_gen_var_7<T: PrimitiveUnsigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_7,
        &random_float_unsigned_rounding_mode_triple_gen_var_7,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_7,
    )
}

pub fn float_unsigned_rounding_mode_triple_gen_var_7_rm() -> Generator<(
    (rug::Float, u32, rug::float::Round),
    (Float, u32, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_t_rounding_mode_triple_rm(
                exhaustive_float_unsigned_rounding_mode_triple_gen_var_7(),
            )
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_unsigned_rounding_mode_triple_gen_var_7(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_unsigned_rounding_mode_triple_gen_var_7(config),
            )
        },
    )
}

// All `(Float, T, RoundingMode)` where `T` is unsigned and the triple is a valid input to
// `Float::shr_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_8<T: PrimitiveUnsigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_8,
        &random_float_unsigned_rounding_mode_triple_gen_var_8,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_8,
    )
}

// All `(Float, T, RoundingMode)` where `T` is unsigned, the `Float` is extreme, and the triple is a
// valid input to `Float::shr_round`.
pub fn float_unsigned_rounding_mode_triple_gen_var_9<T: PrimitiveUnsigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_9,
        &random_float_unsigned_rounding_mode_triple_gen_var_9,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_9,
    )
}

// All `(Float, T, RoundingMode)` where `T` is unsigned, the triple is a valid input to
// `Float::shr_round`, and the `RoundingMode` is not `Exact`.
pub fn float_unsigned_rounding_mode_triple_gen_var_10<T: PrimitiveUnsigned>(
) -> Generator<(Float, T, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_10,
        &random_float_unsigned_rounding_mode_triple_gen_var_10,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_10,
    )
}

pub fn float_unsigned_rounding_mode_triple_gen_var_10_rm() -> Generator<(
    (rug::Float, u32, rug::float::Round),
    (Float, u32, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_t_rounding_mode_triple_rm(
                exhaustive_float_unsigned_rounding_mode_triple_gen_var_10(),
            )
        },
        &|config| {
            float_t_rounding_mode_triple_rm(random_float_unsigned_rounding_mode_triple_gen_var_10(
                config,
            ))
        },
        &|config| {
            float_t_rounding_mode_triple_rm(
                special_random_float_unsigned_rounding_mode_triple_gen_var_10(config),
            )
        },
    )
}

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.square_prec_round`, where the
// `Float` is extreme.
pub fn float_unsigned_rounding_mode_triple_gen_var_11() -> Generator<(Float, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_unsigned_rounding_mode_triple_gen_var_11,
        &random_float_unsigned_rounding_mode_triple_gen_var_11,
        &special_random_float_unsigned_rounding_mode_triple_gen_var_11,
    )
}

// -- (Float, Rational) --

pub fn float_rational_pair_gen() -> Generator<(Float, Rational)> {
    Generator::new(
        &exhaustive_float_rational_pair_gen,
        &random_float_rational_pair_gen,
        &special_random_float_rational_pair_gen,
    )
}

pub fn float_rational_pair_gen_rm() -> Generator<((rug::Float, rug::Rational), (Float, Rational))> {
    Generator::new(
        &|| float_rational_pair_rm(exhaustive_float_rational_pair_gen()),
        &|config| float_rational_pair_rm(random_float_rational_pair_gen(config)),
        &|config| float_rational_pair_rm(special_random_float_rational_pair_gen(config)),
    )
}

// All pairs of finite Floats and Rationals.
pub fn float_rational_pair_gen_var_1() -> Generator<(Float, Rational)> {
    Generator::new(
        &exhaustive_float_rational_pair_gen_var_1,
        &random_float_rational_pair_gen_var_1,
        &special_random_float_rational_pair_gen_var_1,
    )
}

// All pairs of Floats and Rationals where the Float is extreme.
pub fn float_rational_pair_gen_var_2() -> Generator<(Float, Rational)> {
    Generator::new(
        &exhaustive_float_rational_pair_gen_var_2,
        &random_float_rational_pair_gen_var_2,
        &special_random_float_rational_pair_gen_var_2,
    )
}

// -- (Float, Rational, PrimitiveUnsigned) --

// All `(Float, Rational, T)` where `T` is unsigned, small, and positive.
pub fn float_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Float, Rational, T)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_triple_gen_var_1,
        &random_float_rational_unsigned_triple_gen_var_1,
        &special_random_float_rational_unsigned_triple_gen_var_1,
    )
}

pub fn float_rational_unsigned_triple_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Float, rug::Rational, T), (Float, Rational, T))> {
    Generator::new(
        &|| {
            let ef = exhaustive_float_rational_unsigned_triple_gen_var_1();
            float_rational_anything_triple_rm(ef)
        },
        &|config| {
            float_rational_anything_triple_rm(random_float_rational_unsigned_triple_gen_var_1(
                config,
            ))
        },
        &|config| {
            float_rational_anything_triple_rm(
                special_random_float_rational_unsigned_triple_gen_var_1(config),
            )
        },
    )
}

// All `(Float, Rational, T)` where the `Float` is extreme and the `T` is unsigned, small, and
// positive.
pub fn float_rational_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Float, Rational, T)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_triple_gen_var_2,
        &random_float_rational_unsigned_triple_gen_var_2,
        &special_random_float_rational_unsigned_triple_gen_var_2,
    )
}

// -- (Float, Rational, PrimitiveUnsigned, RoundingMode) --

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::add_rational_prec_round`.
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_1(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
    )
}

pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_1_rm() -> Generator<(
    (rug::Float, rug::Rational, u64, rug::float::Round),
    (Float, Rational, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_anything_rounding_mode_quadruple_rm(
                exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_1(),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                random_float_rational_unsigned_rounding_mode_quadruple_gen_var_1(config),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_1(config),
            )
        },
    )
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::sub_rational_prec_round`.
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_2(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
    )
}

pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_2_rm() -> Generator<(
    (rug::Float, rug::Rational, u64, rug::float::Round),
    (Float, Rational, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_anything_rounding_mode_quadruple_rm(
                exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_2(),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                random_float_rational_unsigned_rounding_mode_quadruple_gen_var_2(config),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_2(config),
            )
        },
    )
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::mul_rational_prec_round`.
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_3(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_3,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_3,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_3,
    )
}

pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_3_rm() -> Generator<(
    (rug::Float, rug::Rational, u64, rug::float::Round),
    (Float, Rational, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_anything_rounding_mode_quadruple_rm(
                exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_3(),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                random_float_rational_unsigned_rounding_mode_quadruple_gen_var_3(config),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::div_prec_round_rational`.
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_4(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_4,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_4,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_4,
    )
}

pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_4_rm() -> Generator<(
    (rug::Float, rug::Rational, u64, rug::float::Round),
    (Float, Rational, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_anything_rounding_mode_quadruple_rm(
                exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_4(),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                random_float_rational_unsigned_rounding_mode_quadruple_gen_var_4(config),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_4(config),
            )
        },
    )
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::rational_div_float_prec_round` (with the first two arguments reversed).
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_5(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_5,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_5,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_5,
    )
}

pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_5_rm() -> Generator<(
    (rug::Float, rug::Rational, u64, rug::float::Round),
    (Float, Rational, u64, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_anything_rounding_mode_quadruple_rm(
                exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_5(),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                random_float_rational_unsigned_rounding_mode_quadruple_gen_var_5(config),
            )
        },
        &|config| {
            float_rational_anything_rounding_mode_quadruple_rm(
                special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_5(config),
            )
        },
    )
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::add_rational_prec_round`, where the `Float` is extreme.
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_6(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_6,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_6,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_6,
    )
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::sub_rational_prec_round`, where the `Float` is extreme.
pub fn float_rational_unsigned_rounding_mode_quadruple_gen_var_7(
) -> Generator<(Float, Rational, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_7,
        &random_float_rational_unsigned_rounding_mode_quadruple_gen_var_7,
        &special_random_float_rational_unsigned_rounding_mode_quadruple_gen_var_7,
    )
}

// -- (Float, Rational, Rational) --

pub fn float_rational_rational_triple_gen() -> Generator<(Float, Rational, Rational)> {
    Generator::new(
        &exhaustive_float_rational_rational_triple_gen,
        &random_float_rational_rational_triple_gen,
        &special_random_float_rational_rational_triple_gen,
    )
}

// -- (Float, Rational, RoundingMode) --

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::add_round_rational`.
pub fn float_rational_rounding_mode_triple_gen_var_1() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_1,
        &random_float_rational_rounding_mode_triple_gen_var_1,
        &special_random_float_rational_rounding_mode_triple_gen_var_1,
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::sub_round_rational`.
pub fn float_rational_rounding_mode_triple_gen_var_2() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_2,
        &random_float_rational_rounding_mode_triple_gen_var_2,
        &special_random_float_rational_rounding_mode_triple_gen_var_2,
    )
}

// All `(Float, Rational, RoundingMode)`, excluding those with `Exact`.
pub fn float_rational_rounding_mode_triple_gen_var_3_rm() -> Generator<(
    (rug::Float, rug::Rational, rug::float::Round),
    (Float, Rational, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_rounding_mode_triple_rm(
                exhaustive_float_rational_rounding_mode_triple_gen_var_3(),
            )
        },
        &|config| {
            float_rational_rounding_mode_triple_rm(
                random_float_rational_rounding_mode_triple_gen_var_3(config),
            )
        },
        &|config| {
            float_rational_rounding_mode_triple_rm(
                special_random_float_rational_rounding_mode_triple_gen_var_3(config),
            )
        },
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::mul_round_rational`.
pub fn float_rational_rounding_mode_triple_gen_var_4() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_4,
        &random_float_rational_rounding_mode_triple_gen_var_4,
        &special_random_float_rational_rounding_mode_triple_gen_var_4,
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::div_round_rational`.
pub fn float_rational_rounding_mode_triple_gen_var_5() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_5,
        &random_float_rational_rounding_mode_triple_gen_var_5,
        &special_random_float_rational_rounding_mode_triple_gen_var_5,
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::rational_div_float_round`
// (with the first two arguments reversed).
pub fn float_rational_rounding_mode_triple_gen_var_6() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_6,
        &random_float_rational_rounding_mode_triple_gen_var_6,
        &special_random_float_rational_rounding_mode_triple_gen_var_6,
    )
}

pub fn float_rational_rounding_mode_triple_gen_var_6_rm() -> Generator<(
    (rug::Float, rug::Rational, rug::float::Round),
    (Float, Rational, RoundingMode),
)> {
    Generator::new(
        &|| {
            float_rational_rounding_mode_triple_rm(
                exhaustive_float_rational_rounding_mode_triple_gen_var_6(),
            )
        },
        &|config| {
            float_rational_rounding_mode_triple_rm(
                random_float_rational_rounding_mode_triple_gen_var_6(config),
            )
        },
        &|config| {
            float_rational_rounding_mode_triple_rm(
                special_random_float_rational_rounding_mode_triple_gen_var_6(config),
            )
        },
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::add_round_rational`, where
// the `Float` is extreme.
pub fn float_rational_rounding_mode_triple_gen_var_7() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_7,
        &random_float_rational_rounding_mode_triple_gen_var_7,
        &special_random_float_rational_rounding_mode_triple_gen_var_7,
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to `Float::sub_round_rational`, where
// the `Float` is extreme.
pub fn float_rational_rounding_mode_triple_gen_var_8() -> Generator<(Float, Rational, RoundingMode)>
{
    Generator::new(
        &exhaustive_float_rational_rounding_mode_triple_gen_var_8,
        &random_float_rational_rounding_mode_triple_gen_var_8,
        &special_random_float_rational_rounding_mode_triple_gen_var_8,
    )
}

// -- (Float, RoundingMode) --

pub fn float_rounding_mode_pair_gen() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen,
        &random_float_rounding_mode_pair_gen,
        &special_random_float_rounding_mode_pair_gen,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `Natural::rounding_from`.
pub fn float_rounding_mode_pair_gen_var_1() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_1,
        &random_float_rounding_mode_pair_gen_var_1,
        &special_random_float_rounding_mode_pair_gen_var_1,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `Integer::rounding_from`.
pub fn float_rounding_mode_pair_gen_var_2() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_2,
        &random_float_rounding_mode_pair_gen_var_2,
        &special_random_float_rounding_mode_pair_gen_var_2,
    )
}

// All `(Float, RoundingMode)` where the float is finite and nonzero.
pub fn float_rounding_mode_pair_gen_var_3() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_3,
        &random_float_rounding_mode_pair_gen_var_3,
        &special_random_float_rounding_mode_pair_gen_var_3,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is unsigned.
type GT2 = Generator<(Float, RoundingMode)>;
#[allow(clippy::type_repetition_in_bounds)]
pub fn float_rounding_mode_pair_gen_var_4<T: PrimitiveUnsigned>() -> GT2
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_4::<T>,
        &random_float_rounding_mode_pair_gen_var_4::<T>,
        &special_random_float_rounding_mode_pair_gen_var_4::<T>,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is signed.
#[allow(clippy::type_repetition_in_bounds)]
pub fn float_rounding_mode_pair_gen_var_5<T: PrimitiveSigned>() -> Generator<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_5::<T>,
        &random_float_rounding_mode_pair_gen_var_5::<T>,
        &special_random_float_rounding_mode_pair_gen_var_5::<T>,
    )
}

// All `(T, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is a primitive
// float.
#[allow(clippy::type_repetition_in_bounds)]
pub fn float_rounding_mode_pair_gen_var_6<T: PrimitiveFloat>() -> Generator<(Float, RoundingMode)>
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_6::<T>,
        &random_float_rounding_mode_pair_gen_var_6::<T>,
        &special_random_float_rounding_mode_pair_gen_var_6::<T>,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`.
pub fn float_rounding_mode_pair_gen_var_7() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_7,
        &random_float_rounding_mode_pair_gen_var_7,
        &special_random_float_rounding_mode_pair_gen_var_7,
    )
}

pub fn float_rounding_mode_pair_gen_var_7_rm(
) -> Generator<((rug::Float, rug::float::Round), (Float, RoundingMode))> {
    Generator::new(
        &|| float_rounding_mode_pair_rm(exhaustive_float_rounding_mode_pair_gen_var_7()),
        &|config| float_rounding_mode_pair_rm(random_float_rounding_mode_pair_gen_var_7(config)),
        &|config| {
            float_rounding_mode_pair_rm(special_random_float_rounding_mode_pair_gen_var_7(config))
        },
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`, where the `Float` has a
// precision less than `Limb::WIDTH`.
pub fn float_rounding_mode_pair_gen_var_8() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_8,
        &random_float_rounding_mode_pair_gen_var_8,
        &special_random_float_rounding_mode_pair_gen_var_8,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`, where the `Float` has
// precision `Limb::WIDTH`.
pub fn float_rounding_mode_pair_gen_var_9() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_9,
        &random_float_rounding_mode_pair_gen_var_9,
        &special_random_float_rounding_mode_pair_gen_var_9,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`, where the `Float` has a
// precision greater than `Limb::WIDTH` and less than `Limb::WIDTH` * 2.
pub fn float_rounding_mode_pair_gen_var_10() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_10,
        &random_float_rounding_mode_pair_gen_var_10,
        &special_random_float_rounding_mode_pair_gen_var_10,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`, where the `Float` has
// precision `Limb::WIDTH` * 2.
pub fn float_rounding_mode_pair_gen_var_11() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_11,
        &random_float_rounding_mode_pair_gen_var_11,
        &special_random_float_rounding_mode_pair_gen_var_11,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`, where the `Float` has a
// precision greater than `Limb::WIDTH` * 2 and less than `Limb::WIDTH` * 3.
pub fn float_rounding_mode_pair_gen_var_12() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_12,
        &random_float_rounding_mode_pair_gen_var_12,
        &special_random_float_rounding_mode_pair_gen_var_12,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `reciprocal_round`.
pub fn float_rounding_mode_pair_gen_var_13() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_13,
        &random_float_rounding_mode_pair_gen_var_13,
        &special_random_float_rounding_mode_pair_gen_var_13,
    )
}

pub fn float_rounding_mode_pair_gen_var_13_rm(
) -> Generator<((rug::Float, rug::float::Round), (Float, RoundingMode))> {
    Generator::new(
        &|| float_rounding_mode_pair_rm(exhaustive_float_rounding_mode_pair_gen_var_13()),
        &|config| float_rounding_mode_pair_rm(random_float_rounding_mode_pair_gen_var_13(config)),
        &|config| {
            float_rounding_mode_pair_rm(special_random_float_rounding_mode_pair_gen_var_13(config))
        },
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `reciprocal_round`, where the `Float` has a
// precision less than `Limb::WIDTH`.
pub fn float_rounding_mode_pair_gen_var_14() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_14,
        &random_float_rounding_mode_pair_gen_var_14,
        &special_random_float_rounding_mode_pair_gen_var_14,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `reciprocal_round`, where the `Float` has
// precision `Limb::WIDTH`.
pub fn float_rounding_mode_pair_gen_var_15() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_15,
        &random_float_rounding_mode_pair_gen_var_15,
        &special_random_float_rounding_mode_pair_gen_var_15,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `reciprocal_round`, where the `Float` has a
// precision greater than `Limb::WIDTH` and less than `Limb::WIDTH` * 2.
pub fn float_rounding_mode_pair_gen_var_16() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_16,
        &random_float_rounding_mode_pair_gen_var_16,
        &special_random_float_rounding_mode_pair_gen_var_16,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `reciprocal_round`, where the `Float` has a
// precision greater than `Limb::WIDTH` * 2.
pub fn float_rounding_mode_pair_gen_var_17() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_17,
        &random_float_rounding_mode_pair_gen_var_17,
        &special_random_float_rounding_mode_pair_gen_var_17,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is unsigned
// and the `Float` is extreme.
#[allow(clippy::type_repetition_in_bounds)]
pub fn float_rounding_mode_pair_gen_var_18<T: PrimitiveUnsigned>() -> GT2
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_18::<T>,
        &random_float_rounding_mode_pair_gen_var_18::<T>,
        &special_random_float_rounding_mode_pair_gen_var_18::<T>,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is signed and
// the `Float` is extreme.
#[allow(clippy::type_repetition_in_bounds)]
pub fn float_rounding_mode_pair_gen_var_19<T: PrimitiveSigned>() -> Generator<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_19::<T>,
        &random_float_rounding_mode_pair_gen_var_19::<T>,
        &special_random_float_rounding_mode_pair_gen_var_19::<T>,
    )
}

// All `(T, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is a primitive
// float and the `Float` is extreme.
#[allow(clippy::type_repetition_in_bounds)]
pub fn float_rounding_mode_pair_gen_var_20<T: PrimitiveFloat>() -> Generator<(Float, RoundingMode)>
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_20::<T>,
        &random_float_rounding_mode_pair_gen_var_20::<T>,
        &special_random_float_rounding_mode_pair_gen_var_20::<T>,
    )
}

// All `(T, RoundingMode)` where the `Float` is extreme.
pub fn float_rounding_mode_pair_gen_var_21() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_21,
        &random_float_rounding_mode_pair_gen_var_21,
        &special_random_float_rounding_mode_pair_gen_var_21,
    )
}

// All `(Float, RoundingMode)` that are valid inputs to `square_round`, where the `Float` is
// extreme.
pub fn float_rounding_mode_pair_gen_var_22() -> Generator<(Float, RoundingMode)> {
    Generator::new(
        &exhaustive_float_rounding_mode_pair_gen_var_22,
        &random_float_rounding_mode_pair_gen_var_22,
        &special_random_float_rounding_mode_pair_gen_var_22,
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

// vars 1 and 2 are in malachite-nz.

// All `(Integer, u64, RoundingMode)` that are valid inputs to `Float::from_integer_prec_round`.
pub fn integer_unsigned_rounding_mode_triple_gen_var_3() -> Generator<(Integer, u64, RoundingMode)>
{
    Generator::new(
        &exhaustive_integer_unsigned_rounding_mode_triple_gen_var_3,
        &random_integer_unsigned_rounding_mode_triple_gen_var_3,
        &special_random_integer_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// All `(Integer, u64, RoundingMode)` that are valid inputs to `Float::from_integer_prec_round`,
// excluding those with `Exact`.
pub fn integer_unsigned_rounding_mode_triple_gen_var_4() -> Generator<(Integer, u64, RoundingMode)>
{
    Generator::new(
        &exhaustive_integer_unsigned_rounding_mode_triple_gen_var_4,
        &random_integer_unsigned_rounding_mode_triple_gen_var_4,
        &special_random_integer_unsigned_rounding_mode_triple_gen_var_4,
    )
}

// var 5 is in malachite-nz.

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-base.

// All `(T, u64, RoundingMode)` that are valid inputs to `from_primitive_float_prec_round`.
pub fn primitive_float_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveFloat>(
) -> Generator<(T, u64, RoundingMode)>
where
    Float: From<T>,
{
    Generator::new(
        &exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_3,
        &random_primitive_float_unsigned_rounding_mode_triple_gen_var_3,
        &special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// All `(T, u64, RoundingMode)` that are valid inputs to `from_primitive_float_prec_round`, except
// for those including `Exact`.
pub fn primitive_float_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveFloat>(
) -> Generator<(T, u64, RoundingMode)>
where
    Float: From<T>,
{
    Generator::new(
        &exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_4,
        &random_primitive_float_unsigned_rounding_mode_triple_gen_var_4,
        &special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_4,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-base.

// All `(T, u64, RoundingMode)` that are valid inputs to `Float::from_signed_prec_round`.
pub fn signed_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveSigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_3,
        &random_signed_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_signed_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// All `(T, u64, RoundingMode)` that are valid inputs to `Float::from_signed_prec_round`, excluding
// those with `Exact`.
pub fn signed_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_4,
        &random_signed_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_signed_unsigned_rounding_mode_triple_gen_var_4,
    )
}

// All `(i64, u64, RoundingMode)` that are valid inputs to `Float::power_of_2_prec_round`, where the
// `u64` is small.
pub fn signed_unsigned_rounding_mode_triple_gen_var_5() -> Generator<(i64, u64, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_5,
        &random_signed_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// All `(i64, u64, RoundingMode)` that are valid inputs to `Float::power_of_2_prec_round`.
pub fn signed_unsigned_rounding_mode_triple_gen_var_6() -> Generator<(i64, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_5,
        &random_signed_unsigned_rounding_mode_triple_gen_var_4,
        &special_random_signed_unsigned_rounding_mode_triple_gen_var_5,
    )
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

// var 1 is in malachite-nz

// All `(Natural, u64, RoundingMode)` that are valid inputs to `Float::from_natural_prec_round`.
pub fn natural_unsigned_rounding_mode_triple_gen_var_2() -> Generator<(Natural, u64, RoundingMode)>
{
    Generator::new(
        &exhaustive_natural_unsigned_rounding_mode_triple_gen_var_2,
        &random_natural_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_natural_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// All `(Natural, u64, RoundingMode)` that are valid inputs to `Float::from_natural_prec_round`,
// excluding those with `Exact`.
pub fn natural_unsigned_rounding_mode_triple_gen_var_3() -> Generator<(Natural, u64, RoundingMode)>
{
    Generator::new(
        &exhaustive_natural_unsigned_rounding_mode_triple_gen_var_3,
        &random_natural_unsigned_rounding_mode_triple_gen_var_3,
        &special_random_natural_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 4 are in malachite-base.

// All `(T, u64, RoundingMode)` that are valid inputs to `Float::from_unsigned_prec_round`
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_6,
        &random_unsigned_unsigned_rounding_mode_triple_gen_var_3,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_5,
    )
}

// All `(T, u64, RoundingMode)` that are valid inputs to `Float::from_unsigned_prec_round`,
// excluding those with `Exact`.
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_7,
        &random_unsigned_unsigned_rounding_mode_triple_gen_var_4,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_6,
    )
}

// -- (Rational, PrimitiveUnsigned, RoundingMode) --

// All `(Rational, u64, RoundingMode)` that are valid inputs to `Float::from_rational_prec_round`.
type GT3 = Generator<(Rational, u64, RoundingMode)>;
pub fn rational_unsigned_rounding_mode_triple_gen_var_1() -> GT3 {
    Generator::new(
        &exhaustive_rational_unsigned_rounding_mode_triple_gen_var_1,
        &random_rational_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_rational_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// All `(Rational, u64, RoundingMode)` that are valid inputs to `Float::from_rational_prec_round`,
// excluding those with `Exact`.
type GT4 = Generator<(Rational, u64, RoundingMode)>;
pub fn rational_unsigned_rounding_mode_triple_gen_var_2() -> GT4 {
    Generator::new(
        &exhaustive_rational_unsigned_rounding_mode_triple_gen_var_2,
        &random_rational_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_rational_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// -- (Rational, RoundingMode) --

// vars 1 through 5 are in malachite-q.

// All `(Rational, u64, RoundingMode)` that are valid inputs to `Float::from_rational_prec_round`,
// with precision fixed at 1.
pub fn rational_rounding_mode_pair_gen_var_6() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_6,
        &random_rational_rounding_mode_pair_gen_var_6,
        &special_random_rational_rounding_mode_pair_gen_var_6,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
