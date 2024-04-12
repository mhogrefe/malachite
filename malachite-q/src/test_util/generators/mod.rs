// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::generators::common::{
    rational_integer_pair_rm, rational_natural_pair_rm, rational_nrm, rational_pair_1_nrm,
    rational_pair_1_rm, rational_pair_nm, rational_pair_nrm, rational_pair_rm, rational_rm,
    rational_vec_nrm,
};
use crate::test_util::generators::exhaustive::*;
use crate::test_util::generators::random::*;
use crate::test_util::generators::special_random::*;
use crate::Rational;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::Generator;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::BigRational;
use std::ops::Shr;

// -- Rational --

pub fn rational_gen() -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen,
        &random_rational_gen,
        &special_random_rational_gen,
    )
}

pub fn rational_gen_rm() -> Generator<(rug::Rational, Rational)> {
    Generator::new(
        &|| rational_rm(exhaustive_rational_gen()),
        &|config| rational_rm(random_rational_gen(config)),
        &|config| rational_rm(special_random_rational_gen(config)),
    )
}

pub fn rational_gen_nrm() -> Generator<(BigRational, rug::Rational, Rational)> {
    Generator::new(
        &|| rational_nrm(exhaustive_rational_gen()),
        &|config| rational_nrm(random_rational_gen(config)),
        &|config| rational_nrm(special_random_rational_gen(config)),
    )
}

// All nonzero `Rational`s.
pub fn rational_gen_var_1() -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen_var_1,
        &random_rational_gen_var_1,
        &special_random_rational_gen_var_1,
    )
}

pub fn rational_gen_var_1_nrm() -> Generator<(BigRational, rug::Rational, Rational)> {
    Generator::new(
        &|| rational_nrm(exhaustive_rational_gen_var_1()),
        &|config| rational_nrm(random_rational_gen_var_1(config)),
        &|config| rational_nrm(special_random_rational_gen_var_1(config)),
    )
}

// All positive `Rational`s.
pub fn rational_gen_var_2() -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen_var_2,
        &random_rational_gen_var_2,
        &special_random_rational_gen_var_2,
    )
}

// All non-negative `Rational`s.
pub fn rational_gen_var_3() -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen_var_3,
        &random_rational_gen_var_3,
        &special_random_rational_gen_var_3,
    )
}

// All `Rational`s that are equal to a primitive float of type `T`.
pub fn rational_gen_var_4<T: PrimitiveFloat>() -> Generator<Rational>
where
    Rational: TryFrom<T>,
{
    Generator::new(
        &exhaustive_rational_gen_var_4::<T>,
        &random_rational_gen_var_4::<T>,
        &special_random_rational_gen_var_4::<T>,
    )
}

// All `Rational`s that are not equal to any primitive float of type `T`.
pub fn rational_gen_var_5<T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat>(
) -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen_var_5::<T>,
        &random_rational_gen_var_5::<T>,
        &special_random_rational_gen_var_5::<T>,
    )
}

// All `Rational`s that are halfway between two adjacent floats of type `T`.
pub fn rational_gen_var_6<T: PrimitiveFloat>() -> Generator<Rational>
where
    Rational: TryFrom<T>,
{
    Generator::new(
        &exhaustive_rational_gen_var_6::<T>,
        &random_rational_gen_var_6::<T>,
        &special_random_rational_gen_var_6::<T>,
    )
}

// All `Rational`s with small numerator and denominator.
pub fn rational_gen_var_7() -> Generator<Rational> {
    Generator::new_no_special(&exhaustive_rational_gen, &random_rational_gen_var_7)
}

// All positive `Rational`s that are not equal to 1.
pub fn rational_gen_var_8() -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen_var_7,
        &random_rational_gen_var_8,
        &special_random_rational_gen_var_7,
    )
}

// -- (Rational, Integer) --

pub fn rational_integer_pair_gen() -> Generator<(Rational, Integer)> {
    Generator::new(
        &exhaustive_rational_integer_pair_gen,
        &random_rational_integer_pair_gen,
        &special_random_rational_integer_pair_gen,
    )
}

