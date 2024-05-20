// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::factorial::FAC_DSC_THRESHOLD;
use crate::natural::arithmetic::gcd::half_gcd::HalfGcdMatrix1;
use crate::natural::Natural;
use crate::platform::{Limb, ODD_DOUBLEFACTORIAL_TABLE_LIMIT};
use crate::test_util::generators::common::{
    integer_integer_natural_triple_rm, integer_integer_triple_1_2_rm, integer_natural_pair_rm,
    integer_nrm, integer_pair_1_nrm, integer_pair_1_rm, integer_pair_nm, integer_pair_nrm,
    integer_pair_rm, integer_rm, integer_triple_1_rm, integer_vec_nrm,
    natural_natural_triple_1_2_rm, natural_nrm, natural_pair_1_nm, natural_pair_1_nrm,
    natural_pair_1_rm, natural_pair_nm, natural_pair_nrm, natural_pair_rm, natural_rm,
    natural_triple_1_rm, natural_triple_nrm, natural_triple_rm, natural_vec_nrm,
};
use crate::test_util::generators::exhaustive::*;
use crate::test_util::generators::random::*;
use crate::test_util::generators::special_random::*;
use crate::test_util::natural::arithmetic::gcd::OwnedHalfGcdMatrix;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::Generator;
use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs;
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
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
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
    Generator::new(
        &exhaustive_integer_gen_var_2::<T>,
        &random_integer_gen_var_2::<T>,
        &special_random_integer_gen_var_8::<T>,
    )
}

// All `Integer`s that are exactly between two adjacent floats of type `T`.
pub fn integer_gen_var_3<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>() -> Generator<Integer>
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
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

// All odd positive `Integer`s.
pub fn integer_gen_var_9() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen_var_9,
        &random_integer_gen_var_8,
        &special_random_integer_gen_var_9,
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

pub fn integer_pair_gen_nm() -> Generator<((BigInt, BigInt), (Integer, Integer))> {
    Generator::new(
        &|| integer_pair_nm(exhaustive_integer_pair_gen()),
        &|config| integer_pair_nm(random_integer_pair_gen(config)),
        &|config| integer_pair_nm(special_random_integer_pair_gen(config)),
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

// All pairs of `Integer`s where the second `Integer` is positive and odd.
pub fn integer_pair_gen_var_4() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_4,
        &random_integer_pair_gen_var_4,
        &special_random_integer_pair_gen_var_4,
    )
}

pub fn integer_pair_gen_var_4_rm() -> Generator<((rug::Integer, rug::Integer), (Integer, Integer))>
{
    Generator::new(
        &|| integer_pair_rm(exhaustive_integer_pair_gen_var_4()),
        &|config| integer_pair_rm(random_integer_pair_gen_var_4(config)),
        &|config| integer_pair_rm(special_random_integer_pair_gen_var_4(config)),
    )
}

// All coprime pairs of `Integer`s.
pub fn integer_pair_gen_var_5() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_5,
        &random_integer_pair_gen_var_5,
        &special_random_integer_pair_gen_var_5,
    )
}

// All coprime pairs of odd positive `Integer`s.
pub fn integer_pair_gen_var_6() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_6,
        &random_integer_pair_gen_var_6,
        &special_random_integer_pair_gen_var_6,
    )
}

// All pairs of `Integer`s where the second `Integer` is small and non-negative.
pub fn integer_pair_gen_var_7() -> Generator<(Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_pair_gen_var_7,
        &random_integer_pair_gen_var_7,
        &special_random_integer_pair_gen_var_7,
    )
}

pub fn integer_pair_gen_var_7_rm() -> Generator<((rug::Integer, u32), (Integer, Integer))> {
    Generator::new(
        &|| {
            Box::new(
                exhaustive_integer_pair_gen_var_7()
                    .map(|(x, y)| ((rug::Integer::from(&x), u32::exact_from(&y)), (x, y))),
            )
        },
        &|config| {
            Box::new(
                random_integer_pair_gen_var_7(config)
                    .map(|(x, y)| ((rug::Integer::from(&x), u32::exact_from(&y)), (x, y))),
            )
        },
        &|config| {
            Box::new(
                special_random_integer_pair_gen_var_7(config)
                    .map(|(x, y)| ((rug::Integer::from(&x), u32::exact_from(&y)), (x, y))),
            )
        },
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

// All triples of `Integer`s where the third `Integer` is positive and odd.
pub fn integer_triple_gen_var_2() -> Generator<(Integer, Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_triple_gen_var_2,
        &random_integer_triple_gen_var_2,
        &special_random_integer_triple_gen_var_2,
    )
}

// All triples of `Integer`s where the second and third `Integer`s are positive and odd.
pub fn integer_triple_gen_var_3() -> Generator<(Integer, Integer, Integer)> {
    Generator::new(
        &exhaustive_integer_triple_gen_var_3,
        &random_integer_triple_gen_var_3,
        &special_random_integer_triple_gen_var_3,
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

// -- (Integer, Integer, PrimitiveFloat) --

pub fn integer_integer_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Integer, Integer, T)> {
    Generator::new(
        &exhaustive_integer_integer_primitive_float_triple_gen,
        &random_integer_integer_primitive_float_triple_gen,
        &special_random_integer_integer_primitive_float_triple_gen,
    )
}

// -- (Integer, Integer, PrimitiveSigned) --

pub fn integer_integer_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Integer, Integer, T)> {
    Generator::new(
        &exhaustive_integer_integer_signed_triple_gen,
        &random_integer_integer_primitive_int_triple_gen,
        &special_random_integer_integer_signed_triple_gen,
    )
}

// -- (Integer, Integer, PrimitiveUnsigned) --

pub fn integer_integer_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Integer, Integer, T)> {
    Generator::new(
        &exhaustive_integer_integer_unsigned_triple_gen,
        &random_integer_integer_primitive_int_triple_gen,
        &special_random_integer_integer_unsigned_triple_gen,
    )
}

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

// All `(Integer, Integer, T)` where `T` is unsigned and small, and the `Integer`s are not equal mod
// 2 to the power of the `T`.
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
// `RoundingMode` is `Exact`, the first `Integer` is divisible by the second.
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

// -- (Integer, Natural, Natural) --

pub fn integer_natural_natural_triple_gen() -> Generator<(Integer, Natural, Natural)> {
    Generator::new(
        &exhaustive_integer_natural_natural_triple_gen,
        &random_integer_natural_natural_triple_gen,
        &special_random_integer_natural_natural_triple_gen,
    )
}

// -- (Integer, PrimitiveFloat) --

pub fn integer_primitive_float_pair_gen<T: PrimitiveFloat>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_primitive_float_pair_gen,
        &random_integer_primitive_float_pair_gen,
        &special_random_integer_primitive_float_pair_gen,
    )
}

pub fn integer_primitive_float_pair_gen_rm<T: PrimitiveFloat>(
) -> Generator<((rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_rm(exhaustive_integer_primitive_float_pair_gen()),
        &|config| integer_pair_1_rm(random_integer_primitive_float_pair_gen(config)),
        &|config| integer_pair_1_rm(special_random_integer_primitive_float_pair_gen(config)),
    )
}

// -- (Integer, PrimitiveFloat, PrimitiveFloat) --

pub fn integer_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Integer, T, T)> {
    Generator::new(
        &exhaustive_integer_primitive_float_primitive_float_triple_gen,
        &random_integer_primitive_float_primitive_float_triple_gen,
        &special_random_integer_primitive_float_primitive_float_triple_gen,
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

// -- (Integer, PrimitiveSigned, PrimitiveSigned) --

pub fn integer_signed_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Integer, T, T)> {
    Generator::new(
        &exhaustive_integer_signed_signed_triple_gen,
        &random_integer_primitive_int_primitive_int_triple_gen,
        &special_random_integer_signed_signed_triple_gen,
    )
}

// -- (Integer, PrimitiveSigned, PrimitiveUnsigned) --

// All `(Integer, T, U)` where `T` is signed and small and `U` is unsigned, small, and positive.
pub fn integer_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(Integer, T, U)> {
    Generator::new(
        &exhaustive_integer_signed_unsigned_triple_gen_var_1,
        &random_integer_signed_unsigned_triple_gen_var_1,
        &special_random_integer_signed_unsigned_triple_gen_var_1,
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

// All `(Integer, u64)`s where the `T` is unsigned and small, and the `Integer` is divisible by 2 to
// the power of the `T`.
pub fn integer_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_4,
        &random_integer_unsigned_pair_gen_var_4,
        &special_random_integer_unsigned_pair_gen_var_4,
    )
}

// All `(Integer, u64)`s where the `T` is unsigned and small, and the `Integer` is not divisible by
// 2 to the power of the `T`.
pub fn integer_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_5,
        &random_integer_unsigned_pair_gen_var_5,
        &special_random_integer_unsigned_pair_gen_var_5,
    )
}

