// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::exhaustive::{
    exhaustive_integers, exhaustive_natural_integers, exhaustive_negative_integers,
    exhaustive_nonzero_integers,
};
use crate::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use crate::integer::Integer;
use crate::natural::arithmetic::add::{limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place};
use crate::natural::arithmetic::binomial_coefficient::{
    BIN_GOETGHELUCK_THRESHOLD, BIN_UIUI_RECURSIVE_SMALLDC,
};
use crate::natural::arithmetic::div_exact::{
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use crate::natural::arithmetic::div_mod::{
    limbs_div_mod_barrett_is_len, limbs_div_mod_barrett_scratch_len, limbs_invert_limb,
    limbs_two_limb_inverse_helper,
};
use crate::natural::arithmetic::eq_mod::{
    limbs_eq_limb_mod_limb, limbs_eq_limb_mod_ref_ref, limbs_eq_mod_limb_ref_ref,
    limbs_eq_mod_ref_ref_ref,
};
use crate::natural::arithmetic::gcd::half_gcd::HalfGcdMatrix1;
use crate::natural::arithmetic::mod_mul::limbs_precompute_mod_mul_two_limbs;
use crate::natural::arithmetic::mod_power_of_2::limbs_slice_mod_power_of_2_in_place;
use crate::natural::arithmetic::mod_power_of_2_square::SQRLO_DC_THRESHOLD_LIMIT;
use crate::natural::arithmetic::mul::fft::{
    limbs_mul_greater_to_out_fft_is_valid, limbs_square_to_out_fft_is_valid,
};
use crate::natural::arithmetic::mul::limb::{
    limbs_slice_mul_limb_in_place, limbs_vec_mul_limb_in_place,
};
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::mul::mul_mod::limbs_mul_mod_base_pow_n_minus_1_next_size;
use crate::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
};
use crate::natural::arithmetic::square::{
    limbs_square_to_out_toom_3_input_size_valid, limbs_square_to_out_toom_4_input_size_valid,
    limbs_square_to_out_toom_6_input_size_valid, limbs_square_to_out_toom_8_input_size_valid,
};
use crate::natural::arithmetic::sub::{limbs_sub_greater_in_place_left, limbs_sub_limb_in_place};
use crate::natural::comparison::cmp::limbs_cmp;
use crate::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_per_digit_in_base, GET_STR_PRECOMPUTE_THRESHOLD,
};
use crate::natural::exhaustive::{
    exhaustive_natural_range, exhaustive_natural_range_to_infinity, exhaustive_naturals,
    exhaustive_positive_naturals, ExhaustiveNaturalRange,
};
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::Natural;
use crate::platform::{
    Limb, ODD_CENTRAL_BINOMIAL_OFFSET, ODD_CENTRAL_BINOMIAL_TABLE_LIMIT,
    ODD_FACTORIAL_EXTTABLE_LIMIT, ODD_FACTORIAL_TABLE_LIMIT, SQR_TOOM2_THRESHOLD,
};
use crate::test_util::extra_variadic::{
    exhaustive_quadruples_from_single, exhaustive_quadruples_xxxy,
    exhaustive_quadruples_xxxy_custom_output, exhaustive_quadruples_xyxz,
    exhaustive_quadruples_xyyx, exhaustive_quadruples_xyyz, exhaustive_quintuples_xyyyz,
    exhaustive_sextuples_from_single, exhaustive_triples_from_single, exhaustive_triples_xxy,
    exhaustive_triples_xxy_custom_output, exhaustive_triples_xyx,
};
use crate::test_util::generators::{factors_of_limb_max, limbs_odd_factorial_valid};
use crate::test_util::natural::arithmetic::gcd::{half_gcd_matrix_create, OwnedHalfGcdMatrix};
use itertools::Itertools;
use malachite_base::bools::exhaustive::{exhaustive_bools, ExhaustiveBools};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::iterators::iter_windows;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, CoprimeWith, DivRound, DivisibleBy, DivisibleByPowerOf2, EqMod,
    EqModPowerOf2, Parity, PowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::string::options::exhaustive::exhaustive_to_sci_options;
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, SaturatingFrom, ToSci, WrappingFrom,
};
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_positive_primitive_ints, exhaustive_primitive_floats,
    exhaustive_signeds, exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
    primitive_int_increasing_range, PrimitiveIntIncreasingRange,
};
use malachite_base::num::factorization::prime_sieve::n_to_bit;
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::num::logic::traits::{
    BitAccess, BitConvertible, LeadingZeros, SignificantBits,
};
use malachite_base::rational_sequences::exhaustive::exhaustive_rational_sequences;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_trailing_zeros;
use malachite_base::test_util::generators::common::{
    permute_1_3_2, permute_2_1, reshape_1_2_to_3, reshape_1_3_to_4, reshape_2_1_to_3,
    reshape_2_2_to_4, It,
};
use malachite_base::test_util::generators::exhaustive::{
    exhaustive_unsigned_pair_gen_var_20, exhaustive_unsigned_pair_gen_var_24,
    exhaustive_unsigned_vec_unsigned_pair_gen_var_17, UnsignedVecPairLenGenerator1,
    UnsignedVecPairLenGenerator2, UnsignedVecQuadrupleLenGenerator1,
    UnsignedVecTripleLenGenerator1, UnsignedVecTripleXYYLenGenerator,
};
use malachite_base::test_util::generators::{
    exhaustive_pairs_big_small, exhaustive_pairs_big_tiny,
};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_ordered_unique_pairs, exhaustive_pairs,
    exhaustive_pairs_from_single, exhaustive_triples, exhaustive_triples_custom_output,
    exhaustive_triples_xyy, exhaustive_triples_xyy_custom_output, lex_pairs,
    ExhaustiveDependentPairsYsGenerator,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_vecs, exhaustive_vecs_fixed_length_from_single, exhaustive_vecs_length_range,
    exhaustive_vecs_min_length, lex_vecs_fixed_length_from_single, ExhaustiveVecs,
    LexFixedLengthVecsFromSingle,
};
use num::{BigInt, BigUint};
use std::cmp::{max, Ordering::*};
use std::iter::once;
use std::marker::PhantomData;
use std::ops::{Shl, Shr};

// -- Integer --

pub fn exhaustive_integer_gen() -> It<Integer> {
    Box::new(exhaustive_integers())
}

pub fn exhaustive_integer_gen_var_1<T: PrimitiveFloat>() -> It<Integer>
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    Box::new(
        once(Integer::ZERO).chain(
            lex_pairs(
                exhaustive_positive_float_naturals::<T>(0),
                exhaustive_bools(),
            )
            .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
        ),
    )
}

pub fn exhaustive_integer_gen_var_2<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
) -> It<Integer> {
    Box::new(
        lex_pairs(exhaustive_natural_gen_var_4::<T>(), exhaustive_bools())
            .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
    )
}

pub fn exhaustive_integer_gen_var_3<T: PrimitiveFloat>() -> It<Integer>
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    Box::new(
        lex_pairs(exhaustive_natural_gen_var_5::<T>(), exhaustive_bools())
            .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
    )
}

pub fn exhaustive_integer_gen_var_4() -> It<Integer> {
    Box::new(exhaustive_natural_integers())
}

pub fn exhaustive_integer_gen_var_5<T: PrimitiveUnsigned>() -> It<Integer>
where
    Integer: From<T>,
{
    Box::new(exhaustive_unsigneds::<T>().map(Integer::from))
}

pub fn exhaustive_integer_gen_var_6<T: PrimitiveSigned>() -> It<Integer>
where
    Integer: From<T>,
{
    Box::new(exhaustive_natural_signeds::<T>().map(Integer::from))
}

pub fn exhaustive_integer_gen_var_7() -> It<Integer> {
    Box::new(exhaustive_negative_integers())
}

pub fn exhaustive_integer_gen_var_8() -> It<Integer> {
    Box::new(exhaustive_nonzero_integers())
}

pub fn exhaustive_integer_gen_var_9() -> It<Integer> {
    Box::new(exhaustive_natural_integers().map(|n| (n << 1u32) | Integer::ONE))
}

// -- (Integer, Integer) --

pub fn exhaustive_integer_pair_gen() -> It<(Integer, Integer)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_integers()))
}

pub fn exhaustive_integer_pair_gen_var_1() -> It<(Integer, Integer)> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_nonzero_integers(),
    ))
}

pub fn exhaustive_integer_pair_gen_var_2() -> It<(Integer, Integer)> {
    Box::new(
        exhaustive_pairs(exhaustive_integers(), exhaustive_nonzero_integers())
            .map(|(x, y)| (x * &y, y)),
    )
}

pub fn exhaustive_integer_pair_gen_var_3() -> It<(Integer, Integer)> {
    Box::new(
        exhaustive_pairs(exhaustive_integers(), exhaustive_nonzero_integers())
            .filter(|(x, y)| !x.divisible_by(y)),
    )
}

pub fn exhaustive_integer_pair_gen_var_4() -> It<(Integer, Integer)> {
    Box::new(
        exhaustive_pairs(exhaustive_integers(), exhaustive_natural_integers())
            .map(|(a, n)| (a, (n << 1u32) | Integer::ONE)),
    )
}

pub fn exhaustive_integer_pair_gen_var_5() -> It<(Integer, Integer)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_integers())
            .filter(|(x, y)| x.unsigned_abs_ref().coprime_with(y.unsigned_abs_ref())),
    )
}

pub fn exhaustive_integer_pair_gen_var_6() -> It<(Integer, Integer)> {
    Box::new(
        exhaustive_pairs_from_single(
            exhaustive_natural_integers().map(|n| (n << 1u32) | Integer::ONE),
        )
        .filter(|(x, y)| x.unsigned_abs_ref().coprime_with(y.unsigned_abs_ref())),
    )
}

pub fn exhaustive_integer_pair_gen_var_7() -> It<(Integer, Integer)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_integers(),
        exhaustive_unsigneds::<Limb>().map(Integer::from),
    ))
}

// -- (Integer, Integer, Integer) --

pub fn exhaustive_integer_triple_gen() -> It<(Integer, Integer, Integer)> {
    Box::new(exhaustive_triples_from_single(exhaustive_integers()))
}

pub fn exhaustive_integer_triple_gen_var_1() -> It<(Integer, Integer, Integer)> {
    Box::new(exhaustive_triples_from_single(exhaustive_natural_integers()))
}

pub fn exhaustive_integer_triple_gen_var_2() -> It<(Integer, Integer, Integer)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_integers(), exhaustive_natural_integers())
            .map(|(a, b, n)| (a, b, (n << 1u32) | Integer::ONE)),
    )
}

pub fn exhaustive_integer_triple_gen_var_3() -> It<(Integer, Integer, Integer)> {
    Box::new(
        exhaustive_triples_xyy(exhaustive_integers(), exhaustive_natural_integers())
            .map(|(a, m, n)| (a, (m << 1u32) | Integer::ONE, (n << 1u32) | Integer::ONE)),
    )
}

// -- (Integer, Integer, Integer, PrimitiveUnsigned) --

pub fn exhaustive_integer_integer_integer_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Integer, Integer, Integer, T)> {
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_integers(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Integer, Integer, Natural) --

pub fn exhaustive_integer_integer_natural_triple_gen() -> It<(Integer, Integer, Natural)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_integers(),
        exhaustive_naturals(),
    ))
}

pub fn exhaustive_integer_integer_natural_triple_gen_var_1() -> It<(Integer, Integer, Natural)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_integers(), exhaustive_naturals())
            .map(|(x, y, m)| (x * Integer::from(&m) + &y, y, m)),
    )
}

pub fn exhaustive_integer_integer_natural_triple_gen_var_2() -> It<(Integer, Integer, Natural)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_integers(), exhaustive_naturals())
            .filter(|(x, y, m)| !x.eq_mod(y, m)),
    )
}

