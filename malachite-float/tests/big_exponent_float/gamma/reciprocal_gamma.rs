// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, One, Two, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, reciprocal_gamma_prec_round_naive,
    reciprocal_gamma_rational_prec_round_naive,
};
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

// Whether the reference, which takes twice as many bits as a tiny argument has leading zeros, and
// whose time grows with a large argument, is run on `x`.
fn reference_is_cheap(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_some_and(|e| *e >= -1000i32 && *e <= 20u32)
}

fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn reciprocal_gamma_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().reciprocal_gamma_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.reciprocal_gamma_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.reciprocal_gamma_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().reciprocal_gamma_prec(prec), &r));
        assert!(same_pair(&x.reciprocal_gamma_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.reciprocal_gamma_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn reciprocal_gamma_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::reciprocal_gamma_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::reciprocal_gamma_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::reciprocal_gamma_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_reciprocal_gamma_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &reciprocal_gamma_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_prec_round_all(&inf, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        // The reciprocal gamma function is 0 at the poles of the gamma function.
        assert!(same_pair(
            &reciprocal_gamma_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_rational_prec_round_all(&Rational::NEGATIVE_ONE, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_prec_round_all(&BigExponentFloat::ONE, 10, rm),
            &(BigExponentFloat::ONE.clone(), Equal)
        ));
        assert!(same_pair(
            &reciprocal_gamma_rational_prec_round_all(&Rational::TWO, 10, rm),
            &(BigExponentFloat::ONE.clone(), Equal)
        ));
    }
    // The expected values agree with Azurite's reciprocal gamma function.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = reciprocal_gamma_prec_round_all(&hex(x_hex), prec, rm);
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
    test("3.0", "0x3.0", 10, Floor, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 10, Ceiling, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 10, Down, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 10, Up, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 10, Nearest, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 10, Exact, "0.50", "0x0.8", Equal);
    test("4.0", "0x4.0", 10, Floor, "0.1665", "0x0.2aa", Less);
    test("4.0", "0x4.0", 10, Ceiling, "0.16675", "0x0.2ab", Greater);
    test("4.0", "0x4.0", 10, Down, "0.1665", "0x0.2aa", Less);
    test("4.0", "0x4.0", 10, Up, "0.16675", "0x0.2ab", Greater);
    test("4.0", "0x4.0", 10, Nearest, "0.16675", "0x0.2ab", Greater);
    test("5.0", "0x5.0", 10, Floor, "0.04163", "0x0.0aa8", Less);
    test("5.0", "0x5.0", 10, Ceiling, "0.041687", "0x0.0aac", Greater);
    test("5.0", "0x5.0", 10, Down, "0.04163", "0x0.0aa8", Less);
    test("5.0", "0x5.0", 10, Up, "0.041687", "0x0.0aac", Greater);
    test("5.0", "0x5.0", 10, Nearest, "0.041687", "0x0.0aac", Greater);
    test("10.0", "0xa.0", 10, Floor, "2.7530e-6", "0x0.00002e3", Less);
    test(
        "10.0",
        "0xa.0",
        10,
        Ceiling,
        "2.757e-6",
        "0x0.00002e4",
        Greater,
    );
    test("10.0", "0xa.0", 10, Down, "2.7530e-6", "0x0.00002e3", Less);
    test("10.0", "0xa.0", 10, Up, "2.757e-6", "0x0.00002e4", Greater);
    test(
        "10.0",
        "0xa.0",
        10,
        Nearest,
        "2.757e-6",
        "0x0.00002e4",
        Greater,
    );
    test(
        "25.0",
        "0x19.0",
        10,
        Floor,
        "1.6107e-24",
        "0x1.f28E-20",
        Less,
    );
    test(
        "25.0",
        "0x19.0",
        10,
        Ceiling,
        "1.612e-24",
        "0x1.f3E-20",
        Greater,
    );
    test(
        "25.0",
        "0x19.0",
        10,
        Down,
        "1.6107e-24",
        "0x1.f28E-20",
        Less,
    );
    test("25.0", "0x19.0", 10, Up, "1.612e-24", "0x1.f3E-20", Greater);
    test(
        "25.0",
        "0x19.0",
        10,
        Nearest,
        "1.612e-24",
        "0x1.f3E-20",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Floor,
        "1.377e-307",
        "0x1.8cE-255",
        Less,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Ceiling,
        "1.3785e-307",
        "0x1.8c8E-255",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Down,
        "1.377e-307",
        "0x1.8cE-255",
        Less,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Up,
        "1.3785e-307",
        "0x1.8c8E-255",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        10,
        Nearest,
        "1.3785e-307",
        "0x1.8c8E-255",
        Greater,
    );
    test("0.50", "0x0.8", 10, Floor, "0.56348", "0x0.904", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.5645", "0x0.908", Greater);
    test("0.50", "0x0.8", 10, Down, "0.56348", "0x0.904", Less);
    test("0.50", "0x0.8", 10, Up, "0.5645", "0x0.908", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.5645", "0x0.908", Greater);
    test("1.5", "0x1.8", 10, Floor, "1.1270", "0x1.208", Less);
    test("1.5", "0x1.8", 10, Ceiling, "1.129", "0x1.21", Greater);
    test("1.5", "0x1.8", 10, Down, "1.1270", "0x1.208", Less);
    test("1.5", "0x1.8", 10, Up, "1.129", "0x1.21", Greater);
    test("1.5", "0x1.8", 10, Nearest, "1.129", "0x1.21", Greater);
    test("0.25", "0x0.4", 10, Floor, "0.2754", "0x0.468", Less);
    test("0.25", "0x0.4", 10, Ceiling, "0.27588", "0x0.46a", Greater);
    test("0.25", "0x0.4", 10, Down, "0.2754", "0x0.468", Less);
    test("0.25", "0x0.4", 10, Up, "0.27588", "0x0.46a", Greater);
    test("0.25", "0x0.4", 10, Nearest, "0.27588", "0x0.46a", Greater);
    test("0.75", "0x0.c", 10, Floor, "0.81543", "0x0.d0c", Less);
    test("0.75", "0x0.c", 10, Ceiling, "0.8164", "0x0.d1", Greater);
    test("0.75", "0x0.c", 10, Down, "0.81543", "0x0.d0c", Less);
    test("0.75", "0x0.c", 10, Up, "0.8164", "0x0.d1", Greater);
    test("0.75", "0x0.c", 10, Nearest, "0.8164", "0x0.d1", Greater);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Floor,
        "0.10510",
        "0x0.1ae8",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Ceiling,
        "0.1052",
        "0x0.1af",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Down,
        "0.10510",
        "0x0.1ae8",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Up,
        "0.1052",
        "0x0.1af",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        Nearest,
        "0.10510",
        "0x0.1ae8",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Floor,
        "0.37256",
        "0x0.5f6",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Ceiling,
        "0.3730",
        "0x0.5f8",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Down,
        "0.37256",
        "0x0.5f6",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Up,
        "0.3730",
        "0x0.5f8",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc",
        10,
        Nearest,
        "0.37256",
        "0x0.5f6",
        Less,
    );
    test("-0.50", "-0x0.8", 10, Floor, "-0.2822", "-0x0.484", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-0.28174", "-0x0.482", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "-0.28174", "-0x0.482", Greater);
    test("-0.50", "-0x0.8", 10, Up, "-0.2822", "-0x0.484", Less);
    test("-0.50", "-0x0.8", 10, Nearest, "-0.2822", "-0x0.484", Less);
    test("-1.5", "-0x1.8", 10, Floor, "0.4229", "0x0.6c4", Less);
    test("-1.5", "-0x1.8", 10, Ceiling, "0.42334", "0x0.6c6", Greater);
    test("-1.5", "-0x1.8", 10, Down, "0.4229", "0x0.6c4", Less);
    test("-1.5", "-0x1.8", 10, Up, "0.42334", "0x0.6c6", Greater);
    test("-1.5", "-0x1.8", 10, Nearest, "0.42334", "0x0.6c6", Greater);
    test("-2.5", "-0x2.8", 10, Floor, "-1.059", "-0x1.0f", Less);
    test(
        "-2.5", "-0x2.8", 10, Ceiling, "-1.0566", "-0x1.0e8", Greater,
    );
    test("-2.5", "-0x2.8", 10, Down, "-1.0566", "-0x1.0e8", Greater);
    test("-2.5", "-0x2.8", 10, Up, "-1.059", "-0x1.0f", Less);
    test("-2.5", "-0x2.8", 10, Nearest, "-1.059", "-0x1.0f", Less);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "-0.093628",
        "-0x0.17f8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "-0.09351",
        "-0x0.17f",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "-0.09351",
        "-0x0.17f",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "-0.093628",
        "-0x0.17f8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "-0.093628",
        "-0x0.17f8",
        Less,
    );
    test("-1.0", "-0x1.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", 10, Down, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", 10, Up, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0", 10, Down, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0", 10, Up, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "-100.5",
        "-0x64.8",
        10,
        Floor,
        "-2.9819e158",
        "-0x5.6eE+131",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Ceiling,
        "-2.978e158",
        "-0x5.6cE+131",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Down,
        "-2.978e158",
        "-0x5.6cE+131",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Up,
        "-2.9819e158",
        "-0x5.6eE+131",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8",
        10,
        Nearest,
        "-2.9819e158",
        "-0x5.6eE+131",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Floor,
        "-2.0000e-7",
        "-0x3.5bE-6",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Ceiling,
        "-1.998e-7",
        "-0x3.5aE-6",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Down,
        "-1.998e-7",
        "-0x3.5aE-6",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Up,
        "-2.0000e-7",
        "-0x3.5bE-6",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a",
        10,
        Nearest,
        "-2.0000e-7",
        "-0x3.5bE-6",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Floor,
        "0.43701",
        "0x0.6fe",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Ceiling,
        "0.44",
        "0x0.7",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Down,
        "0.43701",
        "0x0.6fe",
        Less,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Up,
        "0.44",
        "0x0.7",
        Greater,
    );
    test(
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        10,
        Nearest,
        "0.43701",
        "0x0.6fe",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Floor,
        "0.3730",
        "0x0.5f8",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Ceiling,
        "0.37354",
        "0x0.5fa",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Down,
        "0.3730",
        "0x0.5f8",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Up,
        "0.37354",
        "0x0.5fa",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Nearest,
        "0.3730",
        "0x0.5f8",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Floor,
        "-0.76660",
        "-0x0.c44",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Ceiling,
        "-0.766",
        "-0x0.c4",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Down,
        "-0.766",
        "-0x0.c4",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Up,
        "-0.76660",
        "-0x0.c44",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554",
        10,
        Nearest,
        "-0.766",
        "-0x0.c4",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Floor,
        "9.9931e-11",
        "0x6.deE-9",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Ceiling,
        "1.00e-10",
        "0x6.eE-9",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Down,
        "9.9931e-11",
        "0x6.deE-9",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Up,
        "1.00e-10",
        "0x6.eE-9",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9",
        10,
        Nearest,
        "1.00e-10",
        "0x6.eE-9",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Floor,
        "-1.00e-10",
        "-0x6.eE-9",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Ceiling,
        "-9.9931e-11",
        "-0x6.deE-9",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Down,
        "-9.9931e-11",
        "-0x6.deE-9",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Up,
        "-1.00e-10",
        "-0x6.eE-9",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9",
        10,
        Nearest,
        "-1.00e-10",
        "-0x6.eE-9",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Floor,
        "9.9888e-301",
        "0xa.b4E-250",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Ceiling,
        "1.000e-300",
        "0xa.b8E-250",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Down,
        "9.9888e-301",
        "0xa.b4E-250",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Up,
        "1.000e-300",
        "0xa.b8E-250",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250",
        10,
        Nearest,
        "1.000e-300",
        "0xa.b8E-250",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Floor,
        "-1.000e-300",
        "-0xa.b8E-250",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Ceiling,
        "-9.9888e-301",
        "-0xa.b4E-250",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Down,
        "-9.9888e-301",
        "-0xa.b4E-250",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Up,
        "-1.000e-300",
        "-0xa.b8E-250",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250",
        10,
        Nearest,
        "-1.000e-300",
        "-0xa.b8E-250",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Floor,
        "7.9e-2567",
        "0xc.0E-2132",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Ceiling,
        "7.8694e-2567",
        "0xc.04E-2132",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Down,
        "7.9e-2567",
        "0xc.0E-2132",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Up,
        "7.8694e-2567",
        "0xc.04E-2132",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8",
        10,
        Nearest,
        "7.9e-2567",
        "0xc.0E-2132",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Floor,
        "3.538e-456569",
        "0x1.5aE-379172",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Ceiling,
        "3.5432e-456569",
        "0x1.5a8E-379172",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Down,
        "3.538e-456569",
        "0x1.5aE-379172",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Up,
        "3.5432e-456569",
        "0x1.5a8E-379172",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4",
        10,
        Nearest,
        "3.5432e-456569",
        "0x1.5a8E-379172",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Floor,
        "1.209e-5565706",
        "0x2.36E-4622219",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Ceiling,
        "1.2112e-5565706",
        "0x2.37E-4622219",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Down,
        "1.209e-5565706",
        "0x2.36E-4622219",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Up,
        "1.2112e-5565706",
        "0x2.37E-4622219",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8",
        10,
        Nearest,
        "1.209e-5565706",
        "0x2.36E-4622219",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Floor,
        "-2.631e5565711",
        "-0x2.32E+4622223",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Ceiling,
        "-2.6261e5565711",
        "-0x2.31E+4622223",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Down,
        "-2.6261e5565711",
        "-0x2.31E+4622223",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Up,
        "-2.631e5565711",
        "-0x2.32E+4622223",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8",
        10,
        Nearest,
        "-2.631e5565711",
        "-0x2.32E+4622223",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Floor,
        "6.664e-496101295",
        "0x1.f4E-412003207",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Ceiling,
        "6.6702e-496101295",
        "0x1.f48E-412003207",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Down,
        "6.664e-496101295",
        "0x1.f4E-412003207",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Up,
        "6.6702e-496101295",
        "0x1.f48E-412003207",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6",
        10,
        Nearest,
        "6.6702e-496101295",
        "0x1.f48E-412003207",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Floor,
        "8.134e-496101299",
        "0xf.aE-412003211",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Ceiling,
        "8.1423e-496101299",
        "0xf.a4E-412003211",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Down,
        "8.134e-496101299",
        "0xf.aE-412003211",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Up,
        "8.1423e-496101299",
        "0xf.a4E-412003211",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8",
        10,
        Nearest,
        "8.1423e-496101299",
        "0xf.a4E-412003211",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Floor,
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Ceiling,
        "1.2435e-330985980542",
        "0x1.008E-274877906944",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Down,
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Up,
        "1.2435e-330985980542",
        "0x1.008E-274877906944",
        Greater,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        10,
        Nearest,
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Less,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Floor,
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Less,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Ceiling,
        "-1.2399e-330985980542",
        "-0xf.fcE-274877906945",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Down,
        "-1.2399e-330985980542",
        "-0xf.fcE-274877906945",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Up,
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Less,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        10,
        Nearest,
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "1.4756e-37609551808354240533484732605464",
        "0x3.83E-31234056697073438931807029430641",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "1.477e-37609551808354240533484732605464",
        "0x3.84E-31234056697073438931807029430641",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Down,
        "1.4756e-37609551808354240533484732605464",
        "0x3.83E-31234056697073438931807029430641",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Up,
        "1.477e-37609551808354240533484732605464",
        "0x3.84E-31234056697073438931807029430641",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Nearest,
        "1.477e-37609551808354240533484732605464",
        "0x3.84E-31234056697073438931807029430641",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Floor,
        "1.3106e-37609551808354240533484732605479",
        "0xe.0cE-31234056697073438931807029430654",
        Less,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Ceiling,
        "1.312e-37609551808354240533484732605479",
        "0xe.1E-31234056697073438931807029430654",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Down,
        "1.3106e-37609551808354240533484732605479",
        "0xe.0cE-31234056697073438931807029430654",
        Less,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Up,
        "1.312e-37609551808354240533484732605479",
        "0xe.1E-31234056697073438931807029430654",
        Greater,
    );
    test(
        "1267650600228229401496703205376.5",
        "0x10000000000000000000000000.8",
        10,
        Nearest,
        "1.312e-37609551808354240533484732605479",
        "0xe.1E-31234056697073438931807029430654",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Floor,
        "-3.077e37609551808354240533484732605508",
        "-0x5.ccE+31234056697073438931807029430677",
        Less,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Ceiling,
        "-3.0724e37609551808354240533484732605508",
        "-0x5.caE+31234056697073438931807029430677",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Down,
        "-3.0724e37609551808354240533484732605508",
        "-0x5.caE+31234056697073438931807029430677",
        Greater,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Up,
        "-3.077e37609551808354240533484732605508",
        "-0x5.ccE+31234056697073438931807029430677",
        Less,
    );
    test(
        "-1267650600228229401496703205376.5",
        "-0x10000000000000000000000000.8",
        10,
        Nearest,
        "-3.077e37609551808354240533484732605508",
        "-0x5.ccE+31234056697073438931807029430677",
        Less,
    );
    test("1.0", "0x1.0", 1, Floor, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Floor, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 1, Ceiling, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Ceiling, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 1, Down, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 100, Down, "1.0", "0x1.0", Equal);
    // - gamma_helper small positive integer
    // - gamma_of_small_integer reciprocal
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
    test("3.0", "0x3.0", 1, Floor, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 100, Floor, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 1, Ceiling, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 100, Ceiling, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 1, Down, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 100, Down, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 1, Up, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 100, Up, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 1, Nearest, "0.50", "0x0.8", Equal);
    test("3.0", "0x3.0", 100, Nearest, "0.50", "0x0.8", Equal);
    test("4.0", "0x4.0", 1, Floor, "0.12", "0x0.2", Less);
    test(
        "4.0",
        "0x4.0",
        100,
        Floor,
        "0.1666666666666666666666666666665",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaa8",
        Less,
    );
    test("4.0", "0x4.0", 1, Ceiling, "0.25", "0x0.4", Greater);
    test(
        "4.0",
        "0x4.0",
        100,
        Ceiling,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("4.0", "0x4.0", 1, Down, "0.12", "0x0.2", Less);
    test(
        "4.0",
        "0x4.0",
        100,
        Down,
        "0.1666666666666666666666666666665",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaa8",
        Less,
    );
    test("4.0", "0x4.0", 1, Up, "0.25", "0x0.4", Greater);
    test(
        "4.0",
        "0x4.0",
        100,
        Up,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("4.0", "0x4.0", 1, Nearest, "0.12", "0x0.2", Less);
    test(
        "4.0",
        "0x4.0",
        100,
        Nearest,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("5.0", "0x5.0", 1, Floor, "0.031", "0x0.08", Less);
    test(
        "5.0",
        "0x5.0",
        100,
        Floor,
        "0.04166666666666666666666666666663",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa",
        Less,
    );
    test("5.0", "0x5.0", 1, Ceiling, "0.062", "0x0.1", Greater);
    test(
        "5.0",
        "0x5.0",
        100,
        Ceiling,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test("5.0", "0x5.0", 1, Down, "0.031", "0x0.08", Less);
    test(
        "5.0",
        "0x5.0",
        100,
        Down,
        "0.04166666666666666666666666666663",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa",
        Less,
    );
    test("5.0", "0x5.0", 1, Up, "0.062", "0x0.1", Greater);
    test(
        "5.0",
        "0x5.0",
        100,
        Up,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test("5.0", "0x5.0", 1, Nearest, "0.031", "0x0.08", Less);
    test(
        "5.0",
        "0x5.0",
        100,
        Nearest,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test("10.0", "0xa.0", 1, Floor, "1.9e-6", "0x0.00002", Less);
    test(
        "10.0",
        "0xa.0",
        100,
        Floor,
        "2.755731922398589065255731922399e-6",
        "0x0.00002e3bc74aad8e671f5583911ca",
        Less,
    );
    test("10.0", "0xa.0", 1, Ceiling, "3.8e-6", "0x0.00004", Greater);
    test(
        "10.0",
        "0xa.0",
        100,
        Ceiling,
        "2.7557319223985890652557319224016e-6",
        "0x0.00002e3bc74aad8e671f5583911ca4",
        Greater,
    );
    test("10.0", "0xa.0", 1, Down, "1.9e-6", "0x0.00002", Less);
    test(
        "10.0",
        "0xa.0",
        100,
        Down,
        "2.755731922398589065255731922399e-6",
        "0x0.00002e3bc74aad8e671f5583911ca",
        Less,
    );
    test("10.0", "0xa.0", 1, Up, "3.8e-6", "0x0.00004", Greater);
    test(
        "10.0",
        "0xa.0",
        100,
        Up,
        "2.7557319223985890652557319224016e-6",
        "0x0.00002e3bc74aad8e671f5583911ca4",
        Greater,
    );
    test("10.0", "0xa.0", 1, Nearest, "1.9e-6", "0x0.00002", Less);
    test(
        "10.0",
        "0xa.0",
        100,
        Nearest,
        "2.755731922398589065255731922399e-6",
        "0x0.00002e3bc74aad8e671f5583911ca",
        Less,
    );
    test("25.0", "0x19.0", 1, Floor, "8.3e-25", "0x1.0E-20", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Floor,
        "1.611737571096118349048713304800e-24",
        "0x1.f2cf01972f577cca4b4067ca8E-20",
        Less,
    );
    test(
        "25.0",
        "0x19.0",
        1,
        Ceiling,
        "1.7e-24",
        "0x2.0E-20",
        Greater,
    );
    test(
        "25.0",
        "0x19.0",
        100,
        Ceiling,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20",
        Greater,
    );
    test("25.0", "0x19.0", 1, Down, "8.3e-25", "0x1.0E-20", Less);
    test(
        "25.0",
        "0x19.0",
        100,
        Down,
        "1.611737571096118349048713304800e-24",
        "0x1.f2cf01972f577cca4b4067ca8E-20",
        Less,
    );
    test("25.0", "0x19.0", 1, Up, "1.7e-24", "0x2.0E-20", Greater);
    test(
        "25.0",
        "0x19.0",
        100,
        Up,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20",
        Greater,
    );
    test(
        "25.0",
        "0x19.0",
        1,
        Nearest,
        "1.7e-24",
        "0x2.0E-20",
        Greater,
    );
    test(
        "25.0",
        "0x19.0",
        100,
        Nearest,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20",
        Greater,
    );
    test("171.0", "0xab.0", 1, Floor, "8.9e-308", "0x1.0E-255", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Floor,
        "1.377900967791770586745593486602e-307",
        "0x1.8c53af9080a2c005d70793c88E-255",
        Less,
    );
    test(
        "171.0",
        "0xab.0",
        1,
        Ceiling,
        "1.8e-307",
        "0x2.0E-255",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        100,
        Ceiling,
        "1.3779009677917705867455934866037e-307",
        "0x1.8c53af9080a2c005d70793c8aE-255",
        Greater,
    );
    test("171.0", "0xab.0", 1, Down, "8.9e-308", "0x1.0E-255", Less);
    test(
        "171.0",
        "0xab.0",
        100,
        Down,
        "1.377900967791770586745593486602e-307",
        "0x1.8c53af9080a2c005d70793c88E-255",
        Less,
    );
    test("171.0", "0xab.0", 1, Up, "1.8e-307", "0x2.0E-255", Greater);
    test(
        "171.0",
        "0xab.0",
        100,
        Up,
        "1.3779009677917705867455934866037e-307",
        "0x1.8c53af9080a2c005d70793c8aE-255",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        1,
        Nearest,
        "1.8e-307",
        "0x2.0E-255",
        Greater,
    );
    test(
        "171.0",
        "0xab.0",
        100,
        Nearest,
        "1.3779009677917705867455934866037e-307",
        "0x1.8c53af9080a2c005d70793c8aE-255",
        Greater,
    );
    test("0.50", "0x0.8", 1, Floor, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Floor,
        "0.5641895835477562869480794515602",
        "0x0.906eba8214db688d71d48a7f6",
        Less,
    );
    test("0.50", "0x0.8", 1, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Ceiling,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7",
        Greater,
    );
    test("0.50", "0x0.8", 1, Down, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Down,
        "0.5641895835477562869480794515602",
        "0x0.906eba8214db688d71d48a7f6",
        Less,
    );
    test("0.50", "0x0.8", 1, Up, "1.0", "0x1.0", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Up,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7",
        Greater,
    );
    test("1.5", "0x1.8", 1, Floor, "1.0", "0x1.0", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Floor,
        "1.128379167095512573896158903120",
        "0x1.20dd750429b6d11ae3a914fec",
        Less,
    );
    test("1.5", "0x1.8", 1, Ceiling, "2.0", "0x2.0", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Ceiling,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee",
        Greater,
    );
    test("1.5", "0x1.8", 1, Down, "1.0", "0x1.0", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Down,
        "1.128379167095512573896158903120",
        "0x1.20dd750429b6d11ae3a914fec",
        Less,
    );
    test("1.5", "0x1.8", 1, Up, "2.0", "0x2.0", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Up,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee",
        Greater,
    );
    test("1.5", "0x1.8", 1, Nearest, "1.0", "0x1.0", Less);
    test(
        "1.5",
        "0x1.8",
        100,
        Nearest,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee",
        Greater,
    );
    test("0.25", "0x0.4", 1, Floor, "0.25", "0x0.4", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Floor,
        "0.2758156628302093143599455399882",
        "0x0.469bdaf3948f50844c4a13ceb",
        Less,
    );
    test("0.25", "0x0.4", 1, Ceiling, "0.50", "0x0.8", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Ceiling,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8",
        Greater,
    );
    test("0.25", "0x0.4", 1, Down, "0.25", "0x0.4", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Down,
        "0.2758156628302093143599455399882",
        "0x0.469bdaf3948f50844c4a13ceb",
        Less,
    );
    test("0.25", "0x0.4", 1, Up, "0.50", "0x0.8", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Up,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8",
        Greater,
    );
    test("0.25", "0x0.4", 1, Nearest, "0.25", "0x0.4", Less);
    test(
        "0.25",
        "0x0.4",
        100,
        Nearest,
        "0.2758156628302093143599455399882",
        "0x0.469bdaf3948f50844c4a13ceb",
        Less,
    );
    test("0.75", "0x0.c", 1, Floor, "0.50", "0x0.8", Less);
    test(
        "0.75",
        "0x0.c",
        100,
        Floor,
        "0.8160489390982629810770859473506",
        "0x0.d0e895515ccf1bee40557ccfc",
        Less,
    );
    test("0.75", "0x0.c", 1, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Ceiling,
        "0.81604893909826298107708594735135",
        "0x0.d0e895515ccf1bee40557ccfd",
        Greater,
    );
    test("0.75", "0x0.c", 1, Down, "0.50", "0x0.8", Less);
    test(
        "0.75",
        "0x0.c",
        100,
        Down,
        "0.8160489390982629810770859473506",
        "0x0.d0e895515ccf1bee40557ccfc",
        Less,
    );
    test("0.75", "0x0.c", 1, Up, "1.0", "0x1.0", Greater);
    test(
        "0.75",
        "0x0.c",
        100,
        Up,
        "0.81604893909826298107708594735135",
        "0x0.d0e895515ccf1bee40557ccfd",
        Greater,
    );
    test("0.75", "0x0.c", 1, Nearest, "1.0", "0x1.0", Greater);
    // - gamma_core positive
    // - gamma_exp_encl negated
    // - gamma_helper loop for positive x
    // - gamma_ln_bits e = 0
    // - ln_gamma_shift e <= 0
    // - stirling_mu_encl term below tolerance
    test(
        "0.75",
        "0x0.c",
        100,
        Nearest,
        "0.8160489390982629810770859473506",
        "0x0.d0e895515ccf1bee40557ccfc",
        Less,
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
    // - reciprocal_gamma_tiny not tiny
    // - stirling_mu_encl closed at even k
    // - stirling_mu_encl closed at odd k
    // - stirling_mu_encl out of terms
    // - ziv_loop_certified_from retry
    test(
        "-291.25",
        "-0x123.4",
        1390,
        Floor,
        "1.6325546391647837662658444328369229141125427671858320296257068578216854936882015375166308\
        8866878168185923975752675786571225008880196576496256404638903129892042526105704566788614305\
        9984056321971870712366217514110442038232236336449029895604339846125789005548986033336329240\
        2167954633829685906537226758413534141189380232391649184980257452033583453108740348914212017\
        7160155408738518187258182912385907220215549796772115271835e592",
        "0x9.c579a89d168ce144dbc2746972f18af6a01d7da98f485839c03dd54238e96ea403a27804b42f07f192ac99\
        67b32180ccfadd142cdcbe61b3422ad432ffef51045faeb90f76693213b0a9ef7029fb05c9368182ad0dd4dbe91\
        600fccc3156752d0bf0d08674cd88a9e3ced70c5eb27eee423d4ac261f258b31b238236676ecb9e701dfc0cbca6\
        75b26b657cf821a3deab23aeae87bc9338a86f5649650ed3b8296496d8a9268094caa26395741c8E+491",
        Less,
    );
    // - gamma_approx negative value
    // - gamma_core negative with e <= 0
    test("-0.062", "-0x0.1", 1, Up, "-0.062", "-0x0.1", Less);
    // - gamma_ln_bits e < 0
    test("0.062", "0x0.1", 1, Up, "0.12", "0x0.2", Greater);
    // - gamma_helper negative integer
    test("-1.0", "-0x1.0", 1, Up, "0.0", "0x0.0", Equal);
    // - gamma_helper large positive integer
    test(
        "3348.0",
        "0xd14.0",
        1,
        Up,
        "3.2e-10346",
        "0x2.0E-8592",
        Greater,
    );
    // - gamma_helper tiny test succeeds
    test(
        "3.2e-27",
        "0x1.0E-22",
        1,
        Up,
        "6.5e-27",
        "0x2.0E-22",
        Greater,
    );
}

#[test]
fn test_reciprocal_gamma_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = reciprocal_gamma_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
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
    test("3", 10, Floor, "0.50", "0x0.8", Equal);
    test("3", 10, Ceiling, "0.50", "0x0.8", Equal);
    test("3", 10, Down, "0.50", "0x0.8", Equal);
    test("3", 10, Up, "0.50", "0x0.8", Equal);
    test("3", 10, Nearest, "0.50", "0x0.8", Equal);
    test("3", 10, Exact, "0.50", "0x0.8", Equal);
    test("5", 10, Floor, "0.04163", "0x0.0aa8", Less);
    test("5", 10, Ceiling, "0.041687", "0x0.0aac", Greater);
    test("5", 10, Down, "0.04163", "0x0.0aa8", Less);
    test("5", 10, Up, "0.041687", "0x0.0aac", Greater);
    test("5", 10, Nearest, "0.041687", "0x0.0aac", Greater);
    test("25", 10, Floor, "1.6107e-24", "0x1.f28E-20", Less);
    test("25", 10, Ceiling, "1.612e-24", "0x1.f3E-20", Greater);
    test("25", 10, Down, "1.6107e-24", "0x1.f28E-20", Less);
    test("25", 10, Up, "1.612e-24", "0x1.f3E-20", Greater);
    test("25", 10, Nearest, "1.612e-24", "0x1.f3E-20", Greater);
    test("1/2", 10, Floor, "0.56348", "0x0.904", Less);
    test("1/2", 10, Ceiling, "0.5645", "0x0.908", Greater);
    test("1/2", 10, Down, "0.56348", "0x0.904", Less);
    test("1/2", 10, Up, "0.5645", "0x0.908", Greater);
    test("1/2", 10, Nearest, "0.5645", "0x0.908", Greater);
    test("1/3", 10, Floor, "0.3730", "0x0.5f8", Less);
    test("1/3", 10, Ceiling, "0.37354", "0x0.5fa", Greater);
    test("1/3", 10, Down, "0.3730", "0x0.5f8", Less);
    test("1/3", 10, Up, "0.37354", "0x0.5fa", Greater);
    test("1/3", 10, Nearest, "0.3730", "0x0.5f8", Less);
    test("1/4", 10, Floor, "0.2754", "0x0.468", Less);
    test("1/4", 10, Ceiling, "0.27588", "0x0.46a", Greater);
    test("1/4", 10, Down, "0.2754", "0x0.468", Less);
    test("1/4", 10, Up, "0.27588", "0x0.46a", Greater);
    test("1/4", 10, Nearest, "0.27588", "0x0.46a", Greater);
    test("1/6", 10, Floor, "0.17944", "0x0.2df", Less);
    test("1/6", 10, Ceiling, "0.180", "0x0.2e", Greater);
    test("1/6", 10, Down, "0.17944", "0x0.2df", Less);
    test("1/6", 10, Up, "0.180", "0x0.2e", Greater);
    test("1/6", 10, Nearest, "0.180", "0x0.2e", Greater);
    test("2/3", 10, Floor, "0.7383", "0x0.bd", Less);
    test("2/3", 10, Ceiling, "0.73926", "0x0.bd4", Greater);
    test("2/3", 10, Down, "0.7383", "0x0.bd", Less);
    test("2/3", 10, Up, "0.73926", "0x0.bd4", Greater);
    test("2/3", 10, Nearest, "0.7383", "0x0.bd", Less);
    test("-1/2", 10, Floor, "-0.2822", "-0x0.484", Less);
    test("-1/2", 10, Ceiling, "-0.28174", "-0x0.482", Greater);
    test("-1/2", 10, Down, "-0.28174", "-0x0.482", Greater);
    test("-1/2", 10, Up, "-0.2822", "-0x0.484", Less);
    test("-1/2", 10, Nearest, "-0.2822", "-0x0.484", Less);
    test("-1", 10, Floor, "0.0", "0x0.0", Equal);
    test("-1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1", 10, Down, "0.0", "0x0.0", Equal);
    test("-1", 10, Up, "0.0", "0x0.0", Equal);
    // - gamma_rational_helper pole
    test("-1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-2", 10, Floor, "0.0", "0x0.0", Equal);
    test("-2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-2", 10, Down, "0.0", "0x0.0", Equal);
    test("-2", 10, Up, "0.0", "0x0.0", Equal);
    test("-2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-2", 10, Exact, "0.0", "0x0.0", Equal);
    test("-5/2", 10, Floor, "-1.059", "-0x1.0f", Less);
    test("-5/2", 10, Ceiling, "-1.0566", "-0x1.0e8", Greater);
    test("-5/2", 10, Down, "-1.0566", "-0x1.0e8", Greater);
    test("-5/2", 10, Up, "-1.059", "-0x1.0f", Less);
    test("-5/2", 10, Nearest, "-1.059", "-0x1.0f", Less);
    test("-7/3", 10, Floor, "-0.76660", "-0x0.c44", Less);
    test("-7/3", 10, Ceiling, "-0.766", "-0x0.c4", Greater);
    test("-7/3", 10, Down, "-0.766", "-0x0.c4", Greater);
    test("-7/3", 10, Up, "-0.76660", "-0x0.c44", Less);
    test("-7/3", 10, Nearest, "-0.766", "-0x0.c4", Greater);
    test("22/7", 10, Floor, "0.43604", "0x0.6fa", Less);
    test("22/7", 10, Ceiling, "0.4365", "0x0.6fc", Greater);
    test("22/7", 10, Down, "0.43604", "0x0.6fa", Less);
    test("22/7", 10, Up, "0.4365", "0x0.6fc", Greater);
    test("22/7", 10, Nearest, "0.4365", "0x0.6fc", Greater);
    test(
        "1000001/1000",
        10,
        Floor,
        "2.4658e-2565",
        "0x1.788E-2130",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Ceiling,
        "2.469e-2565",
        "0x1.79E-2130",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Down,
        "2.4658e-2565",
        "0x1.788E-2130",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Up,
        "2.469e-2565",
        "0x1.79E-2130",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Nearest,
        "2.469e-2565",
        "0x1.79E-2130",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Floor,
        "-4.054e2564",
        "-0xa.eE+2129",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Ceiling,
        "-4.0481e2564",
        "-0xa.dcE+2129",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Down,
        "-4.0481e2564",
        "-0xa.dcE+2129",
        Greater,
    );
    test("-1000001/1000", 10, Up, "-4.054e2564", "-0xa.eE+2129", Less);
    test(
        "-1000001/1000",
        10,
        Nearest,
        "-4.054e2564",
        "-0xa.eE+2129",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "7.9e-31",
        "0x1.0E-25",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "7.9040e-31",
        "0x1.008E-25",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "7.9e-31",
        "0x1.0E-25",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "7.9040e-31",
        "0x1.008E-25",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Floor,
        "-7.9e-31",
        "-0x1.0E-25",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Ceiling,
        "-7.8809e-31",
        "-0xf.fcE-26",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Down,
        "-7.8809e-31",
        "-0xf.fcE-26",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Up,
        "-7.9e-31",
        "-0x1.0E-25",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Nearest,
        "-7.9e-31",
        "-0x1.0E-25",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "100000000",
        10,
        Floor,
        "6.177e-756570549",
        "0x1.1aE-628318240",
        Less,
    );
    test(
        "100000000",
        10,
        Ceiling,
        "6.1884e-756570549",
        "0x1.1a8E-628318240",
        Greater,
    );
    test(
        "100000000",
        10,
        Down,
        "6.177e-756570549",
        "0x1.1aE-628318240",
        Less,
    );
    test(
        "100000000",
        10,
        Up,
        "6.1884e-756570549",
        "0x1.1a8E-628318240",
        Greater,
    );
    test(
        "100000000",
        10,
        Nearest,
        "6.1884e-756570549",
        "0x1.1a8E-628318240",
        Greater,
    );
    test(
        "200000001/2",
        10,
        Floor,
        "6.177e-756570553",
        "0x7.38E-628318244",
        Less,
    );
    test(
        "200000001/2",
        10,
        Ceiling,
        "6.1837e-756570553",
        "0x7.3aE-628318244",
        Greater,
    );
    test(
        "200000001/2",
        10,
        Down,
        "6.177e-756570553",
        "0x7.38E-628318244",
        Less,
    );
    test(
        "200000001/2",
        10,
        Up,
        "6.1837e-756570553",
        "0x7.3aE-628318244",
        Greater,
    );
    test(
        "200000001/2",
        10,
        Nearest,
        "6.1837e-756570553",
        "0x7.3aE-628318244",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Floor,
        "-5.151e756570559",
        "-0x4.34E+628318249",
        Less,
    );
    test(
        "-200000001/2",
        10,
        Ceiling,
        "-5.1410e756570559",
        "-0x4.32E+628318249",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Down,
        "-5.1410e756570559",
        "-0x4.32E+628318249",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Up,
        "-5.151e756570559",
        "-0x4.34E+628318249",
        Less,
    );
    test(
        "-200000001/2",
        10,
        Nearest,
        "-5.151e756570559",
        "-0x4.34E+628318249",
        Less,
    );
    test(
        "-200000003/2",
        10,
        Floor,
        "5.1462e756570567",
        "0x1.908E+628318256",
        Less,
    );
    test(
        "-200000003/2",
        10,
        Ceiling,
        "5.153e756570567",
        "0x1.91E+628318256",
        Greater,
    );
    test(
        "-200000003/2",
        10,
        Down,
        "5.1462e756570567",
        "0x1.908E+628318256",
        Less,
    );
    test(
        "-200000003/2",
        10,
        Up,
        "5.153e756570567",
        "0x1.91E+628318256",
        Greater,
    );
    test(
        "-200000003/2",
        10,
        Nearest,
        "5.1462e756570567",
        "0x1.908E+628318256",
        Less,
    );
    test("0", 100, Floor, "0.0", "0x0.0", Equal);
    test("0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 100, Down, "0.0", "0x0.0", Equal);
    test("0", 100, Up, "0.0", "0x0.0", Equal);
    test("0", 100, Nearest, "0.0", "0x0.0", Equal);
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
    test("3", 100, Floor, "0.50", "0x0.8", Equal);
    test("3", 100, Ceiling, "0.50", "0x0.8", Equal);
    test("3", 100, Down, "0.50", "0x0.8", Equal);
    test("3", 100, Up, "0.50", "0x0.8", Equal);
    test("3", 100, Nearest, "0.50", "0x0.8", Equal);
    test(
        "5",
        100,
        Floor,
        "0.04166666666666666666666666666663",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa",
        Less,
    );
    test(
        "5",
        100,
        Ceiling,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test(
        "5",
        100,
        Down,
        "0.04166666666666666666666666666663",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa",
        Less,
    );
    test(
        "5",
        100,
        Up,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test(
        "5",
        100,
        Nearest,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test(
        "25",
        100,
        Floor,
        "1.611737571096118349048713304800e-24",
        "0x1.f2cf01972f577cca4b4067ca8E-20",
        Less,
    );
    test(
        "25",
        100,
        Ceiling,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20",
        Greater,
    );
    test(
        "25",
        100,
        Down,
        "1.611737571096118349048713304800e-24",
        "0x1.f2cf01972f577cca4b4067ca8E-20",
        Less,
    );
    test(
        "25",
        100,
        Up,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20",
        Greater,
    );
    test(
        "25",
        100,
        Nearest,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20",
        Greater,
    );
    test(
        "1/2",
        100,
        Floor,
        "0.5641895835477562869480794515602",
        "0x0.906eba8214db688d71d48a7f6",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "0.5641895835477562869480794515602",
        "0x0.906eba8214db688d71d48a7f6",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7",
        Greater,
    );
    test(
        "1/3",
        100,
        Floor,
        "0.3732821739073952283263503124231",
        "0x0.5f8f6ba91caf1d5cb7b2a514a",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.3732821739073952283263503124231",
        "0x0.5f8f6ba91caf1d5cb7b2a514a",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8",
        Greater,
    );
    test(
        "1/4",
        100,
        Floor,
        "0.2758156628302093143599455399882",
        "0x0.469bdaf3948f50844c4a13ceb",
        Less,
    );
    test(
        "1/4",
        100,
        Ceiling,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8",
        Greater,
    );
    test(
        "1/4",
        100,
        Down,
        "0.2758156628302093143599455399882",
        "0x0.469bdaf3948f50844c4a13ceb",
        Less,
    );
    test(
        "1/4",
        100,
        Up,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8",
        Greater,
    );
    test(
        "1/4",
        100,
        Nearest,
        "0.2758156628302093143599455399882",
        "0x0.469bdaf3948f50844c4a13ceb",
        Less,
    );
    test(
        "1/6",
        100,
        Floor,
        "0.1796520355078973457262671951828",
        "0x0.2dfdad012a8f59a314ad4a6878",
        Less,
    );
    test(
        "1/6",
        100,
        Ceiling,
        "0.17965203550789734572626719518301",
        "0x0.2dfdad012a8f59a314ad4a687c",
        Greater,
    );
    test(
        "1/6",
        100,
        Down,
        "0.1796520355078973457262671951828",
        "0x0.2dfdad012a8f59a314ad4a6878",
        Less,
    );
    test(
        "1/6",
        100,
        Up,
        "0.17965203550789734572626719518301",
        "0x0.2dfdad012a8f59a314ad4a687c",
        Greater,
    );
    test(
        "1/6",
        100,
        Nearest,
        "0.1796520355078973457262671951828",
        "0x0.2dfdad012a8f59a314ad4a6878",
        Less,
    );
    test(
        "2/3",
        100,
        Floor,
        "0.7384881116216483129357543751642",
        "0x0.bd0d8e8fe657c99422d13c76e",
        Less,
    );
    test(
        "2/3",
        100,
        Ceiling,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f",
        Greater,
    );
    test(
        "2/3",
        100,
        Down,
        "0.7384881116216483129357543751642",
        "0x0.bd0d8e8fe657c99422d13c76e",
        Less,
    );
    test(
        "2/3",
        100,
        Up,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f",
        Greater,
    );
    test(
        "2/3",
        100,
        Nearest,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f",
        Greater,
    );
    test(
        "-1/2",
        100,
        Floor,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8",
        Less,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-0.2820947917738781434740397257801",
        "-0x0.48375d410a6db446b8ea453fb",
        Greater,
    );
    test(
        "-1/2",
        100,
        Down,
        "-0.2820947917738781434740397257801",
        "-0x0.48375d410a6db446b8ea453fb",
        Greater,
    );
    test(
        "-1/2",
        100,
        Up,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8",
        Less,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8",
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
        "2.0726254582160486006048818028168541945301010950575880299572974506127901450639315689082685\
        8219687958986378687169148292986944701078054664964398311585915827295864160460830027173519279\
        1920999222041428796322710168780549221590534170933241467692188707641883718241959069264182248\
        4854200431198139898682680688030789205512260212338647686967643797011666677748959736045935333\
        2633026073582921491060014182115973077509287758918692216649399775396897574651061880853293409\
        1914263310335072494129131200973900786002795289449627793197281194697165003461129139419716961\
        1870756195462541616109739001092343810175499495087823040203748016609063941759746647491302499\
        2563258565637922732405221569863169158712607792758702115019949058262598317454322389971588063\
        9740722044978936639566031223574238282892606911776754725567569305542682187892526024631703716\
        6948226986427175597978537743953176362350773332221791043599328365896838122715700609788460889\
        49119705170315554239849256693399990282e1159",
        "0x8.f9ef018b1754d7835816b91ec6642f645e95ad13ae96d89605901a48efdca2e8e332c7f90a97ab8bcd18cc\
        4925af68c9a0dba2e2c57a83d68b32277fb6b474c739c701974c937dfaddb634183e5aed45fe1d40a33426428a7\
        3e19f190f15145e2f47137443d21cd048aa0808ea612e8c318b5276da087e8b1dd5a7186a666b006b2f1c751bc4\
        368fb8715201ed5e4e2af8fd7b54ee657ea7d885a149c2b71b220b11e814da032bac1c8494d1c28981bfb35b4cc\
        b76ed5f69eacaf3ae8750df78819673f51d2edbc03d49d19666939acfb52a72afc6f659038b33826ac63535bf39\
        c71a1aded009d18f869b5b5c1d39baa3f91e13d704c44cc5beec8eb8f73f608e33c37999f7226d9d5ff750c2bea\
        9cc753fb6aba3211f00a66f0d19accd2eadbfe83dfae6b3a517edbc3bd0e3a6e986768e1d846400400bcb0472e9\
        07a3ce9e2a4ff06f24ff30a7d813148394226e5571f36095481238d0197be361e48504b80862c895d19f58548d9\
        e6ec197e68b5aeb77ad9e27dc6a90a5302a1a7a365aa34fc527c8c88a284cE+962",
        Less,
    );
    // - gamma_core positive
    // - gamma_exp_encl negated
    // - gamma_ln_bits e < 0
    // - gamma_rational_helper loop for positive q
    // - ln_gamma_shift e <= 0
    // - stirling_mu_encl term below tolerance
    test(
        "511/9007199254741056",
        2,
        Up,
        "5.7e-14",
        "0x1.0E-11",
        Greater,
    );
    // - gamma_approx negative value
    // - gamma_core negative with e <= 0
    // - gamma_rational_helper tiny test fails
    test(
        "-7/1267650600228229401496703205375",
        17,
        Up,
        "-5.5e-30",
        "-0x7.0E-25",
        Less,
    );
    // - gamma_of_small_integer reciprocal
    // - gamma_rational_helper small positive integer
    test("1", 1, Up, "1.0", "0x1.0", Equal);
    // - gamma_ln_bits e = 0
    test("1/2", 1, Up, "1.0", "0x1.0", Greater);
    // - gamma_rational_helper large positive integer
    test("3348", 1, Up, "3.2e-10346", "0x2.0E-8592", Greater);
    // - gamma_rational_helper tiny test succeeds
    test(
        "1/75557863725914324467712",
        2,
        Up,
        "1.3e-23",
        "0x1.0E-19",
        Greater,
    );
}

#[test]
#[should_panic]
fn reciprocal_gamma_prec_round_fail_1() {
    BigExponentFloat::ONE.reciprocal_gamma_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn reciprocal_gamma_prec_round_fail_2() {
    BigExponentFloat::from(0.5).reciprocal_gamma_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_round_fail_1() {
    BigExponentFloat::reciprocal_gamma_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_round_fail_2() {
    BigExponentFloat::reciprocal_gamma_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn reciprocal_gamma_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = reciprocal_gamma_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if *x > 0u32 {
        // The gamma function is positive on the positive reals; its reciprocal may underflow.
        assert!(r.0 >= 0u32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The reference computes the reciprocal of the gamma function.
        if r.1 != Equal && reference_is_cheap(x) {
            assert!(same_pair(
                &reciprocal_gamma_prec_round_naive(x, prec, rm),
                &r
            ));
        }
        // The result lies between the results rounded down and up.
        let floor = reciprocal_gamma_prec_round_all(x, prec, Floor);
        let ceiling = reciprocal_gamma_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &reciprocal_gamma_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = reciprocal_gamma_prec_round_all(x, prec + 16, rm);
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
            &reciprocal_gamma_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn reciprocal_gamma_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            reciprocal_gamma_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            reciprocal_gamma_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn reciprocal_gamma_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.reciprocal_gamma_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = reciprocal_gamma_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn reciprocal_gamma_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = reciprocal_gamma_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if x > 0u32 {
            // The gamma function is positive on the positive reals; its reciprocal may underflow.
            assert!(r.0 >= 0u32);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            // The reference computes the reciprocal of the gamma function, in time that grows with
            // x, and with twice as many bits as a tiny x has leading zeros.
            if r.1 != Equal && x.lt_abs(&0x100000u32) && x.ge_abs(&Rational::power_of_2(-1000i64)) {
                assert!(same_pair(
                    &reciprocal_gamma_rational_prec_round_naive(&x, prec, rm),
                    &r
                ));
            }
            if rm != Nearest {
                let (y, o) = reciprocal_gamma_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn reciprocal_gamma_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::reciprocal_gamma_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = reciprocal_gamma_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
