// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::exhaustive::{exhaustive_bools, ExhaustiveBools};
use crate::chars::constants::NUMBER_OF_CHARS;
use crate::chars::exhaustive::{exhaustive_ascii_chars, exhaustive_chars};
use crate::iterators::bit_distributor::BitDistributorOutputType;
use crate::iterators::iter_windows;
use crate::max;
use crate::num::arithmetic::traits::CoprimeWith;
use crate::num::arithmetic::traits::{
    ArithmeticCheckedShl, CheckedNeg, DivRound, Parity, PowerOf2, ShrRound, UnsignedAbs,
};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::string::options::exhaustive::{
    exhaustive_from_sci_string_options, exhaustive_sci_size_options, exhaustive_to_sci_options,
};
use crate::num::conversion::string::options::{FromSciStringOptions, SciSizeOptions, ToSciOptions};
use crate::num::conversion::traits::{
    ConvertibleFrom, Digits, ExactFrom, HasHalf, JoinHalves, RoundingFrom, SaturatingFrom,
    SplitInHalf, WrappingFrom,
};
use crate::num::exhaustive::{
    exhaustive_finite_primitive_floats, exhaustive_natural_signeds, exhaustive_negative_signeds,
    exhaustive_nonzero_finite_primitive_floats, exhaustive_nonzero_signeds,
    exhaustive_positive_finite_primitive_floats, exhaustive_positive_primitive_ints,
    exhaustive_primitive_float_range, exhaustive_primitive_floats,
    exhaustive_signed_inclusive_range, exhaustive_signed_range, exhaustive_signeds,
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
    ExhaustiveSigneds, PrimitiveIntIncreasingRange,
};
use crate::num::float::NiceFloat;
use crate::num::iterators::{bit_distributor_sequence, ruler_sequence};
use crate::num::logic::traits::{BitBlockAccess, LeadingZeros};
use crate::rational_sequences::exhaustive::exhaustive_rational_sequences;
use crate::rational_sequences::RationalSequence;
use crate::rounding_modes::exhaustive::exhaustive_rounding_modes;
use crate::rounding_modes::RoundingMode::{self, *};
use crate::slices::slice_test_zero;
use crate::strings::exhaustive::{exhaustive_strings, exhaustive_strings_using_chars};
use crate::strings::{strings_from_char_vecs, StringsFromCharVecs};
use crate::test_util::extra_variadic::{
    exhaustive_duodecuples_from_single, exhaustive_octuples_from_single,
    exhaustive_quadruples_from_single, exhaustive_quadruples_xxxy,
    exhaustive_quadruples_xxxy_custom_output, exhaustive_quadruples_xxyx,
    exhaustive_quadruples_xyyx, exhaustive_quadruples_xyyz,
    exhaustive_quadruples_xyyz_custom_output, exhaustive_quadruples_xyzz,
    exhaustive_sextuples_from_single, exhaustive_triples_from_single, exhaustive_triples_xxy,
    exhaustive_triples_xxy_custom_output, exhaustive_triples_xyx,
    exhaustive_triples_xyx_custom_output, lex_triples_from_single, lex_triples_xyy, lex_union3s,
    ExhaustiveTriples1Input, ExhaustiveTriplesXXY, Union3,
};
use crate::test_util::generators::common::{
    permute_1_3_2, permute_2_1, permute_3_1_4_2, reshape_1_2_to_3, reshape_2_1_1_to_4,
    reshape_2_1_to_3, reshape_2_2_to_4, reshape_3_1_to_4, It,
};
use crate::test_util::generators::{
    digits_valid, exhaustive_pairs_big_small, exhaustive_pairs_big_tiny, large_exponent,
    signed_assign_bits_valid, smallest_invalid_value, unsigned_assign_bits_valid,
};
use crate::test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;
use crate::test_util::num::conversion::string::from_sci_string::DECIMAL_SCI_STRING_CHARS;
use crate::test_util::num::float::PRIMITIVE_FLOAT_CHARS;
use crate::test_util::rounding_modes::ROUNDING_MODE_CHARS;
use crate::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_ordered_unique_pairs, exhaustive_pairs,
    exhaustive_pairs_from_single, exhaustive_quadruples, exhaustive_triples,
    exhaustive_triples_custom_output, exhaustive_triples_xyy, exhaustive_triples_xyy_custom_output,
    lex_pairs, lex_pairs_from_single, ExhaustiveDependentPairsYsGenerator, ExhaustivePairs,
    ExhaustivePairs1Input, ExhaustiveQuadruples, ExhaustiveTriples, ExhaustiveTriplesXYY,
};
use crate::vecs::exhaustive::{
    exhaustive_vecs, exhaustive_vecs_fixed_length_from_single,
    exhaustive_vecs_length_inclusive_range, exhaustive_vecs_min_length,
    lex_vecs_fixed_length_from_single, shortlex_vecs, shortlex_vecs_length_inclusive_range,
    shortlex_vecs_min_length, ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs,
    LexFixedLengthVecsFromSingle, ShortlexVecs,
};
use itertools::{repeat_n, Itertools};
use std::cmp::{max, min};
use std::iter::once;
use std::marker::PhantomData;
use std::vec::IntoIter;

// general

fn add_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_add_mul(y, z).is_some()
}

fn sub_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_sub_mul(y, z).is_some()
}

// -- bool --

pub fn exhaustive_bool_gen() -> It<bool> {
    Box::new(exhaustive_bools())
}

// -- char --

pub fn exhaustive_char_gen() -> It<char> {
    Box::new(exhaustive_chars())
}

#[allow(unstable_name_collisions)]
pub fn exhaustive_char_gen_var_1() -> It<char> {
    Box::new(char::MIN..char::MAX)
}

#[allow(unstable_name_collisions)]
pub fn exhaustive_char_gen_var_2() -> It<char> {
    Box::new('\u{1}'..=char::MAX)
}

// -- (char, char) --

pub fn exhaustive_char_pair_gen() -> It<(char, char)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_chars()))
}

// -- FromSciStringOptions --

pub fn exhaustive_from_sci_string_options_gen() -> It<FromSciStringOptions> {
    Box::new(exhaustive_from_sci_string_options())
}

// -- (FromSciStringOptions, PrimitiveUnsigned) --

pub fn exhaustive_from_sci_string_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(FromSciStringOptions, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_from_sci_string_options(),
        primitive_int_increasing_inclusive_range(T::TWO, T::from(36u8)),
    ))
}

// -- (FromSciStringOptions, RoundingMode) --

pub fn exhaustive_from_sci_string_options_rounding_mode_pair_gen(
) -> It<(FromSciStringOptions, RoundingMode)> {
    Box::new(exhaustive_pairs(
        exhaustive_from_sci_string_options(),
        exhaustive_rounding_modes(),
    ))
}

// -- PrimitiveFloat --

pub fn exhaustive_primitive_float_gen<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_floats())
}

pub fn exhaustive_primitive_float_gen_var_1<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_float_range(
        T::NEGATIVE_ONE / T::TWO,
        T::INFINITY,
    ))
}

struct ExhaustivePositiveNaturalFloats<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    done: bool,
    exponent: i64,
    limit: u64,
    mantissa: u64,
}

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveNaturalFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            let f = T::from_integer_mantissa_and_exponent(self.mantissa, self.exponent).unwrap();
            if f == T::MAX_FINITE {
                self.done = true;
            } else {
                self.mantissa += 1;
                if self.mantissa == self.limit {
                    self.mantissa >>= 1;
                    self.exponent += 1;
                    self.limit = u64::power_of_2(T::MANTISSA_WIDTH + 1);
                }
            }
            Some(f)
        }
    }
}

fn exhaustive_positive_natural_floats<T: PrimitiveFloat>() -> ExhaustivePositiveNaturalFloats<T> {
    ExhaustivePositiveNaturalFloats {
        phantom: PhantomData,
        done: false,
        exponent: 0,
        limit: u64::power_of_2(T::MANTISSA_WIDTH + 1),
        mantissa: 1,
    }
}

pub fn exhaustive_primitive_float_gen_var_2<T: PrimitiveFloat>() -> It<T> {
    Box::new(once(T::ZERO).chain(exhaustive_positive_natural_floats()))
}

pub fn exhaustive_primitive_float_gen_var_3<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_positive_finite_primitive_floats::<T>().filter(|f| !f.is_integer()))
}

pub fn exhaustive_primitive_float_gen_var_4<T: PrimitiveFloat>() -> It<T> {
    let limit =
        T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH)).unwrap();
    Box::new(
        exhaustive_positive_natural_floats::<T>()
            .take_while(move |&f| f <= limit)
            .map(|f| f - T::ONE / T::TWO),
    )
}

pub fn exhaustive_primitive_float_gen_var_5<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        lex_pairs(
            exhaustive_primitive_float_gen_var_2::<T>(),
            exhaustive_bools(),
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn exhaustive_primitive_float_gen_var_6<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        lex_pairs(
            exhaustive_primitive_float_gen_var_3::<T>(),
            exhaustive_bools(),
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn exhaustive_primitive_float_gen_var_7<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        lex_pairs(
            exhaustive_primitive_float_gen_var_4::<T>(),
            exhaustive_bools(),
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn exhaustive_primitive_float_gen_var_8<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_finite_primitive_floats())
}

pub fn exhaustive_primitive_float_gen_var_9<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan() && f != T::INFINITY))
}

pub fn exhaustive_primitive_float_gen_var_10<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan() && f != T::NEGATIVE_INFINITY),
    )
}

pub fn exhaustive_primitive_float_gen_var_11<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan()))
}

pub fn exhaustive_primitive_float_gen_var_12<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_nonzero_finite_primitive_floats())
}

pub fn exhaustive_primitive_float_gen_var_13<T: PrimitiveFloat, U: PrimitiveUnsigned>() -> It<T>
where
    NiceFloat<T>: TryFrom<U>,
{
    Box::new(
        exhaustive_unsigneds::<U>().filter_map(|x| NiceFloat::<T>::try_from(x).ok().map(|x| x.0)),
    )
}

pub fn exhaustive_primitive_float_gen_var_14<T: PrimitiveFloat, U: PrimitiveSigned>() -> It<T>
where
    NiceFloat<T>: TryFrom<U>,
{
    Box::new(
        exhaustive_signeds::<U>().filter_map(|x| NiceFloat::<T>::try_from(x).ok().map(|x| x.0)),
    )
}

pub fn exhaustive_primitive_float_gen_var_15<
    T: PrimitiveFloat,
    U: ConvertibleFrom<T> + PrimitiveInt,
>() -> It<T> {
    Box::new(exhaustive_primitive_floats::<T>().filter(|&f| !U::convertible_from(f)))
}

pub fn exhaustive_primitive_float_gen_var_16<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned,
>() -> It<T> {
    let limit = min(
        NiceFloat(T::rounding_from(U::MAX, Down).0),
        NiceFloat(
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        ),
    )
    .0;
    Box::new(
        exhaustive_positive_natural_floats::<T>()
            .take_while(move |&f| f <= limit)
            .map(|f| f - T::ONE / T::TWO),
    )
}

pub fn exhaustive_primitive_float_gen_var_17<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned,
>() -> It<T> {
    let min_limit = min(
        NiceFloat(-T::rounding_from(U::MIN, Down).0),
        NiceFloat(
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        ),
    )
    .0;
    let max_limit = min(
        NiceFloat(T::rounding_from(U::MAX, Down).0),
        NiceFloat(
            T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH))
                .unwrap(),
        ),
    )
    .0;
    Box::new(
        exhaustive_positive_natural_floats::<T>()
            .take_while(move |&f| f <= max_limit)
            .map(|f| f - T::ONE / T::TWO)
            .interleave(
                exhaustive_positive_natural_floats::<T>()
                    .take_while(move |&f| f <= min_limit)
                    .map(|f| T::ONE / T::TWO - f),
            ),
    )
}

pub fn exhaustive_primitive_float_gen_var_18<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_positive_finite_primitive_floats::<T>())
}

pub fn exhaustive_primitive_float_gen_var_19<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_float_range(
        T::ZERO,
        T::power_of_2(T::MAX_EXPONENT),
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_primitive_floats()))
}

pub fn exhaustive_primitive_float_pair_gen_var_1<T: PrimitiveFloat>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan()),
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_primitive_float_triple_gen<T: PrimitiveFloat>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_primitive_floats()))
}

// -- (PrimitiveFloat, PrimitiveInt) --

pub fn exhaustive_primitive_float_primitive_int_pair_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveInt,
>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_finite_primitive_floats(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (PrimitiveFloat, PrimitiveSigned) --

pub fn exhaustive_primitive_float_signed_pair_gen<T: PrimitiveFloat, U: PrimitiveSigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_primitive_floats(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_primitive_float_signed_pair_gen_var_1<T: PrimitiveFloat, U: PrimitiveSigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_finite_primitive_floats(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_primitive_float_signed_pair_gen_var_2<T: PrimitiveFloat>() -> It<(T, i64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_primitive_float_range(T::ONE, T::TWO),
            exhaustive_signed_inclusive_range(T::MIN_EXPONENT, T::MAX_EXPONENT),
        )
        .filter(|&(m, e)| m.precision() <= T::max_precision_for_sci_exponent(e)),
    )
}

pub fn exhaustive_primitive_float_signed_pair_gen_var_3<T: PrimitiveFloat>() -> It<(T, i64)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_primitive_float_range(T::ONE, T::TWO),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_primitive_float_signed_pair_gen_var_4<T: PrimitiveFloat, U: PrimitiveSigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_primitive_floats(),
        exhaustive_signeds(),
    ))
}