// All `(Integer, T)` where `T` is unsigned, positive, and small.
pub fn integer_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_6,
        &random_integer_unsigned_pair_gen_var_6,
        &special_random_integer_unsigned_pair_gen_var_6,
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

pub fn integer_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Integer, T, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_unsigned_triple_gen,
        &random_integer_primitive_int_primitive_int_triple_gen,
        &special_random_integer_unsigned_unsigned_triple_gen,
    )
}

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

// var 3 is in malachite-float

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

// -- (Integer, ToSciOptions) --

pub fn integer_to_sci_options_pair_gen() -> Generator<(Integer, ToSciOptions)> {
    Generator::new(
        &exhaustive_integer_to_sci_options_pair_gen,
        &random_integer_to_sci_options_pair_gen,
        &special_random_integer_to_sci_options_pair_gen,
    )
}

// All `(Integer, ToSciOptions)` pairs where the `Integer` can be formatted using the options.
pub fn integer_to_sci_options_pair_gen_var_1() -> Generator<(Integer, ToSciOptions)> {
    Generator::new(
        &exhaustive_integer_to_sci_options_pair_gen_var_1,
        &random_integer_to_sci_options_pair_gen_var_1,
        &special_random_integer_to_sci_options_pair_gen_var_1,
    )
}

// -- (Integer, Vec<bool>) --

// All `(Integer, Vec<bool>)` pairs where the length of the `Vec` is the twos' complement limb count
// of the `Integer`, including sign extension limbs if necessary.
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
    Generator::new(
        &exhaustive_natural_gen_var_1,
        &random_natural_gen_var_1,
        &special_random_natural_gen_var_6,
    )
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
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
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
    Generator::new(
        &exhaustive_natural_gen_var_4::<T>,
        &random_natural_gen_var_4::<T>,
        &special_random_natural_gen_var_7::<T>,
    )
}

type GN = Generator<Natural>;

// All `Natural`s that are exactly between two adjacent floats of type `T`.
pub fn natural_gen_var_5<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>() -> GN
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
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

// All odd `Natural`s.
pub fn natural_gen_var_8() -> Generator<Natural> {
    Generator::new(
        &exhaustive_natural_gen_var_8,
        &random_natural_gen_var_8,
        &special_random_natural_gen_var_8,
    )
}

// All small `Natural`s.
pub fn natural_gen_var_9() -> Generator<Natural> {
    Generator::new_no_special(&exhaustive_natural_gen, &random_natural_gen_var_9)
}

// -- (Natural, bool) --

pub fn natural_bool_pair_gen() -> Generator<(Natural, bool)> {
    Generator::new(
        &exhaustive_natural_bool_pair_gen,
        &random_natural_bool_pair_gen,
        &special_random_natural_bool_pair_gen,
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

pub fn natural_pair_gen_nm() -> Generator<((BigUint, BigUint), (Natural, Natural))> {
    Generator::new(
        &|| natural_pair_nm(exhaustive_natural_pair_gen()),
        &|config| natural_pair_nm(random_natural_pair_gen(config)),
        &|config| natural_pair_nm(special_random_natural_pair_gen(config)),
    )
}

// All pairs of `Natural`s where the first `Natural` is large (at least 2^`Limb::WIDTH`) and the
// second is at least 2.
pub fn natural_pair_gen_var_1() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_1,
        &random_natural_pair_gen_var_1,
        &special_random_natural_pair_gen_var_10,
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

pub fn natural_pair_gen_var_5_rm() -> Generator<((rug::Integer, rug::Integer), (Natural, Natural))>
{
    Generator::new(
        &|| natural_pair_rm(exhaustive_natural_pair_gen_var_5()),
        &|config| natural_pair_rm(random_natural_pair_gen_var_5(config)),
        &|config| natural_pair_rm(special_random_natural_pair_gen_var_4(config)),
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_pair_gen_var_5_nrm() -> Generator<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Generator::new(
        &|| natural_pair_nrm(exhaustive_natural_pair_gen_var_5()),
        &|config| natural_pair_nrm(random_natural_pair_gen_var_5(config)),
        &|config| natural_pair_nrm(special_random_natural_pair_gen_var_4(config)),
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

#[allow(clippy::type_complexity)]
pub fn natural_pair_gen_var_6_nrm() -> Generator<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Generator::new(
        &|| natural_pair_nrm(exhaustive_natural_pair_gen_var_6()),
        &|config| natural_pair_nrm(random_natural_pair_gen_var_6(config)),
        &|config| natural_pair_nrm(special_random_natural_pair_gen_var_5(config)),
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

// All pairs of `Natural`s where the first is smaller than the second.
pub fn natural_pair_gen_var_8() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_8,
        &random_natural_pair_gen_var_8,
        &special_random_natural_pair_gen_var_7,
    )
}

// All pairs of positive `Natural`s.
pub fn natural_pair_gen_var_9() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_9,
        &random_natural_pair_gen_var_9,
        &special_random_natural_pair_gen_var_8,
    )
}

// All pairs of `Natural`s where the first is greater than or equal to the second.
pub fn natural_pair_gen_var_10() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_10,
        &random_natural_pair_gen_var_10,
        &special_random_natural_pair_gen_var_9,
    )
}

pub fn natural_pair_gen_var_10_rm() -> Generator<((rug::Integer, rug::Integer), (Natural, Natural))>
{
    Generator::new(
        &|| natural_pair_rm(exhaustive_natural_pair_gen_var_10()),
        &|config| natural_pair_rm(random_natural_pair_gen_var_10(config)),
        &|config| natural_pair_rm(special_random_natural_pair_gen_var_9(config)),
    )
}

pub fn natural_pair_gen_var_10_nrm() -> Generator<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Generator::new(
        &|| natural_pair_nrm(exhaustive_natural_pair_gen_var_10()),
        &|config| natural_pair_nrm(random_natural_pair_gen_var_10(config)),
        &|config| natural_pair_nrm(special_random_natural_pair_gen_var_9(config)),
    )
}

// All pairs of positive `Natural`s where the first is smaller than the second.
pub fn natural_pair_gen_var_11() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_11,
        &random_natural_pair_gen_var_11,
        &special_random_natural_pair_gen_var_11,
    )
}

// All pairs of `Natural`s where the second `Natural` is odd.
pub fn natural_pair_gen_var_12() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_12,
        &random_natural_pair_gen_var_12,
        &special_random_natural_pair_gen_var_12,
    )
}

pub fn natural_pair_gen_var_12_rm() -> Generator<((rug::Integer, rug::Integer), (Natural, Natural))>
{
    Generator::new(
        &|| natural_pair_rm(exhaustive_natural_pair_gen_var_12()),
        &|config| natural_pair_rm(random_natural_pair_gen_var_12(config)),
        &|config| natural_pair_rm(special_random_natural_pair_gen_var_12(config)),
    )
}

// All coprime pairs of odd `Natural`s.
pub fn natural_pair_gen_var_13() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_13,
        &random_natural_pair_gen_var_13,
        &special_random_natural_pair_gen_var_13,
    )
}

// All coprime pairs of `Natural`s.
pub fn natural_pair_gen_var_14() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_14,
        &random_natural_pair_gen_var_14,
        &special_random_natural_pair_gen_var_14,
    )
}

// All pairs of `Natural`s where the second `Natural` is small.
pub fn natural_pair_gen_var_15() -> Generator<(Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_pair_gen_var_15,
        &random_natural_pair_gen_var_15,
        &special_random_natural_pair_gen_var_15,
    )
}

