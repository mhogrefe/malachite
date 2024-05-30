// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::exhaustive::{
    exhaustive_finite_floats, exhaustive_floats, exhaustive_non_negative_finite_floats,
    exhaustive_nonzero_finite_floats, exhaustive_positive_finite_floats,
    exhaustive_positive_floats_with_precision, exhaustive_positive_floats_with_sci_exponent,
};
use crate::test_util::extra_variadic::{
    exhaustive_quadruples_xxyz_custom_output, exhaustive_triples_from_single,
    exhaustive_triples_xxy, exhaustive_triples_xxy_custom_output,
};
use crate::{significand_bits, Float};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::{IsPowerOf2, Square};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::num::exhaustive::{
    exhaustive_positive_primitive_ints, exhaustive_primitive_floats, exhaustive_signeds,
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
};
use malachite_base::num::iterators::ruler_sequence;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::{reshape_2_1_to_3, It};
use malachite_base::test_util::generators::exhaustive_pairs_big_tiny;
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, ExhaustiveDependentPairsYsGenerator,
};
use malachite_base::tuples::exhaustive::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples_custom_output,
    exhaustive_triples, exhaustive_triples_custom_output, exhaustive_triples_xyy, lex_pairs,
};
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::exhaustive::exhaustive_naturals;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::exhaustive::exhaustive_rationals;
use malachite_q::Rational;
use std::cmp::max;

// -- Float --

pub fn exhaustive_float_gen() -> It<Float> {
    Box::new(exhaustive_floats())
}

pub fn exhaustive_float_gen_var_1() -> It<Float> {
    Box::new(exhaustive_positive_finite_floats())
}

pub fn exhaustive_float_gen_var_2() -> It<Float> {
    Box::new(exhaustive_floats().filter(|f| !f.is_nan()))
}

pub fn exhaustive_float_gen_var_3() -> It<Float> {
    Box::new(exhaustive_nonzero_finite_floats())
}

pub fn exhaustive_float_gen_var_4() -> It<Float> {
    Box::new(exhaustive_finite_floats())
}

pub fn exhaustive_float_gen_var_5() -> It<Float> {
    Box::new(exhaustive_non_negative_finite_floats())
}

pub fn exhaustive_float_gen_var_6() -> It<Float> {
    Box::new(exhaustive_floats_with_precision_inclusive_range(
        1,
        Limb::WIDTH - 1,
    ))
}

pub fn exhaustive_float_gen_var_7() -> It<Float> {
    Box::new(exhaustive_positive_floats_with_precision(Limb::WIDTH))
}

pub fn exhaustive_float_gen_var_8() -> It<Float> {
    Box::new(exhaustive_floats_with_precision_inclusive_range(
        Limb::WIDTH + 1,
        (Limb::WIDTH << 1) - 1,
    ))
}

pub fn exhaustive_float_gen_var_9() -> It<Float> {
    Box::new(exhaustive_positive_floats_with_precision(Limb::WIDTH << 1))
}

pub fn exhaustive_float_gen_var_10() -> It<Float> {
    Box::new(exhaustive_floats_with_precision_inclusive_range(
        (Limb::WIDTH << 1) + 1,
        Limb::WIDTH * 3 - 1,
    ))
}

struct FloatWithPrecisionRangeGenerator;

impl ExhaustiveDependentPairsYsGenerator<u64, Float, Box<dyn Iterator<Item = Float>>>
    for FloatWithPrecisionRangeGenerator
{
    #[inline]
    fn get_ys(&self, &prec: &u64) -> Box<dyn Iterator<Item = Float>> {
        Box::new(exhaustive_positive_floats_with_precision(prec))
    }
}

fn exhaustive_floats_with_precision_inclusive_range(prec_lo: u64, prec_hi: u64) -> It<Float> {
    Box::new(
        exhaustive_dependent_pairs(
            ruler_sequence(),
            primitive_int_increasing_inclusive_range(prec_lo, prec_hi),
            FloatWithPrecisionRangeGenerator,
        )
        .map(|p| p.1),
    )
}

struct FloatPairWithPrecisionRangeGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (Float, Float),
        Box<dyn Iterator<Item = (Float, Float)>>,
    > for FloatPairWithPrecisionRangeGenerator
{
    #[inline]
    fn get_ys(&self, &prec: &u64) -> Box<dyn Iterator<Item = (Float, Float)>> {
        Box::new(exhaustive_pairs_from_single(
            exhaustive_positive_floats_with_precision(prec),
        ))
    }
}

