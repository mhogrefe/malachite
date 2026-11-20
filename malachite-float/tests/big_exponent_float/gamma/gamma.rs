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

fn gamma_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().gamma_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.gamma_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.gamma_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().gamma_prec(prec), &r));
        assert!(same_pair(&x.gamma_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.gamma_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn gamma_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::gamma_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::gamma_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::gamma_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::gamma_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_gamma_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &gamma_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        // The gamma function has poles at 0 and at the negative integers.
        assert!(same_pair(
            &gamma_prec_round_all(&zero, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_rational_prec_round_all(&Rational::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_prec_round_all(&BigExponentFloat::ONE, 10, rm),
            &(BigExponentFloat::ONE.clone(), Equal)
        ));
        assert!(same_pair(
            &gamma_rational_prec_round_all(&Rational::TWO, 10, rm),
            &(BigExponentFloat::ONE.clone(), Equal)
        ));
    }
    // The expected values agree with Azurite's gamma function.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = gamma_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 10, Floor, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Down, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Up, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Nearest, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Exact, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 10, Floor, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 10, Down, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 10, Up, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 10, Nearest, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 10, Exact, "1.0", "0x1.0", Equal);
    test("3.0", "0x3.0", 10, Floor, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 10, Down, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 10, Up, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 10, Nearest, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 10, Exact, "2.0", "0x2.0", Equal);
    test("4.0", "0x4.0", 10, Floor, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 10, Ceiling, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 10, Down, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 10, Up, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 10, Nearest, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 10, Exact, "6.0", "0x6.0", Equal);
    test("5.0", "0x5.0", 10, Floor, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 10, Ceiling, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 10, Down, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 10, Up, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 10, Nearest, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 10, Exact, "24.0", "0x18.0", Equal);
    test("10.0", "0xa.0", 10, Floor, "3.625e5", "0x5.88E+4", Less);
    test(
        "10.0",
        "0xa.0",
        10,
        Ceiling,
        "3.6301e5",
        "0x5.8aE+4",
        Greater,
    );
    test("10.0", "0xa.0", 10, Down, "3.625e5", "0x5.88E+4", Less);
    test("10.0", "0xa.0", 10, Up, "3.6301e5", "0x5.8aE+4", Greater);
    test(
        "10.0",
        "0xa.0",
        10,
        Nearest,
        "3.6301e5",
        "0x5.8aE+4",
        Greater,
    );
    test("25.0", "0x19.0", 10, Floor, "6.1981e23", "0x8.34E+19", Less);
    test(
        "25.0",
        "0x19.0",
        10,
        Ceiling,
        "6.210e23",
        "0x8.38E+19",
        Greater,
    );
    test("25.0", "0x19.0", 10, Down, "6.1981e23", "0x8.34E+19", Less);
    test("25.0", "0x19.0", 10, Up, "6.210e23", "0x8.38E+19", Greater);
    test(
        "25.0",
        "0x19.0",
        10,
        Nearest,
        "6.210e23",
        "0x8.38E+19",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Floor,
        "7.2527e306",
        "0xa.54E+254",
        Less,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Ceiling,
        "7.264e306",
        "0xa.58E+254",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Down,
        "7.2527e306",
        "0xa.54E+254",
        Less,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Up,
        "7.264e306",
        "0xa.58E+254",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Nearest,
        "7.2527e306",
        "0xa.54E+254",
        Less,
    );
    test("0.50", "0x0.8", 10, Floor, "1.7715", "0x1.c58", Less);
    test("0.50", "0x0.8", 10, Ceiling, "1.773", "0x1.c6", Greater);
    test("0.50", "0x0.8", 10, Down, "1.7715", "0x1.c58", Less);
    test("0.50", "0x0.8", 10, Up, "1.773", "0x1.c6", Greater);
    test("0.50", "0x0.8", 10, Nearest, "1.7715", "0x1.c58", Less);
    test("1.5", "0x1.8", 10, Floor, "0.88574", "0x0.e2c", Less);
    test("1.5", "0x1.8", 10, Ceiling, "0.8867", "0x0.e3", Greater);
    test("1.5", "0x1.8", 10, Down, "0.88574", "0x0.e2c", Less);
    test("1.5", "0x1.8", 10, Up, "0.8867", "0x0.e3", Greater);
    test("1.5", "0x1.8", 10, Nearest, "0.88574", "0x0.e2c", Less);
    test("0.25", "0x0.4", 10, Floor, "3.62", "0x3.a", Less);
    test("0.25", "0x0.4", 10, Ceiling, "3.6289", "0x3.a1", Greater);
    test("0.25", "0x0.4", 10, Down, "3.62", "0x3.a", Less);
    test("0.25", "0x0.4", 10, Up, "3.6289", "0x3.a1", Greater);
    test("0.25", "0x0.4", 10, Nearest, "3.62", "0x3.a", Less);
    test("0.75", "0x0.c", 10, Floor, "1.2246", "0x1.398", Less);
    test("0.75", "0x0.c", 10, Ceiling, "1.227", "0x1.3a", Greater);
    test("0.75", "0x0.c", 10, Down, "1.2246", "0x1.398", Less);
    test("0.75", "0x0.c", 10, Up, "1.227", "0x1.3a", Greater);
    test("0.75", "0x0.c", 10, Nearest, "1.2246", "0x1.398", Less);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Floor,
        "9.50",
        "0x9.8",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Ceiling,
        "9.5156",
        "0x9.84",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Down,
        "9.50",
        "0x9.8",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Up,
        "9.5156",
        "0x9.84",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Nearest,
        "9.5156",
        "0x9.84",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Floor,
        "2.680",
        "0x2.ae",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Ceiling,
        "2.6836",
        "0x2.af",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Down,
        "2.680",
        "0x2.ae",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Up,
        "2.6836",
        "0x2.af",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Nearest,
        "2.6836",
        "0x2.af",
        Greater,
    );
    test("-0.50", "-0x0.8", 10, Floor, "-3.547", "-0x3.8c", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-3.5430", "-0x3.8b", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "-3.5430", "-0x3.8b", Greater);
    test("-0.50", "-0x0.8", 10, Up, "-3.547", "-0x3.8c", Less);
    test(
        "-0.50", "-0x0.8", 10, Nearest, "-3.5430", "-0x3.8b", Greater,
    );
    test("-1.5", "-0x1.8", 10, Floor, "2.359", "0x2.5c", Less);
    test("-1.5", "-0x1.8", 10, Ceiling, "2.3633", "0x2.5d", Greater);
    test("-1.5", "-0x1.8", 10, Down, "2.359", "0x2.5c", Less);
    test("-1.5", "-0x1.8", 10, Up, "2.3633", "0x2.5d", Greater);
    test("-1.5", "-0x1.8", 10, Nearest, "2.3633", "0x2.5d", Greater);
    test("-2.5", "-0x2.8", 10, Floor, "-0.9453", "-0x0.f2", Less);
    test(
        "-2.5", "-0x2.8", 10, Ceiling, "-0.94434", "-0x0.f1c", Greater,
    );
    test("-2.5", "-0x2.8", 10, Down, "-0.94434", "-0x0.f1c", Greater);
    test("-2.5", "-0x2.8", 10, Up, "-0.9453", "-0x0.f2", Less);
    test("-2.5", "-0x2.8", 10, Nearest, "-0.9453", "-0x0.f2", Less);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "-10.69",
        "-0xa.b",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "-10.672",
        "-0xa.ac",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "-10.672",
        "-0xa.ac",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "-10.69",
        "-0xa.b",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "-10.69",
        "-0xa.b",
        Less,
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
    test(
        "-100.5",
        "-0x64.8",
        10,
        Floor,
        "-3.3564e-159",
        "-0x2.f3E-132",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Ceiling,
        "-3.352e-159",
        "-0x2.f2E-132",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Down,
        "-3.352e-159",
        "-0x2.f2E-132",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Up,
        "-3.3564e-159",
        "-0x2.f3E-132",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Nearest,
        "-3.352e-159",
        "-0x2.f2E-132",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Floor,
        "-5.0053e6",
        "-0x4.c6E+5",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Ceiling,
        "-4.997e6",
        "-0x4.c4E+5",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Down,
        "-4.997e6",
        "-0x4.c4E+5",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Up,
        "-5.0053e6",
        "-0x4.c6E+5",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Nearest,
        "-4.997e6",
        "-0x4.c4E+5",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Floor,
        "2.2852",
        "0x2.49",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Ceiling,
        "2.289",
        "0x2.4a",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Down,
        "2.2852",
        "0x2.49",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Up,
        "2.289",
        "0x2.4a",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Nearest,
        "2.289",
        "0x2.4a",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Floor,
        "2.6758",
        "0x2.ad",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Ceiling,
        "2.680",
        "0x2.ae",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Down,
        "2.6758",
        "0x2.ad",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Up,
        "2.680",
        "0x2.ae",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Nearest,
        "2.680",
        "0x2.ae",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Floor,
        "-1.3066",
        "-0x1.4e8",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Ceiling,
        "-1.305",
        "-0x1.4e",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Down,
        "-1.305",
        "-0x1.4e",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Up,
        "-1.3066",
        "-0x1.4e8",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Nearest,
        "-1.3066",
        "-0x1.4e8",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Floor,
        "9.999e9",
        "0x2.54E+8",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Ceiling,
        "1.0016e10",
        "0x2.55E+8",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Down,
        "9.999e9",
        "0x2.54E+8",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Up,
        "1.0016e10",
        "0x2.55E+8",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Nearest,
        "9.999e9",
        "0x2.54E+8",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Floor,
        "-1.0016e10",
        "-0x2.55E+8",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Ceiling,
        "-9.999e9",
        "-0x2.54E+8",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Down,
        "-9.999e9",
        "-0x2.54E+8",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Up,
        "-1.0016e10",
        "-0x2.55E+8",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Nearest,
        "-9.999e9",
        "-0x2.54E+8",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Floor,
        "9.993e299",
        "0x1.7eE+249",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Ceiling,
        "1.0006e300",
        "0x1.7e8E+249",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Down,
        "9.993e299",
        "0x1.7eE+249",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Up,
        "1.0006e300",
        "0x1.7e8E+249",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Nearest,
        "1.0006e300",
        "0x1.7e8E+249",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Floor,
        "-1.0006e300",
        "-0x1.7e8E+249",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Ceiling,
        "-9.993e299",
        "-0x1.7eE+249",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Down,
        "-9.993e299",
        "-0x1.7eE+249",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Up,
        "-1.0006e300",
        "-0x1.7e8E+249",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Nearest,
        "-1.0006e300",
        "-0x1.7e8E+249",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Floor,
        "1.271e2566",
        "0x1.55E+2131",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Ceiling,
        "1.2730e2566",
        "0x1.558E+2131",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Down,
        "1.271e2566",
        "0x1.55E+2131",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Up,
        "1.2730e2566",
        "0x1.558E+2131",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Nearest,
        "1.2730e2566",
        "0x1.558E+2131",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Floor,
        "2.8240e456568",
        "0xb.d4E+379171",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Ceiling,
        "2.828e456568",
        "0xb.d8E+379171",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Down,
        "2.8240e456568",
        "0xb.d4E+379171",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Up,
        "2.828e456568",
        "0xb.d8E+379171",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Nearest,
        "2.8240e456568",
        "0xb.d4E+379171",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Floor,
        "8.2589e5565705",
        "0x7.3aE+4622218",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Ceiling,
        "8.268e5565705",
        "0x7.3cE+4622218",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Down,
        "8.2589e5565705",
        "0x7.3aE+4622218",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Up,
        "8.268e5565705",
        "0x7.3cE+4622218",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Nearest,
        "8.268e5565705",
        "0x7.3cE+4622218",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Floor,
        "-3.806e-5565712",
        "-0x7.4cE-4622224",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Ceiling,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Down,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Up,
        "-3.806e-5565712",
        "-0x7.4cE-4622224",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Nearest,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Floor,
        "1.4970e496101294",
        "0x8.2cE+412003206",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Ceiling,
        "1.500e496101294",
        "0x8.3E+412003206",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Down,
        "1.4970e496101294",
        "0x8.2cE+412003206",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Up,
        "1.500e496101294",
        "0x8.3E+412003206",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Nearest,
        "1.500e496101294",
        "0x8.3E+412003206",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Floor,
        "1.2264e496101298",
        "0x1.058E+412003210",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Ceiling,
        "1.229e496101298",
        "0x1.06E+412003210",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Down,
        "1.2264e496101298",
        "0x1.058E+412003210",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Up,
        "1.229e496101298",
        "0x1.06E+412003210",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Nearest,
        "1.229e496101298",
        "0x1.06E+412003210",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Floor,
        "8.0494e330985980541",
        "0xf.fcE+274877906943",
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Ceiling,
        "8.1e330985980541",
        "0x1.0E+274877906944",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Down,
        "8.0494e330985980541",
        "0xf.fcE+274877906943",
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Up,
        "8.1e330985980541",
        "0x1.0E+274877906944",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Nearest,
        "8.1e330985980541",
        "0x1.0E+274877906944",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Floor,
        "-8.0730e330985980541",
        "-0x1.008E+274877906944",
        Less,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Ceiling,
        "-8.1e330985980541",
        "-0x1.0E+274877906944",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Down,
        "-8.1e330985980541",
        "-0x1.0E+274877906944",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Up,
        "-8.0730e330985980541",
        "-0x1.008E+274877906944",
        Less,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Nearest,
        "-8.1e330985980541",
        "-0x1.0E+274877906944",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "6.763e37609551808354240533484732605463",
        "0x4.8cE+31234056697073438931807029430640",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "6.7746e37609551808354240533484732605463",
        "0x4.8eE+31234056697073438931807029430640",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Down,
        "6.763e37609551808354240533484732605463",
        "0x4.8cE+31234056697073438931807029430640",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Up,
        "6.7746e37609551808354240533484732605463",
        "0x4.8eE+31234056697073438931807029430640",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Nearest,
        "6.7746e37609551808354240533484732605463",
        "0x4.8eE+31234056697073438931807029430640",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Floor,
        "7.614e37609551808354240533484732605478",
        "0x1.23E+31234056697073438931807029430653",
        Less,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Ceiling,
        "7.6275e37609551808354240533484732605478",
        "0x1.238E+31234056697073438931807029430653",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Down,
        "7.614e37609551808354240533484732605478",
        "0x1.23E+31234056697073438931807029430653",
        Less,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Up,
        "7.6275e37609551808354240533484732605478",
        "0x1.238E+31234056697073438931807029430653",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Nearest,
        "7.6275e37609551808354240533484732605478",
        "0x1.238E+31234056697073438931807029430653",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Floor,
        "-3.2523e-37609551808354240533484732605509",
        "-0x2.c3E-31234056697073438931807029430678",
        Less,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Ceiling,
        "-3.248e-37609551808354240533484732605509",
        "-0x2.c2E-31234056697073438931807029430678",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Down,
        "-3.248e-37609551808354240533484732605509",
        "-0x2.c2E-31234056697073438931807029430678",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Up,
        "-3.2523e-37609551808354240533484732605509",
        "-0x2.c3E-31234056697073438931807029430678",
        Less,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Nearest,
        "-3.2523e-37609551808354240533484732605509",
        "-0x2.c3E-31234056697073438931807029430678",
        Less,
    );
    test("1.0", "0x1.0", 1, Floor, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Floor, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 1, Ceiling, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Ceiling, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 1, Down, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Down, "1.0", "0x1.0", Equal);
    // - gamma_helper small positive integer
    // - gamma_of_small_integer factorial
    test("1.0", "0x1.0", 1, Up, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Up, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 1, Nearest, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Nearest, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 1, Floor, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 100, Floor, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 1, Ceiling, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 100, Ceiling, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 1, Down, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 100, Down, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 1, Up, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 100, Up, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 1, Nearest, "1.0", "0x1.0", Equal);
    test("2.0", "0x2.0", 100, Nearest, "1.0", "0x1.0", Equal);
    test("3.0", "0x3.0", 1, Floor, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 100, Floor, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 1, Ceiling, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 100, Ceiling, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 1, Down, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 100, Down, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 1, Up, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 100, Up, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 1, Nearest, "2.0", "0x2.0", Equal);
    test("3.0", "0x3.0", 100, Nearest, "2.0", "0x2.0", Equal);
    test("4.0", "0x4.0", 1, Floor, "4.0", "0x4.0", Less);
    test("4.0", "0x4.0", 100, Floor, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 1, Ceiling, "8.0", "0x8.0", Greater);
    test("4.0", "0x4.0", 100, Ceiling, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 1, Down, "4.0", "0x4.0", Less);
    test("4.0", "0x4.0", 100, Down, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 1, Up, "8.0", "0x8.0", Greater);
    test("4.0", "0x4.0", 100, Up, "6.0", "0x6.0", Equal);
    test("4.0", "0x4.0", 1, Nearest, "8.0", "0x8.0", Greater);
    test("4.0", "0x4.0", 100, Nearest, "6.0", "0x6.0", Equal);
    test("5.0", "0x5.0", 1, Floor, "16.0", "0x1.0E+1", Less);
    test("5.0", "0x5.0", 100, Floor, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 1, Ceiling, "32.0", "0x2.0E+1", Greater);
    test("5.0", "0x5.0", 100, Ceiling, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 1, Down, "16.0", "0x1.0E+1", Less);
    test("5.0", "0x5.0", 100, Down, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 1, Up, "32.0", "0x2.0E+1", Greater);
    test("5.0", "0x5.0", 100, Up, "24.0", "0x18.0", Equal);
    test("5.0", "0x5.0", 1, Nearest, "32.0", "0x2.0E+1", Greater);
    test("5.0", "0x5.0", 100, Nearest, "24.0", "0x18.0", Equal);
    test("10.0", "0xa.0", 1, Floor, "2.6e5", "0x4.0E+4", Less);
    test("10.0", "0xa.0", 100, Floor, "3.6288e5", "0x5.898E+4", Equal);
    test("10.0", "0xa.0", 1, Ceiling, "5.2e5", "0x8.0E+4", Greater);
    test(
        "10.0",
        "0xa.0",
        100,
        Ceiling,
        "3.6288e5",
        "0x5.898E+4",
        Equal,
    );
    test("10.0", "0xa.0", 1, Down, "2.6e5", "0x4.0E+4", Less);
    test("10.0", "0xa.0", 100, Down, "3.6288e5", "0x5.898E+4", Equal);
    test("10.0", "0xa.0", 1, Up, "5.2e5", "0x8.0E+4", Greater);
    test("10.0", "0xa.0", 100, Up, "3.6288e5", "0x5.898E+4", Equal);
    test("10.0", "0xa.0", 1, Nearest, "2.6e5", "0x4.0E+4", Less);
    test(
        "10.0",
        "0xa.0",
        100,
        Nearest,
        "3.6288e5",
        "0x5.898E+4",
        Equal,
    );
    test("25.0", "0x19.0", 1, Floor, "6.0e23", "0x8.0E+19", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Floor,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test("25.0", "0x19.0", 1, Ceiling, "1.2e24", "0x1.0E+20", Greater);
    test(
        "25.0",
        "0x19.0",
        100,
        Ceiling,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test("25.0", "0x19.0", 1, Down, "6.0e23", "0x8.0E+19", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Down,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test("25.0", "0x19.0", 1, Up, "1.2e24", "0x1.0E+20", Greater);
    test(
        "25.0",
        "0x19.0",
        100,
        Up,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test("25.0", "0x19.0", 1, Nearest, "6.0e23", "0x8.0E+19", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Nearest,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test("171.0", "0xab.0", 1, Floor, "5.6e306", "0x8.0E+254", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Floor,
        "7.2574156153079989673967282111251e306",
        "0xa.55bc3220c31c79101dd27c41E+254",
        Less,
    );
    test(
        "171.0",
        "0xab.0",
        1,
        Ceiling,
        "1.1e307",
        "0x1.0E+255",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        100,
        Ceiling,
        "7.257415615307998967396728211134e306",
        "0xa.55bc3220c31c79101dd27c42E+254",
        Greater,
    );
    test("171.0", "0xab.0", 1, Down, "5.6e306", "0x8.0E+254", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Down,
        "7.2574156153079989673967282111251e306",
        "0xa.55bc3220c31c79101dd27c41E+254",
        Less,
    );
    test("171.0", "0xab.0", 1, Up, "1.1e307", "0x1.0E+255", Greater);
    test(
        "171.0",
        "0xab.0",
        100,
        Up,
        "7.257415615307998967396728211134e306",
        "0xa.55bc3220c31c79101dd27c42E+254",
        Greater,
    );
    test("171.0", "0xab.0", 1, Nearest, "5.6e306", "0x8.0E+254", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Nearest,
        "7.2574156153079989673967282111251e306",
        "0xa.55bc3220c31c79101dd27c41E+254",
        Less,
    );
    test("0.50", "0x0.8", 1, Floor, "1.0", "0x1.0", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Floor,
        "1.772453850905516027298167483340",
        "0x1.c5bf891b4ef6aa79c3b0520d4",
        Less,
    );
    test("0.50", "0x0.8", 1, Ceiling, "2.0", "0x2.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Ceiling,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6",
        Greater,
    );
    test("0.50", "0x0.8", 1, Down, "1.0", "0x1.0", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Down,
        "1.772453850905516027298167483340",
        "0x1.c5bf891b4ef6aa79c3b0520d4",
        Less,
    );
    test("0.50", "0x0.8", 1, Up, "2.0", "0x2.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Up,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "2.0", "0x2.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6",
        Greater,
    );
    test("1.5", "0x1.8", 1, Floor, "0.50", "0x0.8", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Floor,
        "0.8862269254527580136490837416698",
        "0x0.e2dfc48da77b553ce1d82906a",
        Less,
    );
    test("1.5", "0x1.8", 1, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Ceiling,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b",
        Greater,
    );
    test("1.5", "0x1.8", 1, Down, "0.50", "0x0.8", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Down,
        "0.8862269254527580136490837416698",
        "0x0.e2dfc48da77b553ce1d82906a",
        Less,
    );
    test("1.5", "0x1.8", 1, Up, "1.0", "0x1.0", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Up,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b",
        Greater,
    );
    test("1.5", "0x1.8", 1, Nearest, "1.0", "0x1.0", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Nearest,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b",
        Greater,
    );
    test("0.25", "0x0.4", 1, Floor, "2.0", "0x2.0", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Floor,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514",
        Less,
    );
    test("0.25", "0x0.4", 1, Ceiling, "4.0", "0x4.0", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Ceiling,
        "3.625609908221908311930685155870",
        "0x3.a027f88fdddd479b38a50a518",
        Greater,
    );
    test("0.25", "0x0.4", 1, Down, "2.0", "0x2.0", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Down,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514",
        Less,
    );
    test("0.25", "0x0.4", 1, Up, "4.0", "0x4.0", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Up,
        "3.625609908221908311930685155870",
        "0x3.a027f88fdddd479b38a50a518",
        Greater,
    );
    test("0.25", "0x0.4", 1, Nearest, "4.0", "0x4.0", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Nearest,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514",
        Less,
    );
    test("0.75", "0x0.c", 1, Floor, "1.0", "0x1.0", Less);
    test(
        "0.75",
        "0x0.c",
        100,
        Floor,
        "1.2254167024651776451290983033620",
        "0x1.39b4e8b50f62c027af524acd2",
        Less,
    );
    test("0.75", "0x0.c", 1, Ceiling, "2.0", "0x2.0", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Ceiling,
        "1.225416702465177645129098303364",
        "0x1.39b4e8b50f62c027af524acd4",
        Greater,
    );
    test("0.75", "0x0.c", 1, Down, "1.0", "0x1.0", Less);
    test(
        "0.75",
        "0x0.c",
        100,
        Down,
        "1.2254167024651776451290983033620",
        "0x1.39b4e8b50f62c027af524acd2",
        Less,
    );
    test("0.75", "0x0.c", 1, Up, "2.0", "0x2.0", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Up,
        "1.225416702465177645129098303364",
        "0x1.39b4e8b50f62c027af524acd4",
        Greater,
    );
    test("0.75", "0x0.c", 1, Nearest, "1.0", "0x1.0", Less);
    // - gamma_core positive
    // - gamma_helper loop for positive x
    // - gamma_ln_bits e = 0
    // - ln_gamma_shift e <= 0
    // - stirling_mu_encl term below tolerance
    test(
        "0.75",
        "0x0.c",
        100,
        Nearest,
        "1.225416702465177645129098303364",
        "0x1.39b4e8b50f62c027af524acd4",
        Greater,
    );
    // - gamma_approx positive value
    // - gamma_core negative with e > 0
    // - gamma_exp_encl not negated
    // - gamma_helper loop for negative x
    // - gamma_ln_bits e > 0
    // - ln_gamma_bracket_encl no shift
    // - ln_gamma_bracket_encl shift
    // - ln_gamma_shift 0 < e <= L
    // - ln_gamma_shift e > L
    // - ln_of_encl exact argument
    // - ln_of_encl inexact argument
    // - stirling_mu_encl closed at even k
    // - stirling_mu_encl closed at odd k
    // - stirling_mu_encl out of terms
    // - ziv_loop_certified_from retry
    test(
        "-291.25",
        "-0x123.4",
        1390,
        Floor,
        "6.1253692587685811499330095984146547546855188539437128322117600611897921688206893249974414\
        5418442824942559472281729738060835608062367332899168866879182667573028270748500582918101256\
        1285519277652049216678500645808486481615163027254663574144656911405559333890351096732982185\
        6300929107117111224182469595090794541938047796595103169247463471000527693237254517873379951\
        9252006152916719767532317119000507529762853640055523059773e-593",
        "0x1.a32edbf1e1ab0ae23c0f56b90f3d4979ba7eba6f1c8879403cc330cf6fd954321b751a0c78da273b5e5961\
        aa5ca06364d3cb05736f0acfa101a67cd3f907955d048ad571c9d3dd4055bec9916e7ba1efa80d961ebf5dd08c6\
        e4130cdd6e578cb3264f247b464d53d8580f30e06b9a232d9e20d2c8967e5cb9923dc3512704f7625324715685d\
        d43f70673a96b09141e4d8bbcf1bb97a03535d7692ca21426a60ae3d7968ae9f2b2163499775c39E-492",
        Less,
    );
    // - gamma_approx negative value
    // - gamma_core negative with e <= 0
    // - gamma_helper tiny argument
    test(
        "-1.033975765691284593589260866745e-25",
        "-0x2.0000000000000000000000feE-21",
        1,
        Up,
        "-1.9e25",
        "-0x1.0E+21",
        Less,
    );
    // - gamma_helper tiny test fails
    // - gamma_ln_bits e < 0
    test(
        "3.454467422037777850154540745120159828446400145756e-77",
        "0x3.ffffffffffffffffffffffffffffffffffffffeE-64",
        162,
        Floor,
        "2.8948022309329048855892746252171976963317496166559e76",
        "0x4.000000000000000000000000000000000000001eE+63",
        Less,
    );
    // - gamma_helper negative integer
    test("-1.0", "-0x1.0", 1, Up, "NaN", "NaN", Equal);
    // - gamma_helper large positive integer
    test(
        "3348.0",
        "0xd14.0",
        1,
        Up,
        "6.3e10345",
        "0x1.0E+8592",
        Greater,
    );
    // - gamma_helper tiny test succeeds
    test(
        "3.2e-27",
        "0x1.0E-22",
        1,
        Up,
        "3.1e26",
        "0x1.0E+22",
        Greater,
    );
}

#[test]
fn test_gamma_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = gamma_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Floor, "NaN", "NaN", Equal);
    test("0", 10, Ceiling, "NaN", "NaN", Equal);
    test("0", 10, Down, "NaN", "NaN", Equal);
    test("0", 10, Up, "NaN", "NaN", Equal);
    test("0", 10, Nearest, "NaN", "NaN", Equal);
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1", 10, Floor, "1.0", "0x1.0", Equal);
    test("1", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("1", 10, Down, "1.0", "0x1.0", Equal);
    test("1", 10, Up, "1.0", "0x1.0", Equal);
    test("1", 10, Nearest, "1.0", "0x1.0", Equal);
    test("1", 10, Exact, "1.0", "0x1.0", Equal);
    test("2", 10, Floor, "1.0", "0x1.0", Equal);
    test("2", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("2", 10, Down, "1.0", "0x1.0", Equal);
    test("2", 10, Up, "1.0", "0x1.0", Equal);
    test("2", 10, Nearest, "1.0", "0x1.0", Equal);
    test("2", 10, Exact, "1.0", "0x1.0", Equal);
    test("3", 10, Floor, "2.0", "0x2.0", Equal);
    test("3", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("3", 10, Down, "2.0", "0x2.0", Equal);
    test("3", 10, Up, "2.0", "0x2.0", Equal);
    test("3", 10, Nearest, "2.0", "0x2.0", Equal);
    test("3", 10, Exact, "2.0", "0x2.0", Equal);
    test("5", 10, Floor, "24.0", "0x18.0", Equal);
    test("5", 10, Ceiling, "24.0", "0x18.0", Equal);
    test("5", 10, Down, "24.0", "0x18.0", Equal);
    test("5", 10, Up, "24.0", "0x18.0", Equal);
    test("5", 10, Nearest, "24.0", "0x18.0", Equal);
    test("5", 10, Exact, "24.0", "0x18.0", Equal);
    test("25", 10, Floor, "6.1981e23", "0x8.34E+19", Less);
    test("25", 10, Ceiling, "6.210e23", "0x8.38E+19", Greater);
    test("25", 10, Down, "6.1981e23", "0x8.34E+19", Less);
    test("25", 10, Up, "6.210e23", "0x8.38E+19", Greater);
    test("25", 10, Nearest, "6.210e23", "0x8.38E+19", Greater);
    test("1/2", 10, Floor, "1.7715", "0x1.c58", Less);
    test("1/2", 10, Ceiling, "1.773", "0x1.c6", Greater);
    test("1/2", 10, Down, "1.7715", "0x1.c58", Less);
    test("1/2", 10, Up, "1.773", "0x1.c6", Greater);
    test("1/2", 10, Nearest, "1.7715", "0x1.c58", Less);
    test("1/3", 10, Floor, "2.6758", "0x2.ad", Less);
    test("1/3", 10, Ceiling, "2.680", "0x2.ae", Greater);
    test("1/3", 10, Down, "2.6758", "0x2.ad", Less);
    test("1/3", 10, Up, "2.680", "0x2.ae", Greater);
    test("1/3", 10, Nearest, "2.680", "0x2.ae", Greater);
    test("1/4", 10, Floor, "3.62", "0x3.a", Less);
    test("1/4", 10, Ceiling, "3.6289", "0x3.a1", Greater);
    test("1/4", 10, Down, "3.62", "0x3.a", Less);
    test("1/4", 10, Up, "3.6289", "0x3.a1", Greater);
    test("1/4", 10, Nearest, "3.62", "0x3.a", Less);
    test("1/6", 10, Floor, "5.562", "0x5.9", Less);
    test("1/6", 10, Ceiling, "5.5703", "0x5.92", Greater);
    test("1/6", 10, Down, "5.562", "0x5.9", Less);
    test("1/6", 10, Up, "5.5703", "0x5.92", Greater);
    test("1/6", 10, Nearest, "5.562", "0x5.9", Less);
    test("2/3", 10, Floor, "1.3535", "0x1.5a8", Less);
    test("2/3", 10, Ceiling, "1.355", "0x1.5b", Greater);
    test("2/3", 10, Down, "1.3535", "0x1.5a8", Less);
    test("2/3", 10, Up, "1.355", "0x1.5b", Greater);
    test("2/3", 10, Nearest, "1.3535", "0x1.5a8", Less);
    test("-1/2", 10, Floor, "-3.547", "-0x3.8c", Less);
    test("-1/2", 10, Ceiling, "-3.5430", "-0x3.8b", Greater);
    test("-1/2", 10, Down, "-3.5430", "-0x3.8b", Greater);
    test("-1/2", 10, Up, "-3.547", "-0x3.8c", Less);
    test("-1/2", 10, Nearest, "-3.5430", "-0x3.8b", Greater);
    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    // - gamma_rational_helper pole
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Exact, "NaN", "NaN", Equal);
    test("-2", 10, Floor, "NaN", "NaN", Equal);
    test("-2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2", 10, Down, "NaN", "NaN", Equal);
    test("-2", 10, Up, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Exact, "NaN", "NaN", Equal);
    test("-5/2", 10, Floor, "-0.9453", "-0x0.f2", Less);
    test("-5/2", 10, Ceiling, "-0.94434", "-0x0.f1c", Greater);
    test("-5/2", 10, Down, "-0.94434", "-0x0.f1c", Greater);
    test("-5/2", 10, Up, "-0.9453", "-0x0.f2", Less);
    test("-5/2", 10, Nearest, "-0.9453", "-0x0.f2", Less);
    test("-7/3", 10, Floor, "-1.3066", "-0x1.4e8", Less);
    test("-7/3", 10, Ceiling, "-1.305", "-0x1.4e", Greater);
    test("-7/3", 10, Down, "-1.305", "-0x1.4e", Greater);
    test("-7/3", 10, Up, "-1.3066", "-0x1.4e8", Less);
    test("-7/3", 10, Nearest, "-1.3066", "-0x1.4e8", Less);
    test("22/7", 10, Floor, "2.289", "0x2.4a", Less);
    test("22/7", 10, Ceiling, "2.2930", "0x2.4b", Greater);
    test("22/7", 10, Down, "2.289", "0x2.4a", Less);
    test("22/7", 10, Up, "2.2930", "0x2.4b", Greater);
    test("22/7", 10, Nearest, "2.289", "0x2.4a", Less);
    test(
        "1000001/1000",
        10,
        Floor,
        "4.0481e2564",
        "0xa.dcE+2129",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Ceiling,
        "4.054e2564",
        "0xa.eE+2129",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Down,
        "4.0481e2564",
        "0xa.dcE+2129",
        Less,
    );
    test("1000001/1000", 10, Up, "4.054e2564", "0xa.eE+2129", Greater);
    test(
        "1000001/1000",
        10,
        Nearest,
        "4.054e2564",
        "0xa.eE+2129",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Floor,
        "-2.469e-2565",
        "-0x1.79E-2130",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Ceiling,
        "-2.4658e-2565",
        "-0x1.788E-2130",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Down,
        "-2.4658e-2565",
        "-0x1.788E-2130",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Up,
        "-2.469e-2565",
        "-0x1.79E-2130",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Nearest,
        "-2.469e-2565",
        "-0x1.79E-2130",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "1.2664e30",
        "0xf.fcE+24",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.3e30",
        "0x1.0E+25",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "1.2664e30",
        "0xf.fcE+24",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "1.3e30",
        "0x1.0E+25",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "1.3e30",
        "0x1.0E+25",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Floor,
        "-1.2701e30",
        "-0x1.008E+25",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Ceiling,
        "-1.3e30",
        "-0x1.0E+25",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Down,
        "-1.3e30",
        "-0x1.0E+25",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Up,
        "-1.2701e30",
        "-0x1.008E+25",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Nearest,
        "-1.3e30",
        "-0x1.0E+25",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "100000000",
        10,
        Floor,
        "1.62e756570548",
        "0xe.8E+628318239",
        Less,
    );
    test(
        "100000000",
        10,
        Ceiling,
        "1.6178e756570548",
        "0xe.84E+628318239",
        Greater,
    );
    test(
        "100000000",
        10,
        Down,
        "1.62e756570548",
        "0xe.8E+628318239",
        Less,
    );
    test(
        "100000000",
        10,
        Up,
        "1.6178e756570548",
        "0xe.84E+628318239",
        Greater,
    );
    test(
        "100000000",
        10,
        Nearest,
        "1.6178e756570548",
        "0xe.84E+628318239",
        Greater,
    );
    test(
        "200000001/2",
        10,
        Floor,
        "1.615e756570552",
        "0x2.36E+628318243",
        Less,
    );
    test(
        "200000001/2",
        10,
        Ceiling,
        "1.6177e756570552",
        "0x2.37E+628318243",
        Greater,
    );
    test(
        "200000001/2",
        10,
        Down,
        "1.615e756570552",
        "0x2.36E+628318243",
        Less,
    );
    test(
        "200000001/2",
        10,
        Up,
        "1.6177e756570552",
        "0x2.37E+628318243",
        Greater,
    );
    test(
        "200000001/2",
        10,
        Nearest,
        "1.6177e756570552",
        "0x2.37E+628318243",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Floor,
        "-1.94e-756570560",
        "-0x3.dE-628318250",
        Less,
    );
    test(
        "-200000001/2",
        10,
        Ceiling,
        "-1.9425e-756570560",
        "-0x3.cfE-628318250",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Down,
        "-1.9425e-756570560",
        "-0x3.cfE-628318250",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Up,
        "-1.94e-756570560",
        "-0x3.dE-628318250",
        Less,
    );
    test(
        "-200000001/2",
        10,
        Nearest,
        "-1.9425e-756570560",
        "-0x3.cfE-628318250",
        Greater,
    );
    test(
        "-200000003/2",
        10,
        Floor,
        "1.942e-756570568",
        "0xa.38E-628318257",
        Less,
    );
    test(
        "-200000003/2",
        10,
        Ceiling,
        "1.9446e-756570568",
        "0xa.3cE-628318257",
        Greater,
    );
    test(
        "-200000003/2",
        10,
        Down,
        "1.942e-756570568",
        "0xa.38E-628318257",
        Less,
    );
    test(
        "-200000003/2",
        10,
        Up,
        "1.9446e-756570568",
        "0xa.3cE-628318257",
        Greater,
    );
    test(
        "-200000003/2",
        10,
        Nearest,
        "1.942e-756570568",
        "0xa.38E-628318257",
        Less,
    );
    test("0", 100, Floor, "NaN", "NaN", Equal);
    test("0", 100, Ceiling, "NaN", "NaN", Equal);
    test("0", 100, Down, "NaN", "NaN", Equal);
    test("0", 100, Up, "NaN", "NaN", Equal);
    test("0", 100, Nearest, "NaN", "NaN", Equal);
    test("1", 100, Floor, "1.0", "0x1.0", Equal);
    test("1", 100, Ceiling, "1.0", "0x1.0", Equal);
    test("1", 100, Down, "1.0", "0x1.0", Equal);
    test("1", 100, Up, "1.0", "0x1.0", Equal);
    test("1", 100, Nearest, "1.0", "0x1.0", Equal);
    test("2", 100, Floor, "1.0", "0x1.0", Equal);
    test("2", 100, Ceiling, "1.0", "0x1.0", Equal);
    test("2", 100, Down, "1.0", "0x1.0", Equal);
    test("2", 100, Up, "1.0", "0x1.0", Equal);
    test("2", 100, Nearest, "1.0", "0x1.0", Equal);
    test("3", 100, Floor, "2.0", "0x2.0", Equal);
    test("3", 100, Ceiling, "2.0", "0x2.0", Equal);
    test("3", 100, Down, "2.0", "0x2.0", Equal);
    test("3", 100, Up, "2.0", "0x2.0", Equal);
    test("3", 100, Nearest, "2.0", "0x2.0", Equal);
    test("5", 100, Floor, "24.0", "0x18.0", Equal);
    test("5", 100, Ceiling, "24.0", "0x18.0", Equal);
    test("5", 100, Down, "24.0", "0x18.0", Equal);
    test("5", 100, Up, "24.0", "0x18.0", Equal);
    test("5", 100, Nearest, "24.0", "0x18.0", Equal);
    test(
        "25",
        100,
        Floor,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test(
        "25",
        100,
        Ceiling,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test(
        "25",
        100,
        Down,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test(
        "25",
        100,
        Up,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test(
        "25",
        100,
        Nearest,
        "6.204484017332394394e23",
        "0x8.3629343d3dcd1cE+19",
        Equal,
    );
    test(
        "1/2",
        100,
        Floor,
        "1.772453850905516027298167483340",
        "0x1.c5bf891b4ef6aa79c3b0520d4",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "1.772453850905516027298167483340",
        "0x1.c5bf891b4ef6aa79c3b0520d4",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6",
        Greater,
    );
    test(
        "1/3",
        100,
        Floor,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "2.678938534707747633655692940977",
        "0x2.adceea72905e2ceec8d3e92d",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "2.678938534707747633655692940977",
        "0x2.adceea72905e2ceec8d3e92d",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc",
        Less,
    );
    test(
        "1/4",
        100,
        Floor,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514",
        Less,
    );
    test(
        "1/4",
        100,
        Ceiling,
        "3.625609908221908311930685155870",
        "0x3.a027f88fdddd479b38a50a518",
        Greater,
    );
    test(
        "1/4",
        100,
        Down,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514",
        Less,
    );
    test(
        "1/4",
        100,
        Up,
        "3.625609908221908311930685155870",
        "0x3.a027f88fdddd479b38a50a518",
        Greater,
    );
    test(
        "1/4",
        100,
        Nearest,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514",
        Less,
    );
    test(
        "1/6",
        100,
        Floor,
        "5.566316001780235204250096895207",
        "0x5.90fa15e2d8fb86a4c6b7ee8e",
        Less,
    );
    test(
        "1/6",
        100,
        Ceiling,
        "5.5663160017802352042500968952130",
        "0x5.90fa15e2d8fb86a4c6b7ee8e8",
        Greater,
    );
    test(
        "1/6",
        100,
        Down,
        "5.566316001780235204250096895207",
        "0x5.90fa15e2d8fb86a4c6b7ee8e",
        Less,
    );
    test(
        "1/6",
        100,
        Up,
        "5.5663160017802352042500968952130",
        "0x5.90fa15e2d8fb86a4c6b7ee8e8",
        Greater,
    );
    test(
        "1/6",
        100,
        Nearest,
        "5.566316001780235204250096895207",
        "0x5.90fa15e2d8fb86a4c6b7ee8e",
        Less,
    );
    test(
        "2/3",
        100,
        Floor,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702",
        Less,
    );
    test(
        "2/3",
        100,
        Ceiling,
        "1.354117939426400416945288028156",
        "0x1.5aa77928c3678cab2f4feb704",
        Greater,
    );
    test(
        "2/3",
        100,
        Down,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702",
        Less,
    );
    test(
        "2/3",
        100,
        Up,
        "1.354117939426400416945288028156",
        "0x1.5aa77928c3678cab2f4feb704",
        Greater,
    );
    test(
        "2/3",
        100,
        Nearest,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702",
        Less,
    );
    test(
        "-1/2",
        100,
        Floor,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac",
        Less,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-3.544907701811032054596334966679",
        "-0x3.8b7f12369ded54f38760a41a8",
        Greater,
    );
    test(
        "-1/2",
        100,
        Down,
        "-3.544907701811032054596334966679",
        "-0x3.8b7f12369ded54f38760a41a8",
        Greater,
    );
    test(
        "-1/2",
        100,
        Up,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac",
        Less,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac",
        Less,
    );
    // - gamma_approx positive value
    // - gamma_core negative with e > 0
    // - gamma_exp_encl not negated
    // - gamma_ln_bits e > 0
    // - gamma_rational_helper loop for negative q
    // - ln_gamma_bracket_encl no shift
    // - ln_gamma_bracket_encl shift
    // - ln_gamma_shift 0 < e <= L
    // - ln_gamma_shift e > L
    // - ln_of_encl exact argument
    // - ln_of_encl inexact argument
    // - stirling_mu_encl closed at even k
    // - stirling_mu_encl closed at odd k
    // - stirling_mu_encl out of terms
    // - ziv_loop_certified_from retry
    test(
        "-8589934591/16777216",
        3139,
        Floor,
        "4.8247984026053630848128095303163553578102969181452223566698422657619879860879631131282680\
        5662375266518617246080899164042899680018231187630218385548290376744395352730713901483302843\
        2839321543132906332582813969549855724011204896732135038819632374750564069534694447379391096\
        4871422286739224130291992265135114040088686976961925332874550399819943574994522362867785112\
        5459329239520020791471115426469143684090609767497807802668997095063603800684572981240510993\
        5937515612087375773966208543303303932470823315066362657299662046133670544342765330153317049\
        1054873969468281587825657081025855343167155174058403536428052448970482563661865169884804211\
        3523471239742658473630222517616736212716151020056675872020658116230671442129261924819136923\
        6427045078599785033597646074930623306780747253788210011740964130414520298269818437785577043\
        5922239315670600470247540560141789803908492675919364419678282194614732728594523577972963706\
        40322513453676728525313107253221398311e-1160",
        "0x1.c85004bdd61034f4004dd779192616eb1187e36e1ec6ef201dfb0613059b897fbeadf40486c455b4a7a8ae\
        4c34b96edf8fd35e8b72cff90c0d9ac29b5c7e2527bc8f5a6b647f0f69ff5d716ad9b8686141b8e1fc4ec505869\
        60dec9ad6d8c2d7b0ae4dd6281c93ca8b62b93bff62446281ad1888ba8ad7a025a3790c2fa1b9c0d9b836a79a44\
        35729f799b8498206851222e2602c51ebf95990fa88ab5e0f19cdee8dd231d45dc28aa3a98bb061d3f996ff8cb9\
        450b46dfb50a7dc6a552d6ca3153b687f800c39655b1013cec09a8911ac4cbf41f408b2af96409e1ce0f5c56b50\
        595f90b1c6c0cfaedf009879ad843d944a9b6a8bc1341a7ce88c6b608cd03aa30d6c240d4c4373b51ea82186f1b\
        e3fd053fe901e20ea8b37b864e43ca6058d7c2e87897ad65ebf8e8c03de6c815b26d97e770ee347ace01e086e2c\
        08af7e1a5b1dae8ce69093642dcf02f092559c95a7d7dc828e96e196cf716d276842c8c8a2e944d613e2a103ae2\
        808ce9abcc4f497db049ad05cd25d38c6fa75afe50aeb60e4b142855975d7E-963",
        Less,
    );
    // - gamma_core positive
    // - gamma_ln_bits e < 0
    // - gamma_rational_helper loop for positive q
    // - ln_gamma_shift e <= 0
    // - stirling_mu_encl term below tolerance
    test(
        "511/9007199254741056",
        2,
        Up,
        "2.6e13",
        "0x1.8E+11",
        Greater,
    );
    // - gamma_approx negative value
    // - gamma_core negative with e <= 0
    // - gamma_rational_helper tiny test fails
    test(
        "-32/116056878683004400771792895",
        2,
        Nearest,
        "-3.6e24",
        "-0x3.0E+20",
        Greater,
    );
    // - gamma_of_small_integer factorial
    // - gamma_rational_helper small positive integer
    test("1", 1, Up, "1.0", "0x1.0", Equal);
    // - gamma_ln_bits e = 0
    test("1/2", 1, Up, "2.0", "0x2.0", Greater);
    // - gamma_rational_helper large positive integer
    test("3348", 1, Up, "6.3e10345", "0x1.0E+8592", Greater);
    // - gamma_rational_helper tiny test succeeds
    test(
        "1/75557863725914324467712",
        2,
        Up,
        "1.1e23",
        "0x1.8E+19",
        Greater,
    );
}

#[test]
#[should_panic]
fn gamma_prec_round_fail_1() {
    BigExponentFloat::ONE.gamma_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gamma_prec_round_fail_2() {
    BigExponentFloat::from(0.5).gamma_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn gamma_rational_prec_round_fail_1() {
    BigExponentFloat::gamma_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn gamma_rational_prec_round_fail_2() {
    BigExponentFloat::gamma_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn gamma_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = gamma_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if *x > 0u32 {
        // The gamma function is positive on the positive reals.
        assert!(r.0 > 0u32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = gamma_prec_round_all(x, prec, Floor);
        let ceiling = gamma_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&gamma_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = gamma_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(&gamma_rational_prec_round_all(&q, prec, rm), &r));
    }
}

#[test]
fn gamma_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            gamma_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            gamma_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn gamma_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.gamma_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = gamma_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn gamma_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = gamma_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if x > 0u32 {
            // The gamma function is positive on the positive reals.
            assert!(r.0 > 0u32);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if rm != Nearest {
                let (y, o) = gamma_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn gamma_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::gamma_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = gamma_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
