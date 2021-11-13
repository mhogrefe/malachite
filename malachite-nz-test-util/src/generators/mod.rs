use crate::generators::common::{
    integer_integer_natural_triple_rm, integer_integer_triple_1_2_rm, integer_natural_pair_rm,
    integer_nrm, integer_pair_1_nrm, integer_pair_1_rm, integer_pair_nrm, integer_pair_rm,
    integer_rm, integer_triple_1_rm, natural_nrm, natural_pair_1_nm, natural_pair_1_nrm,
    natural_pair_1_rm, natural_pair_nrm, natural_pair_rm, natural_rm, natural_triple_1_rm,
};
use crate::generators::exhaustive::*;
use crate::generators::random::*;
use crate::generators::special_random::*;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::common::Generator;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::gcd::half_gcd::{HalfGcdMatrix, HalfGcdMatrix1};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::{BigInt, BigUint};
use std::ops::{Shl, Shr};

// -- Integer --

pub fn integer_gen() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen,
        &random_integer_gen,
        &special_random_integer_gen,
    )
}

pub fn integer_gen_rm() -> Generator<(rug::Integer, Integer)> {
    Generator::new(
        &|| integer_rm(exhaustive_integer_gen()),
        &|config| integer_rm(random_integer_gen(config)),
        &|config| integer_rm(special_random_integer_gen(config)),
    )
}

pub fn integer_gen_nrm() -> Generator<(BigInt, rug::Integer, Integer)> {
    Generator::new(
        &|| integer_nrm(exhaustive_integer_gen()),
        &|config| integer_nrm(random_integer_gen(config)),
        &|config| integer_nrm(special_random_integer_gen(config)),
    )
}

// All `Integer`s that are exactly equal to a floating point value of type `T`.
pub fn integer_gen_var_1<T: PrimitiveFloat>() -> Generator<Integer>
where
    Natural: From<T>,
{
    Generator::new(
        &exhaustive_integer_gen_var_1::<T>,
        &random_integer_gen_var_1::<T>,
        &special_random_integer_gen_var_1::<T>,
    )
}

// All `Integer`s that are not equal to any floating point value of type `T`.
pub fn integer_gen_var_2<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
) -> Generator<Integer> {
    Generator::new_no_special(
        &exhaustive_integer_gen_var_2::<T>,
        &random_integer_gen_var_2::<T>,
    )
}

// All `Integer`s that are exactly between two adjacent floats of type `T`.
pub fn integer_gen_var_3<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>() -> Generator<Integer>
where
    Natural: ExactFrom<T> + From<T>,
{
    Generator::new(
        &exhaustive_integer_gen_var_3::<T>,
        &random_integer_gen_var_3::<T>,
        &special_random_integer_gen_var_2::<T>,
    )
}

// All `Integer`s that are natural (non-negative).
pub fn integer_gen_var_4() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen_var_4,
        &random_integer_gen_var_4,
        &special_random_integer_gen_var_3,
    )
}

pub fn integer_gen_var_4_rm() -> Generator<(rug::Integer, Integer)> {
    Generator::new(
        &|| integer_rm(exhaustive_integer_gen_var_4()),
        &|config| integer_rm(random_integer_gen_var_4(config)),
        &|config| integer_rm(special_random_integer_gen_var_3(config)),
    )
}

pub fn integer_gen_var_4_nrm() -> Generator<(BigInt, rug::Integer, Integer)> {
    Generator::new(
        &|| integer_nrm(exhaustive_integer_gen_var_4()),
        &|config| integer_nrm(random_integer_gen_var_4(config)),
        &|config| integer_nrm(special_random_integer_gen_var_3(config)),
    )
}

// All `Integer`s that are exactly equal to an unsigned value of type `T`.
pub fn integer_gen_var_5<T: PrimitiveUnsigned>() -> Generator<Integer>
where
    Integer: From<T>,
{
    Generator::new(
        &exhaustive_integer_gen_var_5::<T>,
        &random_integer_gen_var_5::<T>,
        &special_random_integer_gen_var_4::<T>,
    )
}

// All `Integer`s that are exactly equal to a signed value of type `T`.
pub fn integer_gen_var_6<T: PrimitiveSigned>() -> Generator<Integer>
where
    Integer: From<T>,
{
    Generator::new(
        &exhaustive_integer_gen_var_6::<T>,
        &random_integer_gen_var_5::<T>,
        &special_random_integer_gen_var_5::<T>,
    )
}

// All `Integer`s that are negative.
pub fn integer_gen_var_7() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen_var_7,
        &random_integer_gen_var_6,
        &special_random_integer_gen_var_6,
    )
}

// All `Integer`s that are nonzero.
pub fn integer_gen_var_8() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen_var_8,
        &random_integer_gen_var_7,
        &special_random_integer_gen_var_7,
    )
}

// -- (Integer, Integer) --

pub fn integer_pair_gen() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen,
        &random_integer_pair_gen,
        &special_random_integer_pair_gen,
    )
}

pub fn integer_pair_gen_rm() -> Generator<((rug::Integer, rug::Integer), (Integer, Integer))> {
    Generator::new(
        &|| integer_pair_rm(exhaustive_integer_pair_gen()),
        &|config| integer_pair_rm(random_integer_pair_gen(config)),
        &|config| integer_pair_rm(special_random_integer_pair_gen(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_pair_gen_nrm() -> Generator<(
    (BigInt, BigInt),
    (rug::Integer, rug::Integer),
    (Integer, Integer),
)> {
    Generator::new(
        &|| integer_pair_nrm(exhaustive_integer_pair_gen()),
        &|config| integer_pair_nrm(random_integer_pair_gen(config)),
        &|config| integer_pair_nrm(special_random_integer_pair_gen(config)),
    )
}

// All pairs of `Integer`s where the second `Integer` is nonzero.
pub fn integer_pair_gen_var_1() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_1,
        &random_integer_pair_gen_var_1,
        &special_random_integer_pair_gen_var_1,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_pair_gen_var_1_nrm() -> Generator<(
    (BigInt, BigInt),
    (rug::Integer, rug::Integer),
    (Integer, Integer),
)> {
    Generator::new(
        &|| integer_pair_nrm(exhaustive_integer_pair_gen_var_1()),
        &|config| integer_pair_nrm(random_integer_pair_gen_var_1(config)),
        &|config| integer_pair_nrm(special_random_integer_pair_gen_var_1(config)),
    )
}

pub fn integer_pair_gen_var_1_rm() -> Generator<((rug::Integer, rug::Integer), (Integer, Integer))>
{
    Generator::new(
        &|| integer_pair_rm(exhaustive_integer_pair_gen_var_1()),
        &|config| integer_pair_rm(random_integer_pair_gen_var_1(config)),
        &|config| integer_pair_rm(special_random_integer_pair_gen_var_1(config)),
    )
}

// All pairs of `Integer`s where the first `Integer` is divisible by the second, and the second is
// nonzero.
pub fn integer_pair_gen_var_2() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_2,
        &random_integer_pair_gen_var_2,
        &special_random_integer_pair_gen_var_2,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_pair_gen_var_2_nrm() -> Generator<(
    (BigInt, BigInt),
    (rug::Integer, rug::Integer),
    (Integer, Integer),
)> {
    Generator::new(
        &|| integer_pair_nrm(exhaustive_integer_pair_gen_var_2()),
        &|config| integer_pair_nrm(random_integer_pair_gen_var_2(config)),
        &|config| integer_pair_nrm(special_random_integer_pair_gen_var_2(config)),
    )
}

// All pairs of `Integer`s where the first `Integer` is not divisible by the second, and the second
// is nonzero.
pub fn integer_pair_gen_var_3() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_3,
        &random_integer_pair_gen_var_3,
        &special_random_integer_pair_gen_var_3,
    )
}

// -- (Integer, Integer, Integer) --

pub fn integer_triple_gen() -> Generator<(Integer, Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_triple_gen,
        &random_integer_triple_gen,
        &special_random_integer_triple_gen,
    )
}

// All triples of natural (non-negative) `Integer`s.
pub fn integer_triple_gen_var_1() -> Generator<(Integer, Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_triple_gen_var_1,
        &random_integer_triple_gen_var_1,
        &special_random_integer_triple_gen_var_1,
    )
}

