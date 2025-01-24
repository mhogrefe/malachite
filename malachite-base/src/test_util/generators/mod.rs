// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::bit_distributor::BitDistributorOutputType;
use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::string::options::{FromSciStringOptions, SciSizeOptions, ToSciOptions};
use crate::num::conversion::traits::{
    ConvertibleFrom, Digits, ExactFrom, HasHalf, JoinHalves, RoundingFrom, SaturatingFrom,
    SplitInHalf, WrappingFrom, WrappingInto,
};
use crate::num::float::NiceFloat;
use crate::num::logic::traits::{BitBlockAccess, LeadingZeros};
use crate::rational_sequences::RationalSequence;
use crate::rounding_modes::RoundingMode;
use crate::slices::slice_trailing_zeros;
use crate::test_util::generators::common::Generator;
use crate::test_util::generators::exhaustive::*;
use crate::test_util::generators::random::*;
use crate::test_util::generators::special_random::*;
use crate::tuples::exhaustive::{exhaustive_pairs_custom_output, ExhaustivePairs};

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

fn digits_valid<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(log_base: u64, digits: &[U]) -> bool {
    let digits = &digits[..digits.len() - slice_trailing_zeros(digits)];
    if digits.is_empty() {
        return true;
    }
    let significant_bits = ((u64::wrapping_from(digits.len()) - 1) * log_base)
        + digits.last().unwrap().significant_bits();
    significant_bits <= T::WIDTH
}

fn unsigned_assign_bits_valid<T: PrimitiveUnsigned>(start: u64, end: u64, bits: T) -> bool {
    let bits_width = end - start;
    let bits = bits.mod_power_of_2(bits_width);
    bits == T::ZERO || LeadingZeros::leading_zeros(bits) >= start
}

fn signed_assign_bits_valid<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>(
    x: T,
    start: u64,
    end: u64,
    bits: U,
) -> bool {
    if x >= T::ZERO {
        unsigned_assign_bits_valid(start, end, bits) && {
            let mut abs_self = x.unsigned_abs();
            abs_self.assign_bits(start, end, &bits);
            !abs_self.get_highest_bit()
        }
    } else {
        start <= end && {
            let width = T::WIDTH;
            let bits_width = end - start;
            let bits = bits.mod_power_of_2(bits_width);
            bits_width <= width
                && if start >= width - 1 {
                    bits == U::low_mask(bits_width)
                } else {
                    end < width || bits >> (width - 1 - start) == U::low_mask(end - width + 1)
                }
        }
    }
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

// All `char`s except for `char::MAX`.
pub fn char_gen_var_1() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_1,
        &random_char_gen_var_1,
        &special_random_char_gen_var_1,
    )
}

// All `char`s except for `char::MIN`.
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

// -- FromSciStringOptions --

pub fn from_sci_string_options_gen() -> Generator<FromSciStringOptions> {
    Generator::new_no_special(
        &exhaustive_from_sci_string_options_gen,
        &random_from_sci_string_options_gen,
    )
}

// -- (FromSciStringOptions, PrimitiveUnsigned) --

// All `(FromSciStringOptions, T)` where `T` is unsigned and the `T` is between 2 and 36, inclusive.
pub fn from_sci_string_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(FromSciStringOptions, T)> {
    Generator::new(
        &exhaustive_from_sci_string_options_unsigned_pair_gen_var_1,
        &random_from_sci_string_options_unsigned_pair_gen_var_1,
        &special_random_from_sci_string_options_unsigned_pair_gen_var_1,
    )
}

// -- (FromSciStringOptions, RoundingMode) --

pub fn from_sci_string_options_rounding_mode_pair_gen(
) -> Generator<(FromSciStringOptions, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_from_sci_string_options_rounding_mode_pair_gen,
        &random_from_sci_string_options_rounding_mode_pair_gen,
    )
}

// -- PrimitiveFloat --

pub fn primitive_float_gen<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen,
        &random_primitive_float_gen,
        &special_random_primitive_float_gen,
    )
}

// All primitive floats that are finite, not NaN, and greater than or equal to -0.5.
pub fn primitive_float_gen_var_1<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_1,
        &random_primitive_float_gen_var_1,
        &special_random_primitive_float_gen_var_1,
    )
}

// All primitive floats that are non-negative and equal to an integer.
pub fn primitive_float_gen_var_2<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_2,
        &random_primitive_float_gen_var_2,
        &special_random_primitive_float_gen_var_2,
    )
}

// All primitive floats that are non-negative and not equal to any integer.
pub fn primitive_float_gen_var_3<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_3,
        &random_primitive_float_gen_var_3,
        &special_random_primitive_float_gen_var_3,
    )
}

// All primitive floats that are non-negative and halfway between two adjacent integers.
pub fn primitive_float_gen_var_4<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_4,
        &random_primitive_float_gen_var_4,
        &special_random_primitive_float_gen_var_4,
    )
}

// All primitive floats that are equal to an integer.
pub fn primitive_float_gen_var_5<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_5,
        &random_primitive_float_gen_var_5,
        &special_random_primitive_float_gen_var_5,
    )
}

// All primitive floats that are not equal to any integer.
pub fn primitive_float_gen_var_6<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_6,
        &random_primitive_float_gen_var_6,
        &special_random_primitive_float_gen_var_6,
    )
}

// All primitive floats that are halfway between two adjacent integers.
pub fn primitive_float_gen_var_7<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_7,
        &random_primitive_float_gen_var_7,
        &special_random_primitive_float_gen_var_7,
    )
}

// All finite primitive floats.
pub fn primitive_float_gen_var_8<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_8,
        &random_primitive_float_gen_var_8,
        &special_random_primitive_float_gen_var_8,
    )
}

// All primitive floats that are not NaN or positive infinity.
pub fn primitive_float_gen_var_9<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_9,
        &random_primitive_float_gen_var_9,
        &special_random_primitive_float_gen_var_9,
    )
}

// All primitive floats that are not NaN or negative infinity.
pub fn primitive_float_gen_var_10<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_10,
        &random_primitive_float_gen_var_10,
        &special_random_primitive_float_gen_var_10,
    )
}

// All primitive floats that are not NaN.
pub fn primitive_float_gen_var_11<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_11,
        &random_primitive_float_gen_var_11,
        &special_random_primitive_float_gen_var_11,
    )
}

// All nonzero finite primitive floats.
pub fn primitive_float_gen_var_12<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_12,
        &random_primitive_float_gen_var_12,
        &special_random_primitive_float_gen_var_12,
    )
}

// All primitive floats `T` that are equal to an unsigned value of type `U`.
pub fn primitive_float_gen_var_13<T: PrimitiveFloat + RoundingFrom<U>, U: PrimitiveUnsigned>(
) -> Generator<T>
where
    NiceFloat<T>: TryFrom<U>,
{
    Generator::new(
        &exhaustive_primitive_float_gen_var_13::<T, U>,
        &random_primitive_float_gen_var_13::<T, U>,
        &special_random_primitive_float_gen_var_13::<T, U>,
    )
}

// All primitive floats `T` that are equal to a signed value of type `U`.
pub fn primitive_float_gen_var_14<T: PrimitiveFloat + RoundingFrom<U>, U: PrimitiveSigned>(
) -> Generator<T>
where
    NiceFloat<T>: TryFrom<U>,
{
    Generator::new(
        &exhaustive_primitive_float_gen_var_14::<T, U>,
        &random_primitive_float_gen_var_13::<T, U>,
        &special_random_primitive_float_gen_var_14::<T, U>,
    )
}

// All primitive floats of type `T` that are not equal to any primitive integer of type `U`.
pub fn primitive_float_gen_var_15<T: PrimitiveFloat, U: ConvertibleFrom<T> + PrimitiveInt>(
) -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_15::<T, U>,
        &random_primitive_float_gen_var_14::<T, U>,
        &special_random_primitive_float_gen_var_15::<T, U>,
    )
}

// All primitive floats of type `T` that are halfway between two adjacent values of the primitive
// unsigned type `U`.
pub fn primitive_float_gen_var_16<T: PrimitiveFloat + RoundingFrom<U>, U: PrimitiveUnsigned>(
) -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_16::<T, U>,
        &random_primitive_float_gen_var_15::<T, U>,
        &special_random_primitive_float_gen_var_16::<T, U>,
    )
}

// All primitive floats of type `T` that are halfway between two adjacent values of the primitive
// signed type `U`.
pub fn primitive_float_gen_var_17<T: PrimitiveFloat + RoundingFrom<U>, U: PrimitiveSigned>(
) -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_17::<T, U>,
        &random_primitive_float_gen_var_16::<T, U>,
        &special_random_primitive_float_gen_var_17::<T, U>,
    )
}

// All positive finite primitive floats of type `T`.
pub fn primitive_float_gen_var_18<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_18,
        &random_primitive_float_gen_var_17,
        &special_random_primitive_float_gen_var_18,
    )
}

// All non-negative finite primitive floats of type `T` that are less than or equal to the largest
// representable power of 2. Negative zero is excluded.
pub fn primitive_float_gen_var_19<T: PrimitiveFloat>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_float_gen_var_19,
        &random_primitive_float_gen_var_18,
        &special_random_primitive_float_gen_var_19,
    )
}

// -- (PrimitiveFloat, PrimitiveFloat) --

pub fn primitive_float_pair_gen<T: PrimitiveFloat>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_primitive_float_pair_gen,
        &random_primitive_float_pair_gen,
        &special_random_primitive_float_pair_gen,
    )
}

// All pairs of primitive floats that are not NaN.
pub fn primitive_float_pair_gen_var_1<T: PrimitiveFloat>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_primitive_float_pair_gen_var_1,
        &random_primitive_float_pair_gen_var_1,
        &special_random_primitive_float_pair_gen_var_1,
    )
}

// -- (PrimitiveFloat, PrimitiveFloat, PrimitiveFloat) --

pub fn primitive_float_triple_gen<T: PrimitiveFloat>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_primitive_float_triple_gen,
        &random_primitive_float_triple_gen,
        &special_random_primitive_float_triple_gen,
    )
}

// -- (PrimitiveFloat, PrimitiveSigned) --

pub fn primitive_float_signed_pair_gen<T: PrimitiveFloat, U: PrimitiveSigned>() -> Generator<(T, U)>
{
    Generator::new(
        &exhaustive_primitive_float_signed_pair_gen,
        &random_primitive_float_signed_pair_gen,
        &special_random_primitive_float_signed_pair_gen,
    )
}

// All `(T, U)` where `T` is a primitive float type, `U` is signed, the `T` is finite and positive,
// and the `U` is small.
pub fn primitive_float_signed_pair_gen_var_1<T: PrimitiveFloat, U: PrimitiveSigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_float_signed_pair_gen_var_1,
        &random_primitive_float_signed_pair_gen_var_1,
        &special_random_primitive_float_signed_pair_gen_var_1,
    )
}

// All `(T, i64)` that are valid inputs to `T::from_sci_mantissa_and_exponent`.
pub fn primitive_float_signed_pair_gen_var_2<T: PrimitiveFloat>() -> Generator<(T, i64)> {
    Generator::new(
        &exhaustive_primitive_float_signed_pair_gen_var_2,
        &random_primitive_float_signed_pair_gen_var_2,
        &special_random_primitive_float_signed_pair_gen_var_2,
    )
}

// All `(T, i64)` where `T` is a primitive float type, the `T` is greater than or equal to 1.0 and
// less than 2.0, and the `i64` is small.
pub fn primitive_float_signed_pair_gen_var_3<T: PrimitiveFloat>() -> Generator<(T, i64)> {
    Generator::new(
        &exhaustive_primitive_float_signed_pair_gen_var_3,
        &random_primitive_float_signed_pair_gen_var_3,
        &special_random_primitive_float_signed_pair_gen_var_3,
    )
}

// All `(T, U)` where `T` is a primitive float type, `U` is signed, and the `U` is small.
pub fn primitive_float_signed_pair_gen_var_4<T: PrimitiveFloat, U: PrimitiveSigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_float_signed_pair_gen_var_4,
        &random_primitive_float_signed_pair_gen_var_4,
        &special_random_primitive_float_signed_pair_gen_var_4,
    )
}

// -- (PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned) --

// All `(T, U, V)` where `T` is a primitive float type, `U` is signed, `V` is unsigned, the `U` is
// small, and the `V` is small and positive.
pub fn primitive_float_signed_unsigned_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_primitive_float_signed_unsigned_triple_gen_var_1,
        &random_primitive_float_signed_unsigned_triple_gen_var_1,
        &special_random_primitive_float_signed_unsigned_triple_gen_var_1,
    )
}

// -- (PrimitiveFloat, PrimitiveUnsigned) --

// All `(T, U)` where `T` is a primitive float type, `U` is unsigned, the `T` is finite and
// positive, and the `U` is small.
pub fn primitive_float_unsigned_pair_gen_var_1<T: PrimitiveFloat, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_float_unsigned_pair_gen_var_1,
        &random_primitive_float_unsigned_pair_gen_var_1,
        &special_random_primitive_float_unsigned_pair_gen_var_1,
    )
}

// All `(T, u64)` where `T` is a primitive float type, the `T` is greater than or equal to 1.0 and
// less than 2.0, and the `u64` is small.
pub fn primitive_float_unsigned_pair_gen_var_2<T: PrimitiveFloat>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_primitive_float_unsigned_pair_gen_var_2,
        &random_primitive_float_unsigned_pair_gen_var_2,
        &special_random_primitive_float_unsigned_pair_gen_var_2,
    )
}

// All `(T, U)` where `T` is a primitive float type, `U` is unsigned, the `T` is finite and
// positive, and the `U` is small and positive.
pub fn primitive_float_unsigned_pair_gen_var_3<T: PrimitiveFloat, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_float_primitive_int_pair_gen_var_1,
        &random_primitive_float_unsigned_pair_gen_var_3,
        &special_random_primitive_float_unsigned_pair_gen_var_3,
    )
}

// All `(T, U)` where `T` is a primitive float type, `U` is unsigned, and the `U` is small and
// positive.
pub fn primitive_float_unsigned_pair_gen_var_4<T: PrimitiveFloat, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_float_unsigned_pair_gen_var_3,
        &random_primitive_float_unsigned_pair_gen_var_4,
        &special_random_primitive_float_unsigned_pair_gen_var_4,
    )
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

// All `(T, U, RoundingMode)` where `T` is a primitive float type, `U` is unsigned, the `T` is
// finite and positive, and the `U` is small.
pub fn primitive_float_unsigned_rounding_mode_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>() -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_1,
        &random_primitive_float_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// All `(T, u64, RoundingMode)` where `T` is a primitive float type, the `T` is greater than or
// equal to 1.0 and less than 2.0, and the `u64` is small.
pub fn primitive_float_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveFloat>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_2,
        &random_primitive_float_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_primitive_float_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// var 3 is in malachite-float.

// -- (PrimitiveFloat, RoundingMode) --

// All `(T, RoundingMode)` where `T` is a primitive float and the values are valid inputs to
// `Natural::rounding_from`.
pub fn primitive_float_rounding_mode_pair_gen_var_1<T: PrimitiveFloat>(
) -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_primitive_float_rounding_mode_pair_gen_var_1,
        &random_primitive_float_rounding_mode_pair_gen_var_1,
        &special_random_primitive_float_rounding_mode_pair_gen_var_1,
    )
}