pub fn rational_integer_pair_gen_rm(
) -> Generator<((rug::Rational, rug::Integer), (Rational, Integer))> {
    Generator::new(
        &|| rational_integer_pair_rm(exhaustive_rational_integer_pair_gen()),
        &|config| rational_integer_pair_rm(random_rational_integer_pair_gen(config)),
        &|config| rational_integer_pair_rm(random_rational_integer_pair_gen(config)),
    )
}

// -- (Rational, Integer, Integer) --

pub fn rational_integer_integer_triple_gen() -> Generator<(Rational, Integer, Integer)> {
    Generator::new(
        &exhaustive_rational_integer_integer_triple_gen,
        &random_rational_integer_integer_triple_gen,
        &special_random_rational_integer_integer_triple_gen,
    )
}

// -- (Rational, Natural) --

pub fn rational_natural_pair_gen() -> Generator<(Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_pair_gen,
        &random_rational_natural_pair_gen,
        &special_random_rational_natural_pair_gen,
    )
}

pub fn rational_natural_pair_gen_rm(
) -> Generator<((rug::Rational, rug::Integer), (Rational, Natural))> {
    Generator::new(
        &|| rational_natural_pair_rm(exhaustive_rational_natural_pair_gen()),
        &|config| rational_natural_pair_rm(random_rational_natural_pair_gen(config)),
        &|config| rational_natural_pair_rm(random_rational_natural_pair_gen(config)),
    )
}

// All `(Rational, Natural)` where the `Natural` is greater than 1.
pub fn rational_natural_pair_gen_var_1() -> Generator<(Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_pair_gen_var_1,
        &random_rational_natural_pair_gen_var_1,
        &special_random_rational_natural_pair_gen_var_1,
    )
}

// All `(Rational, Natural)` where the `Rational` has a small numerator and denominator, and the
// `Natural` is greater than 1.
pub fn rational_natural_pair_gen_var_2() -> Generator<(Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_pair_gen_var_1,
        &random_rational_natural_pair_gen_var_2,
        &special_random_rational_natural_pair_gen_var_2,
    )
}

// All `(Rational, Natural)` where the `Natural` is positive.
pub fn rational_natural_pair_gen_var_3() -> Generator<(Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_pair_gen_var_2,
        &random_rational_natural_pair_gen_var_3,
        &special_random_rational_natural_pair_gen_var_3,
    )
}

// All `(Rational, Natural)` where the `Natural` is small and positive.
pub fn rational_natural_pair_gen_var_4() -> Generator<(Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_pair_gen_var_3,
        &random_rational_natural_pair_gen_var_4,
        &special_random_rational_natural_pair_gen_var_4,
    )
}

// -- (Rational, Natural, Natural) --

pub fn rational_natural_natural_triple_gen() -> Generator<(Rational, Natural, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_natural_triple_gen,
        &random_rational_natural_natural_triple_gen,
        &special_random_rational_natural_natural_triple_gen,
    )
}

// All `(Rational, Natural, Natural)` where the both `Natural`s are positive and the first `Natural`
// is smaller than the second.
pub fn rational_natural_natural_triple_gen_var_1() -> Generator<(Rational, Natural, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_natural_triple_gen_var_1,
        &random_rational_natural_natural_triple_gen_var_1,
        &special_random_rational_natural_natural_triple_gen_var_1,
    )
}

// -- (Rational, PrimitiveFloat) --

pub fn rational_primitive_float_pair_gen<T: PrimitiveFloat>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_primitive_float_pair_gen,
        &random_rational_primitive_float_pair_gen,
        &special_random_rational_primitive_float_pair_gen,
    )
}

pub fn rational_primitive_float_pair_gen_rm<T: PrimitiveFloat>(
) -> Generator<((rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_rm(exhaustive_rational_primitive_float_pair_gen()),
        &|config| rational_pair_1_rm(random_rational_primitive_float_pair_gen(config)),
        &|config| rational_pair_1_rm(special_random_rational_primitive_float_pair_gen(config)),
    )
}