// -- (Integer, Integer, Integer, PrimitiveUnsigned) --

// All `(Integer, Integer, Integer, T)` where `T` is unsigned and small.
pub fn integer_integer_integer_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Integer, Integer, Integer, T)> {
    Generator::new(
        &exhaustive_integer_integer_integer_unsigned_quadruple_gen_var_1,
        &random_integer_integer_integer_unsigned_quadruple_gen_var_1,
        &special_random_integer_integer_integer_unsigned_quadruple_gen_var_1,
    )
}

// -- (Integer, Integer, Natural) --

pub fn integer_integer_natural_triple_gen() -> Generator<(Integer, Integer, Natural)> {
    Generator::new(
        &exhaustive_integer_integer_natural_triple_gen,
        &random_integer_integer_natural_triple_gen,
        &special_random_integer_integer_natural_triple_gen,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_integer_natural_triple_gen_rm() -> Generator<(
    (rug::Integer, rug::Integer, rug::Integer),
    (Integer, Integer, Natural),
)> {
    Generator::new(
        &|| integer_integer_natural_triple_rm(exhaustive_integer_integer_natural_triple_gen()),
        &|config| {
            integer_integer_natural_triple_rm(random_integer_integer_natural_triple_gen(config))
        },
        &|config| {
            integer_integer_natural_triple_rm(special_random_integer_integer_natural_triple_gen(
                config,
            ))
        },
    )
}

// All `(Integer, Integer, Natural)` triples where the first `Integer` is equal to the second mod
// the `Natural`.
pub fn integer_integer_natural_triple_gen_var_1() -> Generator<(Integer, Integer, Natural)> {
    Generator::new(
        &exhaustive_integer_integer_natural_triple_gen_var_1,
        &random_integer_integer_natural_triple_gen_var_1,
        &special_random_integer_integer_natural_triple_gen_var_1,
    )
}

// All `(Integer, Integer, Natural)` triples where the first `Integer` is not equal to the second
// mod the `Natural`.
pub fn integer_integer_natural_triple_gen_var_2() -> Generator<(Integer, Integer, Natural)> {
    Generator::new(
        &exhaustive_integer_integer_natural_triple_gen_var_2,
        &random_integer_integer_natural_triple_gen_var_2,
        &special_random_integer_integer_natural_triple_gen_var_2,
    )
}

// -- (Integer, Integer, PrimitiveUnsigned) --

// All `(Integer, Integer, T)` where `T` is unsigned and small.
pub fn integer_integer_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Integer, Integer, T)> {
    Generator::new(
        &exhaustive_integer_integer_unsigned_triple_gen_var_1,
        &random_integer_integer_unsigned_triple_gen_var_1,
        &special_random_integer_integer_unsigned_triple_gen_var_1,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_integer_unsigned_triple_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, rug::Integer, T), (Integer, Integer, T))> {
    Generator::new(
        &|| integer_integer_triple_1_2_rm(exhaustive_integer_integer_unsigned_triple_gen_var_1()),
        &|config| {
            integer_integer_triple_1_2_rm(random_integer_integer_unsigned_triple_gen_var_1(config))
        },
        &|config| {
            integer_integer_triple_1_2_rm(special_random_integer_integer_unsigned_triple_gen_var_1(
                config,
            ))
        },
    )
}

// All `(Integer, Integer, T)` where `T` is unsigned and small, and the `Integer`s are equal mod 2
// to the power of the `T`.
pub fn integer_integer_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Integer, Integer, T)>
where
    Integer: Shl<T, Output = Integer>,
{
    Generator::new(
        &exhaustive_integer_integer_unsigned_triple_gen_var_2,
        &random_integer_integer_unsigned_triple_gen_var_2,
        &special_random_integer_integer_unsigned_triple_gen_var_2,
    )
}

// All `(Integer, Integer, T)` where `T` is unsigned and small, and the `Integer`s are not equal
// mod 2 to the power of the `T`.
pub fn integer_integer_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> Generator<(Integer, Integer, T)> {
    Generator::new(
        &exhaustive_integer_integer_unsigned_triple_gen_var_3,
        &random_integer_integer_unsigned_triple_gen_var_3,
        &special_random_integer_integer_unsigned_triple_gen_var_3,
    )
}

// -- (Integer, Integer, RoundingMode) --

// All `(Integer, Integer, RoundingMode)` triples where the second `Integer` is positive and if the
// `RoundingMode` is `RoundingMode::Exact`, the first `Integer` is divisible by the second.
pub fn integer_integer_rounding_mode_triple_gen_var_1(
) -> Generator<(Integer, Integer, RoundingMode)> {
    Generator::new(
        &exhaustive_integer_integer_rounding_mode_triple_gen_var_1,
        &random_integer_integer_rounding_mode_triple_gen_var_1,
        &special_random_integer_integer_rounding_mode_triple_gen_var_1,
    )
}

// All `(Integer, Integer, RoundingMode)` triples that are a valid input to
// `Integer::round_to_multiple`.
pub fn integer_integer_rounding_mode_triple_gen_var_2(
) -> Generator<(Integer, Integer, RoundingMode)> {
    Generator::new(
        &exhaustive_integer_integer_rounding_mode_triple_gen_var_2,
        &random_integer_integer_rounding_mode_triple_gen_var_2,
        &special_random_integer_integer_rounding_mode_triple_gen_var_2,
    )
}

// -- (Integer, Natural) --

pub fn integer_natural_pair_gen() -> Generator<(Integer, Natural)> {
    Generator::new(
        &exhaustive_integer_natural_pair_gen,
        &random_integer_natural_pair_gen,
        &special_random_integer_natural_pair_gen,
    )
}

type T1 = Generator<((rug::Integer, rug::Integer), (Integer, Natural))>;
pub fn integer_natural_pair_gen_rm() -> T1 {
    Generator::new(
        &|| integer_natural_pair_rm(exhaustive_integer_natural_pair_gen()),
        &|config| integer_natural_pair_rm(random_integer_natural_pair_gen(config)),
        &|config| integer_natural_pair_rm(special_random_integer_natural_pair_gen(config)),
    )
}

// -- (Integer, Natural, Integer) --

pub fn integer_natural_integer_triple_gen() -> Generator<(Integer, Natural, Integer)> {
    Generator::new(
        &exhaustive_integer_natural_integer_triple_gen,
        &random_integer_natural_integer_triple_gen,
        &special_random_integer_natural_integer_triple_gen,
    )
}

// -- (Integer, PrimitiveSigned) --

pub fn integer_signed_pair_gen<T: PrimitiveSigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_signed_pair_gen,
        &random_integer_primitive_int_pair_gen,
        &special_random_integer_signed_pair_gen,
    )
}

pub fn integer_signed_pair_gen_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_rm(exhaustive_integer_signed_pair_gen()),
        &|config| integer_pair_1_rm(random_integer_primitive_int_pair_gen(config)),
        &|config| integer_pair_1_rm(special_random_integer_signed_pair_gen(config)),
    )
}

// All `(Integer, T)` where `T` is signed and small.
pub fn integer_signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_signed_pair_gen_var_1,
        &random_integer_signed_pair_gen_var_1,
        &special_random_integer_signed_pair_gen_var_1,
    )
}

pub fn integer_signed_pair_gen_var_1_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_rm(exhaustive_integer_signed_pair_gen_var_1()),
        &|config| integer_pair_1_rm(random_integer_signed_pair_gen_var_1(config)),
        &|config| integer_pair_1_rm(special_random_integer_signed_pair_gen_var_1(config)),
    )
}

// -- (Integer, PrimitiveSigned, Integer) --