// -- (Integer, Integer, PrimitiveFloat) --

pub fn exhaustive_integer_integer_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Integer, Integer, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_integers(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Integer, Integer, PrimitiveSigned) --

pub fn exhaustive_integer_integer_signed_triple_gen<T: PrimitiveSigned>(
) -> It<(Integer, Integer, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_integers(),
        exhaustive_signeds::<T>(),
    ))
}

// -- (Integer, Integer, PrimitiveUnsigned) --

pub fn exhaustive_integer_integer_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Integer, Integer, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_integers(),
        exhaustive_unsigneds::<T>(),
    ))
}

pub fn exhaustive_integer_integer_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Integer, Integer, T)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_integers(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_integer_integer_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Integer, Integer, T)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_integers(),
            exhaustive_unsigneds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

pub fn exhaustive_integer_integer_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(Integer, Integer, T)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_integers(),
            exhaustive_unsigneds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_2(y, pow.exact_into())),
    )
}

// -- (Integer, Integer, RoundingMode) --

pub fn exhaustive_integer_integer_rounding_mode_triple_gen_var_1(
) -> It<(Integer, Integer, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_integers(),
            exhaustive_nonzero_integers(),
            exhaustive_rounding_modes(),
        )
        .map(|(x, y, rm)| {
            if rm == Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub(crate) fn round_to_multiple_integer_filter_map(
    x: Integer,
    y: Integer,
    rm: RoundingMode,
) -> Option<(Integer, Integer, RoundingMode)> {
    if x == y {
        Some((x, y, rm))
    } else if y == 0 {
        if rm == Down || rm == if x >= 0 { Floor } else { Ceiling } || rm == Nearest {
            Some((x, y, rm))
        } else {
            None
        }
    } else if rm == Exact {
        Some((x * &y, y, rm))
    } else {
        Some((x, y, rm))
    }
}

pub fn exhaustive_integer_integer_rounding_mode_triple_gen_var_2(
) -> It<(Integer, Integer, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_integers(),
            exhaustive_nonzero_integers(),
            exhaustive_rounding_modes(),
        )
        .filter_map(|(x, y, rm)| round_to_multiple_integer_filter_map(x, y, rm)),
    )
}

// -- (Integer, Natural) --

pub fn exhaustive_integer_natural_pair_gen() -> It<(Integer, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_naturals(),
    ))
}

// -- (Integer, Natural, Natural) --

pub fn exhaustive_integer_natural_natural_triple_gen() -> It<(Integer, Natural, Natural)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_integers(),
        exhaustive_naturals(),
    ))
}

// -- (Integer, PrimitiveFloat) --

pub fn exhaustive_integer_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Integer, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_integer_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Integer, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_integers(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Integer, PrimitiveSigned) --

pub fn exhaustive_integer_signed_pair_gen<T: PrimitiveSigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_integer_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_integers(),
        exhaustive_signeds(),
    ))
}

// -- (Integer, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_integer_signed_signed_triple_gen<T: PrimitiveSigned>() -> It<(Integer, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_integers(),
        exhaustive_signeds::<T>(),
    ))
}

// -- (Integer, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_integer_signed_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(Integer, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_integers(),
        exhaustive_signeds::<T>(),
        exhaustive_positive_primitive_ints::<U>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Integer, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_integer_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shr<T, Output = Integer>,
{
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_integers(), exhaustive_signeds::<T>()),
            exhaustive_rounding_modes(),
        )
        .map(|((n, i), rm)| {
            (
                if i < T::ZERO && rm == Exact {
                    n >> i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

pub fn exhaustive_integer_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_integers(), exhaustive_signeds::<T>()),
            exhaustive_rounding_modes(),
        )
        .map(|((n, i), rm)| {
            (
                if i > T::ZERO && rm == Exact {
                    n << i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

// -- (Integer, PrimitiveUnsigned) --

pub fn exhaustive_integer_unsigned_pair_gen<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_integer_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(lex_pairs(
        exhaustive_integers(),
        primitive_int_increasing_inclusive_range(T::TWO, T::from(36u8)),
    ))
}

pub fn exhaustive_integer_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_integers(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_integer_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_natural_integers(),
            exhaustive_positive_primitive_ints(),
        )
        .interleave(exhaustive_pairs_big_tiny(
            exhaustive_negative_integers(),
            exhaustive_unsigneds::<T>()
                .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE)),
        )),
    )
}

struct IntegerDivisibleByP2PairsGenerator;

impl ExhaustiveDependentPairsYsGenerator<u64, Integer, It<Integer>>
    for IntegerDivisibleByP2PairsGenerator
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<Integer> {
        let pow = *pow;
        if pow == 0 {
            Box::new(exhaustive_integers())
        } else {
            Box::new(exhaustive_integers().map(move |k| k << pow))
        }
    }
}

pub fn exhaustive_integer_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    permute_2_1(Box::new(
        exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_unsigneds(),
            IntegerDivisibleByP2PairsGenerator,
        )
        .map(|(x, y)| (T::exact_from(x), y)),
    ))
}

pub fn exhaustive_integer_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_unsigneds::<T>())
            .filter(|(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

pub fn exhaustive_integer_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_integers(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Integer, PrimitiveUnsigned, bool) --

pub fn exhaustive_integer_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Integer, T, bool)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_integers(),
        exhaustive_unsigneds(),
        exhaustive_bools(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ))
}

// -- (Integer, PrimitiveUnsigned, Natural) --

pub fn exhaustive_integer_unsigned_natural_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Integer, T, Natural)> {
    Box::new(exhaustive_triples(
        exhaustive_integers(),
        exhaustive_unsigneds(),
        exhaustive_naturals(),
    ))
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned) --

type T1<T> = It<(Integer, T, T)>;
pub fn exhaustive_integer_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> T1<T> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_integers(),
        exhaustive_unsigneds::<T>(),
    ))
}

pub fn exhaustive_integer_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Integer, T, U)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::from(36u8)),
    ))))
}

pub fn exhaustive_integer_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Integer, T, T)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_integers(),
            exhaustive_unsigneds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(x, y, z)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

pub fn exhaustive_integer_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(Integer, T, T)> {
    Box::new(exhaustive_triples_xyy_custom_output(
        exhaustive_integers(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

pub fn exhaustive_integer_unsigned_unsigned_natural_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Integer, T, T, Natural)> {
    Box::new(
        exhaustive_quadruples_xyyz(
            exhaustive_integers(),
            exhaustive_unsigneds::<T>(),
            exhaustive_naturals(),
        )
        .filter(|(_, y, z, _)| y < z),
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_1(
) -> It<(Integer, u64, RoundingMode)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_integers(),
            exhaustive_unsigneds::<u64>(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .map(|(n, u, rm)| {
            if rm == Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_integers(), exhaustive_unsigneds::<T>()),
            exhaustive_rounding_modes(),
        )
        .map(|((n, u), rm)| (if rm == Exact { n << u } else { n }, u, rm)),
    )
}

// var 3 is in malachite-float.

// -- (Integer, RoundingMode) --

pub fn exhaustive_integer_rounding_mode_pair_gen() -> It<(Integer, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_integers(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_integer_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Integer> + PrimitiveFloat,
>() -> It<(Integer, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_integers(), exhaustive_rounding_modes())
            .filter(|&(ref n, rm)| rm != Exact || T::convertible_from(n)),
    )
}

pub fn exhaustive_integer_rounding_mode_pair_gen_var_2() -> It<(Integer, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_nonzero_integers(),
        exhaustive_rounding_modes(),
    ))
}

// -- (Integer, ToSciOptions) --

pub fn exhaustive_integer_to_sci_options_pair_gen() -> It<(Integer, ToSciOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_to_sci_options(),
    ))
}

pub fn exhaustive_integer_to_sci_options_pair_gen_var_1() -> It<(Integer, ToSciOptions)> {
    Box::new(
        exhaustive_pairs(exhaustive_integers(), exhaustive_to_sci_options())
            .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (Integer, Vec<bool>) --

struct IntegerBoolVecPairGenerator1;

impl
    ExhaustiveDependentPairsYsGenerator<
        Integer,
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for IntegerBoolVecPairGenerator1
{
    #[inline]
    fn get_ys(&self, x: &Integer) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(
            u64::exact_from(x.to_twos_complement_limbs_asc().len()),
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_integer_bool_vec_pair_gen_var_1() -> It<(Integer, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_integers(),
        IntegerBoolVecPairGenerator1,
    ))
}

struct IntegerBoolVecPairGenerator2;

impl
    ExhaustiveDependentPairsYsGenerator<
        Integer,
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for IntegerBoolVecPairGenerator2
{
    #[inline]
    fn get_ys(&self, x: &Integer) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(
            u64::exact_from(x.to_bits_asc().len()),
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_integer_bool_vec_pair_gen_var_2() -> It<(Integer, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_integers(),
        IntegerBoolVecPairGenerator2,
    ))
}

// -- Natural --

pub fn exhaustive_natural_gen() -> It<Natural> {
    Box::new(exhaustive_naturals())
}

pub fn exhaustive_natural_gen_var_1() -> It<Natural> {
    Box::new(exhaustive_natural_range_to_infinity(Natural::TWO))
}

pub fn exhaustive_natural_gen_var_2() -> It<Natural> {
    Box::new(exhaustive_positive_naturals())
}

struct ExhaustivePositiveFloatNaturals<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    done: bool,
    exponent: i64,
    limit: u64,
    mantissa: u64,
    max_finite: Natural,
}

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveFloatNaturals<T>
where
    Natural: TryFrom<T>,
{
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.done {
            None
        } else {
            let n: Natural = ExactFrom::exact_from(self.mantissa);
            let n = n << self.exponent;
            if n == self.max_finite {
                self.done = true;
            } else {
                self.mantissa += 1;
                if self.mantissa == self.limit {
                    self.mantissa >>= 1;
                    self.exponent += 1;
                    self.limit = u64::power_of_2(T::MANTISSA_WIDTH + 1);
                }
            }
            Some(n)
        }
    }
}

fn exhaustive_positive_float_naturals<T: PrimitiveFloat>(
    start_exponent: i64,
) -> ExhaustivePositiveFloatNaturals<T>
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    ExhaustivePositiveFloatNaturals {
        phantom: PhantomData,
        done: false,
        exponent: start_exponent,
        limit: u64::power_of_2(T::MANTISSA_WIDTH + 1),
        mantissa: if start_exponent == 0 {
            1
        } else {
            u64::power_of_2(T::MANTISSA_WIDTH)
        },
        max_finite: Natural::exact_from(T::MAX_FINITE),
    }
}

pub fn exhaustive_natural_gen_var_3<T: PrimitiveFloat>() -> It<Natural>
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    Box::new(once(Natural::ZERO).chain(exhaustive_positive_float_naturals::<T>(0)))
}

pub fn exhaustive_natural_gen_var_4<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
) -> It<Natural> {
    Box::new(
        exhaustive_natural_range_to_infinity(
            Natural::power_of_2(T::MANTISSA_WIDTH + 1) | Natural::ONE,
        )
        .filter(|n| !T::convertible_from(n)),
    )
}

pub fn exhaustive_natural_gen_var_5<T: PrimitiveFloat>() -> It<Natural>
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    Box::new(
        iter_windows(2, exhaustive_positive_float_naturals::<T>(1)).filter_map(|xs| {
            let mut xs = xs.into_iter();
            let a = xs.next().unwrap();
            let diff = xs.next().unwrap() - &a;
            if diff.even() {
                // This happens almost always
                Some(a + (diff >> 1))
            } else {
                None
            }
        }),
    )
}

