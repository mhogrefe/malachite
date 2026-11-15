// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::exponential::exp::exp_x_minus_1_series;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_19,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
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

fn exp_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().exp_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.exp_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.exp_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().exp_prec(prec), &r));
        assert!(same_pair(&x.exp_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.exp_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn exp_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::exp_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::exp_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::exp_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::exp_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_exp_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &exp_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_prec_round_all(&ninf, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_prec_round_all(&zero, 10, rm),
            &(BigExponentFloat::ONE, Equal)
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
        let r = exp_prec_round_all(&hex(x_hex), prec, rm);
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
        "23.12",
        "0x17.2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "23.156",
        "0x17.28",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "23.12",
        "0x17.2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "23.156",
        "0x17.28",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "23.156",
        "0x17.28",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "23.140692632779266171811570089356",
        "0x17.24046eb09338d2991a30893e",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "0.031",
        "0x0.08",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "0.04321",
        "0x0.0b1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "0.043274",
        "0x0.0b14",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Down,
        "0.04321",
        "0x0.0b1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Up,
        "0.043274",
        "0x0.0b14",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Nearest,
        "0.04321",
        "0x0.0b1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "0.04321391826377225506659640520612",
        "0x0.0b10113dacc77cca4f307226e2",
        Less,
    );
    test("0.50", "0x0.8", 1, Nearest, "2.0", "0x2.0", Greater);
    test("0.50", "0x0.8", 10, Floor, "1.648", "0x1.a6", Less);
    test("0.50", "0x0.8", 10, Ceiling, "1.6504", "0x1.a68", Greater);
    test("0.50", "0x0.8", 10, Down, "1.648", "0x1.a6", Less);
    test("0.50", "0x0.8", 10, Up, "1.6504", "0x1.a68", Greater);
    test("0.50", "0x0.8", 10, Nearest, "1.648", "0x1.a6", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "1.6487212707001281468486507878142",
        "0x1.a61298e1e069bc972dfefab6e",
        Greater,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test("-0.50", "-0x0.8", 10, Floor, "0.60645", "0x0.9b4", Less);
    test("-0.50", "-0x0.8", 10, Ceiling, "0.6074", "0x0.9b8", Greater);
    test("-0.50", "-0x0.8", 10, Down, "0.60645", "0x0.9b4", Less);
    test("-0.50", "-0x0.8", 10, Up, "0.6074", "0x0.9b8", Greater);
    test("-0.50", "-0x0.8", 10, Nearest, "0.60645", "0x0.9b4", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "0.60653065971263342360379953499114",
        "0x0.9b4597e37cb04ff3d675a3553",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("3.0", "0x3", 10, Floor, "20.06", "0x14.1", Less);
    test("3.0", "0x3", 10, Ceiling, "20.094", "0x14.18", Greater);
    test("3.0", "0x3", 10, Down, "20.06", "0x14.1", Less);
    test("3.0", "0x3", 10, Up, "20.094", "0x14.18", Greater);
    test("3.0", "0x3", 10, Nearest, "20.094", "0x14.18", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "20.08553692318766774092852965458",
        "0x14.15e5bf6fb105f2d4bdfc5374",
        Less,
    );
    test("-3.0", "-0x3", 1, Nearest, "0.062", "0x0.1", Greater);
    test("-3.0", "-0x3", 10, Floor, "0.049744", "0x0.0cbc", Less);
    test("-3.0", "-0x3", 10, Ceiling, "0.0498", "0x0.0cc", Greater);
    test("-3.0", "-0x3", 10, Down, "0.049744", "0x0.0cbc", Less);
    test("-3.0", "-0x3", 10, Up, "0.0498", "0x0.0cc", Greater);
    test("-3.0", "-0x3", 10, Nearest, "0.0498", "0x0.0cc", Greater);
    test(
        "-3.0",
        "-0x3",
        100,
        Nearest,
        "0.04978706836786394297934241565005",
        "0x0.0cbed86667585764a4130191c8",
        Less,
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
    test(
        "-100.0",
        "-0x64",
        1,
        Nearest,
        "4.5e-44",
        "0x1.0E-36",
        Greater,
    );
    test(
        "-100.0",
        "-0x64",
        10,
        Floor,
        "3.7178e-44",
        "0xd.44E-37",
        Less,
    );
    test(
        "-100.0",
        "-0x64",
        10,
        Ceiling,
        "3.722e-44",
        "0xd.48E-37",
        Greater,
    );
    test(
        "-100.0",
        "-0x64",
        10,
        Down,
        "3.7178e-44",
        "0xd.44E-37",
        Less,
    );
    test(
        "-100.0",
        "-0x64",
        10,
        Up,
        "3.722e-44",
        "0xd.48E-37",
        Greater,
    );
    test(
        "-100.0",
        "-0x64",
        10,
        Nearest,
        "3.722e-44",
        "0xd.48E-37",
        Greater,
    );
    test(
        "-100.0",
        "-0x64",
        100,
        Nearest,
        "3.720075976020835962959695803864e-44",
        "0xd.460f8a7157ae579eec89bae4E-37",
        Greater,
    );
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
        "8.6e-4342944820",
        "0x8.0E-3606737603",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Floor,
        "9.267e-4342944820",
        "0x8.aE-3606737603",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Ceiling,
        "9.2835e-4342944820",
        "0x8.a4E-3606737603",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Down,
        "9.267e-4342944820",
        "0x8.aE-3606737603",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Up,
        "9.2835e-4342944820",
        "0x8.a4E-3606737603",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Nearest,
        "9.2835e-4342944820",
        "0x8.a4E-3606737603",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        100,
        Nearest,
        "9.278584420324872578073142298936e-4342944820",
        "0x8.a2d1f2d4a1f560f8f4886316E-3606737603",
        Greater,
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
    test(
        "-1.2e24",
        "-0x1.0E+20",
        1,
        Nearest,
        "1.5e-525029812488999453498030",
        "0x1.0E-436027821190162759409476",
        Less,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Floor,
        "1.4668e-525029812488999453498030",
        "0x1.018E-436027821190162759409476",
        Less,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "1.470e-525029812488999453498030",
        "0x1.02E-436027821190162759409476",
        Greater,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Down,
        "1.4668e-525029812488999453498030",
        "0x1.018E-436027821190162759409476",
        Less,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Up,
        "1.470e-525029812488999453498030",
        "0x1.02E-436027821190162759409476",
        Greater,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Nearest,
        "1.4668e-525029812488999453498030",
        "0x1.018E-436027821190162759409476",
        Less,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "1.4676700913082995207723454478026e-525029812488999453498030",
        "0x1.01a637f20b0d2d4c0591d3646E-436027821190162759409476",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.9e-121", "0x1.0E-100", 10, Floor, "1.0", "0x1.0", Less);
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 10, Down, "1.0", "0x1.0", Less);
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 10, Nearest, "1.0", "0x1.0", Less);
    test("3.9e-121", "0x1.0E-100", 100, Nearest, "1.0", "0x1.0", Less);
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test("-3.9e-121", "-0x1.0E-100", 10, Up, "1.0", "0x1.0", Greater);
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
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
    test(
        "-1.3e9",
        "-0x5.0E+7",
        1,
        Nearest,
        "3.3e-582900187",
        "0x4.0E-484088127",
        Less,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Floor,
        "3.630e-582900187",
        "0x4.7E-484088127",
        Less,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Ceiling,
        "3.6368e-582900187",
        "0x4.72E-484088127",
        Greater,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Down,
        "3.630e-582900187",
        "0x4.7E-484088127",
        Less,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Up,
        "3.6368e-582900187",
        "0x4.72E-484088127",
        Greater,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Nearest,
        "3.630e-582900187",
        "0x4.7E-484088127",
        Less,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        100,
        Nearest,
        "3.6314848677037385513805428120359e-582900187",
        "0x4.7053b4b05362634f678632d88E-484088127",
        Less,
    );
}

#[test]
fn test_exp_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &exp_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = exp_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.395", "0x1.65", Less);
    test("1/3", 10, Ceiling, "1.3965", "0x1.658", Greater);
    test("1/3", 10, Down, "1.395", "0x1.65", Less);
    test("1/3", 10, Up, "1.3965", "0x1.658", Greater);
    test("1/3", 10, Nearest, "1.3965", "0x1.658", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.39561242508608952862812531960",
        "0x1.6546db1ba2d1310a7f6c6f28",
        Greater,
    );
    test("-1/3", 1, Nearest, "0.50", "0x0.8", Less);
    test("-1/3", 10, Floor, "0.71582", "0x0.b74", Less);
    test("-1/3", 10, Ceiling, "0.7168", "0x0.b78", Greater);
    test("-1/3", 10, Down, "0.71582", "0x0.b74", Less);
    test("-1/3", 10, Up, "0.7168", "0x0.b78", Greater);
    test("-1/3", 10, Nearest, "0.7168", "0x0.b78", Greater);
    test(
        "-1/3",
        100,
        Nearest,
        "0.71653131057378925042560409692533",
        "0x0.b76e989179752689c5984c9c5",
        Less,
    );
    test("3/5", 1, Nearest, "2.0", "0x2.0", Greater);
    test("3/5", 10, Floor, "1.820", "0x1.d2", Less);
    test("3/5", 10, Ceiling, "1.8223", "0x1.d28", Greater);
    test("3/5", 10, Down, "1.820", "0x1.d2", Less);
    test("3/5", 10, Up, "1.8223", "0x1.d28", Greater);
    test("3/5", 10, Nearest, "1.8223", "0x1.d28", Greater);
    test(
        "3/5",
        100,
        Nearest,
        "1.8221188003905089748753676681628",
        "0x1.d27660b11a9ef66918b8417ce",
        Less,
    );
    test("3", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("3", 10, Floor, "20.06", "0x14.1", Less);
    test("3", 10, Ceiling, "20.094", "0x14.18", Greater);
    test("3", 10, Down, "20.06", "0x14.1", Less);
    test("3", 10, Up, "20.094", "0x14.18", Greater);
    test("3", 10, Nearest, "20.094", "0x14.18", Greater);
    test(
        "3",
        100,
        Nearest,
        "20.08553692318766774092852965458",
        "0x14.15e5bf6fb105f2d4bdfc5374",
        Less,
    );
    test("-3", 1, Nearest, "0.062", "0x0.1", Greater);
    test("-3", 10, Floor, "0.049744", "0x0.0cbc", Less);
    test("-3", 10, Ceiling, "0.0498", "0x0.0cc", Greater);
    test("-3", 10, Down, "0.049744", "0x0.0cbc", Less);
    test("-3", 10, Up, "0.0498", "0x0.0cc", Greater);
    test("-3", 10, Nearest, "0.0498", "0x0.0cc", Greater);
    test(
        "-3",
        100,
        Nearest,
        "0.04978706836786394297934241565005",
        "0x0.0cbed86667585764a4130191c8",
        Less,
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
    test("-100", 1, Nearest, "4.5e-44", "0x1.0E-36", Greater);
    test("-100", 10, Floor, "3.7178e-44", "0xd.44E-37", Less);
    test("-100", 10, Ceiling, "3.722e-44", "0xd.48E-37", Greater);
    test("-100", 10, Down, "3.7178e-44", "0xd.44E-37", Less);
    test("-100", 10, Up, "3.722e-44", "0xd.48E-37", Greater);
    test("-100", 10, Nearest, "3.722e-44", "0xd.48E-37", Greater);
    test(
        "-100",
        100,
        Nearest,
        "3.720075976020835962959695803864e-44",
        "0xd.460f8a7157ae579eec89bae4E-37",
        Greater,
    );
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
    test(
        "-10000000000/3",
        1,
        Nearest,
        "7.5e-1447648274",
        "0x4.0E-1202245868",
        Less,
    );
    test(
        "-10000000000/3",
        10,
        Floor,
        "9.7426e-1447648274",
        "0x5.2aE-1202245868",
        Less,
    );
    test(
        "-10000000000/3",
        10,
        Ceiling,
        "9.757e-1447648274",
        "0x5.2cE-1202245868",
        Greater,
    );
    test(
        "-10000000000/3",
        10,
        Down,
        "9.7426e-1447648274",
        "0x5.2aE-1202245868",
        Less,
    );
    test(
        "-10000000000/3",
        10,
        Up,
        "9.757e-1447648274",
        "0x5.2cE-1202245868",
        Greater,
    );
    test(
        "-10000000000/3",
        10,
        Nearest,
        "9.757e-1447648274",
        "0x5.2cE-1202245868",
        Greater,
    );
    test(
        "-10000000000/3",
        100,
        Nearest,
        "9.753501935676009027041708897005e-1447648274",
        "0x5.2b7b2f46f30d4cfa8bf9eb96E-1202245868",
        Less,
    );
    test("1/10000000000", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/10000000000", 10, Floor, "1.0", "0x1.0", Less);
    test("1/10000000000", 10, Ceiling, "1.0020", "0x1.008", Greater);
    test("1/10000000000", 10, Down, "1.0", "0x1.0", Less);
    test("1/10000000000", 10, Up, "1.0020", "0x1.008", Greater);
    test("1/10000000000", 10, Nearest, "1.0", "0x1.0", Less);
    test(
        "1/10000000000",
        100,
        Nearest,
        "1.00000000010000000000500000000",
        "0x1.000000006df37f6776938bec",
        Greater,
    );
    test("-1/10000000000", 1, Nearest, "1.0", "0x1.0", Greater);
    test("-1/10000000000", 10, Floor, "0.99902", "0x0.ffc", Less);
    test("-1/10000000000", 10, Ceiling, "1.0", "0x1.0", Greater);
    test("-1/10000000000", 10, Down, "0.99902", "0x0.ffc", Less);
    test("-1/10000000000", 10, Up, "1.0", "0x1.0", Greater);
    test("-1/10000000000", 10, Nearest, "1.0", "0x1.0", Greater);
    test(
        "-1/10000000000",
        100,
        Nearest,
        "0.99999999990000000000499999999963",
        "0x0.ffffffff920c8098b8a5b62d3",
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
fn exp_prec_round_fail_1() {
    BigExponentFloat::ONE.exp_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn exp_prec_round_fail_2() {
    BigExponentFloat::from(0.5).exp_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn exp_rational_prec_round_fail_1() {
    BigExponentFloat::exp_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn exp_rational_prec_round_fail_2() {
    BigExponentFloat::exp_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn exp_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = exp_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert!(r.0 >= 0u32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = exp_prec_round_all(x, prec, Floor);
        let ceiling = exp_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&exp_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = exp_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(&exp_rational_prec_round_all(&q, prec, rm), &r));
    }
}

#[test]
fn exp_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            exp_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn exp_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.exp_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = exp_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn exp_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        let r = exp_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = exp_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn exp_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::exp_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = exp_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

#[test]
fn exp_x_minus_1_series_properties() {
    // The series approximation of e^t - 1 is within its stated error bound of `Float`'s correctly
    // rounded value, computed with 64 more bits.
    float_unsigned_pair_gen_var_1::<u64>().test_properties(|(t, prec)| {
        if !t.is_finite() || t.is_zero() || t.get_exponent().unwrap() > -1 {
            return;
        }
        let wp = prec + 16;
        let (s, loss) = exp_x_minus_1_series(&float_to_big_exponent_float(&t), wp);
        let exact =
            float_to_big_exponent_float(&t.exp_x_minus_1_prec_round_ref(wp + 64, Nearest).0);
        let error = (s.clone() - exact).abs();
        let bound = big_exponent_float_power_of_2(
            s.get_exponent().unwrap() - Integer::from(wp) + Integer::from(loss),
        );
        assert!(error < bound, "{t:#x} {wp}");
    });
}