pub fn integer_signed_integer_triple_gen<T: PrimitiveSigned>() -> Generator<(Integer, T, Integer)> {
    Generator::new(
        &exhaustive_integer_signed_integer_triple_gen,
        &random_integer_primitive_int_integer_triple_gen,
        &special_random_integer_signed_integer_triple_gen,
    )
}

// -- (Integer, PrimitiveSigned, RoundingMode) --

// All `(Integer, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `Integer::shl_round`.
pub fn integer_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(Integer, T, RoundingMode)>
where
    Integer: Shr<T, Output = Integer>,
{
    Generator::new(
        &exhaustive_integer_signed_rounding_mode_triple_gen_var_1,
        &random_integer_signed_rounding_mode_triple_gen_var_1,
        &special_random_integer_signed_rounding_mode_triple_gen_var_1,
    )
}

// All `(Integer, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `Integer::shr_round`.
pub fn integer_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
) -> Generator<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Generator::new(
        &exhaustive_integer_signed_rounding_mode_triple_gen_var_2,
        &random_integer_signed_rounding_mode_triple_gen_var_2,
        &special_random_integer_signed_rounding_mode_triple_gen_var_2,
    )
}

// -- (Integer, PrimitiveUnsigned) --

pub fn integer_unsigned_pair_gen<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen,
        &random_integer_primitive_int_pair_gen,
        &special_random_integer_unsigned_pair_gen,
    )
}

pub fn integer_unsigned_pair_gen_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_rm(exhaustive_integer_unsigned_pair_gen()),
        &|config| integer_pair_1_rm(random_integer_primitive_int_pair_gen(config)),
        &|config| integer_pair_1_rm(special_random_integer_unsigned_pair_gen(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_unsigned_pair_gen_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_nrm(exhaustive_integer_unsigned_pair_gen()),
        &|config| integer_pair_1_nrm(random_integer_primitive_int_pair_gen(config)),
        &|config| integer_pair_1_nrm(special_random_integer_unsigned_pair_gen(config)),
    )
}

// All `(Integer, T)` where `T` is unsigned and between 2 and 36, inclusive.
pub fn integer_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_1,
        &random_integer_unsigned_pair_gen_var_1,
        &special_random_integer_unsigned_pair_gen_var_1,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_unsigned_pair_gen_var_1_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_nrm(exhaustive_integer_unsigned_pair_gen_var_1()),
        &|config| integer_pair_1_nrm(random_integer_unsigned_pair_gen_var_1(config)),
        &|config| integer_pair_1_nrm(special_random_integer_unsigned_pair_gen_var_1(config)),
    )
}

// All `(Integer, T)` where `T` is unsigned and small.
pub fn integer_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_2,
        &random_integer_unsigned_pair_gen_var_2,
        &special_random_integer_unsigned_pair_gen_var_2,
    )
}

pub fn integer_unsigned_pair_gen_var_2_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_rm(exhaustive_integer_unsigned_pair_gen_var_2()),
        &|config| integer_pair_1_rm(random_integer_unsigned_pair_gen_var_2(config)),
        &|config| integer_pair_1_rm(special_random_integer_unsigned_pair_gen_var_2(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_unsigned_pair_gen_var_2_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_nrm(exhaustive_integer_unsigned_pair_gen_var_2()),
        &|config| integer_pair_1_nrm(random_integer_unsigned_pair_gen_var_2(config)),
        &|config| integer_pair_1_nrm(special_random_integer_unsigned_pair_gen_var_2(config)),
    )
}

// All `(Integer, T)` where `T` is unsigned, small, and positive, and either the `Integer` is
// non-negative or the `T` is odd.
pub fn integer_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_3,
        &random_integer_unsigned_pair_gen_var_3,
        &special_random_integer_unsigned_pair_gen_var_3,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_unsigned_pair_gen_var_3_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_nrm(exhaustive_integer_unsigned_pair_gen_var_3()),
        &|config| integer_pair_1_nrm(random_integer_unsigned_pair_gen_var_3(config)),
        &|config| integer_pair_1_nrm(special_random_integer_unsigned_pair_gen_var_3(config)),
    )
}

// All `(Integer, u64)`s where the `T` is unsigned and small, and the Integer is divisible by 2 to
// the power of the `T`.
pub fn integer_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_4,
        &random_integer_unsigned_pair_gen_var_4,
        &special_random_integer_unsigned_pair_gen_var_4,
    )
}

// All `(Integer, u64)`s where the `T` is unsigned and small, and the Integer is not divisible by 2
// to the power of the `T`.
pub fn integer_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_5,
        &random_integer_unsigned_pair_gen_var_5,
        &special_random_integer_unsigned_pair_gen_var_5,
    )
}

// -- (Integer, PrimitiveUnsigned, bool) --

// All `(Integer, T, bool)` where `T` is unsigned and small.
pub fn integer_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, bool)> {
    Generator::new(
        &exhaustive_integer_unsigned_bool_triple_gen_var_1,
        &random_integer_unsigned_bool_triple_gen_var_1,
        &special_random_integer_unsigned_bool_triple_gen_var_1,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_unsigned_bool_triple_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T, bool), (Integer, T, bool))> {
    Generator::new(
        &|| integer_triple_1_rm(exhaustive_integer_unsigned_bool_triple_gen_var_1()),
        &|config| integer_triple_1_rm(random_integer_unsigned_bool_triple_gen_var_1(config)),
        &|config| {
            integer_triple_1_rm(special_random_integer_unsigned_bool_triple_gen_var_1(
                config,
            ))
        },
    )
}

// -- (Integer, PrimitiveUnsigned, Integer) --

pub fn integer_unsigned_integer_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, Integer)> {
    Generator::new(
        &exhaustive_integer_unsigned_integer_triple_gen,
        &random_integer_primitive_int_integer_triple_gen,
        &special_random_integer_unsigned_integer_triple_gen,
    )
}

// -- (Integer, PrimitiveUnsigned, Natural) --