// -- (PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_primitive_float_signed_unsigned_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    reshape_1_2_to_3(Box::new(exhaustive_pairs_big_small(
        exhaustive_primitive_floats(),
        exhaustive_pairs(exhaustive_signeds(), exhaustive_positive_primitive_ints()),
    )))
}

// -- (PrimitiveFloat, PrimitiveUnsigned) --

pub fn exhaustive_primitive_float_unsigned_pair_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_finite_primitive_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_primitive_float_unsigned_pair_gen_var_2<T: PrimitiveFloat>() -> It<(T, u64)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_primitive_float_range(T::ONE, T::TWO),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_primitive_float_unsigned_pair_gen_var_3<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_primitive_floats(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_positive_finite_primitive_floats(),
            exhaustive_unsigneds(),
        ),
        exhaustive_rounding_modes(),
    )))
}

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveFloat>(
) -> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_primitive_float_range(T::ONE, T::TWO),
            exhaustive_unsigneds(),
        ),
        exhaustive_rounding_modes(),
    )))
}

// var 3 is in malachite-float.

// -- (PrimitiveFloat, RoundingMode) --

pub(crate) fn float_rounding_mode_filter_var_1<T: PrimitiveFloat>(p: &(T, RoundingMode)) -> bool {
    let &(f, rm) = p;
    match rm {
        Floor | Up => f >= T::ZERO,
        Ceiling | Down => f > T::NEGATIVE_ONE,
        Nearest => f >= T::NEGATIVE_ONE / T::TWO,
        Exact => f >= T::ZERO && f.is_integer(),
    }
}

pub fn exhaustive_primitive_float_rounding_mode_pair_gen_var_1<T: PrimitiveFloat>(
) -> It<(T, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_finite_primitive_floats(),
            exhaustive_rounding_modes(),
        )
        .filter(float_rounding_mode_filter_var_1),
    )
}

pub fn exhaustive_primitive_float_rounding_mode_pair_gen_var_2<T: PrimitiveFloat>(
) -> It<(T, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_finite_primitive_floats::<T>(),
            exhaustive_rounding_modes(),
        )
        .filter(|&(f, rm)| rm != Exact || f.is_integer()),
    )
}

pub fn exhaustive_primitive_float_rounding_mode_pair_gen_var_3<
    T: PrimitiveFloat + RoundingFrom<U>,
    U: ConvertibleFrom<T> + PrimitiveInt,
>() -> It<(T, RoundingMode)> {
    let f_min = T::rounding_from(U::MIN, Down).0;
    let f_max = T::rounding_from(U::MAX, Down).0;
    Box::new(
        lex_pairs(
            exhaustive_primitive_floats::<T>().filter(|f| !f.is_nan()),
            exhaustive_rounding_modes(),
        )
        .filter(move |&(f, rm)| match rm {
            Up => f >= f_min && f <= f_max,
            Ceiling => f <= f_max,
            Floor => f >= f_min,
            Down | Nearest => true,
            Exact => U::convertible_from(f),
        }),
    )
}

// -- PrimitiveInt --

pub fn exhaustive_primitive_int_gen_var_1<T: PrimitiveInt>() -> It<T> {
    Box::new(exhaustive_positive_primitive_ints())
}

pub fn exhaustive_primitive_int_gen_var_2<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(T::TWO, T::MAX))
}

pub fn exhaustive_primitive_int_gen_var_3<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_range(T::ZERO, T::exact_from(36)))
}

pub fn exhaustive_primitive_int_gen_var_4<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::TWO,
        T::exact_from(36),
    ))
}

pub fn exhaustive_primitive_int_gen_var_5<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::power_of_2(T::WIDTH - 2),
        T::MAX,
    ))
}

pub fn exhaustive_primitive_int_gen_var_6<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::wrapping_from(5u8),
        T::MAX,
    ))
}

// -- (PrimitiveInt, PrimitiveInt) --

pub fn exhaustive_primitive_int_pair_gen_var_1<T: PrimitiveInt, U: ExactFrom<u8> + PrimitiveInt>(
) -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_positive_primitive_ints(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_primitive_int_pair_gen_var_2<T: PrimitiveInt, U: PrimitiveInt>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_primitive_ints(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_primitive_int_pair_gen_var_3<T: PrimitiveInt, U: PrimitiveInt>() -> T1<T, U> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_positive_primitive_ints(),
        primitive_int_increasing_inclusive_range(U::TWO, U::MAX),
    ))
}

pub fn exhaustive_primitive_int_pair_gen_var_4<T: PrimitiveInt>() -> It<(T, T)> {
    Box::new(exhaustive_ordered_unique_pairs(
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (PrimitiveInt, PrimitiveUnsigned) --

pub fn exhaustive_primitive_int_unsigned_pair_gen_var_1<T: PrimitiveInt, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_primitive_ints(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_primitive_int_unsigned_pair_gen_var_2<T: PrimitiveInt, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 2), T::MAX),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_primitive_int_unsigned_pair_gen_var_3<T: PrimitiveInt>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

// -- (PrimitiveInt, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_primitive_int_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>() -> It<(T, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_positive_primitive_ints(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(x, y, z): (T, U, U)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

// -- (PrimitiveInt, RoundingMode) --

pub fn exhaustive_primitive_int_rounding_mode_pair_gen_var_1<T: PrimitiveInt>(
) -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_positive_primitive_ints(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_primitive_int_rounding_mode_pair_gen_var_2<T: PrimitiveInt>(
) -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_positive_primitive_ints(),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    ))
}

// -- PrimitiveSigned --

pub fn exhaustive_signed_gen<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds())
}

pub fn exhaustive_signed_gen_var_1<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signed_inclusive_range(T::MIN + T::ONE, T::MAX))
}

pub fn exhaustive_signed_gen_var_2<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_natural_signeds())
}

pub fn exhaustive_signed_gen_var_3<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| x != T::ZERO && x != T::NEGATIVE_ONE))
}

pub fn exhaustive_signed_gen_var_4<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_negative_signeds())
}

pub fn exhaustive_signed_gen_var_5<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_nonzero_signeds())
}

pub fn exhaustive_signed_gen_var_6<T: PrimitiveSigned, U: ConvertibleFrom<T> + PrimitiveFloat>(
) -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| U::convertible_from(x)))
}

pub fn exhaustive_signed_gen_var_7<T: PrimitiveSigned, U: ConvertibleFrom<T> + PrimitiveFloat>(
) -> It<T> {
    Box::new(
        primitive_int_increasing_inclusive_range(
            T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT),
            T::MAX,
        )
        .interleave(
            primitive_int_increasing_inclusive_range(
                T::MIN,
                T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT).saturating_neg(),
            )
            .rev(),
        )
        .filter(|&x| !U::convertible_from(x)),
    )
}

pub fn exhaustive_signed_gen_var_8<T: PrimitiveSigned, U: ConvertibleFrom<T> + PrimitiveFloat>(
) -> It<T> {
    Box::new(
        iter_windows(
            2,
            primitive_int_increasing_inclusive_range(
                T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT),
                T::MAX,
            )
            .filter(|&x| U::convertible_from(x)),
        )
        .filter_map(|xs| {
            let mut xs = xs.into_iter();
            let a = xs.next().unwrap();
            let diff = xs.next().unwrap() - a;
            if diff.even() {
                // This happens almost always
                Some(a + (diff >> 1))
            } else {
                None
            }
        })
        .interleave(
            iter_windows(
                2,
                primitive_int_increasing_inclusive_range(
                    T::MIN,
                    T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT)
                        .checked_neg()
                        .unwrap(),
                )
                .rev()
                .filter(|&x| U::convertible_from(x)),
            )
            .filter_map(|xs| {
                let mut xs = xs.into_iter();
                let a = xs.next().unwrap();
                let diff = a - xs.next().unwrap();
                if diff.even() {
                    // This happens almost always
                    Some(a - (diff >> 1))
                } else {
                    None
                }
            }),
        ),
    )
}

pub fn exhaustive_signed_gen_var_9<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> It<S> {
    let limit = S::wrapping_from(U::wrapping_from(S::MAX).floor_sqrt());
    Box::new(exhaustive_signed_inclusive_range(-limit, limit))
}

pub fn exhaustive_signed_gen_var_10<T: PrimitiveFloat>() -> It<i64> {
    Box::new(exhaustive_signed_inclusive_range(
        T::MIN_EXPONENT,
        T::MAX_EXPONENT,
    ))
}

pub fn exhaustive_signed_gen_var_11<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| x != T::ZERO && x != T::MIN))
}

pub fn exhaustive_signed_gen_var_12<T: PrimitiveSigned>() -> It<T> {
    Box::new(
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 2))
            .map(|u| (u << 1) | T::ONE),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_pair_gen<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_signeds()))
}

pub fn exhaustive_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_natural_signeds())
            .interleave(exhaustive_pairs_from_single(exhaustive_negative_signeds())),
    )
}

pub fn exhaustive_signed_pair_gen_var_3<T: PrimitiveSigned, U: PrimitiveSigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signeds(),
        exhaustive_signeds(),
    ))
}

struct TakeWhileExtra<I: Iterator, P: Fn(&I::Item) -> bool> {
    xs: I,
    p: P,
    false_seen: bool,
}

impl<I: Iterator, P: Fn(&I::Item) -> bool> Iterator for TakeWhileExtra<I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        loop {
            if let Some(x) = self.xs.next() {
                if (self.p)(&x) {
                    return Some(x);
                } else if self.false_seen {
                    return None;
                }
                self.false_seen = true;
            } else {
                return None;
            }
        }
    }
}

#[inline]
const fn take_while_extra<I: Iterator, P: Fn(&I::Item) -> bool>(
    xs: I,
    p: P,
) -> TakeWhileExtra<I, P> {
    TakeWhileExtra {
        xs,
        p,
        false_seen: false,
    }
}

struct SignedDivisiblePairsGenerator<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveSigned> ExhaustiveDependentPairsYsGenerator<T, T, It<T>>
    for SignedDivisiblePairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, y: &T) -> It<T> {
        // A simple take_while doesn't work. For example, if T is i8 and y is 64, trying to
        // checked-multiply y by the exhaustive signeds gives [Some(0), Some(64), Some(-64), None,
        // Some(-128), None, None, ...], where the first None corresponds to 128, which is not
        // representable as an i8. Doing a take_while would lose the Some(-128). Instead, we use
        // take_while_extra, which is like a take_while, but it waits until it sees a second None to
        // stop iterating.
        let y = *y;
        Box::new(
            take_while_extra(
                exhaustive_signeds().map(move |k| y.checked_mul(k)),
                Option::is_some,
            )
            .map(Option::unwrap),
        )
    }
}

pub fn exhaustive_signed_pair_gen_var_4<T: PrimitiveSigned>() -> It<(T, T)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_nonzero_signeds(),
        SignedDivisiblePairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_signed_pair_gen_var_5<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds(), exhaustive_nonzero_signeds())
            .filter(|&(x, y)| x != T::MIN || y != T::NEGATIVE_ONE),
    )
}

pub fn exhaustive_signed_pair_gen_var_6<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_nonzero_signeds::<T>())
            .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn exhaustive_signed_pair_gen_var_7<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        exhaustive_nonzero_signeds(),
    ))
}

pub fn exhaustive_signed_pair_gen_var_8<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_natural_signeds()))
}

pub fn exhaustive_signed_pair_gen_var_9<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 2))
            .map(|u| (u << 1) | T::ONE),
    ))
}

pub fn exhaustive_signed_pair_gen_var_10<T: PrimitiveSigned>() -> It<(T, T)>
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    Box::new(
        exhaustive_pairs_from_single(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 2))
                .map(|u| (u << 1) | T::ONE),
        )
        .filter(|&(a, b): &(T, T)| a.unsigned_abs().coprime_with(b.unsigned_abs())),
    )
}

pub fn exhaustive_signed_pair_gen_var_11<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>() -> It<(S, S)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_signeds())
            .filter(|&(x, y): &(S, S)| x.unsigned_abs().coprime_with(y.unsigned_abs())),
    )
}

pub fn exhaustive_signed_pair_gen_var_12<T: PrimitiveSigned, U: PrimitiveSigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs(exhaustive_signeds(), exhaustive_signeds()))
}

pub fn exhaustive_signed_pair_gen_var_13<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_signeds())
            .filter(|&(n, k)| T::checked_binomial_coefficient(n, k).is_some()),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_triple_gen<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_signeds()))
}

pub fn exhaustive_signed_triple_gen_var_1<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_signeds())
            .filter(|&(x, y, z)| add_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_signed_triple_gen_var_2<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_signeds())
            .filter(|&(x, y, z)| sub_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_signed_triple_gen_var_3<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_natural_signeds())
            .interleave(exhaustive_triples_from_single(exhaustive_negative_signeds())),
    )
}

struct SignedModEqTriplesInnerGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<(U, U), (S, S), It<(S, S)>>
    for SignedModEqTriplesInnerGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, p: &(U, U)) -> It<(S, S)> {
        let &(m, k) = p;
        if k == U::ZERO {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            let d = m.checked_mul(k).unwrap();
            let d_s = S::wrapping_from(d);
            Box::new(
                exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                    .map(move |n| (n, n.wrapping_add(d_s)))
                    .interleave(
                        exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                            .map(move |n| (n.wrapping_add(d_s), n)),
                    ),
            )
        }
    }
}

