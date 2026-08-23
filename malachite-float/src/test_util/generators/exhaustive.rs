// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::arithmetic::log_base::{rational_log_base, rational_log_base_of_rational};
use crate::float::arithmetic::log_base_1_plus_x::log_base_1_plus_x_rational;
use crate::float::arithmetic::log_base_10::float_is_power_of_10;
use crate::float::arithmetic::log_base_float_base::log_base_float_base_rational;
use crate::float::arithmetic::log_base_float_base_1_plus_x::log_base_float_base_1_plus_x_rational;
use crate::float::arithmetic::log_base_rational_base::rational_log_base_rational_base;
use crate::float::arithmetic::log_base_rational_base_1_plus_x::*;
use crate::float::arithmetic::log_base_rational_float_base::log_base_rational_float_base_rational;
use crate::float::arithmetic::log_base_rational_rational_base::*;
use crate::float::conversion::string::to_sci::to_sci_valid;
use crate::float::exhaustive::{
    ExhaustivePositiveFiniteFloatsGenerator, ExhaustivePositiveFloatsWithSciExponent,
    exhaustive_finite_floats, exhaustive_floats, exhaustive_non_negative_finite_floats,
    exhaustive_nonzero_finite_floats, exhaustive_positive_finite_floats,
    exhaustive_positive_floats_with_precision, exhaustive_positive_floats_with_sci_exponent,
};
use crate::test_util::extra_variadic::{
    exhaustive_quadruples_from_single, exhaustive_quadruples_xxxy_custom_output,
    exhaustive_quadruples_xxyz_custom_output, exhaustive_quintuples_xxxxy_custom_output,
    exhaustive_quintuples_xxxyz_custom_output, exhaustive_triples_from_single,
    exhaustive_triples_xxy, exhaustive_triples_xxy_custom_output,
};
use crate::test_util::generators::common::{
    FLOAT_FORMAT_COMBO_COUNT, SCI_STRING_COMBO_COUNT, STRTOFR_STRING_CHARS,
    format_string_from_parts, format_string_output_is_bounded, sci_string_from_parts,
    strtofr_string_from_parts, valid_float_from_sci_string_triple, valid_float_get_str_quadruple,
    valid_strtofr_quadruple,
};
use crate::{Float, significand_bits};
use alloc::vec::IntoIter;
use core::cmp::Ordering::*;
use core::iter::once;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::max;
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase, CheckedLogBase2, IsPowerOf2, Reciprocal, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::string::options::exhaustive::exhaustive_to_sci_options;
use malachite_base::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::num::exhaustive::{
    exhaustive_nonzero_signeds, exhaustive_positive_primitive_ints, exhaustive_primitive_floats,
    exhaustive_signeds, exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
};
use malachite_base::num::iterators::{BitDistributorSequence, bit_distributor_sequence};
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base::options::exhaustive::exhaustive_options;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::strings::exhaustive::exhaustive_strings_using_chars;
use malachite_base::test_util::generators::common::{
    It, reshape_2_1_to_3, reshape_3_1_to_4, reshape_4_1_to_5, reshape_5_1_to_6,
};
use malachite_base::test_util::generators::{exhaustive as base_gen, exhaustive_pairs_big_tiny};
use malachite_base::tuples::exhaustive::{
    ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator, exhaustive_dependent_pairs,
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_triples,
    exhaustive_triples_custom_output, exhaustive_triples_xyy, lex_pairs,
};
use malachite_base::vecs::exhaustive::{exhaustive_vecs, exhaustive_vecs_min_length};
use malachite_nz::integer::Integer;
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::natural::Natural;
use malachite_nz::natural::exhaustive::exhaustive_naturals;
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::rational::exhaustive::{exhaustive_non_negative_rationals, exhaustive_rationals};
use std::cmp::{Ordering, max};
use std::iter::Chain;
use std::mem::swap;

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

pub fn exhaustive_float_gen_var_11() -> It<Float> {
    Box::new(exhaustive_floats_with_precision_inclusive_range(
        (Limb::WIDTH << 1) + 1,
        u64::MAX,
    ))
}

pub fn exhaustive_float_gen_var_12() -> It<Float> {
    Box::new(exhaustive_extreme_floats())
}

pub fn exhaustive_float_gen_var_13() -> It<Float> {
    Box::new(exhaustive_extreme_nonzero_finite_floats())
}

pub fn exhaustive_float_gen_var_14() -> It<Float> {
    Box::new(exhaustive_extreme_floats().filter(|f| !f.is_nan()))
}

pub fn exhaustive_float_gen_var_15() -> It<Float> {
    Box::new(exhaustive_floats().filter(|x| *x > 0u32))
}

#[derive(Clone, Debug)]
pub struct ExhaustiveExtremeExponents {
    counter: bool,
    i: i32,
    end_counter: u8,
}

impl Iterator for ExhaustiveExtremeExponents {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.i == 2 {
            return match self.end_counter {
                0 => {
                    self.end_counter = 1;
                    Some(-2)
                }
                1 => {
                    self.end_counter = 2;
                    Some(-1)
                }
                2 => {
                    self.end_counter = 3;
                    Some(0)
                }
                _ => None,
            };
        }
        Some(if self.counter {
            self.counter = false;
            self.i - 2
        } else {
            self.counter = true;
            self.i -= 1;
            -self.i - 1
        })
    }
}

pub const fn exhaustive_extreme_exponents() -> ExhaustiveExtremeExponents {
    ExhaustiveExtremeExponents {
        counter: true,
        i: Float::MAX_EXPONENT + 1,
        end_counter: 0,
    }
}

#[derive(Clone, Debug)]
struct ExhaustiveMixedExtremeExponents {
    first: bool,
    counter: u8,
    low: i32,
    high: i32,
    end_counter: u8,
}

impl Iterator for ExhaustiveMixedExtremeExponents {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.low == self.high {
            return match self.end_counter {
                0 => {
                    self.end_counter = 1;
                    Some(-self.high)
                }
                1 => {
                    self.end_counter = 2;
                    Some(-self.high - 1)
                }
                _ => None,
            };
        }
        Some(if self.first {
            self.first = false;
            0
        } else {
            match self.counter {
                0 => {
                    self.counter = 1;
                    self.low
                }
                1 => {
                    self.counter = 2;
                    self.low += 1;
                    -self.low + 1
                }
                2 => {
                    self.counter = 3;
                    self.high - 1
                }
                3 => {
                    self.counter = 0;
                    self.high -= 1;
                    -self.high - 2
                }
                _ => unreachable!(),
            }
        })
    }
}

const fn exhaustive_mixed_extreme_exponents() -> ExhaustiveMixedExtremeExponents {
    ExhaustiveMixedExtremeExponents {
        first: true,
        counter: 0,
        low: 1,
        high: Float::MAX_EXPONENT,
        end_counter: 0,
    }
}

#[inline]
fn exhaustive_extreme_positive_finite_floats_helper() -> ExhaustiveDependentPairs<
    i32,
    Float,
    BitDistributorSequence,
    ExhaustivePositiveFiniteFloatsGenerator,
    ExhaustiveExtremeExponents,
    ExhaustivePositiveFloatsWithSciExponent,
> {
    exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_extreme_exponents(),
        ExhaustivePositiveFiniteFloatsGenerator,
    )
}

#[derive(Clone, Debug)]
struct ExhaustiveExtremePositiveFiniteFloats(
    ExhaustiveDependentPairs<
        i32,
        Float,
        BitDistributorSequence,
        ExhaustivePositiveFiniteFloatsGenerator,
        ExhaustiveExtremeExponents,
        ExhaustivePositiveFloatsWithSciExponent,
    >,
);

impl Iterator for ExhaustiveExtremePositiveFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.0.next().map(|p| p.1)
    }
}

#[inline]
fn exhaustive_extreme_positive_finite_floats() -> ExhaustiveExtremePositiveFiniteFloats {
    ExhaustiveExtremePositiveFiniteFloats(exhaustive_extreme_positive_finite_floats_helper())
}

#[derive(Clone, Debug)]
struct ExhaustiveExtremeNonzeroFiniteFloats {
    toggle: bool,
    xs: ExhaustiveExtremePositiveFiniteFloats,
    x: Float,
}

impl Iterator for ExhaustiveExtremeNonzeroFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.toggle.not_assign();
        Some(if self.toggle {
            self.x = self.xs.next().unwrap();
            self.x.clone()
        } else {
            let mut out = Float::NAN;
            swap(&mut out, &mut self.x);
            -out
        })
    }
}

#[inline]
fn exhaustive_extreme_nonzero_finite_floats() -> ExhaustiveExtremeNonzeroFiniteFloats {
    ExhaustiveExtremeNonzeroFiniteFloats {
        toggle: false,
        xs: exhaustive_extreme_positive_finite_floats(),
        x: Float::NAN,
    }
}

type ExhaustiveExtremeFloats = Chain<IntoIter<Float>, ExhaustiveExtremeNonzeroFiniteFloats>;

#[inline]
fn exhaustive_extreme_floats() -> ExhaustiveExtremeFloats {
    alloc::vec![
        Float::NAN,
        Float::INFINITY,
        Float::NEGATIVE_INFINITY,
        Float::ZERO,
        Float::NEGATIVE_ZERO
    ]
    .into_iter()
    .chain(exhaustive_extreme_nonzero_finite_floats())
}

#[inline]
fn exhaustive_mixed_extreme_positive_finite_floats_helper() -> ExhaustiveDependentPairs<
    i32,
    Float,
    BitDistributorSequence,
    ExhaustivePositiveFiniteFloatsGenerator,
    ExhaustiveMixedExtremeExponents,
    ExhaustivePositiveFloatsWithSciExponent,
> {
    exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_mixed_extreme_exponents(),
        ExhaustivePositiveFiniteFloatsGenerator,
    )
}

#[derive(Clone, Debug)]
struct ExhaustiveMixedExtremePositiveFiniteFloats(
    ExhaustiveDependentPairs<
        i32,
        Float,
        BitDistributorSequence,
        ExhaustivePositiveFiniteFloatsGenerator,
        ExhaustiveMixedExtremeExponents,
        ExhaustivePositiveFloatsWithSciExponent,
    >,
);

impl Iterator for ExhaustiveMixedExtremePositiveFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.0.next().map(|p| p.1)
    }
}

#[inline]
fn exhaustive_mixed_extreme_positive_finite_floats() -> ExhaustiveMixedExtremePositiveFiniteFloats {
    ExhaustiveMixedExtremePositiveFiniteFloats(
        exhaustive_mixed_extreme_positive_finite_floats_helper(),
    )
}

#[derive(Clone, Debug)]
struct ExhaustiveMixedExtremeNonzeroFiniteFloats {
    toggle: bool,
    xs: ExhaustiveMixedExtremePositiveFiniteFloats,
    x: Float,
}

impl Iterator for ExhaustiveMixedExtremeNonzeroFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.toggle.not_assign();
        Some(if self.toggle {
            self.x = self.xs.next().unwrap();
            self.x.clone()
        } else {
            let mut out = Float::NAN;
            swap(&mut out, &mut self.x);
            -out
        })
    }
}

#[inline]
fn exhaustive_mixed_extreme_nonzero_finite_floats() -> ExhaustiveMixedExtremeNonzeroFiniteFloats {
    ExhaustiveMixedExtremeNonzeroFiniteFloats {
        toggle: false,
        xs: exhaustive_mixed_extreme_positive_finite_floats(),
        x: Float::NAN,
    }
}

type ExhaustiveMixedExtremeFloats =
    Chain<IntoIter<Float>, ExhaustiveMixedExtremeNonzeroFiniteFloats>;

#[inline]
fn exhaustive_mixed_extreme_floats() -> ExhaustiveMixedExtremeFloats {
    alloc::vec![
        Float::NAN,
        Float::INFINITY,
        Float::NEGATIVE_INFINITY,
        Float::ZERO,
        Float::NEGATIVE_ZERO
    ]
    .into_iter()
    .chain(exhaustive_mixed_extreme_nonzero_finite_floats())
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
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
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
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            primitive_int_increasing_inclusive_range(prec_lo, prec_hi),
            FloatPairWithPrecisionRangeGenerator,
        )
        .map(|p| p.1),
    )
}

struct FloatPairWithPrecisionRangesGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Float, Float),
        Box<dyn Iterator<Item = (Float, Float)>>,
    > for FloatPairWithPrecisionRangesGenerator
{
    #[inline]
    fn get_ys(&self, &precs: &(u64, u64)) -> Box<dyn Iterator<Item = (Float, Float)>> {
        Box::new(exhaustive_pairs(
            exhaustive_positive_floats_with_precision(precs.0),
            exhaustive_positive_floats_with_precision(precs.1),
        ))
    }
}