pub fn integer_unsigned_natural_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, Natural)> {
    Generator::new(
        &exhaustive_integer_unsigned_natural_triple_gen,
        &random_integer_primitive_int_natural_triple_gen,
        &special_random_integer_unsigned_natural_triple_gen,
    )
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(Integer, T, U)` where `T` and `U` are unsigned, the `T` is between 2 and 36, inclusive, and
// the `U` is small.
pub fn integer_unsigned_unsigned_triple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Integer, T, U)> {
    Generator::new(
        &exhaustive_integer_unsigned_unsigned_triple_gen_var_1,
        &random_integer_unsigned_unsigned_triple_gen_var_1,
        &special_random_integer_unsigned_unsigned_triple_gen_var_1,
    )
}

// All `(Integer, T, T)` where `T` is unsigned, both `T`s are small, and the first `T` is less than
// or equal to the second.
pub fn integer_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_unsigned_triple_gen_var_2,
        &random_integer_unsigned_unsigned_triple_gen_var_2,
        &special_random_integer_unsigned_unsigned_triple_gen_var_2,
    )
}

// All `(Integer, T, T)` where `T` is unsigned and both `T`s are small.
pub fn integer_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_unsigned_triple_gen_var_3,
        &random_integer_unsigned_unsigned_triple_gen_var_3,
        &special_random_integer_unsigned_unsigned_triple_gen_var_3,
    )
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

// All `(Integer, T, T, Natural)` where `T` is unsigned and the first `T` is smaller than the
// second.
pub fn integer_unsigned_unsigned_natural_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, T, Natural)> {
    Generator::new(
        &exhaustive_integer_unsigned_unsigned_natural_quadruple_gen_var_1,
        &random_integer_unsigned_unsigned_natural_triple_gen_var_1,
        &special_random_integer_unsigned_unsigned_natural_quadruple_gen_var_1,
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

// All `(Integer, u64, RoundingMode)` where the triple is a valid input to
// `Integer::round_to_multiple_of_power_of_2`.
pub fn integer_unsigned_rounding_mode_triple_gen_var_1() -> Generator<(Integer, u64, RoundingMode)>
{
    Generator::new(
        &exhaustive_integer_unsigned_rounding_mode_triple_gen_var_1,
        &random_integer_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_integer_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// All `(Integer, T, RoundingMode)` where `T` is unsigned and the triple is a valid input to
// `Integer::shr_round`.
pub fn integer_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Generator::new(
        &exhaustive_integer_unsigned_rounding_mode_triple_gen_var_2,
        &random_integer_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_integer_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// -- (Integer, RoundingMode) --

pub fn integer_rounding_mode_pair_gen() -> Generator<(Integer, RoundingMode)> {
    Generator::new(
        &exhaustive_integer_rounding_mode_pair_gen,
        &random_integer_rounding_mode_pair_gen,
        &special_random_integer_rounding_mode_pair_gen,
    )
}

// All `(Integer, RoundingMode)` pairs that are valid inputs to `T::rounding_from`.
pub fn integer_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Integer> + PrimitiveFloat,
>() -> Generator<(Integer, RoundingMode)> {
    Generator::new(
        &exhaustive_integer_rounding_mode_pair_gen_var_1::<T>,
        &random_integer_rounding_mode_pair_gen_var_1::<T>,
        &special_random_integer_rounding_mode_pair_gen_var_1::<T>,
    )
}

// All `(Integer, RoundingMode)` pairs where the `Integer` is nonzero.
pub fn integer_rounding_mode_pair_gen_var_2() -> Generator<(Integer, RoundingMode)> {
    Generator::new(
        &exhaustive_integer_rounding_mode_pair_gen_var_2,
        &random_integer_rounding_mode_pair_gen_var_2,
        &special_random_integer_rounding_mode_pair_gen_var_2,
    )
}

// -- (Integer, Vec<bool>) --

// All `(Integer, Vec<bool>)` pairs where the length of the `Vec` is the twos' complement limb
// count of the `Integer`, including sign extension limbs if necessary.
pub fn integer_bool_vec_pair_gen_var_1() -> Generator<(Integer, Vec<bool>)> {
    Generator::new(
        &exhaustive_integer_bool_vec_pair_gen_var_1,
        &random_integer_bool_vec_pair_gen_var_1,
        &special_random_integer_bool_vec_pair_gen_var_1,
    )
}

// All `(Integer, Vec<bool>)` pairs where the length of the `Vec` is the twos' complement bit count
// of the `Integer`, including sign extension bits if necessary.
pub fn integer_bool_vec_pair_gen_var_2() -> Generator<(Integer, Vec<bool>)> {
    Generator::new(
        &exhaustive_integer_bool_vec_pair_gen_var_2,
        &random_integer_bool_vec_pair_gen_var_2,
        &special_random_integer_bool_vec_pair_gen_var_2,
    )
}

// -- Natural --

pub fn natural_gen() -> Generator<Natural> {
    Generator::new(
        &exhaustive_natural_gen,
        &random_natural_gen,
        &special_random_natural_gen,
    )
}

pub fn natural_gen_rm() -> Generator<(rug::Integer, Natural)> {
    Generator::new(
        &|| natural_rm(exhaustive_natural_gen()),
        &|config| natural_rm(random_natural_gen(config)),
        &|config| natural_rm(special_random_natural_gen(config)),
    )
}

pub fn natural_gen_nrm() -> Generator<(BigUint, rug::Integer, Natural)> {
    Generator::new(
        &|| natural_nrm(exhaustive_natural_gen()),
        &|config| natural_nrm(random_natural_gen(config)),
        &|config| natural_nrm(special_random_natural_gen(config)),
    )
}

// All `Natural`s greater than or equal to 2.
pub fn natural_gen_var_1() -> Generator<Natural> {
    Generator::new_no_special(&exhaustive_natural_gen_var_1, &random_natural_gen_var_1)
}

// All positive `Natural`s.
pub fn natural_gen_var_2() -> Generator<Natural> {
    Generator::new(
        &exhaustive_natural_gen_var_2,
        &random_natural_gen_var_2,
        &special_random_natural_gen_var_1,
    )
}

// All `Natural`s that are exactly equal to a floating point value of type `T`.
pub fn natural_gen_var_3<T: PrimitiveFloat>() -> Generator<Natural>
where
    Natural: From<T>,
{
    Generator::new(
        &exhaustive_natural_gen_var_3::<T>,
        &random_natural_gen_var_3::<T>,
        &special_random_natural_gen_var_2::<T>,
    )
}

// All `Natural`s that are not equal to any floating point value of type `T`.
pub fn natural_gen_var_4<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
) -> Generator<Natural> {
    Generator::new_no_special(
        &exhaustive_natural_gen_var_4::<T>,
        &random_natural_gen_var_4::<T>,
    )
}

type GN = Generator<Natural>;

// All `Natural`s that are exactly between two adjacent floats of type `T`.
pub fn natural_gen_var_5<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>() -> GN
where
    Natural: ExactFrom<T> + From<T>,
{
    Generator::new(
        &exhaustive_natural_gen_var_5::<T>,
        &random_natural_gen_var_5::<T>,
        &special_random_natural_gen_var_3::<T>,
    )
}

// All `Natural`s that are exactly equal to an unsigned value of type `T`.
pub fn natural_gen_var_6<T: PrimitiveUnsigned>() -> Generator<Natural>
where
    Natural: From<T>,
{
    Generator::new(
        &exhaustive_natural_gen_var_6::<T>,
        &random_natural_gen_var_6::<T>,
        &special_random_natural_gen_var_4::<T>,
    )
}

// All `Natural`s that are exactly equal to a signed value of type `T`.
pub fn natural_gen_var_7<T: PrimitiveSigned>() -> Generator<Natural>
where
    Natural: ExactFrom<T>,
{
    Generator::new(
        &exhaustive_natural_gen_var_7::<T>,
        &random_natural_gen_var_7::<T>,
        &special_random_natural_gen_var_5::<T>,
    )
}

// -- (Natural, Integer, Natural) --

pub fn natural_integer_natural_triple_gen() -> Generator<(Natural, Integer, Natural)> {
    Generator::new(
        &exhaustive_natural_integer_natural_triple_gen,
        &random_natural_integer_natural_triple_gen,
        &special_random_natural_integer_natural_triple_gen,
    )
}

// -- (Natural, Natural) --

pub fn natural_pair_gen() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen,
        &random_natural_pair_gen,
        &special_random_natural_pair_gen,
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_pair_gen_nrm() -> Generator<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Generator::new(
        &|| natural_pair_nrm(exhaustive_natural_pair_gen()),
        &|config| natural_pair_nrm(random_natural_pair_gen(config)),
        &|config| natural_pair_nrm(special_random_natural_pair_gen(config)),
    )
}

pub fn natural_pair_gen_rm() -> Generator<((rug::Integer, rug::Integer), (Natural, Natural))> {
    Generator::new(
        &|| natural_pair_rm(exhaustive_natural_pair_gen()),
        &|config| natural_pair_rm(random_natural_pair_gen(config)),
        &|config| natural_pair_rm(special_random_natural_pair_gen(config)),
    )
}

// All pairs of `Natural`s where the first `Natural` is large (at least 2^`Limb::WIDTH`) and the
// second is at least 2.
pub fn natural_pair_gen_var_1() -> Generator<(Natural, Natural)> {
    Generator::new_no_special(
        &exhaustive_natural_pair_gen_var_1,
        &random_natural_pair_gen_var_1,
    )
}

// All pairs of `Natural`s where the second `Natural` is at least 2.
pub fn natural_pair_gen_var_2() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_2,
        &random_natural_pair_gen_var_2,
        &special_random_natural_pair_gen_var_1,
    )
}

// All pairs of `Natural`s where the first `Natural` is positive and the second is at least 2.
pub fn natural_pair_gen_var_3() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_3,
        &random_natural_pair_gen_var_3,
        &special_random_natural_pair_gen_var_2,
    )
}

// All pairs of `Natural`s that tend to have large GCDs.
pub fn natural_pair_gen_var_4() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_4,
        &random_natural_pair_gen_var_4,
        &special_random_natural_pair_gen_var_3,
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_pair_gen_var_4_nrm() -> Generator<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Generator::new(
        &|| natural_pair_nrm(exhaustive_natural_pair_gen_var_4()),
        &|config| natural_pair_nrm(random_natural_pair_gen_var_4(config)),
        &|config| natural_pair_nrm(special_random_natural_pair_gen_var_3(config)),
    )
}

// All pairs of `Natural`s where the second `Natural` is positive.
pub fn natural_pair_gen_var_5() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_5,
        &random_natural_pair_gen_var_5,
        &special_random_natural_pair_gen_var_4,
    )
}

// All pairs of `Natural`s where the first `Natural` is divisible by the second, and the second is
// positive.
pub fn natural_pair_gen_var_6() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_6,
        &random_natural_pair_gen_var_6,
        &special_random_natural_pair_gen_var_5,
    )
}

// All pairs of `Natural`s where the first `Natural` is not divisible by the second, and the second
// is positive.
pub fn natural_pair_gen_var_7() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_7,
        &random_natural_pair_gen_var_7,
        &special_random_natural_pair_gen_var_6,
    )
}

// -- (Natural, Natural, Natural) --

pub fn natural_triple_gen() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen,
        &random_natural_triple_gen,
        &special_random_natural_triple_gen,
    )
}

// -- (Natural, Natural, PrimitiveUnsigned) --

// All `(Natural, Natural, T)` where `T` is unsigned and small.
pub fn natural_natural_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_pair_gen_var_1,
        &random_natural_natural_unsigned_triple_gen_var_1,
        &special_random_natural_natural_unsigned_triple_gen_var_1,
    )
}

// -- (Natural, Natural, RoundingMode) --

// All `(Natural, Natural, RoundingMode)` triples where the second `Natural` is positive and if the
// `RoundingMode` is `RoundingMode::Exact`, the first `Natural` is divisible by the second.
pub fn natural_natural_rounding_mode_triple_gen_var_1(
) -> Generator<(Natural, Natural, RoundingMode)> {
    Generator::new(
        &exhaustive_natural_natural_rounding_mode_triple_gen_var_1,
        &random_natural_natural_rounding_mode_triple_gen_var_1,
        &special_random_natural_natural_rounding_mode_triple_gen_var_1,
    )
}

// All `(Natural, Natural, RoundingMode)` triples that are a valid input to
// `Natural::round_to_multiple`.
pub fn natural_natural_rounding_mode_triple_gen_var_2(
) -> Generator<(Natural, Natural, RoundingMode)> {
    Generator::new(
        &exhaustive_natural_natural_rounding_mode_triple_gen_var_2,
        &random_natural_natural_rounding_mode_triple_gen_var_2,
        &special_random_natural_natural_rounding_mode_triple_gen_var_2,
    )
}

// -- (Natural, PrimitiveSigned) --

pub fn natural_signed_pair_gen<T: PrimitiveSigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_signed_pair_gen,
        &random_natural_primitive_int_pair_gen,
        &special_random_natural_signed_pair_gen,
    )
}

pub fn natural_signed_pair_gen_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_rm(exhaustive_natural_signed_pair_gen()),
        &|config| natural_pair_1_rm(random_natural_primitive_int_pair_gen(config)),
        &|config| natural_pair_1_rm(special_random_natural_signed_pair_gen(config)),
    )
}

// All pairs of `Natural` and signed `T`, where the `T` is natural (non-negative).
pub fn natural_signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_signed_pair_gen_var_1,
        &random_natural_signed_pair_gen_var_1,
        &special_random_natural_signed_pair_gen_var_1,
    )
}

// All `(Natural, T)` where `T` is signed and small.
pub fn natural_signed_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_signed_pair_gen_var_2,
        &random_natural_signed_pair_gen_var_2,
        &special_random_natural_signed_pair_gen_var_2,
    )
}

// -- (Natural, PrimitiveSigned, Natural) --

pub fn natural_signed_natural_triple_gen<T: PrimitiveSigned>() -> Generator<(Natural, T, Natural)> {
    Generator::new(
        &exhaustive_natural_signed_natural_triple_gen,
        &random_natural_primitive_int_natural_triple_gen,
        &special_random_natural_signed_natural_triple_gen,
    )
}

// -- (Natural, PrimitiveSigned, RoundingMode) --

// All `(Natural, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `Natural::shl_round`.
pub fn natural_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(Natural, T, RoundingMode)>
where
    Natural: Shr<T, Output = Natural>,
{
    Generator::new(
        &exhaustive_natural_signed_rounding_mode_triple_gen_var_1,
        &random_natural_signed_rounding_mode_triple_gen_var_1,
        &special_random_natural_signed_rounding_mode_triple_gen_var_1,
    )
}

// All `(Natural, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `Natural::shr_round`.
pub fn natural_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
) -> Generator<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Generator::new(
        &exhaustive_natural_signed_rounding_mode_triple_gen_var_2,
        &random_natural_signed_rounding_mode_triple_gen_var_2,
        &special_random_natural_signed_rounding_mode_triple_gen_var_2,
    )
}

