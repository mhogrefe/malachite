// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Floor, Reciprocal};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::arithmetic::cfrac_helpers::ball_get_cfrac_for_testing;
use malachite_q::test_util::generators::rational_pair_gen;

// The common prefix of the two endpoints' continued fractions, expanded one term at a time. The
// ball engine must produce exactly this, however it batches the work.
fn naive_prefix(
    x: &Rational,
    y: &Rational,
) -> (Natural, Natural, Natural, Natural, bool, Rational, Rational) {
    let (mut m11, mut m12) = (Natural::ONE, Natural::ZERO);
    let (mut m21, mut m22) = (Natural::ZERO, Natural::ONE);
    let mut det_pos = true;
    let (mut l, mut r) = (x.clone(), y.clone());
    loop {
        let q = (&l).floor();
        if q != (&r).floor() {
            break;
        }
        let q = Natural::try_from(q).unwrap();
        let (nl, nr) = (&r - Rational::from(&q), &l - Rational::from(&q));
        if nl == 0u32 || nr == 0u32 {
            break;
        }
        let (nl, nr) = (nl.reciprocal(), nr.reciprocal());
        if nl <= 1u32 || nr <= 1u32 {
            break;
        }
        // m = m * [q 1; 1 0]
        let (a, b) = (&m11 * &q + &m12, m11);
        let (c, d) = (&m21 * &q + &m22, m21);
        m11 = a;
        m12 = b;
        m21 = c;
        m22 = d;
        det_pos = !det_pos;
        l = nl;
        r = nr;
    }
    (m11, m12, m21, m22, det_pos, l, r)
}

// Runs the engine at one cutoff and checks it against the naive expansion. Returns whether the ball
// reduced deeply enough to have exercised anything.
fn check(x: &Rational, y: &Rational, cutoff: u64) -> bool {
    let (m, ball) = ball_get_cfrac_for_testing(
        x.to_numerator(),
        x.to_denominator(),
        y.to_numerator(),
        y.to_denominator(),
        cutoff,
    );
    let (e11, e12, e21, e22, edet, el, er) = naive_prefix(x, y);
    assert_eq!(m.0, e11, "m11 differs for [{x}, {y}] at cutoff {cutoff}");
    assert_eq!(m.1, e12, "m12 differs for [{x}, {y}] at cutoff {cutoff}");
    assert_eq!(m.2, e21, "m21 differs for [{x}, {y}] at cutoff {cutoff}");
    assert_eq!(m.3, e22, "m22 differs for [{x}, {y}] at cutoff {cutoff}");
    assert_eq!(m.4, edet, "det differs for [{x}, {y}] at cutoff {cutoff}");
    assert_eq!(
        Rational::from_naturals(ball.0, ball.1),
        el,
        "reduced left differs for [{x}, {y}] at cutoff {cutoff}"
    );
    assert_eq!(
        Rational::from_naturals(ball.2, ball.3),
        er,
        "reduced right differs for [{x}, {y}] at cutoff {cutoff}"
    );
    m.0 > 1000u32
}

#[test]
fn ball_get_cfrac_properties() {
    // A ball whose endpoints diverge on the first term reduces to the identity and tests nothing,
    // so the depths actually reached are counted and checked at the end, once per tier.
    let deep_split = std::cell::Cell::new(0u32);
    let deep_lehmer = std::cell::Cell::new(0u32);
    let total = std::cell::Cell::new(0u32);
    rational_pair_gen().test_properties(|(a, b)| {
        // A narrow ball greater than one, with a numerator wide enough to reach the Lehmer floor:
        // the endpoints then share a long continued-fraction prefix, which is what the engine is
        // for. A wide ball would diverge on the first term and test nothing.
        let x = ((&a).abs() + Rational::from(2u32)) << 200u32;
        let width = ((&b).abs() + Rational::from(2u32)).reciprocal() >> 20u32;
        let y = &x + width;
        if x >= y {
            return;
        }
        // A tiny cutoff drives the subquadratic split path at these sizes; an unreachable one
        // leaves the Lehmer tier to do the work instead.
        if check(&x, &y, 64) {
            deep_split.set(deep_split.get() + 1);
        }
        if check(&x, &y, u64::MAX) {
            deep_lehmer.set(deep_lehmer.get() + 1);
        }
        total.set(total.get() + 1);
    });
    assert!(
        deep_split.get() * 4 > total.get(),
        "only {} of {} balls reduced deeply through split",
        deep_split.get(),
        total.get()
    );
    assert!(
        deep_lehmer.get() * 4 > total.get(),
        "only {} of {} balls reduced deeply through lehmer",
        deep_lehmer.get(),
        total.get()
    );
}