pub fn natural_pair_gen_var_15_rm() -> Generator<((rug::Integer, u32), (Natural, Natural))> {
    Generator::new(
        &|| {
            Box::new(
                exhaustive_natural_pair_gen_var_15()
                    .map(|(x, y)| ((rug::Integer::from(&x), u32::exact_from(&y)), (x, y))),
            )
        },
        &|config| {
            Box::new(
                random_natural_pair_gen_var_15(config)
                    .map(|(x, y)| ((rug::Integer::from(&x), u32::exact_from(&y)), (x, y))),
            )
        },
        &|config| {
            Box::new(
                special_random_natural_pair_gen_var_15(config)
                    .map(|(x, y)| ((rug::Integer::from(&x), u32::exact_from(&y)), (x, y))),
            )
        },
    )
}

// -- (Natural, Natural, bool) --

// All `(Natural, Natural, bool)` where the second `Natural` is positive.
pub fn natural_natural_bool_triple_gen_var_1() -> Generator<(Natural, Natural, bool)> {
    Generator::new(
        &exhaustive_natural_natural_bool_triple_gen_var_1,
        &random_natural_natural_bool_triple_gen_var_1,
        &special_random_natural_natural_bool_triple_gen_var_1,
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

#[allow(clippy::type_complexity)]
pub fn natural_triple_gen_rm() -> Generator<(
    (rug::Integer, rug::Integer, rug::Integer),
    (Natural, Natural, Natural),
)> {
    Generator::new(
        &|| natural_triple_rm(exhaustive_natural_triple_gen()),
        &|config| natural_triple_rm(random_natural_triple_gen(config)),
        &|config| natural_triple_rm(special_random_natural_triple_gen(config)),
    )
}

// All triples of `Natural` where the first is equal to the second mod the third.
pub fn natural_triple_gen_var_1() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_1,
        &random_natural_triple_gen_var_1,
        &special_random_natural_triple_gen_var_1,
    )
}

// All triples of `Natural` where the first is not equal to the second mod the third.
pub fn natural_triple_gen_var_2() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_2,
        &random_natural_triple_gen_var_2,
        &special_random_natural_triple_gen_var_2,
    )
}

// All triples of `Natural` where the first and second elements each are less than the third.
pub fn natural_triple_gen_var_3() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_3,
        &random_natural_triple_gen_var_3,
        &special_random_natural_triple_gen_var_3,
    )
}

// All triples of `Natural` where the third `Natural` is positive.
pub fn natural_triple_gen_var_4() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_4,
        &random_natural_triple_gen_var_4,
        &special_random_natural_triple_gen_var_4,
    )
}

// All triples of `Natural` where the first element is less than the third.
pub fn natural_triple_gen_var_5() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_5,
        &random_natural_triple_gen_var_5,
        &special_random_natural_triple_gen_var_5,
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_triple_gen_var_5_nrm() -> Generator<(
    (BigUint, BigUint, BigUint),
    (rug::Integer, rug::Integer, rug::Integer),
    (Natural, Natural, Natural),
)> {
    Generator::new(
        &|| natural_triple_nrm(exhaustive_natural_triple_gen_var_5()),
        &|config| natural_triple_nrm(random_natural_triple_gen_var_5(config)),
        &|config| natural_triple_nrm(special_random_natural_triple_gen_var_5(config)),
    )
}

// All triples of positive `Natural`s.
pub fn natural_triple_gen_var_6() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_6,
        &random_natural_triple_gen_var_6,
        &special_random_natural_triple_gen_var_6,
    )
}

// All triples of `Natural`s where the first is greater than or equal to the product of the second
// and third.
pub fn natural_triple_gen_var_7() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_7,
        &random_natural_triple_gen_var_7,
        &special_random_natural_triple_gen_var_7,
    )
}

// All triples of `Natural`s where the third `Natural` is odd.
pub fn natural_triple_gen_var_8() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_8,
        &random_natural_triple_gen_var_8,
        &special_random_natural_triple_gen_var_8,
    )
}

// All triples of `Natural`s where the second and third `Natural`s are odd.
pub fn natural_triple_gen_var_9() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen_var_9,
        &random_natural_triple_gen_var_9,
        &special_random_natural_triple_gen_var_9,
    )
}

// -- (Natural, Natural, Natural, Natural) --

// All quadruples of `Natural` where the first three elements are each less than the fourth.
pub fn natural_quadruple_gen_var_1() -> Generator<(Natural, Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_quadruple_gen_var_1,
        &random_natural_quadruple_gen_var_1,
        &special_random_natural_quadruple_gen_var_1,
    )
}

// All quadruples of `Natural` where the first two elements are each smaller than the fourth.
pub fn natural_quadruple_gen_var_2() -> Generator<(Natural, Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_quadruple_gen_var_2,
        &random_natural_quadruple_gen_var_2,
        &special_random_natural_quadruple_gen_var_2,
    )
}

// All quadruples of `Natural` where the first element is less than the fourth.
pub fn natural_quadruple_gen_var_3() -> Generator<(Natural, Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_quadruple_gen_var_3,
        &random_natural_quadruple_gen_var_3,
        &special_random_natural_quadruple_gen_var_3,
    )
}

// -- (Natural, Natural, Natural, PrimitiveUnsigned) --

// All `(Natural, Natural, Natural, T)` where `T` is unsigned and small.
pub fn natural_natural_natural_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_1,
        &random_natural_natural_natural_unsigned_quadruple_gen_var_1,
        &special_random_natural_natural_natural_unsigned_quadruple_gen_var_1,
    )
}

// All `(Natural, Natural, Natural, u64)` where all `Natural`s are less than 2 to the power of the
// `u64`.
pub fn natural_natural_natural_unsigned_quadruple_gen_var_2(
) -> Generator<(Natural, Natural, Natural, u64)> {
    Generator::new(
        &exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_2,
        &random_natural_natural_natural_unsigned_quadruple_gen_var_2,
        &special_random_natural_natural_natural_unsigned_quadruple_gen_var_2,
    )
}

// All `(Natural, Natural, Natural, u64)` where the first two `Natural`s are less than 2 to the
// power of the `u64`.
pub fn natural_natural_natural_unsigned_quadruple_gen_var_3(
) -> Generator<(Natural, Natural, Natural, u64)> {
    Generator::new(
        &exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_3,
        &random_natural_natural_natural_unsigned_quadruple_gen_var_3,
        &special_random_natural_natural_natural_unsigned_quadruple_gen_var_3,
    )
}

// All `(Natural, Natural, Natural, u64)` where the first `Natural` is less than 2 to the power of
// the `u64`.
pub fn natural_natural_natural_unsigned_quadruple_gen_var_4(
) -> Generator<(Natural, Natural, Natural, u64)> {
    Generator::new(
        &exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_4,
        &random_natural_natural_natural_unsigned_quadruple_gen_var_4,
        &special_random_natural_natural_natural_unsigned_quadruple_gen_var_4,
    )
}

// -- (Natural, Natural, PrimitiveFloat) --

pub fn natural_natural_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_primitive_float_triple_gen,
        &random_natural_natural_primitive_float_triple_gen,
        &special_random_natural_natural_primitive_float_triple_gen,
    )
}

// -- (Natural, Natural, PrimitiveSigned) --

pub fn natural_natural_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_signed_triple_gen,
        &random_natural_natural_primitive_int_triple_gen,
        &special_random_natural_natural_signed_triple_gen,
    )
}

// All `(Natural, Natural, T)` where the `T` is signed and small, and the first `Natural` is smaller
// than the second.
pub fn natural_natural_signed_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_signed_triple_gen_var_1,
        &random_natural_natural_signed_triple_gen_var_1,
        &special_random_natural_natural_signed_triple_gen_var_1,
    )
}

// -- (Natural, Natural, PrimitiveUnsigned) --

pub fn natural_natural_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen,
        &random_natural_natural_primitive_int_triple_gen,
        &special_random_natural_natural_unsigned_triple_gen,
    )
}

// All `(Natural, Natural, T)` where `T` is unsigned and small.
pub fn natural_natural_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen_var_1,
        &random_natural_natural_unsigned_triple_gen_var_1,
        &special_random_natural_natural_unsigned_triple_gen_var_1,
    )
}