// -- (Rational, PrimitiveFloat, PrimitiveFloat) --

pub fn rational_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Rational, T, T)> {
    Generator::new(
        &exhaustive_rational_primitive_float_primitive_float_triple_gen,
        &random_rational_primitive_float_primitive_float_triple_gen,
        &special_random_rational_primitive_float_primitive_float_triple_gen,
    )
}

// -- (Rational, PrimitiveSigned) --

pub fn rational_signed_pair_gen<T: PrimitiveSigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_signed_pair_gen,
        &random_rational_primitive_int_pair_gen,
        &special_random_rational_signed_pair_gen,
    )
}

pub fn rational_signed_pair_gen_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_rm(exhaustive_rational_signed_pair_gen()),
        &|config| rational_pair_1_rm(random_rational_primitive_int_pair_gen(config)),
        &|config| rational_pair_1_rm(special_random_rational_signed_pair_gen(config)),
    )
}

// All `(Rational, T)` where `T` is small and signed.
pub fn rational_signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_signed_pair_gen_var_1,
        &random_rational_signed_pair_gen_var_1,
        &special_random_rational_signed_pair_gen_var_1,
    )
}

pub fn rational_signed_pair_gen_var_1_rm<T: PrimitiveSigned>(
) -> Generator<((rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_rm(exhaustive_rational_signed_pair_gen_var_1()),
        &|config| rational_pair_1_rm(random_rational_signed_pair_gen_var_1(config)),
        &|config| rational_pair_1_rm(special_random_rational_signed_pair_gen_var_1(config)),
    )
}

// All `(Rational, T)` where `T` is small and signed, and if the `Rational` is zero then the `T` is
// non-negative.
pub fn rational_signed_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_signed_pair_gen_var_2,
        &random_rational_signed_pair_gen_var_2,
        &special_random_rational_signed_pair_gen_var_2,
    )
}

pub fn rational_signed_pair_gen_var_2_nrm<T: PrimitiveSigned>(
) -> Generator<((BigRational, T), (rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_nrm(exhaustive_rational_signed_pair_gen_var_2()),
        &|config| rational_pair_1_nrm(random_rational_signed_pair_gen_var_2(config)),
        &|config| rational_pair_1_nrm(special_random_rational_signed_pair_gen_var_2(config)),
    )
}

// All `(Rational, T)` where `T` is small and signed, and the `Rational` divided by 2 to the power
// of the `T` is not an integer.
pub fn rational_signed_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(Rational, T)>
where
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    Generator::new(
        &exhaustive_rational_signed_pair_gen_var_3,
        &random_rational_signed_pair_gen_var_3,
        &special_random_rational_signed_pair_gen_var_3,
    )
}

// All `(Rational, T)` where `T` is signed, small, and nonzero, and either the `Rational` is
// non-negative or the `T` is odd, and either the `Rational` is nonzero or the `T` is positive.
pub fn rational_signed_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_signed_pair_gen_var_4,
        &random_rational_signed_pair_gen_var_4,
        &special_random_rational_signed_pair_gen_var_4,
    )
}

// All `(Rational, T)` where the `Rational` is positive and the `T` is small, signed, and nonzero.
pub fn rational_signed_pair_gen_var_5<T: PrimitiveSigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_signed_pair_gen_var_5,
        &random_rational_signed_pair_gen_var_5,
        &special_random_rational_signed_pair_gen_var_5,
    )
}

// -- (Rational, PrimitiveSigned, RoundingMode) --

// All `(Rational, i64, RoundingMode)` where the triple is a valid input to
// `Rational::round_to_multiple_of_power_of_2`.
pub fn rational_signed_rounding_mode_triple_gen_var_1() -> Generator<(Rational, i64, RoundingMode)>
{
    Generator::new(
        &exhaustive_rational_signed_rounding_mode_triple_gen_var_1,
        &random_rational_signed_rounding_mode_triple_gen_var_1,
        &special_random_rational_signed_rounding_mode_triple_gen_var_1,
    )
}

