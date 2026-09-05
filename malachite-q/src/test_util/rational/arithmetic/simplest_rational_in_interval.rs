// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::conversion::continued_fraction::to_continued_fraction::*;
use crate::rational::conversion::traits::ContinuedFraction;
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, Reciprocal, RoundToMultiple, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::exhaustive::exhaustive_positive_naturals;
use std::cmp::{Ordering::*, min};
use std::mem::swap;

const THREE: Natural = Natural::const_from(3);

// Slow! Only run for rationals with small denominators
pub fn simplest_rational_in_open_interval_naive(x: &Rational, y: &Rational) -> Rational {
    assert!(x < y);
    if *x < 0u32 && *y > 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    for d in exhaustive_positive_naturals() {
        let dr = Rational::from(d).reciprocal();
        let mut q = x.round_to_multiple(&dr, Ceiling).0;
        if q == *x {
            q += dr;
        }
        if q < *y {
            return if neg { -q } else { q };
        }
    }
    unreachable!()
}

fn simplest_rational_helper(
    floor_x: &Integer,
    floor_y: &Integer,
    cf_x: &[Natural],
    cf_y: &[Natural],
) -> Rational {
    if floor_x != floor_y {
        return Rational::from(min(floor_x, floor_y) + Integer::ONE);
    }
    let floor = floor_x;
    for (i, (x, y)) in cf_x.iter().zip(cf_y.iter()).enumerate() {
        if x != y {
            let mut cf = cf_x[..i].to_vec();
            cf.push(min(x, y) + Natural::ONE);
            return Rational::from_continued_fraction_ref(floor, cf.iter());
        }
    }
    let x_len = cf_x.len();
    let y_len = cf_y.len();
    Rational::from_continued_fraction(
        floor.clone(),
        match x_len.cmp(&y_len) {
            Equal => panic!(),
            Greater => {
                let mut cf = cf_y.to_vec();
                cf.push(cf_x[y_len].clone() + Natural::ONE);
                cf.into_iter()
            }
            Less => {
                let mut cf = cf_x.to_vec();
                cf.push(cf_y[x_len].clone() + Natural::ONE);
                cf.into_iter()
            }
        },
    )
}

fn cf_variants(x: &Rational) -> (Integer, Integer, Vec<Natural>, Vec<Natural>) {
    let (floor_1, cf_1) = x.continued_fraction();
    let cf_1 = cf_1.collect_vec();
    let mut cf_2 = cf_1.clone();
    let mut floor_2 = floor_1.clone();
    if let Some(last) = cf_2.last_mut() {
        *last -= Natural::ONE;
    } else {
        floor_2 -= Integer::ONE;
    }
    cf_2.push(Natural::ONE);
    (floor_1, floor_2, cf_1, cf_2)
}

pub fn simplest_rational_in_open_interval_explicit(x: &Rational, y: &Rational) -> Rational {
    assert!(x < y);
    if *x < 0u32 && *y > 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    let (floor_x_1, floor_x_2, cf_x_1, cf_x_2) = cf_variants(x);
    let (floor_y_1, floor_y_2, cf_y_1, cf_y_2) = cf_variants(y);
    let mut best: Option<Rational> = None;
    for (floor_x, cf_x) in [(&floor_x_1, &cf_x_1), (&floor_x_2, &cf_x_2)] {
        for (floor_y, cf_y) in [(&floor_y_1, &cf_y_1), (&floor_y_2, &cf_y_2)] {
            let candidate = simplest_rational_helper(floor_x, floor_y, cf_x, cf_y);
            if candidate > *x
                && candidate < *y
                && (best.is_none()
                    || candidate.denominator_ref() < best.as_ref().unwrap().denominator_ref())
            {
                best = Some(candidate);
            }
        }
    }
    let best = best.unwrap();
    if neg { -best } else { best }
}

// Slow! Only run for rationals with small denominators
pub fn simplest_rational_in_closed_interval_naive(x: &Rational, y: &Rational) -> Rational {
    assert!(x <= y);
    if *x <= 0u32 && *y >= 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    for d in exhaustive_positive_naturals() {
        let dr = Rational::from(d).reciprocal();
        let q = x.round_to_multiple(&dr, Ceiling).0;
        if q <= *y {
            return if neg { -q } else { q };
        }
    }
    unreachable!()
}