#[allow(clippy::type_complexity)]
pub fn natural_natural_unsigned_triple_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Integer, rug::Integer, T), (Natural, Natural, T))> {
    Generator::new(
        &|| natural_natural_triple_1_2_rm(exhaustive_natural_natural_unsigned_triple_gen_var_1()),
        &|config| {
            natural_natural_triple_1_2_rm(random_natural_natural_unsigned_triple_gen_var_1(config))
        },
        &|config| {
            natural_natural_triple_1_2_rm(special_random_natural_natural_unsigned_triple_gen_var_1(
                config,
            ))
        },
    )
}

// All `(Natural, Natural, T)` where `T` is unsigned and small, and the `Natural`s are equal mod 2
// to the power of the `T`.
pub fn natural_natural_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, T)>
where
    Natural: Shl<T, Output = Natural>,
{
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen_var_2,
        &random_natural_natural_unsigned_triple_gen_var_2,
        &special_random_natural_natural_unsigned_triple_gen_var_2,
    )
}

// All `(Natural, Natural, T)` where `T` is unsigned and small, and the `Natural`s are not equal mod
// 2 to the power of the `T`.
pub fn natural_natural_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen_var_3,
        &random_natural_natural_unsigned_triple_gen_var_3,
        &special_random_natural_natural_unsigned_triple_gen_var_3,
    )
}

// All `(Natural, Natural, u64)` where both `Natural`s are less than 2 to the power of the `u64`.
pub fn natural_natural_unsigned_triple_gen_var_4() -> Generator<(Natural, Natural, u64)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen_var_4,
        &random_natural_natural_unsigned_triple_gen_var_4,
        &special_random_natural_natural_unsigned_triple_gen_var_4,
    )
}

// All `(Natural, Natural, u64)` where the first `Natural` is less than 2 to the power of the `u64`.
pub fn natural_natural_unsigned_triple_gen_var_5() -> Generator<(Natural, Natural, u64)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen_var_5,
        &random_natural_natural_unsigned_triple_gen_var_5,
        &special_random_natural_natural_unsigned_triple_gen_var_5,
    )
}

// All `(Natural, Natural, T)` where the `T` is unsigned and small, and the first `Natural` is
// smaller than the second.
pub fn natural_natural_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> Generator<(Natural, Natural, T)> {
    Generator::new(
        &exhaustive_natural_natural_unsigned_triple_gen_var_6,
        &random_natural_natural_unsigned_triple_gen_var_6,
        &special_random_natural_natural_unsigned_triple_gen_var_6,
    )
}

// -- (Natural, Natural, RoundingMode) --

// All `(Natural, Natural, RoundingMode)` triples where the second `Natural` is positive and if the
// `RoundingMode` is `Exact`, the first `Natural` is divisible by the second.
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

// -- (Natural, PrimitiveFloat) --

pub fn natural_primitive_float_pair_gen<T: PrimitiveFloat>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_primitive_float_pair_gen,
        &random_natural_primitive_float_pair_gen,
        &special_random_natural_primitive_float_pair_gen,
    )
}

pub fn natural_primitive_float_pair_gen_rm<T: PrimitiveFloat>(
) -> Generator<((rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_rm(exhaustive_natural_primitive_float_pair_gen()),
        &|config| natural_pair_1_rm(random_natural_primitive_float_pair_gen(config)),
        &|config| natural_pair_1_rm(special_random_natural_primitive_float_pair_gen(config)),
    )
}

// -- (Natural, PrimitiveFloat, PrimitiveFloat) --

pub fn natural_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Natural, T, T)> {
    Generator::new(
        &exhaustive_natural_primitive_float_primitive_float_triple_gen,
        &random_natural_primitive_float_primitive_float_triple_gen,
        &special_random_natural_primitive_float_primitive_float_triple_gen,
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

pub fn natural_signed_pair_gen_var_2_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Integer, T), (Natural, T))> {
    Generator::new(
        &|| natural_pair_1_rm(exhaustive_natural_signed_pair_gen_var_2()),
        &|config| natural_pair_1_rm(random_natural_signed_pair_gen_var_2(config)),
        &|config| natural_pair_1_rm(special_random_natural_signed_pair_gen_var_2(config)),
    )
}

// All `(Natural, T)` where `T` is signed and the `Natural` is positive.
pub fn natural_signed_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_signed_pair_gen_var_3,
        &random_natural_primitive_int_pair_gen_var_1,
        &special_random_natural_signed_pair_gen_var_3,
    )
}

// All `(Natural, T)` where `T` is signed and small, the `Natural` is positive, and the `Natural`s
// bit length is a multiple of the limb bit length.
pub fn natural_signed_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_signed_pair_gen_var_4,
        &random_natural_signed_pair_gen_var_3,
        &special_random_natural_signed_pair_gen_var_4,
    )
}

// -- (Natural, PrimitiveSigned, PrimitiveSigned) --

pub fn natural_signed_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Natural, T, T)> {
    Generator::new(
        &exhaustive_natural_signed_signed_triple_gen,
        &random_natural_primitive_int_primitive_int_triple_gen,
        &special_random_natural_signed_signed_triple_gen,
    )
}

// -- (Natural, PrimitiveSigned, PrimitiveUnsigned) --

type T2<T> = Generator<(Natural, T, u64)>;
// All `(Natural, T, u64)` where the `Natural` is less than 2 to the power of the `u64`, and the `T`
// is small and signed.
pub fn natural_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned>() -> T2<T> {
    Generator::new(
        &exhaustive_natural_signed_unsigned_triple_gen_var_1,
        &random_natural_signed_unsigned_triple_gen_var_1,
        &special_random_natural_signed_unsigned_triple_gen_var_1,
    )
}

// All `(Natural, T, U)` where `T` is signed and small and `U` is unsigned, small, and positive.
pub fn natural_signed_unsigned_triple_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(Natural, T, U)> {
    Generator::new(
        &exhaustive_natural_signed_unsigned_triple_gen_var_2,
        &random_natural_signed_unsigned_triple_gen_var_2,
        &special_random_natural_signed_unsigned_triple_gen_var_2,
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
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_3,
        &random_natural_unsigned_pair_gen_var_5,
        &special_random_natural_unsigned_pair_gen_var_12,
    )
}

// All `(Natural, T)`, where the `T` is between 1 and `T::WIDTH`, inclusive.
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

// All `(Natural, u64)`s where the `T` is unsigned and small, and the `Natural` is divisible by 2 to
// the power of the `T`.
pub fn natural_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_5,
        &random_natural_unsigned_pair_gen_var_9,
        &special_random_natural_unsigned_pair_gen_var_8,
    )
}

// All `(Natural, u64)`s where the `T` is unsigned and small, and the `Natural` is not divisible by
// 2 to the power of the `T`.
pub fn natural_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_6,
        &random_natural_unsigned_pair_gen_var_10,
        &special_random_natural_unsigned_pair_gen_var_9,
    )
}

// All `(Natural, u64)` where the `Natural` is less than 2 to the power of the `u64`.
pub fn natural_unsigned_pair_gen_var_11() -> Generator<(Natural, u64)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_7,
        &random_natural_unsigned_pair_gen_var_11,
        &special_random_natural_unsigned_pair_gen_var_10,
    )
}

// All `(Natural, T)` where the `Natural` is positive and the `T` is unsigned.
pub fn natural_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_8,
        &random_natural_primitive_int_pair_gen_var_1,
        &special_random_natural_unsigned_pair_gen_var_11,
    )
}

// All `(Natural, T)` where the `Natural` is positive and the `T` is unsigned and small.
pub fn natural_unsigned_pair_gen_var_13<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_9,
        &random_natural_unsigned_pair_gen_var_12,
        &special_random_natural_unsigned_pair_gen_var_13,
    )
}

// All `(Natural, u64)` where the `Natural` is nonzero and less than 2 to the power of the `u64`.
pub fn natural_unsigned_pair_gen_var_14() -> Generator<(Natural, u64)> {
    Generator::new(
        &exhaustive_natural_unsigned_pair_gen_var_10,
        &random_natural_unsigned_pair_gen_var_13,
        &special_random_natural_unsigned_pair_gen_var_14,
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

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn natural_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Natural, T, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_triple_gen,
        &random_natural_primitive_int_primitive_int_triple_gen,
        &special_random_natural_unsigned_unsigned_triple_gen,
    )
}

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

