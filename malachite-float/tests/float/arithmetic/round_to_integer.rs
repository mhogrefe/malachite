// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use gmp_mpfr_sys::mpfr::{self, rnd_t};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{NaN, NegativeInfinity};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::float::arithmetic::round_to_integer::{
    primitive_float_round_to_integer, primitive_float_round_to_integer_ties_away,
};
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::natural::Natural;

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

fn expected_ternary(o: Ordering, is_int: bool) -> i32 {
    match (o, is_int) {
        (Equal, true) => 0,
        (Less, true) => -1,
        (Greater, true) => 1,
        (Less, false) => -2,
        (Greater, false) => 2,
        (Equal, false) => unreachable!(),
    }
}

// Every value pattern the sweep uses: spanning integers and non-integers, ties of both kept
// parities, all-ones significands (carry cases), values below 1, and both signs.
fn sweep_values() -> Vec<Float> {
    let mut xs = Vec::new();
    for prec_x in [1u64, 2, 5, 10, 64, 65, 100] {
        let mut sigs = vec![Natural::power_of_2(prec_x - 1), Natural::low_mask(prec_x)];
        for t in [1, 2, prec_x / 2, prec_x.saturating_sub(2)] {
            if t >= prec_x {
                continue;
            }
            sigs.push(Natural::power_of_2(prec_x - 1) + Natural::power_of_2(t));
            if t > 1 {
                sigs.push(
                    Natural::power_of_2(prec_x - 1)
                        + Natural::power_of_2(t)
                        + Natural::power_of_2(0u64),
                );
            }
        }
        sigs.sort_unstable();
        sigs.dedup();
        for sig in sigs {
            // exponents putting the value below 1, near 1, mid-significand, and integral
            for exp in [
                -2i64,
                0,
                1,
                2,
                i64::exact_from(prec_x / 2 + 1),
                i64::exact_from(prec_x),
                i64::exact_from(prec_x) + 10,
            ] {
                let x = Float::from_natural_prec(sig.clone(), prec_x).0
                    << (exp - i64::exact_from(prec_x));
                if x != 0u32 {
                    xs.push(x.clone());
                    xs.push(-x);
                }
            }
        }
    }
    xs
}

// The single-rounding family versus mpfr_rint (and mpfr_round for ties-away), on values and the
// full refined ternary.
#[test]
fn test_round_to_integer_vs_mpfr() {
    for x in sweep_values() {
        let b = rug::Float::exact_from(&x);
        for prec in [1u64, 2, 3, 10, 64, 100] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let (ours, o, is_int) = x.round_to_integer_prec_round_ref(prec, rm);
                let mut r = rug::Float::new(u32::exact_from(prec));
                let t = unsafe { mpfr::rint(r.as_raw_mut(), b.as_raw(), mpfr_rnd(rm)) };
                assert_eq!(
                    ComparableFloat(Float::from(&r)),
                    ComparableFloat(ours),
                    "{x} {prec} {rm}"
                );
                assert_eq!(t, expected_ternary(o, is_int), "ternary {x} {prec} {rm}");
            }
            // ties away: mpfr_round rounds into a precision-prec target
            let (ours, o, is_int) = x.round_to_integer_ties_away_prec_ref(prec);
            let mut r = rug::Float::new(u32::exact_from(prec));
            let t = unsafe { mpfr::round(r.as_raw_mut(), b.as_raw()) };
            assert_eq!(
                ComparableFloat(Float::from(&r)),
                ComparableFloat(ours),
                "ties away {x} {prec}"
            );
            assert_eq!(
                t,
                expected_ternary(o, is_int),
                "ties-away ternary {x} {prec}"
            );
        }
    }
}

