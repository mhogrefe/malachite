use generators::common::{permute_2_1, reshape_1_2_to_3, reshape_2_1_to_3, reshape_2_2_to_4, It};
use generators::{
    digits_valid, exhaustive_pairs_big_small, exhaustive_pairs_big_tiny, signed_assign_bits_valid,
    unsigned_assign_bits_valid,
};
use itertools::{repeat_n, Itertools};
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::exhaustive::{exhaustive_ascii_chars, exhaustive_chars};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, DivRound, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_negative_signeds, exhaustive_nonzero_signeds,
    exhaustive_positive_primitive_ints, exhaustive_signed_inclusive_range, exhaustive_signeds,
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
    PrimitiveIntIncreasingRange,
};
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::exhaustive::{exhaustive_strings, exhaustive_strings_using_chars};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_pairs, exhaustive_pairs_from_single,
    exhaustive_quadruples_xyyz_custom_output, exhaustive_triples, exhaustive_triples_custom_output,
    exhaustive_triples_from_single, exhaustive_triples_xxy_custom_output, exhaustive_triples_xyy,
    exhaustive_triples_xyy_custom_output, lex_pairs, lex_pairs_from_single,
    lex_triples_from_single, ExhaustiveDependentPairsYsGenerator, ExhaustivePairs,
    ExhaustiveTriplesXYY,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_fixed_length_vecs_from_single, exhaustive_vecs,
    exhaustive_vecs_length_inclusive_range, exhaustive_vecs_min_length,
    lex_fixed_length_vecs_from_single, shortlex_vecs_length_inclusive_range,
    ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, LexFixedLengthVecsFromSingle,
};
use rounding_modes::ROUNDING_MODE_CHARS;
use std::cmp::min;
use std::iter::Cloned;
use std::marker::PhantomData;
use std::slice::Iter;

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

// -- (PrimitiveSigned, PrimitiveUnsigned) --

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

// -- (PrimitiveSigned, Vec<bool>) --

struct SignedBoolVecPairGeneratorVar1<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveSigned>
    ExhaustiveDependentPairsYsGenerator<
        T,
        Vec<bool>,
        LexFixedLengthVecsFromSingle<Cloned<Iter<'static, bool>>>,
    > for SignedBoolVecPairGeneratorVar1<T>
{
    #[inline]
    fn get_ys(&self, &x: &T) -> LexFixedLengthVecsFromSingle<Cloned<Iter<'static, bool>>> {
        lex_fixed_length_vecs_from_single(
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
        SignedBoolVecPairGeneratorVar1 {
            phantom: PhantomData,
        },
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

//TODO make better
pub fn exhaustive_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x <= y))
}

pub fn exhaustive_unsigned_pair_gen_var_7<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_unsigneds()))
}

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

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

struct UnsignedUnsignedBoolVecTripleGeneratorVar1<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (T, u64),
        Vec<bool>,
        LexFixedLengthVecsFromSingle<Cloned<Iter<'static, bool>>>,
    > for UnsignedUnsignedBoolVecTripleGeneratorVar1<T>
{
    #[inline]
    fn get_ys(
        &self,
        &(x, log_base): &(T, u64),
    ) -> LexFixedLengthVecsFromSingle<Cloned<Iter<'static, bool>>> {
        lex_fixed_length_vecs_from_single(
            x.significant_bits()
                .div_round(log_base, RoundingMode::Ceiling),
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
        UnsignedUnsignedBoolVecTripleGeneratorVar1 {
            phantom: PhantomData,
        },
    )))
}

// -- (PrimitiveUnsigned, Vec<bool>) --

struct UnsignedBoolVecPairGeneratorVar1<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        T,
        Vec<bool>,
        LexFixedLengthVecsFromSingle<Cloned<Iter<'static, bool>>>,
    > for UnsignedBoolVecPairGeneratorVar1<T>
{
    #[inline]
    fn get_ys(&self, &x: &T) -> LexFixedLengthVecsFromSingle<Cloned<Iter<'static, bool>>> {
        lex_fixed_length_vecs_from_single(x.significant_bits(), exhaustive_bools())
    }
}

