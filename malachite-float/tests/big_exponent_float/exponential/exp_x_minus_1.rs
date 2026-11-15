// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_10, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_20,
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

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn exp_x_minus_1_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().exp_x_minus_1_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.exp_x_minus_1_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.exp_x_minus_1_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().exp_x_minus_1_prec(prec), &r));
        assert!(same_pair(&x.exp_x_minus_1_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.exp_x_minus_1_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn exp_x_minus_1_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::exp_x_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::exp_x_minus_1_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::exp_x_minus_1_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_exp_x_minus_1_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&ninf, 10, rm),
            &(BigExponentFloat::NEGATIVE_ONE, Equal)
        ));
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = exp_x_minus_1_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "22.12",
        "0x16.2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "22.156",
        "0x16.28",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "22.12",
        "0x16.2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "22.156",
        "0x16.28",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "22.156",
        "0x16.28",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "22.140692632779266171811570089356",
        "0x16.24046eb09338d2991a30893e",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "-0.9570",
        "-0x0.f5",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "-0.95605",
        "-0x0.f4c",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Down,
        "-0.95605",
        "-0x0.f4c",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Up,
        "-0.9570",
        "-0x0.f5",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Nearest,
        "-0.9570",
        "-0x0.f5",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "-0.9567860817362277449334035947940",
        "-0x0.f4efeec253388335b0cf8dd92",
        Less,
    );
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test("0.50", "0x0.8", 10, Floor, "0.6484", "0x0.a6", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.64941", "0x0.a64", Greater);
    test("0.50", "0x0.8", 10, Down, "0.6484", "0x0.a6", Less);
    test("0.50", "0x0.8", 10, Up, "0.64941", "0x0.a64", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.6484", "0x0.a6", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.6487212707001281468486507878142",
        "0x0.a61298e1e069bc972dfefab6e",
        Greater,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-0.3936", "-0x0.64c", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-0.39307", "-0x0.64a", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "-0.39307", "-0x0.64a", Greater);
    test("-0.50", "-0x0.8", 10, Up, "-0.3936", "-0x0.64c", Less);
    test("-0.50", "-0x0.8", 10, Nearest, "-0.3936", "-0x0.64c", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.3934693402873665763962004650089",
        "-0x0.64ba681c834fb00c298a5caad",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("3.0", "0x3", 10, Floor, "19.06", "0x13.1", Less);
    test("3.0", "0x3", 10, Ceiling, "19.094", "0x13.18", Greater);
    test("3.0", "0x3", 10, Down, "19.06", "0x13.1", Less);
    test("3.0", "0x3", 10, Up, "19.094", "0x13.18", Greater);
    test("3.0", "0x3", 10, Nearest, "19.094", "0x13.18", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "19.08553692318766774092852965458",
        "0x13.15e5bf6fb105f2d4bdfc5374",
        Less,
    );
    test("-3.0", "-0x3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-3.0", "-0x3", 10, Floor, "-0.9512", "-0x0.f38", Less);
    test("-3.0", "-0x3", 10, Ceiling, "-0.95020", "-0x0.f34", Greater);
    test("-3.0", "-0x3", 10, Down, "-0.95020", "-0x0.f34", Greater);
    test("-3.0", "-0x3", 10, Up, "-0.9512", "-0x0.f38", Less);
    test("-3.0", "-0x3", 10, Nearest, "-0.95020", "-0x0.f34", Greater);
    test(
        "-3.0",
        "-0x3",
        100,
        Nearest,
        "-0.95021293163213605702065758434956",
        "-0x0.f341279998a7a89b5becfe6e3",
        Greater,
    );
    test("100.0", "0x64", 1, Nearest, "2.2e43", "0x1.0E+36", Less);
    test("100.0", "0x64", 10, Floor, "2.6874e43", "0x1.348E+36", Less);
    test(
        "100.0",
        "0x64",
        10,
        Ceiling,
        "2.692e43",
        "0x1.35E+36",
        Greater,
    );
    test("100.0", "0x64", 10, Down, "2.6874e43", "0x1.348E+36", Less);
    test("100.0", "0x64", 10, Up, "2.692e43", "0x1.35E+36", Greater);
    test(
        "100.0",
        "0x64",
        10,
        Nearest,
        "2.6874e43",
        "0x1.348E+36",
        Less,
    );
    test(
        "100.0",
        "0x64",
        100,
        Nearest,
        "2.6881171418161354484126255515796e43",
        "0x1.3494a9b171bf4acc225093322E+36",
        Less,
    );
    test("-100.0", "-0x64", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-100.0", "-0x64", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater,
    );
    test("-100.0", "-0x64", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-100.0", "-0x64", 10, Up, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 100, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "1.00000000e10",
        "0x2540be400.0",
        1,
        Nearest,
        "1.2e4342944819",
        "0x2.0E+3606737602",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Floor,
        "1.077e4342944819",
        "0x1.daE+3606737602",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "1.0782e4342944819",
        "0x1.da8E+3606737602",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "1.077e4342944819",
        "0x1.daE+3606737602",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "1.0782e4342944819",
        "0x1.da8E+3606737602",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "1.0782e4342944819",
        "0x1.da8E+3606737602",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "1.077750607958564910214246295913e4342944819",
        "0x1.da4af321e8100802fc3be5738E+3606737602",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Up,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "6.9e525029812488999453498029",
        "0x1.0E+436027821190162759409476",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Floor,
        "6.8105e525029812488999453498029",
        "0xf.e4E+436027821190162759409475",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "6.817e525029812488999453498029",
        "0xf.e8E+436027821190162759409475",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Down,
        "6.8105e525029812488999453498029",
        "0xf.e4E+436027821190162759409475",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Up,
        "6.817e525029812488999453498029",
        "0xf.e8E+436027821190162759409475",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Nearest,
        "6.8105e525029812488999453498029",
        "0xf.e4E+436027821190162759409475",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "6.8135203266872288865834036651423e525029812488999453498029",
        "0xf.e5c7bf54c99c82105ab42b69E+436027821190162759409475",
        Less,
    );
    test("-1.2e24", "-0x1.0E+20", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.2e24", "-0x1.0E+20", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-1.2e24", "-0x1.0E+20", 10, Up, "-1.0", "-0x1.0", Less);
    test("-1.2e24", "-0x1.0E+20", 10, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        1,
        Nearest,
        "3.9e-121",
        "0x1.0E-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "3.9e-121",
        "0x1.0E-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "3.8802e-121",
        "0x1.008E-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "3.9e-121",
        "0x1.0E-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "3.8802e-121",
        "0x1.008E-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "3.9e-121",
        "0x1.0E-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "3.9e-121",
        "0x1.0E-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        1,
        Nearest,
        "-3.9e-121",
        "-0x1.0E-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Floor,
        "-3.9e-121",
        "-0x1.0E-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Ceiling,
        "-3.8688e-121",
        "-0xf.fcE-101",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Down,
        "-3.8688e-121",
        "-0xf.fcE-101",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Up,
        "-3.9e-121",
        "-0x1.0E-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Nearest,
        "-3.9e-121",
        "-0x1.0E-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        100,
        Nearest,
        "-3.9e-121",
        "-0x1.0E-100",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "3.2e-361235995",
        "0x2.0E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "2.3982e-361235995",
        "0x1.808E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "2.3982e-361235995",
        "0x1.808E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "-1.6e-361235995",
        "-0x1.0E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "-2.3919e-361235995",
        "-0x1.7f8E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "-2.3919e-361235995",
        "-0x1.7f8E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        1,
        Nearest,
        "3.1e582900186",
        "0x4.0E+484088126",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Floor,
        "2.751e582900186",
        "0x3.9aE+484088126",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Ceiling,
        "2.7544e582900186",
        "0x3.9bE+484088126",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Down,
        "2.751e582900186",
        "0x3.9aE+484088126",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Up,
        "2.7544e582900186",
        "0x3.9bE+484088126",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Nearest,
        "2.7544e582900186",
        "0x3.9bE+484088126",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        100,
        Nearest,
        "2.753694525601369937805322777376e582900186",
        "0x3.9ac6d287be5b0a228175e8a3E+484088126",
        Less,
    );
    test("-1.3e9", "-0x5.0E+7", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-1.3e9", "-0x5.0E+7", 10, Up, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 100, Nearest, "-1.0", "-0x1.0", Less);
}

#[test]
fn test_exp_x_minus_1_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &exp_x_minus_1_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ZERO, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = exp_x_minus_1_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/3", 1, Nearest, "0.50", "0x0.8", Greater);
    test("1/3", 10, Floor, "0.3955", "0x0.654", Less);
    test("1/3", 10, Ceiling, "0.39600", "0x0.656", Greater);
    test("1/3", 10, Down, "0.3955", "0x0.654", Less);
    test("1/3", 10, Up, "0.39600", "0x0.656", Greater);
    test("1/3", 10, Nearest, "0.3955", "0x0.654", Less);
    test(
        "1/3",
        100,
        Nearest,
        "0.39561242508608952862812531960265",
        "0x0.6546db1ba2d1310a7f6c6f27f8",
        Greater,
    );
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-1/3", 10, Floor, "-0.28369", "-0x0.48a", Less);
    test("-1/3", 10, Ceiling, "-0.2832", "-0x0.488", Greater);
    test("-1/3", 10, Down, "-0.2832", "-0x0.488", Greater);
    test("-1/3", 10, Up, "-0.28369", "-0x0.48a", Less);
    test("-1/3", 10, Nearest, "-0.28369", "-0x0.48a", Less);
    test(
        "-1/3",
        100,
        Nearest,
        "-0.2834686894262107495743959030747",
        "-0x0.4891676e868ad9763a67b363b",
        Less,
    );
    test("3/5", 1, Nearest, "1.0", "0x1.0", Greater);
    test("3/5", 10, Floor, "0.82129", "0x0.d24", Less);
    test("3/5", 10, Ceiling, "0.8223", "0x0.d28", Greater);
    test("3/5", 10, Down, "0.82129", "0x0.d24", Less);
    test("3/5", 10, Up, "0.8223", "0x0.d28", Greater);
    test("3/5", 10, Nearest, "0.8223", "0x0.d28", Greater);
    test(
        "3/5",
        100,
        Nearest,
        "0.8221188003905089748753676681628",
        "0x0.d27660b11a9ef66918b8417ce",
        Less,
    );
    test("3", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("3", 10, Floor, "19.06", "0x13.1", Less);
    test("3", 10, Ceiling, "19.094", "0x13.18", Greater);
    test("3", 10, Down, "19.06", "0x13.1", Less);
    test("3", 10, Up, "19.094", "0x13.18", Greater);
    test("3", 10, Nearest, "19.094", "0x13.18", Greater);
    test(
        "3",
        100,
        Nearest,
        "19.08553692318766774092852965458",
        "0x13.15e5bf6fb105f2d4bdfc5374",
        Less,
    );
    test("-3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-3", 10, Floor, "-0.9512", "-0x0.f38", Less);
    test("-3", 10, Ceiling, "-0.95020", "-0x0.f34", Greater);
    test("-3", 10, Down, "-0.95020", "-0x0.f34", Greater);
    test("-3", 10, Up, "-0.9512", "-0x0.f38", Less);
    test("-3", 10, Nearest, "-0.95020", "-0x0.f34", Greater);
    test(
        "-3",
        100,
        Nearest,
        "-0.95021293163213605702065758434956",
        "-0x0.f341279998a7a89b5becfe6e3",
        Greater,
    );
    test("100", 1, Nearest, "2.2e43", "0x1.0E+36", Less);
    test("100", 10, Floor, "2.6874e43", "0x1.348E+36", Less);
    test("100", 10, Ceiling, "2.692e43", "0x1.35E+36", Greater);
    test("100", 10, Down, "2.6874e43", "0x1.348E+36", Less);
    test("100", 10, Up, "2.692e43", "0x1.35E+36", Greater);
    test("100", 10, Nearest, "2.6874e43", "0x1.348E+36", Less);
    test(
        "100",
        100,
        Nearest,
        "2.6881171418161354484126255515796e43",
        "0x1.3494a9b171bf4acc225093322E+36",
        Less,
    );
    test("-100", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-100", 10, Floor, "-1.0", "-0x1.0", Less);
    test("-100", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater);
    test("-100", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-100", 10, Up, "-1.0", "-0x1.0", Less);
    test("-100", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-100", 100, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "10000000000/3",
        1,
        Nearest,
        "1.3e1447648273",
        "0x4.0E+1202245867",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Floor,
        "1.025e1447648273",
        "0x3.18E+1202245867",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Ceiling,
        "1.0262e1447648273",
        "0x3.19E+1202245867",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Down,
        "1.025e1447648273",
        "0x3.18E+1202245867",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Up,
        "1.0262e1447648273",
        "0x3.19E+1202245867",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Nearest,
        "1.025e1447648273",
        "0x3.18E+1202245867",
        Less,
    );
    test(
        "10000000000/3",
        100,
        Nearest,
        "1.0252727754553837947859138372518e1447648273",
        "0x3.18494a338819c27f1502529ccE+1202245867",
        Greater,
    );
    test("-10000000000/3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-10000000000/3",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-10000000000/3", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-10000000000/3", 10, Up, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 100, Nearest, "-1.0", "-0x1.0", Less);
    test("1/10000000000", 1, Nearest, "1.2e-10", "0x8.0E-9", Greater);
    test("1/10000000000", 10, Floor, "9.9931e-11", "0x6.deE-9", Less);
    test(
        "1/10000000000",
        10,
        Ceiling,
        "1.00e-10",
        "0x6.eE-9",
        Greater,
    );
    test("1/10000000000", 10, Down, "9.9931e-11", "0x6.deE-9", Less);
    test("1/10000000000", 10, Up, "1.00e-10", "0x6.eE-9", Greater);
    test(
        "1/10000000000",
        10,
        Nearest,
        "1.00e-10",
        "0x6.eE-9",
        Greater,
    );
    test(
        "1/10000000000",
        100,
        Nearest,
        "1.000000000050000000001666666667e-10",
        "0x6.df37f6776938bebf05d27645E-9",
        Less,
    );
    test("-1/10000000000", 1, Nearest, "-1.2e-10", "-0x8.0E-9", Less);
    test("-1/10000000000", 10, Floor, "-1.00e-10", "-0x6.eE-9", Less);
    test(
        "-1/10000000000",
        10,
        Ceiling,
        "-9.9931e-11",
        "-0x6.deE-9",
        Greater,
    );
    test(
        "-1/10000000000",
        10,
        Down,
        "-9.9931e-11",
        "-0x6.deE-9",
        Greater,
    );
    test("-1/10000000000", 10, Up, "-1.00e-10", "-0x6.eE-9", Less);
    test(
        "-1/10000000000",
        10,
        Nearest,
        "-1.00e-10",
        "-0x6.eE-9",
        Less,
    );
    test(
        "-1/10000000000",
        100,
        Nearest,
        "-9.999999999500000000016666666669e-11",
        "-0x6.df37f67475a49d2cbd8e0a9bE-9",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        1,
        Nearest,
        "7.0e175009937496333151166009",
        "0x2.0E+145342607063387586469825",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Floor,
        "8.7899e175009937496333151166009",
        "0x2.83E+145342607063387586469825",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Ceiling,
        "8.804e175009937496333151166009",
        "0x2.84E+145342607063387586469825",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Down,
        "8.7899e175009937496333151166009",
        "0x2.83E+145342607063387586469825",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Up,
        "8.804e175009937496333151166009",
        "0x2.84E+145342607063387586469825",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Nearest,
        "8.804e175009937496333151166009",
        "0x2.84E+145342607063387586469825",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        100,
        Nearest,
        "8.7994835813890005691847382363241e175009937496333151166009",
        "0x2.83b33f00d8170fdf388d8bfdcE+145342607063387586469825",
        Greater,
    );
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_round_fail_1() {
    BigExponentFloat::ONE.exp_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_round_fail_2() {
    BigExponentFloat::from(0.5).exp_x_minus_1_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_fail_1() {
    BigExponentFloat::exp_x_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_fail_2() {
    BigExponentFloat::exp_x_minus_1_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn exp_x_minus_1_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = exp_x_minus_1_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = exp_x_minus_1_prec_round_all(x, prec, Floor);
        let ceiling = exp_x_minus_1_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&exp_x_minus_1_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = exp_x_minus_1_prec_round_all(x, prec + 16, rm);
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
            &exp_x_minus_1_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn exp_x_minus_1_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_10().test_properties(
        |(x, prec, rm)| {
            exp_x_minus_1_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn exp_x_minus_1_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.exp_x_minus_1_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = exp_x_minus_1_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn exp_x_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_20().test_properties(|(x, prec, rm)| {
        let r = exp_x_minus_1_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = exp_x_minus_1_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn exp_x_minus_1_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_20().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::exp_x_minus_1_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = exp_x_minus_1_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
