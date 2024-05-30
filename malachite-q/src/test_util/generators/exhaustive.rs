// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::exhaustive::{
    exhaustive_negative_rationals, exhaustive_non_negative_rationals, exhaustive_nonzero_rationals,
    exhaustive_positive_rationals, exhaustive_rationals,
};
use crate::test_util::extra_variadic::{
    exhaustive_ordered_unique_triples, exhaustive_quadruples_xxyz, exhaustive_triples_from_single,
    exhaustive_triples_xxy, exhaustive_triples_xxy_custom_output,
};
use crate::Rational;
use itertools::Itertools;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::string::options::exhaustive::exhaustive_to_sci_options;
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsInteger, ToSci};
use malachite_base::num::exhaustive::{
    exhaustive_finite_primitive_floats, exhaustive_nonzero_finite_primitive_floats,
    exhaustive_nonzero_signeds, exhaustive_positive_primitive_ints, exhaustive_primitive_floats,
    exhaustive_signeds, exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::{reshape_2_1_to_3, It};
use malachite_base::test_util::generators::{
    exhaustive_pairs_big_small, exhaustive_pairs_big_tiny,
};
use malachite_base::tuples::exhaustive::{
    exhaustive_ordered_unique_pairs, exhaustive_pairs, exhaustive_pairs_from_single,
    exhaustive_triples_custom_output, exhaustive_triples_xyy, exhaustive_triples_xyy_custom_output,
    lex_pairs,
};
use malachite_base::vecs::exhaustive::exhaustive_vecs;
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_range_to_infinity, exhaustive_naturals, exhaustive_positive_naturals,
};
use malachite_nz::natural::Natural;
use num::BigRational;
use std::ops::Shr;

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

pub fn exhaustive_rational_gen_var_4<T: PrimitiveFloat>() -> It<Rational>
where
    Rational: TryFrom<T>,
{
    Box::new(
        exhaustive_finite_primitive_floats()
            .skip(1)
            .map(Rational::exact_from),
    )
}

pub fn exhaustive_rational_gen_var_5<T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat>(
) -> It<Rational> {
    Box::new(exhaustive_rationals().filter(|q| !T::convertible_from(q)))
}

pub fn exhaustive_rational_gen_var_6<T: PrimitiveFloat>() -> It<Rational>
where
    Rational: TryFrom<T>,
{
    Box::new(exhaustive_nonzero_finite_primitive_floats().map(|f| {
        let x = Rational::exact_from(f);
        let y = Rational::exact_from(if f > T::ZERO {
            f.next_lower()
        } else {
            f.next_higher()
        });
        (x + y) >> 1
    }))
}

pub fn exhaustive_rational_gen_var_7() -> It<Rational> {
    Box::new(exhaustive_positive_rationals().filter(|q| *q != 1u32))
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

pub fn exhaustive_rational_natural_pair_gen_var_1() -> It<(Rational, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_rational_natural_pair_gen_var_2() -> It<(Rational, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_positive_naturals(),
    ))
}

pub fn exhaustive_rational_natural_pair_gen_var_3() -> It<(Rational, Natural)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_rationals(),
        exhaustive_positive_naturals(),
    ))
}

// -- (Rational, Natural, Natural) --

pub fn exhaustive_rational_natural_natural_triple_gen() -> It<(Rational, Natural, Natural)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_naturals(),
    ))
}

pub fn exhaustive_rational_natural_natural_triple_gen_var_1() -> It<(Rational, Natural, Natural)> {
    Box::new(
        exhaustive_triples_xyy(exhaustive_rationals(), exhaustive_positive_naturals())
            .filter(|(_, x, y)| x < y),
    )
}

// -- (Rational, PrimitiveFloat) --

pub fn exhaustive_rational_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Rational, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_rational_primitive_float_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Rational, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_primitive_floats(),
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

pub fn exhaustive_rational_signed_pair_gen_var_2<T: PrimitiveSigned>() -> It<(Rational, T)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_signeds())
            .filter(|(x, exp)| *exp >= T::ZERO || *x != 0u32),
    )
}

pub fn exhaustive_rational_signed_pair_gen_var_3<T: PrimitiveSigned>() -> It<(Rational, T)>
where
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_rationals(), exhaustive_signeds::<T>())
            .filter(|(x, pow)| !x.denominator_ref().is_power_of_2() || !(x >> *pow).is_integer()),
    )
}