// All `(Natural, T, T)` where `T` is unsigned and both `T`s are small.
pub fn natural_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, T)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_triple_gen_var_4,
        &random_natural_unsigned_unsigned_triple_gen_var_5,
        &special_random_natural_unsigned_unsigned_triple_gen_var_5,
    )
}

// All `(Natural, T, u64)` where the `Natural` is less than 2 to the power of the `u64`, and the `T`
// is small and unsigned.
pub fn natural_unsigned_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> Generator<(Natural, T, u64)> {
    Generator::new(
        &exhaustive_natural_unsigned_unsigned_triple_gen_var_5,
        &random_natural_unsigned_unsigned_triple_gen_var_6,
        &special_random_natural_unsigned_unsigned_triple_gen_var_6,
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

// var 2 is in malachite-float

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

// -- (Natural, ToSciOptions) --

pub fn natural_to_sci_options_pair_gen() -> Generator<(Natural, ToSciOptions)> {
    Generator::new(
        &exhaustive_natural_to_sci_options_pair_gen,
        &random_natural_to_sci_options_pair_gen,
        &special_random_natural_to_sci_options_pair_gen,
    )
}

// All `(Natural, ToSciOptions)` pairs where the `Natural` can be formatted using the options.
pub fn natural_to_sci_options_pair_gen_var_1() -> Generator<(Natural, ToSciOptions)> {
    Generator::new(
        &exhaustive_natural_to_sci_options_pair_gen_var_1,
        &random_natural_to_sci_options_pair_gen_var_1,
        &special_random_natural_to_sci_options_pair_gen_var_1,
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

// -- (PrimitiveUnsigned, bool) --

#[allow(clippy::redundant_comparisons)]
pub(crate) const fn limbs_odd_factorial_valid(n: usize, b: bool) -> bool {
    !b || n > ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 1 && n >= FAC_DSC_THRESHOLD
}

// All `(T, u63>)` that are valid arguments to `limbs_odd_factorial`.
pub fn unsigned_bool_pair_gen_var_1() -> Generator<(usize, bool)> {
    Generator::new_no_special(
        &exhaustive_unsigned_bool_pair_gen_var_1,
        &random_unsigned_bool_pair_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 44 are in malachite-base

// All `(T, T)` where `T` is unsigned, both `T`s are small, the first `T` is greater than or equal
// to the second, and both are greater than `ODD_FACTORIAL_TABLE_LIMIT`.
pub fn unsigned_pair_gen_var_45<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_32,
        &random_unsigned_pair_gen_var_33,
    )
}

// All `(T, T)` where `T` is unsigned, both `T`s are small, the first `T` is greater than or equal
// to the second, and the second is at least 2 and no greater than `ODD_FACTORIAL_TABLE_LIMIT`.
pub fn unsigned_pair_gen_var_46<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_33,
        &random_unsigned_pair_gen_var_34,
    )
}

// All `(T, T)` where `T` is unsigned, both `T`s are small, the first `T` is at least 2 more than
// the second, the second is at least 2, and the first is no greater than
// `ODD_FACTORIAL_EXTTABLE_LIMIT`.
pub fn unsigned_pair_gen_var_47<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_34,
        &random_unsigned_pair_gen_var_35,
    )
}

// All `(T, T)` where `T` is unsigned, both `T`s are small, the first `T` is greater than or equal
// to the second, and both are greater than `ODD_FACTORIAL_TABLE_LIMIT`.
pub fn unsigned_pair_gen_var_48<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_35,
        &random_unsigned_pair_gen_var_36,
    )
}

// All `(Limb, Limb)` that are valid inputs to `limbs_binomial_coefficient_limb_limb_goetgheluck`.
pub fn unsigned_pair_gen_var_49() -> Generator<(Limb, Limb)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_36,
        &random_unsigned_pair_gen_var_37,
    )
}

// -- (PrimitiveUnsigned * 6) --

// var 2 is in malachite-base.

// All sextuples of unsigneds that are valid inputs to `limbs_div_mod_three_limb_by_two_limb`.
pub fn unsigned_sextuple_gen_var_2() -> Generator<(Limb, Limb, Limb, Limb, Limb, Limb)> {
    Generator::new(
        &exhaustive_unsigned_sextuple_gen_var_2,
        &random_unsigned_sextuple_gen_var_1,
        &special_random_unsigned_sextuple_gen_var_2,
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

// All triples of `String`s corresponding to the serialization of a `num::BigInt`, a `rug::Integer`,
// and an `Integer`, respectively, into a string. The three numbers have the same value.
pub fn string_triple_gen_var_2() -> Generator<(String, String, String)> {
    Generator::new(
        &exhaustive_string_triple_gen_var_2,
        &random_string_triple_gen_var_2,
        &special_random_string_triple_gen_var_2,
    )
}

// var 3 is in malachite-q.

// -- Vec<Integer> --

pub fn integer_vec_gen() -> Generator<Vec<Integer>> {
    Generator::new(
        &exhaustive_integer_vec_gen,
        &random_integer_vec_gen,
        &special_random_integer_vec_gen,
    )
}

pub fn integer_vec_gen_nrm() -> Generator<(Vec<BigInt>, Vec<rug::Integer>, Vec<Integer>)> {
    Generator::new(
        &|| integer_vec_nrm(exhaustive_integer_vec_gen()),
        &|config| integer_vec_nrm(random_integer_vec_gen(config)),
        &|config| integer_vec_nrm(special_random_integer_vec_gen(config)),
    )
}

// -- Vec<Natural> --

pub fn natural_vec_gen() -> Generator<Vec<Natural>> {
    Generator::new(
        &exhaustive_natural_vec_gen,
        &random_natural_vec_gen,
        &special_random_natural_vec_gen,
    )
}

pub fn natural_vec_gen_nrm() -> Generator<(Vec<BigUint>, Vec<rug::Integer>, Vec<Natural>)> {
    Generator::new(
        &|| natural_vec_nrm(exhaustive_natural_vec_gen()),
        &|config| natural_vec_nrm(random_natural_vec_gen(config)),
        &|config| natural_vec_nrm(special_random_natural_vec_gen(config)),
    )
}

// -- (Vec<Natural>, Integer) --

// All `(Vec<Natural>, Integer)` where the `Natural`s are positive.
pub fn natural_vec_integer_pair_gen_var_1() -> Generator<(Vec<Natural>, Integer)> {
    Generator::new(
        &exhaustive_natural_vec_integer_pair_gen_var_1,
        &random_natural_vec_integer_pair_gen_var_1,
        &special_random_natural_vec_integer_pair_gen_var_1,
    )
}

// -- (Vec<Natural>, Natural) --

// All `(Vec<Natural>, Natural)` where the second element of the pair is `Large` and every element
// of the `Vec` is smaller than that second element.
pub fn natural_vec_natural_pair_gen_var_1() -> Generator<(Vec<Natural>, Natural)> {
    Generator::new(
        &exhaustive_natural_vec_natural_pair_gen_var_1,
        &random_natural_vec_natural_pair_gen_var_1,
        &special_random_natural_vec_natural_pair_gen_var_3,
    )
}

// All `(Vec<Natural>, Natural)` where the second element of the pair is at least 2, and every
// element of the `Vec` is smaller than the second element of the pair.
pub fn natural_vec_natural_pair_gen_var_2() -> Generator<(Vec<Natural>, Natural)> {
    Generator::new(
        &exhaustive_natural_vec_natural_pair_gen_var_2,
        &random_natural_vec_natural_pair_gen_var_2,
        &striped_random_natural_vec_natural_pair_gen_var_4,
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

// -- Vec<PrimitiveUnsigned> --

// vars 1 through 4 are in malachite-base.

// All `Vec<Limb>` that are nonempty and represent a `Natural` divisible by 3.
pub fn unsigned_vec_gen_var_5() -> Generator<Vec<Limb>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen_var_5,
        &random_unsigned_vec_gen_var_1,
        &special_random_unsigned_vec_gen_var_5,
    )
}

// var 6 is in malachite-base.

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

// vars 22 through 28 are in malachite-base.

// All `(Vec<Limb>, Limb)` where the `Vec` is nonempty and represents a `Natural` divisible by the
// `Limb`, and the `Limb` is positive.
pub fn unsigned_vec_unsigned_pair_gen_var_29() -> Generator<(Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_19,
        &random_unsigned_vec_unsigned_pair_gen_var_10,
        &special_random_unsigned_vec_unsigned_pair_gen_var_26,
    )
}

// All `(Vec<Limb>, u64)` where the `u64` is small, the `Vec` ends with a nonzero element, and the
// number of significant bits of the `Vec` is less than or equal to the `u64`.
pub fn unsigned_vec_unsigned_pair_gen_var_30() -> Generator<(Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_20,
        &random_unsigned_vec_unsigned_pair_gen_var_11,
        &special_random_unsigned_vec_unsigned_pair_gen_var_27,
    )
}

