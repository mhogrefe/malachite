// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::test_util::common::rug_float_significant_bits;
use malachite_base::max;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering::{self, *};

// The sign of a `Float` that is not NaN. `true` means positive.
fn float_sign(x: &Float) -> bool {
    match x {
        Float(Infinity { sign } | Zero { sign } | Finite { sign, .. }) => *sign,
        _ => panic!(),
    }
}

// A Rational-based reimplementation of x ± y * z with a single rounding, used as an oracle and as
// the naive side of the algorithms benchmarks. The singular cases mirror mpfr_fma_singular.
pub(crate) fn add_mul_prec_round_naive_helper(
    x: &Float,
    y: &Float,
    z: &Float,
    neg_p: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if x.is_nan() || y.is_nan() || z.is_nan() {
        return (float_nan!(), Equal);
    }
    if y.is_infinite() || z.is_infinite() {
        if *y == 0u32 || *z == 0u32 {
            return (float_nan!(), Equal);
        }
        let sp = (float_sign(y) == float_sign(z)) != neg_p;
        if x.is_infinite() && float_sign(x) != sp {
            return (float_nan!(), Equal);
        }
        return (
            if sp {
                float_infinity!()
            } else {
                float_negative_infinity!()
            },
            Equal,
        );
    }
    if x.is_infinite() {
        return (
            if float_sign(x) {
                float_infinity!()
            } else {
                float_negative_infinity!()
            },
            Equal,
        );
    }
    let mut p = Rational::exact_from(y) * Rational::exact_from(z);
    if neg_p {
        p = -p;
    }
    let r = Rational::exact_from(x) + p;
    if r == 0u32 {
        let sign = if *x == 0u32 {
            // the product is also zero: the sign follows the addition-of-zeros rule
            let sp = (float_sign(y) == float_sign(z)) != neg_p;
            if rm == Floor {
                sp && float_sign(x)
            } else {
                sp || float_sign(x)
            }
        } else {
            // exact cancellation of nonzero values
            rm != Floor
        };
        return (
            if sign {
                float_zero!()
            } else {
                float_negative_zero!()
            },
            Equal,
        );
    }
    Float::from_rational_prec_round(r, prec, rm)
}

// The mixed Float-Rational counterpart of `add_mul_prec_round_naive_helper`. A `Rational` zero is
// treated as positive in the product's sign rules.
pub(crate) fn add_mul_rational_prec_round_naive_helper(
    x: &Float,
    y: &Float,
    z: &Rational,
    neg_p: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if x.is_nan() || y.is_nan() {
        return (float_nan!(), Equal);
    }
    if y.is_infinite() {
        if *z == 0u32 {
            return (float_nan!(), Equal);
        }
        let sp = (float_sign(y) == (*z > 0u32)) != neg_p;
        if x.is_infinite() && float_sign(x) != sp {
            return (float_nan!(), Equal);
        }
        return (
            if sp {
                float_infinity!()
            } else {
                float_negative_infinity!()
            },
            Equal,
        );
    }
    if x.is_infinite() {
        return (
            if float_sign(x) {
                float_infinity!()
            } else {
                float_negative_infinity!()
            },
            Equal,
        );
    }
    let mut p = Rational::exact_from(y) * z;
    if neg_p {
        p = -p;
    }
    let r = Rational::exact_from(x) + p;
    if r == 0u32 {
        let sign = if *x == 0u32 {
            let sp = (float_sign(y) == (*z >= 0u32)) != neg_p;
            if rm == Floor {
                sp && float_sign(x)
            } else {
                sp || float_sign(x)
            }
        } else {
            rm != Floor
        };
        return (
            if sign {
                float_zero!()
            } else {
                float_negative_zero!()
            },
            Equal,
        );
    }
    Float::from_rational_prec_round(r, prec, rm)
}

#[inline]
pub fn add_mul_rational_prec_round_naive(
    x: &Float,
    y: &Float,
    z: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    add_mul_rational_prec_round_naive_helper(x, y, z, false, prec, rm)
}

#[inline]
pub fn add_mul_prec_round_naive(
    x: &Float,
    y: &Float,
    z: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    add_mul_prec_round_naive_helper(x, y, z, false, prec, rm)
}

pub fn rug_add_mul_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    z: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(y.mul_add_ref(z, x), rm);
    (sum, o)
}

#[inline]
pub fn rug_add_mul_prec(
    x: &rug::Float,
    y: &rug::Float,
    z: &rug::Float,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_add_mul_prec_round(x, y, z, prec, Round::Nearest)
}

#[inline]
pub fn rug_add_mul_round(
    x: &rug::Float,
    y: &rug::Float,
    z: &rug::Float,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_add_mul_prec_round(
        x,
        y,
        z,
        max!(
            rug_float_significant_bits(x),
            rug_float_significant_bits(y),
            rug_float_significant_bits(z)
        ),
        rm,
    )
}

pub fn rug_add_mul(x: &rug::Float, y: &rug::Float, z: &rug::Float) -> rug::Float {
    rug_add_mul_round(x, y, z, Round::Nearest).0
}