// All `(T, RoundingMode)` where `T` is a primitive float and the values are valid inputs to
// `Integer::rounding_from`.
pub fn primitive_float_rounding_mode_pair_gen_var_2<T: PrimitiveFloat>(
) -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_primitive_float_rounding_mode_pair_gen_var_2,
        &random_primitive_float_rounding_mode_pair_gen_var_2,
        &special_random_primitive_float_rounding_mode_pair_gen_var_2,
    )
}

// All `(T, RoundingMode)` where `T` is a primitive float, `U` is unsigned, and the pair is a valid
// input to `U::rounding_from`.
pub fn primitive_float_rounding_mode_pair_gen_var_3<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: ConvertibleFrom<T> + PrimitiveInt,
>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_primitive_float_rounding_mode_pair_gen_var_3::<T, U>,
        &random_primitive_float_rounding_mode_pair_gen_var_3::<T, U>,
        &special_random_primitive_float_rounding_mode_pair_gen_var_3::<T, U>,
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

// All signed `T`s which are not `T::MIN`.
pub fn signed_gen_var_1<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_1,
        &random_signed_gen_var_1,
        &special_random_signed_gen_var_1,
    )
}

// All signed natural (non-negative) `T`s.
pub fn signed_gen_var_2<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_2,
        &random_signed_gen_var_2,
        &special_random_signed_gen_var_2,
    )
}

// All signed `T`s that are neither 0 nor -1.
pub fn signed_gen_var_3<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_3,
        &random_signed_gen_var_3,
        &special_random_signed_gen_var_3,
    )
}

// All negative signed `T`s.
pub fn signed_gen_var_4<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_4,
        &random_signed_gen_var_4,
        &special_random_signed_gen_var_4,
    )
}

// All small signed `T`s.
pub fn signed_gen_var_5<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new_no_special(&exhaustive_signed_gen::<T>, &random_signed_gen_var_5::<T>)
}

// All nonzero signed `T`s.
pub fn signed_gen_var_6<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_5,
        &random_signed_gen_var_6,
        &special_random_signed_gen_var_5,
    )
}

// All signeds `T` that are equal to a primitive float value of type `U`.
pub fn signed_gen_var_7<
    T: PrimitiveSigned + RoundingFrom<U>,
    U: ConvertibleFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_6::<T, U>,
        &random_primitive_int_gen_var_1::<T, U>,
        &special_random_primitive_int_gen_var_1::<T, U>,
    )
}

type G<T> = Generator<T>;
// All signeds `S` that are not exactly equal to any value of a floating-point type `V`.
//
// Acceptable `(S, V)` pairs are those where `S::WIDTH` > `V::MANTISSA_WIDTH`.
pub fn signed_gen_var_8<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: ConvertibleFrom<S> + PrimitiveFloat,
>() -> G<S> {
    Generator::new(
        &exhaustive_signed_gen_var_7::<S, V>,
        &random_signed_gen_var_7::<S, V>,
        &special_random_signed_gen_var_6::<U, S, V>,
    )
}

// All signeds `S` that are exactly between two values of a floating-point type `V`.
//
// Acceptable `(S, V)` pairs are those where `S::WIDTH` > `V::MANTISSA_WIDTH`.
pub fn signed_gen_var_9<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: TryFrom<NiceFloat<V>> + PrimitiveSigned + WrappingFrom<U>,
    V: ConvertibleFrom<S> + PrimitiveFloat + RoundingFrom<S>,
>() -> Generator<S> {
    Generator::new(
        &exhaustive_signed_gen_var_8::<S, V>,
        &random_signed_gen_var_8::<S, V>,
        &special_random_signed_gen_var_7::<U, S, V>,
    )
}

// All signed `T`s whose square is also representable as a `T`.
pub fn signed_gen_var_10<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<S> {
    Generator::new(
        &exhaustive_signed_gen_var_9::<U, S>,
        &random_signed_gen_var_9::<U, S>,
        &special_random_signed_gen_var_8::<U, S>,
    )
}

// All valid scientific exponents for the float type `T`.
pub fn signed_gen_var_11<T: PrimitiveFloat>() -> Generator<i64> {
    Generator::new(
        &exhaustive_signed_gen_var_10::<T>,
        &random_signed_gen_var_10::<T>,
        &special_random_signed_gen_var_9::<T>,
    )
}

// All signed `T`s that are neither 0 nor -1.
pub fn signed_gen_var_12<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_11,
        &random_signed_gen_var_11,
        &special_random_signed_gen_var_10,
    )
}

// All odd positive signeds.
pub fn signed_gen_var_13<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<S> {
    Generator::new(
        &exhaustive_signed_gen_var_12,
        &random_signed_gen_var_12,
        &special_random_signed_gen_var_11::<U, S>,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn signed_pair_gen<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen,
        &random_primitive_int_pair_gen_var_1,
        &special_random_signed_pair_gen,
    )
}

// All pairs of signeds where either both values are non-negative or both are negative.
pub fn signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_1,
        &random_signed_pair_gen_var_1,
        &special_random_signed_pair_gen_var_1,
    )
}

// All pairs of signeds where the second value is small.
pub fn signed_pair_gen_var_2<T: PrimitiveSigned, U: PrimitiveSigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_3,
        &random_primitive_int_signed_pair_gen_var_1,
        &special_random_signed_pair_gen_var_2,
    )
}

// All pairs of signed `T` where the first is divisible by the second.
pub fn signed_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_4,
        &random_signed_pair_gen_var_2,
        &special_random_signed_pair_gen_var_3,
    )
}

// All pairs of signed `T` where the second `T` is nonzero and the pair is not `(T::MIN, -1)`.
pub fn signed_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_5,
        &random_signed_pair_gen_var_3,
        &special_random_signed_pair_gen_var_4,
    )
}

// All pairs of signed `T` where the second `T` is nonzero and the first is not divisible by the
// second.
pub fn signed_pair_gen_var_5<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_6,
        &random_signed_pair_gen_var_4,
        &special_random_signed_pair_gen_var_5,
    )
}

// All pairs of signed `T` where the second `T` is nonzero.
pub fn signed_pair_gen_var_6<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_7,
        &random_signed_pair_gen_var_5,
        &special_random_signed_pair_gen_var_6,
    )
}

// All pairs of natural (non-negative) signeds.
pub fn signed_pair_gen_var_7<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_8,
        &random_signed_pair_gen_var_6,
        &special_random_signed_pair_gen_var_7,
    )
}

// All pairs of signeds where the second element is positive and odd.
pub fn signed_pair_gen_var_8<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(S, S)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_9,
        &random_signed_pair_gen_var_7,
        &special_random_signed_pair_gen_var_8::<U, S>,
    )
}

// All coprime pairs of signeds where both elements are odd and positive.
pub fn signed_pair_gen_var_9<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() -> Generator<(S, S)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_10,
        &random_signed_pair_gen_var_8,
        &special_random_signed_pair_gen_var_9::<U, S>,
    )
}

// All coprime pairs of signeds.
pub fn signed_pair_gen_var_10<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>() -> Generator<(S, S)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_11,
        &random_signed_pair_gen_var_9,
        &special_random_signed_pair_gen_var_10,
    )
}

// All `(T, U)` where `T` and `U` are signed and both the `T` and the `U` are small.
pub fn signed_pair_gen_var_11<T: PrimitiveSigned, U: PrimitiveSigned>() -> Generator<(T, U)> {
    Generator::new_no_special(
        &exhaustive_signed_pair_gen_var_12,
        &random_signed_pair_gen_var_10,
    )
}

// All `(T, T)` where the `T`s are small, signed, and valid inputs to `T::binomial_coefficient`.
pub fn signed_pair_gen_var_12<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new_no_special(
        &exhaustive_signed_pair_gen_var_13,
        &random_signed_pair_gen_var_11,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn signed_triple_gen<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen,
        &random_primitive_int_triple_gen_var_4,
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

// All triple of signeds where either all values are non-negative or all are negative.
pub fn signed_triple_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_3,
        &random_signed_triple_gen_var_3,
        &special_random_signed_triple_gen_var_3,
    )
}

// All triples of signeds (x, y, m) where x is equal to y mod m.
pub fn signed_triple_gen_var_4<
    U: PrimitiveUnsigned + WrappingInto<S> + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() -> Generator<(S, S, S)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_4,
        &random_signed_triple_gen_var_4::<U, S>,
        &special_random_signed_triple_gen_var_4::<U, S>,
    )
}

// All triples of signeds (x, y, m) where x is not equal to y mod m.
pub fn signed_triple_gen_var_5<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_5,
        &random_primitive_int_triple_gen_var_1,
        &special_random_signed_triple_gen_var_5,
    )
}

// All triples of signeds where the third element is positive and odd.
pub fn signed_triple_gen_var_6<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(S, S, S)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_6,
        &random_signed_triple_gen_var_5,
        &special_random_signed_triple_gen_var_6::<U, S>,
    )
}

// All triples of signeds where the second and third elements are positive and odd.
pub fn signed_triple_gen_var_7<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(S, S, S)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_7,
        &random_signed_triple_gen_var_6,
        &special_random_signed_triple_gen_var_7::<U, S>,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn signed_quadruple_gen<T: PrimitiveSigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_signed_quadruple_gen,
        &random_primitive_int_quadruple_gen_var_4,
        &special_random_signed_quadruple_gen,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

// All `(T, T, T, U)` where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_signed_signed_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> Generator<(T, T, T, U)> {
    Generator::new(
        &exhaustive_signed_signed_signed_unsigned_quadruple_gen_var_1,
        &random_primitive_int_primitive_int_primitive_int_unsigned_quadruple_gen_var_1,
        &special_random_signed_signed_signed_unsigned_quadruple_gen_var_2,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

// All triples `(T, T, u64)` (x, y, k) where `T` is signed and x is equal to y mod $2^k$.
pub fn signed_signed_unsigned_triple_gen_var_1<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(S, S, u64)> {
    Generator::new(
        &exhaustive_signed_signed_unsigned_triple_gen_var_1::<U, S>,
        &random_signed_signed_unsigned_triple_gen_var_1::<U, S>,
        &special_random_signed_signed_unsigned_triple_gen_var_1::<U, S>,
    )
}

// All `(T, T, U)` where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_signed_unsigned_triple_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_4,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_2,
        &special_random_signed_signed_unsigned_triple_gen_var_2,
    )
}

// All triples `(T, T, u64)` (x, y, k) where `T` is unsigned and x is not equal to y mod $2^k$.
pub fn signed_signed_unsigned_triple_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_signed_signed_unsigned_triple_gen_var_2,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_3,
        &special_random_signed_signed_unsigned_triple_gen_var_3,
    )
}

// All `(T, U, V)` where `T` is signed, `U` is signed and small, and `V` is unsigned, small, and
// positive.
pub fn signed_signed_unsigned_triple_gen_var_4<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_signed_signed_unsigned_triple_gen_var_3,
        &random_signed_signed_unsigned_triple_gen_var_2,
        &special_random_signed_signed_unsigned_triple_gen_var_4,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, RoundingMode) --

// All `(T, T, RoundingMode)` where `T` is signed and the triple is a valid input to `T::div_round`.
pub fn signed_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(T, T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_signed_rounding_mode_triple_gen_var_1,
        &random_signed_signed_rounding_mode_triple_gen_var_1,
        &special_random_signed_signed_rounding_mode_triple_gen_var_1,
    )
}

// All `(T, T, RoundingMode)` where `T` is signed and the triple is a valid input to
// `T::round_to_multiple`.
pub fn signed_signed_rounding_mode_triple_gen_var_2<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() -> Generator<(S, S, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_signed_rounding_mode_triple_gen_var_2,
        &random_signed_signed_rounding_mode_triple_gen_var_2,
        &special_random_signed_signed_rounding_mode_triple_gen_var_2,
    )
}

// All `(T, U, RoundingMode)` where `T` and `U` are signed and the triple is a valid input to
// `T::shr_round`.
pub fn signed_signed_rounding_mode_triple_gen_var_3<T: PrimitiveSigned, U: PrimitiveSigned>(
) -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_signed_rounding_mode_triple_gen_var_3,
        &random_primitive_int_signed_rounding_mode_triple_gen_var_1,
        &special_random_signed_signed_rounding_mode_triple_gen_var_3,
    )
}

// All `(T, U, RoundingMode)` where `T` and `U` are signed and the triple is a valid input to
// `T::shl_round`.
pub fn signed_signed_rounding_mode_triple_gen_var_4<T: PrimitiveSigned, U: PrimitiveSigned>(
) -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_signed_rounding_mode_triple_gen_var_4,
        &random_primitive_int_signed_rounding_mode_triple_gen_var_2,
        &special_random_signed_signed_rounding_mode_triple_gen_var_4,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

pub fn signed_unsigned_pair_gen<T: PrimitiveSigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_signed_unsigned_pair_gen,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_unsigned_pair_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_1,
        &special_random_signed_unsigned_pair_gen_var_1,
    )
}

// All `(T, u64)`s where `T` is signed and the `u64` is smaller than `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_4,
        &random_primitive_int_unsigned_pair_gen_var_2,
        &special_random_signed_unsigned_pair_gen_var_2,
    )
}

// All `(T, u64)`s where `T` is signed and the either the `T` is negative or the `u64` is less than
// `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_3,
        &random_signed_unsigned_pair_gen_var_1,
        &special_random_signed_unsigned_pair_gen_var_3,
    )
}

// All `(T, u64)`s where `T` is signed and the either the `T` is non-negative or the `u64` is less
// than `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_5,
        &random_signed_unsigned_pair_gen_var_2,
        &special_random_signed_unsigned_pair_gen_var_4,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, and the `U` is greater than 1 and no greater
// than 36.
pub fn signed_unsigned_pair_gen_var_5<T: PrimitiveSigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_6,
        &random_primitive_int_unsigned_pair_gen_var_5,
        &special_random_signed_unsigned_pair_gen_var_5,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `T` is non-negative, and the `U` is
// small.
pub fn signed_unsigned_pair_gen_var_6<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_7,
        &random_signed_unsigned_pair_gen_var_3,
        &special_random_signed_unsigned_pair_gen_var_6,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `T` is non-negative, and the `U` is
// greater than 1 and no greater than 36.
pub fn signed_unsigned_pair_gen_var_7<T: PrimitiveSigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_8,
        &random_signed_unsigned_pair_gen_var_4,
        &special_random_signed_unsigned_pair_gen_var_7,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `U` is small, and the `T` is not
// divisible by 2 to the power of the `U`.
pub fn signed_unsigned_pair_gen_var_8<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_9,
        &random_primitive_int_unsigned_pair_gen_var_6,
        &special_random_signed_unsigned_pair_gen_var_8,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `U` is small, and the `T` is divisible by
// 2 to the power of the `U`.
pub fn signed_unsigned_pair_gen_var_9<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_10,
        &random_primitive_int_unsigned_pair_gen_var_7,
        &special_random_signed_unsigned_pair_gen_var_9,
    )
}

// All `(T, u64)`s where `T` is signed and the either the `T` is non-negative or the `u64` is less
// than or equal to `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_10<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_11,
        &random_signed_unsigned_pair_gen_var_5,
        &special_random_signed_unsigned_pair_gen_var_10,
    )
}

// All `(S, u64)`s where `S` is signed and the either the `S` is non-positive and not `S::MIN`, or
// the `u64` is less than `S::WIDTH`.
pub fn signed_unsigned_pair_gen_var_11<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(S, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_12,
        &random_signed_unsigned_pair_gen_var_6,
        &special_random_signed_unsigned_pair_gen_var_11::<U, S>,
    )
}

