use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::exhaustive::{exhaustive_pairs_custom_output, ExhaustivePairs};

use generators::common::Generator;
use generators::exhaustive::*;
use generators::random::*;
use generators::special_random::*;

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

/// All signed `T`s which are not `T::MIN`.
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

/// All signed `T`s that are neither 0 nor -1.
pub fn signed_gen_var_3<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_3,
        &random_signed_gen_var_3,
        &special_random_signed_gen_var_3,
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

// All `(x, y, z): (T, T, T)` where `T` is signed and x + y * z does not overflow.
pub fn signed_triple_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_1,
        &random_signed_triple_gen_var_1,
        &special_random_signed_triple_gen_var_1,
    )
}

// All `(x, y, z): (T, T, T)` where `T` is signed and x - y * z does not overflow.
pub fn signed_triple_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_2,
        &random_signed_triple_gen_var_2,
        &special_random_signed_triple_gen_var_2,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

/// All `(T, U)`s where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_unsigned_pair_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_1,
        &special_random_signed_unsigned_pair_gen_var_1,
    )
}

/// All `(T, u64)`s where `T` is signed and the `U` is smaller than `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_4,
        &random_primitive_int_unsigned_pair_gen_var_2,
        &special_random_signed_unsigned_pair_gen_var_2,
    )
}

/// All `(T, u64)`s where `T` is signed and the either the `T` is negative or the `u64` is less than
/// `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_3,
        &random_signed_unsigned_pair_gen_var_1,
        &special_random_signed_unsigned_pair_gen_var_3,
    )
}

/// All `(T, u64)`s where `T` is signed and the either the `T` is non-negative or the `u64` is less
/// than `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_5,
        &random_signed_unsigned_pair_gen_var_2,
        &special_random_signed_unsigned_pair_gen_var_4,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, bool) --

// All (`T`, `u64`, and `bool`) where `T` is signed and either the `u64` is smaller than `T::WIDTH`
// or the `bool` is equal to whether the `T` is negative.
pub fn signed_unsigned_bool_triple_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, u64, bool)> {
    Generator::new(
        &exhaustive_signed_unsigned_bool_triple_gen_var_1,
        &random_primitive_int_unsigned_bool_triple_gen_var_2,
        &random_signed_unsigned_bool_triple_gen_var_1,
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

/// All unsigned positive `T`s.
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

/// All `(T, U)`s where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_1,
        &special_random_unsigned_pair_gen_var_1,
    )
}

/// All `(T, u64)`s where `T` is unsigned and the `U` is smaller than `T::WIDTH`.
pub fn unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_3,
        &random_primitive_int_unsigned_pair_gen_var_2,
        &special_random_unsigned_pair_gen_var_2,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, bool) --

// All (`T`, `u64`, `bool`) where `T` is unsigned and either the `bool` is false or the `u64` is
// smaller than `T::WIDTH`.
pub fn unsigned_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, u64, bool)>
{
    Generator::new(
        &exhaustive_unsigned_unsigned_bool_triple_gen_var_1,
        &random_primitive_int_unsigned_bool_triple_gen_var_1,
        &special_random_unsigned_unsigned_bool_triple_gen_var_1,
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

// All `(x, y, z): (T, T, T)` where `T` is unsigned and x + y * z does not overflow.
pub fn unsigned_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_1,
        &random_unsigned_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_1,
    )
}

// All `(x, y, z): (T, T, T)` where `T` is unsigned and x - y * z does not overflow.
pub fn unsigned_triple_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_2,
        &random_unsigned_triple_gen_var_2,
        &special_random_unsigned_triple_gen_var_2,
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

// -- String --

pub fn string_gen() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen,
        &random_string_gen,
        &special_random_string_gen,
    )
}

// All ASCII `String`s.
pub fn string_gen_var_1() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_1,
        &random_string_gen_var_1,
        &special_random_string_gen_var_1,
    )
}

// All `String`s containing only characters that appear in the `String` representations of
// `RoundingMode`s.
pub fn string_gen_var_2() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_2, &random_string_gen_var_2)
}

// -- (String, String) --

pub fn string_pair_gen() -> Generator<(String, String)> {
    Generator::new(
        &exhaustive_string_pair_gen,
        &random_string_pair_gen,
        &special_random_string_pair_gen,
    )
}

// All pairs of ASCII `String`s.
pub fn string_pair_gen_var_1() -> Generator<(String, String)> {
    Generator::new(
        &exhaustive_string_pair_gen_var_1,
        &random_string_pair_gen_var_1,
        &special_random_string_pair_gen_var_1,
    )
}

// -- Vec<PrimitiveUnsigned> --

pub fn unsigned_vec_gen<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen,
        &random_primitive_int_vec_gen,
        &special_random_unsigned_vec_gen,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