pub fn exhaustive_natural_gen_var_6<T: PrimitiveUnsigned>() -> It<Natural>
where
    Natural: From<T>,
{
    Box::new(exhaustive_unsigneds::<T>().map(Natural::from))
}

pub fn exhaustive_natural_gen_var_7<T: PrimitiveSigned>() -> It<Natural>
where
    Natural: ExactFrom<T>,
{
    Box::new(exhaustive_natural_signeds::<T>().map(Natural::exact_from))
}

pub fn exhaustive_natural_gen_var_8() -> It<Natural> {
    Box::new(exhaustive_naturals().map(|n| (n << 1u32) | Natural::ONE))
}

// -- (Natural, bool) --

pub fn exhaustive_natural_bool_pair_gen() -> It<(Natural, bool)> {
    Box::new(lex_pairs(exhaustive_naturals(), exhaustive_bools()))
}

// -- (Natural, Integer, Natural) --

pub fn exhaustive_natural_integer_natural_triple_gen() -> It<(Natural, Integer, Natural)> {
    Box::new(exhaustive_triples_xyx(
        exhaustive_naturals(),
        exhaustive_integers(),
    ))
}

// -- (Natural, Natural) --

pub fn exhaustive_natural_pair_gen() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_naturals()))
}

pub fn exhaustive_natural_pair_gen_var_1() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_natural_range_to_infinity(Natural::power_of_2(Limb::WIDTH)),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_natural_pair_gen_var_2() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_natural_pair_gen_var_3() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_positive_naturals(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_natural_pair_gen_var_4() -> It<(Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals())
            .map(|(x, y, z)| (x * &y, y * z))
            .unique(),
    )
}

pub fn exhaustive_natural_pair_gen_var_5() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_positive_naturals(),
    ))
}

pub fn exhaustive_natural_pair_gen_var_6() -> It<(Natural, Natural)> {
    Box::new(
        exhaustive_pairs(exhaustive_naturals(), exhaustive_positive_naturals())
            .map(|(x, y)| (x * &y, y)),
    )
}

pub fn exhaustive_natural_pair_gen_var_7() -> It<(Natural, Natural)> {
    Box::new(
        exhaustive_pairs(exhaustive_naturals(), exhaustive_positive_naturals())
            .filter(|(x, y)| !x.divisible_by(y)),
    )
}

pub fn exhaustive_natural_pair_gen_var_8() -> It<(Natural, Natural)> {
    Box::new(exhaustive_ordered_unique_pairs(exhaustive_naturals()))
}

pub fn exhaustive_natural_pair_gen_var_9() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_positive_naturals()))
}

pub fn exhaustive_natural_pair_gen_var_10() -> It<(Natural, Natural)> {
    // TODO
    Box::new(exhaustive_pairs_from_single(exhaustive_naturals()).filter(|(x, y)| x >= y))
}

pub fn exhaustive_natural_pair_gen_var_11() -> It<(Natural, Natural)> {
    Box::new(exhaustive_ordered_unique_pairs(
        exhaustive_positive_naturals(),
    ))
}

pub fn exhaustive_natural_pair_gen_var_12() -> It<(Natural, Natural)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_naturals())
            .map(|(a, n)| (a, (n << 1u32) | Natural::ONE)),
    )
}

pub fn exhaustive_natural_pair_gen_var_13() -> It<(Natural, Natural)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_naturals().map(|n| (n << 1u32) | Natural::ONE))
            .filter(|(x, y)| x.coprime_with(y)),
    )
}

pub fn exhaustive_natural_pair_gen_var_14() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_naturals()).filter(|(x, y)| x.coprime_with(y)))
}

pub fn exhaustive_natural_pair_gen_var_15() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_unsigneds::<Limb>().map(Natural::from),
    ))
}

// -- (Natural, Natural, bool) --

pub fn exhaustive_natural_natural_bool_triple_gen_var_1() -> It<(Natural, Natural, bool)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs(exhaustive_naturals(), exhaustive_positive_naturals()),
        exhaustive_bools(),
    )))
}

// -- (Natural, Natural, Natural) --

pub fn exhaustive_natural_triple_gen() -> It<(Natural, Natural, Natural)> {
    Box::new(exhaustive_triples_from_single(exhaustive_naturals()))
}

pub fn exhaustive_natural_triple_gen_var_1() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals()).map(|(x, y, m)| (x * &m + &y, y, m)),
    )
}

pub fn exhaustive_natural_triple_gen_var_2() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals()).filter(|(x, y, m)| !x.eq_mod(y, m)),
    )
}

pub fn exhaustive_natural_triple_gen_var_3() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals()).map(|(x, y, z)| {
            let z = max(&x, &y) + z + Natural::ONE;
            (x, y, z)
        }),
    )
}

pub fn exhaustive_natural_triple_gen_var_4() -> It<(Natural, Natural, Natural)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_naturals(),
        exhaustive_positive_naturals(),
    ))
}

pub fn exhaustive_natural_triple_gen_var_5() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals()).map(|(x, y, mut z)| {
            z += &x;
            z += Natural::ONE;
            (x, y, z)
        }),
    )
}

pub fn exhaustive_natural_triple_gen_var_6() -> It<(Natural, Natural, Natural)> {
    Box::new(exhaustive_triples_from_single(
        exhaustive_positive_naturals(),
    ))
}

pub fn exhaustive_natural_triple_gen_var_7() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals()).map(|(x, y, z)| (x + &y * &z, y, z)),
    )
}

pub fn exhaustive_natural_triple_gen_var_8() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals())
            .map(|(a, b, n)| (a, b, (n << 1u32) | Natural::ONE)),
    )
}

pub fn exhaustive_natural_triple_gen_var_9() -> It<(Natural, Natural, Natural)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_naturals())
            .map(|(a, m, n)| (a, (m << 1u32) | Natural::ONE, (n << 1u32) | Natural::ONE)),
    )
}

// -- (Natural, Natural, Natural, Natural) --

pub fn exhaustive_natural_quadruple_gen_var_1() -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_naturals()).map(|(x, y, z, w)| {
            let w = max!(&x, &y, &z) + w + Natural::ONE;
            (x, y, z, w)
        }),
    )
}

pub fn exhaustive_natural_quadruple_gen_var_2() -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_naturals()).map(|(x, y, z, mut w)| {
            w += max!(&x, &y);
            w += Natural::ONE;
            (x, y, z, w)
        }),
    )
}

pub fn exhaustive_natural_quadruple_gen_var_3() -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_naturals()).map(|(x, y, z, mut w)| {
            w += &x;
            w += Natural::ONE;
            (x, y, z, w)
        }),
    )
}

// -- (Natural, Natural, Natural, PrimitiveUnsigned) --

pub fn exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Natural, Natural, Natural, T)> {
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_naturals(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_2(
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        exhaustive_quadruples_xxxy(exhaustive_naturals(), exhaustive_unsigneds::<u64>()).map(
            |(x, y, z, mut m)| {
                m += max!(
                    x.significant_bits(),
                    y.significant_bits(),
                    z.significant_bits()
                );
                (x, y, z, m)
            },
        ),
    )
}

pub fn exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_3(
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        exhaustive_quadruples_xxxy(exhaustive_naturals(), exhaustive_unsigneds::<u64>()).map(
            |(x, y, z, mut m)| {
                m += max(x.significant_bits(), y.significant_bits());
                (x, y, z, m)
            },
        ),
    )
}

pub fn exhaustive_natural_natural_natural_unsigned_quadruple_gen_var_4(
) -> It<(Natural, Natural, Natural, u64)> {
    Box::new(
        exhaustive_quadruples_xxxy(exhaustive_naturals(), exhaustive_unsigneds::<u64>()).map(
            |(x, y, z, mut m)| {
                m += x.significant_bits();
                (x, y, z, m)
            },
        ),
    )
}

// -- (Natural, Natural, PrimitiveFloat) --

pub fn exhaustive_natural_natural_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Natural, Natural, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_naturals(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Natural, Natural, PrimitiveSigned) --

pub fn exhaustive_natural_natural_signed_triple_gen<T: PrimitiveSigned>(
) -> It<(Natural, Natural, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_naturals(),
        exhaustive_signeds::<T>(),
    ))
}

pub fn exhaustive_natural_natural_signed_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(Natural, Natural, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs_big_small(
        exhaustive_ordered_unique_pairs(exhaustive_naturals()),
        exhaustive_signeds(),
    )))
}

// -- (Natural, Natural, PrimitiveUnsigned) --

pub fn exhaustive_natural_natural_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Natural, Natural, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_naturals(),
        exhaustive_unsigneds::<T>(),
    ))
}

pub fn exhaustive_natural_natural_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Natural, Natural, T)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_naturals(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_natural_natural_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Natural, Natural, T)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_naturals(),
            exhaustive_unsigneds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

pub fn exhaustive_natural_natural_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(Natural, Natural, T)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_naturals(),
            exhaustive_unsigneds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_2(y, pow.exact_into())),
    )
}

pub fn exhaustive_natural_natural_unsigned_triple_gen_var_4() -> It<(Natural, Natural, u64)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_naturals(), exhaustive_unsigneds::<u64>()).map(
            |(x, y, mut m)| {
                m += max(x.significant_bits(), y.significant_bits());
                (x, y, m)
            },
        ),
    )
}

pub fn exhaustive_natural_natural_unsigned_triple_gen_var_5() -> It<(Natural, Natural, u64)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_naturals(), exhaustive_unsigneds::<u64>()).map(
            |(x, y, mut m)| {
                m += x.significant_bits();
                (x, y, m)
            },
        ),
    )
}

pub fn exhaustive_natural_natural_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> It<(Natural, Natural, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs_big_small(
        exhaustive_ordered_unique_pairs(exhaustive_naturals()),
        exhaustive_unsigneds(),
    )))
}

// -- (Natural, Natural, RoundingMode) --

pub fn exhaustive_natural_natural_rounding_mode_triple_gen_var_1(
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_positive_naturals(),
            exhaustive_rounding_modes(),
        )
        .map(|(x, y, rm)| {
            if rm == Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub(crate) fn round_to_multiple_natural_filter_map(
    x: Natural,
    y: Natural,
    rm: RoundingMode,
) -> Option<(Natural, Natural, RoundingMode)> {
    if x == y {
        Some((x, y, rm))
    } else if y == 0 {
        if rm == Down || rm == Floor || rm == Nearest {
            Some((x, y, rm))
        } else {
            None
        }
    } else if rm == Exact {
        Some((x * &y, y, rm))
    } else {
        Some((x, y, rm))
    }
}

pub fn exhaustive_natural_natural_rounding_mode_triple_gen_var_2(
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_positive_naturals(),
            exhaustive_rounding_modes(),
        )
        .filter_map(|(x, y, rm)| round_to_multiple_natural_filter_map(x, y, rm)),
    )
}

// -- (Natural, PrimitiveFloat) --

pub fn exhaustive_natural_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Natural, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_natural_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Natural, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_naturals(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Natural, PrimitiveInt) --

pub fn exhaustive_natural_primitive_int_pair_gen_var_1<
    T: PrimitiveInt + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_2<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_3<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_4<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_naturals(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Natural, PrimitiveInt, PrimitiveUnsigned) --

pub fn exhaustive_natural_primitive_int_unsigned_triple_gen_var_3<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>() -> It<(Natural, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_naturals(),
        exhaustive_positive_primitive_ints(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Natural, PrimitiveSigned) --

pub fn exhaustive_natural_signed_pair_gen<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_natural_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_natural_signeds(),
    ))
}

pub fn exhaustive_natural_signed_pair_gen_var_2<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_natural_signed_pair_gen_var_3<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_positive_naturals(),
        exhaustive_signeds(),
    ))
}

struct NaturalBitsMultipleOfLimbBitsGenerator;