// var 2 is in malachite-float.

// -- (Rational, PrimitiveSigned, PrimitiveSigned) --

pub fn rational_signed_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Rational, T, T)> {
    Generator::new(
        &exhaustive_rational_signed_signed_triple_gen,
        &random_rational_primitive_int_primitive_int_triple_gen,
        &special_random_rational_signed_signed_triple_gen,
    )
}

// All `(Rational, T, T)` where `T` is signed, both `T`s are small, and if either `T` is negative,
// the `Rational` is nonzero.
pub fn rational_signed_signed_triple_gen_var_1<T: PrimitiveSigned>() -> Generator<(Rational, T, T)>
{
    Generator::new(
        &exhaustive_rational_signed_signed_triple_gen_var_1,
        &random_rational_signed_signed_triple_gen_var_1,
        &special_random_rational_signed_signed_triple_gen_var_1,
    )
}

// -- (Rational, PrimitiveSigned, PrimitiveUnsigned) --

// All `(Rational, T, U)` where `T` is small and signed, and positive and `U` is small, unsigned,
// and positive.
pub fn rational_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(Rational, T, U)> {
    Generator::new(
        &exhaustive_rational_signed_unsigned_triple_gen_var_1,
        &random_rational_signed_unsigned_triple_gen_var_1,
        &special_random_rational_signed_unsigned_triple_gen_var_1,
    )
}

// -- (Rational, PrimitiveUnsigned) --

pub fn rational_unsigned_pair_gen<T: PrimitiveUnsigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_unsigned_pair_gen,
        &random_rational_primitive_int_pair_gen,
        &special_random_rational_unsigned_pair_gen,
    )
}

pub fn rational_unsigned_pair_gen_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_rm(exhaustive_rational_unsigned_pair_gen()),
        &|config| rational_pair_1_rm(random_rational_primitive_int_pair_gen(config)),
        &|config| rational_pair_1_rm(special_random_rational_unsigned_pair_gen(config)),
    )
}

// All `(Rational, T)` where `T` is small and unsigned.
pub fn rational_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_unsigned_pair_gen_var_1,
        &random_rational_unsigned_pair_gen_var_1,
        &special_random_rational_unsigned_pair_gen_var_1,
    )
}

pub fn rational_unsigned_pair_gen_var_1_rm<T: PrimitiveUnsigned>(
) -> Generator<((rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_rm(exhaustive_rational_unsigned_pair_gen_var_1()),
        &|config| rational_pair_1_rm(random_rational_unsigned_pair_gen_var_1(config)),
        &|config| rational_pair_1_rm(special_random_rational_unsigned_pair_gen_var_1(config)),
    )
}

pub fn rational_unsigned_pair_gen_var_1_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigRational, T), (rug::Rational, T), (Rational, T))> {
    Generator::new(
        &|| rational_pair_1_nrm(exhaustive_rational_unsigned_pair_gen_var_1()),
        &|config| rational_pair_1_nrm(random_rational_unsigned_pair_gen_var_1(config)),
        &|config| rational_pair_1_nrm(special_random_rational_unsigned_pair_gen_var_1(config)),
    )
}

// All `(Rational, T)` where the `T` is small, unsigned, and positive, and the `Rational` has a
// small numerator and denominator.
pub fn rational_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Rational, T)> {
    Generator::new_no_special(
        &exhaustive_rational_unsigned_pair_gen_var_2,
        &random_rational_unsigned_pair_gen_var_2,
    )
}

// All `(Rational, T)` where the `T` is small, unsigned, and positive.
pub fn rational_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_unsigned_pair_gen_var_2,
        &random_rational_unsigned_pair_gen_var_3,
        &special_random_rational_unsigned_pair_gen_var_2,
    )
}

