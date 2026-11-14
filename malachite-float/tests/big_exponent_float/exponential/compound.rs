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
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_5,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_17,
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

fn compound_prec_round_all(
    x: &BigExponentFloat,
    b: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().compound_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.compound_prec_round_ref(b, prec, rm), &r));
    let mut z = x.clone();
    let o = z.compound_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().compound_prec(b, prec), &r));
        assert!(same_pair(&x.compound_prec_ref(b, prec), &r));
        let mut z = x.clone();
        let o = z.compound_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_compound_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let cases: &[(&str, &str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", "3", 1, Nearest, "0x4.0E+1", Less),
        ("0x3.243f6a8885a30", "3", 10, Floor, "0x47.0", Less),
        ("0x3.243f6a8885a30", "3", 10, Ceiling, "0x47.2", Greater),
        ("0x3.243f6a8885a30", "3", 10, Down, "0x47.0", Less),
        ("0x3.243f6a8885a30", "3", 10, Up, "0x47.2", Greater),
        ("0x3.243f6a8885a30", "3", 10, Nearest, "0x47.0", Less),
        (
            "0x3.243f6a8885a30",
            "3",
            100,
            Nearest,
            "0x47.0a34c76f9730f9d2cd8f6758",
            Less,
        ),
        (
            "0x3.243f6a8885a30",
            "3",
            300,
            Exact,
            "0x47.0a34c76f9730f9d2cd8f675b782a821d5bfb",
            Equal,
        ),
        ("0x3.243f6a8885a30", "-2", 1, Nearest, "0x0.1", Greater),
        ("0x3.243f6a8885a30", "-2", 10, Floor, "0x0.0eec", Less),
        ("0x3.243f6a8885a30", "-2", 10, Ceiling, "0x0.0ef", Greater),
        ("0x3.243f6a8885a30", "-2", 10, Down, "0x0.0eec", Less),
        ("0x3.243f6a8885a30", "-2", 10, Up, "0x0.0ef", Greater),
        ("0x3.243f6a8885a30", "-2", 10, Nearest, "0x0.0eec", Less),
        (
            "0x3.243f6a8885a30",
            "-2",
            100,
            Nearest,
            "0x0.0eecb83857eabad36f64ae2332",
            Less,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            1,
            Nearest,
            "0x4.0E+512546",
            Greater,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            10,
            Floor,
            "0x3.2bE+512546",
            Less,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            10,
            Ceiling,
            "0x3.2cE+512546",
            Greater,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            10,
            Down,
            "0x3.2bE+512546",
            Less,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            10,
            Up,
            "0x3.2cE+512546",
            Greater,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            10,
            Nearest,
            "0x3.2bE+512546",
            Less,
        ),
        (
            "0x3.243f6a8885a30",
            "1000000",
            100,
            Nearest,
            "0x3.2b628d58b1fa5d291b3360144E+512546",
            Greater,
        ),
        ("0x0.8", "3", 1, Nearest, "0x4.0", Greater),
        ("0x0.8", "3", 10, Floor, "0x3.6", Equal),
        ("0x0.8", "3", 10, Ceiling, "0x3.6", Equal),
        ("0x0.8", "3", 10, Down, "0x3.6", Equal),
        ("0x0.8", "3", 10, Up, "0x3.6", Equal),
        ("0x0.8", "3", 10, Nearest, "0x3.6", Equal),
        ("0x0.8", "3", 100, Nearest, "0x3.6", Equal),
        ("0x0.8", "3", 300, Exact, "0x3.6", Equal),
        ("0x0.8", "-2", 1, Nearest, "0x0.8", Greater),
        ("0x0.8", "-2", 10, Floor, "0x0.71c", Less),
        ("0x0.8", "-2", 10, Ceiling, "0x0.71e", Greater),
        ("0x0.8", "-2", 10, Down, "0x0.71c", Less),
        ("0x0.8", "-2", 10, Up, "0x0.71e", Greater),
        ("0x0.8", "-2", 10, Nearest, "0x0.71c", Less),
        (
            "0x0.8",
            "-2",
            100,
            Nearest,
            "0x0.71c71c71c71c71c71c71c71c7",
            Less,
        ),
        ("0x0.8", "1000000", 1, Nearest, "0x4.0E+146240", Less),
        ("0x0.8", "1000000", 10, Floor, "0x5.a8E+146240", Less),
        ("0x0.8", "1000000", 10, Ceiling, "0x5.aaE+146240", Greater),
        ("0x0.8", "1000000", 10, Down, "0x5.a8E+146240", Less),
        ("0x0.8", "1000000", 10, Up, "0x5.aaE+146240", Greater),
        ("0x0.8", "1000000", 10, Nearest, "0x5.a8E+146240", Less),
        (
            "0x0.8",
            "1000000",
            100,
            Nearest,
            "0x5.a8e0f6076b7d71b9c4eaee69E+146240",
            Greater,
        ),
        ("-0x0.8", "3", 1, Nearest, "0x0.2", Equal),
        ("-0x0.8", "3", 10, Floor, "0x0.2", Equal),
        ("-0x0.8", "3", 10, Ceiling, "0x0.2", Equal),
        ("-0x0.8", "3", 10, Down, "0x0.2", Equal),
        ("-0x0.8", "3", 10, Up, "0x0.2", Equal),
        ("-0x0.8", "3", 10, Nearest, "0x0.2", Equal),
        ("-0x0.8", "3", 100, Nearest, "0x0.2", Equal),
        ("-0x0.8", "3", 300, Exact, "0x0.2", Equal),
        ("-0x0.8", "-2", 1, Nearest, "0x4.0", Equal),
        ("-0x0.8", "-2", 10, Floor, "0x4.0", Equal),
        ("-0x0.8", "-2", 10, Ceiling, "0x4.0", Equal),
        ("-0x0.8", "-2", 10, Down, "0x4.0", Equal),
        ("-0x0.8", "-2", 10, Up, "0x4.0", Equal),
        ("-0x0.8", "-2", 10, Nearest, "0x4.0", Equal),
        ("-0x0.8", "-2", 100, Nearest, "0x4.0", Equal),
        ("-0x0.8", "-2", 300, Exact, "0x4.0", Equal),
        ("-0x0.8", "1000000", 1, Nearest, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 10, Floor, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 10, Ceiling, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 10, Down, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 10, Up, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 10, Nearest, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 100, Nearest, "0x1.0E-250000", Equal),
        ("-0x0.8", "1000000", 300, Exact, "0x1.0E-250000", Equal),
        ("0x1.0E-100", "3", 1, Nearest, "0x1.0", Less),
        ("0x1.0E-100", "3", 10, Floor, "0x1.0", Less),
        ("0x1.0E-100", "3", 10, Ceiling, "0x1.008", Greater),
        ("0x1.0E-100", "3", 10, Down, "0x1.0", Less),
        ("0x1.0E-100", "3", 10, Up, "0x1.008", Greater),
        ("0x1.0E-100", "3", 10, Nearest, "0x1.0", Less),
        ("0x1.0E-100", "3", 100, Nearest, "0x1.0", Less),
        ("0x1.0E-100", "-2", 1, Nearest, "0x1.0", Greater),
        ("0x1.0E-100", "-2", 10, Floor, "0x0.ffc", Less),
        ("0x1.0E-100", "-2", 10, Ceiling, "0x1.0", Greater),
        ("0x1.0E-100", "-2", 10, Down, "0x0.ffc", Less),
        ("0x1.0E-100", "-2", 10, Up, "0x1.0", Greater),
        ("0x1.0E-100", "-2", 10, Nearest, "0x1.0", Greater),
        ("0x1.0E-100", "-2", 100, Nearest, "0x1.0", Greater),
        ("0x1.0E-100", "1000000", 1, Nearest, "0x1.0", Less),
        ("0x1.0E-100", "1000000", 10, Floor, "0x1.0", Less),
        ("0x1.0E-100", "1000000", 10, Ceiling, "0x1.008", Greater),
        ("0x1.0E-100", "1000000", 10, Down, "0x1.0", Less),
        ("0x1.0E-100", "1000000", 10, Up, "0x1.008", Greater),
        ("0x1.0E-100", "1000000", 10, Nearest, "0x1.0", Less),
        ("0x1.0E-100", "1000000", 100, Nearest, "0x1.0", Less),
        ("0x1.0E+20", "3", 1, Nearest, "0x1.0E+60", Less),
        ("0x1.0E+20", "3", 10, Floor, "0x1.0E+60", Less),
        ("0x1.0E+20", "3", 10, Ceiling, "0x1.008E+60", Greater),
        ("0x1.0E+20", "3", 10, Down, "0x1.0E+60", Less),
        ("0x1.0E+20", "3", 10, Up, "0x1.008E+60", Greater),
        ("0x1.0E+20", "3", 10, Nearest, "0x1.0E+60", Less),
        (
            "0x1.0E+20",
            "3",
            100,
            Nearest,
            "0x1.00000000000000000003E+60",
            Less,
        ),
        (
            "0x1.0E+20",
            "3",
            300,
            Exact,
            "0x1000000000000000000030000000000000000000300000000000000000001.0",
            Equal,
        ),
        ("0x1.0E+20", "-2", 1, Nearest, "0x1.0E-40", Greater),
        ("0x1.0E+20", "-2", 10, Floor, "0xf.fcE-41", Less),
        ("0x1.0E+20", "-2", 10, Ceiling, "0x1.0E-40", Greater),
        ("0x1.0E+20", "-2", 10, Down, "0xf.fcE-41", Less),
        ("0x1.0E+20", "-2", 10, Up, "0x1.0E-40", Greater),
        ("0x1.0E+20", "-2", 10, Nearest, "0x1.0E-40", Greater),
        (
            "0x1.0E+20",
            "-2",
            100,
            Nearest,
            "0xf.ffffffffffffffffffeE-41",
            Less,
        ),
        ("0x1.0E+20", "1000000", 1, Nearest, "0x1.0E+20000000", Less),
        ("0x1.0E+20", "1000000", 10, Floor, "0x1.0E+20000000", Less),
        (
            "0x1.0E+20",
            "1000000",
            10,
            Ceiling,
            "0x1.008E+20000000",
            Greater,
        ),
        ("0x1.0E+20", "1000000", 10, Down, "0x1.0E+20000000", Less),
        ("0x1.0E+20", "1000000", 10, Up, "0x1.008E+20000000", Greater),
        ("0x1.0E+20", "1000000", 10, Nearest, "0x1.0E+20000000", Less),
        (
            "0x1.0E+20",
            "1000000",
            100,
            Nearest,
            "0x1.000000000000000f424E+20000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "3",
            1,
            Nearest,
            "0x1.0E+900000000",
            Less,
        ),
        ("0x1.0E+300000000", "3", 10, Floor, "0x1.0E+900000000", Less),
        (
            "0x1.0E+300000000",
            "3",
            10,
            Ceiling,
            "0x1.008E+900000000",
            Greater,
        ),
        ("0x1.0E+300000000", "3", 10, Down, "0x1.0E+900000000", Less),
        (
            "0x1.0E+300000000",
            "3",
            10,
            Up,
            "0x1.008E+900000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "3",
            10,
            Nearest,
            "0x1.0E+900000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "3",
            100,
            Nearest,
            "0x1.0E+900000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            1,
            Nearest,
            "0x1.0E-600000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            10,
            Floor,
            "0xf.fcE-600000001",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            10,
            Ceiling,
            "0x1.0E-600000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            10,
            Down,
            "0xf.fcE-600000001",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            10,
            Up,
            "0x1.0E-600000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            10,
            Nearest,
            "0x1.0E-600000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "-2",
            100,
            Nearest,
            "0x1.0E-600000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            1,
            Nearest,
            "0x1.0E+300000000000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            10,
            Floor,
            "0x1.0E+300000000000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            10,
            Ceiling,
            "0x1.008E+300000000000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            10,
            Down,
            "0x1.0E+300000000000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            10,
            Up,
            "0x1.008E+300000000000000",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            10,
            Nearest,
            "0x1.0E+300000000000000",
            Less,
        ),
        (
            "0x1.0E+300000000",
            "1000000",
            100,
            Nearest,
            "0x1.0E+300000000000000",
            Less,
        ),
        ("0x1.8E-300000000", "3", 1, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", "3", 10, Floor, "0x1.0", Less),
        ("0x1.8E-300000000", "3", 10, Ceiling, "0x1.008", Greater),
        ("0x1.8E-300000000", "3", 10, Down, "0x1.0", Less),
        ("0x1.8E-300000000", "3", 10, Up, "0x1.008", Greater),
        ("0x1.8E-300000000", "3", 10, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", "3", 100, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", "-2", 1, Nearest, "0x1.0", Greater),
        ("0x1.8E-300000000", "-2", 10, Floor, "0x0.ffc", Less),
        ("0x1.8E-300000000", "-2", 10, Ceiling, "0x1.0", Greater),
        ("0x1.8E-300000000", "-2", 10, Down, "0x0.ffc", Less),
        ("0x1.8E-300000000", "-2", 10, Up, "0x1.0", Greater),
        ("0x1.8E-300000000", "-2", 10, Nearest, "0x1.0", Greater),
        ("0x1.8E-300000000", "-2", 100, Nearest, "0x1.0", Greater),
        ("0x1.8E-300000000", "1000000", 1, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", "1000000", 10, Floor, "0x1.0", Less),
        (
            "0x1.8E-300000000",
            "1000000",
            10,
            Ceiling,
            "0x1.008",
            Greater,
        ),
        ("0x1.8E-300000000", "1000000", 10, Down, "0x1.0", Less),
        ("0x1.8E-300000000", "1000000", 10, Up, "0x1.008", Greater),
        ("0x1.8E-300000000", "1000000", 10, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", "1000000", 100, Nearest, "0x1.0", Less),
        ("0x3", "3", 1, Nearest, "0x4.0E+1", Equal),
        ("0x3", "3", 10, Floor, "0x4.0E+1", Equal),
        ("0x3", "3", 10, Ceiling, "0x4.0E+1", Equal),
        ("0x3", "3", 10, Down, "0x4.0E+1", Equal),
        ("0x3", "3", 10, Up, "0x4.0E+1", Equal),
        ("0x3", "3", 10, Nearest, "0x4.0E+1", Equal),
        ("0x3", "3", 100, Nearest, "0x4.0E+1", Equal),
        ("0x3", "3", 300, Exact, "0x4.0E+1", Equal),
        ("0x3", "-2", 1, Nearest, "0x0.1", Equal),
        ("0x3", "-2", 10, Floor, "0x0.1", Equal),
        ("0x3", "-2", 10, Ceiling, "0x0.1", Equal),
        ("0x3", "-2", 10, Down, "0x0.1", Equal),
        ("0x3", "-2", 10, Up, "0x0.1", Equal),
        ("0x3", "-2", 10, Nearest, "0x0.1", Equal),
        ("0x3", "-2", 100, Nearest, "0x0.1", Equal),
        ("0x3", "-2", 300, Exact, "0x0.1", Equal),
        ("0x3", "1000000", 1, Nearest, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 10, Floor, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 10, Ceiling, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 10, Down, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 10, Up, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 10, Nearest, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 100, Nearest, "0x1.0E+500000", Equal),
        ("0x3", "1000000", 300, Exact, "0x1.0E+500000", Equal),
        ("-0x1.0", "3", 1, Nearest, "0x0.0", Equal),
        ("-0x1.0", "3", 10, Floor, "0x0.0", Equal),
        ("-0x1.0", "3", 10, Ceiling, "0x0.0", Equal),
        ("-0x1.0", "3", 10, Down, "0x0.0", Equal),
        ("-0x1.0", "3", 10, Up, "0x0.0", Equal),
        ("-0x1.0", "3", 10, Nearest, "0x0.0", Equal),
        ("-0x1.0", "3", 100, Nearest, "0x0.0", Equal),
        ("-0x1.0", "3", 300, Exact, "0x0.0", Equal),
        ("-0x1.0", "-2", 1, Nearest, "Infinity", Equal),
        ("-0x1.0", "-2", 10, Floor, "Infinity", Equal),
        ("-0x1.0", "-2", 10, Ceiling, "Infinity", Equal),
        ("-0x1.0", "-2", 10, Down, "Infinity", Equal),
        ("-0x1.0", "-2", 10, Up, "Infinity", Equal),
        ("-0x1.0", "-2", 10, Nearest, "Infinity", Equal),
        ("-0x1.0", "-2", 100, Nearest, "Infinity", Equal),
        ("-0x1.0", "-2", 300, Exact, "Infinity", Equal),
        ("-0x1.0", "1000000", 1, Nearest, "0x0.0", Equal),
        ("-0x1.0", "1000000", 10, Floor, "0x0.0", Equal),
        ("-0x1.0", "1000000", 10, Ceiling, "0x0.0", Equal),
        ("-0x1.0", "1000000", 10, Down, "0x0.0", Equal),
        ("-0x1.0", "1000000", 10, Up, "0x0.0", Equal),
        ("-0x1.0", "1000000", 10, Nearest, "0x0.0", Equal),
        ("-0x1.0", "1000000", 100, Nearest, "0x0.0", Equal),
        ("-0x1.0", "1000000", 300, Exact, "0x0.0", Equal),
    ];
    for &(x, b, prec, rm, out, o) in cases {
        let r = compound_prec_round_all(&hex(x), b.parse::<i64>().unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {b} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn compound_prec_round_fail_1() {
    BigExponentFloat::from(5).compound_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn compound_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).compound_prec_round(3, 10, Exact);
}

fn compound_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = compound_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = compound_prec_round_all(x, b, prec, Floor);
        let ceiling = compound_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&compound_prec_round_all(x, b, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = compound_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // For a moderate x > -1, (1 + x)^n is the nth power of the exact sum 1 + x. (At x = -1 the
    // conventions differ: (1 + x)^n is infinity for n < 0, while 0^n is NaN for odd n < 0, since
    // `BigExponentFloat` has no negative zero.)
    if x.is_finite() && exponent_is_moderate(x) && *x > -1i32 {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(&z.pow_s_prec_round_ref(b, prec, rm), &r));
    }
}

#[test]
fn compound_prec_round_properties() {
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, b, prec, rm)| {
            compound_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn compound_prec_round_float_agreement_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.compound_prec_round_ref(b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = compound_prec_round_all(&float_to_big_exponent_float(&x), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