impl ExhaustiveDependentPairsYsGenerator<u64, Natural, It<Natural>>
    for NaturalBitsMultipleOfLimbBitsGenerator
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<Natural> {
        let p = Natural::power_of_2(pow << Limb::LOG_WIDTH);
        Box::new(exhaustive_natural_range(&p >> 1u32, p))
    }
}

pub fn exhaustive_natural_signed_pair_gen_var_4<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_positive_primitive_ints(),
            NaturalBitsMultipleOfLimbBitsGenerator,
        )
        .map(|p| p.1),
        exhaustive_signeds(),
    ))
}

// -- (Natural, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_natural_signed_signed_triple_gen<T: PrimitiveSigned>() -> It<(Natural, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_naturals(),
        exhaustive_signeds::<T>(),
    ))
}

// -- (Natural, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_natural_signed_unsigned_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(Natural, T, u64)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_naturals(),
            exhaustive_signeds::<T>(),
            exhaustive_unsigneds::<u64>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .map(|(x, y, mut m)| {
            m += x.significant_bits();
            (x, y, m)
        }),
    )
}

pub fn exhaustive_natural_signed_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(Natural, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_naturals(),
        exhaustive_signeds::<T>(),
        exhaustive_positive_primitive_ints::<U>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Natural, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_natural_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shr<T, Output = Natural>,
{
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_naturals(), exhaustive_signeds::<T>()),
            exhaustive_rounding_modes(),
        )
        .map(|((n, i), rm)| {
            (
                if i < T::ZERO && rm == Exact {
                    n >> i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

pub fn exhaustive_natural_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>(
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_naturals(), exhaustive_signeds::<T>()),
            exhaustive_rounding_modes(),
        )
        .map(|((n, i), rm)| {
            (
                if i > T::ZERO && rm == Exact {
                    n << i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

// -- (Natural, PrimitiveUnsigned) --

pub fn exhaustive_natural_unsigned_pair_gen<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(lex_pairs(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::from(36u8)),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_natural_range_to_infinity(Natural::TWO),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_4<T: PrimitiveInt>() -> It<(Natural, u64)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
    ))
}

struct NaturalDivisibleByP2PairsGenerator;

impl ExhaustiveDependentPairsYsGenerator<u64, Natural, It<Natural>>
    for NaturalDivisibleByP2PairsGenerator
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<Natural> {
        let pow = *pow;
        if pow == 0 {
            Box::new(exhaustive_naturals())
        } else {
            Box::new(exhaustive_naturals().map(move |k| k << pow))
        }
    }
}

pub fn exhaustive_natural_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    permute_2_1(Box::new(
        exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_unsigneds(),
            NaturalDivisibleByP2PairsGenerator,
        )
        .map(|(x, y)| (T::exact_from(x), y)),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_unsigneds::<T>())
            .filter(|(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

pub fn exhaustive_natural_unsigned_pair_gen_var_7() -> It<(Natural, u64)> {
    Box::new(
        exhaustive_pairs(exhaustive_naturals(), exhaustive_unsigneds::<u64>()).map(|(x, mut m)| {
            m += x.significant_bits();
            (x, m)
        }),
    )
}

pub fn exhaustive_natural_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_positive_naturals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_naturals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_10() -> It<(Natural, u64)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_positive_naturals(),
            exhaustive_unsigneds::<u64>(),
        )
        .map(|(x, mut m)| {
            m += x.significant_bits();
            (x, m)
        }),
    )
}

// -- (Natural, PrimitiveUnsigned, bool) --

pub fn exhaustive_natural_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Natural, T, bool)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
        exhaustive_bools(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ))
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) --

type T2<T> = It<(Natural, T, T)>;
pub fn exhaustive_natural_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> T2<T> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_naturals(),
        exhaustive_unsigneds::<T>(),
    ))
}

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Natural, T, U)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::from(36u8)),
    ))))
}

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Natural, u64, T)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
    ))))
}

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(Natural, T, T)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_naturals(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(x, y, z): (Natural, T, T)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(Natural, T, T)> {
    Box::new(exhaustive_triples_xyy_custom_output(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> It<(Natural, T, u64)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_naturals(),
            exhaustive_unsigneds::<T>(),
            exhaustive_unsigneds::<u64>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .map(|(x, y, mut m)| {
            m += x.significant_bits();
            (x, y, m)
        }),
    )
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned, Natural) --

pub fn exhaustive_natural_unsigned_unsigned_natural_quadruple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Natural, T, T, Natural)> {
    Box::new(
        exhaustive_quadruples_xyyx(exhaustive_naturals(), exhaustive_unsigneds())
            .filter(|(_, y, z, _)| y < z),
    )
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_natural_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_naturals(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        )
        .map(|((n, u), rm)| {
            if rm == Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

// var 2 is in malachite-float

// -- (Natural, PrimitiveUnsigned, Vec<bool>) --

struct NaturalUnsignedBoolVecPairGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        (Natural, u64),
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for NaturalUnsignedBoolVecPairGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Natural, u64)) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(
            p.0.significant_bits().div_round(p.1, Up).0,
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_natural_unsigned_bool_vec_triple_gen_var_1() -> It<(Natural, u64, Vec<bool>)> {
    reshape_2_1_to_3(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_positive_primitive_ints()),
        NaturalUnsignedBoolVecPairGenerator,
    )))
}

pub fn exhaustive_natural_unsigned_bool_vec_triple_gen_var_2<T: PrimitiveInt>(
) -> It<(Natural, u64, Vec<bool>)> {
    reshape_2_1_to_3(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        lex_pairs(
            exhaustive_naturals(),
            primitive_int_increasing_inclusive_range(1, T::WIDTH),
        ),
        NaturalUnsignedBoolVecPairGenerator,
    )))
}

// -- (Natural, RoundingMode) --

pub fn exhaustive_natural_rounding_mode_pair_gen() -> It<(Natural, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_naturals(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_natural_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>() -> It<(Natural, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_naturals(), exhaustive_rounding_modes())
            .filter(|&(ref n, rm)| rm != Exact || T::convertible_from(n)),
    )
}

pub fn exhaustive_natural_rounding_mode_pair_gen_var_2() -> It<(Natural, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_positive_naturals(),
        exhaustive_rounding_modes(),
    ))
}

// -- (Natural, ToSciOptions) --

pub fn exhaustive_natural_to_sci_options_pair_gen() -> It<(Natural, ToSciOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_to_sci_options(),
    ))
}

pub fn exhaustive_natural_to_sci_options_pair_gen_var_1() -> It<(Natural, ToSciOptions)> {
    Box::new(
        exhaustive_pairs(exhaustive_naturals(), exhaustive_to_sci_options())
            .filter(|(x, options)| x.fmt_sci_valid(*options)),
    )
}

// -- (Natural, Vec<bool>) --

struct NaturalBoolVecPairGenerator1;

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for NaturalBoolVecPairGenerator1
{
    #[inline]
    fn get_ys(&self, x: &Natural) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(x.limb_count(), exhaustive_bools())
    }
}

pub fn exhaustive_natural_bool_vec_pair_gen_var_1() -> It<(Natural, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_naturals(),
        NaturalBoolVecPairGenerator1,
    ))
}

struct NaturalBoolVecPairGenerator2;

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for NaturalBoolVecPairGenerator2
{
    #[inline]
    fn get_ys(&self, x: &Natural) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_vecs_fixed_length_from_single(x.significant_bits(), exhaustive_bools())
    }
}

pub fn exhaustive_natural_bool_vec_pair_gen_var_2() -> It<(Natural, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_naturals(),
        NaturalBoolVecPairGenerator2,
    ))
}

// -- (PrimitiveUnsigned, bool)

pub fn exhaustive_unsigned_bool_pair_gen_var_1() -> It<(usize, bool)> {
    Box::new(
        lex_pairs(exhaustive_unsigneds(), exhaustive_bools())
            .filter(|&(n, b)| limbs_odd_factorial_valid(n, b)),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

// vars 1 through 31 are in malachite-base.

pub fn exhaustive_unsigned_pair_gen_var_32<T: PrimitiveUnsigned>() -> It<(T, T)> {
    // TODO
    Box::new(
        exhaustive_pairs(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(
                T::exact_from(ODD_FACTORIAL_TABLE_LIMIT + 1),
                T::MAX,
            ),
        )
        .filter(|&(n, k)| n >= k),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_33<T: PrimitiveUnsigned>() -> It<(T, T)> {
    // TODO
    Box::new(
        exhaustive_pairs(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(
                T::TWO,
                T::wrapping_from(ODD_FACTORIAL_TABLE_LIMIT),
            ),
        )
        .filter(|&(n, k)| n >= k),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_34<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(
                T::from(4u8),
                T::wrapping_from(ODD_FACTORIAL_EXTTABLE_LIMIT),
            ),
            primitive_int_increasing_inclusive_range(
                T::TWO,
                T::wrapping_from(ODD_FACTORIAL_EXTTABLE_LIMIT - 2),
            ),
        )
        .filter(|&(n, k)| n >= k + T::TWO),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_35<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(
                T::wrapping_from((ODD_CENTRAL_BINOMIAL_OFFSET << 1) + 1),
                T::MAX,
            ),
            primitive_int_increasing_inclusive_range(
                T::wrapping_from((ODD_CENTRAL_BINOMIAL_OFFSET << 1) - 1),
                T::wrapping_from(
                    (if BIN_UIUI_RECURSIVE_SMALLDC {
                        ODD_CENTRAL_BINOMIAL_TABLE_LIMIT
                    } else {
                        ODD_FACTORIAL_TABLE_LIMIT
                    }) << 1,
                ),
            ),
        )
        .filter(|&(n, k)| n >= k + T::TWO),
    )
}

#[allow(clippy::useless_conversion)]
pub fn exhaustive_unsigned_pair_gen_var_36() -> It<(Limb, Limb)> {
    Box::new(
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(
                Limb::wrapping_from(BIN_GOETGHELUCK_THRESHOLD) << 1,
                Limb::MAX,
            ),
            primitive_int_increasing_inclusive_range(
                Limb::wrapping_from(BIN_GOETGHELUCK_THRESHOLD),
                Limb::MAX,
            ),
        )
        .filter(|&(n, k)| {
            n >= k + 5
                && k > (n >> 4)
                && n_to_bit(u64::from(n - k)) < n_to_bit(u64::from(n))
                && k <= n - k
        }),
    )
}

// -- (PrimitiveUnsigned * 6) --

// var 1 is in malachite-base.

pub fn exhaustive_unsigned_sextuple_gen_var_2() -> It<(Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_pairs(
                exhaustive_unsigned_pair_gen_var_20(),
                exhaustive_unsigned_pair_gen_var_24(),
            )
            .filter(|&((n_2, n_1), (d_1, d_0))| n_2 < d_1 || n_2 == d_1 && n_1 < d_0),
            exhaustive_unsigneds(),
        )
        .map(|(((n_2, n_1), (d_1, d_0)), n_0)| {
            (
                n_2,
                n_1,
                n_0,
                d_1,
                d_0,
                limbs_two_limb_inverse_helper(d_1, d_0),
            )
        }),
    )
}

// -- (String, String, String) --

pub fn exhaustive_string_triple_gen_var_1() -> It<(String, String, String)> {
    Box::new(exhaustive_naturals().map(|x| {
        (
            serde_json::to_string(&BigUint::from(&x)).unwrap(),
            serde_json::to_string(&rug::Integer::from(&x)).unwrap(),
            serde_json::to_string(&x).unwrap(),
        )
    }))
}

pub fn exhaustive_string_triple_gen_var_2() -> It<(String, String, String)> {
    Box::new(exhaustive_integers().map(|x| {
        (
            serde_json::to_string(&BigInt::from(&x)).unwrap(),
            serde_json::to_string(&rug::Integer::from(&x)).unwrap(),
            serde_json::to_string(&x).unwrap(),
        )
    }))
}

