// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::rug_float_significant_bits;
use crate::test_util::float::arithmetic::sum::naive_sum_prec_round;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering;

// A naive implementation of the dot product to test against: each term is computed exactly as a
// `Float` (the multiplication rules give exactly the term semantics, including NaN, zero times
// infinity, and the signs of zero terms), and the terms are then summed with the naive summation
// oracle. Since the exact products are materialized as `Float`s, this panics when a product
// overflows or underflows the exponent range — precisely the limitation the real implementation
// avoids — so callers must gate on the term exponents.
pub fn naive_dot_prec_round(
    xs: &[Float],
    ys: &[Float],
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_eq!(xs.len(), ys.len());
    let terms: Vec<Float> = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| {
            x.mul_prec_round_ref_ref(y, x.significant_bits() + y.significant_bits(), Exact)
                .0
        })
        .collect();
    naive_sum_prec_round(&terms, prec, rm)
}

#[inline]
pub fn naive_dot_prec(xs: &[Float], ys: &[Float], prec: u64) -> (Float, Ordering) {
    naive_dot_prec_round(xs, ys, prec, Nearest)
}

fn naive_max_prec(xs: &[Float], ys: &[Float]) -> u64 {
    xs.iter()
        .chain(ys.iter())
        .map(SignificantBits::significant_bits)
        .max()
        .unwrap_or(1)
}

#[inline]
pub fn naive_dot_round(xs: &[Float], ys: &[Float], rm: RoundingMode) -> (Float, Ordering) {
    naive_dot_prec_round(xs, ys, naive_max_prec(xs, ys), rm)
}

#[inline]
pub fn naive_dot(xs: &[Float], ys: &[Float]) -> Float {
    naive_dot_prec_round(xs, ys, naive_max_prec(xs, ys), Nearest).0
}

// Warning: mpfr_dot computes each product with mpfr_mul at full precision and asserts that the
// multiplication is exact, so calling this on inputs whose products overflow or underflow the
// exponent range ABORTS the process with a GNU MP assertion failure. Callers must gate on the term
// exponents.
pub fn rug_dot_prec_round(
    xs: &[rug::Float],
    ys: &[rug::Float],
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut dot = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = dot.assign_round(rug::Float::dot(xs.iter().zip(ys.iter())), rm);
    (dot, o)
}

#[inline]
pub fn rug_dot_prec(xs: &[rug::Float], ys: &[rug::Float], prec: u64) -> (rug::Float, Ordering) {
    rug_dot_prec_round(xs, ys, prec, Round::Nearest)
}

fn rug_max_prec(xs: &[rug::Float], ys: &[rug::Float]) -> u64 {
    xs.iter()
        .chain(ys.iter())
        .map(rug_float_significant_bits)
        .max()
        .unwrap_or(1)
}

#[inline]
pub fn rug_dot_round(xs: &[rug::Float], ys: &[rug::Float], rm: Round) -> (rug::Float, Ordering) {
    rug_dot_prec_round(xs, ys, rug_max_prec(xs, ys), rm)
}

pub fn rug_dot(xs: &[rug::Float], ys: &[rug::Float]) -> rug::Float {
    rug_dot_prec_round(xs, ys, rug_max_prec(xs, ys), Round::Nearest).0
}