// All `(T, u64)`s where `T` is signed and the `u64` is between 0 and `U::WIDTH`, inclusive.
pub fn signed_unsigned_pair_gen_var_12<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_13::<T, U>,
        &random_primitive_int_unsigned_pair_gen_var_8::<T, U>,
        &special_random_unsigned_pair_gen_var_17::<T, U>,
    )
}

// All `(S, V)`s where `S` is signed, `V` is unsigned, `S` is between `-u64::MAX` and `u64::MAX`,
// inclusive, and the `V` is positive.
pub fn signed_unsigned_pair_gen_var_13<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveUnsigned,
>() -> Generator<(S, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_14,
        &random_signed_unsigned_pair_gen_var_7,
        &special_random_signed_unsigned_pair_gen_var_12::<U, S, V>,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, and both `T` and `U` are small.
pub fn signed_unsigned_pair_gen_var_14<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new_no_special(
        &exhaustive_signed_unsigned_pair_gen,
        &random_signed_unsigned_pair_gen_var_8,
    )
}

// All `(T, u64)`s where `T` is signed, both the `T` and the `u64` are small, and the `T` raised to
// the power of the `u64` does not overflow.
pub fn signed_unsigned_pair_gen_var_15<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new_no_special(
        &exhaustive_signed_unsigned_pair_gen_var_15,
        &random_signed_unsigned_pair_gen_var_9,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `T` is positive, and the `U` is small.
pub fn signed_unsigned_pair_gen_var_16<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_int_unsigned_pair_gen_var_1,
        &random_signed_unsigned_pair_gen_var_10,
        &special_random_signed_unsigned_pair_gen_var_13,
    )
}

// All `(S, V)`s where `S` is signed, `V` is unsigned, the `S` is negative and not `S::MIN`, and the
// `V` is small.
pub fn signed_unsigned_pair_gen_var_17<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveUnsigned,
>() -> Generator<(S, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_16,
        &random_signed_unsigned_pair_gen_var_11,
        &special_random_signed_unsigned_pair_gen_var_15::<U, S, V>,
    )
}

// All `(T, U)`s where `T` is signed, the `U` is unsigned, positive, and small, and either the `T`
// is non-negative or the `U` is odd.
pub fn signed_unsigned_pair_gen_var_18<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_17,
        &random_signed_unsigned_pair_gen_var_12,
        &special_random_signed_unsigned_pair_gen_var_14,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, and both `T` and `U` are small, and the `U`
// is positive.
pub fn signed_unsigned_pair_gen_var_19<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new_no_special(
        &exhaustive_signed_unsigned_pair_gen_var_18,
        &random_signed_unsigned_pair_gen_var_13,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, and the `U` is small and positive.
pub fn signed_unsigned_pair_gen_var_20<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_19,
        &random_primitive_int_unsigned_pair_gen_var_10,
        &special_random_signed_unsigned_pair_gen_var_16,
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

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(T, U, V)` where `T` is signed, `U` and `V` are unsigned, and the `V` is small.
pub fn signed_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_1,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_2,
        &special_random_signed_unsigned_unsigned_triple_gen_var_1,
    )
}

// All `(S, V, V)` where `S` is signed, `V` is unsigned, both `V`s are small, the first `V` is less
// than or equal to the second, and if the `S` is negative, the difference between the two `V`s is
// no greater than the width of `S`.
pub fn signed_unsigned_unsigned_triple_gen_var_2<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
    V: PrimitiveUnsigned,
>() -> Generator<(S, V, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_2,
        &random_signed_unsigned_unsigned_triple_gen_var_1,
        &special_random_signed_unsigned_unsigned_triple_gen_var_2::<U, S, V>,
    )
}

// All `(T, U, V)`s where `T` is signed, `U` and `V` are unsigned, the `U` is greater than 1 and no
// greater than 36, and the `V` is small.
pub fn signed_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_3,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_3,
        &special_random_signed_unsigned_unsigned_triple_gen_var_3,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(T, u64, u64, U)` where `T` is signed, `U` is unsigned, both `u64`s are small, and the four
// values are valid arguments to `assign_bits`.
pub fn signed_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>() -> Generator<(T, u64, u64, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1,
        &random_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1,
        &special_random_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

// All `(T, u64, RoundingMode)` where `T` is signed and the triple is a valid input to
// `T::round_to_multiple_of_power_of_2`.
pub fn signed_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_1,
        &random_primitive_int_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_signed_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// All `(T, U, RoundingMode)` where `T` is signed, `U` is unsigned, and the triple is a valid input
// to `T::shr_round`.
pub fn signed_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_unsigned_rounding_mode_triple_gen_var_2,
        &random_primitive_int_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_signed_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// var 3 is in malachite-float.

// -- (PrimitiveSigned, RoundingMode) --

pub fn signed_rounding_mode_pair_gen<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen,
        &random_primitive_int_rounding_mode_pair_gen,
        &special_random_signed_rounding_mode_pair_gen,
    )
}

// All `(T, RoundingMode)`s where `T` is signed and the `T` is nonzero.
pub fn signed_rounding_mode_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_1,
        &random_signed_rounding_mode_pair_gen_var_1,
        &special_random_signed_rounding_mode_pair_gen_var_1,
    )
}

// All `(T, RoundingMode)`s where `T` is signed and the `T` is not `T::MIN`.
pub fn signed_rounding_mode_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_2,
        &random_signed_rounding_mode_pair_gen_var_2,
        &special_random_signed_rounding_mode_pair_gen_var_2,
    )
}

// All `(T, RoundingMode)`s where `T` is signed and the `T` is nonzero and not `T::MIN`.
pub fn signed_rounding_mode_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_3,
        &random_signed_rounding_mode_pair_gen_var_3,
        &special_random_signed_rounding_mode_pair_gen_var_3,
    )
}

// All `(T, RoundingMode)` where `T` is signed, `U` is a primitive float type, and the pair is a
// valid input to `U::rounding_from`.
pub fn signed_rounding_mode_pair_gen_var_4<
    T: PrimitiveSigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_4::<T, U>,
        &random_primitive_int_rounding_mode_pair_gen_var_1::<T, U>,
        &special_random_signed_rounding_mode_pair_gen_var_4::<T, U>,
    )
}

// All `(T, RoundingMode)`s where `T` is small and signed.
pub fn signed_rounding_mode_pair_gen_var_5<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_signed_rounding_mode_pair_gen,
        &random_signed_rounding_mode_pair_gen_var_4,
    )
}

// -- (PrimitiveSigned, ToSciOptions) --

pub fn signed_to_sci_options_pair_gen<T: PrimitiveSigned>() -> Generator<(T, ToSciOptions)> {
    Generator::new(
        &exhaustive_signed_to_sci_options_pair_gen,
        &random_primitive_int_to_sci_options_pair_gen,
        &special_random_signed_to_sci_options_pair_gen,
    )
}

// All `(T, ToSciOptions)` pairs where `T` is signed and the `T` can be formatted using the options.
pub fn signed_to_sci_options_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, ToSciOptions)> {
    Generator::new(
        &exhaustive_signed_to_sci_options_pair_gen_var_1,
        &random_primitive_int_to_sci_options_pair_gen_var_1,
        &special_random_signed_to_sci_options_pair_gen_var_1,
    )
}

// -- (PrimitiveSigned, Vec<bool>) --

// All `(T, Vec<bool>)` where `T` is signed and the `Vec` has as many elements as
// `u64::exact_from(n.to_bits_asc().len())` (which is not necessarily the same as the number of
// significant bits).
pub fn signed_bool_vec_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, Vec<bool>)> {
    Generator::new(
        &exhaustive_signed_bool_vec_pair_gen_var_1,
        &random_signed_bool_vec_pair_gen_var_1,
        &special_random_signed_bool_vec_pair_gen_var_1,
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

// All unsigned positive `T`s.
pub fn unsigned_gen_var_1<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_1,
        &random_unsigned_gen_var_1,
        &special_random_unsigned_gen_var_1,
    )
}

// All `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn unsigned_gen_var_2() -> Generator<u32> {
    Generator::new(
        &exhaustive_unsigned_gen_var_1,
        &random_unsigned_gen_var_2,
        &special_random_unsigned_gen_var_2,
    )
}

// All `u64`s between 1 and `T::WIDTH`, inclusive, where `T` is a primitive integer.
pub fn unsigned_gen_var_3<T: PrimitiveInt>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_2::<T>,
        &random_unsigned_gen_var_3::<T>,
        &special_random_unsigned_gen_var_3::<T>,
    )
}

// All `U`s greater than 1 and no greater than `T::MAX`.
pub fn unsigned_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned + SaturatingFrom<T>>(
) -> Generator<U> {
    Generator::new(
        &exhaustive_unsigned_gen_var_4::<T, U>,
        &random_unsigned_gen_var_4::<T, U>,
        &special_random_unsigned_gen_var_4::<T, U>,
    )
}

// All small unsigned `T`s.
pub fn unsigned_gen_var_5<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_unsigned_gen::<T>,
        &random_unsigned_gen_var_5::<T>,
    )
}

// All unsigned `T`s greater than or equal to 2.
pub fn unsigned_gen_var_6<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_2::<T>,
        &random_unsigned_gen_var_6::<T>,
        &special_random_unsigned_gen_var_5::<T>,
    )
}

// All unsigned `T`s less than 36.
pub fn unsigned_gen_var_7<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_3,
        &random_unsigned_gen_var_7,
        &special_random_unsigned_gen_var_6,
    )
}

// All unsigned `T`s greater than or equal to 2 and less than or equal to 36.
pub fn unsigned_gen_var_8<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_4,
        &random_unsigned_gen_var_8,
        &special_random_unsigned_gen_var_7,
    )
}

// All `u64`s between 0 and `T::WIDTH`, inclusive, where `T` is a primitive integer.
pub fn unsigned_gen_var_9<T: PrimitiveInt>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_5::<T>,
        &random_unsigned_gen_var_9::<T>,
        &special_random_unsigned_gen_var_8::<T>,
    )
}

// All `u8`s that correspond to an ASCII alphanumeric character: '0' through '9', 'a' through 'z',
// and 'A' through 'Z'.
pub fn unsigned_gen_var_10() -> Generator<u8> {
    Generator::new(
        &exhaustive_unsigned_gen_var_6,
        &random_unsigned_gen_var_10,
        &special_random_unsigned_gen_var_9,
    )
}

// All small positive unsigned `T`s.
pub fn unsigned_gen_var_11<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_primitive_int_gen_var_1::<T>,
        &random_unsigned_gen_var_11::<T>,
    )
}

// All unsigned `T`s whose highest bit is set.
pub fn unsigned_gen_var_12<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_7,
        &random_unsigned_gen_var_12,
        &special_random_unsigned_gen_var_10,
    )
}

// All unsigneds that are valid inputs into `T::from_ordered_representation` for a float type `T`.
pub fn unsigned_gen_var_13<T: PrimitiveFloat>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_8::<T>,
        &random_unsigned_gen_var_13::<T>,
        &special_random_unsigned_gen_var_11::<T>,
    )
}

// All unsigned `T`s that are less than or equal to the largest representable power of 2.
pub fn unsigned_gen_var_14<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_9,
        &random_unsigned_gen_var_14,
        &special_random_unsigned_gen_var_12,
    )
}

// All `u64`s between 0 and `T::WIDTH - 1`, inclusive, where `T` is a primitive integer.
pub fn unsigned_gen_var_15<T: PrimitiveInt>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_10::<T>,
        &random_unsigned_gen_var_15::<T>,
        &special_random_unsigned_gen_var_13::<T>,
    )
}

// All `u64`s between 0 and `T::WIDTH - 2`, inclusive, where `T` is a primitive integer.
pub fn unsigned_gen_var_16<T: PrimitiveInt>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_11::<T>,
        &random_unsigned_gen_var_16::<T>,
        &special_random_unsigned_gen_var_14::<T>,
    )
}

// All unsigned `T`s whose two highest bits are not both zero.
pub fn unsigned_gen_var_17<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_5,
        &random_unsigned_gen_var_17,
        &special_random_unsigned_gen_var_15,
    )
}

// All unsigneds `T` that are equal to a primitive float value of type `U`.
pub fn unsigned_gen_var_18<
    T: PrimitiveUnsigned + RoundingFrom<U>,
    U: ConvertibleFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_12::<T, U>,
        &random_primitive_int_gen_var_1::<T, U>,
        &special_random_primitive_int_gen_var_1::<T, U>,
    )
}

// All unsigneds `T` that are not exactly equal to any value of a floating-point type `U`.
//
// Acceptable `(T, U)` pairs are those where `T::WIDTH` > `U::MANTISSA_WIDTH`.
pub fn unsigned_gen_var_19<T: PrimitiveUnsigned, U: ConvertibleFrom<T> + PrimitiveFloat>(
) -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_13::<T, U>,
        &random_unsigned_gen_var_18::<T, U>,
        &special_random_unsigned_gen_var_16::<T, U>,
    )
}

// All unsigneds `T` that are exactly between two values of a floating-point type `U`.
//
// Acceptable `(T, U)` pairs are those where `T::WIDTH` > `U::MANTISSA_WIDTH`.
pub fn unsigned_gen_var_20<
    T: TryFrom<NiceFloat<U>> + PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_14::<T, U>,
        &random_unsigned_gen_var_19::<T, U>,
        &special_random_unsigned_gen_var_17::<T, U>,
    )
}

// All unsigned `T`s whose square is also representable as a `T`.
pub fn unsigned_gen_var_21<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_15,
        &random_unsigned_gen_var_20,
        &special_random_unsigned_gen_var_18,
    )
}

// All odd unsigned `T`s.
pub fn unsigned_gen_var_22<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen_var_22,
        &random_unsigned_gen_var_21,
        &special_random_unsigned_gen_var_19,
    )
}

pub(crate) fn smallest_invalid_value<T: PrimitiveUnsigned, F: Fn(u64) -> Option<T>>(f: F) -> u64 {
    for n in 0.. {
        if f(n).is_none() {
            return n;
        }
    }
    0
}

// All `u64`s whose factorial is representable as a `T`.
pub fn unsigned_gen_var_23<T: PrimitiveUnsigned>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_23::<T>,
        &random_unsigned_gen_var_22::<T>,
        &special_random_unsigned_gen_var_20::<T>,
    )
}

// All `u64`s whose double factorial is representable as a `T`.
pub fn unsigned_gen_var_24<T: PrimitiveUnsigned>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_24::<T>,
        &random_unsigned_gen_var_23::<T>,
        &special_random_unsigned_gen_var_21::<T>,
    )
}

// All `u64`s whose subfactorial is representable as a `T`.
pub fn unsigned_gen_var_25<T: PrimitiveUnsigned>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_25::<T>,
        &random_unsigned_gen_var_24::<T>,
        &special_random_unsigned_gen_var_22::<T>,
    )
}

// All small unsigned `T`s greater than 4.
pub fn unsigned_gen_var_26<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_primitive_int_gen_var_6,
        &random_unsigned_gen_var_25,
    )
}

// All `u64`s whose primorial is representable as a `T`.
pub fn unsigned_gen_var_27<T: PrimitiveUnsigned>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_26::<T>,
        &random_unsigned_gen_var_26::<T>,
        &special_random_unsigned_gen_var_23::<T>,
    )
}