// var 3 is in malachite-q.

// -- Vec<Integer> --

pub fn exhaustive_integer_vec_gen() -> It<Vec<Integer>> {
    Box::new(exhaustive_vecs(exhaustive_integers()))
}

// -- Vec<Natural> --

pub fn exhaustive_natural_vec_gen() -> It<Vec<Natural>> {
    Box::new(exhaustive_vecs(exhaustive_naturals()))
}

// -- (Vec<Natural>, Integer)

pub fn exhaustive_natural_vec_integer_pair_gen_var_1() -> It<(Vec<Natural>, Integer)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_positive_naturals()),
        exhaustive_integers(),
    ))
}

// -- (Vec<Natural>, Natural)

struct ValidDigitsGenerator;

impl ExhaustiveDependentPairsYsGenerator<Natural, Vec<Natural>, It<Vec<Natural>>>
    for ValidDigitsGenerator
{
    #[inline]
    fn get_ys(&self, base: &Natural) -> It<Vec<Natural>> {
        Box::new(exhaustive_vecs(exhaustive_natural_range(
            Natural::ZERO,
            base.clone(),
        )))
    }
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_1() -> It<(Vec<Natural>, Natural)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_natural_range_to_infinity(Natural::power_of_2(Limb::WIDTH)),
        ValidDigitsGenerator,
    )))
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_2() -> It<(Vec<Natural>, Natural)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_natural_range_to_infinity(Natural::TWO),
        ValidDigitsGenerator,
    )))
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_3() -> It<(Vec<Natural>, Natural)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_naturals()),
        exhaustive_natural_range_to_infinity(Natural::power_of_2(Limb::WIDTH)),
    ))
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_4() -> It<(Vec<Natural>, Natural)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_naturals()),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

// -- (Vec<Natural>, PrimitiveInt) --

pub fn exhaustive_natural_vec_primitive_int_pair_gen_var_1<T: PrimitiveInt>(
) -> It<(Vec<Natural>, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_naturals()),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Vec<Natural>, u64) --

struct PowerOfTwoDigitsGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        u64,
        Vec<Natural>,
        ExhaustiveVecs<Natural, PrimitiveIntIncreasingRange<u64>, ExhaustiveNaturalRange>,
    > for PowerOfTwoDigitsGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &log_base: &u64,
    ) -> ExhaustiveVecs<Natural, PrimitiveIntIncreasingRange<u64>, ExhaustiveNaturalRange> {
        exhaustive_vecs(exhaustive_natural_range(
            Natural::ZERO,
            Natural::power_of_2(log_base),
        ))
    }
}

pub fn exhaustive_natural_vec_unsigned_pair_gen_var_1() -> It<(Vec<Natural>, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(1, u64::MAX),
        PowerOfTwoDigitsGenerator,
    )))
}

// -- Vec<PrimitiveUnsigned>

// vars 1 through 4 are in malachite-base.

pub fn exhaustive_unsigned_vec_gen_var_5() -> It<Vec<Limb>> {
    Box::new(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds()).map(|mut xs| {
            limbs_vec_mul_limb_in_place(&mut xs, 3);
            xs
        }),
    )
}

// var 6 is in malachite-base

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned)

// vars 1 through 3 are in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

// vars 5 through 17 are in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_18() -> It<(Vec<Limb>, u64)> {
    Box::new(
        exhaustive_unsigned_vec_unsigned_pair_gen_var_17().filter(|(xs, index)| {
            let mut mut_xs = xs.clone();
            limbs_vec_clear_bit_neg(&mut mut_xs, *index);
            mut_xs.len() == xs.len()
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_19() -> It<(Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            exhaustive_positive_primitive_ints(),
        )
        .map(|(mut xs, y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y);
            (xs, y)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_20() -> It<(Vec<Limb>, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds::<Limb>()),
            exhaustive_unsigneds(),
        )
        .filter_map(|(mut xs, mut pow)| {
            let xs_last = xs.last_mut().unwrap();
            *xs_last = xs_last.checked_add(1)?;
            pow += limbs_significant_bits_helper(&xs);
            Some((xs, pow))
        }),
    )
}

// var 21 is in malachite-base.

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned)

// vars 1 through 5 are in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_6() -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        exhaustive_triples_xyy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )
        .filter(|(m, x, y)| {
            !Integer::from(Natural::from(*x)).eq_mod(-Natural::from(*y), Natural::from_limbs_asc(m))
        }),
    )
}

// vars 7 through 8 are in malachite-base.

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_9() -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(exhaustive_triples(
        exhaustive_vecs(exhaustive_unsigneds()),
        factors_of_limb_max().into_iter(),
        exhaustive_unsigneds(),
    ))
}

// var 10 is in malachite-base.

pub(crate) fn map_helper_3(t: (Vec<Limb>, Limb, Limb)) -> (Vec<Limb>, Limb, Limb) {
    let (mut xs, y, m) = t;
    let carry = limbs_slice_mul_limb_in_place(&mut xs, m);
    if carry != 0 {
        xs.push(carry);
    } else if *xs.last().unwrap() == 0 {
        xs.pop();
    }
    limbs_vec_add_limb_in_place(&mut xs, y);
    (xs, y, m)
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_11() -> It<(Vec<Limb>, Limb, Limb)>
{
    Box::new(
        exhaustive_triples(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
        )
        .map(map_helper_3),
    )
}

pub(crate) fn filter_helper_6(t: &(Vec<Limb>, Limb, Limb)) -> bool {
    let (xs, y, m) = t;
    !limbs_eq_limb_mod_limb(xs, *y, *m)
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_12() -> It<(Vec<Limb>, Limb, Limb)>
{
    Box::new(
        exhaustive_triples(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
        )
        .filter(filter_helper_6),
    )
}

// var 13 is in malachite-base.

pub(crate) fn limbs_significant_bits_helper(xs: &[Limb]) -> u64 {
    let trailing_zeros = usize::exact_from(slice_trailing_zeros(xs));
    if trailing_zeros == xs.len() {
        0
    } else {
        limbs_significant_bits(&xs[..xs.len() - trailing_zeros])
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_14<T: PrimitiveUnsigned>(
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_vecs(exhaustive_unsigneds::<Limb>()),
            exhaustive_unsigneds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            (xs, y, pow)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_15<T: PrimitiveUnsigned>(
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds::<Limb>()),
            exhaustive_unsigneds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            (xs, y, pow)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_16<T: PrimitiveUnsigned>(
) -> It<(Vec<Limb>, T, u64)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_vecs(exhaustive_unsigneds::<Limb>()),
            exhaustive_unsigneds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(xs, y, mut pow)| {
            pow += max(limbs_significant_bits_helper(&xs), y.significant_bits());
            if pow == 0 {
                pow = 1;
            }
            (xs, y, pow)
        }),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>)

struct ValidLengthsGenerator;

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), Vec<T>, It<Vec<T>>>
    for ValidLengthsGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            limbs_digit_count(&p.0, p.1),
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, u64, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                exhaustive_vecs(exhaustive_unsigneds()),
                (3u64..256).filter(|&b| !b.is_power_of_two()),
            ),
            ValidLengthsGenerator,
        )
        .map(|((xs, base), out)| (out, base, xs)),
    )
}

pub(crate) fn filter_map_helper_1(
    t: (Vec<Limb>, Limb, Vec<Limb>),
) -> Option<(Vec<Limb>, Limb, Vec<Limb>)> {
    let (xs, y, m) = t;
    let mut product_limbs = if xs.is_empty() {
        Vec::new()
    } else {
        limbs_mul(&xs, &m)
    };
    if product_limbs.last() == Some(&0) {
        product_limbs.pop();
    }
    if limbs_sub_limb_in_place(&mut product_limbs, y) || *product_limbs.last().unwrap() == 0 {
        None
    } else {
        Some((product_limbs, y, m))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2(
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )))
        .filter_map(filter_map_helper_1),
    )
}

pub(crate) fn filter_helper_1(t: &(Vec<Limb>, Limb, Vec<Limb>)) -> bool {
    let (xs, y, m) = t;
    !Integer::from(Natural::from_limbs_asc(xs))
        .eq_mod(-Natural::from(*y), Natural::from_limbs_asc(m))
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3(
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )))
        .filter(filter_helper_1),
    )
}

pub(crate) fn map_helper_1(t: (Vec<Limb>, Limb, Vec<Limb>)) -> (Vec<Limb>, Limb, Vec<Limb>) {
    let (xs, y, m) = t;
    let mut product_limbs = if xs.is_empty() {
        Vec::new()
    } else {
        limbs_mul(&xs, &m)
    };
    if product_limbs.last() == Some(&0) {
        product_limbs.pop();
    }
    limbs_vec_add_limb_in_place(&mut product_limbs, y);
    (product_limbs, y, m)
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_4(
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )))
        .map(map_helper_1),
    )
}

pub(crate) fn filter_helper_4(t: &(Vec<Limb>, Limb, Vec<Limb>)) -> bool {
    let (xs, y, m) = t;
    !limbs_eq_limb_mod_ref_ref(xs, *y, m)
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_5(
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        permute_1_3_2(Box::new(exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )))
        .filter(filter_helper_4),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidLengthsBasecaseGenerator {
    min_out_len: usize,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<usize, Vec<T>, It<Vec<T>>>
    for ValidLengthsBasecaseGenerator
{
    #[inline]
    fn get_ys(&self, &len: &usize) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            u64::exact_from(if len == 0 { self.min_out_len } else { len }),
            exhaustive_unsigneds(),
        ))
    }
}

struct BasecaseDigitsInputGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), (Vec<T>, usize), It<(Vec<T>, usize)>>
    for BasecaseDigitsInputGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<(Vec<T>, usize)> {
        let min_out_len = usize::exact_from(limbs_digit_count(&p.0, p.1));
        permute_2_1(Box::new(exhaustive_dependent_pairs(
            ruler_sequence(),
            once(0).chain(primitive_int_increasing_inclusive_range(
                min_out_len,
                usize::MAX,
            )),
            ValidLengthsBasecaseGenerator { min_out_len },
        )))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>() -> It<(Vec<T>, usize, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                exhaustive_vecs_length_range(
                    0,
                    u64::wrapping_from(GET_STR_PRECOMPUTE_THRESHOLD),
                    exhaustive_unsigneds(),
                ),
                (3u64..256).filter(|&b| !b.is_power_of_two()),
            ),
            BasecaseDigitsInputGenerator,
        )
        .map(|((xs, base), (out, len))| (out, len, xs, base)),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 10 are in malachite-base.

pub(crate) fn gcd_input_filter(xs: &[Limb], ys: &[Limb]) -> bool {
    *xs.last().unwrap() != 0
        && *ys.last().unwrap() != 0
        && limbs_cmp(xs, ys) != Less
        && (xs[0].odd() || ys[0].odd())
}

pub fn exhaustive_unsigned_vec_pair_gen_var_11() -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator2,
        )
        .map(|p| p.1)
        .filter(|(xs, ys)| gcd_input_filter(xs, ys)),
    )
}

// vars 12 through 13 are in malachite-base.

pub fn exhaustive_unsigned_vec_pair_gen_var_14() -> It<(Vec<Limb>, Vec<Limb>)> {
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
        .filter_map(|(_, (out, mut xs))| {
            limbs_vec_mul_limb_in_place(&mut xs, 3);
            if out.len() >= xs.len() {
                Some((out, xs))
            } else {
                None
            }
        }),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_15() -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_vecs_min_length(2, exhaustive_unsigneds()))
            .filter_map(|(ns, ds)| {
                if *ds.last().unwrap() == 0 {
                    return None;
                }
                let mut new_ns = limbs_mul(&ns, &ds);
                if *new_ns.last().unwrap() == 0 {
                    new_ns.pop();
                }
                Some((new_ns, ds))
            }),
    )
}