// -- (Natural, PrimitiveUnsigned) --

pub fn natural_unsigned_pair_gen<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen,
        &random_natural_primitive_int_pair_gen,
        &special_random_natural_unsigned_pair_gen,
    )
}

pub fn natural_unsigned_pair_gen_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_rm(exhaustive_natural_unsigned_pair_gen()),
        &|config| natural_pair_1_rm(random_natural_primitive_int_pair_gen(config)),
        &|config| natural_pair_1_rm(special_random_natural_unsigned_pair_gen(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_unsigned_pair_gen_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_nrm(exhaustive_natural_unsigned_pair_gen()),
        &|config| natural_pair_1_nrm(random_natural_primitive_int_pair_gen(config)),
        &|config| natural_pair_1_nrm(special_random_natural_unsigned_pair_gen(config)),
    )
}

// All `(Natural, T)` where `T` is unsigned and the `T` is at least 2 and at most `U::MAX`.
pub fn natural_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_primitive_int_pair_gen_var_1::<T, U>,
        &random_natural_unsigned_pair_gen_var_1::<T, U>,
        &special_random_natural_unsigned_pair_gen_var_1::<T, U>,
    )
}

// All `(Natural, T)` where `T` is unsigned and the `T` is at least 2.
pub fn natural_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_primitive_int_pair_gen_var_2,
        &random_natural_unsigned_pair_gen_var_2,
        &special_random_natural_unsigned_pair_gen_var_2,
    )
}

// All `(Natural, T)` where `T` is unsigned and between 2 and 36, inclusive.
pub fn natural_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_1,
        &random_natural_unsigned_pair_gen_var_3,
        &special_random_natural_unsigned_pair_gen_var_3,
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_unsigned_pair_gen_var_3_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_nrm(exhaustive_natural_unsigned_pair_gen_var_1()),
        &|config| natural_pair_1_nrm(random_natural_unsigned_pair_gen_var_3(config)),
        &|config| natural_pair_1_nrm(special_random_natural_unsigned_pair_gen_var_3(config)),
    )
}

// All `(Natural, T)` where `T` is unsigned and small.
pub fn natural_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_2,
        &random_natural_unsigned_pair_gen_var_4,
        &special_random_natural_unsigned_pair_gen_var_4,
    )
}

pub fn natural_unsigned_pair_gen_var_4_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_rm(exhaustive_natural_unsigned_pair_gen_var_2()),
        &|config| natural_pair_1_rm(random_natural_unsigned_pair_gen_var_4(config)),
        &|config| natural_pair_1_rm(special_random_natural_unsigned_pair_gen_var_4(config)),
    )
}