// All `u64`s `n` such that the product of the first `n` primes is representable as a `T`.
pub fn unsigned_gen_var_28<T: PrimitiveUnsigned>() -> Generator<u64> {
    Generator::new(
        &exhaustive_unsigned_gen_var_27::<T>,
        &random_unsigned_gen_var_27::<T>,
        &special_random_unsigned_gen_var_24::<T>,
    )
}

// -- (PrimitiveUnsigned, PrimitiveSigned) --

pub fn unsigned_signed_pair_gen<T: PrimitiveUnsigned, U: PrimitiveSigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_signed_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_unsigned_signed_pair_gen,
    )
}

// All `(T, U)`s where `T` is unsigned, `U` is signed, and the `U` is small.
pub fn unsigned_signed_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_signed_pair_gen_var_1,
        &random_primitive_int_signed_pair_gen_var_1,
        &special_random_unsigned_signed_pair_gen_var_1,
    )
}

// Given a float type `T`, returns all `(T::UnsignedOfEqualWidth, i64)` that are valid inputs to
// `T::from_integer_mantissa_and_exponent`.
pub fn unsigned_signed_pair_gen_var_2<T: PrimitiveFloat>() -> Generator<(u64, i64)> {
    Generator::new(
        &exhaustive_unsigned_signed_pair_gen_var_2::<T>,
        &random_unsigned_signed_pair_gen_var_1::<T>,
        &special_random_unsigned_signed_pair_gen_var_2::<T>,
    )
}

// -- (PrimitiveUnsigned, PrimitiveSigned, PrimitiveUnsigned) --

// All `(T, U, u64)` where `T` is unsigned, `U` is signed, the `U` is small, the u64 is no greater
// than `T::WIDTH`, and the `T`s is less than 2 to the power of the `u64`.
pub fn unsigned_signed_unsigned_triple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> Generator<(T, U, u64)> {
    Generator::new(
        &exhaustive_unsigned_signed_unsigned_triple_gen_var_1,
        &random_unsigned_signed_unsigned_triple_gen_var_1,
        &special_random_unsigned_signed_unsigned_triple_gen_var_1,
    )
}

// All `(T, S, T)` where `T` is unsigned, `S` is signed, the `S` is between `-u64::MAX` and
// `u64::MAX`, inclusive, and the first element is less than the third.
pub fn unsigned_signed_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(T, S, T)> {
    Generator::new(
        &exhaustive_unsigned_signed_unsigned_triple_gen_var_2,
        &random_primitive_int_signed_primitive_int_triple_gen_var_1,
        &special_random_unsigned_signed_unsigned_triple_gen_var_2::<T, U, S>,
    )
}

// All `(T, U, T)` where `T` is unsigned, `U` is signed, and the first element is less than the
// third.
pub fn unsigned_signed_unsigned_triple_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> Generator<(T, U, T)> {
    Generator::new(
        &exhaustive_unsigned_signed_unsigned_triple_gen_var_3,
        &random_primitive_int_triple_gen_var_3,
        &special_random_unsigned_signed_unsigned_triple_gen_var_3,
    )
}

// All `(T, U, V)` where `T` is unsigned, `U` is signed and small, and `V` is unsigned, small, and
// positive.
pub fn unsigned_signed_unsigned_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_unsigned_signed_unsigned_triple_gen_var_4,
        &random_unsigned_signed_unsigned_triple_gen_var_2,
        &special_random_unsigned_signed_unsigned_triple_gen_var_4,
    )
}

// -- (PrimitiveUnsigned, PrimitiveSigned, RoundingMode) --

// All `(T, U, RoundingMode)` where `T` is unsigned, `U` is signed, and the triple is a valid input
// to `T::shr_round`.
pub fn unsigned_signed_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_signed_rounding_mode_triple_gen_var_1,
        &random_primitive_int_signed_rounding_mode_triple_gen_var_1,
        &special_random_unsigned_signed_rounding_mode_triple_gen_var_1,
    )
}

// All `(T, U, RoundingMode)` where `T` is unsigned, `U` is signed, and the triple is a valid input
// to `T::shl_round`.
pub fn unsigned_signed_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_signed_rounding_mode_triple_gen_var_2,
        &random_primitive_int_signed_rounding_mode_triple_gen_var_2,
        &special_random_unsigned_signed_rounding_mode_triple_gen_var_2,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_unsigned_pair_gen,
    )
}

// All `(u32, u32)`s where each `u32` is smaller than `NUMBER_OF_CHARS`.
pub fn unsigned_pair_gen_var_1() -> Generator<(u32, u32)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_1,
        &random_unsigned_pair_gen_var_1,
        &special_random_unsigned_pair_gen_var_26,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_1,
        &special_random_unsigned_pair_gen_var_1,
    )
}

// All `(T, u64)`s where `T` is unsigned and the `u64` is smaller than `T::WIDTH`.
pub fn unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_3,
        &random_primitive_int_unsigned_pair_gen_var_2,
        &special_random_unsigned_pair_gen_var_2,
    )
}

// All `(T, u64)`s where `T` is unsigned and the `u64` is between 1 and `U::WIDTH`, inclusive.
pub fn unsigned_pair_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, u64)>
{
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_4::<T, U>,
        &random_primitive_int_unsigned_pair_gen_var_3::<T, U>,
        &special_random_unsigned_pair_gen_var_3::<T, U>,
    )
}

// All `(T, u64)`s where `T` is unsigned, the `T` is small, and the `u64` is between 1 and
// `U::WIDTH`, inclusive.
pub fn unsigned_pair_gen_var_5<T: PrimitiveUnsigned, U: PrimitiveInt>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_4::<T, U>,
        &random_unsigned_pair_gen_var_2::<T, U>,
        &special_random_unsigned_pair_gen_var_27::<T, U>,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is greater than 1 and no greater than
// `T::MAX`.
pub fn unsigned_pair_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned + SaturatingFrom<T>>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_5::<T, U>,
        &random_primitive_int_unsigned_pair_gen_var_4::<T, U>,
        &special_random_unsigned_pair_gen_var_4::<T, U>,
    )
}

// All `(T, T)` where `T` is unsigned and the first element is less than or equal to the second.
pub fn unsigned_pair_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_6,
        &random_primitive_int_pair_gen_var_2,
        &special_random_unsigned_pair_gen_var_5,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is greater than 1 and no greater than
// 36.
pub fn unsigned_pair_gen_var_8<T: PrimitiveUnsigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_primitive_int_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_5,
        &special_random_unsigned_pair_gen_var_6,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `T` is small, and the `U` is greater than 1 and
// no greater than 36.
pub fn unsigned_pair_gen_var_9<T: PrimitiveUnsigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_primitive_int_pair_gen_var_2,
        &random_unsigned_pair_gen_var_3,
        &special_random_unsigned_pair_gen_var_28,
    )
}

// All `(T, V)`s where `T` is unsigned, the `T` is between 2 and `max(T::MAX, U::MAX)`, inclusive,
// and the `V` is small.
pub fn unsigned_pair_gen_var_10<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, V)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_7::<T, U, V>,
        &random_unsigned_pair_gen_var_4::<T, U, V>,
        &special_random_unsigned_pair_gen_var_29::<T, U, V>,
    )
}

// All pairs of unsigned `T` where the first is divisible by the second, and the second is positive.
pub fn unsigned_pair_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_8,
        &random_unsigned_pair_gen_var_5,
        &special_random_unsigned_pair_gen_var_7,
    )
}

// All pairs of unsigned `T` and `U` where the `U` is positive.
pub fn unsigned_pair_gen_var_12<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_9,
        &random_unsigned_pair_gen_var_6,
        &special_random_unsigned_pair_gen_var_8,
    )
}

// All pairs of unsigned `T` where the second `T` is positive and the first is not divisible by the
// second.
pub fn unsigned_pair_gen_var_13<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_10,
        &random_unsigned_pair_gen_var_7,
        &special_random_unsigned_pair_gen_var_9,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `U` is small, and the `T` is not divisible by 2
// to the power of the `U`.
pub fn unsigned_pair_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_11,
        &random_primitive_int_unsigned_pair_gen_var_6,
        &special_random_unsigned_pair_gen_var_10,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `U` is small, and the `T` is divisible by 2 to
// the power of the `U`.
pub fn unsigned_pair_gen_var_15<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_12,
        &random_primitive_int_unsigned_pair_gen_var_7,
        &special_random_unsigned_pair_gen_var_11,
    )
}

// All `(T, T)` where `T` is unsigned and the first element is smaller than the second.
pub fn unsigned_pair_gen_var_16<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_13,
        &random_primitive_int_pair_gen_var_3,
        &special_random_unsigned_pair_gen_var_12,
    )
}

// All `(T, u64)` where `T` is unsigned, the u64 is no greater than `T::WIDTH`, and the `T` is less
// than 2 to the power of the `u64`.
pub fn unsigned_pair_gen_var_17<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_14,
        &random_unsigned_pair_gen_var_8,
        &special_random_unsigned_pair_gen_var_13,
    )
}

// All `(T, U)` where `T` and `U` are unsigned, the `T` and the `U` are small, and the `U` is
// positive.
pub fn unsigned_pair_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new_no_special(
        &exhaustive_unsigned_primitive_int_gen_var_1,
        &random_unsigned_pair_gen_var_9,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `T` is positive and small, and the `U` is
// greater than 1 and no greater than 36.
pub fn unsigned_pair_gen_var_19<T: PrimitiveUnsigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_int_pair_gen_var_1,
        &random_unsigned_pair_gen_var_10,
        &special_random_unsigned_pair_gen_var_30,
    )
}

// All `(T, u64)`s where `T` is unsigned, and either the `T` is 0 or the `u64` is less than
// `T::WIDTH`.
pub fn unsigned_pair_gen_var_20<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_15,
        &random_unsigned_pair_gen_var_11,
        &special_random_unsigned_pair_gen_var_31,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and positive and the `U` is small.
pub fn unsigned_pair_gen_var_21<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_int_pair_gen_var_2,
        &random_unsigned_pair_gen_var_12,
        &special_random_unsigned_pair_gen_var_14,
    )
}

// All pairs of unsigneds that each are valid inputs into `T::from_ordered_representation` for a
// float type `T`.
pub fn unsigned_pair_gen_var_22<T: PrimitiveFloat>() -> Generator<(u64, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_16::<T>,
        &random_unsigned_pair_gen_var_13::<T>,
        &special_random_unsigned_pair_gen_var_32::<T>,
    )
}

// All `(T, u64)`s where `T` is unsigned and the `u64` is between 0 and `U::WIDTH`, inclusive.
pub fn unsigned_pair_gen_var_23<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, u64)>
{
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_17::<T, U>,
        &random_primitive_int_unsigned_pair_gen_var_8::<T, U>,
        &special_random_unsigned_pair_gen_var_15::<T, U>,
    )
}

// All pairs of unsigneds where the first element is positive and the second is greater than 1.
pub fn unsigned_pair_gen_var_24<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_int_pair_gen_var_3,
        &random_unsigned_pair_gen_var_14,
        &special_random_unsigned_pair_gen_var_16,
    )
}

// All pairs of unsigned `T` and `U` where the `U` is no greater than `u64::MAX` and `T` is
// positive.
pub fn unsigned_pair_gen_var_25<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_18,
        &random_unsigned_pair_gen_var_15,
        &special_random_unsigned_pair_gen_var_18,
    )
}

// Given a float type `T`, all `(u64, u64)` that are valid raw mantissas and exponents of a value of
// type `T`.
pub fn unsigned_pair_gen_var_26<T: PrimitiveFloat>() -> Generator<(u64, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_19::<T>,
        &random_unsigned_pair_gen_var_16::<T>,
        &special_random_unsigned_pair_gen_var_19::<T>,
    )
}

// All pairs of unsigneds of the same type.
pub fn unsigned_pair_gen_var_27<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_20,
        &random_primitive_int_pair_gen_var_1,
        &special_random_unsigned_pair_gen_var_35,
    )
}

// All `(T, U)` where `T` and `U` are unsigned and both the `T` and the `U` are small.
pub fn unsigned_pair_gen_var_28<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen,
        &random_unsigned_pair_gen_var_17,
    )
}

// All `(T, u64)` where `T` is unsigned, both the `T` and the `u64` are small, and the `T` raised
// the power of the `u64` does not overflow.
pub fn unsigned_pair_gen_var_29<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_21,
        &random_unsigned_pair_gen_var_18,
    )
}

// All `(T, u64)` where `T` is unsigned and the `u64` is no greater than the number of leading zeros
// of the `T`.
pub fn unsigned_pair_gen_var_30<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_22,
        &random_unsigned_pair_gen_var_19,
        &special_random_unsigned_pair_gen_var_20,
    )
}

// All pairs of unsigned `T` where the two highest bits of the first `T` are not both zero.
pub fn unsigned_pair_gen_var_31<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_primitive_int_unsigned_pair_gen_var_2::<T, T>,
        &random_unsigned_primitive_int_pair_gen_var_1::<T, T>,
        &special_random_unsigned_pair_gen_var_33::<T, T>,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is small and positive.
pub fn unsigned_pair_gen_var_32<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_primitive_int_gen_var_3,
        &random_primitive_int_unsigned_pair_gen_var_9,
        &special_random_unsigned_pair_gen_var_21,
    )
}

// All `(T, T)`s where `T` is unsigned. When the generation mode is random or special random, the
// pairs are selected from a distribution such that their product is likely to be representable.
pub fn unsigned_pair_gen_var_33<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_20,
        &random_unsigned_pair_gen_var_21,
        &special_random_unsigned_pair_gen_var_22,
    )
}

// All `(T, T)`s where the LCM of the two numbers is representable.
pub fn unsigned_pair_gen_var_34<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_23,
        &random_unsigned_pair_gen_var_22,
        &special_random_unsigned_pair_gen_var_23,
    )
}

// All pairs of unsigneds where the highest bit of the first element is set.
pub fn unsigned_pair_gen_var_35<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_24,
        &random_unsigned_pair_gen_var_23,
        &special_random_unsigned_pair_gen_var_24,
    )
}

// All pairs of unsigneds that are valid inputs to `limbs_precompute_mod_mul_two_limbs`.
pub fn unsigned_pair_gen_var_36<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_25,
        &random_unsigned_pair_gen_var_24,
        &special_random_unsigned_pair_gen_var_25,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `T` is positive, and the `U` is small.
pub fn unsigned_pair_gen_var_37<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_primitive_int_unsigned_pair_gen_var_1,
        &random_unsigned_pair_gen_var_25,
        &special_random_unsigned_pair_gen_var_34,
    )
}

// All `(T, T)` where `T` is unsigned, both elements are nonzero, and the first element is smaller
// than the second.
pub fn unsigned_pair_gen_var_38<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_primitive_int_pair_gen_var_4,
        &random_unsigned_pair_gen_var_26,
        &special_random_unsigned_pair_gen_var_36,
    )
}

// All `(T, u64)` where `T` is unsigned, the u64 is no greater than `T::WIDTH`, the `T` is positive,
// and the `T` is less than 2 to the power of the `u64`.
pub fn unsigned_pair_gen_var_39<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_26,
        &random_unsigned_pair_gen_var_27,
        &special_random_unsigned_pair_gen_var_37,
    )
}

// All pairs of unsigneds where the second element is odd.
pub fn unsigned_pair_gen_var_40<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_27,
        &random_unsigned_pair_gen_var_28,
        &special_random_unsigned_pair_gen_var_38,
    )
}