// vars 16 through 17 are in malachite-nz.

pub fn exhaustive_unsigned_vec_pair_gen_var_18() -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()).filter_map(|mut xs| {
                let x_last = xs.last_mut().unwrap();
                if *x_last == Limb::MAX {
                    None
                } else {
                    *x_last += 1;
                    Some(xs)
                }
            }),
        )
        .filter_map(|(ns, ds)| {
            let mut ns = limbs_mul(&ns, &ds);
            if *ns.last().unwrap() == 0 {
                ns.pop();
            }
            if *ns.last().unwrap() == 0 {
                None
            } else {
                Some((ns, ds))
            }
        }),
    )
}

// vars 19 through 21 are in malachite-nz.

pub fn exhaustive_unsigned_vec_pair_gen_var_22<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_pairs(
                primitive_int_increasing_inclusive_range(1, u64::MAX),
                primitive_int_increasing_inclusive_range(
                    1,
                    u64::exact_from(SQRLO_DC_THRESHOLD_LIMIT),
                ),
            )
            .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
    )
}

fn exhaustive_square_helper<T: PrimitiveUnsigned, F: Fn(usize) -> bool>(
    valid: &'static F,
    min_x: u64,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                move |(o, x)| {
                    let x = x.checked_add(min_x)?;
                    let ux = usize::exact_from(x);
                    if valid(ux) {
                        let o = x.arithmetic_checked_shl(1u64)?.checked_add(o)?;
                        Some((o, x))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecPairLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_23<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&|x| x <= SQR_TOOM2_THRESHOLD, 1)
}

pub fn exhaustive_unsigned_vec_pair_gen_var_24<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&|_| true, 2)
}

pub fn exhaustive_unsigned_vec_pair_gen_var_25<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&limbs_square_to_out_toom_3_input_size_valid, 3)
}

pub fn exhaustive_unsigned_vec_pair_gen_var_26<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&limbs_square_to_out_toom_4_input_size_valid, 4)
}

pub fn exhaustive_unsigned_vec_pair_gen_var_27<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&|x| x == 7 || x == 8 || x > 9, 7)
}

pub fn exhaustive_unsigned_vec_pair_gen_var_28<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&limbs_square_to_out_toom_6_input_size_valid, 18)
}

pub fn exhaustive_unsigned_vec_pair_gen_var_29<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    exhaustive_square_helper(&limbs_square_to_out_toom_8_input_size_valid, 40)
}

// vars 32 to 33 are in malachite-base.

pub fn exhaustive_unsigned_vec_pair_gen_var_34<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    #[cfg(feature = "32_bit_limbs")]
    let limit = 56;
    #[cfg(not(feature = "32_bit_limbs"))]
    let limit = 28;
    exhaustive_square_helper(&limbs_square_to_out_fft_is_valid, limit)
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidDigitsGenerator1<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, usize), (Vec<T>, Vec<U>), It<(Vec<T>, Vec<U>)>>
    for ValidDigitsGenerator1<T, U>
{
    #[inline]
    fn get_ys(&self, p: &(u64, usize)) -> It<(Vec<T>, Vec<U>)> {
        Box::new(exhaustive_pairs(
            exhaustive_vecs_fixed_length_from_single(
                u64::wrapping_from(p.1),
                primitive_int_increasing_range(T::ZERO, T::wrapping_from(p.0)),
            ),
            exhaustive_vecs_min_length(limbs_per_digit_in_base(p.1, p.0), exhaustive_unsigneds()),
        ))
    }
}

// var 1 is in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                (3u64..256).filter(|&b| !b.is_power_of_two()),
                exhaustive_positive_primitive_ints(),
            ),
            ValidDigitsGenerator1 {
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            },
        )
        .map(|((base, _), (xs, out))| (out, xs, base)),
    )
}

struct ValidDigitsGenerator2<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, usize), (Vec<T>, Vec<U>), It<(Vec<T>, Vec<U>)>>
    for ValidDigitsGenerator2<T, U>
{
    #[inline]
    fn get_ys(&self, p: &(u64, usize)) -> It<(Vec<T>, Vec<U>)> {
        Box::new(exhaustive_pairs(
            exhaustive_vecs_fixed_length_from_single(
                u64::wrapping_from(p.1),
                exhaustive_unsigneds(),
            ),
            exhaustive_vecs_min_length(limbs_per_digit_in_base(p.1, p.0), exhaustive_unsigneds()),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                (3u64..256).filter(|&b| !b.is_power_of_two()),
                exhaustive_positive_primitive_ints(),
            ),
            ValidDigitsGenerator2 {
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            },
        )
        .map(|((base, _), (xs, out))| (out, xs, base)),
    )
}

// vars 4 through 6 are in malachite-base.

pub(crate) fn filter_map_helper_2(
    t: (Vec<Limb>, Vec<Limb>, Limb),
) -> Option<(Vec<Limb>, Vec<Limb>, Limb)> {
    let (xs, ys, m) = t;
    let mut product_limbs = xs;
    if !product_limbs.is_empty() {
        limbs_vec_mul_limb_in_place(&mut product_limbs, m);
    };
    if product_limbs.last() == Some(&0) {
        product_limbs.pop();
    }
    if product_limbs.len() < ys.len()
        || limbs_sub_greater_in_place_left(&mut product_limbs, &ys)
        || *product_limbs.last().unwrap() == 0
    {
        None
    } else {
        Some((product_limbs, ys, m))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7(
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )
        .filter_map(filter_map_helper_2),
    )
}

pub(crate) fn filter_helper_2(t: &(Vec<Limb>, Vec<Limb>, Limb)) -> bool {
    let (xs, ys, m) = t;
    !Integer::from(Natural::from_limbs_asc(xs))
        .eq_mod(-Natural::from_limbs_asc(ys), Natural::from(*m))
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8(
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )
        .filter(filter_helper_2),
    )
}

// vars 9 through 13 are in malachite-base.

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14(
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                // TODO
                exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(1, u64::MAX))
                    .filter(|(x, y)| x >= y),
                UnsignedVecPairLenGenerator1,
            ),
            exhaustive_positive_primitive_ints(),
        )
        .filter_map(|((_, (out, mut xs)), y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y);
            if out.len() >= xs.len() {
                Some((out, xs, y))
            } else {
                None
            }
        }),
    )
}

pub(crate) fn map_helper_2(t: (Vec<Limb>, Vec<Limb>, Limb)) -> (Vec<Limb>, Vec<Limb>, Limb) {
    let (mut xs, ys, m) = t;
    limbs_vec_mul_limb_in_place(&mut xs, m);
    if xs.last() == Some(&0) {
        xs.pop();
    }
    limbs_vec_add_in_place_left(&mut xs, &ys);
    (xs, ys, m)
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_15(
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )
        .map(map_helper_2),
    )
}

pub(crate) fn filter_helper_5(t: &(Vec<Limb>, Vec<Limb>, Limb)) -> bool {
    let (xs, ys, m) = t;
    !limbs_eq_mod_limb_ref_ref(xs, ys, *m)
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_16(
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_triples_xxy(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
            exhaustive_positive_primitive_ints(),
        )
        .filter(filter_helper_5),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_17(
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                    |(mut n_len, mut d_init_len)| {
                        n_len = n_len.checked_add(3)?;
                        d_init_len = d_init_len.checked_add(2)?;
                        if n_len > d_init_len {
                            Some((n_len, d_init_len))
                        } else {
                            None
                        }
                    },
                ),
                UnsignedVecPairLenGenerator1,
            ),
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )
        .map(|((_, (n, mut d_init)), d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (n, d_init, inverse)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18(
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_vecs(exhaustive_unsigneds::<Limb>()),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(xs, ys, mut pow)| {
            pow += max(
                limbs_significant_bits_helper(&xs),
                limbs_significant_bits_helper(&ys),
            );
            (xs, ys, pow)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19(
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                // TODO
                exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
                UnsignedVecPairLenGenerator1,
            ),
            exhaustive_unsigneds(),
        )
        .map(|((_, (xs, ys)), mut pow)| {
            pow += max(
                limbs_significant_bits_helper(&xs),
                limbs_significant_bits_helper(&ys),
            );
            (xs, ys, pow)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20(
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds::<Limb>()).filter_map(|mut xs| {
                let last_x = xs.last_mut().unwrap();
                *last_x = last_x.checked_add(1)?;
                Some(xs)
            }),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .map(|(xs, ys, mut pow)| {
            pow += max(
                limbs_significant_bits_helper(&xs),
                limbs_significant_bits_helper(&ys),
            );
            (xs, ys, pow)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_21(
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds::<Limb>()),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(mut xs, mut es, pow)| {
            let last_e = es.last_mut().unwrap();
            *last_e = last_e.checked_add(1)?;
            if es == [1] {
                return None;
            }
            limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
            if *xs.last().unwrap() == 0 {
                None
            } else {
                Some((xs, es, pow))
            }
        })
        .unique(),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 3 are in malachite-base

fn exhaustive_mul_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    valid: &'static F,
    min_x: u64,
    min_y: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                move |(o, x, y)| {
                    let x = x.checked_add(min_x)?;
                    let y = y.checked_add(min_y)?;
                    if valid(usize::exact_from(x), usize::exact_from(y)) {
                        let o = x.checked_add(y)?.checked_add(o)?;
                        Some((o, x, y))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_22_input_sizes_valid, 2, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_32_input_sizes_valid, 6, 4)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_33_input_sizes_valid, 3, 3)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_42_input_sizes_valid, 4, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_43_input_sizes_valid, 11, 8)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_44_input_sizes_valid, 4, 4)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_52_input_sizes_valid, 14, 5)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_53_input_sizes_valid, 5, 3)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_54_input_sizes_valid, 14, 11)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_62_input_sizes_valid, 6, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_63_input_sizes_valid, 17, 9)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_6h_input_sizes_valid, 42, 42)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_toom_8h_input_sizes_valid, 86, 86)
}

fn exhaustive_mul_same_length_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    valid: &'static F,
    min_x: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                move |(o, x)| {
                    let x = x.checked_add(min_x)?;
                    let ux = usize::exact_from(x);
                    if valid(ux, ux) {
                        let o = x.arithmetic_checked_shl(1u64)?.checked_add(o)?;
                        Some((o, x))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_18<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(&limbs_mul_greater_to_out_toom_33_input_sizes_valid, 5)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_19<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(&limbs_mul_greater_to_out_toom_6h_input_sizes_valid, 42)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_20<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(&limbs_mul_greater_to_out_toom_8h_input_sizes_valid, 86)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_22<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(
        &|xs_len, ys_len| {
            limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len, ys_len)
        },
        11,
        8,
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_23<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(
        &|xs_len, ys_len| {
            limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len, ys_len)
                && limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len, ys_len)
        },
        5,
        3,
    )
}

// vars 24 through 36 are in malachite-base

pub(crate) fn filter_map_helper_3(
    t: (Vec<Limb>, Vec<Limb>, Vec<Limb>),
) -> Option<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let (xs, ys, m) = t;
    let mut product_limbs = if xs.is_empty() {
        Vec::new()
    } else {
        limbs_mul(&xs, &m)
    };
    if product_limbs.last() == Some(&0) {
        product_limbs.pop();
    }
    if product_limbs.len() < ys.len()
        || limbs_sub_greater_in_place_left(&mut product_limbs, &ys)
        || *product_limbs.last().unwrap() == 0
    {
        None
    } else {
        Some((product_limbs, ys, m))
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_37() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter_map(filter_map_helper_3),
    )
}

