// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
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

// A Rational-based reimplementation of a * b ± c * d with a single rounding, used as an oracle and
// as the naive side of the algorithms benchmarks. The singular cases mirror the UBF product and
// addition rules that mpfr_fmma relies on.
pub(crate) fn mul_add_mul_prec_round_naive_helper(
    a: &Float,
    b: &Float,
    c: &Float,
    d: &Float,
    neg: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan() {
        return (float_nan!(), Equal);
    }
    let inf_zero = |x: &Float, y: &Float| {
        matches!(x, float_either_infinity!()) && matches!(y, float_either_zero!())
    };
    if inf_zero(a, b) || inf_zero(b, a) || inf_zero(c, d) || inf_zero(d, c) {
        return (float_nan!(), Equal);
    }
    let s1 = float_sign(a) == float_sign(b);
    let s2 = (float_sign(c) == float_sign(d)) != neg;
    let p1_inf = a.is_infinite() || b.is_infinite();
    let p2_inf = c.is_infinite() || d.is_infinite();
    if p1_inf || p2_inf {
        return if p1_inf && p2_inf && s1 != s2 {
            (float_nan!(), Equal)
        } else {
            let sp = if p1_inf { s1 } else { s2 };
            (
                if sp {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            )
        };
    }
    let mut p2 = Rational::exact_from(c) * Rational::exact_from(d);
    if neg {
        p2 = -p2;
    }
    let r = Rational::exact_from(a) * Rational::exact_from(b) + p2;
    if r == 0u32 {
        let p1_zero = *a == 0u32 || *b == 0u32;
        let p2_zero = *c == 0u32 || *d == 0u32;
        let sign = if p1_zero && p2_zero {
            if rm == Floor { s1 && s2 } else { s1 || s2 }
        } else if p1_zero {
            s2
        } else if p2_zero {
            s1
        } else {
            // exact cancellation of nonzero products
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

// The mixed Float-Rational counterpart of `mul_add_mul_prec_round_naive_helper`. A `Rational` zero
// is treated as positive in the product's sign rules.
pub(crate) fn mul_add_mul_rational_prec_round_naive_helper(
    x: &Float,
    y: &Float,
    z: &Float,
    w: &Rational,
    neg: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if x.is_nan() || y.is_nan() || z.is_nan() {
        return (float_nan!(), Equal);
    }
    let inf_zero = |u: &Float, v: &Float| {
        matches!(u, float_either_infinity!()) && matches!(v, float_either_zero!())
    };
    if inf_zero(x, y) || inf_zero(y, x) || matches!(z, float_either_infinity!()) && *w == 0u32 {
        return (float_nan!(), Equal);
    }
    let s1 = float_sign(x) == float_sign(y);
    // a zero Rational counts as positive, so >= rather than > (for a nonzero w the two comparisons
    // agree)
    let s2 = (float_sign(z) == (*w >= 0u32)) != neg;
    let p1_inf = x.is_infinite() || y.is_infinite();
    let p2_inf = z.is_infinite();
    if p1_inf || p2_inf {
        return if p1_inf && p2_inf && s1 != s2 {
            (float_nan!(), Equal)
        } else {
            let sp = if p1_inf { s1 } else { s2 };
            (
                if sp {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            )
        };
    }
    let mut p2 = Rational::exact_from(z) * w;
    if neg {
        p2 = -p2;
    }
    let r = Rational::exact_from(x) * Rational::exact_from(y) + p2;
    if r == 0u32 {
        let p1_zero = *x == 0u32 || *y == 0u32;
        let p2_zero = *z == 0u32 || *w == 0u32;
        let sign = if p1_zero && p2_zero {
            if rm == Floor { s1 && s2 } else { s1 || s2 }
        } else if p1_zero {
            s2
        } else if p2_zero {
            s1
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
pub fn mul_add_mul_rational_prec_round_naive(
    x: &Float,
    y: &Float,
    z: &Float,
    w: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    mul_add_mul_rational_prec_round_naive_helper(x, y, z, w, false, prec, rm)
}

#[inline]
pub fn mul_add_mul_prec_round_naive(
    a: &Float,
    b: &Float,
    c: &Float,
    d: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    mul_add_mul_prec_round_naive_helper(a, b, c, d, false, prec, rm)
}

pub fn rug_mul_add_mul_prec_round(
    a: &rug::Float,
    b: &rug::Float,
    c: &rug::Float,
    d: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(a.mul_add_mul_ref(b, c, d), rm);
    (sum, o)
}