fn exhaustive_float_pairs_with_precisions(precisions: It<(u64, u64)>) -> It<(Float, Float)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            precisions,
            FloatPairWithPrecisionRangesGenerator,
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

pub fn exhaustive_float_pair_gen_var_8() -> It<(Float, Float)> {
    exhaustive_float_pairs_with_precisions(Box::new(
        exhaustive_pairs(
            exhaustive_positive_primitive_ints(),
            primitive_int_increasing_inclusive_range(1, Limb::WIDTH),
        )
        .filter(|&(x, y)| x != y),
    ))
}

pub fn exhaustive_float_pair_gen_var_9() -> It<(Float, Float)> {
    exhaustive_float_pairs_with_precisions(Box::new(
        exhaustive_pairs(
            exhaustive_positive_primitive_ints(),
            primitive_int_increasing_inclusive_range(Limb::WIDTH + 1, u64::MAX),
        )
        .filter(|&(x, y)| x != y),
    ))
}

pub fn exhaustive_float_pair_gen_var_10() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_mixed_extreme_floats(),
    ))
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

pub fn exhaustive_float_float_primitive_float_triple_gen<T: PrimitiveFloat>()
-> It<(Float, Float, T)> {
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

pub fn exhaustive_float_float_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>()
-> It<(Float, Float, T)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_floats(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_float_float_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>()
-> It<(Float, Float, T)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_mixed_extreme_floats(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Float, PrimitiveUnsigned, RoundingMode) --

pub(crate) fn average_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    // Rounding toward negative infinity reports whether the average is exactly representable, and
    // unlike a rational recomputation it stays cheap for extreme exponents.
    rm != Exact || x.clone().average_prec_round(y.clone(), prec, Floor).1 == Equal
}

pub(crate) fn add_prec_round_valid(
    x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.add_prec_round_ref_ref(y, prec, Floor).1 == Equal
    } else if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
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

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| add_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_15()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| average_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn min_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    // Rounding toward negative infinity reports whether the operand that the minimum selects is
    // exactly representable at the target precision.
    rm != Exact || x.min_prec_round_ref_ref(y, prec, Floor).1 == Equal
}