pub(crate) fn filter_helper_3(t: &(Vec<Limb>, Vec<Limb>, Vec<Limb>)) -> bool {
    let (xs, ys, m) = t;
    !Integer::from(Natural::from_limbs_asc(xs))
        .eq_mod(-Natural::from_limbs_asc(ys), Natural::from_limbs_asc(m))
}

pub fn exhaustive_unsigned_vec_triple_gen_var_38() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter(filter_helper_3),
    )
}

// vars 39 through 41 are in malachite-base.

pub fn exhaustive_unsigned_vec_triple_gen_var_42() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                    |(q_len, mut n_len, mut d_init_len)| {
                        n_len = n_len.checked_add(2)?;
                        d_init_len = d_init_len.checked_add(1)?;
                        let d_len = d_init_len + 1;
                        if n_len >= d_len && q_len >= n_len - d_len {
                            Some((q_len, n_len, d_init_len))
                        } else {
                            None
                        }
                    },
                ),
                UnsignedVecTripleLenGenerator1,
            )
            .map(|p| p.1),
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_43() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                    |(q_len, mut n_len, mut d_init_len)| {
                        n_len = n_len.checked_add(3)?;
                        d_init_len = d_init_len.checked_add(1)?;
                        let d_len = d_init_len + 1;
                        if n_len > d_len && q_len >= n_len - d_len {
                            Some((q_len, n_len, d_init_len))
                        } else {
                            None
                        }
                    },
                ),
                UnsignedVecTripleLenGenerator1,
            )
            .map(|p| p.1),
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_44() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(1)?;
                    n_len = n_len.checked_add(2)?;
                    d_len = d_len.checked_add(2)?;
                    if n_len >= d_len && q_len > n_len - d_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d))| {
            if *d.last().unwrap() != 0 {
                Some((q, n, d))
            } else {
                None
            }
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_45() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(1)?;
                    n_len = n_len.checked_add(2)?;
                    d_len = d_len.checked_add(2)?;
                    if n_len >= d_len && q_len > n_len - d_len && n_len < (d_len - 1) << 1 {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d))| {
            if *d.last().unwrap() != 0 {
                Some((q, n, d))
            } else {
                None
            }
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_46() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter_map(|(q_len, n_len, d_len)| {
                    if q_len >= n_len && n_len >= d_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                }),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(
            |(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
                if d[0].odd() {
                    Some((q, n, d))
                } else {
                    None
                }
            },
        ),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_47() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(1, u64::MAX))
                .filter_map(|(q_len, n_len, d_len)| {
                    if q_len >= n_len && n_len >= d_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                }),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(
            |(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
                if d[0].odd() {
                    Some((q, n, d))
                } else {
                    None
                }
            },
        ),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_48() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(1, u64::MAX))
                .filter_map(|(q_len, n_len, d_len)| {
                    if q_len + 1 >= n_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                }),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
            if *d.last().unwrap() == 0 {
                return None;
            }
            let mut new_n = limbs_mul(&n, &d);
            if *new_n.last().unwrap() == 0 {
                new_n.pop();
            }
            if q.len() + d.len() >= new_n.len() + 1 {
                Some((q, new_n, d))
            } else {
                None
            }
        }),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_49() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(1, u64::MAX))
                .filter_map(|(q_len, n_len, mut d_len)| {
                    d_len = d_len.checked_add(1)?;
                    if q_len + 1 >= n_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                }),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
            if *d.last().unwrap() == 0 {
                return None;
            }
            let mut new_n = limbs_mul(&n, &d);
            if *new_n.last().unwrap() == 0 {
                new_n.pop();
            }
            if q.len() > new_n.len() - d.len() {
                Some((q, n, d))
            } else {
                None
            }
        }),
    )
}

// vars 50 through 53 are in malachite-base.

pub(crate) fn limbs_eq_mod_map(
    xs: &[Limb],
    ys: Vec<Limb>,
    m: Vec<Limb>,
) -> (Vec<Limb>, Vec<Limb>, Vec<Limb>) {
    let mut product_limbs = if xs.is_empty() {
        Vec::new()
    } else {
        limbs_mul(xs, &m)
    };
    if product_limbs.last() == Some(&0) {
        product_limbs.pop();
    }
    limbs_vec_add_in_place_left(&mut product_limbs, &ys);
    (product_limbs, ys, m)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_54() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
        )
        .map(|(xs, ys, m)| limbs_eq_mod_map(&xs, ys, m)),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_55() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|xs| *xs.last().unwrap() != 0),
        )
        .filter(|(xs, ys, m)| !limbs_eq_mod_ref_ref_ref(xs, ys, m)),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_56() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_pairs_from_single(exhaustive_unsigneds()).filter_map(
                    |(q_len, n_len): (u64, u64)| {
                        Some((q_len.checked_add(n_len)?, n_len.checked_add(2)?))
                    },
                ),
                UnsignedVecPairLenGenerator1,
            ),
            exhaustive_pairs(
                primitive_int_increasing_inclusive_range(
                    Limb::power_of_2(Limb::WIDTH - 1),
                    Limb::MAX,
                ),
                exhaustive_unsigneds(),
            ),
        )
        .map(|((_, (q, n)), (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// var 57 is in malachite-base.

pub fn exhaustive_unsigned_vec_triple_gen_var_58<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(
        &|x, y| {
            limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
                && limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
        },
        7,
        7,
    )
}

// var 59 is in malachite-base.

pub fn exhaustive_unsigned_vec_triple_gen_var_60<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&limbs_mul_greater_to_out_fft_is_valid, 1, 1)
}

