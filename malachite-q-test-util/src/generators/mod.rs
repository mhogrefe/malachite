use crate::generators::common::{
    rational_integer_pair_rm, rational_natural_pair_rm, rational_nrm, rational_pair_1_rm,
    rational_pair_nrm, rational_pair_rm,
};
use crate::generators::exhaustive::*;
use crate::generators::random::*;
use crate::generators::special_random::*;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::common::Generator;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use num::BigRational;

// -- Rational --

pub fn rational_gen() -> Generator<Rational> {
    Generator::new(
        &exhaustive_rational_gen,
        &random_rational_gen,
        &special_random_rational_gen,
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

// -- (Rational, Natural, Natural) --

pub fn rational_natural_natural_triple_gen() -> Generator<(Rational, Natural, Natural)> {
    Generator::new(
        &exhaustive_rational_natural_natural_triple_gen,
        &random_rational_natural_natural_triple_gen,
        &special_random_rational_natural_natural_triple_gen,
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

// All `(Rational, T)` where T is small and signed.
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

// -- (Rational, PrimitiveSigned, PrimitiveSigned) --

pub fn rational_signed_signed_triple_gen<T: PrimitiveSigned>() -> Generator<(Rational, T, T)> {
    Generator::new(
        &exhaustive_rational_signed_signed_triple_gen,
        &random_rational_primitive_int_primitive_int_triple_gen,
        &special_random_rational_signed_signed_triple_gen,
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

// All `(Rational, T)` where T is small and unsigned.
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

// -- (Rational, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn rational_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(Rational, T, T)>
{
    Generator::new(
        &exhaustive_rational_unsigned_unsigned_triple_gen,
        &random_rational_primitive_int_primitive_int_triple_gen,
        &special_random_rational_unsigned_unsigned_triple_gen,
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

// -- (Rational, Rational, PrimitiveUnsigned) --

pub fn rational_rational_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> Generator<(Rational, Rational, T)> {
    Generator::new(
        &exhaustive_rational_rational_unsigned_triple_gen,
        &random_rational_rational_primitive_int_triple_gen,
        &special_random_rational_rational_unsigned_triple_gen,
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

// -- (Rational, RoundingMode) --

// All `(Rational, RoundingMode)` pairs that are valid inputs to
// `Natural::rounding_from(Rational)`.
pub fn rational_rounding_mode_pair_gen_var_1() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_1,
        &random_rational_rounding_mode_pair_gen_var_1,
        &special_random_rational_rounding_mode_pair_gen_var_1,
    )
}

// All `(Rational, RoundingMode)` pairs that are valid inputs to
// `Integer::rounding_from(Rational)`.
pub fn rational_rounding_mode_pair_gen_var_2() -> Generator<(Rational, RoundingMode)> {
    Generator::new(
        &exhaustive_rational_rounding_mode_pair_gen_var_2,
        &random_rational_rounding_mode_pair_gen_var_2,
        &special_random_rational_rounding_mode_pair_gen_var_2,
    )
}

// All `(Rational, RoundingMode)` pairs that are valid inputs to `T::rounding_from(Rational)`.
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

// -- String --

// vars 1 through 10 are in malachite-base.

// All `String`s that are produced by serializing a `Rational` into json.
pub fn string_gen_var_11() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_11,
        &random_string_gen_var_11,
        &special_random_string_gen_var_11,
    )
}

// All `String`s that are produced by converting a `Rational` to a string.
pub fn string_gen_var_12() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_12,
        &random_string_gen_var_12,
        &special_random_string_gen_var_12,
    )
}

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

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