pub(crate) fn max_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    // Rounding toward negative infinity reports whether the operand that the maximum selects is
    // exactly representable at the target precision.
    rm != Exact || x.max_prec_round_ref_ref(y, prec, Floor).1 == Equal
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_16()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| min_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_17()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| max_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn sub_prec_round_valid(
    x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) -> bool {
    if rm != Exact {
        return true;
    }
    if extreme {
        x.sub_prec_round_ref_ref(y, prec, Floor).1 == Equal
    } else if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
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

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| sub_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub(crate) fn mul_prec_round_valid(
    x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.mul_prec_round_ref_ref(y, prec, Floor).1 == Equal
    } else if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
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

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_3()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| mul_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub(crate) fn div_prec_round_valid(
    x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) -> bool {
    if rm != Exact || *y == 0u32 {
        true
    } else if extreme {
        x.div_prec_round_ref_ref(y, prec, Floor).1 == Equal
    } else if let (Ok(rx), Ok(ry)) = (Rational::try_from(x), Rational::try_from(y)) {
        if let Ok(quotient) = Float::try_from(rx / ry) {
            if let Some(min_prec) = quotient.get_min_prec() {
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

// Whether `rm` is a valid rounding mode for computing the remainder of `x` and `y` (with the
// quotient rounded toward zero, or to nearest-even if `nearest_quotient` is set) to precision
// `prec`: `Exact` is only valid when the rounded remainder is exact, which a cheap `Floor` probe
// determines (the remainder is computed with exact integer arithmetic, so the probe never
// balloons).
pub(crate) fn rem_prec_round_valid(
    x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
    nearest_quotient: bool,
) -> bool {
    rm != Exact
        || if nearest_quotient {
            x.ieee_remainder_prec_round_ref_ref(y, prec, Floor)
        } else {
            x.rem_prec_round_ref_ref(y, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn rem_round_valid(
    x: &Float,
    y: &Float,
    rm: RoundingMode,
    nearest_quotient: bool,
) -> bool {
    rem_prec_round_valid(
        x,
        y,
        max(x.significant_bits(), y.significant_bits()),
        rm,
        nearest_quotient,
    )
}

// Whether `rm` is a valid rounding mode for computing the positive difference of `x` and `y` at
// precision `prec`: `Exact` is only valid when the rounded result is exact, decided by a cheap
// `Floor` probe.
pub(crate) fn positive_difference_prec_round_valid(
    x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    rm != Exact || x.positive_difference_prec_round_ref_ref(y, prec, Floor).1 == Equal
}

// Whether `rm` is a valid rounding mode for computing `x` plus or minus (per `sub`) the product of
// `y` and `z` at precision `prec`: `Exact` is only valid when the rounded result is exact, decided
// by a cheap `Floor` probe.
pub(crate) fn add_mul_prec_round_valid(
    x: &Float,
    y: &Float,
    z: &Float,
    prec: u64,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    rm != Exact
        || if sub {
            x.sub_mul_prec_round_ref_ref_ref(y, z, prec, Floor)
        } else {
            x.add_mul_prec_round_ref_ref_ref(y, z, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn positive_difference_round_valid(x: &Float, y: &Float, rm: RoundingMode) -> bool {
    positive_difference_prec_round_valid(x, y, max(x.significant_bits(), y.significant_bits()), rm)
}

// Whether `rm` is a valid rounding mode for computing the positive difference of the `Float` `x`
// and the `Rational` `y` at precision `prec`, in either argument order per `reversed`: `Exact` is
// only valid when the rounded result is exact, decided by a cheap `Floor` probe.
pub(crate) fn positive_difference_rational_prec_round_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
    reversed: bool,
) -> bool {
    rm != Exact
        || if reversed {
            Float::rational_positive_difference_float_prec_round_ref_ref(y, x, prec, Floor)
        } else {
            x.positive_difference_rational_prec_round_ref_ref(y, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn positive_difference_rational_round_valid(
    x: &Float,
    y: &Rational,
    rm: RoundingMode,
    reversed: bool,
) -> bool {
    positive_difference_rational_prec_round_valid(x, y, x.significant_bits(), rm, reversed)
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_4()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| div_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_5()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| add_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_6()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| sub_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_7()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| mul_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_8()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| div_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn agm_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact
        || !x.is_normal()
        || !y.is_normal()
        || *x < 0u32
        || *y < 0u32
        || x == y && Float::from_float_prec_round_ref(x, prec, Floor).1 == Equal
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_9()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| agm_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_10()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| agm_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn sum_prec_round_valid(xs: &[Float], prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || Float::sum_prec_round(xs, prec, Floor).1 == Equal
}

pub(crate) fn sum_round_valid(xs: &[Float], rm: RoundingMode) -> bool {
    rm != Exact || {
        let prec = xs
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        Float::sum_prec_round(xs, prec, Floor).1 == Equal
    }
}

pub fn exhaustive_float_vec_gen() -> It<Vec<Float>> {
    Box::new(exhaustive_vecs(exhaustive_floats()))
}

pub fn exhaustive_float_vec_gen_var_1() -> It<Vec<Float>> {
    Box::new(exhaustive_vecs(exhaustive_mixed_extreme_floats()))
}

pub fn exhaustive_float_vec_unsigned_pair_gen_var_1() -> It<(Vec<Float>, u64)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_floats()),
        exhaustive_positive_primitive_ints::<u64>(),
    ))
}

pub fn exhaustive_float_vec_unsigned_rounding_mode_triple_gen_var_1()
-> It<(Vec<Float>, u64, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(
                exhaustive_vecs(exhaustive_floats()),
                exhaustive_positive_primitive_ints::<u64>(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(xs, prec, rm)| sum_prec_round_valid(xs, *prec, *rm)),
    )
}

pub fn exhaustive_float_vec_unsigned_rounding_mode_triple_gen_var_2()
-> It<(Vec<Float>, u64, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(
                exhaustive_vecs(exhaustive_mixed_extreme_floats()),
                exhaustive_positive_primitive_ints::<u64>(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(xs, prec, rm)| sum_prec_round_valid(xs, *prec, *rm)),
    )
}

pub fn exhaustive_float_vec_rounding_mode_pair_gen_var_1() -> It<(Vec<Float>, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_vecs(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )
        .filter(|(xs, rm)| sum_round_valid(xs, *rm)),
    )
}

pub fn exhaustive_float_vec_rounding_mode_pair_gen_var_2() -> It<(Vec<Float>, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_vecs(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )
        .filter(|(xs, rm)| sum_round_valid(xs, *rm)),
    )
}

pub fn hypot_prec_round_valid(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.hypot_prec_round_ref_ref(y, prec, Floor).1 == Equal
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_24()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| hypot_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_25()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| hypot_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn log_base_float_base_prec_round_valid(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    // Special and degenerate inputs (x or base not finite-positive, or x = 1, or base = 1) yield
    // exact results (0, +-infinity, or NaN) and never panic with Exact.
    if !x.is_finite()
        || *x <= 0u32
        || *x == 1u32
        || !base.is_finite()
        || *base <= 0u32
        || *base == 1u32
    {
        return true;
    }
    // x, base finite positive and not 1: Exact only when log_base(x) is rational and representable.
    log_base_float_base_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub(crate) fn log_base_float_base_round_valid(x: &Float, base: &Float, rm: RoundingMode) -> bool {
    if rm != Exact {
        return true;
    }
    if !x.is_finite()
        || *x <= 0u32
        || *x == 1u32
        || !base.is_finite()
        || *base <= 0u32
        || *base == 1u32
    {
        return true;
    }
    log_base_float_base_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, x.significant_bits()).1 == Equal)
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to
// `Float.log_base_float_base_prec_round`.
pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_11()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_float_base_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_12()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_float_base_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn log_base_float_base_1_plus_x_prec_round_valid(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    // Special and degenerate inputs (x not finite, x <= -1, x = +-0, or base not finite-positive,
    // or base = 1) yield exact results (0, +-infinity, or NaN) and never panic with Exact.
    if !x.is_finite()
        || *x <= -1i32
        || *x == 0u32
        || !base.is_finite()
        || *base <= 0u32
        || *base == 1u32
    {
        return true;
    }
    log_base_float_base_1_plus_x_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub(crate) fn log_base_float_base_1_plus_x_round_valid(
    x: &Float,
    base: &Float,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    if !x.is_finite()
        || *x <= -1i32
        || *x == 0u32
        || !base.is_finite()
        || *base <= 0u32
        || *base == 1u32
    {
        return true;
    }
    log_base_float_base_1_plus_x_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, x.significant_bits()).1 == Equal)
}

// All `(Float, Float, u64, RoundingMode)` that are valid inputs to
// `Float.log_base_float_base_1_plus_x_prec_round`.
pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_13()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_float_base_1_plus_x_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_14()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_float_base_1_plus_x_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_18()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rem_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_19()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rem_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_20()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rem_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_21()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rem_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_22()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| positive_difference_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_float_unsigned_rounding_mode_quadruple_gen_var_23()
-> It<(Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| positive_difference_prec_round_valid(x, y, *prec, *rm)),
    )
}

// -- (Float, Float, Float, PrimitiveUnsigned, RoundingMode) --

// Whether `rm` is a valid rounding mode for computing `x` plus or minus (per `sub`) the product of
// the `Float` `y` and the `Rational` `z` at precision `prec`: `Exact` is only valid when the
// rounded result is exact, decided by a cheap `Floor` probe. Whether `rm` is a valid rounding mode
// for computing a * b plus or minus (per `sub`) c * d at precision `prec`: `Exact` is only valid
// when the rounded result is exact, decided by a cheap `Floor` probe. Whether `rm` is a valid
// rounding mode for computing x * y plus or minus (per `sub`) z * w, with a `Rational` w, at
// precision `prec`: `Exact` is only valid when the rounded result is exact, decided by a cheap
// `Floor` probe.
pub(crate) fn mul_add_mul_rational_prec_round_valid(
    x: &Float,
    y: &Float,
    z: &Float,
    w: &Rational,
    prec: u64,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    rm != Exact
        || if sub {
            x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(y, z, w, prec, Floor)
        } else {
            x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(y, z, w, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn mul_add_mul_rational_round_valid(
    x: &Float,
    y: &Float,
    z: &Float,
    w: &Rational,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    mul_add_mul_rational_prec_round_valid(
        x,
        y,
        z,
        w,
        max!(
            x.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        ),
        rm,
        sub,
    )
}

pub(crate) fn mul_add_mul_prec_round_valid(
    a: &Float,
    b: &Float,
    c: &Float,
    d: &Float,
    prec: u64,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    rm != Exact
        || if sub {
            a.mul_sub_mul_prec_round_ref_ref_ref_ref(b, c, d, prec, Floor)
        } else {
            a.mul_add_mul_prec_round_ref_ref_ref_ref(b, c, d, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn mul_add_mul_round_valid(
    a: &Float,
    b: &Float,
    c: &Float,
    d: &Float,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    mul_add_mul_prec_round_valid(
        a,
        b,
        c,
        d,
        max!(
            a.significant_bits(),
            b.significant_bits(),
            c.significant_bits(),
            d.significant_bits()
        ),
        rm,
        sub,
    )
}

pub(crate) fn add_mul_rational_prec_round_valid(
    x: &Float,
    y: &Float,
    z: &Rational,
    prec: u64,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    rm != Exact
        || if sub {
            x.sub_mul_rational_prec_round_ref_ref_ref(y, z, prec, Floor)
        } else {
            x.add_mul_rational_prec_round_ref_ref_ref(y, z, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn add_mul_rational_round_valid(
    x: &Float,
    y: &Float,
    z: &Rational,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    add_mul_rational_prec_round_valid(
        x,
        y,
        z,
        max(x.significant_bits(), y.significant_bits()),
        rm,
        sub,
    )
}

pub(crate) fn add_mul_round_valid(
    x: &Float,
    y: &Float,
    z: &Float,
    rm: RoundingMode,
    sub: bool,
) -> bool {
    add_mul_prec_round_valid(
        x,
        y,
        z,
        max!(
            x.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        ),
        rm,
        sub,
    )
}

pub fn exhaustive_float_float_float_unsigned_quadruple_gen_var_1() -> It<(Float, Float, Float, u64)>
{
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_floats(),
        exhaustive_positive_primitive_ints::<u64>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_float_float_float_rounding_mode_quadruple_gen_var_1()
-> It<(Float, Float, Float, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, rm)| add_mul_round_valid(x, y, z, *rm, false)),
    )
}

pub fn exhaustive_float_float_float_rounding_mode_quadruple_gen_var_2()
-> It<(Float, Float, Float, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, rm)| add_mul_round_valid(x, y, z, *rm, true)),
    )
}

pub fn exhaustive_float_float_float_unsigned_rounding_mode_quintuple_gen_var_1()
-> It<(Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| add_mul_prec_round_valid(x, y, z, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_float_float_unsigned_rounding_mode_quintuple_gen_var_2()
-> It<(Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| add_mul_prec_round_valid(x, y, z, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_float_float_unsigned_rounding_mode_quintuple_gen_var_3()
-> It<(Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| add_mul_prec_round_valid(x, y, z, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_float_unsigned_rounding_mode_quintuple_gen_var_4()
-> It<(Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| add_mul_prec_round_valid(x, y, z, *prec, *rm, true)),
    )
}

// -- (Float, Float, Float, Float) --

pub fn exhaustive_float_quadruple_gen() -> It<(Float, Float, Float, Float)> {
    Box::new(exhaustive_quadruples_from_single(exhaustive_floats()))
}

// -- (Float, Float, Float, Float, PrimitiveUnsigned) --

pub fn exhaustive_float_float_float_float_unsigned_quintuple_gen_var_1()
-> It<(Float, Float, Float, Float, u64)> {
    Box::new(exhaustive_quintuples_xxxxy_custom_output(
        exhaustive_floats(),
        exhaustive_positive_primitive_ints::<u64>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Float, Float, Float, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_1()
-> It<(Float, Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(a, b, c, d, prec, rm)| {
            mul_add_mul_prec_round_valid(a, b, c, d, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_2()
-> It<(Float, Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(a, b, c, d, prec, rm)| {
            mul_add_mul_prec_round_valid(a, b, c, d, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
-> It<(Float, Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxxy_custom_output(
                exhaustive_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(a, b, c, d, prec, rm)| {
            mul_add_mul_prec_round_valid(a, b, c, d, *prec, *rm, true)
        }),
    )
}

pub fn exhaustive_float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_4()
-> It<(Float, Float, Float, Float, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxxy_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(a, b, c, d, prec, rm)| {
            mul_add_mul_prec_round_valid(a, b, c, d, *prec, *rm, true)
        }),
    )
}

// -- (Float, Float, Float, Float, RoundingMode) --

pub fn exhaustive_float_float_float_float_rounding_mode_quintuple_gen_var_1()
-> It<(Float, Float, Float, Float, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(a, b, c, d, rm)| mul_add_mul_round_valid(a, b, c, d, *rm, false)),
    )
}

pub fn exhaustive_float_float_float_float_rounding_mode_quintuple_gen_var_2()
-> It<(Float, Float, Float, Float, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(a, b, c, d, rm)| mul_add_mul_round_valid(a, b, c, d, *rm, true)),
    )
}

// -- (Float, Float, Float, Rational) --

pub fn exhaustive_float_float_float_rational_quadruple_gen() -> It<(Float, Float, Float, Rational)>
{
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_floats(),
        exhaustive_rationals(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
    ))
}

// -- (Float, Float, Float, Rational, PrimitiveUnsigned) --

pub fn exhaustive_float_float_float_rational_unsigned_quintuple_gen_var_1()
-> It<(Float, Float, Float, Rational, u64)> {
    Box::new(exhaustive_quintuples_xxxyz_custom_output(
        exhaustive_floats(),
        exhaustive_rationals(),
        exhaustive_positive_primitive_ints::<u64>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Float, Float, Rational, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_1()
-> It<(Float, Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxyz_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, w, prec, rm)| {
            mul_add_mul_rational_prec_round_valid(x, y, z, w, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_2()
-> It<(Float, Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxyz_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, w, prec, rm)| {
            mul_add_mul_rational_prec_round_valid(x, y, z, w, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
-> It<(Float, Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxyz_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, w, prec, rm)| {
            mul_add_mul_rational_prec_round_valid(x, y, z, w, *prec, *rm, true)
        }),
    )
}

pub fn exhaustive_float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_4()
-> It<(Float, Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_5_1_to_6(Box::new(lex_pairs(
            exhaustive_quintuples_xxxyz_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, w, prec, rm)| {
            mul_add_mul_rational_prec_round_valid(x, y, z, w, *prec, *rm, true)
        }),
    )
}

// -- (Float, Float, Float, Rational, RoundingMode) --

pub fn exhaustive_float_float_float_rational_rounding_mode_quintuple_gen_var_1()
-> It<(Float, Float, Float, Rational, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxxy_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, w, rm)| mul_add_mul_rational_round_valid(x, y, z, w, *rm, false)),
    )
}

pub fn exhaustive_float_float_float_rational_rounding_mode_quintuple_gen_var_2()
-> It<(Float, Float, Float, Rational, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxxy_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, w, rm)| mul_add_mul_rational_round_valid(x, y, z, w, *rm, true)),
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

pub(crate) fn add_round_valid(x: &Float, y: &Float, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.add_round_ref_ref(y, Floor).1 == Equal
    } else if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
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
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub(crate) fn sub_round_valid(x: &Float, y: &Float, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.sub_round_ref_ref(y, Floor).1 == Equal
    } else if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
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
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
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
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_5() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_6() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_7() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_5(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_8() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_6(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_9() -> It<(Float, Float, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_7(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_10() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_11() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_12() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_13() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_5(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_14() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_6(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_15() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_7(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, false)),
    )
}

pub(crate) fn mul_round_valid(x: &Float, y: &Float, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.mul_round_ref_ref(y, Floor).1 == Equal
    } else if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
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
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_17() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_18() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_19() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_20() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_5(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_21() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_6(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_22() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_7(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, false)),
    )
}

pub(crate) fn div_round_valid(x: &Float, y: &Float, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact || *y == 0u32 {
        true
    } else if extreme {
        x.div_round_ref_ref(y, Floor).1 == Equal
    } else if let (Some(x_prec), Some(y_prec)) = (x.get_prec(), y.get_prec()) {
        if let Ok(quotient) = Float::try_from(Rational::exact_from(x) / Rational::exact_from(y)) {
            if let Some(min_prec) = quotient.get_min_prec() {
                max(x_prec, y_prec) >= min_prec
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

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_23() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_24() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_25() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_3(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_26() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_4(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_27() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_8(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_28() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_9(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_29() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_round_valid(x, y, *rm, true)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_30() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_round_valid(x, y, *rm, true)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_31() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_round_valid(x, y, *rm, true)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_32() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_round_valid(x, y, *rm, true)),
    )
}

pub(crate) fn agm_round_valid(x: &Float, y: &Float, rm: RoundingMode) -> bool {
    rm != Exact || !x.is_normal() || !y.is_normal() || *x < 0u32 || *y < 0u32 || x == y
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_33() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| agm_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_34() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| agm_round_valid(x, y, *rm)),
    )
}

pub(crate) fn hypot_round_valid(x: &Float, y: &Float, rm: RoundingMode) -> bool {
    rm != Exact || {
        let prec = max(x.significant_bits(), y.significant_bits());
        x.hypot_prec_round_ref_ref(y, prec, Floor).1 == Equal
    }
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_43() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| hypot_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_44() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| hypot_round_valid(x, y, *rm)),
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to `Float.log_base_float_base_round`.
pub fn exhaustive_float_float_rounding_mode_triple_gen_var_35() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_float_base_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_36() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_float_base_round_valid(x, y, *rm)),
    )
}

// All `(Float, Float, RoundingMode)` that are valid inputs to
// `Float.log_base_float_base_1_plus_x_round`.
pub fn exhaustive_float_float_rounding_mode_triple_gen_var_37() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_float_base_1_plus_x_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_38() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_mixed_extreme_floats()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_float_base_1_plus_x_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_39() -> It<(Float, Float, RoundingMode)>
{
    Box::new(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_from_single(exhaustive_floats()),
        exhaustive_rounding_modes(),
    ))))
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_40() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rem_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_41() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rem_round_valid(x, y, *rm, true)),
    )
}

pub fn exhaustive_float_float_rounding_mode_triple_gen_var_42() -> It<(Float, Float, RoundingMode)>
{
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_float_pair_gen_var_2(),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| positive_difference_round_valid(x, y, *rm)),
    )
}

// -- (Float, Float, Rational, PrimitiveUnsigned) --

pub fn exhaustive_float_float_rational_unsigned_quadruple_gen_var_1()
-> It<(Float, Float, Rational, u64)> {
    Box::new(exhaustive_quadruples_xxyz_custom_output(
        exhaustive_floats(),
        exhaustive_rationals(),
        exhaustive_positive_primitive_ints::<u64>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Float, Rational, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_float_float_rational_unsigned_rounding_mode_quintuple_gen_var_1()
-> It<(Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxyz_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| {
            add_mul_rational_prec_round_valid(x, y, z, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_float_rational_unsigned_rounding_mode_quintuple_gen_var_2()
-> It<(Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxyz_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| {
            add_mul_rational_prec_round_valid(x, y, z, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
-> It<(Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxyz_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| add_mul_rational_prec_round_valid(x, y, z, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_float_rational_unsigned_rounding_mode_quintuple_gen_var_4()
-> It<(Float, Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_4_1_to_5(Box::new(lex_pairs(
            exhaustive_quadruples_xxyz_custom_output(
                exhaustive_mixed_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, prec, rm)| add_mul_rational_prec_round_valid(x, y, z, *prec, *rm, true)),
    )
}

// -- (Float, Float, Rational, RoundingMode) --

pub fn exhaustive_float_float_rational_rounding_mode_quadruple_gen_var_1()
-> It<(Float, Float, Rational, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, rm)| add_mul_rational_round_valid(x, y, z, *rm, false)),
    )
}

pub fn exhaustive_float_float_rational_rounding_mode_quadruple_gen_var_2()
-> It<(Float, Float, Rational, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, z, rm)| add_mul_rational_round_valid(x, y, z, *rm, true)),
    )
}

// -- (Float, Integer) --

pub fn exhaustive_float_integer_pair_gen() -> It<(Float, Integer)> {
    Box::new(exhaustive_pairs(exhaustive_floats(), exhaustive_integers()))
}

// All `(Float, String)` where the `String` is a valid single-conversion `%R` printf format string.
// The format strings are assembled from their parts (see `format_string_from_parts`), so every
// output is valid by construction; the field width and precision are capped so the strings and
// their outputs stay small.
pub fn exhaustive_float_string_pair_gen_var_1() -> It<(Float, String)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_triples(
            primitive_int_increasing_inclusive_range(0, FLOAT_FORMAT_COMBO_COUNT - 1),
            exhaustive_options(primitive_int_increasing_inclusive_range(0u64, 30)),
            exhaustive_options(primitive_int_increasing_inclusive_range(0u64, 20)),
        )
        .map(|(combo, width, prec)| format_string_from_parts(combo, width, prec)),
    ))
}

// The same as var 1, but over extreme `Float`s, and restricted to the format strings whose output
// stays short for them (see `format_string_output_is_bounded`).
pub fn exhaustive_float_string_pair_gen_var_2() -> It<(Float, String)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
        exhaustive_triples(
            primitive_int_increasing_inclusive_range(0, FLOAT_FORMAT_COMBO_COUNT - 1),
            exhaustive_options(primitive_int_increasing_inclusive_range(0u64, 30)),
            exhaustive_options(primitive_int_increasing_inclusive_range(0u64, 20)),
        )
        .map(|(combo, width, prec)| format_string_from_parts(combo, width, prec))
        .filter(|fmt: &String| format_string_output_is_bounded(fmt)),
    ))
}

pub fn exhaustive_float_integer_pair_gen_var_1() -> It<(Float, Integer)> {
    Box::new(exhaustive_pairs(
        exhaustive_finite_floats(),
        exhaustive_integers(),
    ))
}

pub fn exhaustive_float_integer_pair_gen_var_2() -> It<(Float, Integer)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
        exhaustive_integers(),
    ))
}

// -- (Float, Integer, PrimitiveUnsigned) --

pub fn exhaustive_float_integer_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>()
-> It<(Float, Integer, T)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_floats(),
        exhaustive_integers(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// All `(Float, Integer, T)` where the `Float` is extreme and the `T` is unsigned, small, and
// positive.
pub fn exhaustive_float_integer_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>()
-> It<(Float, Integer, T)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_extreme_floats(),
        exhaustive_integers(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Integer, PrimitiveUnsigned, RoundingMode) --

// Whether `(x, z, prec, rm)` is a valid input to `Float::pow_integer_prec_round`: `Exact` is only
// allowed when the power really is exact at the given precision.
pub fn pow_integer_prec_round_valid(x: &Float, z: &Integer, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.pow_integer_prec_round_ref_ref(z, prec, Floor).1 == Equal
}

pub fn exhaustive_float_integer_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Float, Integer, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_integers(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, z, prec, rm)| pow_integer_prec_round_valid(x, z, *prec, *rm)),
    )
}

// All `(Float, Integer, u64, RoundingMode)` valid for `Float::pow_integer_prec_round`, where the
// `Float` is extreme.
pub fn exhaustive_float_integer_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Float, Integer, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_integers(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, z, prec, rm)| pow_integer_prec_round_valid(x, z, *prec, *rm)),
    )
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

pub fn exhaustive_float_natural_pair_gen_var_2() -> It<(Float, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
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

// -- (Float, Ordering) --

pub fn exhaustive_float_ordering_pair_gen() -> It<(Float, Ordering)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_orderings(),
    ))
}

pub fn exhaustive_float_ordering_pair_gen_var_1() -> It<(Float, Ordering)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
        exhaustive_orderings(),
    ))
}