// vars 31 through 32 are in malachite-base.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 5 are in malachite-base

// All `(Vec<Limb>, Limb, Limb)`s where both `Limb`s are positive, the `Vec` contains at least two
// elements, its last element is nonzero, and the first `Limb` is not equal to the second `Limb` mod
// the `Natural` represented by the `Vec`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_6() -> Generator<(Vec<Limb>, Limb, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_6,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_6,
    )
}

// vars 7 through 8 are in malachite-base.

// All `(Vec<Limb>, Limb, Limb)`s where the first `Limb` is a factor of `Limb::MAX`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_9() -> Generator<(Vec<Limb>, Limb, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_9,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_9,
    )
}

// var 10 is in malachite-base.

// All triples of `(Vec<Limb>, Limb, Limb)` that are valid inputs to `limbs_eq_limb_mod_limb`, such
// that `limbs_eq_limb_mod_limb` would return `true`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_11() -> Generator<(Vec<Limb>, Limb, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_11,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_4,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_11,
    )
}

// All triples of `(Vec<Limb>, Limb, Limb)` that are valid inputs to `limbs_eq_limb_mod_limb`, such
// that `limbs_eq_limb_mod_limb` would return `false`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_12() -> Generator<(Vec<Limb>, Limb, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_12,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_5,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_12,
    )
}

// var 13 is in malachite-base.

// All `(Vec<Limb>, T, u64)` where `T` is unsigned, the `u64` is small, and the number of
// significant bits of both the `Vec` and the `Limb` are less than or equal to the `u64`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_14<T: PrimitiveUnsigned>(
) -> Generator<(Vec<Limb>, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_14,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_6,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_14,
    )
}

// All `(Vec<Limb>, T, u64)` where `T` is unsigned, the `Vec` is nonempty, the `u64` is small, and
// the number of significant bits of both the `Vec` and the `Limb` are less than or equal to the
// `u64`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_15<T: PrimitiveUnsigned>(
) -> Generator<(Vec<Limb>, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_15,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_7,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_14,
    )
}

// All `(Vec<Limb>, T, u64)` where `T` is unsigned, the `u64` is small and positive, and the number
// of significant bits of both the `Vec` and the `Limb` are less than or equal to the `u64`.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_16<T: PrimitiveUnsigned>(
) -> Generator<(Vec<Limb>, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_16,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_8,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_16,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

// All `(Vec<T>, u64, Vec<Limb>)` that are valid inputs to `limbs_to_digits_small_base`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, u64, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
    )
}

// All triples of `(Vec<Limb>, Limb, Vec<Limb>)` that are valid inputs to
// `limbs_pos_eq_neg_limb_mod`, such that `limbs_pos_eq_neg_limb_mod` would return `true`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2(
) -> Generator<(Vec<Limb>, Limb, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
    )
}

// All triples of `(Vec<Limb>, Limb, Vec<Limb>)` that are valid inputs to
// `limbs_pos_eq_neg_limb_mod`, such that `limbs_pos_eq_neg_limb_mod` would return `false`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3(
) -> Generator<(Vec<Limb>, Limb, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
    )
}

// All triples of `(Vec<Limb>, Limb, Vec<Limb>)` that are valid inputs to `limbs_eq_limb_mod`, such
// that `limbs_eq_limb_mod` would return `true`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4(
) -> Generator<(Vec<Limb>, Limb, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4,
    )
}

// All triples of `(Vec<Limb>, Limb, Vec<Limb>)` that are valid inputs to `limbs_eq_limb_mod`, such
// that `limbs_eq_limb_mod` would return `false`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5(
) -> Generator<(Vec<Limb>, Limb, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(Vec<T>, usize, Vec<Limb>, u64)` that are valid inputs to
// `limbs_to_digits_small_base_basecase`.
pub fn unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, usize, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
        &random_primitive_int_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 9 are in malachite-base.

// All `(Vec<Limb>, Vec<Limb>)` where each `Vec` has length at least 2 and ends in a nonzero value,
// the first `Vec` represents an integer at least as large as the second, and the first elements of
// each `Vec` are not both even.
pub fn unsigned_vec_pair_gen_var_10() -> Generator<(Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_11,
        &random_primitive_int_vec_pair_gen_var_9,
        &special_random_unsigned_vec_pair_gen_var_11,
    )
}

// vars 11 through 12 are in malachite-base.

// All `(Vec<Limb>, Vec<Limb>)` where the first `Vec` is at least as long as the second and the
// second `Vec` is nonempty and represents a `Natural` divisible by 3.
pub fn unsigned_vec_pair_gen_var_13() -> Generator<(Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_14,
        &random_unsigned_vec_pair_gen_var_3,
        &special_random_unsigned_vec_pair_gen_var_14,
    )
}

// All `(Vec<Limb>, Vec<Limb>)` that are valid inputs to `limbs_div_exact`.
pub fn unsigned_vec_pair_gen_var_14() -> Generator<(Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_15,
        &random_unsigned_vec_pair_gen_var_4,
        &special_random_unsigned_vec_pair_gen_var_15,
    )
}

// vars 15 through 16 are in malachite-base.

// All `(Vec<Limb>, Vec<Limb>)` that are valid inputs to `limbs_div_exact` and `limbs_divisible_by`.
pub fn unsigned_vec_pair_gen_var_17() -> Generator<(Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_18,
        &random_primitive_int_vec_pair_gen_var_14,
        &special_random_unsigned_vec_pair_gen_var_18,
    )
}

// vars 18 through 20 are in malachite-base.

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_low_basecase`.
pub fn unsigned_vec_pair_gen_var_21<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_22,
        &random_primitive_int_vec_pair_gen_var_16,
        &special_random_unsigned_vec_pair_gen_var_22,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_to_out_basecase`.
pub fn unsigned_vec_pair_gen_var_22<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_23,
        &random_primitive_int_vec_pair_gen_var_17,
        &special_random_unsigned_vec_pair_gen_var_23,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_to_out_toom_2`.
pub fn unsigned_vec_pair_gen_var_23<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_24,
        &random_primitive_int_vec_pair_gen_var_18,
        &special_random_unsigned_vec_pair_gen_var_24,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_to_out_toom_3`.
pub fn unsigned_vec_pair_gen_var_24<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_25,
        &random_primitive_int_vec_pair_gen_var_19,
        &special_random_unsigned_vec_pair_gen_var_25,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_to_out_toom_4`.
pub fn unsigned_vec_pair_gen_var_25<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_26,
        &random_primitive_int_vec_pair_gen_var_20,
        &special_random_unsigned_vec_pair_gen_var_26,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to both
// `limbs_square_to_out_toom_3` and `limbs_square_to_out_toom_4`.
pub fn unsigned_vec_pair_gen_var_26<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_27,
        &random_primitive_int_vec_pair_gen_var_21,
        &special_random_unsigned_vec_pair_gen_var_27,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_to_out_toom_6`.
pub fn unsigned_vec_pair_gen_var_27<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_28,
        &random_primitive_int_vec_pair_gen_var_22,
        &special_random_unsigned_vec_pair_gen_var_28,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_to_out_toom_8`.
pub fn unsigned_vec_pair_gen_var_28<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_29,
        &random_primitive_int_vec_pair_gen_var_23,
        &special_random_unsigned_vec_pair_gen_var_29,
    )
}

// vars 31 to 32 are in malachite-base.

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and `out` and `xs` are valid inputs to
// `limbs_square_fft_alt`.
pub fn unsigned_vec_pair_gen_var_33<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_34,
        &random_primitive_int_vec_pair_gen_var_28,
        &special_random_unsigned_vec_pair_gen_var_34,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// var 1 is in malachite-base

