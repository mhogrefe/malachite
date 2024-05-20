// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::MulAssignRound;
use std::ops::MulAssign;

pub fn rug_mul_round(x: rug::Float, y: rug::Float, rm: Round) -> (rug::Float, Ordering) {
    let xsb = rug_float_significant_bits(&x);
    let ysb = rug_float_significant_bits(&y);
    let mut product = x;
    if product == 0u32 || xsb < ysb {
        product.set_prec(u32::exact_from(ysb));
    }
    let o = product.mul_assign_round(y, rm);
    (product, o)
}

pub fn rug_mul(x: rug::Float, y: rug::Float) -> rug::Float {
    let xsb = rug_float_significant_bits(&x);
    let ysb = rug_float_significant_bits(&y);
    let mut product = x;
    if product == 0u32 || xsb < ysb {
        product.set_prec(u32::exact_from(ysb));
    }
    product.mul_assign(y);
    product
}

pub fn rug_mul_rational_round(
    x: rug::Float,
    y: rug::Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut product = x;
    let o = product.mul_assign_round(y, rm);
    (product, o)
}

pub fn rug_mul_rational(x: rug::Float, y: rug::Rational) -> rug::Float {
    let mut product = x;
    product.mul_assign(y);
    product
}
