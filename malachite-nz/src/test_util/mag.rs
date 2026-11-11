// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::mag::{MAG_BITS, Mag};
use crate::natural::Natural;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{FloorSqrt, SaturatingSub, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;

// The exact value of `x`, as `(m, e)` with `x` = m * 2^e.
pub fn mag_to_scaled_natural(x: Mag) -> (Natural, i64) {
    let (man, exp) = x.mantissa_and_exponent();
    (Natural::from(man), exp - i64::wrapping_from(MAG_BITS))
}

// Compares a * 2^i with b * 2^j.
pub fn cmp_scaled(a: &Natural, i: i64, b: &Natural, j: i64) -> Ordering {
    if i >= j {
        (a << u64::exact_from(i - j)).cmp(b)
    } else {
        a.cmp(&(b << u64::exact_from(j - i)))
    }
}

// Whether `x` is normalized: zero (with a zero exponent), or with a mantissa of exactly `MAG_BITS`
// significant bits.
pub fn mag_is_valid(x: Mag) -> bool {
    let (man, exp) = x.mantissa_and_exponent();
    if man == 0 {
        exp == 0
    } else {
        man.significant_bits() == MAG_BITS
    }
}

// The number of units in the last place of `x` by which `x` exceeds a * 2^i, rounded up, for an `x`
// that is at least a * 2^i: the least `k` such that `x` - k * ulp(`x`) <= a * 2^i, where ulp(`x`) =
// 2^(exp - `MAG_BITS`). It is 0 when `x` is exact.
pub fn mag_ulps_above(x: Mag, a: &Natural, i: i64) -> u64 {
    let (man, ulp_exp) = mag_to_scaled_natural(x);
    if man == 0u32 {
        assert_eq!(*a, 0u32);
        return 0;
    }
    // floor(a * 2^i / ulp(x))
    let floor = floor_scaled(a, i - ulp_exp);
    u64::exact_from(&(man.saturating_sub(floor)))
}

// The number of units in the last place of `x` by which `x` falls short of a * 2^i, rounded up, for
// an `x` that is at most a * 2^i: the least `k` such that `x` + k * ulp(`x`) >= a * 2^i. A zero `x`
// must be exact.
pub fn mag_ulps_below(x: Mag, a: &Natural, i: i64) -> u64 {
    let (man, ulp_exp) = mag_to_scaled_natural(x);
    if man == 0u32 {
        assert_eq!(*a, 0u32);
        return 0;
    }
    // ceiling(a * 2^i / ulp(x))
    let ceiling = if i >= ulp_exp {
        a << u64::exact_from(i - ulp_exp)
    } else {
        let shift = u64::exact_from(ulp_exp - i);
        let q = a >> shift;
        if &q << shift == *a {
            q
        } else {
            q + Natural::ONE
        }
    };
    u64::exact_from(&(ceiling.saturating_sub(man)))
}

// Asserts that `x` is a valid upper bound for a * 2^i, at most `ulps` units in the last place above
// it.
pub fn assert_mag_upper_bound(x: Mag, a: &Natural, i: i64, ulps: u64) {
    assert!(mag_is_valid(x));
    if x.is_zero() {
        assert_eq!(*a, Natural::ZERO);
        return;
    }
    assert_ne!(
        cmp_scaled(
            &mag_to_scaled_natural(x).0,
            mag_to_scaled_natural(x).1,
            a,
            i
        ),
        Ordering::Less
    );
    assert!(mag_ulps_above(x, a, i) <= ulps);
}

// Asserts that `x` is a valid lower bound for a * 2^i, at most `ulps` units in the last place below
// it.
pub fn assert_mag_lower_bound(x: Mag, a: &Natural, i: i64, ulps: u64) {
    assert!(mag_is_valid(x));
    let (man, e) = mag_to_scaled_natural(x);
    assert_ne!(cmp_scaled(&man, e, a, i), Ordering::Greater);
    if !x.is_zero() {
        assert!(mag_ulps_below(x, a, i) <= ulps);
    }
}

// floor(a * 2^i), for a possibly negative `i`.
fn floor_scaled(a: &Natural, i: i64) -> Natural {
    if i >= 0 {
        a << u64::exact_from(i)
    } else {
        a >> u64::exact_from(-i)
    }
}

// Asserts that `r` is a valid upper bound for `x` / `y`, where `y` is positive, at most `ulps`
// units in the last place above it.
pub fn assert_mag_div_u64_upper_bound(r: Mag, x: Mag, y: u64, ulps: u64) {
    assert!(mag_is_valid(r));
    let (mr, er) = mag_to_scaled_natural(r);
    let (mx, ex) = mag_to_scaled_natural(x);
    if x.is_zero() {
        assert!(r.is_zero());
        return;
    }
    let y = Natural::from(y);
    assert_ne!(cmp_scaled(&(&mr * &y), er, &mx, ex), Ordering::Less);
    // floor(x / y / 2^er)
    let q = if ex >= er {
        (mx << u64::exact_from(ex - er)) / y
    } else {
        mx / (y << u64::exact_from(er - ex))
    };
    assert!(mr.saturating_sub(q) <= ulps);
}

// Asserts that `r` is a valid upper bound for the square root of `x`, at most `ulps` units in the
// last place above it.
pub fn assert_mag_sqrt_upper_bound(r: Mag, x: Mag, ulps: u64) {
    assert!(mag_is_valid(r));
    let (mr, er) = mag_to_scaled_natural(r);
    let (mx, ex) = mag_to_scaled_natural(x);
    if x.is_zero() {
        assert!(r.is_zero());
        return;
    }
    assert_ne!(
        cmp_scaled(&(&mr).square(), er << 1, &mx, ex),
        Ordering::Less
    );
    // floor(sqrt(x) / 2^er)
    let q = floor_scaled(&mx, ex - (er << 1)).floor_sqrt();
    assert!(mr.saturating_sub(q) <= ulps);
}