// -- (Float, PrimitiveFloat) --

pub fn exhaustive_float_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_primitive_floats(),
    ))
}

pub fn exhaustive_float_primitive_float_pair_gen_var_1<T: PrimitiveFloat>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Float, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_float_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>()
-> It<(Float, T, T)> {
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

pub fn exhaustive_float_signed_pair_gen_var_3<T: PrimitiveSigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_extreme_floats(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_float_signed_pair_gen_var_4<T: PrimitiveSigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
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

// -- (Float, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_float_signed_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(Float, T, U)> {
    Box::new(exhaustive_triples(
        exhaustive_floats(),
        exhaustive_signeds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_float_signed_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(Float, T, U)> {
    Box::new(exhaustive_triples(
        exhaustive_extreme_floats(),
        exhaustive_signeds(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Float, PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_1<T: PrimitiveSigned>()
-> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shl_prec_round_valid(x, bits, prec, rm)),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_2<T: PrimitiveSigned>()
-> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shl_prec_round_valid(x, bits, prec, rm)),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_3<T: PrimitiveSigned>()
-> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shr_prec_round_valid(x, bits, prec, rm)),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_4<T: PrimitiveSigned>()
-> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shr_prec_round_valid(x, bits, prec, rm)),
    ))
}

// Whether `(x, n, prec, rm)` is a valid input to `Float::pow_s_prec_round`: `Exact` is only allowed
// when the power really is exact at the given precision.
pub fn pow_s_prec_round_valid(x: &Float, n: i64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.pow_s_prec_round_ref(n, prec, Floor).1 == Equal
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_11()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| pow_s_prec_round_valid(x, n, prec, rm)),
    ))
}

// As `..._var_11`, but the `Float` may have an extreme exponent.
pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_12()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| pow_s_prec_round_valid(x, n, prec, rm)),
    ))
}

// -- (Float, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shl_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_2<T: PrimitiveSigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_extreme_floats(), exhaustive_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shl_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_3<T: PrimitiveSigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_signeds()),
            exhaustive_rounding_modes().filter(|&rm| rm != Exact),
        )
        .filter(|&((ref x, bits), rm)| shl_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shr_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_5<T: PrimitiveSigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_extreme_floats(), exhaustive_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shr_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_6<T: PrimitiveSigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_signeds()),
            exhaustive_rounding_modes().filter(|&rm| rm != Exact),
        )
        .filter(|&((ref x, bits), rm)| shr_round_valid(x, bits, rm)),
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

pub fn exhaustive_float_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_extreme_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_4<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_extreme_floats(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_5<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_floats().filter(|x| *x > 0u32),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_7<T: PrimitiveUnsigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_finite_floats(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_float_unsigned_pair_gen_var_8() -> It<(Float, u8)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_floats(),
        primitive_int_increasing_inclusive_range(2, 36),
    ))
}

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_float_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(Float, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Float, T, U)> {
    Box::new(exhaustive_triples(
        exhaustive_floats(),
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Float, T, U)> {
    Box::new(exhaustive_triples(
        exhaustive_extreme_floats(),
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>() -> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shl_prec_round_valid(x, bits, prec, rm)),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2<
    T: PrimitiveUnsigned,
>() -> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shl_prec_round_valid(x, bits, prec, rm)),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_3<
    T: PrimitiveUnsigned,
>() -> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shr_prec_round_valid(x, bits, prec, rm)),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4<
    T: PrimitiveUnsigned,
>() -> It<(Float, T, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits, prec), rm)| shr_prec_round_valid(x, bits, prec, rm)),
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

pub fn square_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.square_prec_round_ref(prec, Floor).1 == Equal
    } else if let Ok(rx) = Rational::try_from(x) {
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
        .filter(|&((ref x, p), rm)| square_prec_round_valid(x, p, rm, false)),
    ))
}

pub fn reciprocal_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact || *x == 0 {
        true
    } else if extreme {
        x.reciprocal_prec_round_ref(prec, Floor).1 == Equal
    } else if let Ok(rx) = Rational::try_from(x) {
        if let Ok(reciprocal) = Float::try_from(rx.reciprocal()) {
            if let Some(min_prec) = reciprocal.get_min_prec() {
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

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_3() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| reciprocal_prec_round_valid(x, p, rm, false)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_4() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| set_prec_round_valid(x, p, rm)),
    ))
}

pub fn shl_round_valid<T: PrimitiveInt>(x: &Float, bits: T, rm: RoundingMode) -> bool {
    rm != Exact || !x.is_finite() || x.is_zero() || {
        let exponent = x.get_exponent().unwrap();
        if let Ok(bits) = bits.try_into()
            && let Some(new_exponent) = exponent.checked_add(bits)
        {
            return (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&new_exponent);
        }
        false
    }
}

pub fn shl_prec_round_valid<T: PrimitiveInt>(
    x: &Float,
    bits: T,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    rm != Exact || !x.is_normal() || x.shl_prec_round_ref(bits, prec, Floor).1 == Equal
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_5<T: PrimitiveUnsigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shl_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_6<T: PrimitiveUnsigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_extreme_floats(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shl_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_7<T: PrimitiveUnsigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_unsigneds()),
            exhaustive_rounding_modes().filter(|&rm| rm != Exact),
        )
        .filter(|&((ref x, bits), rm)| shl_round_valid(x, bits, rm)),
    ))
}

pub fn shr_round_valid<T: PrimitiveInt>(x: &Float, bits: T, rm: RoundingMode) -> bool {
    rm != Exact || !x.is_finite() || x.is_zero() || {
        let exponent = x.get_exponent().unwrap();
        if let Ok(bits) = bits.try_into()
            && let Some(new_exponent) = exponent.checked_sub(bits)
        {
            return (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&new_exponent);
        }
        false
    }
}

pub fn shr_prec_round_valid<T: PrimitiveInt>(
    x: &Float,
    bits: T,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    rm != Exact || !x.is_normal() || x.shr_prec_round_ref(bits, prec, Floor).1 == Equal
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_8<T: PrimitiveUnsigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shr_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_9<T: PrimitiveUnsigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_extreme_floats(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, bits), rm)| shr_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_10<T: PrimitiveUnsigned>()
-> It<(Float, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_unsigneds()),
            exhaustive_rounding_modes().filter(|&rm| rm != Exact),
        )
        .filter(|&((ref x, bits), rm)| shr_round_valid(x, bits, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_11() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| square_prec_round_valid(x, p, rm, true)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_12() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| reciprocal_prec_round_valid(x, p, rm, true)),
    ))
}

pub fn sqrt_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.sqrt_prec_round_ref(prec, Floor).1 == Equal
}

pub fn reciprocal_sqrt_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.reciprocal_sqrt_prec_round_ref(prec, Floor).1 == Equal
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_13() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| sqrt_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_14() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| sqrt_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_15() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| reciprocal_sqrt_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_16() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| reciprocal_sqrt_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_17() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_floats().filter(|x| *x > 0u32),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes(),
    )))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_18() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_finite_floats(),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes(),
    )))
}

pub fn ln_prec_round_valid(x: &Float, _prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || *x <= 0u32 || *x == 1u32
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_19() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| ln_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_20() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| ln_prec_round_valid(x, p, rm)),
    ))
}

pub fn ln_1_plus_x_prec_round_valid(x: &Float, _prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || *x == 0u32 || *x <= -1i32
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_21() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| ln_1_plus_x_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_22() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| ln_1_plus_x_prec_round_valid(x, p, rm)),
    ))
}

pub fn log_base_2_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact
        || *x <= 0u32
        || x.is_power_of_2()
            && Float::from_signed_prec(i64::from(x.get_exponent().unwrap()) - 1, prec).1 == Equal
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_23() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_2_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_24() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_2_prec_round_valid(x, p, rm)),
    ))
}

pub fn log_base_power_of_2_prec_round_valid(
    x: &Float,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if pow == 0 {
        return false;
    }
    rm != Exact
        || !x.is_finite()
        || *x <= 0u32
        || *x == 1u32
        || x.is_power_of_2()
            && Float::from(i64::from(x.get_exponent().unwrap()) - 1)
                .div_prec(Float::from(pow), prec)
                .1
                == Equal
}

pub fn log_base_prec_round_valid(x: &Float, base: u64, prec: u64, rm: RoundingMode) -> bool {
    if base < 2 {
        return false;
    }
    if rm != Exact || !x.is_finite() || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    // rm == Exact and x is finite, positive, and not 1: exact only when log_base(x) is a rational
    // that is representable at the target precision.
    if base.is_power_of_2() {
        return log_base_power_of_2_prec_round_valid(x, i64::from(base.trailing_zeros()), prec, rm);
    }
    match rational_log_base(x, base) {
        Some(q) => Float::from_rational_prec_round(q, prec, Nearest).1 == Equal,
        None => false,
    }
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base, prec), rm)| log_base_prec_round_valid(x, base, prec, rm)),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base, prec), rm)| log_base_prec_round_valid(x, base, prec, rm)),
    ))
}

pub fn log_base_1_plus_x_prec_round_valid(
    x: &Float,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if base < 2 {
        return false;
    }
    if rm != Exact || !x.is_finite() || *x == 0u32 || *x <= -1i32 {
        return true;
    }
    if base.is_power_of_2() {
        return log_base_power_of_2_1_plus_x_prec_round_valid(
            x,
            i64::from(base.trailing_zeros()),
            prec,
            rm,
        );
    }
    // rm == Exact and x is finite, nonzero, and > -1: exact only when 1 + x = g^m and m / e_base is
    // representable at the target precision.
    log_base_1_plus_x_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_7()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base, prec), rm)| log_base_1_plus_x_prec_round_valid(x, base, prec, rm)),
    ))
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base, prec), rm)| log_base_1_plus_x_prec_round_valid(x, base, prec, rm)),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_5()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_nonzero_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow, prec), rm)| log_base_power_of_2_prec_round_valid(x, pow, prec, rm)),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_6()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_nonzero_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow, prec), rm)| log_base_power_of_2_prec_round_valid(x, pow, prec, rm)),
    ))
}