pub fn natural_unsigned_pair_gen_var_4_nm<T: PrimitiveUnsigned>(
) -> Generator<((BigUint, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_nm(exhaustive_natural_unsigned_pair_gen_var_2()),
        &|config| natural_pair_1_nm(random_natural_unsigned_pair_gen_var_4(config)),
        &|config| natural_pair_1_nm(special_random_natural_unsigned_pair_gen_var_4(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_unsigned_pair_gen_var_4_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_nrm(exhaustive_natural_unsigned_pair_gen_var_2()),
        &|config| natural_pair_1_nrm(random_natural_unsigned_pair_gen_var_4(config)),
        &|config| natural_pair_1_nrm(special_random_natural_unsigned_pair_gen_var_4(config)),
    )
}

// All `(Natural, T)` where the `Natural` is at least 2 and the `T` is unsigned and small.
pub fn natural_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new_no_special(
        &exhaustive_natural_unsigned_pair_gen_var_3,
        &random_natural_unsigned_pair_gen_var_5,
    )
}

// All `(Natural, u64)`, where the `u64` is between 1 and `T::WIDTH`, inclusive.
pub fn natural_unsigned_pair_gen_var_6<T: PrimitiveInt>() -> Generator<(Natural, u64)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_4::<T>,
        &random_natural_unsigned_pair_gen_var_6::<T>,
        &special_random_natural_unsigned_pair_gen_var_5::<T>,
    )
}

// All `(Natural, T)` where the `T` is unsigned, positive, and small.
pub fn natural_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_primitive_int_pair_gen_var_3,
        &random_natural_unsigned_pair_gen_var_7,
        &special_random_natural_unsigned_pair_gen_var_6,
    )
}

pub fn natural_unsigned_pair_gen_var_7_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_rm(exhaustive_natural_primitive_int_pair_gen_var_3()),
        &|config| natural_pair_1_rm(random_natural_unsigned_pair_gen_var_7(config)),
        &|config| natural_pair_1_rm(special_random_natural_unsigned_pair_gen_var_6(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_unsigned_pair_gen_var_7_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_nrm(exhaustive_natural_primitive_int_pair_gen_var_3()),
        &|config| natural_pair_1_nrm(random_natural_unsigned_pair_gen_var_7(config)),
        &|config| natural_pair_1_nrm(special_random_natural_unsigned_pair_gen_var_6(config)),
    )
}

// All `(Natural, T)` where the `Natural` is positive and the `T` is unsigned, positive, and small.
pub fn natural_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_primitive_int_pair_gen_var_4,
        &random_natural_unsigned_pair_gen_var_8,
        &special_random_natural_unsigned_pair_gen_var_7,
    )
}

// -- (Natural, PrimitiveUnsigned, bool) --

// All `(Natural, T, bool)` where `T` is unsigned and small.
pub fn natural_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, bool)> {
    Generator::new(
        &exhaustive_natural_unsigned_bool_triple_gen_var_1,
        &random_natural_unsigned_bool_triple_gen_var_1,
        &special_random_natural_unsigned_bool_triple_gen_var_1,
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_unsigned_bool_triple_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, T, bool), (Natural, T, bool))> {
    Generator::new(
        &|| natural_triple_1_rm(exhaustive_natural_unsigned_bool_triple_gen_var_1()),
        &|config| natural_triple_1_rm(random_natural_unsigned_bool_triple_gen_var_1(config)),
        &|config| {
            natural_triple_1_rm(special_random_natural_unsigned_bool_triple_gen_var_1(
                config,
            ))
        },
    )
}

// -- (Natural, PrimitiveUnsigned, Natural) --

