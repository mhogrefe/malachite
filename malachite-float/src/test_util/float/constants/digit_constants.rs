// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{Digits, ExactFrom};
use malachite_base::num::factorization::traits::Primes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

// Computes a constant from its digits the slow, obvious way: take a prefix, bracket the constant
// between the two values that prefix allows, and round both ends as `Rational`s. This shares no
// machinery with `Float::non_dyadic_from_digits_prec_round`, which works in `Natural`s.
pub fn digit_constant_prec_round_naive<I: Iterator<Item = u64>>(
    mut digits: I,
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut ds: Vec<u64> = Vec::new();
    let mut count = prec + 64;
    loop {
        while u64::exact_from(ds.len()) < count {
            ds.push(digits.next().unwrap());
        }
        let mut num = Natural::ZERO;
        for &d in &ds {
            num = num * Natural::from(base) + Natural::from(d);
        }
        let den = Natural::from(base).pow(count);
        let lo = Rational::from_naturals(num.clone(), den.clone());
        let hi = Rational::from_naturals(num + Natural::from(1u32), den);
        let (f_lo, _) = Float::from_rational_prec_round(lo.clone(), prec, rm);
        let (f_hi, _) = Float::from_rational_prec_round(hi.clone(), prec, rm);
        if f_lo == f_hi {
            let q = Rational::exact_from(&f_lo);
            if q <= lo {
                return (f_lo, Less);
            }
            if q >= hi {
                return (f_lo, Greater);
            }
        }
        count *= 2;
    }
}

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
    digit_constant_prec_round_naive(liouvilles_digits_naive(), base, prec, rm)
}

pub fn champernowne_constant_base_prec_round_naive(
    base: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    digit_constant_prec_round_naive(
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
    digit_constant_prec_round_naive(
        u64::primes().flat_map(move |p| p.to_digits_desc(&base)),
        base,
        prec,
        rm,
    )
}