// -- (Vec<PrimitiveUnsigned> * 4) --

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_1(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_quadruples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut r_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(1)?;
                    r_len = r_len.checked_add(2)?;
                    n_len = n_len.checked_add(2)?;
                    d_len = d_len.checked_add(2)?;
                    if r_len >= d_len && n_len >= d_len && q_len > n_len - d_len {
                        Some((q_len, r_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecQuadrupleLenGenerator1,
        )
        .filter_map(|(_, (q, r, n, d))| {
            if *d.last().unwrap() != 0 {
                Some((q, r, n, d))
            } else {
                None
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_2(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_quadruples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut r_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(2)?;
                    r_len = r_len.checked_add(2)?;
                    n_len = n_len.checked_add(4)?;
                    d_len = d_len.checked_add(2)?;
                    if n_len >= d_len + 2 && q_len >= n_len - d_len && r_len >= d_len {
                        Some((q_len, r_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecQuadrupleLenGenerator1,
        )
        .filter_map(
            |(_, (q, r, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
                if d[0].odd() {
                    Some((q, r, n, d))
                } else {
                    None
                }
            },
        ),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_3(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_quadruples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut r_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(4)?;
                    r_len = r_len.checked_add(2)?;
                    n_len = n_len.checked_add(4)?;
                    d_len = d_len.checked_add(2)?;
                    if n_len >= d_len + 2 && q_len >= n_len - d_len && r_len >= d_len {
                        Some((q_len, r_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecQuadrupleLenGenerator1,
        )
        .filter_map(
            #[allow(clippy::type_complexity)]
            |(_, (q, r, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
                if d[0].odd() {
                    Some((q, r, n, d))
                } else {
                    None
                }
            },
        ),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_4(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_quadruples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut r_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(1)?;
                    r_len = r_len.checked_add(2)?;
                    n_len = n_len.checked_add(2)?;
                    d_len = d_len.checked_add(2)?;
                    if r_len >= d_len
                        && n_len >= d_len
                        && q_len > n_len - d_len
                        && (d_len << 1) > n_len + 1
                    {
                        Some((q_len, r_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecQuadrupleLenGenerator1,
        )
        .filter_map(|(_, (q, r, n, d))| {
            if *d.last().unwrap() != 0 {
                Some((q, r, n, d))
            } else {
                None
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_5(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_quadruples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(mut q_len, mut r_len, mut n_len, mut d_len)| {
                    q_len = q_len.checked_add(1)?;
                    r_len = r_len.checked_add(2)?;
                    n_len = n_len.checked_add(3)?;
                    d_len = d_len.checked_add(2)?;
                    if r_len >= d_len && n_len > d_len && q_len + d_len >= n_len {
                        Some((q_len, r_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecQuadrupleLenGenerator1,
        )
        .filter_map(
            #[allow(clippy::type_complexity)]
            |(_, (q, r, n, mut d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
                let d_last = d.last_mut().unwrap();
                if d_last.get_highest_bit() {
                    None
                } else {
                    d_last.set_bit(Limb::WIDTH - 1);
                    Some((q, r, n, d))
                }
            },
        ),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_6(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_quadruples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(out_len, mut b_len, e_len, mut m_len)| {
                    b_len = b_len.checked_add(1)?;
                    m_len = m_len.checked_add(1)?;
                    if out_len >= m_len {
                        Some((out_len, b_len, e_len, m_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecQuadrupleLenGenerator1,
        )
        .filter_map(
            #[allow(clippy::type_complexity)]
            |(_, (out, bs, es, ms)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
                if (es.len() > 1 || es.len() == 1 && es[0] > 1)
                    && *bs.last().unwrap() != 0
                    && *es.last().unwrap() != 0
                    && *ms.last().unwrap() != 0
                {
                    Some((out, bs, es, ms))
                } else {
                    None
                }
            },
        ),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_unsigned_vec_quadruple_gen_var_7(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        exhaustive_unsigned_vec_quadruple_gen_var_6().filter_map(|(out, bs, es, mut ms)| {
            let m = &mut ms[0];
            *m = m.arithmetic_checked_shl(1)?;
            *m |= 1;
            Some((out, bs, es, ms))
        }),
    )
}

// -- large types --

// vars 1 through 4 are in malachite-base

fn exhaustive_half_gcd_matrices_with_size(s: usize, n: usize) -> It<OwnedHalfGcdMatrix> {
    assert_ne!(n, 0);
    assert!(n <= s);
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(n),
            exhaustive_unsigneds(),
        ))
        .map(move |(mut m00, m01, m10, m11)| {
            m00.resize(s << 2, 0);
            m00[s..s + n].copy_from_slice(&m01);
            m00[s << 1..(s << 1) + n].copy_from_slice(&m10);
            m00[s * 3..s * 3 + n].copy_from_slice(&m11);
            half_gcd_matrix_create(s, n, m00)
        }),
    )
}

struct HalfGcdMatrixGenerator;

impl ExhaustiveDependentPairsYsGenerator<(usize, usize), OwnedHalfGcdMatrix, It<OwnedHalfGcdMatrix>>
    for HalfGcdMatrixGenerator
{
    #[inline]
    fn get_ys(&self, t: &(usize, usize)) -> It<OwnedHalfGcdMatrix> {
        let &(n, s) = t;
        exhaustive_half_gcd_matrices_with_size(s, n)
    }
}

fn exhaustive_half_gcd_matrices() -> It<OwnedHalfGcdMatrix> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_ordered_unique_pairs(exhaustive_positive_primitive_ints()),
            HalfGcdMatrixGenerator,
        )
        .map(|p| p.1),
    )
}

struct HalfGcdMatrixAndVecGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize, usize),
        (OwnedHalfGcdMatrix, Vec<Limb>, u8),
        It<(OwnedHalfGcdMatrix, Vec<Limb>, u8)>,
    > for HalfGcdMatrixAndVecGenerator
{
    #[inline]
    fn get_ys(&self, t: &(usize, usize, usize)) -> It<(OwnedHalfGcdMatrix, Vec<Limb>, u8)> {
        let &(qs_len, m_n, m_s) = t;
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(
                exhaustive_half_gcd_matrices_with_size(m_s, m_n),
                exhaustive_vecs_fixed_length_from_single(
                    u64::exact_from(qs_len),
                    exhaustive_unsigneds(),
                ),
            ),
            0..=1,
        )))
    }
}

pub fn exhaustive_large_type_gen_var_5() -> It<(OwnedHalfGcdMatrix, Vec<Limb>, u8)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<usize>()).filter_map(
                |(x, y, z)| {
                    let qs_len = x.checked_add(1)?;
                    let m_n = qs_len.checked_add(y)?;
                    let m_s_1 = m_n.checked_add(qs_len)?.checked_add(z)?;
                    let m_s_2 = m_n.checked_add(2)?.checked_add(z)?;
                    Some((qs_len, m_n, max(m_s_1, m_s_2)))
                },
            ),
            HalfGcdMatrixAndVecGenerator,
        )
        .map(|p| p.1),
    )
}

fn exhaustive_half_gcd_matrices_1() -> It<HalfGcdMatrix1> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_unsigneds()).map(|(m00, m01, m10, m11)| {
            HalfGcdMatrix1 {
                data: [[m00, m01], [m10, m11]],
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_6() -> It<(HalfGcdMatrix1, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    reshape_1_3_to_4(Box::new(exhaustive_pairs(
        exhaustive_half_gcd_matrices_1(),
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                |(x, y, z)| {
                    let xs_len = x;
                    let ys_len = x.checked_add(1)?.checked_add(y)?;
                    let out_len = x.checked_add(1)?.checked_add(z)?;
                    Some((out_len, xs_len, ys_len))
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .map(|p| p.1),
    )))
}

pub fn exhaustive_large_type_gen_var_7() -> It<(OwnedHalfGcdMatrix, HalfGcdMatrix1)> {
    Box::new(exhaustive_pairs(
        exhaustive_half_gcd_matrices(),
        exhaustive_quadruples_from_single(primitive_int_increasing_range(
            0,
            Limb::power_of_2(Limb::WIDTH - 1),
        ))
        .map(|(m00, m01, m10, m11)| HalfGcdMatrix1 {
            data: [[m00, m01], [m10, m11]],
        }),
    ))
}

struct MatrixMul22Generator;
pub(crate) type T8 = (
    Vec<Limb>,
    Vec<Limb>,
    Vec<Limb>,
    Vec<Limb>,
    usize,
    Vec<Limb>,
    Vec<Limb>,
    Vec<Limb>,
    Vec<Limb>,
);

impl ExhaustiveDependentPairsYsGenerator<(u64, u64), T8, It<T8>> for MatrixMul22Generator {
    #[inline]
    fn get_ys(&self, p: &(u64, u64)) -> It<T8> {
        let &(ys_len, xs_len) = p;
        Box::new(
            exhaustive_pairs(
                exhaustive_quadruples_from_single(exhaustive_vecs_fixed_length_from_single(
                    ys_len + xs_len + 1,
                    exhaustive_unsigneds(),
                )),
                exhaustive_quadruples_from_single(exhaustive_vecs_fixed_length_from_single(
                    ys_len,
                    exhaustive_unsigneds(),
                )),
            )
            .map(
                move |((xs00, xs01, xs10, xs11), (ys00, ys01, ys10, ys11))| {
                    (
                        xs00,
                        xs01,
                        xs10,
                        xs11,
                        usize::exact_from(xs_len),
                        ys00,
                        ys01,
                        ys10,
                        ys11,
                    )
                },
            ),
        )
    }
}

pub fn exhaustive_large_type_gen_var_8() -> It<T8> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints()),
            MatrixMul22Generator,
        )
        .map(|p| p.1),
    )
}

// var 9 is in malachite-base.

pub fn exhaustive_large_type_gen_var_10() -> It<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    reshape_2_2_to_4(Box::new(exhaustive_pairs(
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
        exhaustive_pairs(factors_of_limb_max().into_iter(), exhaustive_unsigneds()),
    )))
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_11() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                    |(q_len, mut n_len, mut d_init_len)| {
                        n_len = n_len.checked_add(3)?;
                        d_init_len = d_init_len.checked_add(2)?;
                        let d_len = d_init_len + 1;
                        if n_len >= d_len && q_len >= n_len - d_len {
                            Some((q_len, n_len, d_init_len))
                        } else {
                            None
                        }
                    },
                ),
                UnsignedVecTripleLenGenerator1,
            )
            .map(|p| p.1),
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (q, n, d_init, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_12() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_dependent_pairs(
                bit_distributor_sequence(
                    BitDistributorOutputType::tiny(),
                    BitDistributorOutputType::normal(1),
                ),
                exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).filter_map(
                    |(mut q_len, mut n_len, mut d_init_len)| {
                        q_len = q_len.checked_add(3)?;
                        n_len = n_len.checked_add(9)?;
                        d_init_len = d_init_len.checked_add(5)?;
                        let d_len = d_init_len + 1;
                        if n_len >= d_len + 3 && q_len >= n_len - d_len {
                            Some((q_len, n_len, d_init_len))
                        } else {
                            None
                        }
                    },
                ),
                UnsignedVecTripleLenGenerator1,
            )
            .map(|p| p.1),
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )
        .map(|((q, n, mut d_init), d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (q, n, d_init, inverse)
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_13() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_triples_from_single(exhaustive_positive_primitive_ints())
                .filter(|&(q_len, n_len, d_len)| q_len >= n_len && n_len >= d_len),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
            if d[0].even() {
                None
            } else {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                Some((q, n, d, inverse))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_14() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_positive_primitive_ints::<u64>()).filter_map(
                |(mut q_len, mut n_len, d_len)| {
                    q_len = q_len.checked_add(1)?;
                    n_len = n_len.checked_add(1)?;
                    if q_len >= n_len && n_len > d_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
            if d[0].even() {
                None
            } else {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                Some((q, n, d, inverse))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_15() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter_map(|(mut q_len, mut n_len, d_len)| {
                    q_len = q_len.checked_add(1)?;
                    n_len = n_len.checked_add(1)?;
                    if q_len >= n_len && n_len > d_len {
                        Some((q_len, n_len, d_len))
                    } else {
                        None
                    }
                }),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
            if d[0].even() {
                None
            } else {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                Some((q, n, d, inverse))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_16() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            // TODO
            exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(2, u64::MAX))
                .filter(|&(q_len, n_len, d_len)| q_len >= n_len && n_len >= d_len),
            UnsignedVecTripleLenGenerator1,
        )
        .filter_map(|(_, (q, n, d)): (_, (Vec<Limb>, Vec<Limb>, Vec<Limb>))| {
            if d[0].even() {
                None
            } else {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                Some((q, n, d, inverse))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_17() -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds::<Limb>()).filter_map(|d| {
            if d[0].even() {
                None
            } else {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                let is = vec![0; d.len()];
                let scratch = vec![0; limbs_modular_invert_scratch_len(d.len())];
                Some((is, scratch, d, inverse))
            }
        }),
    )
}

pub fn exhaustive_large_type_gen_var_18() -> It<(Vec<Limb>, usize, Limb, Limb, u64)> {
    Box::new(
        exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
        )
        .filter_map(|(ns, fraction_len, d)| {
            if ns.len() <= fraction_len {
                None
            } else {
                let shift = LeadingZeros::leading_zeros(d);
                let d_inv = limbs_invert_limb(d << shift);
                Some((ns, fraction_len, d, d_inv, shift))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_19() -> It<(Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)> {
    Box::new(
        exhaustive_quadruples_xyxz(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
        )
        .filter_map(|(out, fraction_len, ns, d)| {
            if ns.is_empty() || out.len() < ns.len() + fraction_len {
                None
            } else {
                let shift = LeadingZeros::leading_zeros(d);
                let d_inv = limbs_invert_limb(d << shift);
                Some((out, fraction_len, ns, d, d_inv, shift))
            }
        }),
    )
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_20(
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    Box::new(
        exhaustive_quintuples_xyyyz(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(3, u32::MAX),
        )
        .filter_map(|(ds, mut scratch, mut qs, mut rs_hi, n_len)| {
            let n_len = usize::wrapping_from(n_len);
            let d_len = ds.len();
            if n_len < d_len {
                return None;
            }
            let i_len = limbs_div_mod_barrett_is_len(n_len - d_len, d_len);
            if i_len == 0 || qs.len() < i_len {
                return None;
            }
            qs.truncate(i_len);
            if rs_hi.len() < i_len {
                return None;
            }
            rs_hi.truncate(i_len);
            let scratch_len = limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
            let x = limbs_div_mod_barrett_scratch_len(n_len, d_len);
            if x < i_len {
                return None;
            }
            let actual_scratch_len = x - i_len;
            if actual_scratch_len < d_len + i_len {
                return None;
            }
            if scratch.len() < actual_scratch_len {
                return None;
            }
            scratch.truncate(actual_scratch_len);
            Some((scratch, ds, qs, rs_hi, scratch_len, i_len))
        }),
    )
}

#[allow(clippy::type_complexity)]
pub(crate) fn large_type_filter_map_1(
    s: (Limb, Limb, Limb, Limb, Limb, Limb),
) -> Option<(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)> {
    let (x_1, x_0, y_1, y_0, m_1, m_0) = s;
    if m_1 == 0
        || m_1 == 1 && m_0 == 0
        || x_1 > m_1
        || y_1 > m_1
        || x_1 == m_1 && x_0 > m_0
        || y_1 == m_1 && y_0 > m_0
    {
        None
    } else {
        let (inv_2, inv_1, inv_0) = limbs_precompute_mod_mul_two_limbs(m_1, m_0);
        Some((x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0))
    }
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_21(
) -> It<(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        exhaustive_sextuples_from_single(exhaustive_unsigneds())
            .filter_map(large_type_filter_map_1),
    )
}

// var 22 is in malachite-base.

struct RationalFromPowerOf2DigitsGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (Vec<Natural>, RationalSequence<Natural>),
        It<(Vec<Natural>, RationalSequence<Natural>)>,
    > for RationalFromPowerOf2DigitsGenerator
{
    #[inline]
    fn get_ys(&self, log_base: &u64) -> It<(Vec<Natural>, RationalSequence<Natural>)> {
        Box::new(exhaustive_pairs(
            exhaustive_vecs(exhaustive_natural_range(
                Natural::ZERO,
                Natural::power_of_2(*log_base),
            )),
            exhaustive_rational_sequences(exhaustive_natural_range(
                Natural::ZERO,
                Natural::power_of_2(*log_base),
            )),
        ))
    }
}

pub fn exhaustive_large_type_gen_var_23() -> It<(u64, Vec<Natural>, RationalSequence<Natural>)> {
    reshape_1_2_to_3(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_positive_primitive_ints(),
        RationalFromPowerOf2DigitsGenerator,
    )))
}

pub fn exhaustive_large_type_gen_var_24() -> It<(Vec<Natural>, RationalSequence<Natural>)> {
    RationalFromPowerOf2DigitsGenerator.get_ys(&1)
}

struct RationalFromDigitsGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        (Vec<Natural>, RationalSequence<Natural>),
        It<(Vec<Natural>, RationalSequence<Natural>)>,
    > for RationalFromDigitsGenerator
{
    #[inline]
    fn get_ys(&self, base: &Natural) -> It<(Vec<Natural>, RationalSequence<Natural>)> {
        Box::new(exhaustive_pairs(
            exhaustive_vecs(exhaustive_natural_range(Natural::ZERO, base.clone())),
            exhaustive_rational_sequences(exhaustive_natural_range(Natural::ZERO, base.clone())),
        ))
    }
}

pub fn exhaustive_large_type_gen_var_25() -> It<(Natural, Vec<Natural>, RationalSequence<Natural>)>
{
    reshape_1_2_to_3(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
        RationalFromDigitsGenerator,
    )))
}

pub fn exhaustive_large_type_gen_var_26() -> It<(Vec<Natural>, RationalSequence<Natural>)> {
    RationalFromDigitsGenerator.get_ys(&Natural::from(10u32))
}