fn exhaustive_float_pairs_with_precision_inclusive_range(
    prec_lo: u64,
    prec_hi: u64,
) -> It<(Float, Float)> {
    Box::new(
        exhaustive_dependent_pairs(
            ruler_sequence(),
            primitive_int_increasing_inclusive_range(prec_lo, prec_hi),
            FloatPairWithPrecisionRangeGenerator,
        )
        .map(|p| p.1),
    )
}

// -- (Float, Float) --

pub fn exhaustive_float_pair_gen() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_floats()))
}

pub fn exhaustive_float_pair_gen_var_1() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_finite_floats()))
}

pub fn exhaustive_float_pair_gen_var_2() -> It<(Float, Float)> {
    exhaustive_float_pairs_with_precision_inclusive_range(1, Limb::WIDTH - 1)
}

pub fn exhaustive_float_pair_gen_var_3() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_positive_floats_with_precision(Limb::WIDTH),
    ))
}

pub fn exhaustive_float_pair_gen_var_4() -> It<(Float, Float)> {
    exhaustive_float_pairs_with_precision_inclusive_range(Limb::WIDTH + 1, (Limb::WIDTH << 1) - 1)
}

pub fn exhaustive_float_pair_gen_var_5() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_positive_floats_with_precision(Limb::WIDTH << 1),
    ))
}

pub fn exhaustive_float_pair_gen_var_6() -> It<(Float, Float)> {
    exhaustive_float_pairs_with_precision_inclusive_range(
        (Limb::WIDTH << 1) + 1,
        (Limb::WIDTH * 3) - 1,
    )
}

pub fn exhaustive_float_pair_gen_var_7() -> It<(Float, Float)> {
    exhaustive_float_pairs_with_precision_inclusive_range(Limb::WIDTH * 3, u64::MAX)
}

// -- (Float, Float, Float) --

pub fn exhaustive_float_triple_gen() -> It<(Float, Float, Float)> {
    Box::new(exhaustive_triples_from_single(exhaustive_floats()))
}

// -- (Float, Float, Integer) --

pub fn exhaustive_float_float_integer_triple_gen() -> It<(Float, Float, Integer)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_integers(),
    ))
}

// -- (Float, Float, Natural) --

pub fn exhaustive_float_float_natural_triple_gen() -> It<(Float, Float, Natural)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_naturals(),
    ))
}

// -- (Float, Float, PrimitiveFloat) --

pub fn exhaustive_float_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Float, Float, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Float, Float, PrimitiveSigned) --

pub fn exhaustive_float_float_signed_triple_gen<T: PrimitiveSigned>() -> It<(Float, Float, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_signeds(),
    ))
}

// -- (Float, Float, PrimitiveUnsigned) --

pub fn exhaustive_float_float_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(Float, Float, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_float_float_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Float, Float, T)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_floats(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Float, PrimitiveUnsigned, RoundingMode) --