pub fn exhaustive_rational_signed_pair_gen_var_4<T: PrimitiveSigned>() -> It<(Rational, T)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_non_negative_rationals(),
            exhaustive_nonzero_signeds(),
        )
        .filter(|(q, i)| *i > T::ZERO || *q != 0u32)
        .interleave(exhaustive_pairs_big_tiny(
            exhaustive_negative_rationals(),
            exhaustive_signeds::<T>()
                .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE)),
        )),
    )
}

pub fn exhaustive_rational_signed_pair_gen_var_5<T: PrimitiveSigned>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_rationals(),
        exhaustive_nonzero_signeds(),
    ))
}

// -- (Rational, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_rational_signed_signed_triple_gen<T: PrimitiveSigned>() -> It<(Rational, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_rationals(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_rational_signed_signed_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(Rational, T, T)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_rationals(),
            exhaustive_signeds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter(|(x, e, f)| *e >= T::ZERO && *f >= T::ZERO || *x != 0),
    )
}

// -- (Rational, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_rational_signed_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(Rational, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_rationals(),
        exhaustive_signeds::<T>(),
        exhaustive_positive_primitive_ints::<U>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Rational, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_rational_signed_rounding_mode_triple_gen_var_1(
) -> It<(Rational, i64, RoundingMode)> {
    Box::new(
        exhaustive_triples_custom_output(
            exhaustive_rationals(),
            exhaustive_signeds::<i64>(),
            exhaustive_rounding_modes(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter(|(x, i, rm)| {
            *rm != Exact || x.denominator_ref().is_power_of_2() && (x >> *i).is_integer()
        }),
    )
}

// var 2 is in malachite-float.

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

pub fn exhaustive_rational_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Rational, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_rationals(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_rational_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Rational, T)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_non_negative_rationals(),
            exhaustive_positive_primitive_ints(),
        )
        .interleave(exhaustive_pairs_big_tiny(
            exhaustive_negative_rationals(),
            exhaustive_unsigneds::<T>()
                .filter_map(|i| i.arithmetic_checked_shl(1).map(|j| j | T::ONE)),
        )),
    )
}

pub fn exhaustive_rational_unsigned_pair_gen_var_4() -> It<(Rational, u8)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_rationals(),
        primitive_int_increasing_inclusive_range(2, 36),
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

pub fn exhaustive_rational_unsigned_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Rational, T, T)> {
    Box::new(exhaustive_triples_xyy_custom_output(
        exhaustive_rationals(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
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

pub fn exhaustive_rational_pair_gen_var_2() -> It<(Rational, Rational)> {
    Box::new(
        exhaustive_pairs(exhaustive_rationals(), exhaustive_nonzero_rationals())
            .filter(|(x, y)| !(x / y).is_integer()),
    )
}

pub fn exhaustive_rational_pair_gen_var_3() -> It<(Rational, Rational)> {
    Box::new(
        exhaustive_ordered_unique_pairs(exhaustive_rationals()).map(|(x, y)| {
            if x < y {
                (x, y)
            } else {
                (y, x)
            }
        }),
    )
}

pub fn exhaustive_rational_pair_gen_var_4() -> It<(Rational, Rational)> {
    // TODO
    Box::new(exhaustive_pairs_from_single(exhaustive_rationals()).filter(|(x, y)| x <= y))
}

pub fn exhaustive_rational_pair_gen_var_5() -> It<(Rational, Rational)> {
    Box::new(exhaustive_pairs(
        exhaustive_positive_rationals(),
        exhaustive_positive_rationals()
            .filter(|q| (q - Rational::ONE).gt_abs(&Rational::from_signeds(1, 1000))),
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

// -- (Rational, Rational, PrimitiveFloat) --

pub fn exhaustive_rational_rational_primitive_float_triple_gen<T: PrimitiveFloat>(
) -> It<(Rational, Rational, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_primitive_floats(),
    ))
}

// -- (Rational, Rational, RoundingMode) --

pub(crate) fn round_to_multiple_rational_filter(t: &(Rational, Rational, RoundingMode)) -> bool {
    let &(ref x, ref y, rm) = t;
    if x == y {
        true
    } else if *y == 0u32 {
        rm == Down || rm == (if *x >= 0 { Floor } else { Ceiling }) || rm == Nearest
    } else {
        rm != Exact || (x / y).is_integer()
    }
}

pub fn exhaustive_rational_rational_rounding_mode_triple_gen_var_1(
) -> It<(Rational, Rational, RoundingMode)> {
    Box::new(
        reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_rationals()),
            exhaustive_rounding_modes(),
        )))
        .filter(round_to_multiple_rational_filter),
    )
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

pub fn exhaustive_rational_rational_signed_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(Rational, Rational, T)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_rationals(),
            exhaustive_signeds::<T>(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|(x, y, exp)| *exp >= T::ZERO || *x != 0 && *y != 0),
    )
}