pub fn exhaustive_unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_unsigneds(),
        UnsignedBoolVecPairGeneratorVar1 {
            phantom: PhantomData,
        },
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

// -- (String, String) --

pub fn exhaustive_string_pair_gen() -> It<(String, String)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_strings()))
}

pub fn exhaustive_string_pair_gen_var_1() -> It<(String, String)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_strings_using_chars(exhaustive_ascii_chars()),
    ))
}

// -- Vec<bool> --

pub fn exhaustive_bool_vec_gen_var_1<T: PrimitiveUnsigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_fixed_length_vecs_from_single(T::WIDTH, exhaustive_bools()),
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
                lex_fixed_length_vecs_from_single(T::WIDTH - 1, exhaustive_bools()),
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
                lex_fixed_length_vecs_from_single(T::WIDTH, exhaustive_bools()),
                exhaustive_positive_primitive_ints(),
            )
            .map(|(bs, n)| repeat_n(false, n).chain(bs.into_iter()).collect()),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_4<T: PrimitiveSigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH - 1, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_fixed_length_vecs_from_single(T::WIDTH - 1, exhaustive_bools()),
                exhaustive_nonzero_signeds::<isize>(),
            )
            .map(|(bs, n)| {
                repeat_n(n < 0, n.unsigned_abs())
                    .chain(bs.into_iter())
                    .collect()
            }),
        ),
    )
}

// -- Vec<PrimitiveUnsigned> --

pub fn exhaustive_unsigned_vec_gen<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(exhaustive_vecs(exhaustive_unsigneds()))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

struct UnsignedVecUnsignedPairGeneratorVar1<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > for UnsignedVecUnsignedPairGeneratorVar1<T>
{
    #[inline]
    fn get_ys(
        &self,
        &p: &(usize, usize),
    ) -> ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_fixed_length_vecs_from_single(u64::exact_from(p.1), exhaustive_unsigneds())
    }
}

//TODO generate (usize, usize) pairs better
pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> T1<Vec<T>, usize>
{
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x <= y),
            UnsignedVecUnsignedPairGeneratorVar1 {
                phantom: PhantomData,
            },
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
                T::WIDTH.div_round(log_base, RoundingMode::Ceiling),
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

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

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

struct UnsignedVecUnsignedUnsignedTripleGeneratorVar2<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<T>,
        ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
    > for UnsignedVecUnsignedUnsignedTripleGeneratorVar2<T>
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
        UnsignedVecUnsignedUnsignedTripleGeneratorVar2 {
            phantom: PhantomData,
        },
    ))))
}

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

struct UnsignedVecPairLenGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

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
    > for UnsignedVecPairLenGenerator<T>
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
            exhaustive_fixed_length_vecs_from_single(i, exhaustive_unsigneds()),
            exhaustive_fixed_length_vecs_from_single(j, exhaustive_unsigneds()),
        )
    }
}

//TODO will be used
pub fn exhaustive_unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator {
                phantom: PhantomData,
            },
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
            //TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator {
                phantom: PhantomData,
            },
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

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator {
                phantom: PhantomData,
            },
        )
        .map(|p| p.1),
        exhaustive_unsigneds(),
    )))
}

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

struct UnsignedVecTripleXYYLenGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

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
    > for UnsignedVecTripleXYYLenGenerator<T>
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
            exhaustive_fixed_length_vecs_from_single(i, exhaustive_unsigneds()),
            exhaustive_fixed_length_vecs_from_single(j, exhaustive_unsigneds()),
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
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator {
                phantom: PhantomData,
            },
        )
        .map(|p| p.1),
    )
}

// -- large types --

pub fn exhaustive_large_type_gen_var_1<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator {
                phantom: PhantomData,
            },
        )
        .map(|p| p.1),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
    )))
}
