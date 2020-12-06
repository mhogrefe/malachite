use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_negative_signeds, exhaustive_positive_primitive_ints,
    exhaustive_signeds, exhaustive_unsigneds, primitive_int_increasing_range,
};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::exhaustive::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_triples_from_single, lex_pairs,
    lex_pairs_from_single, lex_triples_from_single,
};

use generators::common::It;
use generators::exhaustive_pairs_big_tiny;

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

// -- PrimitiveInt --

pub fn exhaustive_primitive_int_gen_var_1<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_positive_primitive_ints())
}

// -- PrimitiveSigned --

pub fn exhaustive_signed_gen<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds())
}

pub fn exhaustive_signed_gen_var_1<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| x != T::MIN))
}

pub fn exhaustive_signed_gen_var_2<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_natural_signeds())
}

pub fn exhaustive_signed_gen_var_3<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| x != T::ZERO && x != T::NEGATIVE_ONE))
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_pair_gen<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_signeds()))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_triple_gen<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_signeds()))
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_pair_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
    ))
}

//TODO signed set_bit
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

// -- PrimitiveUnsigned --

pub fn exhaustive_unsigned_gen<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_unsigneds())
}

pub fn exhaustive_unsigned_gen_var_1() -> It<u32> {
    Box::new(primitive_int_increasing_range(0, NUMBER_OF_CHARS))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_pair_gen<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()))
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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_unsigneds()))
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