pub fn natural_unsigned_natural_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, Natural)> {
    Generator::new(
        &exhaustive_natural_unsigned_natural_triple_gen,
        &random_natural_primitive_int_natural_triple_gen,
        &special_random_natural_unsigned_natural_triple_gen,
    )
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(Natural, T, U)` where `T` and `U` are unsigned, the `T` is between 2 and 36, inclusive, and
// the `U` is small.
pub fn natural_unsigned_unsigned_triple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Natural, T, U)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_triple_gen_var_1,
        &random_natural_unsigned_unsigned_triple_gen_var_1,
        &special_random_natural_unsigned_unsigned_triple_gen_var_1,
    )
}

// All `(Natural, u64, T)` where `T` is unsigned, `U` is a primitive int, the `u64` is between 1 and
// `U::WIDTH`, inclusive, and the `T` is small.
pub fn natural_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> Generator<(Natural, u64, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_triple_gen_var_2::<T, U>,
        &random_natural_unsigned_unsigned_triple_gen_var_2::<T, U>,
        &special_random_natural_unsigned_unsigned_triple_gen_var_2::<T, U>,
    )
}

// All `(Natural, T, U)` where `T` and `U` are unsigned, the `T` and the `U` are small, and the `T`
// is positive.
pub fn natural_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Natural, T, U)> {
    Generator::new(
        &exhaustive_natural_primitive_int_unsigned_triple_gen_var_3,
        &random_natural_unsigned_unsigned_triple_gen_var_3,
        &special_random_natural_unsigned_unsigned_triple_gen_var_3,
    )
}

// All `(Natural, T, T)` where `T` is unsigned, both `T`s are small, and the first `T` is less than
// or equal to the second.
pub fn natural_unsigned_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_triple_gen_var_3,
        &random_natural_unsigned_unsigned_triple_gen_var_4,
        &special_random_natural_unsigned_unsigned_triple_gen_var_4,
    )
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

// All `(Natural, T, T, Natural)` where `T` is unsigned and the first `T` is smaller than the
// second.
pub fn natural_unsigned_unsigned_natural_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, T, Natural)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_natural_quadruple_gen_var_1,
        &random_natural_unsigned_unsigned_natural_triple_gen_var_1,
        &special_random_natural_unsigned_unsigned_natural_quadruple_gen_var_1,
    )
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

// All `(Natural, T, RoundingMode)` where `T` is unsigned and the triple is a valid input to
// `Natural::shr_round`.
pub fn natural_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Generator::new(
        &exhaustive_natural_unsigned_rounding_mode_triple_gen_var_1,
        &random_natural_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_natural_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// -- (Natural, PrimitiveUnsigned, Vec<bool>) --

// All `(Natural, u64, Vec<bool>)` where the `u64` is small and the `Vec<bool>` has as many elements
// as the `Natural` has digits when expessed in base 2 to the power of the `u64`.
pub fn natural_unsigned_bool_vec_triple_gen_var_1() -> Generator<(Natural, u64, Vec<bool>)> {
    Generator::new(
        &exhaustive_natural_unsigned_bool_vec_triple_gen_var_1,
        &random_natural_unsigned_bool_vec_triple_gen_var_1,
        &special_random_natural_unsigned_bool_vec_triple_gen_var_1,
    )
}

// All `(Natural, u64, Vec<bool>)` where the `u64` is between 1 and `T::WIDTH`, inclusive, and the
// `Vec<bool>` has as many elements as the `Natural` has digits when expessed in base 2 to the power
// of the `u64`.
pub fn natural_unsigned_bool_vec_triple_gen_var_2<T: PrimitiveInt>(
) -> Generator<(Natural, u64, Vec<bool>)> {
    Generator::new(
        &exhaustive_natural_unsigned_bool_vec_triple_gen_var_2::<T>,
        &random_natural_unsigned_bool_vec_triple_gen_var_2::<T>,
        &special_random_natural_unsigned_bool_vec_triple_gen_var_2::<T>,
    )
}

// -- (Natural, RoundingMode) --

pub fn natural_rounding_mode_pair_gen() -> Generator<(Natural, RoundingMode)> {
    Generator::new(
        &exhaustive_natural_rounding_mode_pair_gen,
        &random_natural_rounding_mode_pair_gen,
        &special_random_natural_rounding_mode_pair_gen,
    )
}

// All `(Natural, RoundingMode)` pairs that are valid inputs to `T::rounding_from`.
pub fn natural_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>() -> Generator<(Natural, RoundingMode)> {
    Generator::new(
        &exhaustive_natural_rounding_mode_pair_gen_var_1::<T>,
        &random_natural_rounding_mode_pair_gen_var_1::<T>,
        &special_random_natural_rounding_mode_pair_gen_var_1::<T>,
    )
}

// All `(Natural, RoundingMode)` pairs where the `Natural` is positive.
pub fn natural_rounding_mode_pair_gen_var_2() -> Generator<(Natural, RoundingMode)> {
    Generator::new(
        &exhaustive_natural_rounding_mode_pair_gen_var_2,
        &random_natural_rounding_mode_pair_gen_var_2,
        &special_random_natural_rounding_mode_pair_gen_var_2,
    )
}

// -- (Natural, Vec<bool>) --

// All `(Natural, Vec<bool>)` pairs where the length of the `Vec` is the number of limbs of the
// `Natural`.
pub fn natural_bool_vec_pair_gen_var_1() -> Generator<(Natural, Vec<bool>)> {
    Generator::new(
        &exhaustive_natural_bool_vec_pair_gen_var_1,
        &random_natural_bool_vec_pair_gen_var_1,
        &special_random_natural_bool_vec_pair_gen_var_1,
    )
}

// All `(Natural, Vec<bool>)` pairs where the length of the `Vec` is the number of significant bits
// of the `Natural`.
pub fn natural_bool_vec_pair_gen_var_2() -> Generator<(Natural, Vec<bool>)> {
    Generator::new(
        &exhaustive_natural_bool_vec_pair_gen_var_2,
        &random_natural_bool_vec_pair_gen_var_2,
        &special_random_natural_bool_vec_pair_gen_var_2,
    )
}

// -- (PrimitiveSigned, Integer, PrimitiveSigned) --

pub fn signed_integer_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(T, Integer, T)> {
    Generator::new(
        &exhaustive_signed_integer_signed_triple_gen,
        &random_primitive_int_integer_primitive_int_triple_gen,
        &special_random_signed_integer_signed_triple_gen,
    )
}

// -- (PrimitiveSigned, Natural, PrimitiveSigned) --

pub fn signed_natural_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(T, Natural, T)> {
    Generator::new(
        &exhaustive_signed_natural_signed_triple_gen,
        &random_primitive_int_natural_primitive_int_triple_gen,
        &special_random_signed_natural_signed_triple_gen,
    )
}

// -- (PrimitiveUnsigned, Integer, PrimitiveUnsigned) --

pub fn unsigned_integer_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(T, Integer, T)> {
    Generator::new(
        &exhaustive_unsigned_integer_unsigned_triple_gen,
        &random_primitive_int_integer_primitive_int_triple_gen,
        &special_random_unsigned_integer_unsigned_triple_gen,
    )
}

// -- (PrimitiveUnsigned, Natural, PrimitiveUnsigned) --

pub fn unsigned_natural_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(T, Natural, T)> {
    Generator::new(
        &exhaustive_unsigned_natural_unsigned_triple_gen,
        &random_primitive_int_natural_primitive_int_triple_gen,
        &special_random_unsigned_natural_unsigned_triple_gen,
    )
}

// -- (String, String, String) --

// All triples of `String`s corresponding to the serialization of a `num::BigUint`, a
// `rug::Integer`, and a `Natural`, respectively, into a string. The three numbers have the same
// value.
pub fn string_triple_gen_var_1() -> Generator<(String, String, String)> {
    Generator::new(
        &exhaustive_string_triple_gen_var_1,
        &random_string_triple_gen_var_1,
        &special_random_string_triple_gen_var_1,
    )
}

// All triples of `String`s corresponding to the serialization of a `num::BigInt`, a
// `rug::Integer`, and an `Integer`, respectively, into a string. The three numbers have the same
// value.
pub fn string_triple_gen_var_2() -> Generator<(String, String, String)> {
    Generator::new(
        &exhaustive_string_triple_gen_var_2,
        &random_string_triple_gen_var_2,
        &special_random_string_triple_gen_var_2,
    )
}

// -- (Vec<Natural>, Natural) --

// All `(Vec<Natural>, Natural)` where the second element of the pair is `Large` and every element
// of the `Vec` is smaller than that second element.
pub fn natural_vec_natural_pair_gen_var_1() -> Generator<(Vec<Natural>, Natural)> {
    Generator::new_no_special(
        &exhaustive_natural_vec_natural_pair_gen_var_1,
        &random_natural_vec_natural_pair_gen_var_1,
    )
}

// All `(Vec<Natural>, Natural)` where the second element of the pair is at least 2, and every
// element of the `Vec` is smaller than the second element of the pair.
pub fn natural_vec_natural_pair_gen_var_2() -> Generator<(Vec<Natural>, Natural)> {
    Generator::new_no_special(
        &exhaustive_natural_vec_natural_pair_gen_var_2,
        &random_natural_vec_natural_pair_gen_var_2,
    )
}

// All `(Vec<Natural>, Natural)` where the second element of the pair is `Large`.
pub fn natural_vec_natural_pair_gen_var_3() -> Generator<(Vec<Natural>, Natural)> {
    Generator::new(
        &exhaustive_natural_vec_natural_pair_gen_var_3,
        &random_natural_vec_natural_pair_gen_var_3,
        &special_random_natural_vec_natural_pair_gen_var_1,
    )
}

// All `(Vec<Natural>, Natural)` where the second element of the pair is at least 2.
pub fn natural_vec_natural_pair_gen_var_4() -> Generator<(Vec<Natural>, Natural)> {
    Generator::new(
        &exhaustive_natural_vec_natural_pair_gen_var_4,
        &random_natural_vec_natural_pair_gen_var_4,
        &special_random_natural_vec_natural_pair_gen_var_2,
    )
}

// -- (Vec<Natural>, PrimitiveUnsigned) --

// All `(Vec<Natural>, u64)`, where the `u64` is positive and each `Natural` in the `Vec` is less
// than 2 to the power of the `u64`.
pub fn natural_vec_unsigned_pair_gen_var_1() -> Generator<(Vec<Natural>, u64)> {
    Generator::new(
        &exhaustive_natural_vec_unsigned_pair_gen_var_1,
        &random_natural_vec_unsigned_pair_gen_var_1,
        &special_random_natural_vec_unsigned_pair_gen_var_1,
    )
}

// All `(Vec<Natural>, T)`, where the `T` is small and positive.
pub fn natural_vec_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Vec<Natural>, T)> {
    Generator::new(
        &exhaustive_natural_vec_primitive_int_pair_gen_var_1,
        &random_natural_vec_unsigned_pair_gen_var_2,
        &special_random_natural_vec_unsigned_pair_gen_var_2,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// vars 1 through 3 are in malachite-base

// All `(Vec<T>, T>)` where `T` is unsigned, the `Vec` has at least two elements, and the `T` is
// greater than 1 and exactly convertible to the unsigned type `U`.
pub fn unsigned_vec_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_4::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_8::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_4::<T, U>,
    )
}

// vars 5 through 20 are in malachite-base

// All `(Vec<Limb>, u64)` where the `u64` is small and `limbs_slice_clear_bit_neg` applied to the
// `Vec` and `u64` doesn't panic.
pub fn unsigned_vec_unsigned_pair_gen_var_21() -> Generator<(Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_18,
        &random_unsigned_vec_unsigned_pair_gen_var_9,
        &special_random_unsigned_vec_unsigned_pair_gen_var_18,
    )
}

// var 22 is in malachite-base

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 5 are in malachite-base

// All `(Vec<Limb>, Limb, Limb)`s where both `Limb`s are positive, the `Vec` contains at least two
// elements, its last element is nonzero, and the first `Limb` is not equal to the second `Limb`
// mod the `Natural` represented by the `Vec`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_6() -> Generator<(Vec<Limb>, Limb, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_6,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_6,
    )
}

// vars 7 through 8 are in malachite-base

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

// All `(Vec<T>, u64, Vec<Limb>)` that are valid inputs to `_limbs_to_digits_small_base`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, u64, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
    )
}

// All triples of `Vec<Limb>`, `Limb`, and `Vec<Limb>` that meet the preconditions for
// `limbs_eq_limb_mod`, where the `Natural` represented by the first `Vec` is equal to the negative
// of the `Limb` mod the `Natural` represented by the second `Vec`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2(
) -> Generator<(Vec<Limb>, Limb, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
    )
}

// All triples of `Vec<Limb>`, `Limb`, and `Vec<Limb>` that meet the preconditions for
// `limbs_eq_limb_mod`, where the `Natural` represented by the first `Vec` is not equal to the
// negative of the `Limb` mod the `Natural` represented by the second `Vec`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3(
) -> Generator<(Vec<Limb>, Limb, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(Vec<T>, usize, Vec<Limb>, u64)` that are valid inputs to
// `_limbs_to_digits_small_base_basecase`.
pub fn unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, usize, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
        &random_primitive_int_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned --

// var 1 is in malachite-base

// All `(Vec<U>, Vec<T>, u64)` that are valid, `Some`-returning inputs to
// `_limbs_from_digits_small_base_basecase`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<U>, Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2,
        &random_primitive_int_vec_unsigned_vec_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2,
    )
}

