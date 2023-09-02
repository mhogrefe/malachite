use crate::test_util::common::rug_round_exact_from_rounding_mode;
use crate::Float;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
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

pub fn float_unsigned_rounding_mode_triple_rm<T: PrimitiveUnsigned>(
    xs: It<(Float, T, RoundingMode)>,
) -> It<((rug::Float, T, rug::float::Round), (Float, T, RoundingMode))> {
    Box::new(xs.map(|(x, p, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                p,
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, p, rm),
        )
    }))
}
