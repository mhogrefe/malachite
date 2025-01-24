// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_round_exact_from_rounding_mode;
use crate::Float;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::It;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

pub fn float_rm(xs: It<Float>) -> It<(rug::Float, Float)> {
    Box::new(xs.map(|x| (rug::Float::exact_from(&x), x)))
}

pub fn float_pair_rm(xs: It<(Float, Float)>) -> It<((rug::Float, rug::Float), (Float, Float))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Float::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_natural_pair_rm(
    xs: It<(Float, Natural)>,
) -> It<((rug::Float, rug::Integer), (Float, Natural))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Integer::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_integer_pair_rm(
    xs: It<(Float, Integer)>,
) -> It<((rug::Float, rug::Integer), (Float, Integer))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Integer::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_rational_pair_rm(
    xs: It<(Float, Rational)>,
) -> It<((rug::Float, rug::Rational), (Float, Rational))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Rational::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_primitive_int_pair_rm<T: PrimitiveInt>(
    xs: It<(Float, T)>,
) -> It<((rug::Float, T), (Float, T))> {
    Box::new(xs.map(|(x, y)| ((rug::Float::exact_from(&x), y), (x, y))))
}

pub fn float_primitive_float_pair_rm<T: PrimitiveFloat>(
    xs: It<(Float, T)>,
) -> It<((rug::Float, T), (Float, T))> {
    Box::new(xs.map(|(x, y)| ((rug::Float::exact_from(&x), y), (x, y))))
}

pub fn float_t_rounding_mode_triple_rm<T: Clone + 'static>(
    xs: It<(Float, T, RoundingMode)>,
) -> It<((rug::Float, T, rug::float::Round), (Float, T, RoundingMode))> {
    Box::new(xs.map(|(x, p, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                p.clone(),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, p, rm),
        )
    }))
}

pub fn float_rounding_mode_pair_rm(
    xs: It<(Float, RoundingMode)>,
) -> It<((rug::Float, rug::float::Round), (Float, RoundingMode))> {
    Box::new(xs.map(|(x, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, rm),
        )
    }))
}

pub fn float_float_rounding_mode_triple_rm(
    xs: It<(Float, Float, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Float, rug::float::Round),
    (Float, Float, RoundingMode),
)> {
    Box::new(xs.map(|(x, y, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, y, rm),
        )
    }))
}

pub fn float_float_anything_triple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, T)>,
) -> It<((rug::Float, rug::Float, T), (Float, Float, T))> {
    Box::new(xs.map(|(x, y, z)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                z.clone(),
            ),
            (x, y, z),
        )
    }))
}

pub fn float_rational_anything_triple_rm<T: Clone + 'static>(
    xs: It<(Float, Rational, T)>,
) -> It<((rug::Float, rug::Rational, T), (Float, Rational, T))> {
    Box::new(xs.map(|(x, y, z)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                z.clone(),
            ),
            (x, y, z),
        )
    }))
}

pub fn float_rational_rounding_mode_triple_rm(
    xs: It<(Float, Rational, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Rational, rug::float::Round),
    (Float, Rational, RoundingMode),
)> {
    Box::new(xs.map(|(x, y, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, y, rm),
        )
    }))
}

pub fn float_float_anything_rounding_mode_quadruple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, T, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Float, T, rug::float::Round),
    (Float, Float, T, RoundingMode),
)> {
    Box::new(xs.map(|(x, y, z, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                z.clone(),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, y, z, rm),
        )
    }))
}

pub fn float_rational_anything_rounding_mode_quadruple_rm<T: Clone + 'static>(
    xs: It<(Float, Rational, T, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Rational, T, rug::float::Round),
    (Float, Rational, T, RoundingMode),
)> {
    Box::new(xs.map(|(x, y, z, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                z.clone(),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, y, z, rm),
        )
    }))
}
