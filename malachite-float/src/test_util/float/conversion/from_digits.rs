// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::iter::repeat;

// Computes a non-dyadic real number from its digits the slow, obvious way: take a prefix, bracket
// the number between the two values that prefix allows, and round both ends as `Rational`s,
// doubling the prefix until they agree. This shares no machinery with
// `Float::non_dyadic_from_digits_prec_round`, which works in `Natural`s, or with
// `Float::non_dyadic_from_bits_prec_round`, which copies bits into limbs.
pub fn non_dyadic_from_digits_prec_round_naive<I: Iterator<Item = u64>>(
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
        let hi = Rational::from_naturals(num + Natural::ONE, den);
        let f_lo = Float::from_rational_prec_round(lo.clone(), prec, rm).0;
        let f_hi = Float::from_rational_prec_round(hi.clone(), prec, rm).0;
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

// The bits of a non-dyadic real number are its base-2 digits.
pub fn non_dyadic_from_bits_prec_round_naive<I: Iterator<Item = bool>>(
    bits: I,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    non_dyadic_from_digits_prec_round_naive(bits.map(u64::from), 2, prec, rm)
}

// The bits 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, ...: a single 1 followed by ever longer runs of 0s, so the
// sequence never becomes periodic and the number it denotes is irrational.
#[derive(Clone)]
pub struct SparseBits {
    b: bool,
    k: usize,
    j: usize,
}

impl Iterator for SparseBits {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        Some(if self.b {
            self.b = false;
            self.j = self.k;
            true
        } else {
            self.j -= 1;
            if self.j == 0 {
                self.k += 1;
                self.b = true;
            }
            false
        })
    }
}

pub const fn sparse_bits() -> SparseBits {
    SparseBits {
        b: true,
        k: 1,
        j: 1,
    }
}

// A non-dyadic `Rational` in (0, 1) built from any `Rational`: if |q| = n / d, then (3n + 1) / (3(n
// + d)) has a numerator not divisible by 3 and a denominator divisible by 3, so its reduced
// denominator keeps a factor of 3.
pub fn non_dyadic_fraction(q: &Rational) -> Rational {
    let (n, d) = q.numerator_and_denominator_ref();
    Rational::from_naturals(
        n * Natural::from(3u32) + Natural::ONE,
        (n + d) * Natural::from(3u32),
    )
}

// The bits after the point of a `Rational` in (0, 1).
pub fn fraction_bits(x: &Rational) -> impl Iterator<Item = bool> + Clone {
    fraction_power_of_2_digits(x, 1).map(|d| d == 1)
}

// The base-`base` digits after the point of a `Rational` in (0, 1), followed by 0s if the expansion
// terminates, so that the iterator is infinite.
pub fn fraction_digits(x: &Rational, base: u64) -> impl Iterator<Item = u64> + Clone {
    x.digits(&Natural::from(base))
        .1
        .map(|d| u64::exact_from(&d))
        .chain(repeat(0))
}

// The base-`2^log_base` digits after the point of a `Rational` in (0, 1), followed by 0s if the
// expansion terminates.
pub fn fraction_power_of_2_digits(
    x: &Rational,
    log_base: u64,
) -> impl Iterator<Item = u64> + Clone {
    x.power_of_2_digits(log_base)
        .1
        .map(|d| u64::exact_from(&d))
        .chain(repeat(0))
}