// All `(Vec<U>, Vec<T>, u64)` that are valid, `Some`-returning inputs to
// `limbs_from_digits_small_base_basecase`.
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

// All `(Vec<U>, Vec<T>, u64)` that are inputs to `limbs_from_digits_small_base_basecase`,
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

// All triples of `(Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_pos_eq_neg_mod_limb`, such that `limbs_pos_eq_neg_mod_limb` would return `true`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7,
    )
}

// All triples of `(Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_pos_eq_neg_mod_limb`, such that `limbs_pos_eq_neg_mod_limb` would return `false`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8,
    )
}

// vars 9 through 13 are in malachite-base.

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned and positive, the first `Vec` is at least as long
// as the second, and the second `Vec` is nonempty and represents a `Natural` divisible by the
// `Limb`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14,
    )
}

// All triples of `(Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to `limbs_eq_limb_mod`, such
// that `limbs_eq_limb_mod` would return `true`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_15(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_15,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_15,
    )
}

// All triples of `(Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to `limbs_eq_limb_mod`, such
// that `limbs_eq_limb_mod` would return `false`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_16(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_16,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_16,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to `limbs_mod_schoolbook`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_17(
) -> Generator<(Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_17,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_17,
    )
}

// All `(Vec<Limb>, Vec<Limb>, u64)` where the `u64` is small and the number of significant bits of
// both `Vec`s are less than or equal to the `u64`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18(
) -> Generator<(Vec<Limb>, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_7,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_18,
    )
}

// All `(Vec<Limb>, Vec<Limb>, u64)` where the `u64` is small, the first `Vec` is at least as long
// as the second, and the number of significant bits of both `Vec`s are less than or equal to the
// `u64`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19(
) -> Generator<(Vec<Limb>, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_8,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_19,
    )
}

// All `(Vec<Limb>, Vec<Limb>, u64)` where the `u64` is small, the number of significant bits of
// both `Vec`s are less than or equal to the `u64`, and both `Vec`s end with a nonzero value.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20(
) -> Generator<(Vec<Limb>, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_9,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_20,
    )
}

// All `(Vec<Limb>, Vec<Limb>, u64)` that are valid inputs to `limbs_mod_power_of_2_pow`, and where
// the `u64` is small.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_21(
) -> Generator<(Vec<Limb>, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_21,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_10,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_21,
    )
}

// vars 22 through 23 are in malachite-base.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned> --

// vars 1 through 3 are in malachite-base

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_22`.
pub fn unsigned_vec_triple_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_4,
        &random_primitive_int_vec_triple_gen_var_4,
        &special_random_unsigned_vec_triple_gen_var_4,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_32`.
pub fn unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_5,
        &random_primitive_int_vec_triple_gen_var_5,
        &special_random_unsigned_vec_triple_gen_var_5,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_33`.
pub fn unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_6,
        &random_primitive_int_vec_triple_gen_var_6,
        &special_random_unsigned_vec_triple_gen_var_6,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_42`.
pub fn unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_7,
        &random_primitive_int_vec_triple_gen_var_7,
        &special_random_unsigned_vec_triple_gen_var_7,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_43`.
pub fn unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_8,
        &random_primitive_int_vec_triple_gen_var_8,
        &special_random_unsigned_vec_triple_gen_var_8,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_44`.
pub fn unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_9,
        &random_primitive_int_vec_triple_gen_var_9,
        &special_random_unsigned_vec_triple_gen_var_9,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_52`.
pub fn unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_10,
        &random_primitive_int_vec_triple_gen_var_10,
        &special_random_unsigned_vec_triple_gen_var_10,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_53`.
pub fn unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_11,
        &random_primitive_int_vec_triple_gen_var_11,
        &special_random_unsigned_vec_triple_gen_var_11,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_54`.
pub fn unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_12,
        &random_primitive_int_vec_triple_gen_var_12,
        &special_random_unsigned_vec_triple_gen_var_12,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_62`.
pub fn unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_13,
        &random_primitive_int_vec_triple_gen_var_13,
        &special_random_unsigned_vec_triple_gen_var_13,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_63`.
pub fn unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_14,
        &random_primitive_int_vec_triple_gen_var_14,
        &special_random_unsigned_vec_triple_gen_var_14,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_6h`.
pub fn unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_15,
        &random_primitive_int_vec_triple_gen_var_15,
        &special_random_unsigned_vec_triple_gen_var_15,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_8h`.
pub fn unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_16,
        &random_primitive_int_vec_triple_gen_var_16,
        &special_random_unsigned_vec_triple_gen_var_16,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_33`, and
// where the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_18<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_18,
        &random_primitive_int_vec_triple_gen_var_18,
        &special_random_unsigned_vec_triple_gen_var_18,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_6h`, and
// where the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_19<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_19,
        &random_primitive_int_vec_triple_gen_var_19,
        &special_random_unsigned_vec_triple_gen_var_19,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_mul_greater_to_out_toom_8h`, and
// where the second and third `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_20<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_20,
        &random_primitive_int_vec_triple_gen_var_20,
        &special_random_unsigned_vec_triple_gen_var_20,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to both `limbs_mul_greater_to_out_toom_32`
// and `limbs_mul_greater_to_out_toom_43`.
pub fn unsigned_vec_triple_gen_var_22<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_22,
        &random_primitive_int_vec_triple_gen_var_22,
        &special_random_unsigned_vec_triple_gen_var_22,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to both `limbs_mul_greater_to_out_toom_42`
// and `limbs_mul_greater_to_out_toom_53`.
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

// vars 39 through 41 are in malachite-base.

// All triples of `Vec<Limb>` that are valid inputs to `limbs_div_barrett_approx`.
pub fn unsigned_vec_triple_gen_var_42() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_42,
        &random_unsigned_vec_triple_gen_var_1,
        &special_random_unsigned_vec_triple_gen_var_42,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_div_barrett`.
pub fn unsigned_vec_triple_gen_var_43() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_43,
        &random_unsigned_vec_triple_gen_var_2,
        &special_random_unsigned_vec_triple_gen_var_43,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_div_to_out`.
pub fn unsigned_vec_triple_gen_var_44() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_44,
        &random_unsigned_vec_triple_gen_var_3,
        &special_random_unsigned_vec_triple_gen_var_44,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_div_to_out` and both the balanced and
// unbalanced div helper functions.
pub fn unsigned_vec_triple_gen_var_45() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_45,
        &random_unsigned_vec_triple_gen_var_4,
        &special_random_unsigned_vec_triple_gen_var_45,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_modular_div_barrett`.
pub fn unsigned_vec_triple_gen_var_46() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_46,
        &random_unsigned_vec_triple_gen_var_5,
        &special_random_unsigned_vec_triple_gen_var_46,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_modular_div`.
pub fn unsigned_vec_triple_gen_var_47() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_47,
        &random_unsigned_vec_triple_gen_var_6,
        &special_random_unsigned_vec_triple_gen_var_47,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_div_exact_to_out`.
pub fn unsigned_vec_triple_gen_var_48() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_48,
        &random_unsigned_vec_triple_gen_var_7,
        &special_random_unsigned_vec_triple_gen_var_48,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to both `limbs_div_to_out` and
// `limbs_div_exact_to_out`.
pub fn unsigned_vec_triple_gen_var_49() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_49,
        &random_unsigned_vec_triple_gen_var_8,
        &special_random_unsigned_vec_triple_gen_var_49,
    )
}

// vars 50 through 53 are in malachite-base.

