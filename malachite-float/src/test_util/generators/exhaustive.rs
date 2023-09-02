use crate::exhaustive::{
    exhaustive_finite_floats, exhaustive_floats, exhaustive_floats_with_sci_exponent,
    exhaustive_non_negative_finite_floats, exhaustive_nonzero_finite_floats,
    exhaustive_positive_finite_floats,
};
use crate::test_util::extra_variadic::{exhaustive_triples_from_single, exhaustive_triples_xxy};
use crate::{significand_bits, Float};
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::exhaustive::{
    exhaustive_positive_primitive_ints, exhaustive_primitive_floats, exhaustive_signeds,
    exhaustive_unsigneds,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::{reshape_2_1_to_3, It};
use malachite_base::test_util::generators::exhaustive_pairs_big_tiny;
use malachite_base::tuples::exhaustive::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_triples_xyy, lex_pairs,
};
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::exhaustive::exhaustive_naturals;
use malachite_nz::natural::Natural;
use malachite_q::exhaustive::exhaustive_rationals;
use malachite_q::Rational;

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

// -- (Float, Float) --

pub fn exhaustive_float_pair_gen() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_floats()))
}

pub fn exhaustive_float_pair_gen_var_1() -> It<(Float, Float)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_finite_floats()))
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

// -- (Float, Float, Rational) --

pub fn exhaustive_float_float_rational_triple_gen() -> It<(Float, Float, Rational)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_floats(),
        exhaustive_rationals(),
    ))
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

// TODO use ranges
pub fn exhaustive_float_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(Float, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_floats_with_sci_exponent(0),
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

// -- (Float, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_float_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(Float, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_unsigneds(),
    ))
}

// -- (Float, PrimitiveUnsigned, RoundingMode) --

pub fn set_prec_round_valid(x: &Float, p: u64, rm: RoundingMode) -> bool {
    rm != RoundingMode::Exact || !x.is_finite() || x.is_zero() || x.get_prec().unwrap() <= p || {
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

// -- (Float, Rational, Rational) --

pub fn exhaustive_float_rational_rational_triple_gen() -> It<(Float, Rational, Rational)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_floats(),
        exhaustive_rationals(),
    ))
}

// -- (Float, RoundingMode) --

pub(crate) fn natural_rounding_from_float_valid(f: &Float, rm: RoundingMode) -> bool {
    match rm {
        RoundingMode::Down | RoundingMode::Ceiling | RoundingMode::Nearest => {
            f.is_finite() || *f == Float::NEGATIVE_INFINITY
        }
        RoundingMode::Up | RoundingMode::Floor => {
            f.is_finite() && (f.is_sign_positive() || f.is_negative_zero())
        }
        RoundingMode::Exact => Natural::convertible_from(f),
    }
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_1() -> It<(Float, RoundingMode)> {
    Box::new(
        exhaustive_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| natural_rounding_from_float_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen() -> It<(Float, RoundingMode)> {
    Box::new(exhaustive_pairs(
        exhaustive_floats(),
        exhaustive_rounding_modes(),
    ))
}

pub(crate) fn integer_rounding_from_float_valid(f: &Float, rm: RoundingMode) -> bool {
    if rm == RoundingMode::Exact {
        Integer::convertible_from(f)
    } else {
        f.is_finite()
    }
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_2() -> It<(Float, RoundingMode)> {
    Box::new(
        exhaustive_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| integer_rounding_from_float_valid(f, *rm)),
    )
}

pub fn exhaustive_float_rounding_mode_pair_gen_var_3() -> It<(Float, RoundingMode)> {
    Box::new(exhaustive_pairs(
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
        RoundingMode::Floor => f.is_sign_positive() || f.is_negative_zero(),
        RoundingMode::Ceiling => *f <= T::MAX,
        RoundingMode::Down | RoundingMode::Nearest => !f.is_nan(),
        RoundingMode::Up => (f.is_sign_positive() || f.is_negative_zero()) && *f <= T::MAX,
        RoundingMode::Exact => T::convertible_from(f),
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
        exhaustive_pairs(exhaustive_floats(), exhaustive_rounding_modes())
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
        RoundingMode::Floor => *f >= T::MIN,
        RoundingMode::Ceiling => *f <= T::MAX,
        RoundingMode::Down | RoundingMode::Nearest => !f.is_nan(),
        RoundingMode::Up => *f >= T::MIN && *f <= T::MAX,
        RoundingMode::Exact => T::convertible_from(f),
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
        exhaustive_pairs(exhaustive_floats(), exhaustive_rounding_modes())
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
        exhaustive_pairs(exhaustive_floats(), exhaustive_rounding_modes())
            .filter(|(f, rm)| *rm != RoundingMode::Exact || T::convertible_from(f)),
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
            rm != RoundingMode::Exact
                || *n == 0u32
                || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
        }),
    ))
}

pub fn exhaustive_integer_unsigned_rounding_mode_triple_gen_var_4(
) -> It<(Integer, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || *n == 0u32
                || n.significant_bits() - n.trailing_zeros().unwrap() <= prec
        }),
    ))
}

pub fn exhaustive_natural_unsigned_rounding_mode_triple_gen_var_3(
) -> It<(Natural, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|&rm| rm != RoundingMode::Exact),
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
        exhaustive_rounding_modes().filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || *n == T::ZERO
                || n.significant_bits() - n.trailing_zeros() <= prec
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
        exhaustive_rounding_modes().filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || *n == T::ZERO
                || n.significant_bits() - n.trailing_zeros() <= prec
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
        exhaustive_rounding_modes().filter(|rm| *rm != RoundingMode::Exact),
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
            rm != RoundingMode::Exact
                || n.denominator_ref().is_power_of_2()
                    && n.numerator_ref().significant_bits() <= prec
        }),
    ))
}

pub fn exhaustive_rational_unsigned_rounding_mode_triple_gen_var_2(
) -> It<(Rational, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_positive_primitive_ints()),
        exhaustive_rounding_modes().filter(|rm| *rm != RoundingMode::Exact),
    )))
}