struct SignedModEqTriplesGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<S, (S, S), It<(S, S)>>
    for SignedModEqTriplesGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, m: &S) -> It<(S, S)> {
        let m = *m;
        let m_abs = m.unsigned_abs();
        if m == S::ZERO {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(U::ZERO, U::MAX / m_abs)
                        .map(move |k| (m_abs, k)),
                    SignedModEqTriplesInnerGenerator {
                        phantom_u: PhantomData,
                        phantom_s: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_signed_triple_gen_var_4<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() -> It<(S, S, S)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_signeds(),
        SignedModEqTriplesGenerator {
            phantom_u: PhantomData,
            phantom_s: PhantomData,
        },
    ))))
}

pub fn exhaustive_signed_triple_gen_var_5<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_signeds::<T>())
            .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

pub fn exhaustive_signed_triple_gen_var_6<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 2))
            .map(|u| (u << 1) | T::ONE),
    ))
}

pub fn exhaustive_signed_triple_gen_var_7<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 2))
            .map(|u| (u << 1) | T::ONE),
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_quadruple_gen<T: PrimitiveSigned>() -> It<(T, T, T, T)> {
    Box::new(exhaustive_quadruples_from_single(exhaustive_signeds()))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_signed_signed_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, T, T, U)> {
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

struct SignedModPow2EqTriplesInnerGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<(u64, U), (S, S), It<(S, S)>>
    for SignedModPow2EqTriplesInnerGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, p: &(u64, U)) -> It<(S, S)> {
        let &(pow, k) = p;
        if k == U::ZERO {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            let d = k << pow;
            let d_s = S::wrapping_from(d);
            Box::new(
                exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                    .map(move |n| (n, n.wrapping_add(d_s)))
                    .interleave(
                        exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                            .map(move |n| (n.wrapping_add(d_s), n)),
                    ),
            )
        }
    }
}

struct SignedModPow2EqTriplesGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<u64, (S, S), It<(S, S)>>
    for SignedModPow2EqTriplesGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<(S, S)> {
        let pow = *pow;
        if pow >= S::WIDTH {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(U::ZERO, U::MAX >> pow)
                        .map(move |k| (pow, k)),
                    SignedModPow2EqTriplesInnerGenerator {
                        phantom_u: PhantomData,
                        phantom_s: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_signed_signed_unsigned_triple_gen_var_1<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> It<(S, S, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        SignedModPow2EqTriplesGenerator::<U, S> {
            phantom_u: PhantomData,
            phantom_s: PhantomData,
        },
    ))))
}

pub fn exhaustive_signed_signed_unsigned_triple_gen_var_2<T: PrimitiveSigned>() -> It<(T, T, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_signeds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

pub fn exhaustive_signed_signed_unsigned_triple_gen_var_3<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    reshape_1_2_to_3(Box::new(exhaustive_pairs_big_small(
        exhaustive_signeds(),
        exhaustive_pairs(exhaustive_signeds(), exhaustive_positive_primitive_ints()),
    )))
}

// -- (PrimitiveSigned, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_signed_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(T, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_nonzero_signeds::<T>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y), rm)| {
            (x != T::MIN || y != T::NEGATIVE_ONE) && (rm != Exact || x.divisible_by(y))
        }),
    ))
}

fn round_to_multiple_unsigned_helper<T: PrimitiveUnsigned>(x: T, y: T, rm: RoundingMode) -> bool {
    if x == y {
        true
    } else if y == T::ZERO {
        rm == Down || rm == Floor || rm == Nearest
    } else {
        x.div_round(y, rm).0.checked_mul(y).is_some()
    }
}

fn round_to_multiple_signed_helper<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    y: S,
    rm: RoundingMode,
) -> bool {
    let x_abs = x.unsigned_abs();
    let y_abs = y.unsigned_abs();
    if x >= S::ZERO {
        round_to_multiple_unsigned_helper(x_abs, y_abs, rm)
            && S::convertible_from(x_abs.round_to_multiple(y_abs, rm).0)
    } else if !round_to_multiple_unsigned_helper(x_abs, y_abs, -rm) {
        false
    } else {
        let abs_result = x_abs.round_to_multiple(y_abs, -rm).0;
        abs_result == S::MIN.unsigned_abs()
            || S::try_from(abs_result)
                .ok()
                .and_then(CheckedNeg::checked_neg)
                .is_some()
    }
}

pub(crate) fn round_to_multiple_signed_filter_map<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    y: S,
    rm: RoundingMode,
) -> Option<(S, S, RoundingMode)> {
    if rm == Exact {
        x.checked_mul(y).map(|product| (product, y, rm))
    } else if round_to_multiple_signed_helper(x, y, rm) {
        Some((x, y, rm))
    } else {
        None
    }
}

pub fn exhaustive_signed_signed_rounding_mode_triple_gen_var_2<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() -> It<(S, S, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs(exhaustive_signeds(), exhaustive_nonzero_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, y), rm)| round_to_multiple_signed_filter_map::<U, S>(x, y, rm)),
    )
}

pub fn exhaustive_signed_signed_rounding_mode_triple_gen_var_3<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_signeds::<T>(), exhaustive_signeds::<U>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, pow), rm)| {
            rm != Exact || pow <= U::ZERO || x.divisible_by_power_of_2(pow.exact_into())
        }),
    ))
}

pub fn exhaustive_signed_signed_rounding_mode_triple_gen_var_4<
    T: PrimitiveSigned,
    U: PrimitiveSigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_signeds::<T>(), exhaustive_signeds::<U>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, pow), rm)| {
            let pow: i64 = pow.exact_into();
            rm != Exact || pow >= 0 || x.divisible_by_power_of_2(pow.unsigned_abs())
        }),
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_pair_gen<T: PrimitiveSigned, U: PrimitiveUnsigned>() -> It<(T, U)>
{
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_3<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        lex_pairs(
            exhaustive_natural_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )
        .interleave(exhaustive_pairs(
            exhaustive_negative_signeds(),
            exhaustive_unsigneds(),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_4<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_range(0, T::WIDTH),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_5<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        lex_pairs(
            exhaustive_negative_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )
        .interleave(exhaustive_pairs(
            exhaustive_natural_signeds(),
            exhaustive_unsigneds(),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_6<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_7<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_natural_signeds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_8<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_natural_signeds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_9<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_signeds::<T>(), exhaustive_unsigneds::<U>())
            .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

struct SignedDivisibleByP2PairsGenerator<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveSigned> ExhaustiveDependentPairsYsGenerator<u64, T, It<T>>
    for SignedDivisibleByP2PairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<T> {
        let pow = *pow;
        if pow >= T::WIDTH {
            Box::new(once(T::ZERO))
        } else if pow == 0 {
            Box::new(exhaustive_signeds())
        } else {
            Box::new(
                exhaustive_signed_inclusive_range(
                    -T::low_mask(T::WIDTH - pow),
                    T::low_mask(T::WIDTH - pow),
                )
                .map(move |k| k << pow),
            )
        }
    }
}

pub fn exhaustive_signed_unsigned_pair_gen_var_10<T: PrimitiveSigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        SignedDivisibleByP2PairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_11<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_natural_signeds(), exhaustive_unsigneds()).interleave(
            exhaustive_pairs_big_tiny(
                exhaustive_signeds(),
                primitive_int_increasing_inclusive_range(0, T::WIDTH),
            ),
        ),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_12<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_signed_range(T::MIN + T::ONE, T::ONE),
            exhaustive_unsigneds(),
        )
        .interleave(exhaustive_pairs_big_tiny(
            exhaustive_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_13<T: PrimitiveSigned, U: PrimitiveInt>(
) -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(0, U::WIDTH),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_14<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_signed_inclusive_range(
            if T::WIDTH <= u64::WIDTH {
                T::MIN
            } else {
                -T::exact_from(u64::MAX)
            },
            T::saturating_from(u64::MAX),
        ),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_15<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_unsigneds())
            .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_16<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signed_range(T::MIN + T::ONE, T::ZERO),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_17<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_natural_signeds(),
            exhaustive_positive_primitive_ints(),
        )
        .interleave(exhaustive_pairs_big_tiny(
            exhaustive_negative_signeds(),
            exhaustive_unsigneds::<U>()
                .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | U::ONE)),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_18<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_19<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signeds(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, bool) --

pub fn exhaustive_signed_unsigned_bool_triple_gen_var_1<T: PrimitiveSigned>() -> It<(T, u64, bool)>
{
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_unsigneds())
            .map(|(x, y)| (x, y, x < T::ZERO))
            .interleave(
                lex_pairs(
                    exhaustive_signeds(),
                    primitive_int_increasing_range(0, T::WIDTH),
                )
                .map(|(x, y)| (x, y, x >= T::ZERO)),
            ),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_positive_primitive_ints(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .interleave(exhaustive_triples_custom_output(
            exhaustive_signed_inclusive_range(T::MIN, T::ZERO),
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(U::ZERO, U::exact_from(T::WIDTH)),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        ))
        .filter_map(|(x, y, z)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))))
}

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, T, U)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>() -> It<(T, u64, u64, U)> {
    Box::new(
        exhaustive_quadruples_xyyz_custom_output(
            exhaustive_signeds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter_map(|(x, y, z, w): (T, u64, u64, U)| {
            y.checked_add(z).and_then(|new_z| {
                if signed_assign_bits_valid(x, y, new_z, w) {
                    Some((x, y, new_z, w))
                } else {
                    None
                }
            })
        }),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_signeds::<T>(), exhaustive_unsigneds::<u64>()),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, pow), rm)| round_to_multiple_of_power_of_2_filter_map(x, pow, rm)),
    )
}

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_signeds::<T>(), exhaustive_unsigneds::<U>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y), rm)| rm != Exact || x.divisible_by_power_of_2(y.exact_into())),
    ))
}

// var 3 is in malachite-float.

// -- (PrimitiveSigned, RoundingMode) --

pub fn exhaustive_signed_rounding_mode_pair_gen<T: PrimitiveSigned>() -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(exhaustive_signeds(), exhaustive_rounding_modes()))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, RoundingMode)>
{
    Box::new(lex_pairs(
        exhaustive_nonzero_signeds(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_2<T: PrimitiveSigned>() -> It<(T, RoundingMode)>
{
    Box::new(lex_pairs(
        exhaustive_signed_inclusive_range(T::MIN + T::ONE, T::MAX),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_3<T: PrimitiveSigned>() -> It<(T, RoundingMode)>
{
    Box::new(lex_pairs(
        exhaustive_nonzero_signeds().filter(|&x| x != T::MIN),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_4<
    T: PrimitiveSigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> It<(T, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_signeds(), exhaustive_rounding_modes())
            .filter(|&(i, rm)| rm != Exact || U::convertible_from(i)),
    )
}

// -- (PrimitiveSigned, ToSciOptions) --

pub fn exhaustive_signed_to_sci_options_pair_gen<T: PrimitiveSigned>() -> It<(T, ToSciOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        exhaustive_to_sci_options(),
    ))
}

type TSO = ToSciOptions;
pub fn exhaustive_signed_to_sci_options_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, TSO)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_to_sci_options())
            .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (PrimitiveSigned, Vec<bool>) --

struct SignedBoolVecPairGeneratorVar1;

impl<T: PrimitiveSigned>
    ExhaustiveDependentPairsYsGenerator<T, Vec<bool>, LexFixedLengthVecsFromSingle<ExhaustiveBools>>
    for SignedBoolVecPairGeneratorVar1
{
    #[inline]
    fn get_ys(&self, &x: &T) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(
            u64::exact_from(x.to_bits_asc().len()),
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_signed_bool_vec_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_signeds(),
        SignedBoolVecPairGeneratorVar1,
    ))
}

// -- PrimitiveUnsigned --

pub fn exhaustive_unsigned_gen<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_unsigneds())
}

pub fn exhaustive_unsigned_gen_var_1() -> It<u32> {
    Box::new(primitive_int_increasing_range(0, NUMBER_OF_CHARS))
}

pub fn exhaustive_unsigned_gen_var_2<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(1, T::WIDTH))
}

pub fn exhaustive_unsigned_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<U> {
    Box::new(primitive_int_increasing_inclusive_range(
        U::TWO,
        U::saturating_from(T::MAX),
    ))
}

pub fn exhaustive_unsigned_gen_var_5<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(0, T::WIDTH))
}

pub fn exhaustive_unsigned_gen_var_6() -> It<u8> {
    Box::new(
        lex_union3s(
            primitive_int_increasing_inclusive_range(b'0', b'9'),
            primitive_int_increasing_inclusive_range(b'a', b'z'),
            primitive_int_increasing_inclusive_range(b'A', b'Z'),
        )
        .map(Union3::unwrap),
    )
}

pub fn exhaustive_unsigned_gen_var_7<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::power_of_2(T::WIDTH - 1),
        T::MAX,
    ))
}

pub fn exhaustive_unsigned_gen_var_8<T: PrimitiveFloat>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(
        0,
        T::LARGEST_ORDERED_REPRESENTATION,
    ))
}

pub fn exhaustive_unsigned_gen_var_9<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::ZERO,
        T::power_of_2(T::WIDTH - 1),
    ))
}

pub fn exhaustive_unsigned_gen_var_10<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_range(0, T::WIDTH))
}

pub fn exhaustive_unsigned_gen_var_11<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(0, T::WIDTH - 2))
}

