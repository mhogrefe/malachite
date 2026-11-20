// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, One, Two, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::gamma::ln_gamma::{
    ln_gamma_prec_round_enclosures, ln_gamma_rational_prec_round_enclosures,
};
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn ln_gamma_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().ln_gamma_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.ln_gamma_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.ln_gamma_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().ln_gamma_prec(prec), &r));
        assert!(same_pair(&x.ln_gamma_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.ln_gamma_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn ln_gamma_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::ln_gamma_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::ln_gamma_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::ln_gamma_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::ln_gamma_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_ln_gamma_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &ln_gamma_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_prec_round_all(&zero, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(inf.clone(), Equal)
        ));
        // The function is not defined at negative arguments.
        assert!(same_pair(
            &ln_gamma_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_rational_prec_round_all(&Rational::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        // The zeros of the function
        assert!(same_pair(
            &ln_gamma_prec_round_all(&BigExponentFloat::ONE, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_prec_round_all(&BigExponentFloat::TWO, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_rational_prec_round_all(&Rational::ONE, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_gamma_rational_prec_round_all(&Rational::TWO, 10, rm),
            &(zero.clone(), Equal)
        ));
    }
    // The expected values agree with Azurite's logarithm of the gamma function.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = ln_gamma_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Up, "0.0", "0x0.0", Equal);
    // - x = 1
    test("1.0", "0x1.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 10, Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 10, Up, "0.0", "0x0.0", Equal);
    // - x = 2
    test("2.0", "0x2.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0", 10, Floor, "0.69238", "0x0.b14", Less);
    test("3.0", "0x3.0", 10, Ceiling, "0.6934", "0x0.b18", Greater);
    test("3.0", "0x3.0", 10, Down, "0.69238", "0x0.b14", Less);
    test("3.0", "0x3.0", 10, Up, "0.6934", "0x0.b18", Greater);
    test("3.0", "0x3.0", 10, Nearest, "0.6934", "0x0.b18", Greater);
    test("4.0", "0x4.0", 10, Floor, "1.7910", "0x1.ca8", Less);
    test("4.0", "0x4.0", 10, Ceiling, "1.793", "0x1.cb", Greater);
    test("4.0", "0x4.0", 10, Down, "1.7910", "0x1.ca8", Less);
    test("4.0", "0x4.0", 10, Up, "1.793", "0x1.cb", Greater);
    test("4.0", "0x4.0", 10, Nearest, "1.7910", "0x1.ca8", Less);
    test("5.0", "0x5.0", 10, Floor, "3.1758", "0x3.2d", Less);
    test("5.0", "0x5.0", 10, Ceiling, "3.180", "0x3.2e", Greater);
    test("5.0", "0x5.0", 10, Down, "3.1758", "0x3.2d", Less);
    test("5.0", "0x5.0", 10, Up, "3.180", "0x3.2e", Greater);
    // - x is a small positive integer
    test("5.0", "0x5.0", 10, Nearest, "3.180", "0x3.2e", Greater);
    test("10.0", "0xa.0", 10, Floor, "12.797", "0xc.cc", Less);
    test("10.0", "0xa.0", 10, Ceiling, "12.81", "0xc.d", Greater);
    test("10.0", "0xa.0", 10, Down, "12.797", "0xc.cc", Less);
    test("10.0", "0xa.0", 10, Up, "12.81", "0xc.d", Greater);
    test("10.0", "0xa.0", 10, Nearest, "12.797", "0xc.cc", Less);
    test("25.0", "0x19.0", 10, Floor, "54.75", "0x36.c", Less);
    test("25.0", "0x19.0", 10, Ceiling, "54.812", "0x36.d", Greater);
    test("25.0", "0x19.0", 10, Down, "54.75", "0x36.c", Less);
    test("25.0", "0x19.0", 10, Up, "54.812", "0x36.d", Greater);
    test("25.0", "0x19.0", 10, Nearest, "54.812", "0x36.d", Greater);
    test("171.0", "0xab.0", 10, Floor, "706.0", "0x2c2.0", Less);
    test("171.0", "0xab.0", 10, Ceiling, "707.00", "0x2c3.0", Greater);
    test("171.0", "0xab.0", 10, Down, "706.0", "0x2c2.0", Less);
    test("171.0", "0xab.0", 10, Up, "707.00", "0x2c3.0", Greater);
    test("171.0", "0xab.0", 10, Nearest, "707.00", "0x2c3.0", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.5723", "0x0.928", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.57324", "0x0.92c", Greater);
    test("0.50", "0x0.8", 10, Down, "0.5723", "0x0.928", Less);
    test("0.50", "0x0.8", 10, Up, "0.57324", "0x0.92c", Greater);
    // - x > 0
    // - x - 1 and x - 2 are not small
    // - the argument is shifted
    test("0.50", "0x0.8", 10, Nearest, "0.5723", "0x0.928", Less);
    test("1.5", "0x1.8", 10, Floor, "-0.1208", "-0x0.1ef", Less);
    test(
        "1.5",
        "0x1.8",
        10,
        Ceiling,
        "-0.12073",
        "-0x0.1ee8",
        Greater,
    );
    test("1.5", "0x1.8", 10, Down, "-0.12073", "-0x0.1ee8", Greater);
    test("1.5", "0x1.8", 10, Up, "-0.1208", "-0x0.1ef", Less);
    test(
        "1.5",
        "0x1.8",
        10,
        Nearest,
        "-0.12073",
        "-0x0.1ee8",
        Greater,
    );
    test("0.25", "0x0.4", 10, Floor, "1.2871", "0x1.498", Less);
    test("0.25", "0x0.4", 10, Ceiling, "1.289", "0x1.4a", Greater);
    test("0.25", "0x0.4", 10, Down, "1.2871", "0x1.498", Less);
    test("0.25", "0x0.4", 10, Up, "1.289", "0x1.4a", Greater);
    test("0.25", "0x0.4", 10, Nearest, "1.2871", "0x1.498", Less);
    test("0.75", "0x0.c", 10, Floor, "0.203", "0x0.34", Less);
    test("0.75", "0x0.c", 10, Ceiling, "0.20337", "0x0.341", Greater);
    test("0.75", "0x0.c", 10, Down, "0.203", "0x0.34", Less);
    test("0.75", "0x0.c", 10, Up, "0.20337", "0x0.341", Greater);
    test("0.75", "0x0.c", 10, Nearest, "0.20337", "0x0.341", Greater);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Floor,
        "2.25",
        "0x2.4",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Ceiling,
        "2.2539",
        "0x2.41",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Down,
        "2.25",
        "0x2.4",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Up,
        "2.2539",
        "0x2.41",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Nearest,
        "2.2539",
        "0x2.41",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Floor,
        "0.9863",
        "0x0.fc8",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Ceiling,
        "0.98730",
        "0x0.fcc",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Down,
        "0.9863",
        "0x0.fc8",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Up,
        "0.98730",
        "0x0.fcc",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Nearest,
        "0.98730",
        "0x0.fcc",
        Greater,
    );
    test("-0.50", "-0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Down, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Up, "NaN", "NaN", Equal);
    // - x < 0
    test("-0.50", "-0x0.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Down, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Up, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Exact, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8", 10, Floor, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8", 10, Down, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8", 10, Up, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8", 10, Exact, "NaN", "NaN", Equal);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("-1.0", "-0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0", 10, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0", 10, Down, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0", 10, Up, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0", 10, Exact, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8", 10, Floor, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8", 10, Down, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8", 10, Up, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Floor,
        "-5.77e-8",
        "-0xf.8E-7",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Ceiling,
        "-5.7684e-8",
        "-0xf.7cE-7",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Down,
        "-5.7684e-8",
        "-0xf.7cE-7",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Up,
        "-5.77e-8",
        "-0xf.8E-7",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Nearest,
        "-5.77e-8",
        "-0xf.8E-7",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Floor,
        "-4.2317e-8",
        "-0xb.5cE-7",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Ceiling,
        "-4.226e-8",
        "-0xb.58E-7",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Down,
        "-4.226e-8",
        "-0xb.58E-7",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Up,
        "-4.2317e-8",
        "-0xb.5cE-7",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Nearest,
        "-4.226e-8",
        "-0xb.58E-7",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Floor,
        "0.82715",
        "0x0.d3c",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Ceiling,
        "0.828",
        "0x0.d4",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Down,
        "0.82715",
        "0x0.d3c",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Up,
        "0.828",
        "0x0.d4",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Nearest,
        "0.828",
        "0x0.d4",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Floor,
        "0.98535",
        "0x0.fc4",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Ceiling,
        "0.9863",
        "0x0.fc8",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Down,
        "0.98535",
        "0x0.fc4",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Up,
        "0.9863",
        "0x0.fc8",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Nearest,
        "0.98535",
        "0x0.fc4",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Floor,
        "23.0",
        "0x17.0",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Ceiling,
        "23.031",
        "0x17.08",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Down,
        "23.0",
        "0x17.0",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Up,
        "23.031",
        "0x17.08",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Nearest,
        "23.031",
        "0x17.08",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Floor,
        "690.0",
        "0x2b2.0",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Ceiling,
        "691.00",
        "0x2b3.0",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Down,
        "690.0",
        "0x2b2.0",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Up,
        "691.00",
        "0x2b3.0",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Nearest,
        "691.00",
        "0x2b3.0",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("1000.5", "0x3e8.8", 10, Floor, "5904.0", "0x1.71E+3", Less);
    test(
        "1000.5", "0x3e8.8", 10, Ceiling, "5912.0", "0x1718.0", Greater,
    );
    test("1000.5", "0x3e8.8", 10, Down, "5904.0", "0x1.71E+3", Less);
    test("1000.5", "0x3e8.8", 10, Up, "5912.0", "0x1718.0", Greater);
    test(
        "1000.5", "0x3e8.8", 10, Nearest, "5912.0", "0x1718.0", Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Floor,
        "1.0506e6",
        "0x1.008E+5",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Ceiling,
        "1.053e6",
        "0x1.01E+5",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Down,
        "1.0506e6",
        "0x1.008E+5",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Up,
        "1.053e6",
        "0x1.01E+5",
        Greater,
    );
    // - x is an integer above the bound for the factorial
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Nearest,
        "1.0506e6",
        "0x1.008E+5",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Floor,
        "1.281e7",
        "0xc.38E+5",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Ceiling,
        "1.2829e7",
        "0xc.3cE+5",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Down,
        "1.281e7",
        "0xc.38E+5",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Up,
        "1.2829e7",
        "0xc.3cE+5",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Nearest,
        "1.281e7",
        "0xc.38E+5",
        Less,
    );
    test("-1000000.5", "-0xf4240.8", 10, Floor, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8", 10, Down, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8", 10, Up, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8", 10, Exact, "NaN", "NaN", Equal);
    test("6.7e7", "0x4.0E+6", 10, Floor, "1.14e9", "0x4.4E+7", Less);
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Ceiling,
        "1.1429e9",
        "0x4.42E+7",
        Greater,
    );
    test("6.7e7", "0x4.0E+6", 10, Down, "1.14e9", "0x4.4E+7", Less);
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Up,
        "1.1429e9",
        "0x4.42E+7",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Nearest,
        "1.1429e9",
        "0x4.42E+7",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Floor,
        "1.14e9",
        "0x4.4E+7",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Ceiling,
        "1.1429e9",
        "0x4.42E+7",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Down,
        "1.14e9",
        "0x4.4E+7",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Up,
        "1.1429e9",
        "0x4.42E+7",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Nearest,
        "1.1429e9",
        "0x4.42E+7",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Floor,
        "7.6128e11",
        "0xb.14E+9",
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Ceiling,
        "7.624e11",
        "0xb.18E+9",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Down,
        "7.6128e11",
        "0xb.14E+9",
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Up,
        "7.624e11",
        "0xb.18E+9",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Nearest,
        "7.624e11",
        "0xb.18E+9",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "8.652e31",
        "0x4.44E+26",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "8.6676e31",
        "0x4.46E+26",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Down,
        "8.652e31",
        "0x4.44E+26",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Up,
        "8.6676e31",
        "0x4.46E+26",
        Greater,
    );
    // - the argument is not shifted
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Nearest,
        "8.6676e31",
        "0x4.46E+26",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Floor,
        "8.652e31",
        "0x4.44E+26",
        Less,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Ceiling,
        "8.6676e31",
        "0x4.46E+26",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Down,
        "8.652e31",
        "0x4.44E+26",
        Less,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Up,
        "8.6676e31",
        "0x4.46E+26",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Nearest,
        "8.6676e31",
        "0x4.46E+26",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        10,
        Floor,
        "6.1338e330985980553",
        "0xb.14E+274877906953",
        Less,
    );
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        10,
        Ceiling,
        "6.142e330985980553",
        "0xb.18E+274877906953",
        Greater,
    );
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        10,
        Down,
        "6.1338e330985980553",
        "0xb.14E+274877906953",
        Less,
    );
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        10,
        Up,
        "6.142e330985980553",
        "0xb.18E+274877906953",
        Greater,
    );
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        10,
        Nearest,
        "6.142e330985980553",
        "0xb.18E+274877906953",
        Greater,
    );
    test("1.0", "0x1.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 100, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 1, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 100, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 1, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 100, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 100, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 1, Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 100, Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 1, Up, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 100, Up, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0", 1, Floor, "0.50", "0x0.8", Less);
    test(
        "3.0",
        "0x3.0",
        100,
        Floor,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test("3.0", "0x3.0", 1, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "3.0",
        "0x3.0",
        100,
        Ceiling,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981",
        Greater,
    );
    test("3.0", "0x3.0", 1, Down, "0.50", "0x0.8", Less);
    test(
        "3.0",
        "0x3.0",
        100,
        Down,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test("3.0", "0x3.0", 1, Up, "1.0", "0x1.0", Greater);
    test(
        "3.0",
        "0x3.0",
        100,
        Up,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981",
        Greater,
    );
    test("3.0", "0x3.0", 1, Nearest, "0.50", "0x0.8", Less);
    test(
        "3.0",
        "0x3.0",
        100,
        Nearest,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test("4.0", "0x4.0", 1, Floor, "1.0", "0x1.0", Less);
    test(
        "4.0",
        "0x4.0",
        100,
        Floor,
        "1.791759469228055000812477358380",
        "0x1.cab0bfa2a20023226dfd40ed",
        Less,
    );
    test("4.0", "0x4.0", 1, Ceiling, "2.0", "0x2.0", Greater);
    test(
        "4.0",
        "0x4.0",
        100,
        Ceiling,
        "1.7917594692280550008124773583818",
        "0x1.cab0bfa2a20023226dfd40ed2",
        Greater,
    );
    test("4.0", "0x4.0", 1, Down, "1.0", "0x1.0", Less);
    test(
        "4.0",
        "0x4.0",
        100,
        Down,
        "1.791759469228055000812477358380",
        "0x1.cab0bfa2a20023226dfd40ed",
        Less,
    );
    test("4.0", "0x4.0", 1, Up, "2.0", "0x2.0", Greater);
    test(
        "4.0",
        "0x4.0",
        100,
        Up,
        "1.7917594692280550008124773583818",
        "0x1.cab0bfa2a20023226dfd40ed2",
        Greater,
    );
    test("4.0", "0x4.0", 1, Nearest, "2.0", "0x2.0", Greater);
    test(
        "4.0",
        "0x4.0",
        100,
        Nearest,
        "1.791759469228055000812477358380",
        "0x1.cab0bfa2a20023226dfd40ed",
        Less,
    );
    test("5.0", "0x5.0", 1, Floor, "2.0", "0x2.0", Less);
    test(
        "5.0",
        "0x5.0",
        100,
        Floor,
        "3.178053830347945619646941601296",
        "0x3.2d94ef92459f167a01c4a81d",
        Less,
    );
    test("5.0", "0x5.0", 1, Ceiling, "4.0", "0x4.0", Greater);
    test(
        "5.0",
        "0x5.0",
        100,
        Ceiling,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4",
        Greater,
    );
    test("5.0", "0x5.0", 1, Down, "2.0", "0x2.0", Less);
    test(
        "5.0",
        "0x5.0",
        100,
        Down,
        "3.178053830347945619646941601296",
        "0x3.2d94ef92459f167a01c4a81d",
        Less,
    );
    test("5.0", "0x5.0", 1, Up, "4.0", "0x4.0", Greater);
    test(
        "5.0",
        "0x5.0",
        100,
        Up,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4",
        Greater,
    );
    test("5.0", "0x5.0", 1, Nearest, "4.0", "0x4.0", Greater);
    test(
        "5.0",
        "0x5.0",
        100,
        Nearest,
        "3.178053830347945619646941601296",
        "0x3.2d94ef92459f167a01c4a81d",
        Less,
    );
    test("10.0", "0xa.0", 1, Floor, "8.0", "0x8.0", Less);
    test(
        "10.0",
        "0xa.0",
        100,
        Floor,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61",
        Less,
    );
    test("10.0", "0xa.0", 1, Ceiling, "16.0", "0x1.0E+1", Greater);
    test(
        "10.0",
        "0xa.0",
        100,
        Ceiling,
        "12.80182748008146961120771787457",
        "0xc.cd4490d3fbe7a583ebac5a62",
        Greater,
    );
    test("10.0", "0xa.0", 1, Down, "8.0", "0x8.0", Less);
    test(
        "10.0",
        "0xa.0",
        100,
        Down,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61",
        Less,
    );
    test("10.0", "0xa.0", 1, Up, "16.0", "0x1.0E+1", Greater);
    test(
        "10.0",
        "0xa.0",
        100,
        Up,
        "12.80182748008146961120771787457",
        "0xc.cd4490d3fbe7a583ebac5a62",
        Greater,
    );
    test("10.0", "0xa.0", 1, Nearest, "16.0", "0x1.0E+1", Greater);
    test(
        "10.0",
        "0xa.0",
        100,
        Nearest,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61",
        Less,
    );
    test("25.0", "0x19.0", 1, Floor, "32.0", "0x2.0E+1", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Floor,
        "54.784729398112319190093344083558",
        "0x36.c8e4069d1a2824fcb11da734",
        Less,
    );
    test("25.0", "0x19.0", 1, Ceiling, "64.0", "0x4.0E+1", Greater);
    test(
        "25.0",
        "0x19.0",
        100,
        Ceiling,
        "54.78472939811231919009334408361",
        "0x36.c8e4069d1a2824fcb11da738",
        Greater,
    );
    test("25.0", "0x19.0", 1, Down, "32.0", "0x2.0E+1", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Down,
        "54.784729398112319190093344083558",
        "0x36.c8e4069d1a2824fcb11da734",
        Less,
    );
    test("25.0", "0x19.0", 1, Up, "64.0", "0x4.0E+1", Greater);
    test(
        "25.0",
        "0x19.0",
        100,
        Up,
        "54.78472939811231919009334408361",
        "0x36.c8e4069d1a2824fcb11da738",
        Greater,
    );
    test("25.0", "0x19.0", 1, Nearest, "64.0", "0x4.0E+1", Greater);
    test(
        "25.0",
        "0x19.0",
        100,
        Nearest,
        "54.78472939811231919009334408361",
        "0x36.c8e4069d1a2824fcb11da738",
        Greater,
    );
    test("171.0", "0xab.0", 1, Floor, "5.1e2", "0x2.0E+2", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Floor,
        "706.5730622457873471107222627208",
        "0x2c2.92b435143a9dc09fad34148",
        Less,
    );
    test("171.0", "0xab.0", 1, Ceiling, "1.0e3", "0x4.0E+2", Greater);
    test(
        "171.0",
        "0xab.0",
        100,
        Ceiling,
        "706.57306224578734711072226272161",
        "0x2c2.92b435143a9dc09fad3414c",
        Greater,
    );
    test("171.0", "0xab.0", 1, Down, "5.1e2", "0x2.0E+2", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Down,
        "706.5730622457873471107222627208",
        "0x2c2.92b435143a9dc09fad34148",
        Less,
    );
    test("171.0", "0xab.0", 1, Up, "1.0e3", "0x4.0E+2", Greater);
    test(
        "171.0",
        "0xab.0",
        100,
        Up,
        "706.57306224578734711072226272161",
        "0x2c2.92b435143a9dc09fad3414c",
        Greater,
    );
    test("171.0", "0xab.0", 1, Nearest, "5.1e2", "0x2.0E+2", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Nearest,
        "706.57306224578734711072226272161",
        "0x2c2.92b435143a9dc09fad3414c",
        Greater,
    );
    test("0.50", "0x0.8", 1, Floor, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Floor,
        "0.5723649429247000870717136756764",
        "0x0.928682473d0de85eafcab6354",
        Less,
    );
    test("0.50", "0x0.8", 1, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Ceiling,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355",
        Greater,
    );
    test("0.50", "0x0.8", 1, Down, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Down,
        "0.5723649429247000870717136756764",
        "0x0.928682473d0de85eafcab6354",
        Less,
    );
    test("0.50", "0x0.8", 1, Up, "1.0", "0x1.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Up,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.5723649429247000870717136756764",
        "0x0.928682473d0de85eafcab6354",
        Less,
    );
    test("1.5", "0x1.8", 1, Floor, "-0.12", "-0x0.2", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Floor,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2",
        Less,
    );
    test("1.5", "0x1.8", 1, Ceiling, "-0.062", "-0x0.1", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Ceiling,
        "-0.120782237635245222345518445782",
        "-0x0.1eeb95b094c1914d1a18fd62c",
        Greater,
    );
    test("1.5", "0x1.8", 1, Down, "-0.062", "-0x0.1", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Down,
        "-0.120782237635245222345518445782",
        "-0x0.1eeb95b094c1914d1a18fd62c",
        Greater,
    );
    test("1.5", "0x1.8", 1, Up, "-0.12", "-0x0.2", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Up,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2",
        Less,
    );
    test("1.5", "0x1.8", 1, Nearest, "-0.12", "-0x0.2", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Nearest,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2",
        Less,
    );
    test("0.25", "0x0.4", 1, Floor, "1.0", "0x1.0", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Floor,
        "1.288022524698077457370610440220",
        "0x1.49bbd81c16efb1e317135ab9",
        Less,
    );
    test("0.25", "0x0.4", 1, Ceiling, "2.0", "0x2.0", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Ceiling,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92",
        Greater,
    );
    test("0.25", "0x0.4", 1, Down, "1.0", "0x1.0", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Down,
        "1.288022524698077457370610440220",
        "0x1.49bbd81c16efb1e317135ab9",
        Less,
    );
    test("0.25", "0x0.4", 1, Up, "2.0", "0x2.0", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Up,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92",
        Greater,
    );
    test("0.25", "0x0.4", 1, Nearest, "1.0", "0x1.0", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Nearest,
        "1.288022524698077457370610440220",
        "0x1.49bbd81c16efb1e317135ab9",
        Less,
    );
    test("0.75", "0x0.c", 1, Floor, "0.12", "0x0.2", Less);
    test(
        "0.75",
        "0x0.c",
        100,
        Floor,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84",
        Less,
    );
    test("0.75", "0x0.c", 1, Ceiling, "0.25", "0x0.4", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Ceiling,
        "0.2032809514312953714814329718626",
        "0x0.340a386e4c13dbb02d73eb7d88",
        Greater,
    );
    test("0.75", "0x0.c", 1, Down, "0.12", "0x0.2", Less);
    test(
        "0.75",
        "0x0.c",
        100,
        Down,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84",
        Less,
    );
    test("0.75", "0x0.c", 1, Up, "0.25", "0x0.4", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Up,
        "0.2032809514312953714814329718626",
        "0x0.340a386e4c13dbb02d73eb7d88",
        Greater,
    );
    test("0.75", "0x0.c", 1, Nearest, "0.25", "0x0.4", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Nearest,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84",
        Less,
    );
    // - x - 1 is small
    test(
        "1.00000006",
        "0x1.000001",
        10,
        Nearest,
        "-3.4401e-8",
        "-0x9.3cE-7",
        Greater,
    );
    // - x - 2 is small
    test(
        "1.99999988",
        "0x1.fffffe",
        10,
        Nearest,
        "-5.041e-8",
        "-0xd.88E-7",
        Less,
    );
}

#[test]
fn test_ln_gamma_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = ln_gamma_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    // - q = 0
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Down, "0.0", "0x0.0", Equal);
    test("1", 10, Up, "0.0", "0x0.0", Equal);
    // - q = 1
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("2", 10, Floor, "0.0", "0x0.0", Equal);
    test("2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2", 10, Down, "0.0", "0x0.0", Equal);
    test("2", 10, Up, "0.0", "0x0.0", Equal);
    // - q = 2
    test("2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("2", 10, Exact, "0.0", "0x0.0", Equal);
    test("3", 10, Floor, "0.69238", "0x0.b14", Less);
    test("3", 10, Ceiling, "0.6934", "0x0.b18", Greater);
    test("3", 10, Down, "0.69238", "0x0.b14", Less);
    test("3", 10, Up, "0.6934", "0x0.b18", Greater);
    test("3", 10, Nearest, "0.6934", "0x0.b18", Greater);
    test("5", 10, Floor, "3.1758", "0x3.2d", Less);
    test("5", 10, Ceiling, "3.180", "0x3.2e", Greater);
    test("5", 10, Down, "3.1758", "0x3.2d", Less);
    test("5", 10, Up, "3.180", "0x3.2e", Greater);
    // - q is a small positive integer
    test("5", 10, Nearest, "3.180", "0x3.2e", Greater);
    test("25", 10, Floor, "54.75", "0x36.c", Less);
    test("25", 10, Ceiling, "54.812", "0x36.d", Greater);
    test("25", 10, Down, "54.75", "0x36.c", Less);
    test("25", 10, Up, "54.812", "0x36.d", Greater);
    test("25", 10, Nearest, "54.812", "0x36.d", Greater);
    test("1/2", 10, Floor, "0.5723", "0x0.928", Less);
    test("1/2", 10, Ceiling, "0.57324", "0x0.92c", Greater);
    test("1/2", 10, Down, "0.5723", "0x0.928", Less);
    test("1/2", 10, Up, "0.57324", "0x0.92c", Greater);
    test("1/2", 10, Nearest, "0.5723", "0x0.928", Less);
    test("1/3", 10, Floor, "0.98535", "0x0.fc4", Less);
    test("1/3", 10, Ceiling, "0.9863", "0x0.fc8", Greater);
    test("1/3", 10, Down, "0.98535", "0x0.fc4", Less);
    test("1/3", 10, Up, "0.9863", "0x0.fc8", Greater);
    // - q > 0
    // - q - 1 and q - 2 are not small
    test("1/3", 10, Nearest, "0.98535", "0x0.fc4", Less);
    test("1/4", 10, Floor, "1.2871", "0x1.498", Less);
    test("1/4", 10, Ceiling, "1.289", "0x1.4a", Greater);
    test("1/4", 10, Down, "1.2871", "0x1.498", Less);
    test("1/4", 10, Up, "1.289", "0x1.4a", Greater);
    test("1/4", 10, Nearest, "1.2871", "0x1.498", Less);
    test("1/6", 10, Floor, "1.715", "0x1.b7", Less);
    test("1/6", 10, Ceiling, "1.7168", "0x1.b78", Greater);
    test("1/6", 10, Down, "1.715", "0x1.b7", Less);
    test("1/6", 10, Up, "1.7168", "0x1.b78", Greater);
    test("1/6", 10, Nearest, "1.7168", "0x1.b78", Greater);
    test("2/3", 10, Floor, "0.3027", "0x0.4d8", Less);
    test("2/3", 10, Ceiling, "0.30322", "0x0.4da", Greater);
    test("2/3", 10, Down, "0.3027", "0x0.4d8", Less);
    test("2/3", 10, Up, "0.30322", "0x0.4da", Greater);
    test("2/3", 10, Nearest, "0.30322", "0x0.4da", Greater);
    test("-1/2", 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 10, Down, "NaN", "NaN", Equal);
    test("-1/2", 10, Up, "NaN", "NaN", Equal);
    // - q < 0
    test("-1/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 10, Exact, "NaN", "NaN", Equal);
    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Exact, "NaN", "NaN", Equal);
    test("-2", 10, Floor, "NaN", "NaN", Equal);
    test("-2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2", 10, Down, "NaN", "NaN", Equal);
    test("-2", 10, Up, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Exact, "NaN", "NaN", Equal);
    test("-5/2", 10, Floor, "NaN", "NaN", Equal);
    test("-5/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-5/2", 10, Down, "NaN", "NaN", Equal);
    test("-5/2", 10, Up, "NaN", "NaN", Equal);
    test("-5/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-5/2", 10, Exact, "NaN", "NaN", Equal);
    test("-7/3", 10, Floor, "NaN", "NaN", Equal);
    test("-7/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-7/3", 10, Down, "NaN", "NaN", Equal);
    test("-7/3", 10, Up, "NaN", "NaN", Equal);
    test("-7/3", 10, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 10, Exact, "NaN", "NaN", Equal);
    test("22/7", 10, Floor, "0.828", "0x0.d4", Less);
    test("22/7", 10, Ceiling, "0.82910", "0x0.d44", Greater);
    test("22/7", 10, Down, "0.828", "0x0.d4", Less);
    test("22/7", 10, Up, "0.82910", "0x0.d44", Greater);
    test("22/7", 10, Nearest, "0.82910", "0x0.d44", Greater);
    test("1000001/1000", 10, Floor, "5904.0", "0x1.71E+3", Less);
    test("1000001/1000", 10, Ceiling, "5912.0", "0x1718.0", Greater);
    test("1000001/1000", 10, Down, "5904.0", "0x1.71E+3", Less);
    test("1000001/1000", 10, Up, "5912.0", "0x1718.0", Greater);
    test("1000001/1000", 10, Nearest, "5904.0", "0x1.71E+3", Less);
    test("-1000001/1000", 10, Floor, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Down, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Up, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Nearest, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "69.25",
        "0x45.4",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "69.375",
        "0x45.6",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "69.25",
        "0x45.4",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "69.375",
        "0x45.6",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "69.375",
        "0x45.6",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "-4.56e-31",
        "-0x9.4E-26",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "-4.5529e-31",
        "-0x9.3cE-26",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Down,
        "-4.5529e-31",
        "-0x9.3cE-26",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Up,
        "-4.56e-31",
        "-0x9.4E-26",
        Less,
    );
    // - q - 1 is small
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "-4.5529e-31",
        "-0x9.3cE-26",
        Greater,
    );
    test("100000000", 10, Floor, "1.741e9", "0x6.7cE+7", Less);
    test("100000000", 10, Ceiling, "1.7427e9", "0x6.7eE+7", Greater);
    test("100000000", 10, Down, "1.741e9", "0x6.7cE+7", Less);
    test("100000000", 10, Up, "1.7427e9", "0x6.7eE+7", Greater);
    // - q is an integer above the bound for the factorial
    test("100000000", 10, Nearest, "1.7427e9", "0x6.7eE+7", Greater);
    test("200000001/2", 10, Floor, "1.741e9", "0x6.7cE+7", Less);
    test("200000001/2", 10, Ceiling, "1.7427e9", "0x6.7eE+7", Greater);
    test("200000001/2", 10, Down, "1.741e9", "0x6.7cE+7", Less);
    test("200000001/2", 10, Up, "1.7427e9", "0x6.7eE+7", Greater);
    test("200000001/2", 10, Nearest, "1.7427e9", "0x6.7eE+7", Greater);
    test("-200000001/2", 10, Floor, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Down, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Up, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Exact, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Floor, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Down, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Up, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Exact, "NaN", "NaN", Equal);
    test("0", 100, Floor, "Infinity", "Infinity", Equal);
    test("0", 100, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 100, Down, "Infinity", "Infinity", Equal);
    test("0", 100, Up, "Infinity", "Infinity", Equal);
    test("0", 100, Nearest, "Infinity", "Infinity", Equal);
    test("1", 100, Floor, "0.0", "0x0.0", Equal);
    test("1", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 100, Down, "0.0", "0x0.0", Equal);
    test("1", 100, Up, "0.0", "0x0.0", Equal);
    test("1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("2", 100, Floor, "0.0", "0x0.0", Equal);
    test("2", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("2", 100, Down, "0.0", "0x0.0", Equal);
    test("2", 100, Up, "0.0", "0x0.0", Equal);
    test("2", 100, Nearest, "0.0", "0x0.0", Equal);
    test(
        "3",
        100,
        Floor,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test(
        "3",
        100,
        Ceiling,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981",
        Greater,
    );
    test(
        "3",
        100,
        Down,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test(
        "3",
        100,
        Up,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981",
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test(
        "5",
        100,
        Floor,
        "3.178053830347945619646941601296",
        "0x3.2d94ef92459f167a01c4a81d",
        Less,
    );
    test(
        "5",
        100,
        Ceiling,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4",
        Greater,
    );
    test(
        "5",
        100,
        Down,
        "3.178053830347945619646941601296",
        "0x3.2d94ef92459f167a01c4a81d",
        Less,
    );
    test(
        "5",
        100,
        Up,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4",
        Greater,
    );
    test(
        "5",
        100,
        Nearest,
        "3.178053830347945619646941601296",
        "0x3.2d94ef92459f167a01c4a81d",
        Less,
    );
    test(
        "25",
        100,
        Floor,
        "54.784729398112319190093344083558",
        "0x36.c8e4069d1a2824fcb11da734",
        Less,
    );
    test(
        "25",
        100,
        Ceiling,
        "54.78472939811231919009334408361",
        "0x36.c8e4069d1a2824fcb11da738",
        Greater,
    );
    test(
        "25",
        100,
        Down,
        "54.784729398112319190093344083558",
        "0x36.c8e4069d1a2824fcb11da734",
        Less,
    );
    test(
        "25",
        100,
        Up,
        "54.78472939811231919009334408361",
        "0x36.c8e4069d1a2824fcb11da738",
        Greater,
    );
    test(
        "25",
        100,
        Nearest,
        "54.78472939811231919009334408361",
        "0x36.c8e4069d1a2824fcb11da738",
        Greater,
    );
    test(
        "1/2",
        100,
        Floor,
        "0.5723649429247000870717136756764",
        "0x0.928682473d0de85eafcab6354",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "0.5723649429247000870717136756764",
        "0x0.928682473d0de85eafcab6354",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.5723649429247000870717136756764",
        "0x0.928682473d0de85eafcab6354",
        Less,
    );
    test(
        "1/3",
        100,
        Floor,
        "0.985420646927767069187174036978",
        "0x0.fc44870b5ba0ce071ed58f22",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.98542064692776706918717403697846",
        "0x0.fc44870b5ba0ce071ed58f221",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.985420646927767069187174036978",
        "0x0.fc44870b5ba0ce071ed58f22",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.98542064692776706918717403697846",
        "0x0.fc44870b5ba0ce071ed58f221",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.985420646927767069187174036978",
        "0x0.fc44870b5ba0ce071ed58f22",
        Less,
    );
    test(
        "1/4",
        100,
        Floor,
        "1.288022524698077457370610440220",
        "0x1.49bbd81c16efb1e317135ab9",
        Less,
    );
    test(
        "1/4",
        100,
        Ceiling,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92",
        Greater,
    );
    test(
        "1/4",
        100,
        Down,
        "1.288022524698077457370610440220",
        "0x1.49bbd81c16efb1e317135ab9",
        Less,
    );
    test(
        "1/4",
        100,
        Up,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92",
        Greater,
    );
    test(
        "1/4",
        100,
        Nearest,
        "1.288022524698077457370610440220",
        "0x1.49bbd81c16efb1e317135ab9",
        Less,
    );
    test(
        "1/6",
        100,
        Floor,
        "1.716733435078240460527846309587",
        "0x1.b77bd7a79c5c35319ca148314",
        Less,
    );
    test(
        "1/6",
        100,
        Ceiling,
        "1.7167334350782404605278463095890",
        "0x1.b77bd7a79c5c35319ca148316",
        Greater,
    );
    test(
        "1/6",
        100,
        Down,
        "1.716733435078240460527846309587",
        "0x1.b77bd7a79c5c35319ca148314",
        Less,
    );
    test(
        "1/6",
        100,
        Up,
        "1.7167334350782404605278463095890",
        "0x1.b77bd7a79c5c35319ca148316",
        Greater,
    );
    test(
        "1/6",
        100,
        Nearest,
        "1.716733435078240460527846309587",
        "0x1.b77bd7a79c5c35319ca148314",
        Less,
    );
    test(
        "2/3",
        100,
        Floor,
        "0.30315027514752356867586281737163",
        "0x0.4d9b41a5883227a6b896ca35f8",
        Less,
    );
    test(
        "2/3",
        100,
        Ceiling,
        "0.303150275147523568675862817372",
        "0x0.4d9b41a5883227a6b896ca36",
        Greater,
    );
    test(
        "2/3",
        100,
        Down,
        "0.30315027514752356867586281737163",
        "0x0.4d9b41a5883227a6b896ca35f8",
        Less,
    );
    test(
        "2/3",
        100,
        Up,
        "0.303150275147523568675862817372",
        "0x0.4d9b41a5883227a6b896ca36",
        Greater,
    );
    test(
        "2/3",
        100,
        Nearest,
        "0.303150275147523568675862817372",
        "0x0.4d9b41a5883227a6b896ca36",
        Greater,
    );
    test("-1/2", 100, Floor, "NaN", "NaN", Equal);
    test("-1/2", 100, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 100, Down, "NaN", "NaN", Equal);
    test("-1/2", 100, Up, "NaN", "NaN", Equal);
    test("-1/2", 100, Nearest, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn ln_gamma_prec_round_fail_1() {
    BigExponentFloat::ONE.ln_gamma_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_gamma_prec_round_fail_2() {
    BigExponentFloat::from(0.5).ln_gamma_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_round_fail_1() {
    BigExponentFloat::ln_gamma_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_round_fail_2() {
    BigExponentFloat::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn ln_gamma_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = ln_gamma_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if *x > 0u32 {
        // The function is negative between its zeros 1 and 2, and positive on the rest of the
        // positive reals; a result next to a zero may underflow.
        if *x > 1u32 && *x < 2u32 {
            assert!(r.0 <= 0u32);
        } else {
            assert!(r.0 >= 0u32);
        }
    } else if *x < 0u32 {
        assert!(r.0.is_nan());
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = ln_gamma_prec_round_all(x, prec, Floor);
        let ceiling = ln_gamma_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&ln_gamma_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = ln_gamma_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(
            &ln_gamma_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn ln_gamma_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            ln_gamma_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            ln_gamma_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn ln_gamma_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.ln_gamma_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = ln_gamma_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn ln_gamma_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = ln_gamma_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if x > 0u32 {
            // The function is negative between its zeros 1 and 2, and positive on the rest of the
            // positive reals; a result next to a zero may underflow.
            if x > 1u32 && x < 2u32 {
                assert!(r.0 <= 0u32);
            } else {
                assert!(r.0 >= 0u32);
            }
        } else if x < 0u32 {
            assert!(r.0.is_nan());
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if rm != Nearest {
                let (y, o) = ln_gamma_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn ln_gamma_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::ln_gamma_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = ln_gamma_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

fn same_triple(
    x: &(BigExponentFloat, bool, Ordering),
    y: &(BigExponentFloat, bool, Ordering),
) -> bool {
    same(&x.0, &y.0) && x.1 == y.1 && x.2 == y.2
}

// The functions enclose the logarithm of the gamma function of a positive number with single values
// and error bounds. Computing with enclosures throughout, as in Azurite, gives the same results.
#[test]
fn ln_gamma_prec_round_enclosures_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            if rm == Exact && ln_gamma_prec_round_enclosures(true, &x, prec, Floor).2 != Equal {
                return;
            }
            let (y, o) = x.ln_gamma_prec_round_ref(prec, rm);
            let r = ln_gamma_prec_round_enclosures(false, &x, prec, rm);
            assert!(same_triple(&r, &(y, r.1, o)));
            assert!(same_triple(
                &ln_gamma_prec_round_enclosures(true, &x, prec, rm),
                &x.ln_abs_gamma_prec_round_ref(prec, rm)
            ));
        },
    );

    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact && ln_gamma_rational_prec_round_enclosures(true, &x, prec, Floor).2 != Equal
        {
            return;
        }
        let (y, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(&x, prec, rm);
        let r = ln_gamma_rational_prec_round_enclosures(false, &x, prec, rm);
        assert!(same_triple(&r, &(y, r.1, o)));
        assert!(same_triple(
            &ln_gamma_rational_prec_round_enclosures(true, &x, prec, rm),
            &BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(&x, prec, rm)
        ));
    });
}
