// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_15,
    rational_float_unsigned_rounding_mode_quadruple_gen_var_1,
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
        || !x.is_zero() || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn log_base_rational_big_exponent_float_base_prec_round_all(
    x: &Rational,
    b: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
        x.clone(),
        b.clone(),
        prec,
        rm,
    );
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(x, b, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_big_exponent_float_base_prec(
                x.clone(),
                b.clone(),
                prec
            ),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_big_exponent_float_base_prec_ref(x, b, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_rational_big_exponent_float_base_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let test = |x: &str,
                b: &str,
                b_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(b_hex).to_string(), b);
        let r = log_base_rational_big_exponent_float_base_prec_round_all(
            &Rational::from_str(x).unwrap(),
            &hex(b_hex),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    };
    test("3/5", "3.0", "0x3", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("3/5", "3.0", "0x3", 10, Floor, "-0.46533", "-0x0.772", Less);
    test(
        "3/5", "3.0", "0x3", 10, Ceiling, "-0.4648", "-0x0.77", Greater,
    );
    test("3/5", "3.0", "0x3", 10, Down, "-0.4648", "-0x0.77", Greater);
    test("3/5", "3.0", "0x3", 10, Up, "-0.46533", "-0x0.772", Less);
    test(
        "3/5", "3.0", "0x3", 10, Nearest, "-0.4648", "-0x0.77", Greater,
    );
    test(
        "3/5",
        "3.0",
        "0x3",
        100,
        Nearest,
        "-0.46497352071792716719704040767853",
        "-0x0.77088130fd4e46455c3c9ca988",
        Greater,
    );
    test("3/5", "0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test("3/5", "0.50", "0x0.8", 10, Floor, "0.7363", "0x0.bc8", Less);
    test(
        "3/5", "0.50", "0x0.8", 10, Ceiling, "0.73730", "0x0.bcc", Greater,
    );
    test("3/5", "0.50", "0x0.8", 10, Down, "0.7363", "0x0.bc8", Less);
    test(
        "3/5", "0.50", "0x0.8", 10, Up, "0.73730", "0x0.bcc", Greater,
    );
    test(
        "3/5", "0.50", "0x0.8", 10, Nearest, "0.73730", "0x0.bcc", Greater,
    );
    test(
        "3/5",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.73696559416620616641658048554156",
        "0x0.bca9c6f53897a45984a4c9f33",
        Less,
    );
    test("3/5", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("3/5", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
    test("9", "3.0", "0x3", 1, Nearest, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 10, Floor, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 10, Down, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 10, Up, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 10, Nearest, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 100, Nearest, "2.0", "0x2.0", Equal);
    test("9", "3.0", "0x3", 300, Exact, "2.0", "0x2.0", Equal);
    test("9", "0.50", "0x0.8", 1, Nearest, "-4.0", "-0x4.0", Less);
    test("9", "0.50", "0x0.8", 10, Floor, "-3.172", "-0x3.2c", Less);
    test(
        "9", "0.50", "0x0.8", 10, Ceiling, "-3.1680", "-0x3.2b", Greater,
    );
    test(
        "9", "0.50", "0x0.8", 10, Down, "-3.1680", "-0x3.2b", Greater,
    );
    test("9", "0.50", "0x0.8", 10, Up, "-3.172", "-0x3.2c", Less);
    test("9", "0.50", "0x0.8", 10, Nearest, "-3.172", "-0x3.2c", Less);
    test(
        "9",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-3.16992500144231236290747788789",
        "-0x3.2b803473f7ad0f3f40162414",
        Greater,
    );
    test("9", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("9", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
    test("1/9", "3.0", "0x3", 1, Nearest, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 10, Floor, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 10, Ceiling, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 10, Down, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 10, Up, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 10, Nearest, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 100, Nearest, "-2.0", "-0x2.0", Equal);
    test("1/9", "3.0", "0x3", 300, Exact, "-2.0", "-0x2.0", Equal);
    test("1/9", "0.50", "0x0.8", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1/9", "0.50", "0x0.8", 10, Floor, "3.1680", "0x3.2b", Less);
    test(
        "1/9", "0.50", "0x0.8", 10, Ceiling, "3.172", "0x3.2c", Greater,
    );
    test("1/9", "0.50", "0x0.8", 10, Down, "3.1680", "0x3.2b", Less);
    test("1/9", "0.50", "0x0.8", 10, Up, "3.172", "0x3.2c", Greater);
    test(
        "1/9", "0.50", "0x0.8", 10, Nearest, "3.172", "0x3.2c", Greater,
    );
    test(
        "1/9",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "3.16992500144231236290747788789",
        "0x3.2b803473f7ad0f3f40162414",
        Less,
    );
    test("1/9", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("1/9", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
    test("1/3", "3.0", "0x3", 1, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 10, Floor, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 10, Ceiling, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 10, Down, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 10, Up, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 10, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 100, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/3", "3.0", "0x3", 300, Exact, "-1.0", "-0x1.0", Equal);
    test("1/3", "0.50", "0x0.8", 1, Nearest, "2.0", "0x2.0", Greater);
    test("1/3", "0.50", "0x0.8", 10, Floor, "1.5840", "0x1.958", Less);
    test(
        "1/3", "0.50", "0x0.8", 10, Ceiling, "1.586", "0x1.96", Greater,
    );
    test("1/3", "0.50", "0x0.8", 10, Down, "1.5840", "0x1.958", Less);
    test("1/3", "0.50", "0x0.8", 10, Up, "1.586", "0x1.96", Greater);
    test(
        "1/3", "0.50", "0x0.8", 10, Nearest, "1.586", "0x1.96", Greater,
    );
    test(
        "1/3",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "1.58496250072115618145373894395",
        "0x1.95c01a39fbd6879fa00b120a",
        Less,
    );
    test("1/3", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("1/3", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        10,
        Floor,
        "19.94",
        "0x13.f",
        Less,
    );
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "19.969",
        "0x13.f8",
        Greater,
    );
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        10,
        Down,
        "19.94",
        "0x13.f",
        Less,
    );
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        10,
        Up,
        "19.969",
        "0x13.f8",
        Greater,
    );
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        10,
        Nearest,
        "19.969",
        "0x13.f8",
        Greater,
    );
    test(
        "10000000000/3",
        "3.0",
        "0x3",
        100,
        Nearest,
        "19.95903274289384604296567522023",
        "0x13.f5832b7a85b1dbc1c4d0f94c",
        Greater,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "-32.0",
        "-0x2.0E+1",
        Less,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "-31.656",
        "-0x1f.a8",
        Less,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "-31.62",
        "-0x1f.a",
        Greater,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        10,
        Down,
        "-31.62",
        "-0x1f.a",
        Greater,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        10,
        Up,
        "-31.656",
        "-0x1f.a8",
        Less,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        10,
        Nearest,
        "-31.62",
        "-0x1f.a",
        Greater,
    );
    test(
        "10000000000/3",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-31.634318448152467297249455350945",
        "-0x1f.a262b19e1077301bced385da",
        Greater,
    );
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test("10000000000/3", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000/3",
        "1.0",
        "0x1.0",
        300,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("1", "3.0", "0x3", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 10, Down, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 10, Up, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3.0", "0x3", 300, Exact, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 10, Down, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 10, Up, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", "0.50", "0x0.8", 300, Exact, "0.0", "0x0.0", Equal);
    test("1", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("1", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
    test(
        "0",
        "3.0",
        "0x3",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0",
        "3.0",
        "0x3",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0", "3.0", "0x3", 10, Down, "-Infinity", "-Infinity", Equal);
    test("0", "3.0", "0x3", 10, Up, "-Infinity", "-Infinity", Equal);
    test(
        "0",
        "3.0",
        "0x3",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0",
        "3.0",
        "0x3",
        100,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0",
        "3.0",
        "0x3",
        300,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0", "0.50", "0x0.8", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0", "0.50", "0x0.8", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "0", "0.50", "0x0.8", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "0", "0.50", "0x0.8", 10, Down, "Infinity", "Infinity", Equal,
    );
    test("0", "0.50", "0x0.8", 10, Up, "Infinity", "Infinity", Equal);
    test(
        "0", "0.50", "0x0.8", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0", "0.50", "0x0.8", 100, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0", "0.50", "0x0.8", 300, Exact, "Infinity", "Infinity", Equal,
    );
    test("0", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("0", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 10, Floor, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 10, Down, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 10, Up, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", "3.0", "0x3", 300, Exact, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 10, Down, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 10, Up, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", "0.50", "0x0.8", 300, Exact, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn log_base_rational_big_exponent_float_base_prec_round_fail_1() {
    BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
        Rational::from_unsigneds(1u8, 3),
        BigExponentFloat::from(3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn log_base_rational_big_exponent_float_base_prec_round_fail_2() {
    BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
        Rational::from_unsigneds(1u8, 5),
        BigExponentFloat::from(3),
        10,
        Exact,
    );
}

fn log_base_rational_big_exponent_float_base_prec_round_properties_helper(
    x: &Rational,
    b: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_rational_big_exponent_float_base_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_rational_big_exponent_float_base_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_rational_big_exponent_float_base_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_rational_big_exponent_float_base_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) =
                log_base_rational_big_exponent_float_base_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A dyadic `Rational` gives the same result as the equal `BigExponentFloat`.
    if let Ok(y) = BigExponentFloat::try_from(x) {
        assert!(same_pair(
            &y.log_base_big_exponent_float_base_prec_round_ref(b, prec, rm),
            &r
        ));
    }
    // A base of 1 gives NaN, as in `arf_div`, since ln(1) = 0 and there is no signed zero.
    if *b == 1u32 {
        assert!(r.0.is_nan());
    }
}

#[test]
fn log_base_rational_big_exponent_float_base_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_15().test_properties(
        |(b, x, prec, rm)| {
            log_base_rational_big_exponent_float_base_prec_round_properties_helper(
                &x, &b, prec, rm,
            );
        },
    );
}

#[test]
fn log_base_rational_big_exponent_float_base_prec_round_float_agreement_properties() {
    rational_float_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = Float::log_base_rational_float_base_prec_round_ref(&x, &b, prec, rm);
            if float_result_comparable(&f, fo) && b != 1u32 {
                let r = log_base_rational_big_exponent_float_base_prec_round_all(
                    &x.clone(),
                    &float_to_big_exponent_float(&b),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
