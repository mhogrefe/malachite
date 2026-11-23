// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{NaN, One, OneHalf, Zero};
use malachite_base::oracle_row;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_pair_gen, big_exponent_float_rational_pair_gen, float_pair_gen,
    float_rational_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

const PRECS: [u64; 5] = [1, 2, 10, 64, 100];
const RMS: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_min_max() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let test =
        |s: &str, s_hex: &str, t: &str, t_hex: &str, u: &str, u_hex: &str, v: &str, v_hex: &str| {
            oracle_row!("flint", "arf_min_max", "{s_hex}.min({t_hex}) = {u_hex}");
            oracle_row!("flint", "arf_min_max", "{s_hex}.max({t_hex}) = {v_hex}");

            let x = &parse_big_exponent_float_hex(s_hex);
            assert_eq!(x.to_string(), s);
            let y = &parse_big_exponent_float_hex(t_hex);
            assert_eq!(y.to_string(), t);
            let min = &parse_big_exponent_float_hex(u_hex);
            assert_eq!(min.to_string(), u);
            let max = &parse_big_exponent_float_hex(v_hex);
            assert_eq!(max.to_string(), v);
            assert!(same(&x.clone().min(y.clone()), min));
            assert!(same(&x.clone().min_val_ref(y), min));
            assert!(same(&x.min_ref_val(y.clone()), min));
            assert!(same(&x.min_ref_ref(y), min));
            assert!(same(&x.clone().max(y.clone()), max));
            assert!(same(&x.clone().max_val_ref(y), max));
            assert!(same(&x.max_ref_val(y.clone()), max));
            assert!(same(&x.max_ref_ref(y), max));
        };
    let nan = BigExponentFloat::NAN;
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test(
        "NaN",
        "NaN",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "NaN",
        "NaN",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
    );
    test(
        "NaN",
        "NaN",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
    );
    test(
        "0.0",
        "0x0.0",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "0.0",
        "0x0.0",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "1.0",
        "0x1.0",
        "1.0",
        "0x1.0",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", "0.50", "0x0.8", "0.50", "0x0.8",
    );

    // - rounding the selected operand
    let (x, o) = pi.min_prec_round_ref_ref(&huge, 5, Floor);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{pi:?}.min_prec_round({huge:?}, 5, Floor) = ({x:?}, {o:?})"
    );
    assert_eq!(format!("{x:?}"), "0x3.2");
    assert_eq!(o, Less);
    let (x, o) = pi.max_prec_round_ref_ref(&huge, 5, Floor);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{pi:?}.max_prec_round({huge:?}, 5, Floor) = ({x:?}, {o:?})"
    );
    assert!(same(&x, &huge));
    assert_eq!(o, Equal);
    let (x, o) = nan.min_prec_ref_ref(&nan, 5);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{nan:?}.min_prec({nan:?}, 5) = ({x:?}, {o:?})"
    );
    assert!(x.is_nan());
    assert_eq!(o, Equal);

    // - a Rational
    let third = Rational::from_signeds(1, 3);
    let (x, o) = tiny.min_rational_prec_round_ref_ref(&third, 10, Floor);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{tiny:?}.min_rational_prec_round({third}, 10, Floor) = ({x:?}, {o:?})"
    );
    assert!(same(&x, &tiny));
    assert_eq!(o, Equal);
    let (x, o) = tiny.max_rational_prec_round_ref_ref(&third, 10, Floor);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{tiny:?}.max_rational_prec_round({third}, 10, Floor) = ({x:?}, {o:?})"
    );
    assert_eq!(format!("{x:?}"), "0x0.554");
    assert_eq!(o, Less);
    let (x, o) = nan.max_rational_prec_ref_ref(&third, 10);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{nan:?}.max_rational_prec({third}, 10) = ({x:?}, {o:?})"
    );
    assert_eq!(format!("{x:?}"), "0x0.556");
    assert_eq!(o, Greater);
    let (x, o) = huge.min_rational_prec_ref_ref(&-third.clone(), 10);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{huge:?}.min_rational_prec({}, 10) = ({x:?}, {o:?})",
        -&third
    );
    assert_eq!(format!("{x:?}"), "-0x0.556");
    assert_eq!(o, Less);
    // - a tie selects the BigExponentFloat
    let (x, o) =
        BigExponentFloat::ONE_HALF.min_rational_prec_round_ref_ref(&Rational::ONE_HALF, 1, Exact);
    oracle_row!(
        "flint",
        "arf_min_max",
        "{:?}.min_rational_prec_round({}, 1, Exact) = ({x:?}, {o:?})",
        BigExponentFloat::ONE_HALF,
        Rational::ONE_HALF
    );
    assert!(same(&x, &BigExponentFloat::ONE_HALF));
    assert_eq!(o, Equal);
    // - the comparison is exact: 0x0.554 is the 10-bit Floor of 1/3, but less than it
    let x = parse_big_exponent_float_hex("0x0.554#10");
    let y = x.max_rational_prec_round_ref_ref(&third, 64, Nearest).0;
    oracle_row!(
        "flint",
        "arf_min_max",
        "{x:?}.max_rational_prec_round({third}, 64, Nearest) = ({y:?}, {:?})",
        x.max_rational_prec_round_ref_ref(&third, 64, Nearest).1
    );
    assert_eq!(format!("{y:?}"), "0x0.55555555555555558");
}