pub fn log_base_2_1_plus_x_prec_round_valid(x: &Float, _prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || *x == 0u32 || *x <= -1i32
}

pub fn log_base_power_of_2_1_plus_x_prec_round_valid(
    x: &Float,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if pow == 0 {
        return false;
    }
    // For `Exact` the result must be representable: this happens when `x` is special (NaN,
    // infinite, zero, or at most -1, all of which give NaN, an infinity, or a signed zero), or when
    // `1 + x = 2^m` and `m / pow` rounds exactly at `prec`.
    rm != Exact
        || !x.is_finite()
        || *x == 0u32
        || *x <= -1i32
        || crate::float::arithmetic::log_base_2_1_plus_x::log_base_2_1_plus_x_exact(x)
            .is_some_and(|m| Float::from(m).div_prec(Float::from(pow), prec).1 == Equal)
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_7()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_nonzero_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow, prec), rm)| {
            log_base_power_of_2_1_plus_x_prec_round_valid(x, pow, prec, rm)
        }),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_8()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_nonzero_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow, prec), rm)| {
            log_base_power_of_2_1_plus_x_prec_round_valid(x, pow, prec, rm)
        }),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_25() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_2_1_plus_x_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_26() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_2_1_plus_x_prec_round_valid(x, p, rm)),
    ))
}

// Whether `(x, n, prec, rm)` is a valid input to `Float::pow_u_prec_round`: `Exact` is only allowed
// when the power really is exact at the given precision.
pub fn pow_u_prec_round_valid(x: &Float, n: u64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.pow_u_prec_round_ref(n, prec, Floor).1 == Equal
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| pow_u_prec_round_valid(x, n, prec, rm)),
    ))
}

// As `..._var_9`, but the `Float` may have an extreme exponent.
pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_10()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| pow_u_prec_round_valid(x, n, prec, rm)),
    ))
}

// Whether `(exp, base, prec, rm)` is a valid input to `Float::unsigned_pow_prec_round`: `Exact` is
// only allowed when base^exp is exactly representable at the given precision.
pub fn unsigned_pow_prec_round_valid(exp: &Float, base: u64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || Float::unsigned_pow_prec_round_ref(base, exp, prec, Floor).1 == Equal
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| unsigned_pow_prec_round_valid(x, n, prec, rm)),
    ))
}

// As `..._var_11`, but the `Float` may have an extreme exponent.
pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_12()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| unsigned_pow_prec_round_valid(x, n, prec, rm)),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// Whether `(x, y, prec, rm)` is a valid input to `Float::unsigned_pow_unsigned_prec_round`: `Exact`
// is only allowed when x^y is exactly representable at the given precision.
pub fn unsigned_pow_unsigned_prec_round_valid(x: u64, y: u64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || Float::unsigned_pow_unsigned_prec_round(x, y, prec, Floor).1 == Equal
}

pub fn exhaustive_unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(u64, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_unsigneds(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y, prec), rm)| unsigned_pow_unsigned_prec_round_valid(x, y, prec, rm)),
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

pub fn exhaustive_float_rational_pair_gen_var_2() -> It<(Float, Rational)> {
    Box::new(exhaustive_pairs(
        exhaustive_extreme_floats(),
        exhaustive_rationals(),
    ))
}

// -- (Float, Rational, PrimitiveUnsigned) --

pub fn exhaustive_float_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>()
-> It<(Float, Rational, T)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_floats(),
        exhaustive_rationals(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_float_rational_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>()
-> It<(Float, Rational, T)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_extreme_floats(),
        exhaustive_rationals(),
        exhaustive_positive_primitive_ints::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Float, Rational, PrimitiveUnsigned, RoundingMode) --

pub(crate) fn add_rational_prec_round_valid(
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

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| add_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn sub_rational_prec_round_valid(
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

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| sub_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn mul_rational_prec_round_valid(
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

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_3()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| mul_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn div_rational_prec_round_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact || *y == 0u32 {
        return true;
    }
    if let Ok(rx) = Rational::try_from(x) {
        if let Ok(quotient) = Float::try_from(rx / y) {
            if let Some(min_prec) = quotient.get_min_prec() {
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

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_4()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| div_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub(crate) fn rational_div_float_prec_round_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact || *x == 0u32 {
        return true;
    }
    if let Ok(rx) = Rational::try_from(x) {
        if let Ok(quotient) = Float::try_from(y / rx) {
            if let Some(min_prec) = quotient.get_min_prec() {
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

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_5()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rational_div_float_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_6()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| add_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_7()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| sub_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_8()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| mul_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_9()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| div_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_10()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rational_div_float_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn log_base_rational_base_prec_round_valid(
    x: &Float,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if *base <= 1u32 {
        // The base must be greater than 1.
        return false;
    }
    if rm != Exact || !x.is_finite() || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    // rm == Exact and x is finite, positive, and not 1: exact only when log_base(x) is rational and
    // representable at the target precision.
    rational_log_base_rational_base(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_11()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_rational_base_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_12()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_rational_base_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn pow_rational_prec_round_valid(x: &Float, y: &Rational, prec: u64, rm: RoundingMode) -> bool {
    // For `Exact`, the power is representable exactly iff computing it toward negative infinity is
    // already exact.
    rm != Exact || Float::pow_rational_prec_round_ref_ref(x, y, prec, Floor).1 == Equal
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_15()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| pow_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_16()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| pow_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_17()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rem_rational_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_18()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rem_rational_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_19()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rational_rem_float_prec_round_valid(y, x, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_20()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rational_rem_float_prec_round_valid(y, x, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_21()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| min_max_rational_prec_round_valid(x, y, *prec, *rm, false)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_22()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| min_max_rational_prec_round_valid(x, y, *prec, *rm, true)),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_23()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| {
            positive_difference_rational_prec_round_valid(x, y, *prec, *rm, false)
        }),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_24()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| {
            positive_difference_rational_prec_round_valid(x, y, *prec, *rm, true)
        }),
    )
}

pub fn log_base_rational_base_1_plus_x_prec_round_valid(
    x: &Float,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if *base <= 1u32 {
        // The base must be greater than 1.
        return false;
    }
    if rm != Exact || !x.is_finite() || *x == 0u32 || *x <= -1i32 {
        return true;
    }
    // rm == Exact and x is finite, nonzero, and > -1: exact only when 1 + x = g^m and m / e_base is
    // representable at the target precision.
    rational_log_base_rational_base_1_plus_x(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub(crate) fn log_base_rational_base_1_plus_x_round_valid(
    x: &Float,
    base: &Rational,
    rm: RoundingMode,
) -> bool {
    if *base <= 1u32 {
        return false;
    }
    if rm != Exact || !x.is_finite() || *x == 0u32 || *x <= -1i32 {
        return true;
    }
    rational_log_base_rational_base_1_plus_x(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, x.significant_bits()).1 == Equal)
}

// All `(Float, Rational, u64, RoundingMode)` that are valid inputs to
// `Float.log_base_rational_base_1_plus_x_prec_round`.
pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| {
            log_base_rational_base_1_plus_x_prec_round_valid(x, y, *prec, *rm)
        }),
    )
}

pub fn exhaustive_float_rational_unsigned_rounding_mode_quadruple_gen_var_14()
-> It<(Float, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_extreme_floats(),
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| {
            log_base_rational_base_1_plus_x_prec_round_valid(x, y, *prec, *rm)
        }),
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

pub(crate) fn add_rational_round_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact {
        true
    } else if let Some(x_prec) = x.get_prec() {
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

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_1()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_rational_round_valid(x, y, *rm)),
    )
}

pub(crate) fn sub_rational_round_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact {
        true
    } else if let Some(x_prec) = x.get_prec() {
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

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_2()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_rational_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_3()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(exhaustive_triples(
        exhaustive_floats(),
        exhaustive_rationals(),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    ))
}

pub(crate) fn mul_rational_round_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact {
        true
    } else if let Some(x_prec) = x.get_prec() {
        if let Ok(product) = Float::try_from(Rational::exact_from(x) * y) {
            if let Some(min_prec) = product.get_min_prec() {
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

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_4()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_rational_round_valid(x, y, *rm)),
    )
}

pub(crate) fn div_rational_round_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact || *y == 0 {
        true
    } else if let Some(x_prec) = x.get_prec() {
        if let Ok(quotient) = Float::try_from(Rational::exact_from(x) / y) {
            if let Some(min_prec) = quotient.get_min_prec() {
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

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_5()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_rational_round_valid(x, y, *rm)),
    )
}

pub(crate) fn rational_div_float_round_valid(x: &Float, y: &Rational, rm: RoundingMode) -> bool {
    if rm != Exact || *x == 0u32 {
        true
    } else if let Some(prec) = x.get_prec() {
        if let Ok(quotient) = Float::try_from(y / Rational::exact_from(x)) {
            if let Some(min_prec) = quotient.get_min_prec() {
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

// Whether `rm` is a valid rounding mode for computing the remainder of the `Float` `x` by the
// `Rational` `y` (quotient toward zero, or nearest-even if `nearest_quotient`) at precision `prec`:
// `Exact` is only valid when the rounded remainder is exact, decided by a cheap `Floor` probe.
pub(crate) fn rem_rational_prec_round_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
    nearest_quotient: bool,
) -> bool {
    rm != Exact
        || if nearest_quotient {
            x.ieee_remainder_rational_prec_round_ref_ref(y, prec, Floor)
        } else {
            x.rem_rational_prec_round_ref_ref(y, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn rem_rational_round_valid(
    x: &Float,
    y: &Rational,
    rm: RoundingMode,
    nearest_quotient: bool,
) -> bool {
    rem_rational_prec_round_valid(x, y, x.significant_bits(), rm, nearest_quotient)
}

// The reversed direction: the remainder of the `Rational` `x` by the `Float` `y`.
pub(crate) fn rational_rem_float_prec_round_valid(
    x: &Rational,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
    nearest_quotient: bool,
) -> bool {
    rm != Exact
        || if nearest_quotient {
            Float::rational_ieee_remainder_float_prec_round_ref_ref(x, y, prec, Floor)
        } else {
            Float::rational_rem_float_prec_round_ref_ref(x, y, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn rational_rem_float_round_valid(
    x: &Rational,
    y: &Float,
    rm: RoundingMode,
    nearest_quotient: bool,
) -> bool {
    rational_rem_float_prec_round_valid(x, y, y.significant_bits(), rm, nearest_quotient)
}

// Whether `rm` is valid for the mixed min or max: `Exact` is only valid when rounding the winner is
// exact.
pub(crate) fn min_max_rational_prec_round_valid(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
    is_max: bool,
) -> bool {
    rm != Exact
        || if is_max {
            x.max_rational_prec_round_ref_ref(y, prec, Floor)
        } else {
            x.min_rational_prec_round_ref_ref(y, prec, Floor)
        }
        .1 == Equal
}

pub(crate) fn min_max_rational_round_valid(
    x: &Float,
    y: &Rational,
    rm: RoundingMode,
    is_max: bool,
) -> bool {
    min_max_rational_prec_round_valid(x, y, x.significant_bits(), rm, is_max)
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_6()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rational_div_float_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_7()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| add_rational_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_8()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| sub_rational_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_9()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| mul_rational_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_10()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| div_rational_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_11()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rational_div_float_round_valid(x, y, *rm)),
    )
}

pub(crate) fn log_base_rational_base_round_valid(
    x: &Float,
    base: &Rational,
    rm: RoundingMode,
) -> bool {
    if *base <= 1u32 {
        return false;
    }
    if rm != Exact || !x.is_finite() || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    rational_log_base_rational_base(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, x.significant_bits()).1 == Equal)
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_12()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_rational_base_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_13()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_rational_base_round_valid(x, y, *rm)),
    )
}

// All `(Float, Rational, RoundingMode)` that are valid inputs to
// `Float.log_base_rational_base_1_plus_x_round`.
pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_14()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_rational_base_1_plus_x_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_15()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| log_base_rational_base_1_plus_x_round_valid(x, y, *rm)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_16()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rem_rational_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_17()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rem_rational_round_valid(x, y, *rm, true)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_18()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rational_rem_float_round_valid(y, x, *rm, false)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_19()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| rational_rem_float_round_valid(y, x, *rm, true)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_20()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| min_max_rational_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_21()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| min_max_rational_round_valid(x, y, *rm, true)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_22()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| positive_difference_rational_round_valid(x, y, *rm, false)),
    )
}

pub fn exhaustive_float_rational_rounding_mode_triple_gen_var_23()
-> It<(Float, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_floats(), exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, rm)| positive_difference_rational_round_valid(x, y, *rm, true)),
    )
}

// -- (Float, RoundingMode) --

pub fn exhaustive_float_rounding_mode_pair_gen() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(exhaustive_floats(), exhaustive_rounding_modes()))
}

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
pub fn exhaustive_float_rounding_mode_pair_gen_var_4<T: PrimitiveUnsigned>()
-> It<(Float, RoundingMode)>
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
pub fn exhaustive_float_rounding_mode_pair_gen_var_5<T: PrimitiveSigned>()
-> It<(Float, RoundingMode)>
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
pub fn exhaustive_float_rounding_mode_pair_gen_var_6<T: PrimitiveFloat>()
-> It<(Float, RoundingMode)>
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| *rm != Exact || T::convertible_from(f)),
    )
}