// All triples of `Vec<Limb>` that are valid inputs to `limbs_eq_mod`, such that `limbs_eq_mod`
// would return `true`.
pub fn unsigned_vec_triple_gen_var_54() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_54,
        &random_unsigned_vec_triple_gen_var_11,
        &special_random_unsigned_vec_triple_gen_var_54,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_eq_mod`, such that `limbs_eq_mod`
// would return `false`.
pub fn unsigned_vec_triple_gen_var_55() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_55,
        &random_unsigned_vec_triple_gen_var_12,
        &special_random_unsigned_vec_triple_gen_var_55,
    )
}

// All triples of `Vec<Limb>` that are valid inputs to `limbs_div_mod_by_two_limb_normalized`.
pub fn unsigned_vec_triple_gen_var_56() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_56,
        &random_unsigned_vec_triple_gen_var_13,
        &special_random_unsigned_vec_triple_gen_var_56,
    )
}

// var 57 is in malachite-base.

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to both `limbs_mul_greater_to_out_toom_33`
// and `limbs_mul_greater_to_out_toom_33`.
pub fn unsigned_vec_triple_gen_var_58<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_58,
        &random_primitive_int_vec_triple_gen_var_45,
        &special_random_unsigned_vec_triple_gen_var_58,
    )
}

// var 59 is in malachite-base.

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to both `limbs_mul_fft_alt`.
pub fn unsigned_vec_triple_gen_var_60<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_60,
        &random_primitive_int_vec_triple_gen_var_47,
        &special_random_unsigned_vec_triple_gen_var_60,
    )
}

// -- (Vec<PrimitiveUnsigned> * 4) --

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_div_mod_to_out`.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_1() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_1,
        &random_unsigned_vec_quadruple_gen_var_1,
        &special_random_unsigned_vec_quadruple_gen_var_1,
    )
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_modular_div_mod_barrett`.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_2() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_2,
        &random_unsigned_vec_quadruple_gen_var_2,
        &special_random_unsigned_vec_quadruple_gen_var_2,
    )
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_modular_div_mod_barrett`, and the
// first, second and third `Vec`s would meet the preconditions of
// `limbs_modular_div_mod_divide_and_conquer`, given the correct `inverse`.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_3() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_3,
        &random_unsigned_vec_quadruple_gen_var_3,
        &special_random_unsigned_vec_quadruple_gen_var_3,
    )
}

// All quadruples of `Vec<Limb>` that meet certain preconditions that enable comparing the
// performance of two kinds of Barrett division.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_4() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_4,
        &random_unsigned_vec_quadruple_gen_var_4,
        &special_random_unsigned_vec_quadruple_gen_var_4,
    )
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_div_mod_barrett`.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_5() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_5,
        &random_unsigned_vec_quadruple_gen_var_5,
        &special_random_unsigned_vec_quadruple_gen_var_5,
    )
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_mod_pow`.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_6() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_6,
        &random_unsigned_vec_quadruple_gen_var_6,
        &special_random_unsigned_vec_quadruple_gen_var_6,
    )
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_mod_pow_odd`.
#[allow(clippy::type_complexity)]
pub fn unsigned_vec_quadruple_gen_var_7() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_quadruple_gen_var_7,
        &random_unsigned_vec_quadruple_gen_var_7,
        &special_random_unsigned_vec_quadruple_gen_var_7,
    )
}

// -- large types --

// vars 1 through 4 are in malachite-base

// All `(HalfGcdMatrix, Vec<Limb>, u8)` that are valid inputs to `HalfGcdMatrix::update_q`.
pub fn large_type_gen_var_5() -> Generator<(OwnedHalfGcdMatrix, Vec<Limb>, u8)> {
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
pub fn large_type_gen_var_7() -> Generator<(OwnedHalfGcdMatrix, HalfGcdMatrix1)> {
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

// var 9 is in malachite-base.

#[cfg(feature = "32_bit_limbs")]
const PRIME_FACTORS_OF_LIMB_MAX: &[Limb] = &[3, 5, 17, 257, 65_537];
#[cfg(not(feature = "32_bit_limbs"))]
const PRIME_FACTORS_OF_LIMB_MAX: &[Limb] = &[3, 5, 17, 257, 641, 65_537, 6_700_417];

pub(crate) fn factors_of_limb_max() -> Vec<Limb> {
    lex_ordered_unique_vecs(PRIME_FACTORS_OF_LIMB_MAX.iter())
        .map(|pfs| pfs.into_iter().product())
        .collect()
}

// All `(Vec<Limb>, Vec<Limb>, Limb, Limb)` where the first `Vec` is at least as long as the second,
// and the first `Limb` is a divisor of `Limb::MAX`.
pub fn large_type_gen_var_10() -> Generator<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_10,
        &random_large_type_gen_var_10,
        &special_random_large_type_gen_var_10,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_div_mod_schoolbook`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_11() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_11,
        &random_large_type_gen_var_11,
        &special_random_large_type_gen_var_11,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_div_mod_divide_and_conquer`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_12() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_12,
        &random_large_type_gen_var_12,
        &special_random_large_type_gen_var_12,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_modular_div_schoolbook`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_13() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_13,
        &random_large_type_gen_var_13,
        &special_random_large_type_gen_var_13,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_modular_div_mod_schoolbook`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_14() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_14,
        &random_large_type_gen_var_14,
        &special_random_large_type_gen_var_14,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_modular_div_mod_divide_and_conquer`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_15() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_15,
        &random_large_type_gen_var_15,
        &special_random_large_type_gen_var_15,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_modular_div_divide_and_conquer`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_16() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_16,
        &random_large_type_gen_var_16,
        &special_random_large_type_gen_var_16,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)` that are valid inputs to
// `limbs_modular_invert_small`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_17() -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Generator::new(
        &exhaustive_large_type_gen_var_17,
        &random_large_type_gen_var_17,
        &special_random_large_type_gen_var_17,
    )
}

// All `(Vec<Limb>, usize, Limb, Limb, u64)` that are valid inputs to
// `limbs_div_mod_extra_in_place`.
pub fn large_type_gen_var_18() -> Generator<(Vec<Limb>, usize, Limb, Limb, u64)> {
    Generator::new(
        &exhaustive_large_type_gen_var_18,
        &random_large_type_gen_var_18,
        &special_random_large_type_gen_var_18,
    )
}

// All `(Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)` that are valid inputs to
// `limbs_div_mod_extra`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_19() -> Generator<(Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)> {
    Generator::new(
        &exhaustive_large_type_gen_var_19,
        &random_large_type_gen_var_19,
        &special_random_large_type_gen_var_19,
    )
}

// All `(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)` that are valid inputs to
// `limbs_div_barrett_large_product`.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_20(
) -> Generator<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    Generator::new(
        &exhaustive_large_type_gen_var_20,
        &random_large_type_gen_var_20,
        &special_random_large_type_gen_var_20,
    )
}

#[allow(clippy::type_complexity)]
/// All nonuples of `Limb`s that are valid inputs to `limbs_mod_mul_two_limbs`.
pub fn large_type_gen_var_21() -> Generator<(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)>
{
    Generator::new(
        &exhaustive_large_type_gen_var_21,
        &random_large_type_gen_var_21,
        &special_random_large_type_gen_var_21,
    )
}

// var 22 is in malachite-base.

/// All `(u64, Vec<Natural>, RationalSequence<Natural>)` that are valid inputs to
/// `Rational::from_power_of_2_digits`.
pub fn large_type_gen_var_23() -> Generator<(u64, Vec<Natural>, RationalSequence<Natural>)> {
    Generator::new(
        &exhaustive_large_type_gen_var_23,
        &random_large_type_gen_var_23,
        &special_random_large_type_gen_var_23,
    )
}

/// All `(Vec<Natural>, RationalSequence<Natural>)` that are valid inputs to
/// `Rational::from_power_of_2_digits` with `log_base` == 1.
pub fn large_type_gen_var_24() -> Generator<(Vec<Natural>, RationalSequence<Natural>)> {
    Generator::new_no_special(
        &exhaustive_large_type_gen_var_24,
        &random_large_type_gen_var_24,
    )
}

/// All `(u64, Vec<Natural>, RationalSequence<Natural>)` that are valid inputs to
/// `Rational::from_digits`.
pub fn large_type_gen_var_25() -> Generator<(Natural, Vec<Natural>, RationalSequence<Natural>)> {
    Generator::new(
        &exhaustive_large_type_gen_var_25,
        &random_large_type_gen_var_25,
        &special_random_large_type_gen_var_24,
    )
}

/// All `(Vec<Natural>, RationalSequence<Natural>)` that are valid inputs to `Rational::from_digits`
/// with `base` == 10.
pub fn large_type_gen_var_26() -> Generator<(Vec<Natural>, RationalSequence<Natural>)> {
    Generator::new_no_special(
        &exhaustive_large_type_gen_var_26,
        &random_large_type_gen_var_26,
    )
}

// var 27 is in malachite-base.

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