pub(crate) fn add_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
        let sum = Float::exact_from(rx + ry);
        if let Some(min_prec) = sum.get_min_prec() {
            prec >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_1(
) -> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        exhaustive_quadruples_xxyz_custom_output(
            exhaustive_floats(),
            exhaustive_positive_primitive_ints::<u64>(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter(|(x, y, prec, rm)| add_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn sub_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
        let diff = Float::exact_from(rx - ry);
        if let Some(min_prec) = diff.get_min_prec() {
            prec >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_2(
) -> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        exhaustive_quadruples_xxyz_custom_output(
            exhaustive_floats(),
            exhaustive_positive_primitive_ints::<u64>(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter(|(x, y, prec, rm)| sub_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn mul_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
        let product = Float::exact_from(rx * ry);
        if let Some(min_prec) = product.get_min_prec() {
            prec >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_3(
) -> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        exhaustive_quadruples_xxyz_custom_output(
            exhaustive_floats(),
            exhaustive_positive_primitive_ints::<u64>(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter(|(x, y, prec, rm)| mul_prec_round_valid(x, y, *prec, *rm)),
    )
}

// -- (Float, Float, Rational) --

pub fn exhaustive_float_float_rational_triple_gen() -> It<(Float, Float, Rational)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_rationals(),
    ))
}

// -- (Float, Float, RoundingMode) --

pub(crate) fn add_round_valid(x: &Float, y: &Float, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
        let sum = Float::exact_from(Rational::exact_from(x) + Rational::exact_from(y));
        if let Some(min_prec) = sum.get_min_prec() {
            max(x_prec, y_prec) >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_1() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub(crate) fn sub_round_valid(x: &Float, y: &Float, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
        let diff = Float::exact_from(Rational::exact_from(x) - Rational::exact_from(y));
        if let Some(min_prec) = diff.get_min_prec() {
            max(x_prec, y_prec) >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_2() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_3() -> It<(Float, Float, RoundingMode)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    ))
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_4() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_5() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_6() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_7() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_5(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_8() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_6(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_9() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_7(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_10() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_11() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_12() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_13() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_5(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_14() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_6(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_15() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_7(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm)),
    )
}

pub(crate) fn mul_round_valid(x: &Float, y: &Float, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
        let product = Float::exact_from(Rational::exact_from(x) * Rational::exact_from(y));
        if let Some(min_prec) = product.get_min_prec() {
            max(x_prec, y_prec) >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_16() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        exhaustive_triples_xxy(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_17() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_18() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_19() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_20() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_5(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_21() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_6(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_22() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_7(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm)),
    )
}

// -- (Float, Integer) --

pub fn exhaustive_float_integer_pair_gen() -> It<(Float, Integer)> {
    Box::new(exhaustive_pairs(exhaustive_floats(), exhaustive_integers()))
}

pub fn exhaustive_float_integer_pair_gen_var_1() -> It<(Float, Integer)> {
    Box::new(exhaustive_pairs(
        exhaustive_finite_floats(),
        exhaustive_integers(),
    ))
}

// -- (Float, Integer, Integer) --

pub fn exhaustive_float_integer_integer_triple_gen() -> It<(Float, Integer, Integer)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_integers(),
    ))
}

// -- (Float, Natural) --

pub fn exhaustive_float_natural_pair_gen() -> It<(Float, Natural)> {
    Box::new(exhaustive_pairs(exhaustive_floats(), exhaustive_naturals()))
}

pub fn exhaustive_float_natural_pair_gen_var_1() -> It<(Float, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_finite_floats(),
        exhaustive_naturals(),
    ))
}

// -- (Float, Natural, Natural) --

pub fn exhaustive_float_natural_natural_triple_gen() -> It<(Float, Natural, Natural)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_naturals(),
    ))
}

// -- (Float, PrimitiveFloat) --

pub fn exhaustive_float_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Float, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_float_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Float, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Float, PrimitiveSigned) --

pub fn exhaustive_float_signed_pair_gen<T: PrimitiveSigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(exhaustive_floats(), exhaustive_signeds()))
}

pub fn exhaustive_float_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_floats_with_sci_exponent(0),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_float_signed_pair_gen_var_2<T: PrimitiveSigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_floats(),
        exhaustive_signeds(),
    ))
}

// -- (Float, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_float_signed_signed_triple_gen<T: PrimitiveSigned>() -> It<(Float, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_signeds(),
    ))
}

// -- (Float, PrimitiveUnsigned) --

pub fn exhaustive_float_unsigned_pair_gen<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_floats(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_floats(),
        exhaustive_unsigneds(),
    ))
}

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_float_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(Float, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_unsigneds(),
    ))
}

// -- (Float, PrimitiveUnsigned, RoundingMode) --

pub fn set_prec_round_valid(x: &Float, p: u64, rm: RoundingMode) -> bool {
    rm != Exact || !x.is_finite() || x.is_zero() || x.get_prec().unwrap() <= p || {
        let significand = x.significand_ref().unwrap();
        significand_bits(significand) - significand.trailing_zeros().unwrap() <= p
    }
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_1() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| set_prec_round_valid(x, p, rm)),
    ))
}

pub fn square_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let Ok(rx) = Rational::try_from(x) {
        let square = Float::exact_from(rx.square());
        if let Some(min_prec) = square.get_min_prec() {
            prec >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_2() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| square_prec_round_valid(x, p, rm)),
    ))
}

// -- (Float, Rational) --

pub fn exhaustive_float_rational_pair_gen() -> It<(Float, Rational)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_rationals(),
    ))
}

pub fn exhaustive_float_rational_pair_gen_var_1() -> It<(Float, Rational)> {
    Box::new(exhaustive_pairs(
        exhaustive_finite_floats(),
        exhaustive_rationals(),
    ))
}

// -- (Float, Rational, PrimitiveUnsigned) --

pub fn exhaustive_float_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Float, Rational, T)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_floats(),
        exhaustive_rationals(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Rational, PrimitiveUnsigned, RoundingMode) --