pub(crate) fn square_round_valid(x: &Float, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact {
        true
    } else if extreme {
        x.square_round_ref(Floor).1 == Equal
    } else if let Some(x_prec) = x.get_prec() {
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
            .filter(|(f, rm)| square_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_8() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(1, Limb::WIDTH - 1),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_9() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_positive_floats_with_precision(Limb::WIDTH),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm, false)),
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
        .filter(|(f, rm)| square_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_11() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_positive_floats_with_precision(Limb::WIDTH << 1),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| square_round_valid(f, *rm, false)),
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
        .filter(|(f, rm)| square_round_valid(f, *rm, false)),
    )
}

pub(crate) fn reciprocal_round_valid(x: &Float, rm: RoundingMode, extreme: bool) -> bool {
    if rm != Exact || *x == 0 {
        true
    } else if extreme {
        x.reciprocal_round_ref(Floor).1 == Equal
    } else if let Some(x_prec) = x.get_prec() {
        if let Ok(reciprocal) = Float::try_from(Rational::exact_from(x).reciprocal()) {
            if let Some(min_prec) = reciprocal.get_min_prec() {
                x_prec >= min_prec
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

pub fn exhaustive_float_rounding_mode_pair_gen_var_13() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| reciprocal_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_14() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(1, Limb::WIDTH - 1),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| reciprocal_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_15() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_positive_floats_with_precision(Limb::WIDTH),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| reciprocal_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_16() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(
                Limb::WIDTH + 1,
                (Limb::WIDTH << 1) - 1,
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| reciprocal_round_valid(f, *rm, false)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_17() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(Limb::WIDTH + 1, u64::MAX),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| reciprocal_round_valid(f, *rm, false)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn exhaustive_float_rounding_mode_pair_gen_var_18<T: PrimitiveUnsigned>()
-> It<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| unsigned_rounding_from_float_valid::<T>(f, *rm)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn exhaustive_float_rounding_mode_pair_gen_var_19<T: PrimitiveSigned>()
-> It<(Float, RoundingMode)>
where
    Float: PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| signed_rounding_from_float_valid::<T>(f, *rm)),
    )
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn exhaustive_float_rounding_mode_pair_gen_var_20<T: PrimitiveFloat>()
-> It<(Float, RoundingMode)>
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| *rm != Exact || T::convertible_from(f)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_21() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_extreme_floats(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_22() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| square_round_valid(f, *rm, true)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_23() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| reciprocal_round_valid(f, *rm, true)),
    )
}

pub(crate) fn sqrt_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || x.sqrt_round_ref(Floor).1 == Equal
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_24() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_25() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(1, Limb::WIDTH - 1),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_26() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_positive_floats_with_precision(Limb::WIDTH),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_27() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range(
                Limb::WIDTH + 1,
                (Limb::WIDTH << 1) - 1,
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_28() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_floats_with_precision_inclusive_range((Limb::WIDTH << 1) + 1, u64::MAX),
            exhaustive_rounding_modes(),
        )
        .filter(|(f, rm)| sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_29() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| sqrt_round_valid(f, *rm)),
    )
}

pub(crate) fn reciprocal_sqrt_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || x.reciprocal_sqrt_round_ref(Floor).1 == Equal
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_30() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| reciprocal_sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_31() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| reciprocal_sqrt_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_32() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_floats().filter(|x| *x > 0u32),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_33() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_finite_floats(),
        exhaustive_rounding_modes(),
    ))
}

pub(crate) fn ln_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || *x <= 0u32 || *x == 1u32
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_34() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| ln_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_35() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| ln_round_valid(f, *rm)),
    )
}

pub(crate) fn ln_1_plus_x_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || *x == 0u32 || *x <= -1i32
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_36() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| ln_1_plus_x_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_37() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| ln_1_plus_x_round_valid(f, *rm)),
    )
}

pub(crate) fn log_base_2_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact
        || *x <= 0u32
        || x.is_power_of_2()
            && Float::from_signed_prec(
                i64::from(x.get_exponent().unwrap()) - 1,
                x.get_prec().unwrap(),
            )
            .1 == Equal
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_38() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_2_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_39() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_2_round_valid(f, *rm)),
    )
}

pub(crate) fn log_base_power_of_2_round_valid(x: &Float, pow: i64, rm: RoundingMode) -> bool {
    if pow == 0 {
        return false;
    }
    rm != Exact
        || !x.is_finite()
        || *x <= 0u32
        || *x == 1u32
        || x.is_power_of_2()
            && Float::from(i64::from(x.get_exponent().unwrap()) - 1)
                .div_prec(Float::from(pow), x.significant_bits())
                .1
                == Equal
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_7() -> It<(Float, i64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_nonzero_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow), rm)| log_base_power_of_2_round_valid(x, pow, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_8() -> It<(Float, i64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_extreme_floats(), exhaustive_nonzero_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow), rm)| log_base_power_of_2_round_valid(x, pow, rm)),
    ))
}

pub(crate) fn log_base_round_valid(x: &Float, base: u64, rm: RoundingMode) -> bool {
    if base < 2 {
        return false;
    }
    if rm != Exact || !x.is_finite() || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    if base.is_power_of_2() {
        return log_base_power_of_2_round_valid(x, i64::from(base.trailing_zeros()), rm);
    }
    match rational_log_base(x, base) {
        Some(q) => Float::from_rational_prec_round(q, x.significant_bits(), Nearest).1 == Equal,
        None => false,
    }
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_27() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base), rm)| log_base_round_valid(x, base, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_28() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base), rm)| log_base_round_valid(x, base, rm)),
    ))
}

pub(crate) fn log_base_1_plus_x_round_valid(x: &Float, base: u64, rm: RoundingMode) -> bool {
    if base < 2 {
        return false;
    }
    if rm != Exact || !x.is_finite() || *x == 0u32 || *x <= -1i32 {
        return true;
    }
    if base.is_power_of_2() {
        return log_base_power_of_2_1_plus_x_round_valid(x, i64::from(base.trailing_zeros()), rm);
    }
    log_base_1_plus_x_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, x.significant_bits()).1 == Equal)
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_32() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base), rm)| log_base_1_plus_x_round_valid(x, base, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_33() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, base), rm)| log_base_1_plus_x_round_valid(x, base, rm)),
    ))
}

pub fn log_base_10_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    if rm != Exact || !x.is_finite() || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    // rm == Exact and x is finite, positive, and not 1: exact only when x = 10^n and the integer n
    // is representable at the target precision.
    match float_is_power_of_10(x) {
        Some(n) => Float::from_unsigned_prec_round(n, prec, Nearest).1 == Equal,
        None => false,
    }
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_29() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_10_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_30() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_10_prec_round_valid(x, p, rm)),
    ))
}

// Valid inputs to `Float::log_base_10_prec_round`, excluding those with `Exact` (a non-`Exact`
// rounding mode is valid for any `Float`).
pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_31() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

pub(crate) fn log_base_2_1_plus_x_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || *x == 0u32 || *x <= -1i32
}

pub fn log_base_power_of_2_1_plus_x_round_valid(x: &Float, pow: i64, rm: RoundingMode) -> bool {
    if pow == 0 {
        return false;
    }
    rm != Exact
        || !x.is_finite()
        || *x == 0u32
        || *x <= -1i32
        || crate::float::arithmetic::log_base_2_1_plus_x::log_base_2_1_plus_x_exact(x).is_some_and(
            |m| {
                Float::from(m)
                    .div_prec(Float::from(pow), x.significant_bits())
                    .1
                    == Equal
            },
        )
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_9() -> It<(Float, i64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_nonzero_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow), rm)| log_base_power_of_2_1_plus_x_round_valid(x, pow, rm)),
    ))
}

pub fn exhaustive_float_signed_rounding_mode_triple_gen_var_10() -> It<(Float, i64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_extreme_floats(), exhaustive_nonzero_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, pow), rm)| log_base_power_of_2_1_plus_x_round_valid(x, pow, rm)),
    ))
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_40() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_2_1_plus_x_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_41() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_2_1_plus_x_round_valid(f, *rm)),
    )
}

pub(crate) fn log_base_10_round_valid(x: &Float, rm: RoundingMode) -> bool {
    if rm != Exact || !x.is_finite() || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    match float_is_power_of_10(x) {
        Some(n) => Float::from_unsigned_prec_round(n, x.significant_bits(), Nearest).1 == Equal,
        None => false,
    }
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_42() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_10_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_43() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_10_round_valid(f, *rm)),
    )
}

// Valid inputs to `Float::log_base_10_round`, excluding those with `Exact` (a non-`Exact` rounding
// mode is valid for any `Float`).
pub fn exhaustive_float_rounding_mode_pair_gen_var_44() -> It<(Float, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_floats(),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    ))
}

pub fn log_base_10_1_plus_x_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    if rm != Exact || !x.is_finite() || *x == 0u32 || *x <= -1i32 {
        return true;
    }
    // rm == Exact and x is finite, nonzero, and greater than -1: exact only when 1 + x = 10^m and
    // the result m is representable at the target precision.
    log_base_1_plus_x_rational(x, 10).is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_34() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_10_1_plus_x_prec_round_valid(x, p, rm)),
    ))
}

pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_35() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| log_base_10_1_plus_x_prec_round_valid(x, p, rm)),
    ))
}

pub(crate) fn log_base_10_1_plus_x_round_valid(x: &Float, rm: RoundingMode) -> bool {
    if rm != Exact || !x.is_finite() || *x == 0u32 || *x <= -1i32 {
        return true;
    }
    log_base_1_plus_x_rational(x, 10)
        .is_some_and(|q| Float::from_rational_prec(q, x.significant_bits()).1 == Equal)
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_45() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_10_1_plus_x_round_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_46() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_extreme_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| log_base_10_1_plus_x_round_valid(f, *rm)),
    )
}

// -- (Float, ToSciOptions) --

// Whether `to_sci_string` accepts `(x, options)` and produces a string of manageable length: the
// pair must be valid per `to_sci_valid`, and combinations that would make the digit count huge (a
// large precision or scale, or a `Complete` or `Scale` conversion of a `Float` with a large
// exponent) are excluded.
pub(crate) fn float_to_sci_options_valid(x: &Float, options: ToSciOptions) -> bool {
    const MAX_DIGITS: u64 = 10_000;
    if !to_sci_valid(x, options) {
        return false;
    }
    let exponent_small = match x.get_exponent() {
        Some(exponent) => u64::from(exponent.unsigned_abs()) <= MAX_DIGITS,
        None => true,
    };
    match options.get_size_options() {
        SciSizeOptions::Precision(precision) => precision <= MAX_DIGITS,
        SciSizeOptions::Scale(scale) => scale <= MAX_DIGITS && exponent_small,
        SciSizeOptions::Complete => exponent_small,
    }
}

pub fn exhaustive_float_to_sci_options_pair_gen() -> It<(Float, ToSciOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_to_sci_options(),
    ))
}

pub fn exhaustive_float_to_sci_options_pair_gen_var_1() -> It<(Float, ToSciOptions)> {
    Box::new(
        exhaustive_pairs(exhaustive_floats(), exhaustive_to_sci_options())
            .filter(|(x, options)| float_to_sci_options_valid(x, *options)),
    )
}

// The same as var 1, but over extreme `Float`s. `float_to_sci_options_valid` rejects an extreme
// exponent paired with `Scale` or `Complete` sizing, since writing every digit of such a value
// would take hundreds of millions of them, so what survives is the digit-count-bounded `Precision`
// sizing.
pub fn exhaustive_float_to_sci_options_pair_gen_var_2() -> It<(Float, ToSciOptions)> {
    Box::new(
        exhaustive_pairs(exhaustive_extreme_floats(), exhaustive_to_sci_options())
            .filter(|(x, options)| float_to_sci_options_valid(x, *options)),
    )
}