// The double-rounding family versus the five mpfr_rint_* functions.
#[test]
fn test_round_to_integer_then_vs_mpfr() {
    type RawFn =
        unsafe extern "C" fn(*mut mpfr::mpfr_t, *const mpfr::mpfr_t, rnd_t) -> core::ffi::c_int;
    let cases: [(Option<RoundingMode>, RawFn); 5] = [
        (Some(Ceiling), mpfr::rint_ceil),
        (Some(Floor), mpfr::rint_floor),
        (Some(Down), mpfr::rint_trunc),
        (Some(Nearest), mpfr::rint_roundeven),
        (None, mpfr::rint_round),
    ];
    for x in sweep_values() {
        let b = rug::Float::exact_from(&x);
        for prec in [1u64, 2, 3, 10, 64, 100] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                for (irm, raw) in cases {
                    let (ours, o) = irm.map_or_else(
                        || x.round_to_integer_ties_away_then_prec_round_ref(prec, rm),
                        |irm| x.round_to_integer_then_prec_round_ref(irm, prec, rm),
                    );
                    let mut r = rug::Float::new(u32::exact_from(prec));
                    let t = unsafe { raw(r.as_raw_mut(), b.as_raw(), mpfr_rnd(rm)) };
                    assert_eq!(
                        ComparableFloat(Float::from(&r)),
                        ComparableFloat(ours),
                        "{x} {irm:?} {prec} {rm}"
                    );
                    let expected = match o {
                        Equal => 0,
                        Less => -1,
                        Greater => 1,
                    };
                    assert_eq!(t.signum(), expected, "ternary {x} {irm:?} {prec} {rm}");
                }
            }
        }
    }
}

// Overflow at the maximum exponent: an integer whose significand rounds up past the top of the
// exponent range becomes an infinity (or the maximum finite value for the appropriate modes in the
// double-rounding family).
#[test]
fn test_round_to_integer_max_exponent() {
    // all-ones significand at the maximum exponent: an integer not representable at prec 2
    let x = Float::from_natural_prec(Natural::low_mask(64), 64).0
        << (i64::from(Float::MAX_EXPONENT) - 64);
    for sign in [false, true] {
        let x = if sign { -x.clone() } else { x.clone() };
        let b = rug::Float::exact_from(&x);
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (ours, o, is_int) = x.round_to_integer_prec_round_ref(2, rm);
            let mut r = rug::Float::new(2);
            let t = unsafe { mpfr::rint(r.as_raw_mut(), b.as_raw(), mpfr_rnd(rm)) };
            assert_eq!(
                ComparableFloat(Float::from(&r)),
                ComparableFloat(ours),
                "max-exp {sign} {rm}"
            );
            assert_eq!(
                t,
                expected_ternary(o, is_int),
                "max-exp ternary {sign} {rm}"
            );
        }
    }
}

#[test]
fn round_to_integer_special() {
    let (r, o, is_int) = Float::NAN.round_to_integer_ref();
    assert!(r.is_nan());
    assert_eq!(o, Equal);
    assert!(!is_int);
    let (r, o, is_int) = Float::NEGATIVE_INFINITY.round_to_integer_ref();
    assert_eq!(r, Float::NEGATIVE_INFINITY);
    assert_eq!(o, Equal);
    assert!(!is_int);
    let (r, o, is_int) = Float::from(0u32).round_to_integer_ref();
    assert_eq!(ComparableFloat(r), ComparableFloat(Float::from(0u32)));
    assert_eq!(o, Equal);
    assert!(is_int);
    // variants agree
    let x = Float::from(2.5f64);
    let a = x.round_to_integer_prec_round_ref(3, Nearest);
    let b = x.clone().round_to_integer_prec_round(3, Nearest);
    assert_eq!(ComparableFloat(a.0.clone()), ComparableFloat(b.0));
    assert_eq!((a.1, a.2), (b.1, b.2));
    let c = x.round_to_integer_round_ref(Nearest);
    let d = x.clone().round_to_integer();
    assert_eq!(ComparableFloat(c.0.clone()), ComparableFloat(d.0));
}

#[test]
#[should_panic]
fn round_to_integer_fail_1() {
    Float::from(3u32).round_to_integer_prec_round_ref(0, Nearest);
}

#[test]
#[should_panic]
fn round_to_integer_fail_2() {
    Float::from(3u32).round_to_integer_prec_round_ref(5, Exact);
}

// Overflow in the double-rounding family requires a non-integer at the maximum exponent, whose
// integer part spans nearly 2^30 bits: an all-ones significand there rounds up to 2^(2^30), which
// overflows, and the final rounding mode then decides between infinity and the maximum finite
// value. Release-scale: the operand is a gigabit float.
#[test]
fn test_round_to_integer_then_overflow_extreme() {
    let prec_x = u64::from(u32::exact_from(Float::MAX_EXPONENT)) + 9;
    // Constructed through a Rational so that the exponent is in range from the start; building the
    // integer first would overflow before the scaling shift.
    let x = Float::from_rational_prec(
        malachite_q::Rational::from(Natural::low_mask(prec_x)) >> 9u32,
        prec_x,
    )
    .0;
    let b = rug::Float::exact_from(&x);
    for (rm, expect_infinite) in [(Up, true), (Nearest, true), (Floor, false), (Down, false)] {
        let (ours, o) = x.round_to_integer_then_prec_round_ref(Up, 2, rm);
        let mut r = rug::Float::new(2);
        let t = unsafe { mpfr::rint_ceil(r.as_raw_mut(), b.as_raw(), mpfr_rnd(rm)) };
        assert_eq!(
            ComparableFloat(Float::from(&r)),
            ComparableFloat(ours.clone()),
            "then-overflow {rm}"
        );
        assert_eq!(ours.is_infinite(), expect_infinite, "{rm}");
        let expected = match o {
            Equal => 0,
            Less => -1,
            Greater => 1,
        };
        assert_eq!(t.signum(), expected, "then-overflow ternary {rm}");
    }
}

