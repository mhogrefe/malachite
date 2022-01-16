use crate::common::{rational_to_bigrational, rational_to_rug_rational};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, IsInteger};
use malachite_base::num::exhaustive::{exhaustive_signeds, exhaustive_unsigneds};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::exhaustive::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples_xxyz,
    exhaustive_triples_from_single, exhaustive_triples_xxy, exhaustive_triples_xyy, lex_pairs,
};
use malachite_base_test_util::generators::common::It;
use malachite_base_test_util::generators::exhaustive_pairs_big_tiny;
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::exhaustive::{exhaustive_naturals, exhaustive_positive_naturals};
use malachite_nz::natural::Natural;
use malachite_q::exhaustive::{
    exhaustive_non_negative_rationals, exhaustive_nonzero_rationals, exhaustive_positive_rationals,
    exhaustive_rationals,
};
use malachite_q::Rational;

// -- Rational --

pub fn exhaustive_rational_gen() -> It<Rational> {
    Box::new(exhaustive_rationals())
}

pub fn exhaustive_rational_gen_var_1() -> It<Rational> {
    Box::new(exhaustive_nonzero_rationals())
}

pub fn exhaustive_rational_gen_var_2() -> It<Rational> {
    Box::new(exhaustive_positive_rationals())
}

pub fn exhaustive_rational_gen_var_3() -> It<Rational> {
    Box::new(exhaustive_non_negative_rationals())
}

// -- (Rational, Integer) --

pub fn exhaustive_rational_integer_pair_gen() -> It<(Rational, Integer)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_integers(),
    ))
}

// -- (Rational, Integer, Integer) --

pub fn exhaustive_rational_integer_integer_triple_gen() -> It<(Rational, Integer, Integer)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_integers(),
    ))
}

// -- (Rational, Natural) --

pub fn exhaustive_rational_natural_pair_gen() -> It<(Rational, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_naturals(),
    ))
}

// -- (Rational, Natural, Natural) --

pub fn exhaustive_rational_natural_natural_triple_gen() -> It<(Rational, Natural, Natural)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_naturals(),
    ))
}

// -- (Rational, PrimitiveSigned) --

pub fn exhaustive_rational_signed_pair_gen<T: PrimitiveSigned>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_rational_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_rationals(),
        exhaustive_signeds(),
    ))
}

// -- (Rational, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_rational_signed_signed_triple_gen<T: PrimitiveSigned>() -> It<(Rational, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_signeds(),
    ))
}

// -- (Rational, PrimitiveUnsigned) --

pub fn exhaustive_rational_unsigned_pair_gen<T: PrimitiveUnsigned>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_rational_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_rationals(),
        exhaustive_unsigneds(),
    ))
}

// -- (Rational, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_rational_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Rational, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_unsigneds(),
    ))
}

// -- (Rational, Rational) --

pub fn exhaustive_rational_pair_gen() -> It<(Rational, Rational)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_rationals()))
}

pub fn exhaustive_rational_pair_gen_var_1() -> It<(Rational, Rational)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_nonzero_rationals(),
    ))
}

// -- (Rational, Rational, Integer) --

pub fn exhaustive_rational_rational_integer_triple_gen() -> It<(Rational, Rational, Integer)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_integers(),
    ))
}

// -- (Rational, Rational, Natural) --

pub fn exhaustive_rational_rational_natural_triple_gen() -> It<(Rational, Rational, Natural)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_naturals(),
    ))
}

pub fn exhaustive_rational_rational_natural_triple_gen_var_1() -> It<(Rational, Rational, Natural)>
{
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_positive_naturals(),
    ))
}

// -- (Rational, Rational, Natural, Natural) --

pub fn exhaustive_rational_rational_natural_natural_quadruple_gen_var_1(
) -> It<(Rational, Rational, Natural, Natural)> {
    Box::new(exhaustive_quadruples_xxyz(
        exhaustive_rationals(),
        exhaustive_naturals(),
        exhaustive_positive_naturals(),
    ))
}

// -- (Rational, Rational, PrimitiveSigned) --

pub fn exhaustive_rational_rational_signed_triple_gen<T: PrimitiveSigned>(
) -> It<(Rational, Rational, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_signeds(),
    ))
}

// -- (Rational, Rational, PrimitiveUnsigned) --

pub fn exhaustive_rational_rational_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Rational, Rational, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_unsigneds(),
    ))
}

// -- (Rational, Rational, Rational) --

pub fn exhaustive_rational_triple_gen() -> It<(Rational, Rational, Rational)> {
    Box::new(exhaustive_triples_from_single(exhaustive_rationals()))
}

pub fn exhaustive_rational_triple_gen_var_1() -> It<(Rational, Rational, Rational)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_nonzero_rationals(),
    ))
}

// -- (Rational, RoundingMode) --

pub fn exhaustive_rational_rounding_mode_pair_gen_var_1() -> It<(Rational, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes()).filter(|(x, rm)| match rm {
            RoundingMode::Floor | RoundingMode::Up => *x >= 0u32,
            RoundingMode::Exact => Natural::convertible_from(x),
            _ => true,
        }),
    )
}

pub fn exhaustive_rational_rounding_mode_pair_gen_var_2() -> It<(Rational, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes())
            .filter(|(x, rm)| *rm != RoundingMode::Exact || x.is_integer()),
    )
}

pub fn exhaustive_rational_rounding_mode_pair_gen_var_3<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>() -> It<(Rational, RoundingMode)>
where
    Rational: PartialOrd<T>,
{
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes()).filter(|(x, rm)| match rm {
            RoundingMode::Floor => *x >= T::MIN,
            RoundingMode::Ceiling => *x <= T::MAX,
            RoundingMode::Up => *x >= T::MIN && *x <= T::MAX,
            RoundingMode::Exact => T::convertible_from(x),
            _ => true,
        }),
    )
}

// -- String --

// vars 1 through 10 are in malachite-base.

pub fn exhaustive_string_gen_var_11() -> It<String> {
    Box::new(exhaustive_rationals().map(|r| serde_json::to_string(&r).unwrap()))
}

pub fn exhaustive_string_gen_var_12() -> It<String> {
    Box::new(exhaustive_rationals().map(|x| x.to_string()))
}

// -- (String, String, String) --

// vars 1 through 2 are in malachite-nz.

pub fn exhaustive_string_triple_gen_var_3() -> It<(String, String, String)> {
    Box::new(exhaustive_rationals().map(|x| {
        (
            serde_json::to_string(&rational_to_bigrational(&x)).unwrap(),
            serde_json::to_string(&rational_to_rug_rational(&x)).unwrap(),
            serde_json::to_string(&x).unwrap(),
        )
    }))
}
