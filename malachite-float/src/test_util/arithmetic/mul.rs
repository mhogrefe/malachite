// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::malachite_base::num::arithmetic::traits::NegAssign;
use crate::test_util::common::rug_float_significant_bits;
use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::max;

pub fn mul_prec_round_naive(x: Float, y: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (float_nan!(), _)
        | (_, float_nan!())
        | (float_either_infinity!(), float_either_zero!())
        | (float_either_zero!(), float_either_infinity!()) => (float_nan!(), Equal),
        (
            Float(Infinity { sign: x_sign }),
            Float(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
        )
        | (Float(Finite { sign: x_sign, .. }), Float(Infinity { sign: y_sign })) => (
            Float(Infinity {
                sign: x_sign == y_sign,
            }),
            Equal,
        ),
        (
            Float(Zero { sign: x_sign }),
            Float(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
        )
        | (Float(Finite { sign: x_sign, .. }), Float(Zero { sign: y_sign })) => (
            Float(Zero {
                sign: x_sign == y_sign,
            }),
            Equal,
        ),
        (x, y) => {
            let (mut product, o) = Float::from_rational_prec_round(
                Rational::exact_from(x) * Rational::exact_from(y),
                prec,
                rm,
            );
            if rm == Floor && o == Equal && product == 0u32 {
                product.neg_assign();
            }
            (product, o)
        }
    }
}

pub fn rug_mul_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(x * y, rm);
    (sum, o)
}

#[inline]
pub fn rug_mul_round(x: &rug::Float, y: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_mul_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        rm,
    )
}

#[inline]
pub fn rug_mul_prec(x: &rug::Float, y: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_mul_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_mul(x: &rug::Float, y: &rug::Float) -> rug::Float {
    rug_mul_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        Round::Nearest,
    )
    .0
}

pub fn rug_mul_rational_prec_round(
    x: &rug::Float,
    y: &rug::Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(x * y, rm);
    (sum, o)
}

pub fn rug_mul_rational_round(
    x: &rug::Float,
    y: &rug::Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_mul_rational_prec_round(x, y, rug_float_significant_bits(x), rm)
}

pub fn rug_mul_rational_prec(
    x: &rug::Float,
    y: &rug::Rational,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_mul_rational_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_mul_rational(x: &rug::Float, y: &rug::Rational) -> rug::Float {
    rug_mul_rational_prec_round(x, y, rug_float_significant_bits(x), Round::Nearest).0
}
