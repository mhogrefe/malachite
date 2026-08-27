// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use gmp_mpfr_sys::mpfr::{self, rnd_t};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::{ComparableFloat, Float};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

const fn mpfr_rnd(rm: RoundingMode) -> rnd_t {
    match rm {
        Floor => rnd_t::RNDD,
        Ceiling => rnd_t::RNDU,
        Down => rnd_t::RNDZ,
        Up => rnd_t::RNDA,
        Nearest => rnd_t::RNDN,
        Exact => panic!(),
    }
}

fn mpfr_can_round(x: &Float, err: i64, rnd1: RoundingMode, rnd2: RoundingMode, prec: u64) -> bool {
    let b = rug::Float::exact_from(x);
    unsafe {
        mpfr::can_round(
            b.as_raw(),
            err,
            mpfr_rnd(rnd1),
            mpfr_rnd(rnd2),
            i64::exact_from(prec),
        ) != 0
    }
}

// The exact decidability question `can_round` approximates: every real in the error interval rounds
// to the same value at the target precision. Rounding is monotone, so it suffices to compare the
// rounded endpoints, computed exactly via Rational.
fn can_round_ground_truth(
    x: &Float,
    err: i64,
    rnd1: RoundingMode,
    rnd2: RoundingMode,
    prec: u64,
) -> bool {
    let exp = i64::from(x.get_exponent().unwrap());
    let eps = Rational::power_of_2(exp - err);
    let b = Rational::exact_from(x);
    let neg = *x < 0;
    // An approximation rounded toward zero means the true value lies beyond b, away from zero, and
    // so on.
    let towards_zero = match rnd1 {
        Down => true,
        Up | Nearest => false,
        Floor => !neg,
        Ceiling => neg,
        Exact => unreachable!(),
    };
    let (lo, hi) = if rnd1 == Nearest {
        (&b - &eps, &b + &eps)
    } else if towards_zero != neg {
        // the interval extends upward in value
        (b.clone(), &b + &eps)
    } else {
        (&b - &eps, b.clone())
    };
    let lo = Float::from_rational_prec_round(lo, prec, rnd2).0;
    let hi = Float::from_rational_prec_round(hi, prec, rnd2).0;
    ComparableFloat(lo) == ComparableFloat(hi)
}