// All `(Vec<U>, Vec<T>, u64)` that are inputs to `_limbs_from_digits_small_base_basecase`,
// regardless of whether they return `Some` or `None`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<U>, Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3,
        &random_primitive_int_vec_unsigned_vec_unsigned_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3,
    )
}

// vars 4 through 6 are in malachite-base.

// All triples of `Vec<Limb>`, and `Vec<Limb>`, and `Limb` that meet the preconditions for
// `limbs_eq_mod_limb`, where the `Natural` represented by the first `Vec` is equal to the
// negative of the `Natural` represented by the second `Vec` mod the `Limb`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7,
    )
}

// All triples of `Vec<Limb>`, and `Vec<Limb>`, and `Limb` that meet the preconditions for
// `limbs_eq_mod_limb`, where the `Natural` represented by the first `Vec` is not equal to the
// negative of the `Natural` represented by the second `Vec` mod the `Limb`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8,
    )
}

// var 9 is in malachite-base.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned> --

// vars 1 through 3 are in malachite-base

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_22`.
pub fn unsigned_vec_triple_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_4,
        &random_primitive_int_vec_triple_gen_var_4,
        &special_random_unsigned_vec_triple_gen_var_4,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_32`.
pub fn unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_5,
        &random_primitive_int_vec_triple_gen_var_5,
        &special_random_unsigned_vec_triple_gen_var_5,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_33`.
pub fn unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_6,
        &random_primitive_int_vec_triple_gen_var_6,
        &special_random_unsigned_vec_triple_gen_var_6,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_42`.
pub fn unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_7,
        &random_primitive_int_vec_triple_gen_var_7,
        &special_random_unsigned_vec_triple_gen_var_7,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_43`.
pub fn unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_8,
        &random_primitive_int_vec_triple_gen_var_8,
        &special_random_unsigned_vec_triple_gen_var_8,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_44`.
pub fn unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_9,
        &random_primitive_int_vec_triple_gen_var_9,
        &special_random_unsigned_vec_triple_gen_var_9,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_52`.
pub fn unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_10,
        &random_primitive_int_vec_triple_gen_var_10,
        &special_random_unsigned_vec_triple_gen_var_10,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_53`.
pub fn unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_11,
        &random_primitive_int_vec_triple_gen_var_11,
        &special_random_unsigned_vec_triple_gen_var_11,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_54`.
pub fn unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_12,
        &random_primitive_int_vec_triple_gen_var_12,
        &special_random_unsigned_vec_triple_gen_var_12,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_62`.
pub fn unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_13,
        &random_primitive_int_vec_triple_gen_var_13,
        &special_random_unsigned_vec_triple_gen_var_13,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_63`.
pub fn unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_14,
        &random_primitive_int_vec_triple_gen_var_14,
        &special_random_unsigned_vec_triple_gen_var_14,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_6h`.
pub fn unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_15,
        &random_primitive_int_vec_triple_gen_var_15,
        &special_random_unsigned_vec_triple_gen_var_15,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_8h`.
pub fn unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_16,
        &random_primitive_int_vec_triple_gen_var_16,
        &special_random_unsigned_vec_triple_gen_var_16,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that would trigger the actual FFT code of
// `_limbs_mul_greater_to_out_fft`.
pub fn unsigned_vec_triple_gen_var_17<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_17,
        &random_primitive_int_vec_triple_gen_var_17,
        &special_random_unsigned_vec_triple_gen_var_17,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_33`, and
// where the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_18<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_18,
        &random_primitive_int_vec_triple_gen_var_18,
        &special_random_unsigned_vec_triple_gen_var_18,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_6h`, and
// where the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_19<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_19,
        &random_primitive_int_vec_triple_gen_var_19,
        &special_random_unsigned_vec_triple_gen_var_19,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_8h`, and
// where the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_20<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_20,
        &random_primitive_int_vec_triple_gen_var_20,
        &special_random_unsigned_vec_triple_gen_var_20,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `_limbs_mul_greater_to_out_toom_8h`,
// where the `Vec`s would trigger the actual FFT code of `_limbs_mul_greater_to_out_fft`, and where
// the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_21<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_21,
        &random_primitive_int_vec_triple_gen_var_21,
        &special_random_unsigned_vec_triple_gen_var_21,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to both `_limbs_mul_greater_to_out_toom_32`
// and `_limbs_mul_greater_to_out_toom_43`.
pub fn unsigned_vec_triple_gen_var_22<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_22,
        &random_primitive_int_vec_triple_gen_var_22,
        &special_random_unsigned_vec_triple_gen_var_22,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to both `_limbs_mul_greater_to_out_toom_42`
// and `_limbs_mul_greater_to_out_toom_53`.
pub fn unsigned_vec_triple_gen_var_23<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_23,
        &random_primitive_int_vec_triple_gen_var_23,
        &special_random_unsigned_vec_triple_gen_var_23,
    )
}

// vars 24 through 36 are in malachite-base

// All triples of `Vec<Limb>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is equal to the negative of `Natural` represented by the second
// `Vec` mod the `Natural` represented by the third `Vec`.
pub fn unsigned_vec_triple_gen_var_37() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_37,
        &random_primitive_int_vec_triple_gen_var_36,
        &special_random_unsigned_vec_triple_gen_var_37,
    )
}

// All triples of `Vec<Limb>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is not equal to the negative of `Natural` represented by the
// second `Vec` mod the `Natural` represented by the third `Vec`.
pub fn unsigned_vec_triple_gen_var_38() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_38,
        &random_primitive_int_vec_triple_gen_var_37,
        &special_random_unsigned_vec_triple_gen_var_38,
    )
}

// var 39 is in malachite-base

// -- large types --

// vars 1 through 4 are in malachite-base

// All `(HalfGcdMatrix, Vec<Limb>, u8)` that are valid inputs to `HalfGcdMatrix::update_q`.
pub fn large_type_gen_var_5() -> Generator<(HalfGcdMatrix, Vec<Limb>, u8)> {
    Generator::new(
        &exhaustive_large_type_gen_var_5,
        &random_large_type_gen_var_5,
        &special_random_large_type_gen_var_5,
    )
}

// All `(HalfGcdMatrix1, Vec<Limb>, Vec<Limb>, Vec<Limb>)` that are valid inputs to
// `HalfGcdMatrix1::mul_vector`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_6() -> Generator<(HalfGcdMatrix1, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_large_type_gen_var_6,
        &random_large_type_gen_var_6,
        &special_random_large_type_gen_var_6,
    )
}

// All `(HalfGcdMatrix, HalfGcdMatrix1)` that are valid inputs to `HalfGcdMatrix::mul_matrix_1`.
pub fn large_type_gen_var_7() -> Generator<(HalfGcdMatrix, HalfGcdMatrix1)> {
    Generator::new(
        &exhaustive_large_type_gen_var_7,
        &random_large_type_gen_var_7,
        &special_random_large_type_gen_var_7,
    )
}

// All valid inputs to `limbs_matrix_mul_2_2`.
pub fn large_type_gen_var_8() -> Generator<T8> {
    Generator::new(
        &exhaustive_large_type_gen_var_8,
        &random_large_type_gen_var_8,
        &special_random_large_type_gen_var_8,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
