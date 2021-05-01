use crate::generators::common::{
    integer_nrm, integer_pair_1_nrm, natural_nrm, natural_pair_1_nrm, natural_pair_nrm,
    natural_pair_rm,
};
use crate::generators::exhaustive::*;
use crate::generators::random::*;
use crate::generators::special_random::*;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base_test_util::generators::common::Generator;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::{BigInt, BigUint};

// -- Integer --

pub fn integer_gen() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen,
        &random_integer_gen,
        &special_random_integer_gen,
    )
}

pub fn integer_gen_nrm() -> Generator<(BigInt, rug::Integer, Integer)> {
    Generator::new(
        &|| integer_nrm(exhaustive_integer_gen()),
        &|config| integer_nrm(random_integer_gen(config)),
        &|config| integer_nrm(special_random_integer_gen(config)),
    )
}

// -- (Integer, PrimitiveUnsigned) --

// All `(Integer, T)` where `T` is unsigned and between 2 and 36, inclusive.
pub fn integer_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Integer, T)> {
    Generator::new(
        &exhaustive_integer_unsigned_pair_gen_var_1,
        &random_integer_unsigned_pair_gen_var_1,
        &special_random_integer_unsigned_pair_gen_var_3,
    )
}

#[allow(clippy::type_complexity)]
pub fn integer_unsigned_pair_gen_var_1_nrm<T: PrimitiveUnsigned>(
) -> Generator<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Generator::new(
        &|| integer_pair_1_nrm(exhaustive_integer_unsigned_pair_gen_var_1()),
        &|config| integer_pair_1_nrm(random_integer_unsigned_pair_gen_var_1(config)),
        &|config| integer_pair_1_nrm(special_random_integer_unsigned_pair_gen_var_3(config)),
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

// -- Natural --

pub fn natural_gen() -> Generator<Natural> {
    Generator::new(
        &exhaustive_natural_gen,
        &random_natural_gen,
        &special_random_natural_gen,
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

// -- (Natural, Natural, Natural) --

pub fn natural_triple_gen() -> Generator<(Natural, Natural, Natural)> {
    Generator::new(
        &exhaustive_natural_triple_gen,
        &random_natural_triple_gen,
        &special_random_natural_triple_gen,
    )
}

// -- (Natural, PrimitiveUnsigned) --

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

// All `(Natural, T)` where `T` is unsigned, positive, and small.
pub fn natural_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(Natural, T)> {
    Generator::new(
        &exhaustive_natural_primitive_int_pair_gen_var_3,
        &random_natural_unsigned_pair_gen_var_7,
        &special_random_natural_unsigned_pair_gen_var_6,
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

// All `(Vec<Natural>, T)`, where the `T` is positive.
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
        &random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_4::<T, U>,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

// All `(Vec<T>, u64, Vec<Limb>)` that are valid inputs to `_limbs_to_digits_small_base`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, u64, Vec<Limb>)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &random_primitive_int_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
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

// vars 24 through 27 are in malachite-base

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