pub fn exhaustive_unsigned_gen_var_12<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> It<T> {
    Box::new(exhaustive_unsigneds().filter(|&x| U::convertible_from(x)))
}

pub fn exhaustive_unsigned_gen_var_13<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> It<T> {
    Box::new(
        primitive_int_increasing_inclusive_range(
            T::saturating_from(U::SMALLEST_UNREPRESENTABLE_UINT),
            T::MAX,
        )
        .filter(|&x| !U::convertible_from(x)),
    )
}

pub fn exhaustive_unsigned_gen_var_14<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> It<T> {
    Box::new(
        iter_windows(
            2,
            primitive_int_increasing_inclusive_range(
                T::exact_from(U::SMALLEST_UNREPRESENTABLE_UINT),
                T::MAX,
            )
            .filter(|&x| U::convertible_from(x)),
        )
        .filter_map(|xs| {
            let mut xs = xs.into_iter();
            let a = xs.next().unwrap();
            let diff = xs.next().unwrap() - a;
            if diff.even() {
                // This happens almost always
                Some(a + (diff >> 1))
            } else {
                None
            }
        }),
    )
}

pub fn exhaustive_unsigned_gen_var_15<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::ZERO,
        T::MAX.floor_sqrt(),
    ))
}

pub fn exhaustive_unsigned_gen_var_22<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    )
}

pub fn exhaustive_unsigned_gen_var_23<T: PrimitiveUnsigned>() -> It<u64> {
    let limit = smallest_invalid_value(T::checked_factorial);
    Box::new(primitive_int_increasing_range(0, limit))
}

pub fn exhaustive_unsigned_gen_var_24<T: PrimitiveUnsigned>() -> It<u64> {
    let limit = smallest_invalid_value(T::checked_double_factorial);
    Box::new(primitive_int_increasing_range(0, limit))
}

pub fn exhaustive_unsigned_gen_var_25<T: PrimitiveUnsigned>() -> It<u64> {
    let limit = smallest_invalid_value(T::checked_subfactorial);
    Box::new(primitive_int_increasing_range(0, limit))
}

pub fn exhaustive_unsigned_gen_var_26<T: PrimitiveUnsigned>() -> It<u64> {
    let limit = smallest_invalid_value(T::checked_primorial);
    Box::new(primitive_int_increasing_range(0, limit))
}

pub fn exhaustive_unsigned_gen_var_27<T: PrimitiveUnsigned>() -> It<u64> {
    let limit = smallest_invalid_value(T::checked_product_of_first_n_primes);
    Box::new(primitive_int_increasing_range(0, limit))
}

// -- (PrimitiveUnsigned, PrimitiveInt) --

pub fn exhaustive_unsigned_primitive_int_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_primitive_int_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveInt,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_unsigned_primitive_int_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveInt, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_primitive_int_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
    V: PrimitiveUnsigned,
>() -> It<(T, u64, V)> {
    Box::new(exhaustive_triples(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
        exhaustive_unsigneds(),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveSigned) --

pub fn exhaustive_unsigned_signed_pair_gen<T: PrimitiveUnsigned, U: PrimitiveSigned>() -> It<(T, U)>
{
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_unsigned_signed_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_unsigneds(),
        exhaustive_signeds(),
    ))
}

struct IntegerMantissaAndExponentGenerator<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveFloat> ExhaustiveDependentPairsYsGenerator<(u64, i64), (u64, i64), It<(u64, i64)>>
    for IntegerMantissaAndExponentGenerator<T>
{
    #[inline]
    fn get_ys(&self, p: &(u64, i64)) -> It<(u64, i64)> {
        let &(mantissa, exponent) = p;
        Box::new(exhaustive_natural_signeds().filter_map(move |i| {
            Some((
                mantissa.arithmetic_checked_shl(i)?,
                exponent.checked_sub(i)?,
            ))
        }))
    }
}

pub fn exhaustive_unsigned_signed_pair_gen_var_2<T: PrimitiveFloat>() -> It<(u64, i64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_positive_finite_primitive_floats().map(T::integer_mantissa_and_exponent),
            IntegerMantissaAndExponentGenerator::<T> {
                phantom: PhantomData,
            },
        )
        .map(|p| p.1),
    )
}

// -- (PrimitiveUnsigned, PrimitiveSigned, PrimitiveUnsigned) --

struct ModPowerOf2PairWithExtraSignedGenerator<T: PrimitiveUnsigned, U: PrimitiveSigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

type LongType<T, U> = ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, ExhaustiveSigneds<U>>;

impl<T: PrimitiveUnsigned, U: PrimitiveSigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, U),
        ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, ExhaustiveSigneds<U>>,
    > for ModPowerOf2PairWithExtraSignedGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> LongType<T, U> {
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_signeds(),
        )
    }
}

pub fn exhaustive_unsigned_signed_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2PairWithExtraSignedGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_signed_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(
            exhaustive_unsigneds(),
            exhaustive_signed_inclusive_range(
                if U::WIDTH <= u64::WIDTH {
                    U::MIN
                } else {
                    -U::exact_from(u64::MAX)
                },
                U::saturating_from(u64::MAX),
            ),
        )
        .filter_map(|(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?))),
    )
}

pub fn exhaustive_unsigned_signed_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(exhaustive_unsigneds(), exhaustive_signeds()).filter_map(
            |(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?)),
        ),
    )
}

pub fn exhaustive_unsigned_signed_unsigned_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    reshape_1_2_to_3(Box::new(exhaustive_pairs_big_small(
        exhaustive_unsigneds(),
        exhaustive_pairs(exhaustive_signeds(), exhaustive_positive_primitive_ints()),
    )))
}

// -- (PrimitiveUnsigned, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_unsigned_signed_rounding_mode_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_unsigneds::<T>(), exhaustive_signeds::<U>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, pow), rm)| {
            rm != Exact || pow <= U::ZERO || x.divisible_by_power_of_2(pow.exact_into())
        }),
    ))
}

pub fn exhaustive_unsigned_signed_rounding_mode_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_unsigneds::<T>(), exhaustive_signeds::<U>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, pow), rm)| {
            let pow: i64 = pow.exact_into();
            rm != Exact || pow >= 0 || x.divisible_by_power_of_2(pow.unsigned_abs())
        }),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_1() -> It<(u32, u32)> {
    Box::new(exhaustive_pairs_from_single(
        primitive_int_increasing_range(0, NUMBER_OF_CHARS),
    ))
}

type T1<T, U> = It<(T, U)>;

pub fn exhaustive_unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> T1<T, U>
{
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_range(0, T::WIDTH),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveInt>() -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_5<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::saturating_from(T::MAX)),
    ))
}

// TODO make better
pub fn exhaustive_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x <= y))
}

pub fn exhaustive_unsigned_pair_gen_var_7<
    T: PrimitiveInt + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, V)> {
    Box::new(exhaustive_pairs_big_tiny(
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
        exhaustive_unsigneds(),
    ))
}

struct UnsignedDivisiblePairsGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<T, T, It<T>>
    for UnsignedDivisiblePairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, y: &T) -> It<T> {
        let y = *y;
        Box::new(
            exhaustive_unsigneds()
                .map(move |k| y.checked_mul(k))
                .take_while(Option::is_some)
                .map(Option::unwrap),
        )
    }
}

pub fn exhaustive_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>() -> It<(T, T)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_positive_primitive_ints(),
        UnsignedDivisiblePairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> T1<T, U>
{
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_unsigneds::<T>(),
            exhaustive_positive_primitive_ints::<T>(),
        )
        .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_11<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> T1<T, U>
{
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>())
            .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

struct UnsignedDivisibleByP2PairsGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<u64, T, It<T>>
    for UnsignedDivisibleByP2PairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<T> {
        let pow = *pow;
        if pow >= T::WIDTH {
            Box::new(once(T::ZERO))
        } else {
            Box::new(
                primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - pow))
                    .map(move |k| k << pow),
            )
        }
    }
}

pub fn exhaustive_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        UnsignedDivisibleByP2PairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_13<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_ordered_unique_pairs(exhaustive_unsigneds()))
}

struct ModPowerOf2SingleGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<u64, T, PrimitiveIntIncreasingRange<T>>
    for ModPowerOf2SingleGenerator<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> PrimitiveIntIncreasingRange<T> {
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow))
    }
}

struct ModPowerOf2SingleGenerator2<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<u64, T, PrimitiveIntIncreasingRange<T>>
    for ModPowerOf2SingleGenerator2<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> PrimitiveIntIncreasingRange<T> {
        primitive_int_increasing_inclusive_range(T::ONE, T::low_mask(pow))
    }
}

pub fn exhaustive_unsigned_pair_gen_var_14<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2SingleGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_15<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_unsigneds()
            .map(|x| (T::ZERO, x))
            .interleave(exhaustive_pairs_big_tiny(
                exhaustive_positive_primitive_ints(),
                primitive_int_increasing_range(0, T::WIDTH),
            )),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_16<T: PrimitiveFloat>() -> It<(u64, u64)> {
    Box::new(exhaustive_pairs_from_single(
        primitive_int_increasing_inclusive_range(0, T::LARGEST_ORDERED_REPRESENTATION),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_17<T: PrimitiveUnsigned, U: PrimitiveInt>() -> It<(T, u64)>
{
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(0, U::WIDTH),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range(T::ZERO, T::saturating_from(u64::MAX)),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_19<T: PrimitiveFloat>() -> It<(u64, u64)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_range(0, u64::power_of_2(T::MANTISSA_WIDTH)),
        primitive_int_increasing_range(0, u64::power_of_2(T::EXPONENT_WIDTH)),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_20<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_pair_gen_var_21<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs(exhaustive_unsigneds::<T>(), exhaustive_unsigneds())
            .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_22<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(exhaustive_unsigned_pair_gen_var_14().map(|(x, p)| (x, T::WIDTH - p)))
}

pub fn exhaustive_unsigned_pair_gen_var_23<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_unsigneds::<T>())
            .filter(|&(x, y)| x.checked_lcm(y).is_some()),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_24<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 1), T::MAX),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_25<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(exhaustive_positive_primitive_ints(), exhaustive_unsigneds())
            .filter(|&(x, y)| x != T::ONE || y != T::ZERO),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_26<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
        ModPowerOf2SingleGenerator2 {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_27<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_28<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
                .map(|u| (u << 1) | T::ONE),
        )
        .filter(|&(a, b): &(T, T)| a.coprime_with(b)),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_29<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_unsigneds())
            .filter(|&(x, y): &(T, T)| x.coprime_with(y)),
    )
}

struct MultifactorialNGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<u64, u64, PrimitiveIntIncreasingRange<u64>>
    for MultifactorialNGenerator<T>
{
    #[inline]
    fn get_ys(&self, &m: &u64) -> PrimitiveIntIncreasingRange<u64> {
        let limit = smallest_invalid_value(|n| T::checked_multifactorial(n, m));
        primitive_int_increasing_range(0, limit)
    }
}

pub fn exhaustive_unsigned_pair_gen_var_30<T: PrimitiveUnsigned>() -> It<(u64, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_positive_primitive_ints(),
        MultifactorialNGenerator {
            phantom: PhantomData::<*const T>,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_31<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_unsigneds())
            .filter(|&(n, k)| T::checked_binomial_coefficient(n, k).is_some()),
    )
}

// vars 32 through 36 are in malachite-nz.

// -- (PrimitiveUnsigned, PrimitiveUnsigned, bool) --

pub fn exhaustive_unsigned_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, u64, bool)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_unsigneds(), exhaustive_unsigneds())
            .map(|(x, y)| (x, y, false))
            .interleave(
                lex_pairs(
                    exhaustive_unsigneds(),
                    primitive_int_increasing_range(0, T::WIDTH),
                )
                .map(|(x, y)| (x, y, true)),
            ),
    )
}

pub fn exhaustive_unsigned_unsigned_bool_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(T, U, bool)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs(exhaustive_unsigneds(), exhaustive_positive_primitive_ints()),
        exhaustive_bools(),
    )))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds())
            .filter(|&(x, y, z)| add_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds())
            .filter(|&(x, y, z)| sub_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, U)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_unsigned_triple_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(x, y, z): (T, U, U)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_5<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_unsigneds(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))))
}

struct UnsignedModEqTriplesInnerGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(T, T), (T, T), It<(T, T)>>
    for UnsignedModEqTriplesInnerGenerator<T>
{
    #[inline]
    fn get_ys(&self, p: &(T, T)) -> It<(T, T)> {
        let &(m, k) = p;
        if k == T::ZERO {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            let d = m.checked_mul(k).unwrap();
            Box::new(
                primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                    .map(move |n| (n, n + d))
                    .interleave(
                        primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                            .map(move |n| (n + d, n)),
                    ),
            )
        }
    }
}

struct UnsignedModEqTriplesGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<T, (T, T), It<(T, T)>>
    for UnsignedModEqTriplesGenerator<T>
{
    #[inline]
    fn get_ys(&self, m: &T) -> It<(T, T)> {
        let m = *m;
        if m == T::ZERO {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX / m)
                        .map(move |k| (m, k)),
                    UnsignedModEqTriplesInnerGenerator {
                        phantom: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_unsigneds(),
        UnsignedModEqTriplesGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_7<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds::<T>())
            .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

struct UnsignedModPow2EqTriplesInnerGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(u64, T), (T, T), It<(T, T)>>
    for UnsignedModPow2EqTriplesInnerGenerator<T>
{
    #[inline]
    fn get_ys(&self, p: &(u64, T)) -> It<(T, T)> {
        let &(pow, k) = p;
        if k == T::ZERO {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            let d = k << pow;
            Box::new(
                primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                    .map(move |n| (n, n + d))
                    .interleave(
                        primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                            .map(move |n| (n + d, n)),
                    ),
            )
        }
    }
}

struct UnsignedModPow2EqTriplesGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<u64, (T, T), It<(T, T)>>
    for UnsignedModPow2EqTriplesGenerator<T>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<(T, T)> {
        let pow = *pow;
        if pow >= T::WIDTH {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX >> pow)
                        .map(move |k| (pow, k)),
                    UnsignedModPow2EqTriplesInnerGenerator {
                        phantom: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_unsigned_triple_gen_var_8<T: PrimitiveUnsigned>() -> It<(T, T, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        UnsignedModPow2EqTriplesGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_9<T: PrimitiveUnsigned>() -> It<(T, T, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_unsigneds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

struct ModPowerOf2PairGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, T),
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<T>>,
    > for ModPowerOf2PairGenerator<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> ExhaustivePairs1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(
            T::ZERO,
            T::low_mask(pow),
        ))
    }
}

pub fn exhaustive_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>() -> It<(T, T, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2PairGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_11<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds::<T>())
            .filter_map(|(x, y, z)| Some((x, y, max(x, y).checked_add(z)?.checked_add(T::ONE)?))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_12<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U)> {
    Box::new(exhaustive_triples_xyy_custom_output(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_unsigned_triple_gen_var_13<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx_custom_output(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter_map(|(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(exhaustive_unsigneds(), exhaustive_unsigneds()).filter_map(
            |(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?)),
        ),
    )
}

struct ModPowerOf2PairWithExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, U),
        ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>,
    > for ModPowerOf2PairWithExtraUnsignedGenerator<T, U>
{
    #[inline]
    fn get_ys(
        &self,
        &pow: &u64,
    ) -> ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>> {
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_unsigneds(),
        )
    }
}

pub fn exhaustive_unsigned_triple_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2PairWithExtraUnsignedGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_16<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(U::ZERO, U::saturating_from(u64::MAX)),
        )
        .filter_map(|(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_17<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_triple_gen_var_18<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_xyx(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    ))
}

pub fn exhaustive_unsigned_triple_gen_var_19<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    ))
}

pub fn exhaustive_unsigned_triple_gen_var_20<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, u64, u64, U)> {
    Box::new(
        exhaustive_quadruples_xyyz_custom_output(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter_map(|(x, y, z, w): (T, u64, u64, U)| {
            y.checked_add(z).and_then(|new_z| {
                if unsigned_assign_bits_valid(y, new_z, w) {
                    Some((x, y, new_z, w))
                } else {
                    None
                }
            })
        }),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, T, U)> {
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

struct ModPowerOf2TripleGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, T, T),
        ExhaustiveTriples1Input<PrimitiveIntIncreasingRange<T>>,
    > for ModPowerOf2TripleGenerator<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> ExhaustiveTriples1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(
            T::ZERO,
            T::low_mask(pow),
        ))
    }
}

pub fn exhaustive_unsigned_quadruple_gen_var_3<T: PrimitiveUnsigned>() -> It<(T, T, T, u64)> {
    reshape_3_1_to_4(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2TripleGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_quadruple_gen_var_4<T: PrimitiveUnsigned>() -> It<(T, T, T, T)> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_unsigneds::<T>()).filter_map(
            |(x, y, z, w)| Some((x, y, z, max!(x, y, z).checked_add(w)?.checked_add(T::ONE)?)),
        ),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_5<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>() -> It<(T, T, T, T)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_unsigneds(), exhaustive_positive_primitive_ints()).map(
            |(x_1, x_0, d)| {
                let inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
                (x_1, x_0, d, inv)
            },
        ),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, U, T)> {
    Box::new(
        exhaustive_quadruples_xxyx(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>())
            .filter_map(|(x, y, z, w)| {
                Some((x, y, z, max(x, y).checked_add(w)?.checked_add(T::ONE)?))
            }),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_7<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U, T)> {
    Box::new(
        exhaustive_quadruples_xyyx(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>())
            .filter_map(|(x, y, z, w)| Some((x, y, z, x.checked_add(w)?.checked_add(T::ONE)?))),
    )
}

struct ModPowerOf2TripleWithExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, T, U),
        ExhaustiveTriplesXXY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>,
    > for ModPowerOf2TripleWithExtraUnsignedGenerator<T, U>
{
    #[inline]
    fn get_ys(
        &self,
        &pow: &u64,
    ) -> ExhaustiveTriplesXXY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>
    {
        exhaustive_triples_xxy(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_unsigneds(),
        )
    }
}

pub fn exhaustive_unsigned_quadruple_gen_var_8<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, U, u64)> {
    reshape_3_1_to_4(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2TripleWithExtraUnsignedGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

struct ModPowerOf2QuadrupleWithTwoExtraUnsignedsGenerator<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, U, U),
        ExhaustiveTriplesXYY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>,
    > for ModPowerOf2QuadrupleWithTwoExtraUnsignedsGenerator<T, U>
{
    #[inline]
    fn get_ys(
        &self,
        &pow: &u64,
    ) -> ExhaustiveTriplesXYY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>
    {
        exhaustive_triples_xyy(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_unsigneds(),
        )
    }
}

pub fn exhaustive_unsigned_quadruple_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U, u64)> {
    reshape_3_1_to_4(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOf2QuadrupleWithTwoExtraUnsignedsGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_quadruple_gen_var_10<T: PrimitiveUnsigned>() -> It<(T, T, T, T)> {
    Box::new(exhaustive_quadruples_from_single(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_quadruple_gen_var_11<T: PrimitiveUnsigned>() -> It<(T, T, T, T)> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_unsigneds()).filter(|&(n1, n0, d1, d0)| {
            // conditions: D >= 2^W, N >= D, and N / D < 2^W
            d1 != T::ZERO && (n1 > d1 || n1 == d1 && n0 >= d0)
        }),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_12<T: PrimitiveUnsigned>() -> It<(T, T, T, T)> {
    Box::new(exhaustive_quadruples_xxxy(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
    ))
}

// -- (PrimitiveUnsigned * 6) --

pub fn exhaustive_unsigned_sextuple_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, T, T, T, T, T)> {
    Box::new(exhaustive_sextuples_from_single(exhaustive_unsigneds()))
}

// var 2 is in malachite-nz.

// -- (PrimitiveUnsigned * 8) --

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_octuple_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, T, T, T, T, T, T, T)>
{
    Box::new(exhaustive_octuples_from_single(exhaustive_unsigneds()))
}

// -- (PrimitiveUnsigned * 9) --

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_nonuple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, T, T, T, T, T, T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_triples_from_single(exhaustive_unsigneds()))
            .map(|((a, b, c), (d, e, f), (g, h, i))| (a, b, c, d, e, f, g, h, i)),
    )
}

// -- (PrimitiveUnsigned * 12) --

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_duodecuple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, T, T, T, T, T, T, T, T, T, T, T)> {
    Box::new(exhaustive_duodecuples_from_single(exhaustive_unsigneds()))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs(
                exhaustive_unsigneds::<T>(),
                exhaustive_positive_primitive_ints::<T>(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y), rm)| rm != Exact || x.divisible_by(y)),
    ))
}

pub(crate) fn round_to_multiple_unsigned_filter_map<T: PrimitiveUnsigned>(
    x: T,
    y: T,
    rm: RoundingMode,
) -> Option<(T, T, RoundingMode)> {
    if x == y {
        Some((x, y, rm))
    } else if y == T::ZERO {
        if rm == Floor || rm == Down || rm == Nearest {
            Some((x, y, rm))
        } else {
            None
        }
    } else if rm != Exact {
        x.div_round(y, rm).0.checked_mul(y).map(|_| (x, y, rm))
    } else {
        x.checked_mul(y).map(|product| (product, y, rm))
    }
}

pub(crate) fn round_to_multiple_of_power_of_2_filter_map<T: PrimitiveInt>(
    n: T,
    u: u64,
    rm: RoundingMode,
) -> Option<(T, u64, RoundingMode)> {
    if n == T::ZERO || rm != Exact {
        n.shr_round(u, rm)
            .0
            .arithmetic_checked_shl(u)
            .map(|_| (n, u, rm))
    } else {
        n.arithmetic_checked_shl(u).map(|shifted| (shifted, u, rm))
    }
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(T, T, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs(
                exhaustive_unsigneds::<T>(),
                exhaustive_positive_primitive_ints::<T>(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, y), rm)| round_to_multiple_unsigned_filter_map(x, y, rm)),
    )
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<u64>()),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, pow), rm)| round_to_multiple_of_power_of_2_filter_map(x, pow, rm)),
    )
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_5<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y), rm)| rm != Exact || x.divisible_by_power_of_2(y.exact_into())),
    ))
}

// var 6 is in malachite-float.

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

struct UnsignedUnsignedBoolVecTripleGeneratorVar1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (T, u64),
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for UnsignedUnsignedBoolVecTripleGeneratorVar1
{
    #[inline]
    fn get_ys(&self, &(x, log_base): &(T, u64)) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(
            x.significant_bits().div_round(log_base, Ceiling).0,
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_unsigned_unsigned_bool_vec_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(T, u64, Vec<bool>)> {
    reshape_2_1_to_3(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        lex_pairs(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(1, U::WIDTH),
        ),
        UnsignedUnsignedBoolVecTripleGeneratorVar1,
    )))
}

// -- (PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_unsigned_rounding_mode_pair_gen<T: PrimitiveUnsigned>() -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_unsigned_rounding_mode_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: ConvertibleFrom<T> + PrimitiveFloat,
>() -> It<(T, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_unsigneds(), exhaustive_rounding_modes())
            .filter(move |&(u, rm)| rm != Exact || U::convertible_from(u)),
    )
}

// -- (PrimitiveUnsigned, String) --

pub fn valid_digit_chars(base: u8) -> Vec<char> {
    let mut chars = Vec::new();
    if base <= 10 {
        chars.extend('0'..char::from(base + b'0'));
    } else {
        chars.extend('0'..='9');
        chars.extend('a'..char::from(base - 10 + b'a'));
        chars.extend('A'..char::from(base - 10 + b'A'));
    }
    chars
}

struct DigitStringGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        u8,
        String,
        StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>,
    > for DigitStringGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &base: &u8,
    ) -> StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>
    {
        assert!((2..=36).contains(&base));
        strings_from_char_vecs(exhaustive_vecs_min_length(
            1,
            valid_digit_chars(base).into_iter(),
        ))
    }
}

pub fn exhaustive_unsigned_string_pair_gen_var_1() -> It<(u8, String)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(2, 36),
        DigitStringGenerator,
    ))
}

pub fn exhaustive_unsigned_string_pair_gen_var_2() -> It<(u8, String)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range(2, 36),
        exhaustive_strings(),
    ))
}

struct TargetedIntegerFromStringBaseInputs {
    neg: bool,
    u: u8,
    s: String,
    uss: It<(u8, String)>,
}

impl Iterator for TargetedIntegerFromStringBaseInputs {
    type Item = (u8, String);

    fn next(&mut self) -> Option<(u8, String)> {
        Some(if self.neg {
            self.neg = false;
            let next = self.uss.next().unwrap();
            self.u = next.0;
            self.s = next.1;
            (self.u, self.s.clone())
        } else {
            self.neg = true;
            let mut s = '-'.to_string();
            s.push_str(&self.s);
            (self.u, s)
        })
    }
}

pub fn exhaustive_unsigned_string_pair_gen_var_3() -> It<(u8, String)> {
    Box::new(TargetedIntegerFromStringBaseInputs {
        neg: true,
        u: 0,
        s: String::new(),
        uss: exhaustive_unsigned_string_pair_gen_var_1(),
    })
}

// -- (PrimitiveUnsigned, ToSciOptions) --

pub fn exhaustive_unsigned_to_sci_options_pair_gen<T: PrimitiveUnsigned>() -> It<(T, ToSciOptions)>
{
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_to_sci_options(),
    ))
}

pub fn exhaustive_unsigned_to_sci_options_pair_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, ToSciOptions)> {
    Box::new(
        exhaustive_pairs(exhaustive_unsigneds::<T>(), exhaustive_to_sci_options())
            .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (PrimitiveUnsigned, Vec<bool>) --

struct UnsignedBoolVecPairGeneratorVar1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<T, Vec<bool>, LexFixedLengthVecsFromSingle<ExhaustiveBools>>
    for UnsignedBoolVecPairGeneratorVar1
{
    #[inline]
    fn get_ys(&self, &x: &T) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(x.significant_bits(), exhaustive_bools())
    }
}

pub fn exhaustive_unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_unsigneds(),
        UnsignedBoolVecPairGeneratorVar1,
    ))
}

// -- RationalSequence<PrimitiveUnsigned> --

pub fn exhaustive_unsigned_rational_sequence_gen<T: PrimitiveUnsigned>() -> It<RationalSequence<T>>
{
    Box::new(exhaustive_rational_sequences(exhaustive_unsigneds()))
}

// -- (RationalSequence<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_rational_sequence_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(RationalSequence<T>, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_rational_sequences(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_rational_sequence_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(RationalSequence<T>, usize)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_rational_sequences(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )
        .filter(|&(ref xs, i)| {
            if let Some(len) = xs.len() {
                i < len
            } else {
                true
            }
        }),
    )
}