// All `(Rational, T)` where `T` is unsigned, small, and positive, and either the `Rational` is
// non-negative or the `T` is odd.
pub fn rational_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Rational, T)> {
    Generator::new(
        &exhaustive_rational_unsigned_pair_gen_var_3,
        &random_rational_unsigned_pair_gen_var_4,
        &special_random_rational_unsigned_pair_gen_var_3,
    )
}

// All `(Rational, u8)`s where the `u8` is between 2 and 36, inclusive.
pub fn rational_unsigned_pair_gen_var_5() -> Generator<(Rational, u8)> {
    Generator::new(
        &exhaustive_rational_unsigned_pair_gen_var_4,
        &random_rational_unsigned_pair_gen_var_5,
        &special_random_rational_unsigned_pair_gen_var_4,
    )
}

// All `(Rational, u8)`s where the numerator and denominator of the `Rational` is small and the `u8`
// is between 2 and 36, inclusive.
pub fn rational_unsigned_pair_gen_var_6() -> Generator<(Rational, u8)> {
    Generator::new_no_special(
        &exhaustive_rational_unsigned_pair_gen_var_4,
        &random_rational_unsigned_pair_gen_var_6,
    )
}

// -- (Rational, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn rational_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Rational, T, T)>
{
    Generator::new(
        &exhaustive_rational_unsigned_unsigned_triple_gen,
        &random_rational_primitive_int_primitive_int_triple_gen,
        &special_random_rational_unsigned_unsigned_triple_gen,
    )
}

// All `(Rational, T, T)` where `T` is unsigned and both `T`s are small.
pub fn rational_unsigned_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Rational, T, T)> {
    Generator::new(
        &exhaustive_rational_unsigned_unsigned_triple_gen_var_1,
        &random_rational_unsigned_unsigned_triple_gen_var_1,
        &special_random_rational_unsigned_unsigned_triple_gen_var_1,
    )
}

// -- (Rational, Rational) --

pub fn rational_pair_gen() -> Generator<(Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_pair_gen,
        &random_rational_pair_gen,
        &special_random_rational_pair_gen,
    )
}

pub fn rational_pair_gen_rm() -> Generator<((rug::Rational, rug::Rational), (Rational, Rational))> {
    Generator::new(
        &|| rational_pair_rm(exhaustive_rational_pair_gen()),
        &|config| rational_pair_rm(random_rational_pair_gen(config)),
        &|config| rational_pair_rm(special_random_rational_pair_gen(config)),
    )
}

pub fn rational_pair_gen_nm() -> Generator<((BigRational, BigRational), (Rational, Rational))> {
    Generator::new(
        &|| rational_pair_nm(exhaustive_rational_pair_gen()),
        &|config| rational_pair_nm(random_rational_pair_gen(config)),
        &|config| rational_pair_nm(special_random_rational_pair_gen(config)),
    )
}

pub fn rational_pair_gen_nrm() -> Generator<(
    (BigRational, BigRational),
    (rug::Rational, rug::Rational),
    (Rational, Rational),
)> {
    Generator::new(
        &|| rational_pair_nrm(exhaustive_rational_pair_gen()),
        &|config| rational_pair_nrm(random_rational_pair_gen(config)),
        &|config| rational_pair_nrm(special_random_rational_pair_gen(config)),
    )
}

// All pairs of `Rational`s where the second `Rational` is nonzero.
pub fn rational_pair_gen_var_1() -> Generator<(Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_pair_gen_var_1,
        &random_rational_pair_gen_var_1,
        &special_random_rational_pair_gen_var_1,
    )
}

pub fn rational_pair_gen_var_1_rm(
) -> Generator<((rug::Rational, rug::Rational), (Rational, Rational))> {
    Generator::new(
        &|| rational_pair_rm(exhaustive_rational_pair_gen_var_1()),
        &|config| rational_pair_rm(random_rational_pair_gen_var_1(config)),
        &|config| rational_pair_rm(special_random_rational_pair_gen_var_1(config)),
    )
}

