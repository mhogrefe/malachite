// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_12,
    float_float_unsigned_rounding_mode_quadruple_gen_var_13,
};
use malachite_float::{BigExponentFloat, Float};
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

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn log_base_big_exponent_float_base_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    b: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .log_base_big_exponent_float_base_1_plus_x_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_big_exponent_float_base_1_plus_x_prec_round_ref(b, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_big_exponent_float_base_1_plus_x_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone()
                .log_base_big_exponent_float_base_1_plus_x_prec(b, prec),
            &r
        ));
        assert!(same_pair(
            &x.log_base_big_exponent_float_base_1_plus_x_prec_ref(b, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.log_base_big_exponent_float_base_1_plus_x_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_big_exponent_float_base_1_plus_x_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let test = |x: &str,
                x_hex: &str,
                b: &str,
                b_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        assert_eq!(hex(b_hex).to_string(), b);
        let r = log_base_big_exponent_float_base_1_plus_x_prec_round_all(
            &hex(x_hex),
            &hex(b_hex),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.293",
        "0x1.4b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.2949",
        "0x1.4b8",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        10,
        Down,
        "1.293",
        "0x1.4b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        10,
        Up,
        "1.2949",
        "0x1.4b8",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.293",
        "0x1.4b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.2935231359164237386853033768279",
        "0x1.4b24550d616216685f5c9e1f6",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "-2.0508",
        "-0x2.0d",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "-2.047",
        "-0x2.0c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        10,
        Down,
        "-2.047",
        "-0x2.0c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        10,
        Up,
        "-2.0508",
        "-0x2.0d",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        10,
        Nearest,
        "-2.0508",
        "-0x2.0d",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-2.050185664242766965245559053624",
        "-0x2.0cd8f7baa695a2146d5d1f398",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1.0",
        "0x1.0",
        300,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.50", "0x0.8", "3.0", "0x3", 1, Nearest, "0.25", "0x0.4", Less,
    );
    test(
        "0.50", "0x0.8", "3.0", "0x3", 10, Floor, "0.36865", "0x0.5e6", Less,
    );
    test(
        "0.50", "0x0.8", "3.0", "0x3", 10, Ceiling, "0.3691", "0x0.5e8", Greater,
    );
    test(
        "0.50", "0x0.8", "3.0", "0x3", 10, Down, "0.36865", "0x0.5e6", Less,
    );
    test(
        "0.50", "0x0.8", "3.0", "0x3", 10, Up, "0.3691", "0x0.5e8", Greater,
    );
    test(
        "0.50", "0x0.8", "3.0", "0x3", 10, Nearest, "0.3691", "0x0.5e8", Greater,
    );
    test(
        "0.50",
        "0x0.8",
        "3.0",
        "0x3",
        100,
        Nearest,
        "0.3690702464285425629004728856573",
        "0x0.5e7b633e565616b1fbc150887",
        Greater,
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", 1, Nearest, "-0.50", "-0x0.8", Greater,
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", 10, Floor, "-0.5859", "-0x0.96", Less,
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", 10, Ceiling, "-0.58496", "-0x0.95c", Greater,
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", 10, Down, "-0.58496", "-0x0.95c", Greater,
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", 10, Up, "-0.5859", "-0x0.96", Less,
    );
    test(
        "0.50", "0x0.8", "0.50", "0x0.8", 10, Nearest, "-0.58496", "-0x0.95c", Greater,
    );
    test(
        "0.50",
        "0x0.8",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-0.584962500721156181453738943947",
        "-0x0.95c01a39fbd6879fa00b120a",
        Greater,
    );
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal,
    );
    test("0.50", "0x0.8", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.50", "0x0.8", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "3.0", "0x3", 1, Nearest, "-0.50", "-0x0.8", Greater,
    );
    test(
        "-0.50", "-0x0.8", "3.0", "0x3", 10, Floor, "-0.63184", "-0x0.a1c", Less,
    );
    test(
        "-0.50", "-0x0.8", "3.0", "0x3", 10, Ceiling, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "-0.50", "-0x0.8", "3.0", "0x3", 10, Down, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "-0.50", "-0x0.8", "3.0", "0x3", 10, Up, "-0.63184", "-0x0.a1c", Less,
    );
    test(
        "-0.50", "-0x0.8", "3.0", "0x3", 10, Nearest, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "-0.50",
        "-0x0.8",
        "3.0",
        "0x3",
        100,
        Nearest,
        "-0.63092975357145743709952711434266",
        "-0x0.a1849cc1a9a9e94e043eaf779",
        Greater,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 1, Nearest, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 10, Floor, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 10, Ceiling, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 10, Down, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 10, Up, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 10, Nearest, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 100, Nearest, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "0.50", "0x0.8", 300, Exact, "1.0", "0x1.0", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.50", "-0x0.8", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "2.0", "0x2", "3.0", "0x3", 1, Nearest, "1.0", "0x1.0", Equal,
    );
    test("2.0", "0x2", "3.0", "0x3", 10, Floor, "1.0", "0x1.0", Equal);
    test(
        "2.0", "0x2", "3.0", "0x3", 10, Ceiling, "1.0", "0x1.0", Equal,
    );
    test("2.0", "0x2", "3.0", "0x3", 10, Down, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3.0", "0x3", 10, Up, "1.0", "0x1.0", Equal);
    test(
        "2.0", "0x2", "3.0", "0x3", 10, Nearest, "1.0", "0x1.0", Equal,
    );
    test(
        "2.0", "0x2", "3.0", "0x3", 100, Nearest, "1.0", "0x1.0", Equal,
    );
    test(
        "2.0", "0x2", "3.0", "0x3", 300, Exact, "1.0", "0x1.0", Equal,
    );
    test(
        "2.0", "0x2", "0.50", "0x0.8", 1, Nearest, "-2.0", "-0x2.0", Less,
    );
    test(
        "2.0", "0x2", "0.50", "0x0.8", 10, Floor, "-1.586", "-0x1.96", Less,
    );
    test(
        "2.0", "0x2", "0.50", "0x0.8", 10, Ceiling, "-1.5840", "-0x1.958", Greater,
    );
    test(
        "2.0", "0x2", "0.50", "0x0.8", 10, Down, "-1.5840", "-0x1.958", Greater,
    );
    test(
        "2.0", "0x2", "0.50", "0x0.8", 10, Up, "-1.586", "-0x1.96", Less,
    );
    test(
        "2.0", "0x2", "0.50", "0x0.8", 10, Nearest, "-1.586", "-0x1.96", Less,
    );
    test(
        "2.0",
        "0x2",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-1.58496250072115618145373894395",
        "-0x1.95c01a39fbd6879fa00b120a",
        Greater,
    );
    test(
        "2.0", "0x2", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("2.0", "0x2", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test(
        "2.0", "0x2", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test("2.0", "0x2", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("2.0", "0x2", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "2.0", "0x2", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "2.0", "0x2", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "2.0", "0x2", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "8.0", "0x8", "3.0", "0x3", 1, Nearest, "2.0", "0x2.0", Equal,
    );
    test("8.0", "0x8", "3.0", "0x3", 10, Floor, "2.0", "0x2.0", Equal);
    test(
        "8.0", "0x8", "3.0", "0x3", 10, Ceiling, "2.0", "0x2.0", Equal,
    );
    test("8.0", "0x8", "3.0", "0x3", 10, Down, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3.0", "0x3", 10, Up, "2.0", "0x2.0", Equal);
    test(
        "8.0", "0x8", "3.0", "0x3", 10, Nearest, "2.0", "0x2.0", Equal,
    );
    test(
        "8.0", "0x8", "3.0", "0x3", 100, Nearest, "2.0", "0x2.0", Equal,
    );
    test(
        "8.0", "0x8", "3.0", "0x3", 300, Exact, "2.0", "0x2.0", Equal,
    );
    test(
        "8.0", "0x8", "0.50", "0x0.8", 1, Nearest, "-4.0", "-0x4.0", Less,
    );
    test(
        "8.0", "0x8", "0.50", "0x0.8", 10, Floor, "-3.172", "-0x3.2c", Less,
    );
    test(
        "8.0", "0x8", "0.50", "0x0.8", 10, Ceiling, "-3.1680", "-0x3.2b", Greater,
    );
    test(
        "8.0", "0x8", "0.50", "0x0.8", 10, Down, "-3.1680", "-0x3.2b", Greater,
    );
    test(
        "8.0", "0x8", "0.50", "0x0.8", 10, Up, "-3.172", "-0x3.2c", Less,
    );
    test(
        "8.0", "0x8", "0.50", "0x0.8", 10, Nearest, "-3.172", "-0x3.2c", Less,
    );
    test(
        "8.0",
        "0x8",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-3.16992500144231236290747788789",
        "-0x3.2b803473f7ad0f3f40162414",
        Greater,
    );
    test(
        "8.0", "0x8", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("8.0", "0x8", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test(
        "8.0", "0x8", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test("8.0", "0x8", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("8.0", "0x8", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "8.0", "0x8", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "8.0", "0x8", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "8.0", "0x8", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "26.0", "0x1a", "3.0", "0x3", 1, Nearest, "4.0", "0x4.0", Greater,
    );
    test(
        "26.0", "0x1a", "3.0", "0x3", 10, Floor, "3.0", "0x3.0", Equal,
    );
    test(
        "26.0", "0x1a", "3.0", "0x3", 10, Ceiling, "3.0", "0x3.0", Equal,
    );
    test(
        "26.0", "0x1a", "3.0", "0x3", 10, Down, "3.0", "0x3.0", Equal,
    );
    test("26.0", "0x1a", "3.0", "0x3", 10, Up, "3.0", "0x3.0", Equal);
    test(
        "26.0", "0x1a", "3.0", "0x3", 10, Nearest, "3.0", "0x3.0", Equal,
    );
    test(
        "26.0", "0x1a", "3.0", "0x3", 100, Nearest, "3.0", "0x3.0", Equal,
    );
    test(
        "26.0", "0x1a", "3.0", "0x3", 300, Exact, "3.0", "0x3.0", Equal,
    );
    test(
        "26.0", "0x1a", "0.50", "0x0.8", 1, Nearest, "-4.0", "-0x4.0", Greater,
    );
    test(
        "26.0", "0x1a", "0.50", "0x0.8", 10, Floor, "-4.7578", "-0x4.c2", Less,
    );
    test(
        "26.0", "0x1a", "0.50", "0x0.8", 10, Ceiling, "-4.75", "-0x4.c", Greater,
    );
    test(
        "26.0", "0x1a", "0.50", "0x0.8", 10, Down, "-4.75", "-0x4.c", Greater,
    );
    test(
        "26.0", "0x1a", "0.50", "0x0.8", 10, Up, "-4.7578", "-0x4.c2", Less,
    );
    test(
        "26.0", "0x1a", "0.50", "0x0.8", 10, Nearest, "-4.7578", "-0x4.c2", Less,
    );
    test(
        "26.0",
        "0x1a",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-4.754887502163468544361216831842",
        "-0x4.c1404eadf38396dee021361e",
        Greater,
    );
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal,
    );
    test("26.0", "0x1a", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "26.0", "0x1a", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        10,
        Floor,
        "7.571e8",
        "0x2.d2E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "7.5812e8",
        "0x2.d3E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        10,
        Down,
        "7.571e8",
        "0x2.d2E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        10,
        Up,
        "7.5812e8",
        "0x2.d3E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        10,
        Nearest,
        "7.571e8",
        "0x2.d2E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3.0",
        "0x3",
        100,
        Nearest,
        "757115704.28574892451943253721173",
        "0x2d20ab38.4926d76dad9a45cc4c",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "-1.1e9",
        "-0x4.0E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "-1.2017e9",
        "-0x4.7aE+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "-1.200e9",
        "-0x4.78E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        10,
        Down,
        "-1.200e9",
        "-0x4.78E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        10,
        Up,
        "-1.2017e9",
        "-0x4.7aE+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        10,
        Nearest,
        "-1.200e9",
        "-0x4.78E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-1.2000000e9",
        "-0x4.7868cE+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1.0",
        "0x1.0",
        300,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        1,
        Nearest,
        "1.6e-361235995",
        "0x1.0E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        10,
        Floor,
        "2.1799e-361235995",
        "0x1.5d8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "2.183e-361235995",
        "0x1.5eE-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        10,
        Down,
        "2.1799e-361235995",
        "0x1.5d8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        10,
        Up,
        "2.183e-361235995",
        "0x1.5eE-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        10,
        Nearest,
        "2.1799e-361235995",
        "0x1.5d8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3.0",
        "0x3",
        100,
        Nearest,
        "2.1800646469651550091401663766377e-361235995",
        "0x1.5d88282cd90fe05161c272afeE-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "-3.2e-361235995",
        "-0x2.0E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "-3.455e-361235995",
        "-0x2.2aE-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "-3.4491e-361235995",
        "-0x2.29E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        10,
        Down,
        "-3.4491e-361235995",
        "-0x2.29E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        10,
        Up,
        "-3.455e-361235995",
        "-0x2.2aE-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        10,
        Nearest,
        "-3.455e-361235995",
        "-0x2.2aE-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-3.4553207145876765923020056078474e-361235995",
        "-0x2.29feb17c1447d2333b97fc714E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1.0",
        "0x1.0",
        300,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.0", "-0x3", "3.0", "0x3", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("-3.0", "-0x3", "3.0", "0x3", 10, Floor, "NaN", "NaN", Equal);
    test(
        "-3.0", "-0x3", "3.0", "0x3", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test("-3.0", "-0x3", "3.0", "0x3", 10, Down, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3.0", "0x3", 10, Up, "NaN", "NaN", Equal);
    test(
        "-3.0", "-0x3", "3.0", "0x3", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "3.0", "0x3", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "3.0", "0x3", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 10, Down, "NaN", "NaN", Equal,
    );
    test("-3.0", "-0x3", "0.50", "0x0.8", 10, Up, "NaN", "NaN", Equal);
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "0.50", "0x0.8", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal,
    );
    test("-3.0", "-0x3", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-3.0", "-0x3", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0", "3.0", "0x3", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0", "3.0", "0x3", 10, Floor, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0", "3.0", "0x3", 10, Ceiling, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0", "3.0", "0x3", 10, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3.0", "0x3", 10, Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0", "3.0", "0x3", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "3.0", "0x3", 100, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "3.0", "0x3", 300, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.50", "0x0.8", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.50", "0x0.8", 10, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.50", "0x0.8", 10, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.50", "0x0.8", 10, Down, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0", "0.50", "0x0.8", 10, Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0", "0.50", "0x0.8", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.50", "0x0.8", 100, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.50", "0x0.8", 300, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "1.0", "0x1.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0", "1.0", "0x1.0", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0", "1.0", "0x1.0", 10, Down, "NaN", "NaN", Equal);
    test("0.0", "0x0", "1.0", "0x1.0", 10, Up, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0", "1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0", "1.0", "0x1.0", 100, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0", "1.0", "0x1.0", 300, Exact, "NaN", "NaN", Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        10,
        Floor,
        "20.94",
        "0x14.f",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "20.969",
        "0x14.f8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        10,
        Down,
        "20.94",
        "0x14.f",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        10,
        Up,
        "20.969",
        "0x14.f8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        10,
        Nearest,
        "20.969",
        "0x14.f8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3.0",
        "0x3",
        100,
        Nearest,
        "20.959032742984869965623807763437",
        "0x14.f5832b7ae9c6d09fa0e6795e",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "-32.0",
        "-0x2.0E+1",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "-33.25",
        "-0x21.4",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "-33.188",
        "-0x21.3",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        10,
        Down,
        "-33.188",
        "-0x21.3",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        10,
        Up,
        "-33.25",
        "-0x21.4",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        10,
        Nearest,
        "-33.25",
        "-0x21.4",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-33.219280949017892982784877160443",
        "-0x21.3822cbd8aaedf916de6f2404",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "1.0",
        "0x1.0",
        300,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.8905291817133e502048999",
        "0xd.0f17968968E+416942668",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.053e9",
        "0x3.ecE+7",
        Greater,
    );
    test(
        "1.1270158135820e-323228557",
        "0xc.2977b473e4E-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.0266e-323228557",
        "0xb.14E-268435507",
        Greater,
    );
    test(
        "6.5391849545604e467528328",
        "0x6.44a13472e5E+388273872",
        "3.0",
        "0x3",
        10,
        Nearest,
        "9.794e8",
        "0x3.a6E+7",
        Less,
    );
    test(
        "9.6953560282953e-323228558",
        "0xa.766a9bd7e6E-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "8.8179e-323228558",
        "0x9.84E-268435507",
        Less,
    );
    test(
        "7.065355269614096e483537920",
        "0x1.288e4af4d2fcE+401569551",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.0140e9",
        "0x3.c7E+7",
        Greater,
    );
    test(
        "1.166783354799258e-323228557",
        "0xc.9754173298aE-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.063e-323228557",
        "0xb.78E-268435507",
        Greater,
    );
    test(
        "3.19952755994631e493543871",
        "0x1.c183a55eed38E+409879313",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.034e9",
        "0x3.daE+7",
        Less,
    );
    test(
        "1.47697113672503e-323228557",
        "0xf.f03efa7e188E-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.34e-323228557",
        "0xe.8E-268435507",
        Less,
    );
    test(
        "1.821989292692220e502549238",
        "0x3.1ef199a98426E+417358108",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.0538e9",
        "0x3.edE+7",
        Greater,
    );
    test(
        "1.160420521567528e-323228557",
        "0xc.85c02b892f1E-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.057e-323228557",
        "0xb.68E-268435507",
        Greater,
    );
    test(
        "2.561826124768833e488040597",
        "0xf.bf6c57e5b6eE+405308942",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.0224e9",
        "0x3.cfE+7",
        Less,
    );
    test(
        "1.55173435856358e-323228557",
        "0x1.0bec8ff179bE-268435506",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.4117e-323228557",
        "0xf.3cE-268435507",
        Less,
    );
    test(
        "2.4153459996319467e502549237",
        "0x6.9e9c1d3437928E+417358107",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.0538e9",
        "0x3.edE+7",
        Greater,
    );
    test(
        "1.2224580798730439e-323228557",
        "0xd.31225957b0798E-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.1135e-323228557",
        "0xc.04E-268435507",
        Greater,
    );
    test(
        "1.2860232967649278e450017958",
        "0x4.bf2ed608df1aE+373731824",
        "3.0",
        "0x3",
        10,
        Nearest,
        "9.4267e8",
        "0x3.83E+7",
        Less,
    );
    test(
        "1.2495000930601205e-323228557",
        "0xd.7bd6fdd3b94aE-268435507",
        "3.0",
        "0x3",
        10,
        Nearest,
        "1.1366e-323228557",
        "0xc.44E-268435507",
        Less,
    );
    test(
        "3.3313234829212e464776801",
        "0x7.1c2ef93ce1E+385988778",
        "3.0",
        "0x3",
        10,
        Floor,
        "9.7413e8",
        "0x3.a1E+7",
        Less,
    );
    test(
        "1.099178443451e-323228557",
        "0xb.dc9092016E-268435507",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.0005e-323228557",
        "0xa.ccE-268435507",
        Less,
    );
    test(
        "3.3313234829212e464776801",
        "0x7.1c2ef93ce1E+385988778",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "9.752e8",
        "0x3.a2E+7",
        Greater,
    );
    test(
        "1.099178443451e-323228557",
        "0xb.dc9092016E-268435507",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.002e-323228557",
        "0xa.dE-268435507",
        Greater,
    );
    test(
        "6.6937337702026e479785625",
        "0x3.df748114408E+398453337",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.005e9",
        "0x3.beE+7",
        Less,
    );
    test(
        "9.5601459770385e-323228558",
        "0xa.511049ddeaE-268435507",
        "3.0",
        "0x3",
        10,
        Floor,
        "8.688e-323228558",
        "0x9.6E-268435507",
        Less,
    );
    test(
        "6.6937337702026e479785625",
        "0x3.df748114408E+398453337",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.0056e9",
        "0x3.bfE+7",
        Greater,
    );
    test(
        "9.5601459770385e-323228558",
        "0xa.511049ddeaE-268435507",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "8.7020e-323228558",
        "0x9.64E-268435507",
        Greater,
    );
    test(
        "9.276267820693966e452769599",
        "0x6.3e87804b4a5E+376017013",
        "3.0",
        "0x3",
        10,
        Floor,
        "9.4896e8",
        "0x3.89E+7",
        Less,
    );
    test(
        "1.55093901967181e-323228557",
        "0x1.0bc968465ce8E-268435506",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.4117e-323228557",
        "0xf.3cE-268435507",
        Less,
    );
    test(
        "9.276267820693966e452769599",
        "0x6.3e87804b4a5E+376017013",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "9.500e8",
        "0x3.8aE+7",
        Greater,
    );
    test(
        "1.55093901967181e-323228557",
        "0x1.0bc968465ce8E-268435506",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.41e-323228557",
        "0xf.4E-268435507",
        Greater,
    );
    test(
        "2.71930365542484e451768997",
        "0x1.f7c527569cc8E+375186031",
        "3.0",
        "0x3",
        10,
        Floor,
        "9.458e8",
        "0x3.86E+7",
        Less,
    );
    test(
        "9.05112091326373e-323228558",
        "0x9.c471097214cE-268435507",
        "3.0",
        "0x3",
        10,
        Floor,
        "8.224e-323228558",
        "0x8.eE-268435507",
        Less,
    );
    test(
        "2.71930365542484e451768997",
        "0x1.f7c527569cc8E+375186031",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "9.4686e8",
        "0x3.87E+7",
        Greater,
    );
    test(
        "9.05112091326373e-323228558",
        "0x9.c471097214cE-268435507",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "8.2387e-323228558",
        "0x8.e4E-268435507",
        Greater,
    );
    test(
        "1.895754760406790e487790450",
        "0x2.4a4f88ae1be6E+405101200",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.0224e9",
        "0x3.cfE+7",
        Less,
    );
    test(
        "1.605023038055702e-323228557",
        "0x1.151ffcbeb8a4E-268435506",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.4610e-323228557",
        "0xf.c4E-268435507",
        Less,
    );
    test(
        "1.895754760406790e487790450",
        "0x2.4a4f88ae1be6E+405101200",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.02e9",
        "0x3.dE+7",
        Greater,
    );
    test(
        "1.605023038055702e-323228557",
        "0x1.151ffcbeb8a4E-268435506",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.462e-323228557",
        "0xf.c8E-268435507",
        Greater,
    );
    test(
        "1.339394062988955e479785682",
        "0x1.f9ad7d768e79E+398453384",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.005e9",
        "0x3.beE+7",
        Less,
    );
    test(
        "9.591961232330468e-323228558",
        "0xa.59da536a92b8E-268435507",
        "3.0",
        "0x3",
        10,
        Floor,
        "8.717e-323228558",
        "0x9.68E-268435507",
        Less,
    );
    test(
        "1.339394062988955e479785682",
        "0x1.f9ad7d768e79E+398453384",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.0056e9",
        "0x3.bfE+7",
        Greater,
    );
    test(
        "9.591961232330468e-323228558",
        "0xa.59da536a92b8E-268435507",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "8.7310e-323228558",
        "0x9.6cE-268435507",
        Greater,
    );
    test(
        "1.6583731482131300e488791045",
        "0x3.206cb14aaf0f2E+405932176",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.0245e9",
        "0x3.d1E+7",
        Less,
    );
    test(
        "1.1723508196049881e-323228557",
        "0xc.a6b582d158cbE-268435507",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.0671e-323228557",
        "0xb.84E-268435507",
        Less,
    );
    test(
        "1.6583731482131300e488791045",
        "0x3.206cb14aaf0f2E+405932176",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.026e9",
        "0x3.d2E+7",
        Greater,
    );
    test(
        "1.1723508196049881e-323228557",
        "0xc.a6b582d158cbE-268435507",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.069e-323228557",
        "0xb.88E-268435507",
        Greater,
    );
    test(
        "7.3560540658422518e494794619",
        "0x1.e9254b26f72e2E+410918037",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.036e9",
        "0x3.dcE+7",
        Less,
    );
    test(
        "9.0829351131423132e-323228558",
        "0x9.cd3affe312d48E-268435507",
        "3.0",
        "0x3",
        10,
        Floor,
        "8.253e-323228558",
        "0x8.e8E-268435507",
        Less,
    );
    test(
        "7.3560540658422518e494794619",
        "0x1.e9254b26f72e2E+410918037",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.0370e9",
        "0x3.ddE+7",
        Greater,
    );
    test(
        "9.0829351131423132e-323228558",
        "0x9.cd3affe312d48E-268435507",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "8.2676e-323228558",
        "0x8.ecE-268435507",
        Greater,
    );
    test(
        "3.43151311307435493844682996505668332598490e500068027",
        "0x4.11ed06daa99e79c4f938dd31feb41e6f2E+415297507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1048094216.2790585401728541997664",
        "0x3e78a608.47706167b63fcc332c",
        Greater,
    );
    test(
        "9.93555131619011973912378282398966340636624e-323228558",
        "0xa.b8c5b0e499e0c33cd610861f56d389f76E-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "9.0437285461601509513485020326727e-323228558",
        "0x9.c2663bf392c0b857ef11b7c5E-268435507",
        Greater,
    );
    test(
        "2.02398564345573544285201777651606924420470e472407042",
        "0x4.2b36adc73e9c50b0a9a088a5bdce14b06E+392325556",
        "3.0",
        "0x3",
        100,
        Nearest,
        "990119466.76694398311654417494",
        "0x3b04062a.c45670dd597b0526",
        Less,
    );
    test(
        "1.56277005705445198521579378386244362282112e-323228557",
        "0x1.0dd45a8b6d6bacc4f068aed8edd08e2026E-268435506",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.42249460812882292425937765711e-323228557",
        "0xf.59c01bb16735648a6d51815E-268435507",
        Less,
    );
    test(
        "3.2569768526240845915040088000157194052477773e508090262",
        "0x3.6f2e00b60a767bdf63f0654d75bafa6a21cE+421959829",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1064908044.835161237646897308033",
        "0x3f79350c.d5cd207a948f0f8d98",
        Greater,
    );
    test(
        "1.184597869089555929253712990424081873025556e-323228557",
        "0xc.c88add630a14ce2256d98e128d581c950eE-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.078267448223876954468690583692e-323228557",
        "0xb.a2cbe7706c669aab616a8408E-268435507",
        Greater,
    );
    test(
        "1.1112981772552822242219461981610692501859239e481453721",
        "0x5.075df1ae1d8a16be92c405932663055dca8E+399838660",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1009080430.358764357377351758588",
        "0x3c25586e.5bd7fb1de7fb8d66b8",
        Less,
    );
    test(
        "1.0195075958391630683318728832766914612804773e-323228557",
        "0xb.0077c8789a91d776fb14463dac6ffa49e1E-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "9.2799580557682609595193022409608e-323228558",
        "0xa.03a8d94a07304b4d70c74e33E-268435507",
        Less,
    );
    test(
        "7.7307682702424082617370181921233855604475677e471381169",
        "0xc.935cf3367cd063dc97de4c9d5f08431a45cE+391473587",
        "3.0",
        "0x3",
        100,
        Nearest,
        "987969337.40708653100294400217056",
        "0x3ae33739.6836d2a94cbb679b84",
        Greater,
    );
    test(
        "1.3263971880009921582343680305757126988257195e-323228557",
        "0xe.5046205d2d55205b5df600645910c00cb7cE-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.2073387506060349455671199172217e-323228557",
        "0xd.075da99e12e54bd7f6ab041bE-268435507",
        Greater,
    );
    test(
        "1.0586314506378563923463218970852435783424457e478081741",
        "0x8.f497828cfc3850cf85f51c025cd3b46393E+397038291",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1002013086.391732220437663955699",
        "0x3bb9819e.6448901391b90636d",
        Less,
    );
    test(
        "1.466357552257149483450861911530042289405228e-323228557",
        "0xf.d2ecd7dd7811ff98ce554da334d6e4dda4E-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.3347361643249700449460409091279e-323228557",
        "0xe.674f9d3985095830a2028263E-268435507",
        Less,
    );
    test(
        "1.25130399605868009995879895331186180133575481e488089618",
        "0x9.10e818abfae2d1d2bf2fbd1d387de40511b4E+405349653",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1022988628.7169180302882569103123",
        "0x3cf99154.b787f0a600347c94bc",
        Greater,
    );
    test(
        "1.20279490412280345923197908202783335270514791e-323228557",
        "0xc.fad02a471042401d4f66da29b896dcc88928E-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.094831103319441652434267207565e-323228557",
        "0xb.d08e0bcaa723e5fc39f2488cE-268435507",
        Greater,
    );
    test(
        "4.84916892693620663318590921231156377634945470e473179925",
        "0x2.1a2ce3f70bf87751a0526211c648467105a2E+392967422",
        "3.0",
        "0x3",
        100,
        Nearest,
        "991739355.57259780295630008291809",
        "0x3b1cbddb.9295c50575715bf0fc",
        Less,
    );
    test(
        "1.0790641766361426300254960420802133723849545e-323228557",
        "0xb.a4ff5db432d3c47767ef35946c8de8b89f4E-268435507",
        "3.0",
        "0x3",
        100,
        Nearest,
        "9.822065416220075272051075920868e-323228558",
        "0xa.996bbed1b76cdd258b586136E-268435507",
        Less,
    );
    test(
        "9.79243297103166413107271064011323373076726e492252299",
        "0x2.2995d0d77e18e82e8ad40858c7ab589b2cE+408806686",
        "3.0",
        "0x3",
        100,
        Floor,
        "1031713207.3273880314711116496811",
        "0x3d7eb1b7.53cfb3b8452eaa4944",
        Less,
    );
    test(
        "1.3543085078940773284967275346120687321360e-323228557",
        "0xe.9d618f48d361ba83ad2869987c7fc5dbE-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "1.2327447288396510527129230497201e-323228557",
        "0xd.4d8d44e0b98d81820b95da31E-268435507",
        Less,
    );
    test(
        "9.79243297103166413107271064011323373076726e492252299",
        "0x2.2995d0d77e18e82e8ad40858c7ab589b2cE+408806686",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1031713207.327388031471111649682",
        "0x3d7eb1b7.53cfb3b8452eaa4948",
        Greater,
    );
    test(
        "1.3543085078940773284967275346120687321360e-323228557",
        "0xe.9d618f48d361ba83ad2869987c7fc5dbE-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1.232744728839651052712923049721e-323228557",
        "0xd.4d8d44e0b98d81820b95da32E-268435507",
        Greater,
    );
    test(
        "1.86039361869456473198154122102456750572934e469398778",
        "0x2.5c8091903fa3dfc96f13345e000609d2bE+389827247",
        "3.0",
        "0x3",
        100,
        Floor,
        "983814436.3227016210132452231940",
        "0x3aa3d124.529c92cc9e3a23582",
        Less,
    );
    test(
        "1.07009541173391798601539278898003131990687e-323228557",
        "0xb.8c387abd326a04289c3ea92935e06cdd7E-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "9.740428199936086443479762994126e-323228558",
        "0xa.82de338291221ebbb5a059caE-268435507",
        Less,
    );
    test(
        "1.86039361869456473198154122102456750572934e469398778",
        "0x2.5c8091903fa3dfc96f13345e000609d2bE+389827247",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "983814436.32270162101324522319483",
        "0x3aa3d124.529c92cc9e3a235824",
        Greater,
    );
    test(
        "1.07009541173391798601539278898003131990687e-323228557",
        "0xb.8c387abd326a04289c3ea92935e06cdd7E-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "9.7404281999360864434797629941375e-323228558",
        "0xa.82de338291221ebbb5a059cbE-268435507",
        Greater,
    );
    test(
        "3.0917878855136258200963142468175039209289135e504741443",
        "0x3.71b0ee0aa56d0f9bdd6d5e5aa35e5c5e29cE+419178695",
        "3.0",
        "0x3",
        100,
        Floor,
        "1057889244.0806808815479237524558",
        "0x3f0e1bdc.14a78093a929089bf4",
        Less,
    );
    test(
        "1.2403409402826204519205952123462951200328370e-323228557",
        "0xd.62897579afe2cdad484ededa1d4e8e7dffE-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "1.1290069782364567436077665115643e-323228557",
        "0xc.2ef7e547608292ed2d2c3663E-268435507",
        Less,
    );
    test(
        "3.0917878855136258200963142468175039209289135e504741443",
        "0x3.71b0ee0aa56d0f9bdd6d5e5aa35e5c5e29cE+419178695",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1057889244.080680881547923752457",
        "0x3f0e1bdc.14a78093a929089bf8",
        Greater,
    );
    test(
        "1.2403409402826204519205952123462951200328370e-323228557",
        "0xd.62897579afe2cdad484ededa1d4e8e7dffE-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1.129006978236456743607766511565e-323228557",
        "0xc.2ef7e547608292ed2d2c3664E-268435507",
        Greater,
    );
    test(
        "1.2574944251593781698086953211265913105698315e471439815",
        "0x7.1943aa330363b432488d2e4d1b90f60af88E+391522291",
        "3.0",
        "0x3",
        100,
        Floor,
        "988092252.097436828838026709034",
        "0x3ae5175c.18f19eb94907be2b4",
        Less,
    );
    test(
        "1.3265721348803061714086087122610992890882804e-323228557",
        "0xe.50c1da2f987108b80e241c68af6684b621E-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "1.207497994118162511575644699502e-323228557",
        "0xd.07ce485f673d13a2c9b7a90eE-268435507",
        Less,
    );
    test(
        "1.2574944251593781698086953211265913105698315e471439815",
        "0x7.1943aa330363b432488d2e4d1b90f60af88E+391522291",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "988092252.09743682883802670903492",
        "0x3ae5175c.18f19eb94907be2b44",
        Greater,
    );
    test(
        "1.3265721348803061714086087122610992890882804e-323228557",
        "0xe.50c1da2f987108b80e241c68af6684b621E-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1.2074979941181625115756446995028e-323228557",
        "0xd.07ce485f673d13a2c9b7a90fE-268435507",
        Greater,
    );
    test(
        "1.9171393735699252129929856743994958270425732e488328802",
        "0xe.5fe168ad65322a252d74b36ccdbe1594064E+405548291",
        "3.0",
        "0x3",
        100,
        Floor,
        "1023489935.6340273725291618216148",
        "0x3d01378f.a24f9e2dc8143d4f4c",
        Less,
    );
    test(
        "1.0438114241939067469822683272108564299921844e-323228557",
        "0xb.439bed73cd3acfb0bdc68d477ec263b7468E-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "9.5011810350251938385579429184231e-323228558",
        "0xa.40c62c0f08cf69014ccd01dfE-268435507",
        Less,
    );
    test(
        "1.9171393735699252129929856743994958270425732e488328802",
        "0xe.5fe168ad65322a252d74b36ccdbe1594064E+405548291",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1023489935.634027372529161821616",
        "0x3d01378f.a24f9e2dc8143d4f5",
        Greater,
    );
    test(
        "1.0438114241939067469822683272108564299921844e-323228557",
        "0xb.439bed73cd3acfb0bdc68d477ec263b7468E-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "9.50118103502519383855794291843e-323228558",
        "0xa.40c62c0f08cf69014ccd01eE-268435507",
        Greater,
    );
    test(
        "5.759609304903433221228363713722410448210957e506156295",
        "0x9.c8f37abaaabde9ad34694b2c9f5e739742E+420353704",
        "3.0",
        "0x3",
        100,
        Floor,
        "1060854637.586393899271327691228",
        "0x3f3b5b6d.961de91bf1bbb8e5d8",
        Less,
    );
    test(
        "1.2388819864411335831204281823018497372290153e-323228557",
        "0xd.5e81a878dff6094658d555a97368b944abcE-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "1.127678981220097482638537813333e-323228557",
        "0xc.2b4cb5c6c00e43a509b165f4E-268435507",
        Less,
    );
    test(
        "5.759609304903433221228363713722410448210957e506156295",
        "0x9.c8f37abaaabde9ad34694b2c9f5e739742E+420353704",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1060854637.5863938992713276912290",
        "0x3f3b5b6d.961de91bf1bbb8e5dc",
        Greater,
    );
    test(
        "1.2388819864411335831204281823018497372290153e-323228557",
        "0xd.5e81a878dff6094658d555a97368b944abcE-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1.1276789812200974826385378133341e-323228557",
        "0xc.2b4cb5c6c00e43a509b165f5E-268435507",
        Greater,
    );
    test(
        "6.13023895285964203990852997986971418797833034e476969670",
        "0x3.7a44d14c5739823a60416080a0caa6583615E+396114737",
        "3.0",
        "0x3",
        100,
        Floor,
        "999682294.74020372737524440078392",
        "0x3b95f0f6.bd7dfdd17437ae4804",
        Less,
    );
    test(
        "1.04601081500484731620032730811103510978754745e-323228557",
        "0xb.49af61412521de42f94fa2dce7951f58c63cE-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "9.5212007529332010031630882030205e-323228558",
        "0xa.464e017f69271bd95eec7513E-268435507",
        Less,
    );
    test(
        "6.13023895285964203990852997986971418797833034e476969670",
        "0x3.7a44d14c5739823a60416080a0caa6583615E+396114737",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "999682294.7402037273752444007848",
        "0x3b95f0f6.bd7dfdd17437ae4808",
        Greater,
    );
    test(
        "1.04601081500484731620032730811103510978754745e-323228557",
        "0xb.49af61412521de42f94fa2dce7951f58c63cE-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "9.521200752933201003163088203032e-323228558",
        "0xa.464e017f69271bd95eec7514E-268435507",
        Greater,
    );
    test(
        "3.34845918461852248682535394189475463088042290e468301122",
        "0xd.c2f54b4aebe0ddfe409bfa4e6e8f5d40df68E+388915663",
        "3.0",
        "0x3",
        100,
        Floor,
        "981513856.0532169369080431585027",
        "0x3a80b680.0d9fa00b9d03e5d3b8",
        Less,
    );
    test(
        "1.46522706304360478325929510199197764667553670e-323228557",
        "0xf.cfcd56ccbd47477864d6a60b90a9f465cdccE-268435507",
        "3.0",
        "0x3",
        100,
        Floor,
        "1.333707148697523135415139712647e-323228557",
        "0xe.6477dfc59fdbd104934dd9eeE-268435507",
        Less,
    );
    test(
        "3.34845918461852248682535394189475463088042290e468301122",
        "0xd.c2f54b4aebe0ddfe409bfa4e6e8f5d40df68E+388915663",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "981513856.05321693690804315850350",
        "0x3a80b680.0d9fa00b9d03e5d3bc",
        Greater,
    );
    test(
        "1.46522706304360478325929510199197764667553670e-323228557",
        "0xf.cfcd56ccbd47477864d6a60b90a9f465cdccE-268435507",
        "3.0",
        "0x3",
        100,
        Ceiling,
        "1.3337071486975231354151397126483e-323228557",
        "0xe.6477dfc59fdbd104934dd9efE-268435507",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_big_exponent_float_base_1_plus_x_prec_round(
        &BigExponentFloat::from(3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI)
        .log_base_big_exponent_float_base_1_plus_x_prec_round(
            &BigExponentFloat::from(3),
            10,
            Exact,
        );
}

fn log_base_big_exponent_float_base_1_plus_x_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) =
                log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(
            &z.log_base_big_exponent_float_base_prec_round_ref(b, prec, rm),
            &r
        ));
    }
    // A base of 1 gives NaN, as in `arf_div`, since ln(1) = 0 and there is no signed zero.
    if *b == 1u32 {
        assert!(r.0.is_nan());
    }
}

#[test]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_12()
        .test_properties(|(x, b, prec, rm)| {
            log_base_big_exponent_float_base_1_plus_x_prec_round_properties_helper(
                &x, &b, prec, rm,
            );
        });
}

#[test]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_float_base_1_plus_x_prec_round_ref(&b, prec, rm);
            if float_result_comparable(&f, fo) && b != 1u32 {
                let r = log_base_big_exponent_float_base_1_plus_x_prec_round_all(
                    &float_to_big_exponent_float(&x),
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