// -- (RationalSequence<PrimitiveUnsigned>, RationalSequence<PrimitiveUnsigned>) --

pub fn exhaustive_unsigned_rational_sequence_pair_gen<T: PrimitiveUnsigned>(
) -> It<(RationalSequence<T>, RationalSequence<T>)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_rational_sequences(
        exhaustive_unsigneds(),
    )))
}

// -- RationalSequence<PrimitiveUnsigned> * 3 --

pub fn exhaustive_unsigned_rational_sequence_triple_gen<T: PrimitiveUnsigned>() -> It<(
    RationalSequence<T>,
    RationalSequence<T>,
    RationalSequence<T>,
)> {
    Box::new(exhaustive_triples_from_single(
        exhaustive_rational_sequences(exhaustive_unsigneds()),
    ))
}

// -- RoundingMode --

pub fn exhaustive_rounding_mode_gen() -> It<RoundingMode> {
    Box::new(exhaustive_rounding_modes())
}

// -- (RoundingMode, RoundingMode) --

pub fn exhaustive_rounding_mode_pair_gen() -> It<(RoundingMode, RoundingMode)> {
    Box::new(lex_pairs_from_single(exhaustive_rounding_modes()))
}

// -- (RoundingMode, RoundingMode, RoundingMode) --

pub fn exhaustive_rounding_mode_triple_gen() -> It<(RoundingMode, RoundingMode, RoundingMode)> {
    Box::new(lex_triples_from_single(exhaustive_rounding_modes()))
}

// -- SciSizeOptions --

pub fn exhaustive_sci_size_options_gen() -> It<SciSizeOptions> {
    Box::new(exhaustive_sci_size_options())
}

// -- String --

pub fn exhaustive_string_gen() -> It<String> {
    Box::new(exhaustive_strings())
}

pub fn exhaustive_string_gen_var_1() -> It<String> {
    Box::new(exhaustive_strings_using_chars(exhaustive_ascii_chars()))
}

pub fn exhaustive_string_gen_var_2() -> It<String> {
    Box::new(exhaustive_strings_using_chars(ROUNDING_MODE_CHARS.chars()))
}

pub fn exhaustive_string_gen_var_3() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        '0'..='9',
    )))
}

struct TargetedIntegerFromStrStringsVar1 {
    neg: bool,
    s: String,
    ss: It<String>,
}

impl Iterator for TargetedIntegerFromStrStringsVar1 {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(if self.neg {
            self.neg = false;
            self.s = self.ss.next().unwrap();
            self.s.clone()
        } else {
            self.neg = true;
            format!("-{}", self.s)
        })
    }
}

pub fn exhaustive_string_gen_var_4() -> It<String> {
    Box::new(TargetedIntegerFromStrStringsVar1 {
        neg: true,
        s: String::new(),
        ss: exhaustive_string_gen_var_3(),
    })
}

pub fn exhaustive_string_gen_var_5() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        '0'..='1',
    )))
}

pub fn exhaustive_string_gen_var_6() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        '0'..='7',
    )))
}

pub fn exhaustive_string_gen_var_7() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        lex_union3s('0'..='9', 'a'..='f', 'A'..='F').map(Union3::unwrap),
    )))
}

pub fn exhaustive_string_gen_var_8() -> It<String> {
    Box::new(
        strings_from_char_vecs(exhaustive_vecs_min_length(
            1,
            lex_union3s('0'..='9', 'a'..='f', 'A'..='F').map(Union3::unwrap),
        ))
        .map(|s| format!("\"0x{s}\"")),
    )
}

struct TargetedIntegerFromStrStringsVar2 {
    neg: bool,
    s: String,
    ss: It<String>,
}

impl Iterator for TargetedIntegerFromStrStringsVar2 {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(if self.neg {
            self.neg = false;
            self.s = self.ss.next().unwrap();
            format!("\"0x{}\"", self.s)
        } else {
            self.neg = true;
            format!("\"-0x{}\"", self.s)
        })
    }
}

pub fn exhaustive_string_gen_var_9() -> It<String> {
    Box::new(TargetedIntegerFromStrStringsVar2 {
        neg: true,
        s: String::new(),
        ss: exhaustive_string_gen_var_7(),
    })
}

pub fn exhaustive_string_gen_var_10() -> It<String> {
    Box::new(exhaustive_strings_using_chars(
        PRIMITIVE_FLOAT_CHARS.chars(),
    ))
}

// vars 11 through 12 are in malachite-q.

pub fn exhaustive_string_gen_var_13() -> It<String> {
    Box::new(exhaustive_strings_using_chars(
        DECIMAL_SCI_STRING_CHARS.chars(),
    ))
}

pub fn exhaustive_string_gen_var_14() -> It<String> {
    Box::new(exhaustive_strings().filter(|s| !large_exponent(s)))
}

pub fn exhaustive_string_gen_var_15() -> It<String> {
    Box::new(
        exhaustive_strings_using_chars(DECIMAL_SCI_STRING_CHARS.chars())
            .filter(|s| !large_exponent(s)),
    )
}

// -- (String, FromSciStringOptions) --

pub fn exhaustive_string_from_sci_string_options_pair_gen() -> It<(String, FromSciStringOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_strings(),
        exhaustive_from_sci_string_options(),
    ))
}

struct SciDigitStringGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        FromSciStringOptions,
        String,
        StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>,
    > for SciDigitStringGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &options: &FromSciStringOptions,
    ) -> StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>
    {
        let base = options.get_base();
        let mut cs = vec!['+', '-', '.'];
        if base < 15 {
            cs.push('e');
            cs.push('E');
        }
        cs.extend(valid_digit_chars(base));
        assert!((2..=36).contains(&base));
        strings_from_char_vecs(exhaustive_vecs_min_length(1, cs.into_iter()))
    }
}

struct SciDigitStringGenerator2;

impl
    ExhaustiveDependentPairsYsGenerator<
        u8,
        String,
        StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>,
    > for SciDigitStringGenerator2
{
    #[inline]
    fn get_ys(
        &self,
        &base: &u8,
    ) -> StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>
    {
        let mut cs = vec!['+', '-', '.'];
        if base < 15 {
            cs.push('e');
            cs.push('E');
        }
        cs.extend(valid_digit_chars(base));
        assert!((2..=36).contains(&base));
        strings_from_char_vecs(exhaustive_vecs_min_length(1, cs.into_iter()))
    }
}

pub fn exhaustive_string_from_sci_string_options_pair_gen_var_1(
) -> It<(String, FromSciStringOptions)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_from_sci_string_options(),
        SciDigitStringGenerator,
    )))
}

pub fn exhaustive_string_from_sci_string_options_pair_gen_var_2(
) -> It<(String, FromSciStringOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_strings().filter(|s| !large_exponent(s)),
        exhaustive_from_sci_string_options(),
    ))
}

pub fn exhaustive_string_from_sci_string_options_pair_gen_var_3(
) -> It<(String, FromSciStringOptions)> {
    permute_2_1(Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_from_sci_string_options(),
            SciDigitStringGenerator,
        )
        .filter(|(_, s)| !large_exponent(s)),
    ))
}

// -- (String, PrimitiveUnsigned) --

pub fn exhaustive_string_unsigned_pair_gen_var_1() -> It<(String, u8)> {
    Box::new(exhaustive_pairs(
        exhaustive_strings().filter(|s| !large_exponent(s)),
        primitive_int_increasing_inclusive_range(2, 36),
    ))
}

pub fn exhaustive_string_unsigned_pair_gen_var_2() -> It<(String, u8)> {
    permute_2_1(Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            primitive_int_increasing_inclusive_range(2, 36),
            SciDigitStringGenerator2,
        )
        .filter(|(_, s)| !large_exponent(s)),
    ))
}

// -- (String, String) --

pub fn exhaustive_string_pair_gen() -> It<(String, String)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_strings()))
}

pub fn exhaustive_string_pair_gen_var_1() -> It<(String, String)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_strings_using_chars(exhaustive_ascii_chars()),
    ))
}

// -- ToSciOptions --

pub fn exhaustive_to_sci_options_gen() -> It<ToSciOptions> {
    Box::new(exhaustive_to_sci_options())
}

// -- (ToSciOptions, bool) --

pub fn exhaustive_to_sci_options_bool_pair_gen() -> It<(ToSciOptions, bool)> {
    Box::new(lex_pairs(exhaustive_to_sci_options(), exhaustive_bools()))
}

// -- (ToSciOptions, PrimitiveInt) --

pub fn exhaustive_to_sci_options_primitive_int_pair_gen_var_1<T: PrimitiveInt>(
) -> It<(ToSciOptions, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_to_sci_options(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (ToSciOptions, PrimitiveSigned) --

pub fn exhaustive_to_sci_options_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(TSO, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_to_sci_options(),
        exhaustive_negative_signeds(),
    ))
}

// -- (ToSciOptions, PrimitiveUnsigned) --

pub fn exhaustive_to_sci_options_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(ToSciOptions, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_to_sci_options(),
        primitive_int_increasing_inclusive_range(T::TWO, T::from(36u8)),
    ))
}

pub fn exhaustive_to_sci_options_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(ToSciOptions, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_to_sci_options(),
        exhaustive_unsigneds(),
    ))
}

// -- (ToSciOptions, RoundingMode) --

pub fn exhaustive_to_sci_options_rounding_mode_pair_gen() -> It<(ToSciOptions, RoundingMode)> {
    Box::new(exhaustive_pairs(
        exhaustive_to_sci_options(),
        exhaustive_rounding_modes(),
    ))
}

// -- Vec<bool> --

pub fn exhaustive_bool_vec_gen() -> It<Vec<bool>> {
    Box::new(shortlex_vecs(exhaustive_bools()))
}

pub fn exhaustive_bool_vec_gen_var_1<T: PrimitiveUnsigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_vecs_fixed_length_from_single(T::WIDTH, exhaustive_bools()),
                exhaustive_positive_primitive_ints(),
            )
            .map(|(bs, n)| bs.into_iter().chain(repeat_n(false, n)).collect()),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_2<T: PrimitiveSigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH - 1, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_vecs_fixed_length_from_single(T::WIDTH - 1, exhaustive_bools()),
                exhaustive_nonzero_signeds::<isize>(),
            )
            .map(|(bs, n)| {
                bs.into_iter()
                    .chain(repeat_n(n < 0, n.unsigned_abs()))
                    .collect()
            }),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_3<T: PrimitiveUnsigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_vecs_fixed_length_from_single(T::WIDTH, exhaustive_bools()),
                exhaustive_positive_primitive_ints(),
            )
            .map(|(bs, n)| repeat_n(false, n).chain(bs).collect()),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_4<T: PrimitiveSigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH - 1, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_vecs_fixed_length_from_single(T::WIDTH - 1, exhaustive_bools()),
                exhaustive_nonzero_signeds::<isize>(),
            )
            .map(|(bs, n)| repeat_n(n < 0, n.unsigned_abs()).chain(bs).collect()),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_5() -> It<Vec<bool>> {
    Box::new(shortlex_vecs_min_length(1, exhaustive_bools()).filter(|bs| bs.iter().any(|&b| b)))
}

// -- Vec<PrimitiveUnsigned> --

pub fn exhaustive_unsigned_vec_gen<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(exhaustive_vecs(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_vec_gen_var_1<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
    )
}

pub fn exhaustive_unsigned_vec_gen_var_2<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
    )
}

pub fn exhaustive_unsigned_vec_gen_var_3<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(exhaustive_vecs(exhaustive_unsigneds()).filter(|xs| xs.last() != Some(&T::ZERO)))
}

pub fn exhaustive_unsigned_vec_gen_var_4<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(exhaustive_vecs_min_length(1, exhaustive_unsigneds()))
}

// var 5 is in malachite-nz.

pub fn exhaustive_unsigned_vec_gen_var_6<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(exhaustive_vecs_min_length(2, exhaustive_unsigneds()))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveInt) --

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        primitive_int_increasing_inclusive_range(U::exact_from(3), U::MAX),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_5<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_6<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::power_of_2(U::WIDTH - 1), U::MAX),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_7<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        primitive_int_increasing_range(U::ONE, U::power_of_2(U::WIDTH - 1)),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_8<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::power_of_2(U::WIDTH - 1), U::MAX),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_9<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
        primitive_int_increasing_range(U::ONE, U::power_of_2(U::WIDTH - 1)),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_10<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
        primitive_int_increasing_range(U::ONE, U::power_of_2(U::WIDTH - 2)),
    ))
}

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_11<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        primitive_int_increasing_inclusive_range(U::TWO, U::MAX),
    ))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

struct UnsignedVecUnsignedPairGeneratorVar1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > for UnsignedVecUnsignedPairGeneratorVar1
{
    #[inline]
    fn get_ys(
        &self,
        &p: &(usize, usize),
    ) -> ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_vecs_fixed_length_from_single(u64::exact_from(p.1), exhaustive_unsigneds())
    }
}

// TODO generate (usize, usize) pairs better
pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> T1<Vec<T>, usize>
{
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x <= y),
            UnsignedVecUnsignedPairGeneratorVar1,
        )
        .map(|((x, _), zs)| (zs, x)),
    )
}