// The continued-fraction expansion of both endpoints, one term at a time. This is the
// implementation that shipped before the half-gcd ball engine took over the common-prefix walk. It
// is kept as a third reference for the property tests and as the arm the engine is benchmarked
// against.
fn min_helper_oo<'a>(ox: &'a Option<Natural>, oy: &'a Option<Natural>) -> &'a Natural {
    if let Some(x) = ox.as_ref() {
        if let Some(y) = oy.as_ref() {
            min(x, y)
        } else {
            x
        }
    } else {
        oy.as_ref().unwrap()
    }
}

fn min_helper_xo<'a>(x: &'a Natural, oy: &'a Option<Natural>) -> &'a Natural {
    if let Some(y) = oy.as_ref() {
        min(x, y)
    } else {
        x
    }
}

fn simplest_rational_one_alt_helper(
    x: &Natural,
    oy_n: &Option<Natural>,
    mut cf_y: RationalContinuedFraction,
    numerator: &Natural,
    denominator: &Natural,
    previous_numerator: &Natural,
    previous_denominator: &Natural,
) -> Rational {
    // use [a_0; a_1, ... a_k - 1, 1] and [b_0; b_1, ... b_k]
    let (n, d) = if oy_n.is_some() && x - Natural::ONE == *oy_n.as_ref().unwrap() {
        let next_numerator = previous_numerator.add_mul(numerator, oy_n.as_ref().unwrap());
        let next_denominator = previous_denominator.add_mul(denominator, oy_n.as_ref().unwrap());
        let next_oy_n = cf_y.next();
        if next_oy_n == Some(Natural::ONE) {
            let next_next_numerator = numerator + &next_numerator;
            let next_next_denominator = denominator + &next_denominator;
            // since y_n = 1, cf_y is not exhausted yet
            let y_n = cf_y.next().unwrap() + Natural::ONE;
            (
                next_numerator.add_mul(next_next_numerator, &y_n),
                next_denominator.add_mul(next_next_denominator, y_n),
            )
        } else {
            (
                numerator + (next_numerator << 1u32),
                denominator + (next_denominator << 1u32),
            )
        }
    } else {
        let ox_n_m_1 = x - Natural::ONE;
        let m = min_helper_xo(&ox_n_m_1, oy_n);
        let next_numerator = previous_numerator.add_mul(numerator, m);
        let next_denominator = previous_denominator.add_mul(denominator, m);
        (
            numerator + (next_numerator << 1u32),
            denominator + (next_denominator << 1u32),
        )
    };
    Rational {
        sign: true,
        numerator: n,
        denominator: d,
    }
}

fn update_best(best: &mut Option<Rational>, x: &Rational, y: &Rational, candidate: Rational) {
    if best.is_none() && candidate > *x && candidate < *y {
        *best = Some(candidate);
    }
}