pub fn rational_pair_gen_var_1_nrm() -> Generator<(
    (BigRational, BigRational),
    (rug::Rational, rug::Rational),
    (Rational, Rational),
)> {
    Generator::new(
        &|| rational_pair_nrm(exhaustive_rational_pair_gen_var_1()),
        &|config| rational_pair_nrm(random_rational_pair_gen_var_1(config)),
        &|config| rational_pair_nrm(special_random_rational_pair_gen_var_1(config)),
    )
}

// All pairs of `Rational`s where the second is nonzero and the first is not an integer multiple of
// the second.
pub fn rational_pair_gen_var_2() -> Generator<(Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_pair_gen_var_2,
        &random_rational_pair_gen_var_2,
        &special_random_rational_pair_gen_var_2,
    )
}

// All pairs of `Rational`s where the first is less than the second.
pub fn rational_pair_gen_var_3() -> Generator<(Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_pair_gen_var_3,
        &random_rational_pair_gen_var_3,
        &special_random_rational_pair_gen_var_3,
    )
}

// All pairs of `Rational`s where the first is less than or equal to the second.
pub fn rational_pair_gen_var_4() -> Generator<(Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_pair_gen_var_4,
        &random_rational_pair_gen_var_4,
        &special_random_rational_pair_gen_var_4,
    )
}

// All pairs of `Rational`s where the first is less than the second, and the numerators and
// denominators are small.
pub fn rational_pair_gen_var_5() -> Generator<(Rational, Rational)> {
    Generator::new_no_special(
        &exhaustive_rational_pair_gen_var_3,
        &random_rational_pair_gen_var_5,
    )
}

// All pairs of `Rational`s where the first is less than or equal to the second, and the numerators
// and denominators are small.
pub fn rational_pair_gen_var_6() -> Generator<(Rational, Rational)> {
    Generator::new_no_special(
        &exhaustive_rational_pair_gen_var_4,
        &random_rational_pair_gen_var_6,
    )
}

// All pairs of positive `Rational`s where the second `Rational` is not within 1/1000 of 1.
pub fn rational_pair_gen_var_7() -> Generator<(Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_pair_gen_var_5,
        &random_rational_pair_gen_var_7,
        &special_random_rational_pair_gen_var_5,
    )
}

// -- (Rational, Rational, Integer) --

pub fn rational_rational_integer_triple_gen() -> Generator<(Rational, Rational, Integer)> {
    Generator::new(
        &exhaustive_rational_rational_integer_triple_gen,
        &random_rational_rational_integer_triple_gen,
        &special_random_rational_rational_integer_triple_gen,
    )
}

// -- (Rational, Rational, Natural) --

pub fn rational_rational_natural_triple_gen() -> Generator<(Rational, Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_rational_natural_triple_gen,
        &random_rational_rational_natural_triple_gen,
        &special_random_rational_rational_natural_triple_gen,
    )
}

// All `(Rational, Rational, Natural)` where the `Natural` is positive.
pub fn rational_rational_natural_triple_gen_var_1() -> Generator<(Rational, Rational, Natural)> {
    Generator::new(
        &exhaustive_rational_rational_natural_triple_gen_var_1,
        &random_rational_rational_natural_triple_gen_var_1,
        &special_random_rational_rational_natural_triple_gen_var_1,
    )
}

// -- (Rational, Rational, Natural, Natural) --

// All `(Rational, Rational, Natural, Natural)` where the last `Natural` is positive.
pub fn rational_rational_natural_natural_quadruple_gen_var_1(
) -> Generator<(Rational, Rational, Natural, Natural)> {
    Generator::new(
        &exhaustive_rational_rational_natural_natural_quadruple_gen_var_1,
        &random_rational_rational_natural_natural_quadruple_gen_var_1,
        &special_random_rational_rational_natural_natural_quadruple_gen_var_1,
    )
}

// -- (Rational, Rational, PrimitiveFloat) --

pub fn rational_rational_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> Generator<(Rational, Rational, T)> {
    Generator::new(
        &exhaustive_rational_rational_primitive_float_triple_gen,
        &random_rational_rational_primitive_float_triple_gen,
        &special_random_rational_rational_primitive_float_triple_gen,
    )
}

