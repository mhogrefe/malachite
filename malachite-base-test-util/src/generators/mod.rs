use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::exhaustive::{exhaustive_pairs_custom_output, ExhaustivePairs};

use generators::common::Generator;
use generators::exhaustive::{
    exhaustive_bool_gen, exhaustive_char_gen, exhaustive_char_gen_var_1, exhaustive_char_gen_var_2,
    exhaustive_char_pair_gen, exhaustive_primitive_int_gen_var_1, exhaustive_rounding_mode_gen,
    exhaustive_rounding_mode_pair_gen, exhaustive_rounding_mode_triple_gen, exhaustive_signed_gen,
    exhaustive_signed_gen_var_1, exhaustive_signed_gen_var_2, exhaustive_signed_pair_gen,
    exhaustive_signed_triple_gen, exhaustive_unsigned_gen, exhaustive_unsigned_gen_var_1,
    exhaustive_unsigned_pair_gen, exhaustive_unsigned_pair_gen_var_1,
    exhaustive_unsigned_triple_gen,
};
use generators::random::{
    random_bool_gen, random_char_gen, random_char_gen_var_1, random_char_gen_var_2,
    random_char_pair_gen, random_primitive_int_gen, random_primitive_int_pair_gen,
    random_primitive_int_triple_gen, random_rounding_mode_gen, random_rounding_mode_pair_gen,
    random_rounding_mode_triple_gen, random_signed_gen_var_1, random_signed_gen_var_2,
    random_unsigned_gen_var_1, random_unsigned_gen_var_2, random_unsigned_pair_gen_var_1,
};
use generators::special_random::{
    special_random_char_gen, special_random_char_gen_var_1, special_random_char_gen_var_2,
    special_random_char_pair_gen, special_random_signed_gen, special_random_signed_gen_var_1,
    special_random_signed_gen_var_2, special_random_signed_pair_gen,
    special_random_signed_triple_gen, special_random_unsigned_gen,
    special_random_unsigned_gen_var_1, special_random_unsigned_pair_gen,
    special_random_unsigned_triple_gen,
};

// general

#[inline]
pub fn exhaustive_pairs_big_tiny<
    X: Clone,
    I: Iterator<Item = X>,
    Y: Clone,
    J: Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
) -> ExhaustivePairs<X, I, Y, J> {
    exhaustive_pairs_custom_output(
        xs,
        ys,
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    )
}

#[inline]
pub fn exhaustive_pairs_big_small<
    X: Clone,
    I: Iterator<Item = X>,
    Y: Clone,
    J: Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
) -> ExhaustivePairs<X, I, Y, J> {
    exhaustive_pairs_custom_output(
        xs,
        ys,
        BitDistributorOutputType::normal(2),
        BitDistributorOutputType::normal(1),
    )
}

// -- bool --

pub fn bool_gen() -> Generator<bool> {
    Generator::new_no_special(&exhaustive_bool_gen, &random_bool_gen)
}

// -- char --

pub fn char_gen() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen,
        &random_char_gen,
        &special_random_char_gen,
    )
}

/// All `char`s except for `char::MAX`.
pub fn char_gen_var_1() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_1,
        &random_char_gen_var_1,
        &special_random_char_gen_var_1,
    )
}

/// All `char`s except for `char::MIN`.
pub fn char_gen_var_2() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_2,
        &random_char_gen_var_2,
        &special_random_char_gen_var_2,
    )
}

// -- (char, char) --

pub fn char_pair_gen() -> Generator<(char, char)> {
    Generator::new(
        &exhaustive_char_pair_gen,
        &random_char_pair_gen,
        &special_random_char_pair_gen,
    )
}

// -- PrimitiveSigned --

pub fn signed_gen<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen,
        &random_primitive_int_gen,
        &special_random_signed_gen,
    )
}

/// All `T`s where `T` is signed and the `T` is not `T::MIN`.
pub fn signed_gen_var_1<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_1,
        &random_signed_gen_var_1,
        &special_random_signed_gen_var_1,
    )
}

/// All signed natural (non-negative) `T`s.
pub fn signed_gen_var_2<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_2,
        &random_signed_gen_var_2,
        &special_random_signed_gen_var_2,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn signed_pair_gen<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_signed_pair_gen,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn signed_triple_gen<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen,
        &random_primitive_int_triple_gen,
        &special_random_signed_triple_gen,
    )
}

// -- PrimitiveUnsigned --

pub fn unsigned_gen<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen,
        &random_primitive_int_gen,
        &special_random_unsigned_gen,
    )
}

/// All `T` where `T` is unsigned and the `T` is positive.
pub fn unsigned_gen_var_1<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_1,
        &random_unsigned_gen_var_1,
        &special_random_unsigned_gen_var_1,
    )
}

// All `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn unsigned_gen_var_2() -> Generator<u32> {
    Generator::new_no_special(&exhaustive_unsigned_gen_var_1, &random_unsigned_gen_var_2)
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn unsigned_pair_gen<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_unsigned_pair_gen,
    )
}

// All `(u32, u32)`s where each `u32` is smaller than `NUMBER_OF_CHARS`.
pub fn unsigned_pair_gen_var_1() -> Generator<(u32, u32)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_1,
        &random_unsigned_pair_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen,
        &random_primitive_int_triple_gen,
        &special_random_unsigned_triple_gen,
    )
}

// -- RoundingMode --

pub fn rounding_mode_gen() -> Generator<RoundingMode> {
    Generator::new_no_special(&exhaustive_rounding_mode_gen, &random_rounding_mode_gen)
}

// -- (RoundingMode, RoundingMode) --

pub fn rounding_mode_pair_gen() -> Generator<(RoundingMode, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_rounding_mode_pair_gen,
        &random_rounding_mode_pair_gen,
    )
}

// -- (RoundingMode, RoundingMode, RoundingMode) --

pub fn rounding_mode_triple_gen() -> Generator<(RoundingMode, RoundingMode, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_rounding_mode_triple_gen,
        &random_rounding_mode_triple_gen,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