// All coprime pairs of unsigneds where both elements are odd.
pub fn unsigned_pair_gen_var_41<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_28,
        &random_unsigned_pair_gen_var_29,
        &special_random_unsigned_pair_gen_var_39,
    )
}

// All coprime pairs of unsigneds of the same type.
pub fn unsigned_pair_gen_var_42<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_29,
        &random_unsigned_pair_gen_var_30,
        &special_random_unsigned_pair_gen_var_40,
    )
}

// All pairs `u64`s that are valid inputs to `T::multifactorial`.
pub fn unsigned_pair_gen_var_43<T: PrimitiveUnsigned>() -> Generator<(u64, u64)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_30::<T>,
        &random_unsigned_pair_gen_var_31::<T>,
    )
}

// All `(T, T)` where the `T`s are small, unsigned, and valid inputs to `T::binomial_coefficient`.
pub fn unsigned_pair_gen_var_44<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_31,
        &random_unsigned_pair_gen_var_32,
    )
}

// vars 45 through 49 are in malachite-nz

// All pairs of unsigneds of the same type, where each value is greater than 1.
pub fn unsigned_pair_gen_var_50<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_primitive_int_unsigned_pair_gen_var_3,
        &random_unsigned_pair_gen_var_38,
        &special_random_unsigned_pair_gen_var_41,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, bool) --

// All `(T, u64, `bool) where `T` is unsigned and either the `bool` is false or the `u64` is smaller
// than `T::WIDTH`.
pub fn unsigned_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, u64, bool)>
{
    Generator::new(
        &exhaustive_unsigned_unsigned_bool_triple_gen_var_1,
        &random_primitive_int_unsigned_bool_triple_gen_var_1,
        &special_random_unsigned_unsigned_bool_triple_gen_var_1,
    )
}

// All `(T, U, bool)` where `T` and `U` are unsigned and the `U` is positive.
pub fn unsigned_unsigned_bool_triple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, bool)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_bool_triple_gen_var_2,
        &random_primitive_int_unsigned_bool_triple_gen_var_3,
        &special_random_unsigned_unsigned_bool_triple_gen_var_2,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

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

// All `(T, u64, V)` where `T` is unsigned, the `u64` is between 1 and `U::WIDTH`, inclusive, and
// `V` is unsigned and the `V` is small.
pub fn unsigned_triple_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveInt, V: PrimitiveUnsigned>(
) -> Generator<(T, u64, V)> {
    Generator::new(
        &exhaustive_unsigned_primitive_int_unsigned_triple_gen_var_3::<T, U, V>,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_1::<T, U, V>,
        &special_random_unsigned_triple_gen_var_3::<T, U, V>,
    )
}

// All `(T, T, U)` where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_triple_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, U)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_3,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_4,
    )
}

// All `(T, U, U)` where `T` and `U` are unsigned, both `U`s are small, and the first `U` is less
// than or equal to the second.
pub fn unsigned_triple_gen_var_5<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_4,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_2,
        &special_random_unsigned_triple_gen_var_5,
    )
}

// All `(T, U, V)`s where `T`, `U`, and `V` are unsigned, the `U` is greater than 1 and no greater
// than 36, and the `V` is small.
pub fn unsigned_triple_gen_var_6<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_5,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_6,
    )
}

// All triples of unsigneds (x, y, m) where x is equal to y mod m.
pub fn unsigned_triple_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_6,
        &random_unsigned_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_7,
    )
}

// All triples of unsigneds (x, y, m) where x is not equal to y mod m.
pub fn unsigned_triple_gen_var_8<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_7,
        &random_primitive_int_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_8,
    )
}

// All triples `(T, T, u64)` (x, y, k) where `T` is unsigned and x is equal to y mod $2^k$.
pub fn unsigned_triple_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_8,
        &random_unsigned_triple_gen_var_4,
        &special_random_unsigned_triple_gen_var_9,
    )
}

// All triples `(T, T, u64)` (x, y, k) where `T` is unsigned and x is not equal to y mod $2^k$.
pub fn unsigned_triple_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_9,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_10,
    )
}

// All `(T, T, u64)` where `T` is unsigned, the u64 is no greater than `T::WIDTH`, and both `T`s are
// less than 2 to the power of the `u64`.
pub fn unsigned_triple_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_10,
        &random_unsigned_triple_gen_var_5,
        &special_random_unsigned_triple_gen_var_11,
    )
}

// All `(T, T, T)` where `T` is unsigned and the first and second elements are less than the third.
pub fn unsigned_triple_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_11,
        &random_primitive_int_triple_gen_var_2,
        &special_random_unsigned_triple_gen_var_12,
    )
}

// All `(T, U, U)` where `T` and `U` are unsigned and the `U`s are small.
pub fn unsigned_triple_gen_var_13<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_12,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_4,
        &special_random_unsigned_triple_gen_var_13,
    )
}

// All `(T, U, T)` where `T` and `U` are unsigned, the `U` is small, and the first element is less
// than the third.
pub fn unsigned_triple_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_13,
        &random_primitive_int_unsigned_primitive_int_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_14,
    )
}

// All `(T, U, T)` where `T` and `U` are unsigned and the first element is less than the third.
pub fn unsigned_triple_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_14,
        &random_primitive_int_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_15,
    )
}

// All `(T, U, u64)` where `T` and `U` are unsigned, the u64 is no greater than `T::WIDTH`, and the
// `T`s is less than 2 to the power of the `u64`.
pub fn unsigned_triple_gen_var_16<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_15,
        &random_unsigned_primitive_int_unsigned_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_16,
    )
}

// All `(T, U, u64)` where `T` and `U` are unsigned, the `U` is small, the u64 is no greater than
// `T::WIDTH`, and the `T`s is less than 2 to the power of the `u64`.
pub fn unsigned_triple_gen_var_17<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_15,
        &random_unsigned_triple_gen_var_6,
        &special_random_unsigned_triple_gen_var_17,
    )
}

// All `(T, U, T)` where `T` and `U` are unsigned, the `U` is no larger than `u64::MAX`, and the
// first element is less than the third.
pub fn unsigned_triple_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_16,
        &random_primitive_int_unsigned_primitive_int_triple_gen_var_2,
        &special_random_unsigned_triple_gen_var_18,
    )
}

// All triples of unsigneds of the same type.
pub fn unsigned_triple_gen_var_19<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_17,
        &random_primitive_int_triple_gen_var_4,
        &special_random_unsigned_triple_gen_var_19,
    )
}

// All `(T, U, U)` where `T` and `U` are unsigned, both `U`s are small, the `T` is positive, and the
// first `U` is less than or equal to the second.
pub fn unsigned_triple_gen_var_20<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U)> {
    Generator::new(
        &exhaustive_primitive_int_unsigned_unsigned_triple_gen_var_1,
        &random_unsigned_triple_gen_var_7,
        &special_random_unsigned_triple_gen_var_20,
    )
}

// All triples of unsigneds where the second element is odd.
pub fn unsigned_triple_gen_var_21<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_18,
        &random_unsigned_triple_gen_var_8,
        &special_random_unsigned_triple_gen_var_21,
    )
}

// All triples of unsigneds where the third element is odd.
pub fn unsigned_triple_gen_var_22<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_19,
        &random_unsigned_triple_gen_var_9,
        &special_random_unsigned_triple_gen_var_22,
    )
}

// All triples of unsigneds where the second and third elements are odd.
pub fn unsigned_triple_gen_var_23<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_20,
        &random_unsigned_triple_gen_var_10,
        &special_random_unsigned_triple_gen_var_23,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(T, u64, u64, U)` where `T` and `U` are unsigned, both `u64`s are small, and the four values
// are valid arguments to `assign_bits`.
pub fn unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, u64, u64, U)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_1,
        &random_primitive_int_unsigned_unsigned_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_1,
    )
}

// All `(T, T, T, U)` where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_quadruple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, T, U)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_2,
        &random_primitive_int_primitive_int_primitive_int_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_2,
    )
}

// All `(T, T, T, u64)` where `T` is unsigned, the u64 is no greater than `T::WIDTH`, and all three
// `T`s are less than 2 to the power of the `u64`.
pub fn unsigned_quadruple_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(T, T, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_3,
        &random_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_3,
    )
}

// All `(T, T, T, T)` where `T` is unsigned and the first three elements are each less than the
// fourth.
pub fn unsigned_quadruple_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_4,
        &random_primitive_int_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_4,
    )
}

// All `(T, T, T, T)` that are valid inputs to `limbs_mod_preinverted`.
pub fn unsigned_quadruple_gen_var_5<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_5::<T, DT>,
        &random_unsigned_quadruple_gen_var_2::<T, DT>,
        &special_random_unsigned_quadruple_gen_var_5::<T, DT>,
    )
}

// All `(T, T, U, T)` where `T` and `U` are unsigned and the first two elements are each less than
// the fourth.
pub fn unsigned_quadruple_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, U, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_6,
        &random_primitive_int_quadruple_gen_var_2,
        &special_random_unsigned_quadruple_gen_var_6,
    )
}

// All `(T, U, U, T)` where `T` and `U` are unsigned and the first element is less than the fourth.
pub fn unsigned_quadruple_gen_var_7<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_7,
        &random_primitive_int_quadruple_gen_var_3,
        &special_random_unsigned_quadruple_gen_var_7,
    )
}

// All `(T, T, U, u64)` where `T` and `U` are unsigned, the u64 is no greater than `T::WIDTH`, and
// both `T`s are less than 2 to the power of the `u64`.
pub fn unsigned_quadruple_gen_var_8<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, U, u64)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_8,
        &random_unsigned_unsigned_primitive_int_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_8,
    )
}

// All `(T, U, U, u64)` where `T` and `U` are unsigned, the u64 is no greater than `T::WIDTH`, and
// the `T`s is less than 2 to the power of the `u64`.
pub fn unsigned_quadruple_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U, u64)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_9,
        &random_unsigned_primitive_int_primitive_int_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_9,
    )
}

// All quadruples of unsigneds of the same type.
pub fn unsigned_quadruple_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_10,
        &random_primitive_int_quadruple_gen_var_4,
        &special_random_unsigned_quadruple_gen_var_10,
    )
}

// All quadruples of unsigneds which, if grouped into two double-width unsigneds N and D (high
// halves first) satisfy D >= 2^W, N >= D, and N / D < 2^W.
pub fn unsigned_quadruple_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_11,
        &random_primitive_int_quadruple_gen_var_5,
        &special_random_unsigned_quadruple_gen_var_11,
    )
}

// All quadruples of unsigneds where the fourth element is odd.
pub fn unsigned_quadruple_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_12,
        &random_unsigned_quadruple_gen_var_3,
        &special_random_unsigned_quadruple_gen_var_12,
    )
}

// -- (PrimitiveUnsigned * 6) --

// All sextuples of unsigneds of the same type.
pub fn unsigned_sextuple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_sextuple_gen_var_1,
        &random_primitive_int_sextuple_gen_var_1,
        &special_random_unsigned_sextuple_gen_var_1,
    )
}

// var 2 is in malachite-nz.

// -- (PrimitiveUnsigned * 8) --

// All octuples of unsigneds of the same type.
#[allow(clippy::type_complexity)]
pub fn unsigned_octuple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T, T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_octuple_gen_var_1,
        &random_primitive_int_octuple_gen_var_1,
        &special_random_unsigned_octuple_gen_var_1,
    )
}

// -- (PrimitiveUnsigned * 9) --

// All nonuples of unsigneds of the same type.
#[allow(clippy::type_complexity)]
pub fn unsigned_nonuple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T, T, T, T, T, T)>
{
    Generator::new(
        &exhaustive_unsigned_nonuple_gen_var_1,
        &random_primitive_int_nonuple_gen_var_1,
        &special_random_unsigned_nonuple_gen_var_1,
    )
}

// -- (PrimitiveUnsigned * 12) --

// All duodecuples of unsigneds of the same type.
#[allow(clippy::type_complexity)]
pub fn unsigned_duodecuple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(T, T, T, T, T, T, T, T, T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_duodecuple_gen_var_1,
        &random_primitive_int_duodecuple_gen_var_1,
        &special_random_unsigned_duodecuple_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// All `(T, T, RoundingMode)` that are valid inputs to `T::div_round`.
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(T, T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_1,
        &random_unsigned_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// All `(T, T, RoundingMode)` where `T` is unsigned and the triple is a valid input to
// `T::round_to_multiple`.
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(T, T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_3,
        &random_unsigned_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// All `(T, u64, RoundingMode)` where `T` is unsigned and the triple is a valid input to
// `T::round_to_multiple_of_power_of_2`.
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> Generator<(T, u64, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_4,
        &random_primitive_int_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_3,
    )
}

// All `(T, U, RoundingMode)` where `T` is unsigned, `U` is unsigned, and the triple is a valid
// input to `T::shr_round`.
pub fn unsigned_unsigned_rounding_mode_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(T, U, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_5,
        &random_primitive_int_unsigned_rounding_mode_triple_gen_var_2,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_4,
    )
}

// var 5 is in malachite-float.

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

// All `(T, u64, Vec<bool>)` where `T` is unsigned, the `u64` is between 1 and `U::WIDTH`,
// inclusive, and the `Vec` has as many elements as the `T` has digits in base $2^\ell$, where
// $\ell$ is the `u64`.
pub fn unsigned_unsigned_bool_vec_triple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> Generator<(T, u64, Vec<bool>)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_bool_vec_triple_gen_var_1::<T, U>,
        &random_primitive_int_unsigned_bool_vec_triple_gen_var_1::<T, U>,
        &special_random_unsigned_unsigned_bool_vec_triple_gen_var_1::<T, U>,
    )
}

// -- (PrimitiveUnsigned, RoundingMode) --

pub fn unsigned_rounding_mode_pair_gen<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_rounding_mode_pair_gen,
        &random_primitive_int_rounding_mode_pair_gen,
        &special_random_unsigned_rounding_mode_pair_gen,
    )
}

// All `(T, RoundingMode)`s where `T` is unsigned and the `T` is positive.
pub fn unsigned_rounding_mode_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)>
{
    Generator::new(
        &exhaustive_primitive_int_rounding_mode_pair_gen_var_1,
        &random_unsigned_rounding_mode_pair_gen_var_1,
        &special_random_unsigned_rounding_mode_pair_gen_var_1,
    )
}

// All `(T, RoundingMode)` where `T` is unsigned, `U` is a primitive float type, and the pair is a
// valid input to `U::rounding_from`.
pub fn unsigned_rounding_mode_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_rounding_mode_pair_gen_var_1::<T, U>,
        &random_primitive_int_rounding_mode_pair_gen_var_1::<T, U>,
        &special_random_unsigned_rounding_mode_pair_gen_var_2::<T, U>,
    )
}

// All `(T, RoundingMode)`s where `T` is small, unsigned, and positive.
pub fn unsigned_rounding_mode_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)>
{
    Generator::new_no_special(
        &exhaustive_primitive_int_rounding_mode_pair_gen_var_1,
        &random_unsigned_rounding_mode_pair_gen_var_2,
    )
}

// All `(T, RoundingMode)`s where `T` is small, unsigned, and positive, and the rounding mode is not
// `Exact`.
pub fn unsigned_rounding_mode_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)>
{
    Generator::new_no_special(
        &exhaustive_primitive_int_rounding_mode_pair_gen_var_2,
        &random_unsigned_rounding_mode_pair_gen_var_3,
    )
}

// All `(T, RoundingMode)`s where `T` is small and unsigned.
pub fn unsigned_rounding_mode_pair_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)>
{
    Generator::new_no_special(
        &exhaustive_unsigned_rounding_mode_pair_gen,
        &random_unsigned_rounding_mode_pair_gen_var_4,
    )
}

// -- (PrimitiveUnsigned, String) --

// All `(u8, String)` that, when passed to `Natural::from_string_base`, return a `Some`.
pub fn unsigned_string_pair_gen_var_1() -> Generator<(u8, String)> {
    Generator::new_no_special(
        &exhaustive_unsigned_string_pair_gen_var_1,
        &random_unsigned_string_pair_gen_var_1,
    )
}

// All `(u8, String)` that are valid inputs to `Natural::from_string_base` or
// `Integer::from_string_base`, regardless of whether it returns `Some` or `None`.
pub fn unsigned_string_pair_gen_var_2() -> Generator<(u8, String)> {
    Generator::new(
        &exhaustive_unsigned_string_pair_gen_var_2,
        &random_unsigned_string_pair_gen_var_2,
        &special_random_unsigned_string_pair_gen_var_1,
    )
}

// All `(u8, String)` that, when passed to `Integer::from_string_base`, return a `Some`.
pub fn unsigned_string_pair_gen_var_3() -> Generator<(u8, String)> {
    Generator::new_no_special(
        &exhaustive_unsigned_string_pair_gen_var_3,
        &random_unsigned_string_pair_gen_var_3,
    )
}

// -- (PrimitiveUnsigned, ToSciOptions) --

pub fn unsigned_to_sci_options_pair_gen<T: PrimitiveUnsigned>() -> Generator<(T, ToSciOptions)> {
    Generator::new(
        &exhaustive_unsigned_to_sci_options_pair_gen,
        &random_primitive_int_to_sci_options_pair_gen,
        &special_random_unsigned_to_sci_options_pair_gen,
    )
}

type TSO = ToSciOptions;
// All `(T, ToSciOptions)` pairs where `T` is unsigned and the `T` can be formatted using the
// options.
pub fn unsigned_to_sci_options_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, TSO)> {
    Generator::new(
        &exhaustive_unsigned_to_sci_options_pair_gen_var_1,
        &random_primitive_int_to_sci_options_pair_gen_var_1,
        &special_random_unsigned_to_sci_options_pair_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, Vec<bool>) --

// All `(T, Vec<bool>)` where `T` is unsigned and the `Vec` has as many elements as the `T` has
// significant bits.
pub fn unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, Vec<bool>)> {
    Generator::new(
        &exhaustive_unsigned_bool_vec_pair_gen_var_1,
        &random_unsigned_bool_vec_pair_gen_var_1,
        &special_random_unsigned_bool_vec_pair_gen_var_1,
    )
}

// -- RationalSequence<PrimitiveUnsigned> --

pub fn unsigned_rational_sequence_gen<T: PrimitiveUnsigned>() -> Generator<RationalSequence<T>> {
    Generator::new(
        &exhaustive_unsigned_rational_sequence_gen,
        &random_primitive_int_rational_sequence_gen,
        &special_random_unsigned_rational_sequence_gen,
    )
}

// -- (RationalSequence<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(RationalSequence<T>, U)` pairs where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_rational_sequence_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(RationalSequence<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_rational_sequence_unsigned_pair_gen_var_1,
        &random_primitive_int_rational_sequence_unsigned_pair_gen_var_1,
        &special_random_unsigned_rational_sequence_unsigned_pair_gen_var_1,
    )
}