pub(crate) fn add_prec_round_rational_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    if let Ok(rx) = Rational::try_from(x) {
        if let Ok(sum) = Float::try_from(rx + y) {
            if let Some(min_prec) = sum.get_min_prec() {
                prec >= min_prec
            } else {
                true
            }
        } else {
            false
        }
    } else {
        true
    }
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_1(
) -> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        exhaustive_quadruples_custom_output(
            exhaustive_floats(),
            exhaustive_rationals(),
            exhaustive_positive_primitive_ints(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter(|(x, y, prec, rm)| add_prec_round_rational_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn sub_prec_round_rational_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    if let Ok(rx) = Rational::try_from(x) {
        if let Ok(diff) = Float::try_from(rx - y) {
            if let Some(min_prec) = diff.get_min_prec() {
                prec >= min_prec
            } else {
                true
            }
        } else {
            false
        }
    } else {
        true
    }
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_2(
) -> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        exhaustive_quadruples_custom_output(
            exhaustive_floats(),
            exhaustive_rationals(),
            exhaustive_positive_primitive_ints(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter(|(x, y, prec, rm)| sub_prec_round_rational_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn mul_prec_round_rational_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    if let Ok(rx) = Rational::try_from(x) {
        if let Ok(product) = Float::try_from(rx * y) {
            if let Some(min_prec) = product.get_min_prec() {
                prec >= min_prec
            } else {
                true
            }
        } else {
            false
        }
    } else {
        true
    }
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_3(
) -> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        exhaustive_quadruples_custom_output(
            exhaustive_floats(),
            exhaustive_rationals(),
            exhaustive_positive_primitive_ints(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter(|(x, y, prec, rm)| mul_prec_round_rational_valid(x, y, *prec, *rm)),
    )
}

// -- (Float, Rational, Rational) --

pub fn exhaustive_float_rational_rational_triple_gen() -> It<(Float, Rational, Rational)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_rationals(),
    ))
}

// -- (Float, Rational, RoundingMode) --

pub(crate) fn add_round_rational_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let Some(x_prec) = x.get_prec() {
        if let Ok(sum) = Float::try_from(Rational::exact_from(x) + y) {
            if let Some(min_prec) = sum.get_min_prec() {
                x_prec >= min_prec
            } else {
                true
            }
        } else {
            false
        }
    } else {
        // y must be representable by precision-1 float
        y.is_power_of_2()
    }
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_1(
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_floats(),
            exhaustive_rationals(),
            exhaustive_rounding_modes(),
        )
        .filter(|(x, y, rm)| add_round_rational_valid(x, y, *rm)),
    )
}

pub(crate) fn sub_round_rational_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let Some(x_prec) = x.get_prec() {
        if let Ok(diff) = Float::try_from(Rational::exact_from(x) - y) {
            if let Some(min_prec) = diff.get_min_prec() {
                x_prec >= min_prec
            } else {
                true
            }
        } else {
            false
        }
    } else {
        // y must be representable by precision-1 float
        y.is_power_of_2()
    }
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_2(
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_floats(),
            exhaustive_rationals(),
            exhaustive_rounding_modes(),
        )
        .filter(|(x, y, rm)| sub_round_rational_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_3(
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(exhaustive_triples(
        exhaustive_floats(),
        exhaustive_rationals(),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    ))
}

pub(crate) fn mul_round_rational_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let Some(x_prec) = x.get_prec() {
        if let Ok(sum) = Float::try_from(Rational::exact_from(x) * y) {
            if let Some(min_prec) = sum.get_min_prec() {
                x_prec >= min_prec
            } else {
                true
            }
        } else {
            false
        }
    } else {
        // y must be representable by precision-1 float
        y.is_power_of_2()
    }
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_4(
) -> It<(Float, Rational, RoundingMode)> {
    Box::new(
        exhaustive_triples(
            exhaustive_floats(),
            exhaustive_rationals(),
            exhaustive_rounding_modes(),
        )
        .filter(|(x, y, rm)| mul_round_rational_valid(x, y, *rm)),
    )
}

// -- (Float, RoundingMode) --

pub(crate) fn natural_rounding_from_float_valid(f: &Float, rm: RoundingMode) -> bool {
    match rm {
        Down | Ceiling | Nearest => f.is_finite() || *f == Float::NEGATIVE_INFINITY,
        Up | Floor => f.is_finite() && (f.is_sign_positive() || f.is_negative_zero()),
        Exact => Natural::convertible_from(f),
    }
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_1() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| natural_rounding_from_float_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(exhaustive_floats(), exhaustive_rounding_modes()))
}

pub(crate) fn integer_rounding_from_float_valid(f: &Float, rm: RoundingMode) -> bool {
    if rm == Exact {
        Integer::convertible_from(f)
    } else {
        f.is_finite()
    }
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_2() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| integer_rounding_from_float_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_3() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_nonzero_finite_floats(),
        exhaustive_rounding_modes(),
    ))
}