// -- (Rational, Rational, PrimitiveUnsigned) --

pub fn rational_rational_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Rational, Rational, T)> {
    Generator::new(
        &exhaustive_rational_rational_unsigned_triple_gen,
        &random_rational_rational_primitive_int_triple_gen,
        &special_random_rational_rational_unsigned_triple_gen,
    )
}

// All `(Rational, Rational, T)` where `T` is unsigned and small.
pub fn rational_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Rational, Rational, T)> {
    Generator::new(
        &exhaustive_rational_rational_unsigned_triple_gen_var_1,
        &random_rational_rational_unsigned_triple_gen_var_1,
        &special_random_rational_rational_unsigned_triple_gen_var_1,
    )
}

// -- (Rational, Rational, PrimitiveSigned) --

pub fn rational_rational_signed_triple_gen<T: PrimitiveSigned>(
) -> Generator<(Rational, Rational, T)> {
    Generator::new(
        &exhaustive_rational_rational_signed_triple_gen,
        &random_rational_rational_primitive_int_triple_gen,
        &special_random_rational_rational_signed_triple_gen,
    )
}

// All `(Rational, Rational, T)` where `T` is signed and small, and if `T` is negative, neither
// `Rational` is zero.
pub fn rational_rational_signed_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(Rational, Rational, T)> {
    Generator::new(
        &exhaustive_rational_rational_signed_triple_gen_var_1,
        &random_rational_rational_signed_triple_gen_var_1,
        &special_random_rational_rational_signed_triple_gen_var_1,
    )
}

// -- (Rational, Rational, Rational) --

pub fn rational_triple_gen() -> Generator<(Rational, Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_triple_gen,
        &random_rational_triple_gen,
        &special_random_rational_triple_gen,
    )
}

// All triples of `Rational` where the last `Rational` is nonzero.
pub fn rational_triple_gen_var_1() -> Generator<(Rational, Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_triple_gen_var_1,
        &random_rational_triple_gen_var_1,
        &special_random_rational_triple_gen_var_1,
    )
}

// All triples of `Rational`s `(x, y, z)` where x < y < z.
pub fn rational_triple_gen_var_2() -> Generator<(Rational, Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_triple_gen_var_2,
        &random_rational_triple_gen_var_2,
        &special_random_rational_triple_gen_var_2,
    )
}

// All triples of `Rational`s `(x, y, z)` where x <= y <= z.
pub fn rational_triple_gen_var_3() -> Generator<(Rational, Rational, Rational)> {
    Generator::new(
        &exhaustive_rational_triple_gen_var_3,
        &random_rational_triple_gen_var_3,
        &special_random_rational_triple_gen_var_3,
    )
}

// -- (Rational, Rational, RoundingMode) --

// All `(Rational, Rational, RoundingMode)` triples that are a valid input to
// `Rational::round_to_multiple`.
pub fn rational_rational_rounding_mode_triple_gen_var_1(
) -> Generator<(Rational, Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rational_rounding_mode_triple_gen_var_1,
        &random_rational_rational_rounding_mode_triple_gen_var_1,
        &special_random_rational_rational_rounding_mode_triple_gen_var_1,
    )
}

// -- (Rational, RoundingMode) --

pub fn rational_rounding_mode_pair_gen() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen,
        &random_rational_rounding_mode_pair_gen,
        &special_random_rational_rounding_mode_pair_gen,
    )
}

// All `(Rational, RoundingMode)` pairs that are valid inputs to `Natural::rounding_from(Rational)`.
pub fn rational_rounding_mode_pair_gen_var_1() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_1,
        &random_rational_rounding_mode_pair_gen_var_1,
        &special_random_rational_rounding_mode_pair_gen_var_1,
    )
}