pub fn simplest_rational_in_open_interval_term_by_term(x: &Rational, y: &Rational) -> Rational {
    assert!(x < y);
    if *x < 0u32 && *y > 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    let (floor_x, mut cf_x) = x.continued_fraction();
    let floor_x = floor_x.unsigned_abs();
    let (floor_y, mut cf_y) = y.continued_fraction();
    let floor_y = floor_y.unsigned_abs();
    let mut best = None;
    if floor_x == floor_y {
        let floor = floor_x;
        let mut previous_numerator = Natural::ONE;
        let mut previous_denominator = Natural::ZERO;
        let mut numerator = floor;
        let mut denominator = Natural::ONE;
        let mut ox_n = cf_x.next();
        let mut oy_n = cf_y.next();
        while ox_n == oy_n {
            // They are both Some
            swap(&mut numerator, &mut previous_numerator);
            swap(&mut denominator, &mut previous_denominator);
            numerator.add_mul_assign(&previous_numerator, &ox_n.unwrap());
            denominator.add_mul_assign(&previous_denominator, &oy_n.unwrap());
            ox_n = cf_x.next();
            oy_n = cf_y.next();
        }
        // use [x_0; x_1, ... x_k] and [y_0; y_1, ... y_k]
        let m = min_helper_oo(&ox_n, &oy_n) + Natural::ONE;
        let n = (&previous_numerator).add_mul(&numerator, &m);
        let d = (&previous_denominator).add_mul(&denominator, &m);
        let candidate = Rational {
            sign: true,
            numerator: n,
            denominator: d,
        };
        update_best(&mut best, x, y, candidate);
        if let Some(x_n) = ox_n.as_ref()
            && cf_x.is_done()
        {
            update_best(
                &mut best,
                x,
                y,
                simplest_rational_one_alt_helper(
                    x_n,
                    &oy_n,
                    cf_y.clone(),
                    &numerator,
                    &denominator,
                    &previous_numerator,
                    &previous_denominator,
                ),
            );
        }
        if let Some(y_n) = oy_n.as_ref()
            && cf_y.is_done()
        {
            update_best(
                &mut best,
                x,
                y,
                simplest_rational_one_alt_helper(
                    y_n,
                    &ox_n,
                    cf_x.clone(),
                    &numerator,
                    &denominator,
                    &previous_numerator,
                    &previous_denominator,
                ),
            );
        }
        if ox_n.is_some() && oy_n.is_some() && cf_x.is_done() != cf_y.is_done() {
            if cf_y.is_done() {
                swap(&mut ox_n, &mut oy_n);
                swap(&mut cf_y, &mut cf_x);
            }
            let x_n = ox_n.unwrap();
            let y_n = oy_n.unwrap();
            if y_n == x_n - Natural::ONE {
                let next_y_n = cf_y.next().unwrap();
                let next_numerator = (&previous_numerator).add_mul(&numerator, &y_n);
                let next_denominator = (&previous_denominator).add_mul(&denominator, &y_n);
                let (n, d) = if cf_y.is_done() && next_y_n == 2u32 {
                    (
                        (numerator << 1u32).add_mul(next_numerator, THREE),
                        (denominator << 1u32).add_mul(next_denominator, THREE),
                    )
                } else {
                    (
                        previous_numerator + (numerator << 1u32),
                        previous_denominator + (denominator << 1u32),
                    )
                };
                let candidate = Rational {
                    sign: true,
                    numerator: n,
                    denominator: d,
                };
                update_best(&mut best, x, y, candidate);
            }
        }
    } else {
        let candidate = if floor_y - Natural::ONE != floor_x || !cf_y.is_done() {
            Rational::from(floor_x + Natural::ONE)
        } else {
            let floor = floor_x;
            // [f; x_1, x_2, x_3...] and [f + 1]. But to get any good candidates, we need [f; x_1,
            // x_2, x_3...] and [f; 1]. If x_1 does not exist, the result is [f; 2].
            let (n, d) = if cf_x.is_done() {
                ((floor << 1u32) | Natural::ONE, Natural::TWO)
            } else {
                let x_1 = cf_x.next().unwrap();
                if x_1 > 1u32 {
                    if x_1 == 2u32 && cf_x.is_done() {
                        // [f; 1, 1] and [f; 1], so [f; 1, 2] is a candidate.
                        (Natural::TWO.add_mul(floor, THREE), THREE)
                    } else {
                        // If x_1 > 1, we have [f; 2] as a candidate.
                        ((floor << 1u32) | Natural::ONE, Natural::TWO)
                    }
                } else {
                    // x_2 exists since x_1 was 1
                    let x_2 = cf_x.next().unwrap();
                    // [f; 1, x_2] and [f; 1], so [f; 1, x_2 + 1] is a candidate. [f; 1, x_2 - 1, 1]
                    // and [f; 1], but [f; 1, x_2] is not in the interval
                    let k = &x_2 + Natural::ONE;
                    (&floor * (&k + Natural::ONE) + k, x_2 + Natural::TWO)
                }
            };
            Rational {
                sign: true,
                numerator: n,
                denominator: d,
            }
        };
        update_best(&mut best, x, y, candidate);
    }
    let best = best.unwrap();
    if neg { -best } else { best }
}