// All `(RationalSequence<T>, usize)` pairs where `T` is unsigned and the `usize` is less than the
// length of the `RationalSequence`.
pub fn unsigned_rational_sequence_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(RationalSequence<T>, usize)> {
    Generator::new(
        &exhaustive_unsigned_rational_sequence_unsigned_pair_gen_var_2,
        &random_primitive_int_rational_sequence_unsigned_pair_gen_var_2,
        &special_random_unsigned_rational_sequence_unsigned_pair_gen_var_2,
    )
}

// -- (RationalSequence<PrimitiveUnsigned>, RationalSequence<PrimitiveUnsigned>) --

pub fn unsigned_rational_sequence_pair_gen<T: PrimitiveUnsigned>(
) -> Generator<(RationalSequence<T>, RationalSequence<T>)> {
    Generator::new(
        &exhaustive_unsigned_rational_sequence_pair_gen,
        &random_primitive_int_rational_sequence_pair_gen,
        &special_random_unsigned_rational_sequence_pair_gen,
    )
}

// -- RationalSequence<PrimitiveUnsigned> * 3 --

pub fn unsigned_rational_sequence_triple_gen<T: PrimitiveUnsigned>() -> Generator<(
    RationalSequence<T>,
    RationalSequence<T>,
    RationalSequence<T>,
)> {
    Generator::new(
        &exhaustive_unsigned_rational_sequence_triple_gen,
        &random_primitive_int_rational_sequence_triple_gen,
        &special_random_unsigned_rational_sequence_triple_gen,
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

// -- SciSizeOptions --

pub fn sci_size_options_gen() -> Generator<SciSizeOptions> {
    Generator::new_no_special(
        &exhaustive_sci_size_options_gen,
        &random_sci_size_options_gen,
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

// All nonempty `String`s containing only the characters '0' through '9'.
pub fn string_gen_var_3() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_3, &random_string_gen_var_3)
}

// All nonempty `String`s containing only the characters '0' through '9', except for a possible '-'
// in the first position.
pub fn string_gen_var_4() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_4, &random_string_gen_var_4)
}

// All nonempty `String`s containing only the characters '0' and '1'.
pub fn string_gen_var_5() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_5, &random_string_gen_var_5)
}

// All nonempty `String`s containing only the characters '0' through '7'.
pub fn string_gen_var_6() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_6, &random_string_gen_var_6)
}

// All nonempty `String`s containing only the characters '0' through '9', 'a' through 'f', or 'A'
// through 'F'.
pub fn string_gen_var_7() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_7, &random_string_gen_var_7)
}

// All `String`s of the form `"\"0xD\""`, where D is a nonempty substring containing only the
// characters '0' through '9', 'a' through 'f', or 'A' through 'F'.
pub fn string_gen_var_8() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_8, &random_string_gen_var_8)
}

// All `String`s of the form `"\"0xD\""` or `"\"-0xD\""`, where D is a nonempty substring containing
// only the characters '0' through '9', 'a' through 'f', or 'A' through 'F'.
pub fn string_gen_var_9() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_9, &random_string_gen_var_9)
}

// All `String`s containing only characters that appear in the `String` representations of
// `NiceFloat`s.
pub fn string_gen_var_10() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_10, &random_string_gen_var_10)
}

// vars 11 through 12 are in malachite-q.

// All `String`s containing only the characters `+-.0123456789Ee`.
pub fn string_gen_var_13() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_13, &random_string_gen_var_13)
}

fn large_exponent(s: &str) -> bool {
    let mut i = 0;
    let mut expect_e = false;
    for c in s.chars().rev() {
        if expect_e {
            return c == 'e' || c == 'E';
        } else if c.is_ascii_digit() {
            i += 1;
        } else if i <= 3 {
            return false;
        } else if c == 'e' || c == 'E' {
            return true;
        } else if c == '+' || c == '-' {
            expect_e = true;
        }
    }
    false
}

// All `Strings` that do not end in an 'e' or 'E' followed by an optional plus or minus sign and
// more than three digits.
pub fn string_gen_var_14() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_14,
        &random_string_gen_var_14,
        &special_random_string_gen_var_4,
    )
}

// All `String`s containing only the characters `+-.0123456789Ee`, and that do not end in an 'e' or
// 'E' followed by an optional plus or minus sign and more than three digits.
pub fn string_gen_var_15() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_15, &random_string_gen_var_15)
}

// -- (String, FromSciStringOptions) --

pub fn string_from_sci_string_options_pair_gen() -> Generator<(String, FromSciStringOptions)> {
    Generator::new(
        &exhaustive_string_from_sci_string_options_pair_gen,
        &random_string_from_sci_string_options_pair_gen,
        &special_random_string_from_sci_string_options_pair_gen,
    )
}

// All `(String, FromSciStringOptions)`, where the `String` only contains characters that occur in
// valid inputs to `T::from_sci_string_options`, using the specified options.
pub fn string_from_sci_string_options_pair_gen_var_1() -> Generator<(String, FromSciStringOptions)>
{
    Generator::new_no_special(
        &exhaustive_string_from_sci_string_options_pair_gen_var_1,
        &random_string_from_sci_string_options_pair_gen_var_1,
    )
}

// All `(String, FromSciStringOptions)`, where the string does not end in an 'e' or 'E' followed by
// an optional plus or minus sign and more than three digits.
pub fn string_from_sci_string_options_pair_gen_var_2() -> Generator<(String, FromSciStringOptions)>
{
    Generator::new(
        &exhaustive_string_from_sci_string_options_pair_gen_var_2,
        &random_string_from_sci_string_options_pair_gen_var_2,
        &special_random_string_from_sci_string_options_pair_gen_var_1,
    )
}

// All `(String, FromSciStringOptions)`, where the `String` only contains characters that occur in
// valid inputs to `T::from_sci_string_options`, using the specified options, and the string does
// not end in an 'e' or 'E' followed by an optional plus or minus sign and more than three digits.
pub fn string_from_sci_string_options_pair_gen_var_3() -> Generator<(String, FromSciStringOptions)>
{
    Generator::new_no_special(
        &exhaustive_string_from_sci_string_options_pair_gen_var_3,
        &random_string_from_sci_string_options_pair_gen_var_3,
    )
}

// -- (String, PrimitiveUnsigned) --

// All `(String, u8)`s where the `u8` is between 2 and 36, inclusive, and the string does not end in
// an 'e' or 'E' followed by an optional plus or minus sign and more than three digits.
pub fn string_unsigned_pair_gen_var_1() -> Generator<(String, u8)> {
    Generator::new(
        &exhaustive_string_unsigned_pair_gen_var_1,
        &random_string_unsigned_pair_gen_var_1,
        &special_random_string_unsigned_pair_gen_var_1,
    )
}

// All `(String, u8)`s where the `u8` is between 2 and 36, inclusive, the `String` only contains
// characters that occur in valid inputs to `T::from_sci_string_options` with the specified base,
// and the string does not end in an 'e' or 'E' followed by an optional plus or minus sign and more
// than three digits.
pub fn string_unsigned_pair_gen_var_2() -> Generator<(String, u8)> {
    Generator::new_no_special(
        &exhaustive_string_unsigned_pair_gen_var_2,
        &random_string_unsigned_pair_gen_var_2,
    )
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

// -- ToSciOptions --

pub fn to_sci_options_gen() -> Generator<ToSciOptions> {
    Generator::new_no_special(&exhaustive_to_sci_options_gen, &random_to_sci_options_gen)
}

// -- (ToSciOptions, bool) --

pub fn to_sci_options_bool_pair_gen() -> Generator<(ToSciOptions, bool)> {
    Generator::new_no_special(
        &exhaustive_to_sci_options_bool_pair_gen,
        &random_to_sci_options_bool_pair_gen,
    )
}

// -- (ToSciOptions, PrimitiveSigned) --

// All `(ToSciOptions, T)` where `T` is signed and the `T` is small and negative.
pub fn to_sci_options_signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(ToSciOptions, T)> {
    Generator::new_no_special(
        &exhaustive_to_sci_options_signed_pair_gen_var_1,
        &random_to_sci_options_signed_pair_gen_var_1,
    )
}

// -- (ToSciOptions, PrimitiveUnsigned) --

// All `(ToSciOptions, T)` where `T` is unsigned and the `T` is between 2 and 36, inclusive.
pub fn to_sci_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(TSO, T)> {
    Generator::new(
        &exhaustive_to_sci_options_unsigned_pair_gen_var_1,
        &random_to_sci_options_unsigned_pair_gen_var_1,
        &special_random_to_sci_options_unsigned_pair_gen_var_1,
    )
}

// All `(ToSciOptions, T)` where `T` is unsigned and the `T` is small.
pub fn to_sci_options_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(TSO, T)> {
    Generator::new_no_special(
        &exhaustive_to_sci_options_unsigned_pair_gen_var_2,
        &random_to_sci_options_unsigned_pair_gen_var_2,
    )
}

// All `(ToSciOptions, T)` where `T` is unsigned and the `T` is small and positive.
pub fn to_sci_options_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(TSO, T)> {
    Generator::new_no_special(
        &exhaustive_to_sci_options_primitive_int_pair_gen_var_1,
        &random_to_sci_options_unsigned_pair_gen_var_3,
    )
}

// -- (ToSciOptions, RoundingMode) --

pub fn to_sci_options_rounding_mode_pair_gen() -> Generator<(ToSciOptions, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_to_sci_options_rounding_mode_pair_gen,
        &random_to_sci_options_rounding_mode_pair_gen,
    )
}

// -- Vec<bool> --

pub fn bool_vec_gen() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen,
        &random_bool_vec_gen,
        &special_random_bool_vec_gen,
    )
}

// All `Vec<bool>`s that could be the bits, in ascending order, of an unsigned value of type `T`.
// The `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_1<T: PrimitiveUnsigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_1::<T>,
        &random_bool_vec_gen_var_1::<T>,
        &special_random_bool_vec_gen_var_1::<T>,
    )
}

// All `Vec<bool>`s that could be the bits, in ascending order, of a signed value of type `T`. The
// `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_2<T: PrimitiveSigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_2::<T>,
        &random_bool_vec_gen_var_2::<T>,
        &special_random_bool_vec_gen_var_2::<T>,
    )
}

// All `Vec<bool>`s that could be the bits, in descending order, of an unsigned value of type `T`.
// The `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_3<T: PrimitiveUnsigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_3::<T>,
        &random_bool_vec_gen_var_3::<T>,
        &special_random_bool_vec_gen_var_3::<T>,
    )
}

// All `Vec<bool>`s that could be the bits, in descending order, of a signed value of type `T`. The
// `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_4<T: PrimitiveSigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_4::<T>,
        &random_bool_vec_gen_var_4::<T>,
        &special_random_bool_vec_gen_var_4::<T>,
    )
}

// All `Vec<bool>`s that contain at least one `true`.
pub fn bool_vec_gen_var_5() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_5,
        &random_bool_vec_gen_var_5,
        &special_random_bool_vec_gen_var_5,
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

// All nonempty `Vec`s of unsigneds whose last element is not zero.
pub fn unsigned_vec_gen_var_1<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen_var_1,
        &random_primitive_int_vec_gen_var_1,
        &special_random_unsigned_vec_gen_var_1,
    )
}

// All `Vec`s of unsigneds that contain at least one nonzero value.
pub fn unsigned_vec_gen_var_2<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen_var_2,
        &random_primitive_int_vec_gen_var_2,
        &special_random_unsigned_vec_gen_var_2,
    )
}

// All nonempty `Vec`s of unsigneds that do not end in a nonzero value.
pub fn unsigned_vec_gen_var_3<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen_var_3,
        &random_primitive_int_vec_gen_var_3,
        &special_random_unsigned_vec_gen_var_3,
    )
}