struct UnsignedVecUnsignedPairGeneratorVar2<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<U>, It<Vec<U>>>
    for UnsignedVecUnsignedPairGeneratorVar2<T, U>
{
    #[inline]
    fn get_ys(&self, &log_base: &u64) -> It<Vec<U>> {
        Box::new(
            exhaustive_vecs_length_inclusive_range(
                0,
                T::WIDTH.div_round(log_base, Ceiling).0,
                primitive_int_increasing_inclusive_range(
                    U::ZERO,
                    U::low_mask(min(T::WIDTH, log_base)),
                ),
            )
            .filter(move |xs| digits_valid::<T, U>(log_base, xs)),
        )
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
        UnsignedVecUnsignedPairGeneratorVar2::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, u64)> {
    Box::new(
        exhaustive_unsigned_vec_unsigned_pair_gen_var_2::<T, U>()
            .map(|(xs, y)| (xs.into_iter().rev().collect(), y)),
    )
}

// var 4 is in malachite-nz

struct ValidDigitsGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned + WrappingFrom<U>, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<U, Vec<T>, It<Vec<T>>> for ValidDigitsGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, base: &U) -> It<Vec<T>> {
        Box::new(exhaustive_vecs(primitive_int_increasing_range(
            T::ZERO,
            T::wrapping_from(*base),
        )))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_5<
    T: PrimitiveUnsigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<(Vec<T>, U)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(U::TWO, U::saturating_from(T::MAX)),
        ValidDigitsGenerator {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(Vec<T>, u64)>
{
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
    ))
}

struct DigitsDesc<T: PrimitiveUnsigned> {
    max_digits: Vec<T>,
    ds: ShortlexVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
}

impl<T: PrimitiveUnsigned> Iterator for DigitsDesc<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        loop {
            let digits = self.ds.next()?;
            if digits.len() < self.max_digits.len() || digits <= self.max_digits {
                return Some(digits);
            }
        }
    }
}

struct DigitsDescGenerator<T: PrimitiveUnsigned, U: Digits<T> + PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: Digits<T> + PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<T, Vec<T>, DigitsDesc<T>> for DigitsDescGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, base: &T) -> DigitsDesc<T> {
        let max_digits = U::MAX.to_digits_desc(base);
        DigitsDesc {
            ds: shortlex_vecs_length_inclusive_range(
                0,
                u64::exact_from(max_digits.len()),
                primitive_int_increasing_range(T::ZERO, *base),
            ),
            max_digits,
        }
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_7<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> It<(Vec<T>, T)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
        DigitsDescGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_8<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> It<(Vec<T>, T)> {
    Box::new(
        exhaustive_unsigned_vec_unsigned_pair_gen_var_7::<T, U>().map(|(mut xs, base)| {
            xs.reverse();
            (xs, base)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

struct PowerOf2DigitsGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        Vec<T>,
        ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
    > for PowerOf2DigitsGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &log_base: &u64,
    ) -> ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>> {
        exhaustive_vecs(primitive_int_increasing_inclusive_range(
            T::ZERO,
            T::low_mask(log_base),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>() -> It<(Vec<T>, u64)>
{
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
        PowerOf2DigitsGenerator,
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_11<T: PrimitiveUnsigned>() -> It<(Vec<T>, u64)>
{
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_12<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::TWO, U::saturating_from(T::MAX)),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_13<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_14<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_15<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )
        .filter(|(xs, y)| *y < U::exact_from(xs.len() << T::LOG_WIDTH)),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_16<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_17<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
        exhaustive_unsigneds(),
    ))
}

// vars 18 through 20 are in malachite-nz.

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_21<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> It<(Vec<T>, u64)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_range(1, U::WIDTH),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_22<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> It<(Vec<T>, u64)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
        primitive_int_increasing_range(1, U::WIDTH),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(Vec<T>, U, V)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ))
}

struct UnsignedVecUnsignedUnsignedTripleGeneratorVar2;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<T>,
        ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
    > for UnsignedVecUnsignedUnsignedTripleGeneratorVar2
{
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(usize, usize),
    ) -> ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>> {
        exhaustive_vecs_min_length(u64::exact_from(i * j), exhaustive_unsigneds())
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, usize, usize)> {
    reshape_1_2_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
        UnsignedVecUnsignedUnsignedTripleGeneratorVar2,
    ))))
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(xs, y, z): (Vec<T>, U, U)| y.checked_add(z).map(|new_z| (xs, y, new_z))),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(xs, y, z): (Vec<T>, U, U)| y.checked_add(z).map(|new_z| (xs, y, new_z))),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_positive_primitive_ints(),
    ))
}

// var 6 is in malachite-nz.

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_7<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, T, T)> {
    Box::new(exhaustive_triples(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_8<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// var 9 is in malachite-nz.

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, T, T)> {
    Box::new(exhaustive_triples(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - 1))
            .map(|u| (u << 1) | T::ONE),
        exhaustive_unsigneds(),
    ))
}

// vars 11 through 12 are in malachite-nz.

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_13<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// var 14 is in malachite-nz.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_unsigned_vec_unsigned_rounding_mode_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U, RoundingMode)> {
    Box::new(exhaustive_triples(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_unsigneds(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_rounding_mode_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, U, RoundingMode)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
        exhaustive_unsigneds(),
        exhaustive_rounding_modes(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ))
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub fn exhaustive_unsigned_vec_pair_gen<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_vecs(
        exhaustive_unsigneds(),
    )))
}

pub struct UnsignedVecPairLenGenerator1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>),
        ExhaustivePairs<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecPairLenGenerator1
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(u64, u64),
    ) -> ExhaustivePairs<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_pairs(
            exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds()),
        )
    }
}

pub fn exhaustive_unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_vecs_min_length(
        1,
        exhaustive_unsigneds(),
    )))
}

pub fn exhaustive_unsigned_vec_pair_gen_var_4<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_vecs_min_length(1, exhaustive_unsigneds())).filter(
            |(ref xs, ref es)| {
                !xs.is_empty()
                    && (es.len() > 1 || es.len() == 1 && es[0] > T::ONE)
                    && *es.last().unwrap() != T::ZERO
            },
        ),
    )
}

struct UnsignedVecSqrtRemGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, u64), (Vec<T>, Vec<T>), It<(Vec<T>, Vec<T>)>>
    for UnsignedVecSqrtRemGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(&self, &(out_len, len): &(u64, u64)) -> It<(Vec<T>, Vec<T>)> {
        Box::new(
            exhaustive_triples(
                exhaustive_vecs_fixed_length_from_single(out_len, exhaustive_unsigneds()),
                exhaustive_vecs_fixed_length_from_single(len - 1, exhaustive_unsigneds()),
                primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 2), T::MAX),
            )
            .map(move |(out, mut ns, n_hi)| {
                ns.insert((usize::exact_from(out_len) << 1) - 1, n_hi);
                (out, ns)
            }),
        )
    }
}

pub fn exhaustive_unsigned_vec_pair_gen_var_5<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let n = x.checked_add(2)?;
                let len: u64 = n.arithmetic_checked_shl(1)?;
                let len = len.checked_add(y)?;
                Some((n, len))
            }),
            UnsignedVecSqrtRemGenerator,
        )
        .map(|p| p.1),
    )
}

struct UnsignedVecSqrtGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, u64), (Vec<T>, Vec<T>), It<(Vec<T>, Vec<T>)>>
    for UnsignedVecSqrtGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(&self, &(out_len, len): &(u64, u64)) -> It<(Vec<T>, Vec<T>)> {
        Box::new(
            exhaustive_triples(
                exhaustive_vecs_fixed_length_from_single(out_len, exhaustive_unsigneds()),
                exhaustive_vecs_fixed_length_from_single(len - 1, exhaustive_unsigneds()),
                exhaustive_positive_primitive_ints(),
            )
            .map(move |(out, mut ns, n_hi)| {
                ns.push(n_hi);
                (out, ns)
            }),
        )
    }
}

pub fn exhaustive_unsigned_vec_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let in_len = x.checked_add(1)?;
                let mut out_len: u64 = in_len.shr_round(1, Ceiling).0;
                out_len = out_len.checked_add(y)?;
                Some((out_len, in_len))
            }),
            UnsignedVecSqrtGenerator,
        )
        .map(|p| p.1),
    )
}

struct UnsignedVecPairSameLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (Vec<T>, Vec<T>),
        ExhaustivePairs1Input<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>,
    > for UnsignedVecPairSameLenGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &len: &u64,
    ) -> ExhaustivePairs1Input<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>
    {
        exhaustive_pairs_from_single(exhaustive_vecs_fixed_length_from_single(
            len,
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_pair_gen_var_7<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_unsigneds(),
            UnsignedVecPairSameLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_8<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_vecs(exhaustive_unsigneds()).filter(|xs| xs.last() != Some(&T::ZERO)),
    ))
}

pub fn exhaustive_unsigned_vec_pair_gen_var_9<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).filter(|xs| !slice_test_zero(xs)),
    ))
}

pub struct UnsignedVecPairLenGenerator2;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>),
        ExhaustivePairs<Vec<T>, It<Vec<T>>, Vec<T>, It<Vec<T>>>,
    > for UnsignedVecPairLenGenerator2
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(u64, u64),
    ) -> ExhaustivePairs<Vec<T>, It<Vec<T>>, Vec<T>, It<Vec<T>>> {
        exhaustive_pairs(
            Box::new(
                exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds())
                    .filter(|xs| !slice_test_zero(xs)),
            ),
            Box::new(
                exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds())
                    .filter(|xs| !slice_test_zero(xs)),
            ),
        )
    }
}

pub fn exhaustive_unsigned_vec_pair_gen_var_10<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator2,
        )
        .map(|p| p.1),
    )
}

// var 11 is in malachite-nz.

pub fn exhaustive_unsigned_vec_pair_gen_var_12<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            ruler_sequence(),
            // TODO
            exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .filter_map(|(_, (ns, ds))| {
            if *ds.last().unwrap() == T::ZERO {
                None
            } else {
                Some((ns, ds))
            }
        }),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_13<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .filter_map(
            |(_, (xs, ys)): (_, (Vec<T>, Vec<T>))| if ys[0].odd() { Some((xs, ys)) } else { None },
        ),
    )
}

// vars 14 through 15 are in malachite-nz

pub fn exhaustive_unsigned_vec_pair_gen_var_16<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .filter_map(|(_, (mut xs, mut ys))| {
            let last_x = xs.last_mut().unwrap();
            let last_y = ys.last_mut().unwrap();
            if *last_x == T::MAX || *last_y == T::MAX {
                None
            } else {
                *last_x += T::ONE;
                *last_y += T::ONE;
                Some((xs, ys))
            }
        }),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_17<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .filter_map(|(_, (mut xs, mut ys))| {
            let last_x = xs.last_mut().unwrap();
            let last_y = ys.last_mut().unwrap();
            if *last_x == T::MAX || *last_y == T::MAX {
                None
            } else {
                *last_x += T::ONE;
                *last_y += T::ONE;
                Some((xs, ys))
            }
        }),
    )
}

// var 18 is in malachite-nz.

pub fn exhaustive_unsigned_vec_pair_gen_var_19<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_triples(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 1), T::MAX),
            exhaustive_unsigneds(),
        )
        .map(|(n, d_1, d_0)| (n, vec![d_0, d_1])),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_20<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds::<T>()).filter_map(|mut xs| {
            let last = xs.last_mut().unwrap();
            *last = last.checked_add(T::ONE)?;
            Some(xs)
        }),
    ))
}

pub fn exhaustive_unsigned_vec_pair_gen_var_21<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
    )
}

// vars 22 through 31 are in malachite-nz.

pub fn exhaustive_unsigned_vec_pair_gen_var_32<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_33<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_positive_primitive_ints(),
            UnsignedVecPairSameLenGenerator,
        )
        .filter_map(|(_, (xs, ys)): (_, (Vec<T>, Vec<T>))| {
            if (*xs.last().unwrap() != T::ZERO || *ys.last().unwrap() != T::ZERO) && ys[0].odd() {
                Some((xs, ys))
            } else {
                None
            }
        }),
    )
}

// var 34 is in malachite-nz.

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, bool) --

pub fn exhaustive_unsigned_vec_unsigned_vec_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, bool)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_unsigned_vec_pair_gen_var_7(),
        exhaustive_bools(),
    )))
}

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
        exhaustive_unsigneds(),
    )))
}

// var 2 and 3 are in malachite-nz

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
        exhaustive_unsigneds(),
    )))
}

struct UnsignedVecPairLenGenerator3;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>),
        ExhaustivePairs<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            It<Vec<T>>,
        >,
    > for UnsignedVecPairLenGenerator3
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(u64, u64),
    ) -> ExhaustivePairs<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        It<Vec<T>>,
    > {
        exhaustive_pairs(
            exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds()),
            Box::new(
                exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds())
                    .filter(|xs| !slice_test_zero(xs)),
            ),
        )
    }
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator3,
        )
        .map(|p| p.1),
        exhaustive_unsigneds(),
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_positive_primitive_ints(),
    ))
}

// vars 7 through 8 are in malachite-nz.

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<T>, Vec<T>, U)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_positive_primitive_ints(),
    ))
}

struct UnsignedVecPairLenAndIndexGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64, u64),
        (Vec<T>, Vec<T>, usize),
        It<(Vec<T>, Vec<T>, usize)>,
    > for UnsignedVecPairLenAndIndexGenerator
{
    #[inline]
    fn get_ys(&self, &(x, y, i): &(u64, u64, u64)) -> It<(Vec<T>, Vec<T>, usize)> {
        Box::new(
            exhaustive_pairs(
                exhaustive_vecs_fixed_length_from_single(x, exhaustive_unsigneds()),
                exhaustive_vecs_fixed_length_from_single(y, exhaustive_unsigneds()),
            )
            .map(move |(x, y)| (x, y, usize::exact_from(i))),
        )
    }
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, usize)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(x, y, i)| {
                    let y = y.checked_add(i)?;
                    let x = x.checked_add(y)?;
                    Some((x, y, i))
                },
            ),
            UnsignedVecPairLenAndIndexGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_unsigned_vec_pair_gen_var_7(),
        exhaustive_unsigneds(),
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
        exhaustive_positive_primitive_ints(),
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, Vec<T>, u64)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
        primitive_int_increasing_range(1, U::WIDTH),
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, Vec<T>, u64)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
        primitive_int_increasing_range(1, U::WIDTH),
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, usize)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, i)| {
                let x = x.checked_add(i)?;
                Some((x, x, i))
            }),
            UnsignedVecPairLenAndIndexGenerator,
        )
        .map(|p| p.1),
    )
}

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub struct UnsignedVecTripleXYYLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriplesXYY<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecTripleXYYLenGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(u64, u64),
    ) -> ExhaustiveTriplesXYY<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_triples_xyy(
            exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds()),
        )
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub struct UnsignedVecTripleLenGenerator1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64, u64),
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriples<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecTripleLenGenerator1
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j, k): &(u64, u64, u64),
    ) -> ExhaustiveTriples<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_triples(
            exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(k, exhaustive_unsigneds()),
        )
    }
}

pub struct UnsignedVecQuadrupleLenGenerator1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64, u64, u64),
        (Vec<T>, Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveQuadruples<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecQuadrupleLenGenerator1
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j, k, l): &(u64, u64, u64, u64),
    ) -> ExhaustiveQuadruples<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_quadruples(
            exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(k, exhaustive_unsigneds()),
            exhaustive_vecs_fixed_length_from_single(l, exhaustive_unsigneds()),
        )
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let y = y.checked_add(1)?;
                    let x = x.checked_add(y)?;
                    let o = x.checked_add(y)?.checked_add(o)?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let y = y.checked_add(1)?;
                    let x = x.checked_add(1)?;
                    let o = x.checked_add(y)?.checked_add(o)?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .map(|p| p.1),
    )
}

// vars 4 through 23 are in malachite-nz

pub fn exhaustive_unsigned_vec_triple_gen_var_24<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_25<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let y = y.checked_add(2)?;
                let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_26<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let y = y.checked_add(2)?;
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

struct UnsignedVecTripleXXXLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriples1Input<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>,
    > for UnsignedVecTripleXXXLenGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &i: &u64,
    ) -> ExhaustiveTriples1Input<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>
    {
        exhaustive_triples_from_single(exhaustive_vecs_fixed_length_from_single(
            i,
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_27<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            primitive_int_increasing_inclusive_range::<u64>(2, u64::MAX),
            UnsignedVecTripleXXXLenGenerator,
        )
        .map(|p| p.1),
    )
}

struct UnsignedVecSqrtRemGenerator3;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64, u64),
        (Vec<T>, Vec<T>, Vec<T>),
        It<(Vec<T>, Vec<T>, Vec<T>)>,
    > for UnsignedVecSqrtRemGenerator3
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(&self, &(out_len, rem_len, len): &(u64, u64, u64)) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
        Box::new(
            exhaustive_quadruples(
                exhaustive_vecs_fixed_length_from_single(out_len, exhaustive_unsigneds()),
                exhaustive_vecs_fixed_length_from_single(rem_len, exhaustive_unsigneds()),
                exhaustive_vecs_fixed_length_from_single(len - 1, exhaustive_unsigneds()),
                exhaustive_positive_primitive_ints(),
            )
            .map(move |(out, rs, mut ns, n_hi)| {
                ns.push(n_hi);
                (out, rs, ns)
            }),
        )
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_28<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(x, y, z)| {
                    let in_len = x.checked_add(1)?;
                    let mut out_len: u64 = in_len.shr_round(1, Ceiling).0;
                    out_len = out_len.checked_add(y)?;
                    let rem_len = in_len.checked_add(z)?;
                    Some((out_len, rem_len, in_len))
                },
            ),
            UnsignedVecSqrtRemGenerator3,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_29<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_unsigneds(),
            UnsignedVecTripleXXXLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_30<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(exhaustive_triples_from_single(
        exhaustive_vecs(exhaustive_unsigneds()).filter(|xs| xs.last() != Some(&T::ZERO)),
    ))
}

pub fn exhaustive_unsigned_vec_triple_gen_var_31<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_32<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let o = max(x, y).checked_add(o)?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .map(|p| p.1),
    )
}

struct UnsignedVecTripleLenGenerator2;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64, u64),
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriples<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            It<Vec<T>>,
            Vec<T>,
            It<Vec<T>>,
        >,
    > for UnsignedVecTripleLenGenerator2
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j, k): &(u64, u64, u64),
    ) -> ExhaustiveTriples<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        It<Vec<T>>,
        Vec<T>,
        It<Vec<T>>,
    > {
        exhaustive_triples(
            exhaustive_vecs_fixed_length_from_single(i, exhaustive_unsigneds()),
            Box::new(
                exhaustive_vecs_fixed_length_from_single(j, exhaustive_unsigneds())
                    .filter(|xs| !slice_test_zero(xs)),
            ),
            Box::new(
                exhaustive_vecs_fixed_length_from_single(k, exhaustive_unsigneds())
                    .filter(|xs| !slice_test_zero(xs)),
            ),
        )
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_33<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let x = x.checked_add(1)?;
                    let y = y.checked_add(1)?;
                    let o = o.checked_add(x)?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator2,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_34<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let x = x.checked_add(1)?;
                    let y = y.checked_add(1)?;
                    let o = o.checked_add(max(x, y))?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator2,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_35<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let x = x.checked_add(1)?;
                    let y = y.checked_add(1)?;
                    let o = o.checked_add(min(x, y))?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator2,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_36<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(exhaustive_triples_from_single(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

// vars 37 through 38 are in malachite-nz.

pub fn exhaustive_unsigned_vec_triple_gen_var_39<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(exhaustive_triples_from_single(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

pub fn exhaustive_unsigned_vec_triple_gen_var_40<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(o, x, y)| {
                    let x = x.checked_add(y)?;
                    let o = o.checked_add(x)?;
                    Some((o, x, y))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_41<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_vecs_min_length(2, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
    ))
}

// vars 42 through 49 are in malachite-nz.

pub fn exhaustive_unsigned_vec_triple_gen_var_50<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(x, y, z)| {
                    let y = y.checked_add(1)?;
                    let x = x.checked_add(y)?;
                    let z = z.checked_add(y.arithmetic_checked_shl(1)?)?;
                    Some((x, y, z))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (x, mut y, z)): (_, (Vec<T>, Vec<T>, Vec<T>))| {
            let last_y = y.last_mut().unwrap();
            if last_y.get_highest_bit() {
                return None;
            }
            last_y.set_bit(T::WIDTH - 1);
            Some((x, y, z))
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_51<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(x, y, z)| {
                    let y = y.checked_add(5)?;
                    let x = x.checked_add(y)?;
                    let z = z.checked_add(y.arithmetic_checked_shl(1)?)?;
                    Some((x, y, z))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (x, mut y, z)): (_, (Vec<T>, Vec<T>, Vec<T>))| {
            let last_y = y.last_mut().unwrap();
            if last_y.get_highest_bit() {
                return None;
            }
            last_y.set_bit(T::WIDTH - 1);
            Some((x, y, z))
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_52<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(x, y, z)| {
                    let q = x.checked_add(3)?;
                    let n = y.checked_add(9)?;
                    let d = z.checked_add(6)?;
                    if n < d {
                        return None;
                    }
                    let q_alt = n - d + 1;
                    if (q_alt << 1) > n || q_alt > d {
                        return None;
                    }
                    let n_alt = q_alt << 1;
                    let d_alt = q_alt;
                    if q >= q_alt && d_alt >= 6 && n_alt >= d_alt + 3 && d_alt >= q_alt {
                        Some((q, n, d))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (x, y, mut z)): (_, (Vec<T>, Vec<T>, Vec<T>))| {
            let last_z = z.last_mut().unwrap();
            if last_z.get_highest_bit() {
                return None;
            }
            last_z.set_bit(T::WIDTH - 1);
            Some((x, y, z))
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_53<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_pairs(
                    exhaustive_unsigneds(),
                    primitive_int_increasing_inclusive_range(2, u64::MAX),
                )
                .filter(|&(x, y)| x >= y - 2),
                UnsignedVecPairLenGenerator1,
            ),
            exhaustive_unsigned_pair_gen_var_24(),
        )
        .map(|((_, (q, n)), (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// vars 54 through 56 are in malachite-nz.

pub fn exhaustive_unsigned_vec_triple_gen_var_57<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(r_len, n_len, d_len)| {
                    let d_len = d_len.checked_add(2)?;
                    let r_len = r_len.checked_add(d_len)?;
                    let n_len = n_len.checked_add(d_len)?;
                    Some((r_len, n_len, d_len))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (r, n, mut d)): (_, (Vec<T>, Vec<T>, Vec<T>))| {
            let last_d = d.last_mut().unwrap();
            *last_d = last_d.checked_add(T::ONE)?;
            Some((r, n, d))
        }),
    )
}

// var 58 is in malachite-nz.

pub fn exhaustive_unsigned_vec_triple_gen_var_59<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(xs_len, ys_len, zs_len)| {
                    let ys_len = ys_len.checked_add(2)?;
                    let zs_len = zs_len.checked_add(2)?;
                    let xs_len = xs_len.checked_add(ys_len + zs_len - 1)?;
                    Some((xs_len, ys_len, zs_len))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(
            |(_, (mut xs, mut ys, mut zs)): (_, (Vec<T>, Vec<T>, Vec<T>))| {
                let last_x = xs.last_mut().unwrap();
                *last_x = last_x.checked_add(T::ONE)?;
                let last_y = ys.last_mut().unwrap();
                *last_y = last_y.checked_add(T::ONE)?;
                let last_z = zs.last_mut().unwrap();
                *last_z = last_z.checked_add(T::ONE)?;
                Some((xs, ys, zs))
            },
        ),
    )
}

// var 60 is in malachite-nz.

// -- large types --

pub fn exhaustive_large_type_gen_var_1<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
    )))
}

struct UnsignedVecSqrtRemGenerator2;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>, u64, bool),
        It<(Vec<T>, Vec<T>, u64, bool)>,
    > for UnsignedVecSqrtRemGenerator2
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(&self, &(n, len): &(u64, u64)) -> It<(Vec<T>, Vec<T>, u64, bool)> {
        Box::new(
            exhaustive_pairs(
                exhaustive_vecs_fixed_length_from_single(n, exhaustive_unsigneds()),
                exhaustive_vecs_fixed_length_from_single(len, exhaustive_unsigneds::<T>())
                    .filter(|xs| *xs.last().unwrap() != T::ZERO),
            )
            .map(move |(out, ns)| {
                let shift = ns.last().unwrap().leading_zeros() >> 1;
                (out, ns, shift, len.odd())
            }),
        )
    }
}

pub fn exhaustive_large_type_gen_var_2<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>, u64, bool)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            Box::new(exhaustive_unsigneds::<u64>().filter_map(|x| {
                let len = x.checked_add(9)?;
                let n = len.shr_round(1, Ceiling).0;
                Some((n, len))
            })),
            UnsignedVecSqrtRemGenerator2,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_large_type_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        exhaustive_quadruples_xyyx::<Vec<T>, _, U, _>(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )
        .filter_map(|(x, y, z, w)| Some((x, y, y.checked_add(U::ONE)?.checked_add(z)?, w))),
    )
}

pub fn exhaustive_large_type_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        exhaustive_quadruples_xyyz(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds::<T>())
                .filter(|xs| !slice_test_zero(xs)),
            exhaustive_unsigneds::<U>(),
            exhaustive_vecs(exhaustive_unsigneds::<T>()),
        )
        .filter_map(|(x, y, z, w)| Some((x, y, y.checked_add(U::ONE)?.checked_add(z)?, w))),
    )
}

// vars 5 through 8 are in malachite-nz

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_9<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>, Vec<T>, bool)>
{
    reshape_3_1_to_4(Box::new(lex_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(|(x, y)| {
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
        exhaustive_bools(),
    )))
}

// vars 10 through 21 are in malachite-nz.

pub fn exhaustive_large_type_gen_var_22<T: PrimitiveUnsigned>(
) -> It<(RationalSequence<T>, usize, T, T)> {
    Box::new(
        exhaustive_quadruples_xyzz(
            exhaustive_rational_sequences(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
        )
        .filter(|&(ref xs, index, _, _)| {
            if let Some(len) = xs.len() {
                index < len
            } else {
                true
            }
        }),
    )
}

// vars 23 through 26 are in malachite-nz.

pub fn exhaustive_large_type_gen_var_27<T: PrimitiveUnsigned>() -> It<(bool, Vec<T>, bool, Vec<T>)>
{
    permute_3_1_4_2(reshape_2_1_1_to_4(Box::new(lex_triples_xyy(
        exhaustive_pairs_from_single(
            exhaustive_vecs(exhaustive_unsigneds()).filter(|xs| xs.last() != Some(&T::ZERO)),
        ),
        exhaustive_bools(),
    ))))
}
