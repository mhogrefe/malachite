// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering::{self, *};
use std::ops::Shr;

pub fn shr_prec_round_naive<T: PrimitiveInt>(
    x: Float,
    bits: T,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering)
where
    Rational: Shr<T, Output = Rational>,
{
    if x.is_normal() {
        Float::from_rational_prec_round(Rational::exact_from(x) >> bits, prec, rm)
    } else {
        (x, Equal)
    }
}

pub fn shr_prec_naive<T: PrimitiveInt>(x: Float, bits: T, prec: u64) -> (Float, Ordering)
where
    Rational: Shr<T, Output = Rational>,
{
    if x.is_normal() {
        Float::from_rational_prec_round(Rational::exact_from(x) >> bits, prec, Nearest)
    } else {
        (x, Equal)
    }
}

pub fn shr_round_naive<T: PrimitiveInt>(x: Float, bits: T, rm: RoundingMode) -> (Float, Ordering)
where
    Rational: Shr<T, Output = Rational>,
{
    if x.is_normal() {
        let prec = x.significant_bits();
        Float::from_rational_prec_round(Rational::exact_from(x) >> bits, prec, rm)
    } else {
        (x, Equal)
    }
}

pub fn rug_shr_prec_round_unsigned(
    x: &rug::Float,
    u: u32,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut shifted = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = shifted.assign_round(x >> u, rm);
    (shifted, o)
}

pub fn rug_shr_prec_round_signed(
    x: &rug::Float,
    i: i32,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut shifted = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = shifted.assign_round(x >> i, rm);
    (shifted, o)
}

pub fn rug_shr_prec_unsigned(x: &rug::Float, u: u32, prec: u64) -> (rug::Float, Ordering) {
    let mut shifted = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = shifted.assign_round(x >> u, Round::Nearest);
    (shifted, o)
}

pub fn rug_shr_prec_signed(x: &rug::Float, i: i32, prec: u64) -> (rug::Float, Ordering) {
    let mut shifted = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = shifted.assign_round(x >> i, Round::Nearest);
    (shifted, o)
}

pub fn rug_shr_round_unsigned(x: &rug::Float, u: u32, rm: Round) -> (rug::Float, Ordering) {
    let mut shifted = rug::Float::with_val(x.prec(), 0);
    let o = shifted.assign_round(x >> u, rm);
    (shifted, o)
}

pub fn rug_shr_round_signed(x: &rug::Float, i: i32, rm: Round) -> (rug::Float, Ordering) {
    let mut shifted = rug::Float::with_val(x.prec(), 0);
    let o = shifted.assign_round(x >> i, rm);
    (shifted, o)
}