#[test]
fn test_round_to_integer() {
    let test = |s, s_hex, out: &str, out_hex: &str, o_out: Ordering, is_int_out: bool| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (r, o, is_int) = x.clone().round_to_integer();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(is_int, is_int_out);

        let (r_alt, o_alt, is_int_alt) = x.round_to_integer_ref();
        assert!(r_alt.is_valid());
        assert_eq!(ComparableFloatRef(&r_alt), ComparableFloatRef(&r));
        assert_eq!(o_alt, o);
        assert_eq!(is_int_alt, is_int);
    };
    // - specials: zeros are integers, NaN and infinities are not
    test("NaN", "NaN", "NaN", "NaN", Equal, false);
    test("Infinity", "Infinity", "Infinity", "Infinity", Equal, false);
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Equal,
        false,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", Equal, true);
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", Equal, true);
    // - integers are unchanged
    test("2.0", "0x2.0#1", "2.0", "0x2.0#1", Equal, true);
    test("-2.0", "-0x2.0#1", "-2.0", "-0x2.0#1", Equal, true);
    test(
        "1.3e30",
        "0x1.0E+25#1",
        "1.3e30",
        "0x1.0E+25#1",
        Equal,
        true,
    );
    // - ties round to even, both parities and both signs
    test("2.50", "0x2.8#4", "2.00", "0x2.0#4", Less, false);
    test("-2.50", "-0x2.8#4", "-2.00", "-0x2.0#4", Greater, false);
    test("3.50", "0x3.8#4", "4.00", "0x4.0#4", Greater, false);
    test("-3.50", "-0x3.8#4", "-4.00", "-0x4.0#4", Less, false);
    test("10.5", "0xa.8#6", "10.0", "0xa.0#6", Less, false);
    test("-10.5", "-0xa.8#6", "-10.0", "-0xa.0#6", Greater, false);
    // - values below 1 round to 0 or +/-1
    test("0.75", "0x0.c#3", "1.0", "0x1.0#3", Greater, false);
    test("-0.75", "-0x0.c#3", "-1.0", "-0x1.0#3", Less, false);
    // - a non-tie fraction
    test("10.31", "0xa.50#9", "10.00", "0xa.00#9", Less, false);
}

#[test]
fn test_round_to_integer_ties_away() {
    let test = |s, s_hex, out: &str, out_hex: &str, o_out: Ordering, is_int_out: bool| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (r, o, is_int) = x.clone().round_to_integer_ties_away();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(is_int, is_int_out);

        let (r_alt, o_alt, is_int_alt) = x.round_to_integer_ties_away_ref();
        assert!(r_alt.is_valid());
        assert_eq!(ComparableFloatRef(&r_alt), ComparableFloatRef(&r));
        assert_eq!(o_alt, o);
        assert_eq!(is_int_alt, is_int);
    };
    // - ties round away from zero (2.5 -> 3, unlike roundeven's 2)
    test("2.50", "0x2.8#4", "3.00", "0x3.0#4", Greater, false);
    test("-2.50", "-0x2.8#4", "-3.00", "-0x3.0#4", Less, false);
    test("3.50", "0x3.8#4", "4.00", "0x4.0#4", Greater, false);
    test("10.5", "0xa.8#6", "11.0", "0xb.0#6", Greater, false);
}