#[test]
#[should_panic]
fn min_prec_round_fail() {
    BigExponentFloat::NAN.min_prec_round(BigExponentFloat::NAN, 0, Floor);
}

#[test]
#[should_panic]
fn max_prec_round_fail() {
    BigExponentFloat::from(3).max_prec_round(BigExponentFloat::ONE, 1, Exact);
}

#[test]
#[should_panic]
fn min_rational_prec_round_fail() {
    BigExponentFloat::NAN.min_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn max_rational_prec_round_fail() {
    BigExponentFloat::ZERO.max_rational_prec_round(Rational::from_signeds(1, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn min_max_properties_helper(x: BigExponentFloat, y: BigExponentFloat) {
    let min = x.min_ref_ref(&y);
    assert!(same(&x.clone().min(y.clone()), &min));
    assert!(same(&x.clone().min_val_ref(&y), &min));
    assert!(same(&x.min_ref_val(y.clone()), &min));
    let max = x.max_ref_ref(&y);
    assert!(same(&x.clone().max(y.clone()), &max));
    assert!(same(&x.clone().max_val_ref(&y), &max));
    assert!(same(&x.max_ref_val(y.clone()), &max));

    assert!(same(&y.min_ref_ref(&x), &min));
    assert!(same(&y.max_ref_ref(&x), &max));
    assert!(same(&min, &x) || same(&min, &y));
    assert!(same(&max, &x) || same(&max, &y));
    if x.is_nan() || y.is_nan() {
        let other = if x.is_nan() { &y } else { &x };
        assert!(same(&min, other));
        assert!(same(&max, other));
    } else {
        assert!(min <= x && min <= y);
        assert!(max >= x && max >= y);
        assert!((same(&min, &x) && same(&max, &y)) || (same(&min, &y) && same(&max, &x)));
    }

    for prec in PRECS {
        for rm in RMS {
            let (r, o) = x.min_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = min.round_to_prec_round_ref(prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().min_prec_round(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().min_prec_round_val_ref(&y, prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.min_prec_round_ref_val(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);

            let (r, o) = x.max_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = max.round_to_prec_round_ref(prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().max_prec_round(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().max_prec_round_val_ref(&y, prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.max_prec_round_ref_val(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
        let (r, o) = x.min_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.min_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().min_prec(y.clone(), prec),
            x.clone().min_prec_val_ref(&y, prec),
            x.min_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
        let (r, o) = x.max_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.max_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().max_prec(y.clone(), prec),
            x.clone().max_prec_val_ref(&y, prec),
            x.max_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
    }
}

#[test]
fn min_max_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        min_max_properties_helper(x, y);
    });

    // - agreement with `Float`, whose selection rules are the same apart from the sign of zero
    float_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        let by = float_to_big_exponent_float(&y);
        assert!(same(
            &bx.min_ref_ref(&by),
            &float_to_big_exponent_float(&x.min_ref_ref(&y).0)
        ));
        assert!(same(
            &bx.max_ref_ref(&by),
            &float_to_big_exponent_float(&x.max_ref_ref(&y).0)
        ));
    });
}

// The result is the winner of the exact comparison, rounded. A NaN `BigExponentFloat` loses, and a
// tie goes to the `BigExponentFloat`.
fn rational_alt(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
    is_max: bool,
) -> (BigExponentFloat, Ordering) {
    let rational_wins = x
        .partial_cmp(y)
        .is_none_or(|o| o == if is_max { Less } else { Greater });
    if rational_wins {
        BigExponentFloat::from_rational_prec_round_ref(y, prec, rm)
    } else {
        x.round_to_prec_round_ref(prec, rm)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn min_max_rational_properties_helper(x: BigExponentFloat, y: Rational) {
    for prec in PRECS {
        for rm in RMS {
            let (r, o) = x.min_rational_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = rational_alt(&x, &y, prec, rm, false);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            for (r_alt, o_alt) in [
                x.clone().min_rational_prec_round(y.clone(), prec, rm),
                x.clone().min_rational_prec_round_val_ref(&y, prec, rm),
                x.min_rational_prec_round_ref_val(y.clone(), prec, rm),
            ] {
                assert!(same(&r, &r_alt));
                assert_eq!(o, o_alt);
            }
            if !x.is_nan() {
                assert!(r <= x || o == Greater);
            }

            let (r, o) = x.max_rational_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = rational_alt(&x, &y, prec, rm, true);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            for (r_alt, o_alt) in [
                x.clone().max_rational_prec_round(y.clone(), prec, rm),
                x.clone().max_rational_prec_round_val_ref(&y, prec, rm),
                x.max_rational_prec_round_ref_val(y.clone(), prec, rm),
            ] {
                assert!(same(&r, &r_alt));
                assert_eq!(o, o_alt);
            }
            if !x.is_nan() {
                assert!(r >= x || o == Less);
            }
        }
        let (r, o) = x.min_rational_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.min_rational_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().min_rational_prec(y.clone(), prec),
            x.clone().min_rational_prec_val_ref(&y, prec),
            x.min_rational_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
        let (r, o) = x.max_rational_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.max_rational_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().max_rational_prec(y.clone(), prec),
            x.clone().max_rational_prec_val_ref(&y, prec),
            x.max_rational_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
    }
}

#[test]
fn min_max_rational_properties() {
    big_exponent_float_rational_pair_gen().test_properties(|(x, y)| {
        min_max_rational_properties_helper(x, y);
    });

    // - agreement with `Float`, wherever `Float` neither overflows nor underflows
    float_rational_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        for prec in PRECS {
            for rm in RMS {
                for (b, f) in [
                    (
                        bx.min_rational_prec_round_ref_ref(&y, prec, rm),
                        x.min_rational_prec_round_ref_ref(&y, prec, rm),
                    ),
                    (
                        bx.max_rational_prec_round_ref_ref(&y, prec, rm),
                        x.max_rational_prec_round_ref_ref(&y, prec, rm),
                    ),
                ] {
                    let fb = float_to_big_exponent_float(&f.0);
                    if fb.is_normal() == b.0.is_normal() {
                        assert!(same(&b.0, &fb));
                        assert_eq!(b.1, f.1);
                    }
                }
            }
        }
    });
}

#[test]
fn min_max_float_agreement_properties() {
    // - agreement with `Float`'s rounding versions, wherever `Float` does not overflow
    float_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        let by = float_to_big_exponent_float(&y);
        for prec in PRECS {
            for rm in RMS {
                for (b, f) in [
                    (
                        bx.min_prec_round_ref_ref(&by, prec, rm),
                        Float::min_prec_round_ref_ref(&x, &y, prec, rm),
                    ),
                    (
                        bx.max_prec_round_ref_ref(&by, prec, rm),
                        Float::max_prec_round_ref_ref(&x, &y, prec, rm),
                    ),
                ] {
                    let fb = float_to_big_exponent_float(&f.0);
                    if fb.is_normal() == b.0.is_normal() {
                        assert!(same(&b.0, &fb));
                        assert_eq!(b.1, f.1);
                    }
                }
            }
        }
    });
}
