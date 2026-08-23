// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::num::arithmetic::traits::ShlRoundAssign;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use std::cmp::Ordering::{self, *};

// A naive implementation of the `Float` product to test against. Each partial product is computed
// with enough precision to be exact, the intermediate values are kept normalized (with the true
// exponent accumulated separately in an `i128`) so that overflow and underflow are impossible
// mid-computation, and the result is rounded only once, at the end. The singular rules (NaN,
// infinities, and the signs of zero and infinite results) are the same as the real
// implementation's. Unlike `naive_sum_prec_round`, this oracle needs no exponent gate: the
// intermediate values never leave the representable range.
pub fn naive_product_prec_round(xs: &[Float], prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let mut sign = true;
    let mut any_zero = false;
    let mut any_inf = false;
    let mut regulars: Vec<&Float> = Vec::new();
    for x in xs {
        if x.is_nan() {
            return (Float::NAN, Equal);
        }
        if x.is_sign_negative() {
            sign = !sign;
        }
        if x.is_infinite() {
            any_inf = true;
        } else if *x == 0u32 {
            any_zero = true;
        } else {
            regulars.push(x);
        }
    }
    if any_inf {
        return if any_zero {
            (Float::NAN, Equal)
        } else if sign {
            (Float::INFINITY, Equal)
        } else {
            (Float::NEGATIVE_INFINITY, Equal)
        };
    }
    if any_zero {
        return if sign {
            (Float::ZERO, Equal)
        } else {
            (Float::NEGATIVE_ZERO, Equal)
        };
    }
    if regulars.is_empty() {
        return (Float::one_prec(prec), Equal);
    }
    // Exact accumulation of the absolute values, normalized to exponent 1, with the true exponent
    // tracked in drift.
    let mut drift = 0i128;
    let mut normalize = |mut t: Float| {
        let Float(Finite { sign, exponent, .. }) = &mut t else {
            unreachable!()
        };
        *sign = true;
        drift += i128::from(*exponent) - 1;
        *exponent = 1;
        t
    };
    let mut acc = normalize(regulars[0].clone());
    for x in &regulars[1..] {
        let t = normalize((*x).clone());
        let p = acc.significant_bits() + t.significant_bits();
        let o;
        (acc, o) = acc.mul_prec_round(t, p, Exact);
        assert_eq!(o, Equal);
        acc = normalize(acc);
    }
    if !sign {
        acc = -acc;
    }
    let (mut f, mut o) = Float::from_float_prec_round(acc, prec, rm);
    let o_shift = f.shl_round_assign(i64::exact_from(drift.clamp(-(1 << 40), 1 << 40)), rm);
    if o_shift != Equal {
        o = o_shift;
    }
    (f, o)
}

#[inline]
pub fn naive_product_prec(xs: &[Float], prec: u64) -> (Float, Ordering) {
    naive_product_prec_round(xs, prec, Nearest)
}

fn naive_max_prec(xs: &[Float]) -> u64 {
    xs.iter()
        .map(SignificantBits::significant_bits)
        .max()
        .unwrap_or(1)
}

#[inline]
pub fn naive_product_round(xs: &[Float], rm: RoundingMode) -> (Float, Ordering) {
    naive_product_prec_round(xs, naive_max_prec(xs), rm)
}

#[inline]
pub fn naive_product(xs: &[Float]) -> Float {
    naive_product_prec_round(xs, naive_max_prec(xs), Nearest).0
}