// All nonempty `Vec`s of unsigneds.
pub fn unsigned_vec_gen_var_4<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen_var_4,
        &random_primitive_int_vec_gen_var_4,
        &special_random_unsigned_vec_gen_var_4,
    )
}

// var 5 is in malachite-nz.

// All `Vec`s of unsigneds with lengths at least 2.
pub fn unsigned_vec_gen_var_6<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen_var_6,
        &random_primitive_int_vec_gen_var_5,
        &special_random_unsigned_vec_gen_var_6,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn unsigned_vec_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen,
        &random_primitive_int_vec_primitive_int_pair_gen,
        &special_random_unsigned_vec_unsigned_pair_gen,
    )
}

// All `(Vec<T>, usize)` where `T` is unsigned and the `usize` is less than or equal to the length
// of the `Vec`.
pub fn unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, usize)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_1,
        &random_primitive_int_vec_unsigned_pair_gen_var_1,
        &special_random_unsigned_vec_unsigned_pair_gen_var_3,
    )
}

// All `(Vec<U>, u64)` such that the flipped `(u64, Vec<U>)` is a `Some`-returning input to
// `from_power_of_2_digits_asc<T, U>`, where the `Vec` is no longer than the number of digits of
// `T::MAX` in the base 2 to the power of the `u64`.
pub fn unsigned_vec_unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<U>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_2::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
    )
}

// All `(Vec<U>, u64)` such that the flipped `(u64, Vec<U>)` is a `Some`-returning input to
// `from_power_of_2_digits_desc<T, U>`, where the `Vec` is no longer than the number of digits of
// `T::MAX` in the base 2 to the power of the `u64`.
pub fn unsigned_vec_unsigned_pair_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<U>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_3::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_2::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_2::<T, U>,
    )
}

// var 4 is in malachite-nz

// All `(Vec<T>, U)` that are valid, `Some`-returning inputs to _from_digits_desc_basecase in
// malachite-nz.
pub fn unsigned_vec_unsigned_pair_gen_var_5<
    T: ExactFrom<U> + PrimitiveUnsigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>,
>() -> Generator<(Vec<T>, U)> {
    Generator::new_no_special(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_5::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_3::<T, U>,
    )
}

// All `(Vec<T>, u64)` such that the flipped `(u64, Vec<T>)` is a `Some`-returning input to
// `from_power_of_2_digits_asc<T, U>` or `from_power_of_2_digits_desc<T, U>`, regardless of whether
// the returned value is `Some` or `None`.
pub fn unsigned_vec_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_6::<T>,
        &random_primitive_int_vec_unsigned_pair_gen_var_2::<T>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_5::<T>,
    )
}

// All `(T, Vec<T>)` that are a valid input to `U::from_digits_desc`, where the `Vec` is no longer
// than the number of digits of `U::MAX` in the base of the `T`.
pub fn unsigned_vec_unsigned_pair_gen_var_7<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> Generator<(Vec<T>, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_7::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_4::<T, U>,
    )
}

// All `(Vec<T>, T)` that are is a valid input to `U::from_digits_asc`, where the `Vec` is no longer
// than the number of digits of `U::MAX` in the base of the `T`.
pub fn unsigned_vec_unsigned_pair_gen_var_8<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> Generator<(Vec<T>, T)> {
    Generator::new_no_special(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_8::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_5::<T, U>,
    )
}

// All `(Vec<T>, T)` such that the flipped `(u64, Vec<T>)` is a `Some`-returning input to
// `from_digits_asc<T, U>` or `from_digits_desc<T, U>`, regardless of whether the returned value is
// `Some` or `None`.
pub fn unsigned_vec_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_9::<T>,
        &random_unsigned_vec_unsigned_pair_gen_var_6::<T>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_6::<T>,
    )
}

// All `(Vec<T>, u64)`, where the `u64` is between 1 and `T::WIDTH`, inclusive, and each `T` in the
// `Vec` is less than 2 to the power of the `u64`.
pub fn unsigned_vec_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_10,
        &random_unsigned_vec_unsigned_pair_gen_var_7,
        &special_random_unsigned_vec_unsigned_pair_gen_var_7,
    )
}

// All `(Vec<T>, u64)`, where the `u64` is between 1 and `T::WIDTH`, inclusive.
pub fn unsigned_vec_unsigned_pair_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_11,
        &random_primitive_int_vec_unsigned_pair_gen_var_3,
        &special_random_unsigned_vec_unsigned_pair_gen_var_8,
    )
}

// All `(Vec<T>, U)` that are valid, inputs to from_digits_desc_basecase in malachite-nz, regardless
// of whether they return `Some` or `None`.
pub fn unsigned_vec_unsigned_pair_gen_var_12<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_12::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_4::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_9::<T, U>,
    )
}

// All `(Vec<T>, U)` where the last element of the `Vec` is nonzero.
pub fn unsigned_vec_unsigned_pair_gen_var_13<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_1::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_5::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_10::<T, U>,
    )
}

// All `(Vec<T>, U)` where the last element of the `Vec` is nonzero and `U` is small and greater
// than 2.
pub fn unsigned_vec_unsigned_pair_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_2::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_6::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_11::<T, U>,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned and the `Vec` is nonempty.
pub fn unsigned_vec_unsigned_pair_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_13,
        &random_primitive_int_vec_primitive_int_pair_gen_var_1,
        &special_random_unsigned_vec_unsigned_pair_gen_var_11,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_vec_unsigned_pair_gen_var_16<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_14,
        &random_primitive_int_vec_unsigned_pair_gen_var_7,
        &special_random_unsigned_vec_unsigned_pair_gen_var_13,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned and the U is less than `T::WIDTH` times the
// length of the `Vec`.
pub fn unsigned_vec_unsigned_pair_gen_var_17<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_15,
        &random_primitive_int_vec_unsigned_pair_gen_var_8,
        &special_random_unsigned_vec_unsigned_pair_gen_var_14,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned and the `Vec` contains at least one nonzero
// value.
pub fn unsigned_vec_unsigned_pair_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_16,
        &random_primitive_int_vec_primitive_int_pair_gen_var_2,
        &special_random_unsigned_vec_unsigned_pair_gen_var_15,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` contains at least one nonzero value,
// and the `U` is positive.
pub fn unsigned_vec_unsigned_pair_gen_var_19<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_3,
        &random_primitive_int_vec_unsigned_pair_gen_var_9,
        &special_random_unsigned_vec_unsigned_pair_gen_var_16,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` contains at least one nonzero value,
// and the `U` is small.
pub fn unsigned_vec_unsigned_pair_gen_var_20<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_17,
        &random_primitive_int_vec_unsigned_pair_gen_var_10,
        &special_random_unsigned_vec_unsigned_pair_gen_var_17,
    )
}

// var 21 is in malachite-nz.

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` has at least 2 elements, and the `U`
// is positive.
pub fn unsigned_vec_unsigned_pair_gen_var_22<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_4,
        &random_primitive_int_vec_unsigned_pair_gen_var_11,
        &special_random_unsigned_vec_unsigned_pair_gen_var_19,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` has at least 2 elements and at least
// one nonzero element, and the `U` is positive.
pub fn unsigned_vec_unsigned_pair_gen_var_23<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_5,
        &random_primitive_int_vec_unsigned_pair_gen_var_12,
        &special_random_unsigned_vec_unsigned_pair_gen_var_20,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` has at least 2 elements, and the
// highest bit of the `U` is set.
pub fn unsigned_vec_unsigned_pair_gen_var_24<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_6,
        &random_primitive_int_vec_unsigned_pair_gen_var_13,
        &special_random_unsigned_vec_unsigned_pair_gen_var_21,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` has at least 2 elements, the `U` is
// positive, and the highest bit of the `U` is not set.
pub fn unsigned_vec_unsigned_pair_gen_var_25<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_7,
        &random_primitive_int_vec_unsigned_pair_gen_var_14,
        &special_random_unsigned_vec_unsigned_pair_gen_var_22,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` is nonempty, and the highest bit of
// the `U` is set.
pub fn unsigned_vec_unsigned_pair_gen_var_26<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_8,
        &random_primitive_int_vec_unsigned_pair_gen_var_15,
        &special_random_unsigned_vec_unsigned_pair_gen_var_23,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` is nonempty, the `U` is positive, and
// the highest bit of the `U` is not set.
pub fn unsigned_vec_unsigned_pair_gen_var_27<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_9,
        &random_primitive_int_vec_unsigned_pair_gen_var_16,
        &special_random_unsigned_vec_unsigned_pair_gen_var_24,
    )
}

// All `(Vec<T>, U)` where `T` and `U` are unsigned, the `Vec` is nonempty, the `U` is positive, and
// the two highest bits of the `U` are not set.
pub fn unsigned_vec_unsigned_pair_gen_var_28<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_10,
        &random_primitive_int_vec_unsigned_pair_gen_var_17,
        &special_random_unsigned_vec_unsigned_pair_gen_var_25,
    )
}

// vars 29 through 30 are in malachite-nz.

// All `(Vec<T>, U)` where the last element of the `Vec` is nonzero and `U` is small and greater
// than 1.
pub fn unsigned_vec_unsigned_pair_gen_var_31<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_primitive_int_pair_gen_var_11::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_18::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_28::<T, U>,
    )
}

// All `(Vec<T>, u64)`, where `T` is unsigned, `U` is a primitive integer type, and the `u64` is
// between 1 and `U::WIDTH` - 1, inclusive.
pub fn unsigned_vec_unsigned_pair_gen_var_32<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> Generator<(Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_21::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_19::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_29::<T, U>,
    )
}

// All `(Vec<T>, u64)`, where `T` is unsigned, `U` is a primitive integer type, the `Vec` is
// nonempty, and the `u64` is between 1 and `U::WIDTH` - 1, inclusive.
pub fn unsigned_vec_unsigned_pair_gen_var_33<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> Generator<(Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_22::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_20::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_30::<T, U>,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

type T1<T> = Generator<(Vec<T>, T, T)>;

pub fn unsigned_vec_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> T1<T> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen,
        &random_primitive_int_vec_primitive_int_primitive_int_triple_gen,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen,
    )
}

// All `(Vec<T>, U, V)` where `T`, `U`, and `V` are unsigned and the `U` is small.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, U, V)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_1,
        &random_primitive_int_vec_unsigned_primitive_int_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_1,
    )
}

// All `(Vec<T>, usize, usize)` where `T` is unsigned and the length of the `Vec` is at least the
// product of the `usize`s.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, usize, usize)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_2,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_2,
    )
}

// All `(Vec<T>, U, U)` where `T` and `U` are unsigned, both `U`s are small, and the first `U` is
// less than or equal to the second.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, U, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_3,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_3,
    )
}

// All `(Vec<T>, U, U)` where `T` and `U` are unsigned, both `U`s are small, the `Vec` contains at
// least one nonzero value, and the first `U` is less than or equal to the second.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, U, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_4,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_3,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_4,
    )
}

// All `(Vec<T>, T, T)` where `T` is unsigned, both `T`s are positive, the `Vec` contains at least
// two elements, and its last element is nonzero.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>() -> T1<T> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_5,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_4,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_5,
    )
}

// var 6 is in malachite-nz.

// All `(Vec<T>, T, T)` where `T` is unsigned, the second `T` is positive, the `Vec` contains at
// least two elements, and its last element is nonzero.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_7<T: PrimitiveUnsigned>() -> T1<T> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_7,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_5,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_7,
    )
}

// All `(Vec<T>, T, U)` where `T` and `U` are unsigned, the `U` is small, the `Vec` contains at
// least two elements, and its last element is nonzero.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_8<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, T, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_8,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_6,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_8,
    )
}

// var 9 is in malachite-nz.

// All `(Vec<T>, T, T)`s where `T` is unsigned, the `Vec` is nonempty, and the first `T` is odd.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, T, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_10,
        &random_unsigned_vec_unsigned_unsigned_triple_gen_var_3,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_10,
    )
}

// vars 11 through 12 are in malachite-nz.

// All `(Vec<T>, T, U)` where `T` and `U` are unsigned, the `U` is small, and the `Vec` ends with a
// nonzero element.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_13<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, T, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_13,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_7,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_13,
    )
}

// var 14 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, RoundingMode) --

// All `(Vec<T>, T, RoundingMode)` where `T` is unsigned, the `Vec` has at least two elements, and
// the `Vec`s last element is nonzero.
pub fn unsigned_vec_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_rounding_mode_triple_gen_var_1,
        &random_primitive_int_vec_primitive_int_rounding_mode_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// All `(Vec<T>, U, RoundingMode)` where `T` and `U` are unsigned, the `U` is small, and the `Vec`
// does not only contain zeros.
pub fn unsigned_vec_unsigned_rounding_mode_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, U, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
        &random_primitive_int_vec_primitive_int_rounding_mode_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub fn unsigned_vec_pair_gen<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen,
        &random_primitive_int_vec_pair_gen,
        &special_random_unsigned_vec_pair_gen,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s are nonempty, and the first `Vec` is at
// least as long as the second.
pub fn unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_2,
        &random_primitive_int_vec_pair_gen_var_2,
        &special_random_unsigned_vec_pair_gen_var_2,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and both `Vec`s are nonempty.
pub fn unsigned_vec_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_3,
        &random_primitive_int_vec_pair_gen_var_3,
        &special_random_unsigned_vec_pair_gen_var_3,
    )
}

// All `(Vec<T>, Vec<T>)` that are valid inputs to `limbs_pow_low` in malachite-nz.
pub fn unsigned_vec_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_4,
        &random_primitive_int_vec_pair_gen_var_4,
        &special_random_unsigned_vec_pair_gen_var_4,
    )
}

// All `(Vec<T>, Vec<T>)` that are valid `(out, xs)` inputs to `limbs_sqrt_rem_helper`.
pub fn unsigned_vec_pair_gen_var_4<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_5::<T>,
        &random_unsigned_vec_pair_gen_var_1::<T>,
        &special_random_unsigned_vec_pair_gen_var_5::<T>,
    )
}

// All `(Vec<T>, Vec<T>)` that are valid `(out, xs)` inputs to `limbs_sqrt_to_out`.
pub fn unsigned_vec_pair_gen_var_5<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_6::<T>,
        &random_unsigned_vec_pair_gen_var_2::<T>,
        &special_random_unsigned_vec_pair_gen_var_6::<T>,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and both `Vec`s have the same length.
pub fn unsigned_vec_pair_gen_var_6<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_7,
        &random_primitive_int_vec_pair_gen_var_5,
        &special_random_unsigned_vec_pair_gen_var_7,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and neither `Vec` ends with zero.
pub fn unsigned_vec_pair_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_8,
        &random_primitive_int_vec_pair_gen_var_6,
        &special_random_unsigned_vec_pair_gen_var_8,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and each `Vec` contains at least one nonzero value.
pub fn unsigned_vec_pair_gen_var_8<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_9,
        &random_primitive_int_vec_pair_gen_var_7,
        &special_random_unsigned_vec_pair_gen_var_9,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, each `Vec` contains at least one nonzero value, and
// the first `Vec` is at least as long as the second.
pub fn unsigned_vec_pair_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_10,
        &random_primitive_int_vec_pair_gen_var_8,
        &special_random_unsigned_vec_pair_gen_var_10,
    )
}

// var 10 is in malachite-nz.

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s have at least 2 elements, the first
// `Vec` is at least as long as the second, and the last element of the second `Vec` is nonzero.
pub fn unsigned_vec_pair_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_12,
        &random_primitive_int_vec_pair_gen_var_10,
        &special_random_unsigned_vec_pair_gen_var_12,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s are nonempty, the first `Vec` is at
