use crate::test_util::generators::common::{
    float_integer_pair_rm, float_natural_pair_rm, float_pair_rm, float_primitive_float_pair_rm,
    float_primitive_int_pair_rm, float_rational_pair_rm, float_rm,
    float_unsigned_rounding_mode_triple_rm,
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

// -- (Float, Float, Rational) --

pub fn float_float_rational_triple_gen() -> Generator<(Float, Float, Rational)> {
    Generator::new(
        &exhaustive_float_float_rational_triple_gen,
        &random_float_float_rational_triple_gen,
        &special_random_float_float_rational_triple_gen,
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

pub fn float_integer_pair_gen_var_1() -> Generator<(Float, Integer)> {
    Generator::new(
        &exhaustive_float_integer_pair_gen_var_1,
        &random_float_integer_pair_gen_var_1,
        &special_random_float_integer_pair_gen_var_1,
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

// -- (Float, PrimitiveSigned, PrimitiveSigned) --

pub fn float_signed_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Float, T, T)> {
    Generator::new(
        &exhaustive_float_signed_signed_triple_gen,
        &random_float_primitive_int_primitive_int_triple_gen,
        &special_random_float_signed_signed_triple_gen,
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

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn float_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Float, T, T)> {
    Generator::new(
        &exhaustive_float_unsigned_unsigned_triple_gen,
        &random_float_primitive_int_primitive_int_triple_gen,
        &special_random_float_unsigned_unsigned_triple_gen,
    )
}

// -- (Float, PrimitiveUnsigned, RoundingMode) --

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.set_prec`.
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
            float_unsigned_rounding_mode_triple_rm(
                exhaustive_float_unsigned_rounding_mode_triple_gen_var_1(),
            )
        },
        &|config| {
            float_unsigned_rounding_mode_triple_rm(
                random_float_unsigned_rounding_mode_triple_gen_var_1(config),
            )
        },
        &|config| {
            float_unsigned_rounding_mode_triple_rm(
                special_random_float_unsigned_rounding_mode_triple_gen_var_1(config),
            )
        },
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

// -- (Float, Rational, Rational) --

pub fn float_rational_rational_triple_gen() -> Generator<(Float, Rational, Rational)> {
    Generator::new(
        &exhaustive_float_rational_rational_triple_gen,
        &random_float_rational_rational_triple_gen,
        &special_random_float_rational_rational_triple_gen,
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

// All `(T, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is unsigned.
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

// All `(T, RoundingMode)` that are valid inputs to `T::rounding_from`, where `T` is signed.
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

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-nz.

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
// excluding those with `RoundingMode::Exact`.
pub fn integer_unsigned_rounding_mode_triple_gen_var_4() -> Generator<(Integer, u64, RoundingMode)>
{
    Generator::new(
        &exhaustive_integer_unsigned_rounding_mode_triple_gen_var_4,
        &random_integer_unsigned_rounding_mode_triple_gen_var_4,
        &special_random_integer_unsigned_rounding_mode_triple_gen_var_4,
    )
}

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
// for those including `RoundingMode::Exact`.
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
// those with `RoundingMode::Exact`.
pub fn signed_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_4,
        &random_signed_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_signed_unsigned_rounding_mode_triple_gen_var_4,
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
// excluding those with `RoundingMode::Exact`.
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

// All `(T, u64, RoundingMode)` that are valid inputs to `Float::from_unsigned_prec_round`.
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_6,
        &random_unsigned_unsigned_rounding_mode_triple_gen_var_3,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_5,
    )
}

// All `(T, u64, RoundingMode)` that are valid inputs to `Float::from_unsigned_prec_round`,
// excluding those with `RoundingMode::Exact`.
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
// excluding those with `RoundingMode::Exact`.
type GT4 = Generator<(Rational, u64, RoundingMode)>;
pub fn rational_unsigned_rounding_mode_triple_gen_var_2() -> GT4 {
    Generator::new(
        &exhaustive_rational_unsigned_rounding_mode_triple_gen_var_2,
        &random_rational_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_rational_unsigned_rounding_mode_triple_gen_var_3,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