#[test]
fn test_round_to_integer_prec_round() {
    let test = |s,
                s_hex,
                prec,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering,
                is_int_out: bool| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (r, o, is_int) = x.clone().round_to_integer_prec_round(prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(is_int, is_int_out);

        let (r_alt, o_alt, is_int_alt) = x.round_to_integer_prec_round_ref(prec, rm);
        assert!(r_alt.is_valid());
        assert_eq!(ComparableFloatRef(&r_alt), ComparableFloatRef(&r));
        assert_eq!(o_alt, o);
        assert_eq!(is_int_alt, is_int);
    };
    // - the single-rounding contract: 10.5 at precision 2 rounds to 12 (not first to 10, then to 8)
    test(
        "10.5", "0xa.8#6", 2, Nearest, "12.0", "0xc.0#2", Greater, false,
    );
    test("10.5", "0xa.8#6", 2, Floor, "8.0", "0x8.0#2", Less, false);
    test(
        "10.5", "0xa.8#6", 2, Ceiling, "12.0", "0xc.0#2", Greater, false,
    );
    test(
        "-10.5", "-0xa.8#6", 2, Nearest, "-12.0", "-0xc.0#2", Less, false,
    );
    // - values below 1 under each direction
    test(
        "0.75", "0x0.c#3", 5, Nearest, "1.00", "0x1.0#5", Greater, false,
    );
    test("0.75", "0x0.c#3", 5, Floor, "0.0", "0x0.0", Less, false);
    test(
        "-0.75", "-0x0.c#3", 5, Ceiling, "-0.0", "-0x0.0", Greater, false,
    );
    test(
        "-0.75", "-0x0.c#3", 5, Nearest, "-1.00", "-0x1.0#5", Less, false,
    );
    // - integers re-round to the requested precision, staying exact when representable
    test("2.0", "0x2.0#1", 5, Nearest, "2.00", "0x2.0#5", Equal, true);
    test(
        "1.3e30",
        "0x1.0E+25#1",
        5,
        Nearest,
        "1.27e30",
        "0x1.0E+25#5",
        Equal,
        true,
    );
    // - specials
    test("NaN", "NaN", 5, Nearest, "NaN", "NaN", Equal, false);
    test(
        "Infinity", "Infinity", 5, Nearest, "Infinity", "Infinity", Equal, false,
    );
    test("0.0", "0x0.0", 5, Nearest, "0.0", "0x0.0", Equal, true);
}

#[test]
fn test_round_to_integer_then_prec_round() {
    let test = |s,
                s_hex,
                irm: RoundingMode,
                prec,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (r, o) = x.clone().round_to_integer_then_prec_round(irm, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);

        let (r_alt, o_alt) = x.round_to_integer_then_prec_round_ref(irm, prec, rm);
        assert!(r_alt.is_valid());
        assert_eq!(ComparableFloatRef(&r_alt), ComparableFloatRef(&r));
        assert_eq!(o_alt, o);
    };
    // - the double rounding: 10.5 -> 10 (integer step) -> 8 at precision 2, in contrast to the
    //   single-rounding 12 of round_to_integer_prec_round
    test("10.5", "0xa.8#6", Down, 2, Nearest, "8.0", "0x8.0#2", Less);
    test(
        "10.5", "0xa.8#6", Nearest, 2, Nearest, "8.0", "0x8.0#2", Less,
    );
    // - 10.5 -> 11 (Up) -> 8 (Down at precision 2)
    test("10.5", "0xa.8#6", Up, 2, Down, "8.0", "0x8.0#2", Less);
    test(
        "-10.5", "-0xa.8#6", Floor, 2, Nearest, "-12.0", "-0xc.0#2", Less,
    );
}

// The emulated primitive-float integer rounding agrees bit-for-bit with the standard library
// wherever a standard function exists (integer rounding of a float is always exact).
#[test]
fn primitive_float_round_to_integer_properties() {
    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer(x, Floor)),
            NiceFloat(x.floor())
        );
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer(x, Ceiling)),
            NiceFloat(x.ceil())
        );
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer(x, Down)),
            NiceFloat(x.trunc())
        );
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer(x, Nearest)),
            NiceFloat(x.round_ties_even())
        );
        // Up (away from zero) has no standard equivalent; it is the sign-adjusted Ceiling
        let away = if x.is_sign_positive() {
            x.ceil()
        } else {
            x.floor()
        };
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer(x, Up)),
            NiceFloat(away)
        );
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer_ties_away(x)),
            NiceFloat(x.round())
        );
    });

    primitive_float_gen::<f32>().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer(x, Nearest)),
            NiceFloat(x.round_ties_even())
        );
        assert_eq!(
            NiceFloat(primitive_float_round_to_integer_ties_away(x)),
            NiceFloat(x.round())
        );
    });
}