// -- (Rational, Rational, PrimitiveUnsigned) --

pub fn exhaustive_rational_rational_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Rational, Rational, T)> {
    Box::new(exhaustive_triples_xxy(
        exhaustive_rationals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_rational_rational_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Rational, Rational, T)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_rationals(),
        exhaustive_unsigneds::<T>(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
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

pub fn exhaustive_rational_triple_gen_var_2() -> It<(Rational, Rational, Rational)> {
    Box::new(
        exhaustive_ordered_unique_triples(exhaustive_rationals()).map(|(x, y, z)| {
            let mut xs = vec![x, y, z];
            xs.sort_unstable();
            let mut xs = xs.into_iter();
            (xs.next().unwrap(), xs.next().unwrap(), xs.next().unwrap())
        }),
    )
}

pub fn exhaustive_rational_triple_gen_var_3() -> It<(Rational, Rational, Rational)> {
    // TODO
    Box::new(
        exhaustive_triples_from_single(exhaustive_rationals()).filter(|(x, y, z)| x <= y && y <= z),
    )
}

// -- (Rational, RoundingMode) --

pub fn exhaustive_rational_rounding_mode_pair_gen() -> It<(Rational, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_rationals(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_rational_rounding_mode_pair_gen_var_1() -> It<(Rational, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes()).filter(|(x, rm)| match rm {
            Floor | Up => *x >= 0u32,
            Exact => Natural::convertible_from(x),
            _ => true,
        }),
    )
}

pub fn exhaustive_rational_rounding_mode_pair_gen_var_2() -> It<(Rational, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes())
            .filter(|(x, rm)| *rm != Exact || x.is_integer()),
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
            Floor => *x >= T::MIN,
            Ceiling => *x <= T::MAX,
            Up => *x >= T::MIN && *x <= T::MAX,
            Exact => T::convertible_from(x),
            _ => true,
        }),
    )
}

pub fn exhaustive_rational_rounding_mode_pair_gen_var_4() -> It<(Rational, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_nonzero_rationals(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_rational_rounding_mode_pair_gen_var_5<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat,
>() -> It<(Rational, RoundingMode)>
where
    Rational: TryFrom<T>,
{
    let max = Rational::exact_from(T::MAX_FINITE);
    let min = -&max;
    Box::new(
        lex_pairs(exhaustive_rationals(), exhaustive_rounding_modes()).filter(move |(x, rm)| {
            match rm {
                Floor => *x >= min,
                Ceiling => *x <= max,
                Up => *x >= min && *x <= max,
                Exact => T::convertible_from(x),
                _ => true,
            }
        }),
    )
}

// -- (Rational, ToSciOptions) --

pub fn exhaustive_rational_to_sci_options_pair_gen() -> It<(Rational, ToSciOptions)> {
    Box::new(exhaustive_pairs(
        exhaustive_rationals(),
        exhaustive_to_sci_options(),
    ))
}

pub fn exhaustive_rational_to_sci_options_pair_gen_var_1() -> It<(Rational, ToSciOptions)> {
    Box::new(
        exhaustive_pairs(exhaustive_rationals(), exhaustive_to_sci_options())
            .filter(|(x, options)| x.fmt_sci_valid(*options)),
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
            serde_json::to_string(&BigRational::from(&x)).unwrap(),
            serde_json::to_string(&rug::Rational::from(&x)).unwrap(),
            serde_json::to_string(&x).unwrap(),
        )
    }))
}

// -- Vec<Rational> --

pub fn exhaustive_rational_vec_gen() -> It<Vec<Rational>> {
    Box::new(exhaustive_vecs(exhaustive_rationals()))
}