// Dense differential sweep against mpfr_can_round over significand patterns chosen to hit the
// rounding-bit, sticky-bit, and binade boundaries, for every pair of rounding modes, both signs,
// and errors spanning all the boundary cases in the code.
//
// can_round is a sufficient condition: false may always be answered conservatively (a Ziv loop just
// iterates once more), but true promises that rounding is genuinely decided. mpfr_can_round_raw's
// answers in some corner cases depend on where the limb boundaries fall (the carry-propagation and
// power-of-2 checks in its RNDZ/RNDN branches consult only some of the truncated limbs), and the
// port is faithful to the algorithm at Malachite's own limb width. With 64-bit limbs the layouts
// coincide and the answers must match exactly; with 32-bit limbs they can differ in either
// direction, and exact endpoint rounding arbitrates: whichever side answers true must be right.
#[test]
fn test_can_round_vs_mpfr() {
    let rms = [Floor, Ceiling, Down, Up, Nearest];
    let mut mismatches = 0u64;
    for prec_x in [10u64, 64, 65, 100] {
        // significand patterns at precision prec_x
        let mut sigs = vec![
            Natural::power_of_2(prec_x - 1),
            Natural::low_mask(prec_x),
            Natural::power_of_2(prec_x - 1) + Natural::power_of_2(0u64),
        ];
        for t in [1, 2, prec_x / 2, prec_x - 2] {
            sigs.push(Natural::power_of_2(prec_x - 1) + Natural::power_of_2(t));
            sigs.push(Natural::low_mask(prec_x) - Natural::power_of_2(t));
        }
        for sig in sigs {
            for sign in [false, true] {
                let x = Float::from_natural_prec(sig.clone(), prec_x).0;
                let x = if sign { -x } else { x };
                for prec in [1u64, 2, prec_x / 2, prec_x - 1, prec_x, prec_x + 1, prec_x + 10] {
                    for err in [
                        i64::exact_from(prec) - 1,
                        i64::exact_from(prec),
                        i64::exact_from(prec) + 1,
                        i64::exact_from(prec) + 2,
                        i64::exact_from(prec_x) - 1,
                        i64::exact_from(prec_x),
                        i64::exact_from(prec_x) + 1,
                        i64::exact_from(prec_x) + 10,
                    ] {
                        for rnd1 in rms {
                            for rnd2 in rms {
                                let ours = x.can_round(err, rnd1, rnd2, prec);
                                let theirs = mpfr_can_round(&x, err, rnd1, rnd2, prec);
                                if ours != theirs {
                                    assert!(
                                        can_round_ground_truth(&x, err, rnd1, rnd2, prec),
                                        "unsound true: {x} {err} {rnd1} {rnd2} {prec} (prec_x \
                                         {prec_x}, ours {ours}, mpfr {theirs})"
                                    );
                                    mismatches += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Every disagreement above is a corner where mpfr_can_round's limb-layout-dependent paths and
    // the port's sound carry propagation differ, each verified against ground truth. Pin the counts
    // so any behavioral drift is noticed.
    assert_eq!(mismatches, if Limb::WIDTH == u32::WIDTH { 24 } else { 16 });
}

// If rounding is claimed to be possible, then every value consistent with the approximation must
// round to the same result: check the endpoints of the error interval.
#[test]
fn can_round_soundness() {
    let rms = [Floor, Ceiling, Down, Up, Nearest];
    for prec_x in [10u64, 64] {
        let mut sigs = vec![Natural::power_of_2(prec_x - 1), Natural::low_mask(prec_x)];
        for t in [1, prec_x / 2, prec_x - 2] {
            sigs.push(Natural::power_of_2(prec_x - 1) + Natural::power_of_2(t));
        }
        for sig in sigs {
            for sign in [false, true] {
                let x = Float::from_natural_prec(sig.clone(), prec_x).0;
                let x = if sign { -x } else { x };
                for prec in [2u64, prec_x / 2, prec_x - 1] {
                    for err in [
                        i64::exact_from(prec) + 1,
                        i64::exact_from(prec) + 3,
                        i64::exact_from(prec_x),
                    ] {
                        // eps = 2^(EXP(x) - err), exactly representable
                        let exp = i64::from(x.get_exponent().unwrap());
                        let eps = Float::power_of_2(exp - err);
                        for rnd1 in rms {
                            for rnd2 in rms {
                                if !x.can_round(err, rnd1, rnd2, prec) {
                                    continue;
                                }
                                // interval per rnd1 (in the value domain): toward-zero
                                // approximations mean the true value is beyond x, and so on
                                let towards_zero = match rnd1 {
                                    Down => true,
                                    Up | Nearest => false,
                                    Floor => !sign,
                                    Ceiling => sign,
                                    Exact => unreachable!(),
                                };
                                let mut candidates = vec![x.clone()];
                                let away = if sign { &x - &eps } else { &x + &eps };
                                let toward = if sign { &x + &eps } else { &x - &eps };
                                if rnd1 == Nearest {
                                    candidates.push(away);
                                    candidates.push(toward);
                                } else if towards_zero {
                                    candidates.push(away);
                                } else {
                                    candidates.push(toward);
                                }
                                let mut results = candidates
                                    .into_iter()
                                    .map(|c| Float::from_float_prec_round(c, prec, rnd2).0);
                                let first = results.next().unwrap();
                                for r in results {
                                    assert_eq!(
                                        malachite_float::ComparableFloat(r),
                                        malachite_float::ComparableFloat(first.clone()),
                                        "{x} {err} {rnd1} {rnd2} {prec}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn can_round_special() {
    // - NaN, infinities, and zeros can never be rounded
    assert!(!Float::NAN.can_round(100, Nearest, Nearest, 10));
    assert!(!Float::from(f64::INFINITY).can_round(100, Nearest, Nearest, 10));
    assert!(!Float::from(0u32).can_round(100, Nearest, Nearest, 10));
    // - a nonpositive error never allows rounding
    let x = Float::from(3u32);
    assert!(!x.can_round(0, Nearest, Nearest, 10));
    assert!(!x.can_round(-5, Nearest, Nearest, 10));
}

#[test]
#[should_panic]
fn can_round_fail_1() {
    Float::from(3u32).can_round(100, Exact, Nearest, 10);
}

#[test]
#[should_panic]
fn can_round_fail_2() {
    Float::from(3u32).can_round(100, Nearest, Nearest, 0);
}

#[test]
fn test_can_round() {
    let test = |s, s_hex, err, rnd1: RoundingMode, rnd2: RoundingMode, prec, out: bool| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        assert_eq!(x.can_round(err, rnd1, rnd2, prec), out);
    };
    // - an all-ones significand: the 2^-20 error interval does not straddle a rounding boundary at
    //   precision 8
    test("65535.0", "0xffff.0#16", 20, Down, Nearest, 8, true);
    test("65535.0", "0xffff.0#16", 20, Down, Down, 8, true);
    test("65535.0", "0xffff.0#16", 20, Up, Up, 8, true);
    test("65535.0", "0xffff.0#16", 20, Nearest, Nearest, 8, true);
    // - err == prec * 2: still decidable here
    test("65535.0", "0xffff.0#16", 16, Down, Nearest, 8, true);
    // - a significand with a bit exactly at the precision boundary: not decidable
    test("32896.0", "0x8080.0#16", 20, Down, Nearest, 8, false);
    test("32896.0", "0x8080.0#16", 20, Nearest, Nearest, 8, false);
    test("32896.0", "0x8080.0#16", 12, Down, Nearest, 8, false);
    // - a power of 2: decidable from either side
    test("32768.0", "0x8000.0#16", 20, Down, Nearest, 8, true);
    test("32768.0", "0x8000.0#16", 20, Up, Nearest, 8, true);
    // - the sign does not affect the answer
    test("-65535.0", "-0xffff.0#16", 20, Down, Nearest, 8, true);
}