// -- (Integer, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 2 are in malachite-nz.

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_3()
-> It<(Integer, u64, RoundingMode)> {
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

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_4()
-> It<(Integer, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (Natural, PrimitiveUnsigned, RoundingMode) --

// var 1 is in malachite-nz.

pub fn exhaustive_natural_unsigned_rounding_mode_triple_gen_var_2()
-> It<(Natural, u64, RoundingMode)> {
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

pub fn exhaustive_natural_unsigned_rounding_mode_triple_gen_var_3()
-> It<(Natural, u64, RoundingMode)> {
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

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveFloat>()
-> It<(T, u64, RoundingMode)>
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

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveFloat>()
-> It<(T, u64, RoundingMode)>
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

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveSigned>()
-> It<(T, u64, RoundingMode)> {
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

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveSigned>()
-> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_signeds::<T>(),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_5() -> It<(i64, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((pow, _), rm)| {
            rm != Exact
                || (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
                    .contains(&i32::saturating_from(pow).saturating_add(1))
        }),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(base, prec)` pairs with `base` in `[2, 62]` and `prec` positive: the inputs of
// `get_str_digit_count`.
pub fn exhaustive_unsigned_pair_gen_var_51() -> It<(u64, u64)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range::<u64>(2, 62),
        exhaustive_positive_primitive_ints::<u64>(),
    ))
}

// The name and the return type are both fixed by convention, and rustfmt cannot break the signature
// anywhere that brings it under the limit.
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_10() -> It<(u64, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range::<u64>(2, 62),
            exhaustive_positive_primitive_ints::<u64>(),
        ),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// vars 1 through 5 are in malachite-base.

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_6<T: PrimitiveUnsigned>()
-> It<(T, u64, RoundingMode)> {
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

// All `(n, prec, RoundingMode)` where `Exact` is only used when n is 0 or 1. Valid for both
// `Float::ln_unsigned_prec_round` (log(n) is exact iff n is 0 or 1) and
// `Float::sqrt_unsigned_prec_round` (n = 0 and n = 1 are perfect squares, so sqrt(n) is exact).
pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_8<T: PrimitiveUnsigned>()
-> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_unsigneds::<T>(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, _prec), rm)| rm != Exact || n.significant_bits() <= 1),
    ))
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_7<T: PrimitiveUnsigned>()
-> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_unsigneds::<T>(),
            exhaustive_positive_primitive_ints(),
        ),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

// All `(u64, u64, RoundingMode)` that are valid inputs to `Float::factorial_prec_round`, with the
// first element small.
type UURM = It<(u64, u64, RoundingMode)>;

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_11() -> UURM {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                primitive_int_increasing_inclusive_range(0u64, 600),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((n, prec), rm)| {
            rm != Exact || Float::factorial_prec_round(n, prec, Floor).1 == Equal
        }),
    ))
}

// -- (Rational, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_1()
-> It<(Rational, u64, RoundingMode)> {
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

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_2()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != Exact),
    )))
}

pub fn sqrt_rational_prec_round_valid(x: &Rational, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || Float::sqrt_rational_prec_round_ref(x, prec, Floor).1 == Equal
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_3()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| sqrt_rational_prec_round_valid(n, prec, rm)),
    ))
}

pub fn reciprocal_sqrt_rational_prec_round_valid(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    rm != Exact || Float::reciprocal_sqrt_rational_prec_round_ref(x, prec, Floor).1 == Equal
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_4()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| reciprocal_sqrt_rational_prec_round_valid(n, prec, rm)),
    ))
}

pub fn agm_rational_prec_round_valid(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    rm != Exact
        || *x < 0u32
        || *y < 0u32
        || x == y && Float::from_rational_prec_round_ref(x, prec, Floor).1 == Equal
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_5()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_non_negative_rationals(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| {
            rm != Exact
                || n.denominator_ref().is_power_of_2()
                    && n.numerator_ref().significant_bits() <= prec
        }),
    ))
}

pub fn ln_rational_prec_round_valid(x: &Rational, _prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || *x <= 0 || *x == 1u32
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_6()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| ln_rational_prec_round_valid(n, prec, rm)),
    ))
}

pub fn log_base_2_rational_prec_round_valid(x: &Rational, prec: u64, rm: RoundingMode) -> bool {
    // `checked_log_base_2` panics for nonpositive arguments, so the order of these tests matters.
    rm != Exact
        || *x <= 0
        || x.checked_log_base_2()
            .is_some_and(|k| Float::from_signed_prec(k, prec).1 == Equal)
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_7()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| log_base_2_rational_prec_round_valid(n, prec, rm)),
    ))
}

pub fn log_base_10_rational_prec_round_valid(x: &Rational, prec: u64, rm: RoundingMode) -> bool {
    // `checked_log_base` panics for nonpositive arguments, so the order of these tests matters.
    rm != Exact
        || *x <= 0
        || x.checked_log_base(10)
            .is_some_and(|m| Float::from_signed_prec(m, prec).1 == Equal)
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_9()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| log_base_10_rational_prec_round_valid(n, prec, rm)),
    ))
}

pub fn exp_rational_prec_round_valid(x: &Rational, _prec: u64, rm: RoundingMode) -> bool {
    // exp of a nonzero rational is transcendental, so `Exact` is valid only for x = 0 (exp(0) = 1).
    rm != Exact || *x == 0u32
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_10()
-> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, prec), rm)| exp_rational_prec_round_valid(n, prec, rm)),
    ))
}

// -- (Rational, PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

pub fn log_base_power_of_2_rational_prec_round_valid(
    x: &Rational,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if pow == 0 {
        return false;
    }
    // `checked_log_base_2` panics for nonpositive arguments, so the order of these tests matters.
    rm != Exact
        || *x <= 0
        || x.checked_log_base_2()
            .is_some_and(|k| Float::from(k).div_prec(Float::from(pow), prec).1 == Equal)
}

pub fn exhaustive_rational_signed_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Rational, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                exhaustive_nonzero_signeds(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, pow, prec), rm)| {
            log_base_power_of_2_rational_prec_round_valid(n, pow, prec, rm)
        }),
    ))
}

// -- (Rational, PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

pub fn log_base_rational_prec_round_valid(
    x: &Rational,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if base < 2 {
        return false;
    }
    // `checked_log_base` panics for nonpositive arguments, so the order of these tests matters.
    if rm != Exact || *x <= 0 {
        return true;
    }
    if base.is_power_of_2() {
        return log_base_power_of_2_rational_prec_round_valid(
            x,
            i64::from(base.trailing_zeros()),
            prec,
            rm,
        );
    }
    // rm == Exact and x > 0: exact only when log_base(x) is a rational representable at `prec`.
    rational_log_base_of_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

pub fn exhaustive_rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Rational, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                primitive_int_increasing_inclusive_range(2, u64::MAX),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, base, prec), rm)| log_base_rational_prec_round_valid(n, base, prec, rm)),
    ))
}

// Whether `(exp, base, prec, rm)` is a valid input to `Float::unsigned_pow_rational_prec_round`:
// `Exact` is only allowed when base^exp is exactly representable at the given precision.
pub fn unsigned_pow_rational_prec_round_valid(
    exp: &Rational,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    rm != Exact || Float::unsigned_pow_rational_prec_round_ref(base, exp, prec, Floor).1 == Equal
}

pub fn exhaustive_rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Rational, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref q, k, prec), rm)| unsigned_pow_rational_prec_round_valid(q, k, prec, rm)),
    ))
}

// -- (Rational, Rational, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_rational_rational_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Rational, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| agm_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn log_base_rational_rational_base_prec_round_valid(
    x: &Rational,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if *base <= 1u32 {
        // The base must be greater than 1.
        return false;
    }
    if rm != Exact || *x <= 0u32 || *x == 1u32 {
        return true;
    }
    // rm == Exact and x is positive and not 1: exact only when log_base(x) is rational and
    // representable at the target precision.
    rational_log_base_rational_rational_base(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

// All `(Rational, Rational, u64, RoundingMode)` that are valid inputs to
// `Float::log_base_rational_rational_base_prec_round`.
pub fn exhaustive_rational_rational_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Rational, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| {
            log_base_rational_rational_base_prec_round_valid(x, y, *prec, *rm)
        }),
    )
}

pub fn rational_pow_rational_prec_round_valid(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    // For `Exact`, the power is representable exactly iff computing it toward negative infinity is
    // already exact.
    rm != Exact || Float::rational_pow_rational_prec_round_ref(x, y, prec, Floor).1 == Equal
}

pub fn exhaustive_rational_rational_unsigned_rounding_mode_quadruple_gen_var_3()
-> It<(Rational, Rational, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_xxy_custom_output(
                exhaustive_rationals(),
                exhaustive_positive_primitive_ints::<u64>(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| rational_pow_rational_prec_round_valid(x, y, *prec, *rm)),
    )
}

// -- (Rational, Float, PrimitiveUnsigned, RoundingMode) --

pub fn log_base_rational_float_base_prec_round_valid(
    x: &Rational,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm != Exact {
        return true;
    }
    // Special and degenerate inputs (x not positive, x = 1, or base not finite-positive, or base =
    // 1) yield exact results (0, +-infinity, or NaN) and never panic with Exact.
    if *x <= 0u32 || *x == 1u32 || !base.is_finite() || *base <= 0u32 || *base == 1u32 {
        return true;
    }
    log_base_rational_float_base_rational(x, base)
        .is_some_and(|q| Float::from_rational_prec(q, prec).1 == Equal)
}

// All `(Rational, Float, u64, RoundingMode)` that are valid inputs to
// `Float::log_base_rational_float_base_prec_round`.
pub fn exhaustive_rational_float_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(Rational, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                exhaustive_floats(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_rational_float_base_prec_round_valid(x, y, *prec, *rm)),
    )
}

pub fn exhaustive_rational_float_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Rational, Float, u64, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                exhaustive_extreme_floats(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, y, prec, rm)| log_base_rational_float_base_prec_round_valid(x, y, *prec, *rm)),
    )
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

// For each `(Float, base, m)` triple (base in 2..=62, m in 0..=20), all six rounding modes are
// generated consecutively.
pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_9()
-> It<(Float, i64, usize, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                primitive_int_increasing_inclusive_range::<i64>(-36, 62)
                    .filter(|&b| (2..=62).contains(&b) || (-36..=-2).contains(&b)),
                primitive_int_increasing_inclusive_range::<usize>(0, 20),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, b0, m, rnd)| valid_float_get_str_quadruple(x, *b0, *m, *rnd)),
    )
}

// The same as var 9, but over extreme `Float`s: `get_str` has to scale by a power of the base as
// large as the exponent, which is where that scaling works hardest.
pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_15()
-> It<(Float, i64, usize, RoundingMode)> {
    Box::new(
        reshape_3_1_to_4(Box::new(lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                primitive_int_increasing_inclusive_range::<i64>(-36, 62)
                    .filter(|&b| (2..=62).contains(&b) || (-36..=-2).contains(&b)),
                primitive_int_increasing_inclusive_range::<usize>(0, 20),
            ),
            exhaustive_rounding_modes(),
        )))
        .filter(|(x, b0, m, rnd)| valid_float_get_str_quadruple(x, *b0, *m, *rnd)),
    )
}

// All `(Float, base, m, RoundingMode)` inputs for `get_str` that rug's `to_sign_string_exp_round`
// also accepts: base restricted to 2..=36 (rug supports neither negative bases nor bases above 36)
// and rounding mode not `Exact` (rug has no exact rounding mode).
pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_10()
-> It<(Float, i64, usize, RoundingMode)> {
    reshape_3_1_to_4(Box::new(lex_pairs(
        exhaustive_triples(
            exhaustive_floats(),
            primitive_int_increasing_inclusive_range::<i64>(2, 36),
            primitive_int_increasing_inclusive_range::<usize>(0, 20),
        ),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    )))
}

// The same as var 10, but over extreme `Float`s, so that the rug cross-check applies to them too.
// rug's exponent range is `Float`'s, so it is a faithful oracle there.
pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_16()
-> It<(Float, i64, usize, RoundingMode)> {
    reshape_3_1_to_4(Box::new(lex_pairs(
        exhaustive_triples(
            exhaustive_extreme_floats(),
            primitive_int_increasing_inclusive_range::<i64>(2, 36),
            primitive_int_increasing_inclusive_range::<usize>(0, 20),
        ),
        exhaustive_rounding_modes().filter(|&rm| rm != Exact),
    )))
}

pub fn exp_prec_round_valid(x: &Float, _prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || !x.is_normal()
}

pub(crate) fn exp_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || !x.is_normal()
}

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.exp_prec_round`.
pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_36() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| exp_prec_round_valid(x, p, rm)),
    ))
}

// All `(Float, RoundingMode)` that are valid inputs to `Float.exp_round`.
pub fn exhaustive_float_rounding_mode_pair_gen_var_47() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| exp_round_valid(f, *rm)),
    )
}

