// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_13,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_11,
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

fn log_base_rational_base_prec_round_all(
    x: &BigExponentFloat,
    b: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_rational_base_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_rational_base_prec_round_ref(b, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_rational_base_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().log_base_rational_base_prec(b, prec),
            &r
        ));
        assert!(same_pair(&x.log_base_rational_base_prec_ref(b, prec), &r));
        let mut z = x.clone();
        let o = z.log_base_rational_base_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_rational_base_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let cases: &[(&str, &str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", "3/2", 1, Nearest, "0x2.0", Less),
        ("0x3.243f6a8885a30", "3/2", 10, Floor, "0x2.d2", Less),
        ("0x3.243f6a8885a30", "3/2", 10, Ceiling, "0x2.d3", Greater),
        ("0x3.243f6a8885a30", "3/2", 10, Down, "0x2.d2", Less),
        ("0x3.243f6a8885a30", "3/2", 10, Up, "0x2.d3", Greater),
        ("0x3.243f6a8885a30", "3/2", 10, Nearest, "0x2.d3", Greater),
        (
            "0x3.243f6a8885a30",
            "3/2",
            100,
            Nearest,
            "0x2.d2c0989018c6e00b6a8da65d4",
            Less,
        ),
        ("0x3.243f6a8885a30", "9", 1, Nearest, "0x0.8", Less),
        ("0x3.243f6a8885a30", "9", 10, Floor, "0x0.854", Less),
        ("0x3.243f6a8885a30", "9", 10, Ceiling, "0x0.858", Greater),
        ("0x3.243f6a8885a30", "9", 10, Down, "0x0.854", Less),
        ("0x3.243f6a8885a30", "9", 10, Up, "0x0.858", Greater),
        ("0x3.243f6a8885a30", "9", 10, Nearest, "0x0.854", Less),
        (
            "0x3.243f6a8885a30",
            "9",
            100,
            Nearest,
            "0x0.855f895f57dcad29b9983f0c6",
            Less,
        ),
        ("0x0.8", "3/2", 1, Nearest, "-0x2.0", Less),
        ("0x0.8", "3/2", 10, Floor, "-0x1.b6", Less),
        ("0x0.8", "3/2", 10, Ceiling, "-0x1.b58", Greater),
        ("0x0.8", "3/2", 10, Down, "-0x1.b58", Greater),
        ("0x0.8", "3/2", 10, Up, "-0x1.b6", Less),
        ("0x0.8", "3/2", 10, Nearest, "-0x1.b58", Greater),
        (
            "0x0.8",
            "3/2",
            100,
            Nearest,
            "-0x1.b5a288307f4a327a7634fec24",
            Less,
        ),
        ("0x0.8", "9", 1, Nearest, "-0x0.4", Greater),
        ("0x0.8", "9", 10, Floor, "-0x0.50e", Less),
        ("0x0.8", "9", 10, Ceiling, "-0x0.50c", Greater),
        ("0x0.8", "9", 10, Down, "-0x0.50c", Greater),
        ("0x0.8", "9", 10, Up, "-0x0.50e", Less),
        ("0x0.8", "9", 10, Nearest, "-0x0.50c", Greater),
        (
            "0x0.8",
            "9",
            100,
            Nearest,
            "-0x0.50c24e60d4d4f4a7021f57bbc8",
            Greater,
        ),
        ("0x3", "3/2", 1, Nearest, "0x2.0", Less),
        ("0x3", "3/2", 10, Floor, "0x2.b5", Less),
        ("0x3", "3/2", 10, Ceiling, "0x2.b6", Greater),
        ("0x3", "3/2", 10, Down, "0x2.b5", Less),
        ("0x3", "3/2", 10, Up, "0x2.b6", Greater),
        ("0x3", "3/2", 10, Nearest, "0x2.b6", Greater),
        (
            "0x3",
            "3/2",
            100,
            Nearest,
            "0x2.b5a288307f4a327a7634fec24",
            Greater,
        ),
        ("0x3", "9", 1, Nearest, "0x0.8", Equal),
        ("0x3", "9", 10, Floor, "0x0.8", Equal),
        ("0x3", "9", 10, Ceiling, "0x0.8", Equal),
        ("0x3", "9", 10, Down, "0x0.8", Equal),
        ("0x3", "9", 10, Up, "0x0.8", Equal),
        ("0x3", "9", 10, Nearest, "0x0.8", Equal),
        ("0x3", "9", 100, Nearest, "0x0.8", Equal),
        ("0x3", "9", 300, Exact, "0x0.8", Equal),
        ("0x9", "3/2", 1, Nearest, "0x4.0", Less),
        ("0x9", "3/2", 10, Floor, "0x5.6a", Less),
        ("0x9", "3/2", 10, Ceiling, "0x5.6c", Greater),
        ("0x9", "3/2", 10, Down, "0x5.6a", Less),
        ("0x9", "3/2", 10, Up, "0x5.6c", Greater),
        ("0x9", "3/2", 10, Nearest, "0x5.6c", Greater),
        (
            "0x9",
            "3/2",
            100,
            Nearest,
            "0x5.6b451060fe9464f4ec69fd848",
            Greater,
        ),
        ("0x9", "9", 1, Nearest, "0x1.0", Equal),
        ("0x9", "9", 10, Floor, "0x1.0", Equal),
        ("0x9", "9", 10, Ceiling, "0x1.0", Equal),
        ("0x9", "9", 10, Down, "0x1.0", Equal),
        ("0x9", "9", 10, Up, "0x1.0", Equal),
        ("0x9", "9", 10, Nearest, "0x1.0", Equal),
        ("0x9", "9", 100, Nearest, "0x1.0", Equal),
        ("0x9", "9", 300, Exact, "0x1.0", Equal),
        ("0x1b", "3/2", 1, Nearest, "0x8.0", Less),
        ("0x1b", "3/2", 10, Floor, "0x8.2", Less),
        ("0x1b", "3/2", 10, Ceiling, "0x8.24", Greater),
        ("0x1b", "3/2", 10, Down, "0x8.2", Less),
        ("0x1b", "3/2", 10, Up, "0x8.24", Greater),
        ("0x1b", "3/2", 10, Nearest, "0x8.2", Less),
        (
            "0x1b",
            "3/2",
            100,
            Nearest,
            "0x8.20e798917dde976f629efc47",
            Greater,
        ),
        ("0x1b", "9", 1, Nearest, "0x2.0", Greater),
        ("0x1b", "9", 10, Floor, "0x1.8", Equal),
        ("0x1b", "9", 10, Ceiling, "0x1.8", Equal),
        ("0x1b", "9", 10, Down, "0x1.8", Equal),
        ("0x1b", "9", 10, Up, "0x1.8", Equal),
        ("0x1b", "9", 10, Nearest, "0x1.8", Equal),
        ("0x1b", "9", 100, Nearest, "0x1.8", Equal),
        ("0x1b", "9", 300, Exact, "0x1.8", Equal),
        ("0x2.0E+20", "3/2", 1, Nearest, "0x8.0E+1", Less),
        ("0x2.0E+20", "3/2", 10, Floor, "0x8a.4", Less),
        ("0x2.0E+20", "3/2", 10, Ceiling, "0x8a.8", Greater),
        ("0x2.0E+20", "3/2", 10, Down, "0x8a.4", Less),
        ("0x2.0E+20", "3/2", 10, Up, "0x8a.8", Greater),
        ("0x2.0E+20", "3/2", 10, Nearest, "0x8a.8", Greater),
        (
            "0x2.0E+20",
            "3/2",
            100,
            Nearest,
            "0x8a.786d17584679f8bf66c49b7",
            Less,
        ),
        ("0x2.0E+20", "9", 1, Nearest, "0x2.0E+1", Greater),
        ("0x2.0E+20", "9", 10, Floor, "0x19.88", Less),
        ("0x2.0E+20", "9", 10, Ceiling, "0x19.9", Greater),
        ("0x2.0E+20", "9", 10, Down, "0x19.88", Less),
        ("0x2.0E+20", "9", 10, Up, "0x19.9", Greater),
        ("0x2.0E+20", "9", 10, Nearest, "0x19.9", Greater),
        (
            "0x2.0E+20",
            "9",
            100,
            Nearest,
            "0x19.8d7acca3576168d7abeac26a",
            Less,
        ),
        ("0x1.0E+300000000", "3/2", 1, Nearest, "0x8.0E+7", Greater),
        ("0x1.0E+300000000", "3/2", 10, Floor, "0x7.a4E+7", Less),
        ("0x1.0E+300000000", "3/2", 10, Ceiling, "0x7.a6E+7", Greater),
        ("0x1.0E+300000000", "3/2", 10, Down, "0x7.a4E+7", Less),
        ("0x1.0E+300000000", "3/2", 10, Up, "0x7.a6E+7", Greater),
        ("0x1.0E+300000000", "3/2", 10, Nearest, "0x7.a4E+7", Less),
        (
            "0x1.0E+300000000",
            "3/2",
            100,
            Nearest,
            "0x7a46162d.9f2aba72f67d8b0a3",
            Less,
        ),
        ("0x1.0E+300000000", "9", 1, Nearest, "0x1.0E+7", Less),
        ("0x1.0E+300000000", "9", 10, Floor, "0x1.69E+7", Less),
        ("0x1.0E+300000000", "9", 10, Ceiling, "0x1.698E+7", Greater),
        ("0x1.0E+300000000", "9", 10, Down, "0x1.69E+7", Less),
        ("0x1.0E+300000000", "9", 10, Up, "0x1.698E+7", Greater),
        ("0x1.0E+300000000", "9", 10, Nearest, "0x1.69E+7", Less),
        (
            "0x1.0E+300000000",
            "9",
            100,
            Nearest,
            "0x1690559c.24936bb6d6cd22e626",
            Greater,
        ),
        ("0x1.8E-300000000", "3/2", 1, Nearest, "-0x8.0E+7", Less),
        ("0x1.8E-300000000", "3/2", 10, Floor, "-0x7.a6E+7", Less),
        (
            "0x1.8E-300000000",
            "3/2",
            10,
            Ceiling,
            "-0x7.a4E+7",
            Greater,
        ),
        ("0x1.8E-300000000", "3/2", 10, Down, "-0x7.a4E+7", Greater),
        ("0x1.8E-300000000", "3/2", 10, Up, "-0x7.a6E+7", Less),
        (
            "0x1.8E-300000000",
            "3/2",
            10,
            Nearest,
            "-0x7.a4E+7",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "3/2",
            100,
            Nearest,
            "-0x7a46162c.9f2aba72f67d8b0a3",
            Greater,
        ),
        ("0x1.8E-300000000", "9", 1, Nearest, "-0x1.0E+7", Greater),
        ("0x1.8E-300000000", "9", 10, Floor, "-0x1.698E+7", Less),
        ("0x1.8E-300000000", "9", 10, Ceiling, "-0x1.69E+7", Greater),
        ("0x1.8E-300000000", "9", 10, Down, "-0x1.69E+7", Greater),
        ("0x1.8E-300000000", "9", 10, Up, "-0x1.698E+7", Less),
        ("0x1.8E-300000000", "9", 10, Nearest, "-0x1.69E+7", Greater),
        (
            "0x1.8E-300000000",
            "9",
            100,
            Nearest,
            "-0x1690559b.f555ba17aba2178d28",
            Less,
        ),
        ("-0x3", "3/2", 1, Nearest, "NaN", Equal),
        ("-0x3", "3/2", 10, Floor, "NaN", Equal),
        ("-0x3", "3/2", 10, Ceiling, "NaN", Equal),
        ("-0x3", "3/2", 10, Down, "NaN", Equal),
        ("-0x3", "3/2", 10, Up, "NaN", Equal),
        ("-0x3", "3/2", 10, Nearest, "NaN", Equal),
        ("-0x3", "3/2", 100, Nearest, "NaN", Equal),
        ("-0x3", "3/2", 300, Exact, "NaN", Equal),
        ("-0x3", "9", 1, Nearest, "NaN", Equal),
        ("-0x3", "9", 10, Floor, "NaN", Equal),
        ("-0x3", "9", 10, Ceiling, "NaN", Equal),
        ("-0x3", "9", 10, Down, "NaN", Equal),
        ("-0x3", "9", 10, Up, "NaN", Equal),
        ("-0x3", "9", 10, Nearest, "NaN", Equal),
        ("-0x3", "9", 100, Nearest, "NaN", Equal),
        ("-0x3", "9", 300, Exact, "NaN", Equal),
        ("0x1.0", "3/2", 1, Nearest, "0x0.0", Equal),
        ("0x1.0", "3/2", 10, Floor, "0x0.0", Equal),
        ("0x1.0", "3/2", 10, Ceiling, "0x0.0", Equal),
        ("0x1.0", "3/2", 10, Down, "0x0.0", Equal),
        ("0x1.0", "3/2", 10, Up, "0x0.0", Equal),
        ("0x1.0", "3/2", 10, Nearest, "0x0.0", Equal),
        ("0x1.0", "3/2", 100, Nearest, "0x0.0", Equal),
        ("0x1.0", "3/2", 300, Exact, "0x0.0", Equal),
        ("0x1.0", "9", 1, Nearest, "0x0.0", Equal),
        ("0x1.0", "9", 10, Floor, "0x0.0", Equal),
        ("0x1.0", "9", 10, Ceiling, "0x0.0", Equal),
        ("0x1.0", "9", 10, Down, "0x0.0", Equal),
        ("0x1.0", "9", 10, Up, "0x0.0", Equal),
        ("0x1.0", "9", 10, Nearest, "0x0.0", Equal),
        ("0x1.0", "9", 100, Nearest, "0x0.0", Equal),
        ("0x1.0", "9", 300, Exact, "0x0.0", Equal),
        ("0x0.ffffffff", "3/2", 1, Nearest, "-0x2.0E-8", Greater),
        ("0x0.ffffffff", "3/2", 10, Floor, "-0x2.78E-8", Less),
        ("0x0.ffffffff", "3/2", 10, Ceiling, "-0x2.77E-8", Greater),
        ("0x0.ffffffff", "3/2", 10, Down, "-0x2.77E-8", Greater),
        ("0x0.ffffffff", "3/2", 10, Up, "-0x2.78E-8", Less),
        ("0x0.ffffffff", "3/2", 10, Nearest, "-0x2.77E-8", Greater),
        (
            "0x0.ffffffff",
            "3/2",
            100,
            Nearest,
            "-0x2.775fa9ea26c8326f500b859d4E-8",
            Greater,
        ),
        ("0x0.ffffffff", "9", 1, Nearest, "-0x8.0E-9", Less),
        ("0x0.ffffffff", "9", 10, Floor, "-0x7.4aE-9", Less),
        ("0x0.ffffffff", "9", 10, Ceiling, "-0x7.48E-9", Greater),
        ("0x0.ffffffff", "9", 10, Down, "-0x7.48E-9", Greater),
        ("0x0.ffffffff", "9", 10, Up, "-0x7.4aE-9", Less),
        ("0x0.ffffffff", "9", 10, Nearest, "-0x7.48E-9", Greater),
        (
            "0x0.ffffffff",
            "9",
            100,
            Nearest,
            "-0x7.482b80f2d46a6cd6b99b3a298E-9",
            Greater,
        ),
        ("0x2540be400.0", "3/2", 1, Nearest, "0x4.0E+1", Greater),
        ("0x2540be400.0", "3/2", 10, Floor, "0x38.c", Less),
        ("0x2540be400.0", "3/2", 10, Ceiling, "0x38.d", Greater),
        ("0x2540be400.0", "3/2", 10, Down, "0x38.c", Less),
        ("0x2540be400.0", "3/2", 10, Up, "0x38.d", Greater),
        ("0x2540be400.0", "3/2", 10, Nearest, "0x38.d", Greater),
        (
            "0x2540be400.0",
            "3/2",
            100,
            Nearest,
            "0x38.c9ea981a53037ce6cec01c18",
            Greater,
        ),
        ("0x2540be400.0", "9", 1, Nearest, "0x8.0", Less),
        ("0x2540be400.0", "9", 10, Floor, "0xa.78", Less),
        ("0x2540be400.0", "9", 10, Ceiling, "0xa.7c", Greater),
        ("0x2540be400.0", "9", 10, Down, "0xa.78", Less),
        ("0x2540be400.0", "9", 10, Up, "0xa.7c", Greater),
        ("0x2540be400.0", "9", 10, Nearest, "0xa.7c", Greater),
        (
            "0x2540be400.0",
            "9",
            100,
            Nearest,
            "0xa.7ac195bd42d8ede0e2687ca6",
            Greater,
        ),
        (
            "0x3.a0666ba2118E+294267957",
            "3/2",
            10,
            Nearest,
            "0x7.8E+7",
            Greater,
        ),
        (
            "0x4.6971b10edfE-294267958",
            "3/2",
            10,
            Nearest,
            "-0x7.8E+7",
            Less,
        ),
        (
            "0x3.081e8996b88E+299788285",
            "3/2",
            10,
            Nearest,
            "0x7.a2E+7",
            Less,
        ),
        (
            "0x5.470ccff4e5E-299788286",
            "3/2",
            10,
            Nearest,
            "-0x7.a2E+7",
            Greater,
        ),
        (
            "0xf.965c71421bE+283227126",
            "3/2",
            10,
            Nearest,
            "0x7.38E+7",
            Greater,
        ),
        (
            "0x1.06c6f7ee5ebE-283227127",
            "3/2",
            10,
            Nearest,
            "-0x7.38E+7",
            Less,
        ),
        (
            "0x6.d6ea776b1dcE+278320101",
            "3/2",
            10,
            Nearest,
            "0x7.16E+7",
            Less,
        ),
        (
            "0x2.56df739f6e6E-278320102",
            "3/2",
            10,
            Nearest,
            "-0x7.16E+7",
            Greater,
        ),
        (
            "0x9.ab7e893dea08E+304695342",
            "3/2",
            10,
            Nearest,
            "0x7.c4E+7",
            Greater,
        ),
        (
            "0x1.a7951d3907eaE-304695343",
            "3/2",
            10,
            Nearest,
            "-0x7.c4E+7",
            Less,
        ),
        (
            "0x4.48b122ea098E+279546858",
            "3/2",
            10,
            Nearest,
            "0x7.1eE+7",
            Less,
        ),
        (
            "0x3.bc2054d88346E-279546859",
            "3/2",
            10,
            Nearest,
            "-0x7.1eE+7",
            Greater,
        ),
        (
            "0x8.3ed6e6d496e58E+308988985",
            "3/2",
            10,
            Nearest,
            "0x7.eE+7",
            Greater,
        ),
        (
            "0x1.f0c2003b53ceE-308988986",
            "3/2",
            10,
            Nearest,
            "-0x7.eE+7",
            Less,
        ),
        (
            "0x1.db705130e53aeE+285987324",
            "3/2",
            10,
            Nearest,
            "0x7.48E+7",
            Less,
        ),
        (
            "0x8.9d7dc585f3d9E-285987325",
            "3/2",
            10,
            Nearest,
            "-0x7.48E+7",
            Greater,
        ),
        (
            "0x5.3f920d8d1fE+297181501",
            "3/2",
            10,
            Floor,
            "0x7.92E+7",
            Less,
        ),
        (
            "0x3.0c709b40878E-297181502",
            "3/2",
            10,
            Floor,
            "-0x7.94E+7",
            Less,
        ),
        (
            "0x5.3f920d8d1fE+297181501",
            "3/2",
            10,
            Ceiling,
            "0x7.94E+7",
            Greater,
        ),
        (
            "0x3.0c709b40878E-297181502",
            "3/2",
            10,
            Ceiling,
            "-0x7.92E+7",
            Greater,
        ),
        (
            "0x2.21363204a8E+302088451",
            "3/2",
            10,
            Floor,
            "0x7.bE+7",
            Less,
        ),
        (
            "0x7.833edf1a9fE-302088452",
            "3/2",
            10,
            Floor,
            "-0x7.b2E+7",
            Less,
        ),
        (
            "0x2.21363204a8E+302088451",
            "3/2",
            10,
            Ceiling,
            "0x7.b2E+7",
            Greater,
        ),
        (
            "0x7.833edf1a9fE-302088452",
            "3/2",
            10,
            Ceiling,
            "-0x7.bE+7",
            Greater,
        ),
        (
            "0x1.fcc062ba972cE+300861734",
            "3/2",
            10,
            Floor,
            "0x7.aaE+7",
            Less,
        ),
        (
            "0x8.0d13b29247cE-300861735",
            "3/2",
            10,
            Floor,
            "-0x7.acE+7",
            Less,
        ),
        (
            "0x1.fcc062ba972cE+300861734",
            "3/2",
            10,
            Ceiling,
            "0x7.acE+7",
            Greater,
        ),
        (
            "0x8.0d13b29247cE-300861735",
            "3/2",
            10,
            Ceiling,
            "-0x7.aaE+7",
            Greater,
        ),
        (
            "0x1.19102483e4e4E+302088485",
            "3/2",
            10,
            Floor,
            "0x7.bE+7",
            Less,
        ),
        (
            "0xe.92bfffde2faE-302088486",
            "3/2",
            10,
            Floor,
            "-0x7.b2E+7",
            Less,
        ),
        (
            "0x1.19102483e4e4E+302088485",
            "3/2",
            10,
            Ceiling,
            "0x7.b2E+7",
            Greater,
        ),
        (
            "0xe.92bfffde2faE-302088486",
            "3/2",
            10,
            Ceiling,
            "-0x7.bE+7",
            Greater,
        ),
        (
            "0xe.b3d17e5166d8E+306995508",
            "3/2",
            10,
            Floor,
            "0x7.d2E+7",
            Less,
        ),
        (
            "0x1.1697fb67a2b4E-306995509",
            "3/2",
            10,
            Floor,
            "-0x7.d4E+7",
            Less,
        ),
        (
            "0xe.b3d17e5166d8E+306995508",
            "3/2",
            10,
            Ceiling,
            "0x7.d4E+7",
            Greater,
        ),
        (
            "0x1.1697fb67a2b4E-306995509",
            "3/2",
            10,
            Ceiling,
            "-0x7.d2E+7",
            Greater,
        ),
        (
            "0x3.f2efa9ff0278E+285527290",
            "3/2",
            10,
            Floor,
            "0x7.44E+7",
            Less,
        ),
        (
            "0x4.0d3b8d97e53cE-285527291",
            "3/2",
            10,
            Floor,
            "-0x7.46E+7",
            Less,
        ),
        (
            "0x3.f2efa9ff0278E+285527290",
            "3/2",
            10,
            Ceiling,
            "0x7.46E+7",
            Greater,
        ),
        (
            "0x4.0d3b8d97e53cE-285527291",
            "3/2",
            10,
            Ceiling,
            "-0x7.44E+7",
            Greater,
        ),
        (
            "0x6.0b30b7453083cE+294727955",
            "3/2",
            10,
            Floor,
            "0x7.82E+7",
            Less,
        ),
        (
            "0x2.a5baab13be83E-294727956",
            "3/2",
            10,
            Floor,
            "-0x7.84E+7",
            Less,
        ),
        (
            "0x6.0b30b7453083cE+294727955",
            "3/2",
            10,
            Ceiling,
            "0x7.84E+7",
            Greater,
        ),
        (
            "0x2.a5baab13be83E-294727956",
            "3/2",
            10,
            Ceiling,
            "-0x7.82E+7",
            Greater,
        ),
        (
            "0x8.0fd960d541ebE+313129284",
            "3/2",
            10,
            Floor,
            "0x7.f8E+7",
            Less,
        ),
        (
            "0x1.fc1171eb7b636E-313129285",
            "3/2",
            10,
            Floor,
            "-0x7.faE+7",
            Less,
        ),
        (
            "0x8.0fd960d541ebE+313129284",
            "3/2",
            10,
            Ceiling,
            "0x7.faE+7",
            Greater,
        ),
        (
            "0x1.fc1171eb7b636E-313129285",
            "3/2",
            10,
            Ceiling,
            "-0x7.f8E+7",
            Greater,
        ),
        (
            "0xc.dd0a36518ac887a6996715ae8d75a6b75E+281621174",
            "3/2",
            100,
            Nearest,
            "0x72c86f49.a581a1d4d1c1caa5b",
            Greater,
        ),
        (
            "0x1.3e6c0142c2609357dd35c1da78e0224556E-281621175",
            "3/2",
            100,
            Nearest,
            "-0x72c86f49.a581a1d4d1c1caa5b",
            Less,
        ),
        (
            "0x1.e4c036c3f1c4d9ba54be98804dca83d194E+289194514",
            "3/2",
            100,
            Nearest,
            "0x75dea39d.e4eb502b2dc93902a",
            Less,
        ),
        (
            "0x8.731fa7eff8eaa77afcf0226139f0d05a2E-289194515",
            "3/2",
            100,
            Nearest,
            "-0x75dea39d.e4eb502b2dc93902a",
            Greater,
        ),
        (
            "0x1.317e0044ad41fb2f507f88f707c5a31aa02E+311517615",
            "3/2",
            100,
            Nearest,
            "0x7ef7d681.a01b07e241716cf328",
            Greater,
        ),
        (
            "0xd.686a418e9ff20ef9bedaefcebd17d44f65E-311517616",
            "3/2",
            100,
            Nearest,
            "-0x7ef7d681.a01b07e241716cf328",
            Less,
        ),
        (
            "0x1.bc4a337ed5c7974cbd3639cb5732c44d4ccE+275987646",
            "3/2",
            100,
            Nearest,
            "0x707ca175.cd3c8ac8da20b6dae",
            Less,
        ),
        (
            "0x9.381df0337a9c4f06e9222d8457a230ecd7E-275987647",
            "3/2",
            100,
            Nearest,
            "-0x707ca175.cd3c8ac8da20b6dae",
            Greater,
        ),
        (
            "0x9.a21b457c98d3fb8a75575a12111b3f1f1ccE+288401089",
            "3/2",
            100,
            Nearest,
            "0x758bda5d.ebc2afca0f1b0db398",
            Greater,
        ),
        (
            "0x1.a931e71e70ca5a8fdd1d06a2af0beaa16278E-288401090",
            "3/2",
            100,
            Nearest,
            "-0x758bda5d.ebc2afca0f1b0db398",
            Less,
        ),
        (
            "0x8.e9afc949374ade11e5d0f8242cba6ab2c1E+313009880",
            "3/2",
            100,
            Nearest,
            "0x7f938a96.0da388b6700037dba8",
            Less,
        ),
        (
            "0x1.cb8fd13d02aa10d7e81603176b49673d43bE-313009881",
            "3/2",
            100,
            Nearest,
            "-0x7f938a96.0da388b6700037dba8",
            Greater,
        ),
        (
            "0x3.19d9a53f1fc0a554a9f0fe2b44fd28de059cE+307661643",
            "3/2",
            100,
            Nearest,
            "0x7d658135.49c2310bf045740f08",
            Greater,
        ),
        (
            "0x5.28dfb98a6fc125ba4b3b36dbeba47c75f9feE-307661644",
            "3/2",
            100,
            Nearest,
            "-0x7d658135.49c2310bf045740f08",
            Less,
        ),
        (
            "0x3.fabe1378685c157ee68f3024a05b892303cE+310256936",
            "3/2",
            100,
            Nearest,
            "0x7e744c60.a79db00f3130568af",
            Less,
        ),
        (
            "0x4.0548deb69d7809b0730139fdbf82b95c7466E-310256937",
            "3/2",
            100,
            Nearest,
            "-0x7e744c60.a79db00f3130568af",
            Greater,
        ),
        (
            "0x5.090d8e3d323860965ecf607d3202141c5E+275105387",
            "3/2",
            100,
            Floor,
            "0x70209359.8a086be6d972347bf8",
            Less,
        ),
        (
            "0x3.2d72618f061d84ad131ce9065a7ee29484E-275105388",
            "3/2",
            100,
            Floor,
            "-0x70209359.8a086be6d972347c",
            Less,
        ),
        (
            "0x5.090d8e3d323860965ecf607d3202141c5E+275105387",
            "3/2",
            100,
            Ceiling,
            "0x70209359.8a086be6d972347c",
            Greater,
        ),
        (
            "0x3.2d72618f061d84ad131ce9065a7ee29484E-275105388",
            "3/2",
            100,
            Ceiling,
            "-0x70209359.8a086be6d972347bf8",
            Greater,
        ),
        (
            "0x2.977e8a37bd3876dee43c828eb59b62e634E+299511456",
            "3/2",
            100,
            Floor,
            "0x7a131ca2.08815fb9e61f4b391",
            Less,
        ),
        (
            "0x6.2c6264824670dc7311d065ce890608e5eE-299511457",
            "3/2",
            100,
            Floor,
            "-0x7a131ca2.08815fb9e61f4b3918",
            Less,
        ),
        (
            "0x2.977e8a37bd3876dee43c828eb59b62e634E+299511456",
            "3/2",
            100,
            Ceiling,
            "0x7a131ca2.08815fb9e61f4b3918",
            Greater,
        ),
        (
            "0x6.2c6264824670dc7311d065ce890608e5eE-299511457",
            "3/2",
            100,
            Ceiling,
            "-0x7a131ca2.08815fb9e61f4b391",
            Greater,
        ),
        (
            "0xe.7e4aa213ec1f6edf370dbb493ce1cc1e2fE+292146090",
            "3/2",
            100,
            Floor,
            "0x77129b84.e92ccf6cf6bbb34ee8",
            Less,
        ),
        (
            "0x1.1a9ce295f57079a2a28835da8bcdb93389cE-292146091",
            "3/2",
            100,
            Floor,
            "-0x77129b84.e92ccf6cf6bbb34ef",
            Less,
        ),
        (
            "0xe.7e4aa213ec1f6edf370dbb493ce1cc1e2fE+292146090",
            "3/2",
            100,
            Ceiling,
            "0x77129b84.e92ccf6cf6bbb34ef",
            Greater,
        ),
        (
            "0x1.1a9ce295f57079a2a28835da8bcdb93389cE-292146091",
            "3/2",
            100,
            Ceiling,
            "-0x77129b84.e92ccf6cf6bbb34ee8",
            Greater,
        ),
        (
            "0x2.818c4b9f54ed4f3329363b7b907eaf38c44E+292061176",
            "3/2",
            100,
            Floor,
            "0x7709bf5a.d0ba9a46a2870b8a5",
            Less,
        ),
        (
            "0x6.627254ee587a5ef3d2e34fc627224c945dE-292061177",
            "3/2",
            100,
            Floor,
            "-0x7709bf5a.d0ba9a46a2870b8a58",
            Less,
        ),
        (
            "0x2.818c4b9f54ed4f3329363b7b907eaf38c44E+292061176",
            "3/2",
            100,
            Ceiling,
            "0x7709bf5a.d0ba9a46a2870b8a58",
            Greater,
        ),
        (
            "0x6.627254ee587a5ef3d2e34fc627224c945dE-292061177",
            "3/2",
            100,
            Ceiling,
            "-0x7709bf5a.d0ba9a46a2870b8a5",
            Greater,
        ),
        (
            "0x5.e3dd1509dbbf5b401ecff3d82276031a356E+280327119",
            "3/2",
            100,
            Floor,
            "0x724169a1.2efb04e57b152a1908",
            Less,
        ),
        (
            "0x2.b767b2eee25c714a889486feba686caa59bE-280327120",
            "3/2",
            100,
            Floor,
            "-0x724169a1.2efb04e57b152a191",
            Less,
        ),
        (
            "0x5.e3dd1509dbbf5b401ecff3d82276031a356E+280327119",
            "3/2",
            100,
            Ceiling,
            "0x724169a1.2efb04e57b152a191",
            Greater,
        ),
        (
            "0x2.b767b2eee25c714a889486feba686caa59bE-280327120",
            "3/2",
            100,
            Ceiling,
            "-0x724169a1.2efb04e57b152a1908",
            Greater,
        ),
        (
            "0x2.90361284986598c4dfa39bbfc975d3d11afE+292922690",
            "3/2",
            100,
            Floor,
            "0x7763a35a.8382a00c90b0f928f",
            Less,
        ),
        (
            "0x6.3decad758304ac983ce6a41aea5a393993E-292922691",
            "3/2",
            100,
            Floor,
            "-0x7763a35a.8382a00c90b0f928f8",
            Less,
        ),
        (
            "0x2.90361284986598c4dfa39bbfc975d3d11afE+292922690",
            "3/2",
            100,
            Ceiling,
            "0x7763a35a.8382a00c90b0f928f8",
            Greater,
        ),
        (
            "0x6.3decad758304ac983ce6a41aea5a393993E-292922691",
            "3/2",
            100,
            Ceiling,
            "-0x7763a35a.8382a00c90b0f928f",
            Greater,
        ),
        (
            "0x1.0744ccf9b8cee9957daca67f6f6312a974238E+303403610",
            "3/2",
            100,
            Floor,
            "0x7ba93864.98a2239e3b31799978",
            Less,
        ),
        (
            "0xf.8ee9348f0e9901ef9f2e1a4acf9b1b03be24E-303403611",
            "3/2",
            100,
            Floor,
            "-0x7ba93864.98a2239e3b3179998",
            Less,
        ),
        (
            "0x1.0744ccf9b8cee9957daca67f6f6312a974238E+303403610",
            "3/2",
            100,
            Ceiling,
            "0x7ba93864.98a2239e3b3179998",
            Greater,
        ),
        (
            "0xf.8ee9348f0e9901ef9f2e1a4acf9b1b03be24E-303403611",
            "3/2",
            100,
            Ceiling,
            "-0x7ba93864.98a2239e3b31799978",
            Greater,
        ),
        (
            "0x8.a17b13c5e301b698469608b45cf55ca7c684E+307811248",
            "3/2",
            100,
            Floor,
            "0x7d751d55.8f44be9bae11375e9",
            Less,
        ),
        (
            "0x1.da948d266529d6d02d25eb26b540b477a74b8E-307811249",
            "3/2",
            100,
            Floor,
            "-0x7d751d55.8f44be9bae11375e98",
            Less,
        ),
        (
            "0x8.a17b13c5e301b698469608b45cf55ca7c684E+307811248",
            "3/2",
            100,
            Ceiling,
            "0x7d751d55.8f44be9bae11375e98",
            Greater,
        ),
        (
            "0x1.da948d266529d6d02d25eb26b540b477a74b8E-307811249",
            "3/2",
            100,
            Ceiling,
            "-0x7d751d55.8f44be9bae11375e9",
            Greater,
        ),
    ];
    for &(x, b, prec, rm, out, o) in cases {
        let r = log_base_rational_base_prec_round_all(
            &hex(x),
            &Rational::from_str(b).unwrap(),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out, "{x} {b} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_rational_base_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_rational_base_prec_round(
        &Rational::from_unsigneds(3u8, 2),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn log_base_rational_base_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_rational_base_prec_round(
        &Rational::from_unsigneds(3u8, 2),
        10,
        Exact,
    );
}

#[test]
#[should_panic]
fn log_base_rational_base_prec_round_fail_3() {
    BigExponentFloat::from(5).log_base_rational_base_prec_round(&Rational::ONE, 10, Floor);
}

fn log_base_rational_base_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: &Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_rational_base_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_rational_base_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_rational_base_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_rational_base_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_rational_base_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // An integer base gives the same result as the `u64` base.
    if let Ok(k) = u64::try_from(b)
        && k > 1
    {
        assert!(same_pair(&r, &x.log_base_prec_round_ref(k, prec, rm)));
    }
}

#[test]
fn log_base_rational_base_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, b, prec, rm)| {
            log_base_rational_base_prec_round_properties_helper(&x, &b, prec, rm);
        },
    );
}

#[test]
fn log_base_rational_base_prec_round_float_agreement_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_rational_base_prec_round_ref(&b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_rational_base_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &b,
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