#[allow(clippy::type_repetition_in_bounds)]
pub(crate) fn unsigned_rounding_from_float_valid<T: PrimitiveUnsigned>(
    f: &Float,
    rm: RoundingMode,
) -> bool
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    match rm {
        Floor => f.is_sign_positive() || f.is_negative_zero(),
        Ceiling => *f <= T::MAX,
        Down | Nearest => !f.is_nan(),
        Up => (f.is_sign_positive() || f.is_negative_zero()) && *f <= T::MAX,
        Exact => T::convertible_from(f),
    }
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn exhaustive_float_rounding_mode_pair_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| unsigned_rounding_from_float_valid::<T>(f, *rm)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub(crate) fn signed_rounding_from_float_valid<T: PrimitiveSigned>(
    f: &Float,
    rm: RoundingMode,
) -> bool
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    match rm {
        Floor => *f >= T::MIN,
        Ceiling => *f <= T::MAX,
        Down | Nearest => !f.is_nan(),
        Up => *f >= T::MIN && *f <= T::MAX,
        Exact => T::convertible_from(f),
    }
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn exhaustive_float_rounding_mode_pair_gen_var_5<T: PrimitiveSigned>(
) -> It<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| signed_rounding_from_float_valid::<T>(f, *rm)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn exhaustive_float_rounding_mode_pair_gen_var_6<T: PrimitiveFloat>(
) -> It<(Float, RoundingMode)>
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| *rm != Exact || T::convertible_from(f)),
    )
}

pub(crate) fn square_round_valid(x: &Float, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if let Some(x_prec) = x.get_prec() {
        let square = Float::exact_from(Rational::exact_from(x).square());
        if let Some(min_prec) = square.get_min_prec() {
            x_prec >= min_prec
        } else {
            true
        }
    } else {
        true
    }
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_7() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_8() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(1, Limb::WIDTH - 1),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_9() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_positive_floats_with_precision(Limb::WIDTH),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_10() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(
                Limb::WIDTH + 1,
                (Limb::WIDTH << 1) - 1,
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_11() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_positive_floats_with_precision(Limb::WIDTH << 1),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_12() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(
                (Limb::WIDTH << 1) + 1,
                Limb::WIDTH * 3 - 1,
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm)),
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-nz.

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_3(
) -> It<(Integer, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| {
            rm != Exact || *n == 0u32 || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
        }),
    ))
}

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_4(
) -> It<(Integer, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

// var 1 is in malachite-nz.

pub fn exhaustive_natural_unsigned_rounding_mode_triple_gen_var_2(
) -> It<(Natural, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| {
            rm != Exact || *n == 0u32 || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
        }),
    ))
}

pub fn exhaustive_natural_unsigned_rounding_mode_triple_gen_var_3(
) -> It<(Natural, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    )))
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-base.

pub fn from_primitive_float_prec_round_valid<T: PrimitiveFloat>(
    x: T,
    p: u64,
    rm: RoundingMode,
) -> bool
where
    Float: From<T>,
{
    set_prec_round_valid(&Float::from(x), p, rm)
}

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveFloat>(
) -> It<(T, u64, RoundingMode)>
where
    Float: From<T>,
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_primitive_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, p), rm)| from_primitive_float_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveFloat>(
) -> It<(T, u64, RoundingMode)>
where
    Float: From<T>,
{
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_primitive_floats(),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-base.

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveSigned>(
) -> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_signeds::<T>(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| {
            rm != Exact || *n == T::ZERO || n.significant_bits() - n.trailing_zeros() <= prec
        }),
    ))
}

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>(
) -> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_signeds::<T>(),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 5 are in malachite-base.

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_unsigneds::<T>(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| {
            rm != Exact || *n == T::ZERO || n.significant_bits() - n.trailing_zeros() <= prec
        }),
    ))
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_7<T: PrimitiveUnsigned>(
) -> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_unsigneds::<T>(),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (Rational, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_1(
) -> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| {
            rm != Exact
                || n.denominator_ref().is_power_of_2()
                    && n.numerator_ref().significant_bits() <= prec
        }),
    ))
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_2(
) -> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (Rational, RoundingMode) --

// vars 1 through 5 are in malachite-q.

pub fn exhaustive_rational_rounding_mode_pair_gen_var_6() -> It<(Rational, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes()).filter(|&(ref n, rm)| {
            rm != Exact
                || n.denominator_ref().is_power_of_2() && n.numerator_ref().significant_bits() <= 1
        }),
    )
}