// All `(Rational, RoundingMode)` pairs that are valid inputs to `Integer::rounding_from(Rational)`.
pub fn rational_rounding_mode_pair_gen_var_2() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_2,
        &random_rational_rounding_mode_pair_gen_var_2,
        &special_random_rational_rounding_mode_pair_gen_var_2,
    )
}

// All `(Rational, RoundingMode)` pairs that are valid inputs to `T::rounding_from(Rational)` for an
// primitive integer type `T`.
pub fn rational_rounding_mode_pair_gen_var_3<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>() -> Generator<(Rational, RoundingMode)>
where
    Rational: PartialOrd<T>,
{
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_3::<T>,
        &random_rational_rounding_mode_pair_gen_var_3::<T>,
        &special_random_rational_rounding_mode_pair_gen_var_3::<T>,
    )
}

// All `(Rational, RoundingMode)` pairs where the `Rational` is nonzero.
pub fn rational_rounding_mode_pair_gen_var_4() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_4,
        &random_rational_rounding_mode_pair_gen_var_4,
        &special_random_rational_rounding_mode_pair_gen_var_4,
    )
}

// All `(Rational, RoundingMode)` pairs that are valid inputs to `T::rounding_from(Rational)` for an
// primitive float type `T`.
pub fn rational_rounding_mode_pair_gen_var_5<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat,
>() -> Generator<(Rational, RoundingMode)>
where
    Rational: TryFrom<T>,
{
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_5::<T>,
        &random_rational_rounding_mode_pair_gen_var_5::<T>,
        &special_random_rational_rounding_mode_pair_gen_var_5::<T>,
    )
}

// var 6 is in malachite-float.

// -- (Rational, ToSciOptions) --

pub fn rational_to_sci_options_pair_gen() -> Generator<(Rational, ToSciOptions)> {
    Generator::new(
        &exhaustive_rational_to_sci_options_pair_gen,
        &random_rational_to_sci_options_pair_gen,
        &special_random_rational_to_sci_options_pair_gen,
    )
}

// All `(Rational, ToSciOptions)` pairs where the `Rational` can be formatted using the options.
pub fn rational_to_sci_options_pair_gen_var_1() -> Generator<(Rational, ToSciOptions)> {
    Generator::new(
        &exhaustive_rational_to_sci_options_pair_gen_var_1,
        &random_rational_to_sci_options_pair_gen_var_1,
        &special_random_rational_to_sci_options_pair_gen_var_1,
    )
}

// -- String --

// vars 1 through 10 are in malachite-base.

// All `String`s that are produced by serializing a `Rational` into json.
pub fn string_gen_var_11() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_11,
        &random_string_gen_var_11,
        &special_random_string_gen_var_2,
    )
}

// All `String`s that are produced by converting a `Rational` to a string.
pub fn string_gen_var_12() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_12,
        &random_string_gen_var_12,
        &special_random_string_gen_var_3,
    )
}

// var 13 is in malachite-base.

// -- (String, String, String) --

// vars 1 through 2 are in malachite-nz.

// All triples of `String`s corresponding to the serialization of a `num::BigRational`, a
// `rug::Rational`, and a `Rational`, respectively, into a JSON string. The three numbers have the
// same value.
pub fn string_triple_gen_var_3() -> Generator<(String, String, String)> {
    Generator::new(
        &exhaustive_string_triple_gen_var_3,
        &random_string_triple_gen_var_3,
        &special_random_string_triple_gen_var_3,
    )
}

// -- Vec<Rational> --

pub fn rational_vec_gen() -> Generator<Vec<Rational>> {
    Generator::new(
        &exhaustive_rational_vec_gen,
        &random_rational_vec_gen,
        &special_random_rational_vec_gen,
    )
}

pub fn rational_vec_gen_nrm() -> Generator<(Vec<BigRational>, Vec<rug::Rational>, Vec<Rational>)> {
    Generator::new(
        &|| rational_vec_nrm(exhaustive_rational_vec_gen()),
        &|config| rational_vec_nrm(random_rational_vec_gen(config)),
        &|config| rational_vec_nrm(special_random_rational_vec_gen(config)),
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