// least as long as the second, and the first element of the second `Vec` is odd.
pub fn unsigned_vec_pair_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_13,
        &random_primitive_int_vec_pair_gen_var_11,
        &special_random_unsigned_vec_pair_gen_var_13,
    )
}

// vars 13 through 14 are in malachite-nz.

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s are nonempty, the last elements of both
// `Vec`s are nonzero, and the first `Vec` is at least as long as the second.
pub fn unsigned_vec_pair_gen_var_15<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_16,
        &random_primitive_int_vec_pair_gen_var_12,
        &special_random_unsigned_vec_pair_gen_var_16,
    )
}

// All `(Vec<T>, Vec<T>)` that are valid inputs to `limbs_div_mod` and `limbs_divisible_by`.
pub fn unsigned_vec_pair_gen_var_16<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_17,
        &random_primitive_int_vec_pair_gen_var_13,
        &special_random_unsigned_vec_pair_gen_var_17,
    )
}

// var 17 is in malachite-nz.

// All `(Vec<T>, Vec<T>)` that are valid inputs to `limbs_mod_by_two_limb_normalized`.
pub fn unsigned_vec_pair_gen_var_18<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_19,
        &random_unsigned_vec_pair_gen_var_5,
        &special_random_unsigned_vec_pair_gen_var_19,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and both `Vec`s end with a nonzero number.
pub fn unsigned_vec_pair_gen_var_19<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_20,
        &random_primitive_int_vec_pair_gen_var_14,
        &special_random_unsigned_vec_pair_gen_var_20,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s have at least 2 elements, and the first
// `Vec` is at least as long as the second.
pub fn unsigned_vec_pair_gen_var_20<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_21,
        &random_primitive_int_vec_pair_gen_var_15,
        &special_random_unsigned_vec_pair_gen_var_21,
    )
}

// vars 21 through 30 are malachite-nz.

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and the first `Vec` is at least as long as the
// second.
pub fn unsigned_vec_pair_gen_var_31<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_32,
        &random_primitive_int_vec_pair_gen_var_26,
        &special_random_unsigned_vec_pair_gen_var_32,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s have the same length, at least one ends
// with a nonzero value, and the first element of the second `Vec` is odd.
pub fn unsigned_vec_pair_gen_var_32<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_33,
        &random_primitive_int_vec_pair_gen_var_27,
        &special_random_unsigned_vec_pair_gen_var_33,
    )
}

// var 33 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, bool) --

// All `(Vec<T>, Vec<T>, bool)` where `T` is unsigned and both `Vec`s have the same length.
pub fn unsigned_vec_unsigned_vec_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, bool)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_bool_triple_gen_var_1,
        &random_primitive_int_vec_primitive_int_vec_bool_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_bool_triple_gen_var_1,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned and the first `Vec` is at least as long as the
// second.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    )
}

// vars 2 and 3 are in malachite-nz

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second is nonempty.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4,
    )
}

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second contains at least one nonzero value.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_3,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
    )
}

// All `(Vec<T>, Vec<T>, U)` where `T` and `U` are unsigned, the `U` is positive, both `Vec`s
// contain at least two elements, and their last elements are nonzero.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_4,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6,
    )
}

// vars 7 through 8 are in malachite-nz.

// All `(Vec<T>, Vec<T>, U)` where `T` and `U` are unsigned, the `U` is small, and the last elements
// of both `Vec`s are nonzero.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_5,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9,
    )
}

// All `(Vec<T>, Vec<T>, U)` where `T` and `U` are unsigned, the `U` is positive, and the last
// elements of both `Vec`s are nonzero.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_6,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10,
    )
}

// All `(Vec<T>, Vec<T>, usize)` where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the `usize` is no greater than the length of the second `Vec`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, usize)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11,
        &random_primitive_int_vec_primitive_int_vec_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11,
    )
}

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned and the two `Vec`s have the same length.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_7,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12,
    )
}

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned and positive, the first `Vec` is at least as long
// as the second, and the second has at least two elements.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13,
        &random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13,
    )
}

// vars 14 through 21 are in malachite-nz.

// All `(Vec<T>, Vec<T>, u64)` where `T` is unsigned, `U` is a primitive integer type, the first
// `Vec` is at least as long as the second, and the `u64` is between 1 and `U::WIDTH - 1`,
// inclusive.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> Generator<(Vec<T>, Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22::<T, U>,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_8::<T, U>,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22::<T, U>,
    )
}

// All `(Vec<T>, Vec<T>, u64)` where `T` is unsigned, `U` is a primitive integer type, the first
// `Vec` is at least as long as the second, neither `Vec` is empty, and the `u64` is between 1 and
// `U::WIDTH - 1`, inclusive.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> Generator<(Vec<T>, Vec<T>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23::<T, U>,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_9::<T, U>,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23::<T, U>,
    )
}

// All `(Vec<T>, Vec<T>, usize)` where `T` is unsigned, the `Vec`s have the same length, and the
// `usize` is no greater than the length of either `Vec`.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, usize)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24,
        &random_primitive_int_vec_primitive_int_vec_unsigned_triple_gen_var_2,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, the second and third
// `Vec`s have equal length, and the first is at least twice as long as the second.
pub fn unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_1,
        &random_primitive_int_vec_triple_gen_var_1,
        &special_random_unsigned_vec_triple_gen_var_1,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, the second is at least
// as long as the third, and the length of the first is at least the sum of the lengths of the
// second and the third.
pub fn unsigned_vec_triple_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_2,
        &random_primitive_int_vec_triple_gen_var_2,
        &special_random_unsigned_vec_triple_gen_var_2,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, and the length of the
// first is at least the sum of the lengths of the second and the third.
pub fn unsigned_vec_triple_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_3,
        &random_primitive_int_vec_triple_gen_var_3,
        &special_random_unsigned_vec_triple_gen_var_3,
    )
}

// vars 4 through 23 are in malachite-nz

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, the second and third
// `Vec`s have equal length, and the first is at least as long as the second.
pub fn unsigned_vec_triple_gen_var_24<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_24,
        &random_primitive_int_vec_triple_gen_var_24,
        &special_random_unsigned_vec_triple_gen_var_24,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, all `Vec`s have length at least 2, the
// second and third `Vec`s have equal length, and the first is at least twice as long as the second.
pub fn unsigned_vec_triple_gen_var_25<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_25,
        &random_primitive_int_vec_triple_gen_var_25,
        &special_random_unsigned_vec_triple_gen_var_25,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, all `Vec`s have length at least 2, the
// second and third `Vec`s have equal length, and the first is at least as long as the second.
pub fn unsigned_vec_triple_gen_var_26<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_26,
        &random_primitive_int_vec_triple_gen_var_26,
        &special_random_unsigned_vec_triple_gen_var_26,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and all three `Vec`s have the same length,
// which is at least 2.
pub fn unsigned_vec_triple_gen_var_27<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_27,
        &random_primitive_int_vec_triple_gen_var_27,
        &special_random_unsigned_vec_triple_gen_var_27,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid `(out, rs, xs)` inputs to `limbs_sqrt_rem_to_out`.
pub fn unsigned_vec_triple_gen_var_28<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_28::<T>,
        &random_unsigned_vec_triple_gen_var_9::<T>,
        &special_random_unsigned_vec_triple_gen_var_28::<T>,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and all three `Vec`s have the same length.
pub fn unsigned_vec_triple_gen_var_29<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_29,
        &random_primitive_int_vec_triple_gen_var_28,
        &special_random_unsigned_vec_triple_gen_var_29,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and no `Vec` ends with zero.
pub fn unsigned_vec_triple_gen_var_30<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_30,
        &random_primitive_int_vec_triple_gen_var_29,
        &special_random_unsigned_vec_triple_gen_var_30,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, the second and third `Vec`s have equal
// length, and the first is at least as long as the second.
pub fn unsigned_vec_triple_gen_var_31<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_31,
        &random_primitive_int_vec_triple_gen_var_30,
        &special_random_unsigned_vec_triple_gen_var_31,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and the first `Vec` is at least as long as
// the second and at least as long as the third.
pub fn unsigned_vec_triple_gen_var_32<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_32,
        &random_primitive_int_vec_triple_gen_var_31,
        &special_random_unsigned_vec_triple_gen_var_32,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, the second and third `Vec`s each contain at
// least one nonzero element, and the first `Vec` is at least as long as the second.
pub fn unsigned_vec_triple_gen_var_33<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_33,
        &random_primitive_int_vec_triple_gen_var_32,
        &special_random_unsigned_vec_triple_gen_var_33,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, the second and third `Vec`s each contain at
// least one nonzero element, and the first `Vec` is at least as long as the second AND at least as
// long as the third.
pub fn unsigned_vec_triple_gen_var_34<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_34,
        &random_primitive_int_vec_triple_gen_var_33,
        &special_random_unsigned_vec_triple_gen_var_34,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, the second and third `Vec`s each contain at
// least one nonzero element, and the first `Vec` is at least as long as the second OR at least as
// long as the third.
pub fn unsigned_vec_triple_gen_var_35<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_35,
        &random_primitive_int_vec_triple_gen_var_34,
        &special_random_unsigned_vec_triple_gen_var_35,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, each `Vec` contains at least two elements,
// and the last element of each `Vec` is nonzero.
pub fn unsigned_vec_triple_gen_var_36<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_36,
        &random_primitive_int_vec_triple_gen_var_35,
        &special_random_unsigned_vec_triple_gen_var_36,
    )
}

// vars 37 through 38 are in malachite-nz

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and each `Vec` ends with a nonzero element.
pub fn unsigned_vec_triple_gen_var_39<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_39,
        &random_primitive_int_vec_triple_gen_var_38,
        &special_random_unsigned_vec_triple_gen_var_39,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and the first `Vec` is at least as long as
// the second and the second is at least as long as the third.
pub fn unsigned_vec_triple_gen_var_40<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_40,
        &random_primitive_int_vec_triple_gen_var_39,
        &special_random_unsigned_vec_triple_gen_var_40,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, every `Vec` ends with a nonzero element,
// and the last `Vec` has length at least 2.
pub fn unsigned_vec_triple_gen_var_41<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_41,
        &random_primitive_int_vec_triple_gen_var_40,
        &special_random_unsigned_vec_triple_gen_var_41,
    )
}

// vars 42 through 49 are in malachite-nz.

// All `(Vec<T>, Vec<T>, Vec<T>)` where the first `Vec` is at least as long as the second, the third
// is at least as long as twice the length of the second, and the second is nonempty and its most
// significant bit is set.
pub fn unsigned_vec_triple_gen_var_50<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_50,
        &random_primitive_int_vec_triple_gen_var_41,
        &special_random_unsigned_vec_triple_gen_var_50,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where the first `Vec` is at least as long as the second, the third
// is at least as long as twice the length of the second, and the second has length at least 5 and
// its most significant bit is set.
pub fn unsigned_vec_triple_gen_var_51<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_51,
        &random_primitive_int_vec_triple_gen_var_42,
        &special_random_unsigned_vec_triple_gen_var_51,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that meet certain preconditions that enable comparing the
// performance of divide-and-conquer division and Barrett division.
pub fn unsigned_vec_triple_gen_var_52<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_52,
        &random_primitive_int_vec_triple_gen_var_43,
        &special_random_unsigned_vec_triple_gen_var_52,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` that are valid inputs to `limbs_div_mod_by_two_limb_normalized`.
pub fn unsigned_vec_triple_gen_var_53<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_53,
        &random_unsigned_vec_triple_gen_var_10,
        &special_random_unsigned_vec_triple_gen_var_53,
    )
}

// vars 54 through 56 are in malachite-base.

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, the first `Vec` is at least as long as the
// second and at least as long as the third, all `Vec`s have at least 2 elements, and the last
// element of the third is nonzero.
pub fn unsigned_vec_triple_gen_var_57<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_57,
        &random_primitive_int_vec_triple_gen_var_44,
        &special_random_unsigned_vec_triple_gen_var_57,
    )
}

// var 58 is in malachite-nz.

// All `(Vec<T>, Vec<T>, Vec<T>)` `(xs, ys, zs)` where `T` is unsigned, `ys` and `zs` have at least
// two elements, `xs` has at least `ys.len() + zs.len() - 1` elements, and each slice ends with a
// nonzero value.
pub fn unsigned_vec_triple_gen_var_59<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_59,
        &random_primitive_int_vec_triple_gen_var_46,
        &special_random_unsigned_vec_triple_gen_var_59,
    )
}

// var 60 is in malachite-nz.

// -- large types --

// TODO description
pub fn large_type_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, T, T)> {
    Generator::new(
        &exhaustive_large_type_gen_var_1,
        &random_large_type_gen_var_1,
        &special_random_large_type_gen_var_1,
    )
}

// All `(Vec<T>, Vec<T>, u64, bool)` that are valid inputs to `limbs_sqrt_helper`.
pub fn large_type_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, u64, bool)> {
    Generator::new(
        &exhaustive_large_type_gen_var_2,
        &random_large_type_gen_var_2,
        &special_random_large_type_gen_var_2,
    )
}

// All `(Vec<T>, U, U, Vec<T>)` where `T` and `U` are unsigned and the first `U` is less than the
// second.
pub fn large_type_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U, U, Vec<T>)> {
    Generator::new(
        &exhaustive_large_type_gen_var_3,
        &random_large_type_gen_var_3,
        &special_random_large_type_gen_var_3,
    )
}

// All `(Vec<T>, U, U, Vec<T>)` where `T` and `U` are unsigned, the first `U` is less than the
// second, and the first `Vec` contains at least one nonzero value.
pub fn large_type_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U, U, Vec<T>)> {
    Generator::new(
        &exhaustive_large_type_gen_var_4,
        &random_large_type_gen_var_4,
        &special_random_large_type_gen_var_4,
    )
}

// vars 5 through 8 are in malachite-nz

// All `(Vec<T>, Vec<T>, Vec<T>, bool)` where `T` is unsigned, the second and third `Vec`s have
// equal length, and the first is at least as long as the second.
#[allow(clippy::type_complexity)]
pub fn large_type_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>, bool)> {
    Generator::new(
        &exhaustive_large_type_gen_var_9,
        &random_large_type_gen_var_9,
        &special_random_large_type_gen_var_9,
    )
}

// vars 10 through 21 are in malachite-nz.

type T2<T> = Generator<(RationalSequence<T>, usize, T, T)>;

// All `(RationalSequence<T>, usize, T, T)` quadruples where `T` is unsigned and the `usize` is less
// than the length of the `RationalSequence`.
pub fn large_type_gen_var_22<T: PrimitiveUnsigned>() -> T2<T> {
    Generator::new(
        &exhaustive_large_type_gen_var_22,
        &random_large_type_gen_var_22,
        &special_random_large_type_gen_var_22,
    )
}

// vars 23 through 26 are in malachite-nz.

// All (bool, Vec<T>, bool, Vec<T>) where `T` is unsigned and neither `Vec` ends with a zero.
pub fn large_type_gen_var_27<T: PrimitiveUnsigned>() -> Generator<(bool, Vec<T>, bool, Vec<T>)> {
    Generator::new(
        &exhaustive_large_type_gen_var_27,
        &random_large_type_gen_var_27,
        &special_random_large_type_gen_var_25,
    )
}

// var 23 is in malachite-nz.

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
