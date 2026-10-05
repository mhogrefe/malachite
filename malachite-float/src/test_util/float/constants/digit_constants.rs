// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::float::conversion::from_digits::non_dyadic_from_digits_prec_round_naive;
use malachite_base::num::conversion::traits::Digits;
use malachite_base::num::factorization::traits::Primes;
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;

pub fn liouvilles_digits_naive() -> impl Iterator<Item = u64> {
    let mut position = 0u64;
    let mut factorial = 1u64;
    let mut index = 1u64;
    std::iter::from_fn(move || {
        position += 1;
        Some(if position == factorial {
            index += 1;
            factorial = factorial.saturating_mul(index);
            1
        } else {
            0
        })
    })
}

pub fn liouvilles_constant_base_prec_round_naive(
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    non_dyadic_from_digits_prec_round_naive(liouvilles_digits_naive(), base, prec, rm)
}

pub fn champernowne_constant_base_prec_round_naive(
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    non_dyadic_from_digits_prec_round_naive(
        (1u64..).flat_map(move |n| n.to_digits_desc(&base)),
        base,
        prec,
        rm,
    )
}

pub fn copeland_erdos_constant_base_prec_round_naive(
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    non_dyadic_from_digits_prec_round_naive(
        u64::primes().flat_map(move |p| p.to_digits_desc(&base)),
        base,
        prec,
        rm,
    )
}