// Whether `(x, prec, rm)` is a valid input to `Float::cbrt_prec_round`: `Exact` is only allowed
// when the cube root really is exact at the given precision.
pub fn cbrt_prec_round_valid(x: &Float, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.cbrt_prec_round_ref(prec, Floor).1 == Equal
}

// Whether `(x, rm)` is a valid input to `Float::cbrt_round`.
pub fn cbrt_round_valid(x: &Float, rm: RoundingMode) -> bool {
    rm != Exact || x.cbrt_round_ref(Floor).1 == Equal
}

// All `(Float, u64, RoundingMode)` that are valid inputs to `Float.cbrt_prec_round`.
pub fn exhaustive_float_unsigned_rounding_mode_triple_gen_var_37() -> It<(Float, u64, RoundingMode)>
{
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(exhaustive_floats(), exhaustive_positive_primitive_ints()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, p), rm)| cbrt_prec_round_valid(x, p, rm)),
    ))
}

// All `(Float, RoundingMode)` that are valid inputs to `Float.cbrt_round`.
pub fn exhaustive_float_rounding_mode_pair_gen_var_48() -> It<(Float, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| cbrt_round_valid(f, *rm)),
    )
}

// Whether `(x, k, prec, rm)` is a valid input to `Float::root_u_prec_round`: `Exact` is only
// allowed when the root really is exact at the given precision.
pub fn root_u_prec_round_valid(x: &Float, k: u64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.root_u_prec_round_ref(k, prec, Floor).1 == Equal
}

// Whether `(x, k, prec, rm)` is a valid input to `Float::root_s_prec_round`: `Exact` is only
// allowed when the root really is exact at the given precision.
pub fn root_s_prec_round_valid(x: &Float, k: i64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || x.root_s_prec_round_ref(k, prec, Floor).1 == Equal
}

// Whether `(x, k, prec, rm)` is a valid input to `Float::root_u_rational_prec_round`: `Exact` is
// only allowed when the root really is exact at the given precision.
pub fn root_u_rational_prec_round_valid(x: &Rational, k: u64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || Float::root_u_rational_prec_round_ref(x, k, prec, Floor).1 == Equal
}

// Whether `(x, k, prec, rm)` is a valid input to `Float::root_s_rational_prec_round`: `Exact` is
// only allowed when the root really is exact at the given precision.
pub fn root_s_rational_prec_round_valid(x: &Rational, k: i64, prec: u64, rm: RoundingMode) -> bool {
    rm != Exact || Float::root_s_rational_prec_round_ref(x, k, prec, Floor).1 == Equal
}

pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| root_u_prec_round_valid(x, n, prec, rm)),
    ))
}

// As `..._var_13`, but the `Float` may have an extreme exponent.
pub fn exhaustive_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_14()
-> It<(Float, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| root_u_prec_round_valid(x, n, prec, rm)),
    ))
}

pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_13()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| root_s_prec_round_valid(x, n, prec, rm)),
    ))
}

// As `..._var_13`, but the `Float` may have an extreme exponent.
pub fn exhaustive_float_signed_unsigned_rounding_mode_quadruple_gen_var_14()
-> It<(Float, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples(
                exhaustive_extreme_floats(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref x, n, prec), rm)| root_s_prec_round_valid(x, n, prec, rm)),
    ))
}

// All `(Rational, u64, u64, RoundingMode)` that are valid inputs to
// `Float::root_u_rational_prec_round`.
pub fn exhaustive_rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
-> It<(Rational, u64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                exhaustive_unsigneds(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, k, prec), rm)| root_u_rational_prec_round_valid(n, k, prec, rm)),
    ))
}

// All `(Rational, i64, u64, RoundingMode)` that are valid inputs to
// `Float::root_s_rational_prec_round`.
pub fn exhaustive_rational_signed_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(Rational, i64, u64, RoundingMode)> {
    reshape_3_1_to_4(Box::new(
        lex_pairs(
            exhaustive_triples_custom_output(
                exhaustive_rationals(),
                exhaustive_signeds(),
                exhaustive_positive_primitive_ints(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((ref n, k, prec), rm)| root_s_rational_prec_round_valid(n, k, prec, rm)),
    ))
}

// -- (String, PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

// All valid `(String, base, prec, RoundingMode)` inputs for `strtofr`: base 0 or in 2..=62, a
// string the whole of which parses, a positive precision, and (since `strtofr` panics on `Exact`
// for values it cannot represent exactly) `Exact` only paired with exactly-representable values.
//
// The string parts are a flat tuple rather than nested pairs so that each gets an equal share of
// the bit distributor's output; nesting them starves the combo, which is what chooses the syntactic
// shape.
fn exhaustive_strtofr_quadruples(rug_compatible: bool) -> It<(String, u8, u64, RoundingMode)> {
    // A property test runs 10000 values, far too few to cross every base with every syntactic
    // shape, every digit string, and every exponent. So the base and the shape are enumerated
    // exhaustively over the values that behave differently, and put in the lex cycle so that a
    // single run covers all of them; the digits, exponent, and precision then vary underneath. The
    // random modes supply the breadth these curated lists leave out.
    //
    // The bases are the ones on either side of every boundary the parser tests: prefix detection
    // (0), the `p` marker (2 and 16), the `e` marker (10 and 11), the bare special spellings (16
    // and 17), and the switch to case-sensitive digits (36 and 62).
    const BASES: [u8; 7] = [0, 2, 10, 11, 16, 36, 62];
    const RUG_BASES: [u8; 5] = [2, 10, 11, 16, 36];
    let bases: &[u8] = if rug_compatible { &RUG_BASES } else { &BASES };
    // Every combination of kind (NaN, infinity, a number), exponent marker, prefix or spelling
    // variant, and sign. The point position and the leading whitespace are left at their first
    // choice, being the two fields that cannot change which branch the parser takes.
    let combos = (0..3u32).flat_map(|kind| {
        (0..6).flat_map(move |marker| {
            (0..4).flat_map(move |variant| {
                (0..3).map(move |sign| kind + 8 * marker + 48 * variant + 960 * sign)
            })
        })
    });
    let base_combo_rms: Vec<(u8, u32, RoundingMode)> = bases
        .iter()
        .flat_map(|&base| {
            combos
                .clone()
                .flat_map(move |combo| exhaustive_rounding_modes().map(move |rm| (base, combo, rm)))
        })
        .collect();
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_pairs(
                    exhaustive_vecs_min_length(
                        1,
                        primitive_int_increasing_inclusive_range::<u8>(0, 61),
                    ),
                    exhaustive_signeds::<i64>(),
                ),
                exhaustive_positive_primitive_ints::<u64>(),
            ),
            base_combo_rms.into_iter(),
        )
        .map(move |(((digits, exp), prec), (base, combo, rm))| {
            (
                strtofr_string_from_parts(base, combo, &digits, exp, rug_compatible),
                base,
                prec,
                rm,
            )
        }),
    )
}

pub fn exhaustive_string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
-> It<(String, u8, u64, RoundingMode)> {
    Box::new(
        exhaustive_strtofr_quadruples(false)
            .filter(|(s, base, prec, rm)| valid_strtofr_quadruple(s, *base, *prec, *rm)),
    )
}

// All `(String, base, prec, RoundingMode)` inputs for `strtofr` that rug's `parse_radix` also
// accepts: base in 2..=36 (rug supports no others), no syntax rug rejects, and rounding mode not
// `Exact` (rug has no exact rounding mode).
pub fn exhaustive_string_unsigned_unsigned_rounding_mode_quadruple_gen_var_2()
-> It<(String, u8, u64, RoundingMode)> {
    Box::new(exhaustive_strtofr_quadruples(true).filter(|(_, _, _, rm)| *rm != Exact))
}

// All `(String, base, prec, RoundingMode)` where the string is an arbitrary sequence of the
// characters that appear in `strtofr` input, so that most of them are invalid. The base and
// rounding-mode restrictions are those of
// `exhaustive_string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1`.
pub fn exhaustive_string_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
-> It<(String, u8, u64, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_tiny(
                exhaustive_pairs(
                    exhaustive_strings_using_chars(STRTOFR_STRING_CHARS.chars()),
                    once(0).chain(primitive_int_increasing_inclusive_range(2, 62)),
                ),
                exhaustive_positive_primitive_ints::<u64>(),
            ),
            exhaustive_rounding_modes(),
        )
        .map(|(((s, base), prec), rm)| (s, base, prec, rm))
        .filter(|(s, base, prec, rm)| valid_strtofr_quadruple(s, *base, *prec, *rm)),
    )
}

// -- (String, FromSciStringOptions, PrimitiveUnsigned) --

// All valid `(String, FromSciStringOptions, prec)` inputs for
// `Float::from_sci_string_with_options_prec`: a string the whole of which parses, a positive
// precision, and (since `Exact` panics on a value it cannot represent) `Exact` only paired with
// exactly-representable values.
//
// As for `strtofr`, the base and the syntactic shape are enumerated over the values that behave
// differently and put in the lex cycle, so that a single property run covers all of them; the
// digits, exponent, and precision vary underneath.
pub fn exhaustive_string_from_sci_string_options_unsigned_triple_gen_var_1()
-> It<(String, FromSciStringOptions, u64)> {
    // A property test runs 10000 values, far too few to cross every base and rounding mode with
    // every syntactic shape, digit string, exponent, and precision. Under a bit distributor the
    // digits never leave their first value and every number comes out zero, so the dimensions are
    // curated and enumerated in lex order instead, which spends the budget predictably: the
    // innermost components are covered in full and the outermost get what is left. The random modes
    // supply the breadth these lists leave out.
    //
    // The bases sit on either side of the one boundary this grammar has: above base 14 the exponent
    // marker needs an explicit sign, to tell it from the digit `e`. That pair comes first because
    // the budget runs out partway through the list; the rest are here for their digit alphabets.
    const BASES: [u8; 5] = [14, 15, 2, 10, 36];
    // Every combination of kind (a NaN, an infinity, or a number), exponent marker, and sign, with
    // the point in the first two of its positions.
    let combos: Vec<u32> = (0..SCI_STRING_COMBO_COUNT)
        .filter(|combo| (combo / 27) % 5 < 2)
        .collect();
    // Digit strings that exercise the value: a single zero and a single one, a run, a string whose
    // leading and trailing digits are zero, and the largest digits of the base.
    let digit_sets: Vec<Vec<u8>> =
        vec![vec![0], vec![1], vec![1, 0], vec![0, 1, 0], vec![1, 2, 3, 4], vec![35, 35, 35]];
    let exps: [i64; 9] = [0, 1, -1, 5, -5, 1000, -1000, i64::MAX, i64::MIN];
    // Digits innermost, then the shape, then the base, then the exponent.
    let mut shapes: Vec<(i64, u8, u32, Vec<u8>)> = Vec::new();
    for exp in exps {
        for base in BASES {
            for &combo in &combos {
                for digits in &digit_sets {
                    shapes.push((exp, base, combo, digits.clone()));
                }
            }
        }
    }
    // The lex cycle repeats for every shape, so it holds only the two small dimensions.
    let rm_precs: Vec<(RoundingMode, u64)> = exhaustive_rounding_modes()
        .flat_map(|rm| [1u64, 10, 53].into_iter().map(move |prec| (rm, prec)))
        .collect();
    Box::new(
        lex_pairs(shapes.into_iter(), rm_precs.into_iter())
            .map(|((exp, base, combo, digits), (rm, prec))| {
                let mut options = FromSciStringOptions::default();
                options.set_base(base);
                options.set_rounding_mode(rm);
                (
                    sci_string_from_parts(base, combo, &digits, exp),
                    options,
                    prec,
                )
            })
            .filter(|(s, options, prec)| valid_float_from_sci_string_triple(s, *options, *prec)),
    )
}

// All `(String, FromSciStringOptions, prec)` where the string is an arbitrary sequence of the
// characters that appear in scientific notation, so that most of them are invalid; for exercising
// the parser's rejection paths.
pub fn exhaustive_string_from_sci_string_options_unsigned_triple_gen_var_2()
-> It<(String, FromSciStringOptions, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            base_gen::exhaustive_string_from_sci_string_options_pair_gen_var_1(),
            exhaustive_positive_primitive_ints::<u64>(),
        )
        .map(|((s, options), prec)| (s, options, prec))
        .filter(|(s, options, prec)| valid_float_from_sci_string_triple(s, *options, *prec)),
    )
}
