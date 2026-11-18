// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, wright_omega_prec_round_naive,
    wright_omega_rational_prec_round_naive,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_25, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_25,
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

// Whether the reference, whose time grows with the argument, is run on `x`.
fn reference_is_cheap(x: &BigExponentFloat) -> bool {
    x.get_exponent().is_none_or(|e| *e <= 8u32)
}

fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn wright_omega_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().wright_omega_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.wright_omega_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.wright_omega_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().wright_omega_prec(prec), &r));
        assert!(same_pair(&x.wright_omega_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.wright_omega_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn wright_omega_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::wright_omega_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::wright_omega_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::wright_omega_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::wright_omega_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_wright_omega_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &wright_omega_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &wright_omega_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &wright_omega_prec_round_all(&ninf, 10, rm),
            &(zero.clone(), Equal)
        ));
        // omega(1) = 1
        assert!(same_pair(
            &wright_omega_prec_round_all(&BigExponentFloat::ONE, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
        assert!(same_pair(
            &wright_omega_rational_prec_round_all(&Rational::ONE, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
    }
    // The expected values agree with Azurite's verified Wright omega function.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = wright_omega_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 1, Nearest, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 64, Nearest, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Floor, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Down, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Up, "1.0", "0x1.0", Equal);
    test("1.0", "0x1.0", 10, Nearest, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 1, Nearest, "1.0", "0x1.0", Greater);
    test(
        "0.50",
        "0x0.8",
        64,
        Nearest,
        "0.766248608161750258854",
        "0x0.c428de68a9053aa7",
        Less,
    );
    test("0.50", "0x0.8", 10, Floor, "0.766", "0x0.c4", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.76660", "0x0.c44", Greater);
    test("0.50", "0x0.8", 10, Down, "0.766", "0x0.c4", Less);
    test("0.50", "0x0.8", 10, Up, "0.76660", "0x0.c44", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.76660", "0x0.c44", Greater);
    test("2.0", "0x2.0", 1, Nearest, "2.0", "0x2.0", Greater);
    test(
        "2.0",
        "0x2.0",
        64,
        Nearest,
        "1.55714559899761141690",
        "0x1.8ea1180ece192912",
        Greater,
    );
    test("2.0", "0x2.0", 10, Floor, "1.5566", "0x1.8e8", Less);
    test("2.0", "0x2.0", 10, Ceiling, "1.559", "0x1.8f", Greater);
    test("2.0", "0x2.0", 10, Down, "1.5566", "0x1.8e8", Less);
    test("2.0", "0x2.0", 10, Up, "1.559", "0x1.8f", Greater);
    test("2.0", "0x2.0", 10, Nearest, "1.5566", "0x1.8e8", Less);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        64,
        Nearest,
        "2.01677976489220052761",
        "0x2.044badbd72526d3c",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "2.016",
        "0x2.04",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "2.0195",
        "0x2.05",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Down,
        "2.016",
        "0x2.04",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Up,
        "2.0195",
        "0x2.05",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Nearest,
        "2.016",
        "0x2.04",
        Less,
    );
    test("10.0", "0xa.0", 1, Nearest, "8.0", "0x8.0", Greater);
    test(
        "10.0",
        "0xa.0",
        64,
        Nearest,
        "7.9294200950196973487",
        "0x7.edee79b05ad4ff7",
        Greater,
    );
    test("10.0", "0xa.0", 10, Floor, "7.922", "0x7.ec", Less);
    test("10.0", "0xa.0", 10, Ceiling, "7.9297", "0x7.ee", Greater);
    test("10.0", "0xa.0", 10, Down, "7.922", "0x7.ec", Less);
    test("10.0", "0xa.0", 10, Up, "7.9297", "0x7.ee", Greater);
    test("10.0", "0xa.0", 10, Nearest, "7.9297", "0x7.ee", Greater);
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        1,
        Nearest,
        "8.6e9",
        "0x2.0E+8",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        64,
        Nearest,
        "9999999976.9741490725",
        "0x2540be3e8.f961d568",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Floor,
        "9.999e9",
        "0x2.54E+8",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Ceiling,
        "1.0016e10",
        "0x2.55E+8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Down,
        "9.999e9",
        "0x2.54E+8",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Up,
        "1.0016e10",
        "0x2.55E+8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Nearest,
        "9.999e9",
        "0x2.54E+8",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        64,
        Nearest,
        "0.567146909379718001086",
        "0x0.91308a33d038a73d",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Floor,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Ceiling,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Down,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Up,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Nearest,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        1,
        Nearest,
        "8.7e99",
        "0x1.0E+83",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        64,
        Nearest,
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Floor,
        "9.9964e99",
        "0x1.248E+83",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Ceiling,
        "1.001e100",
        "0x1.25E+83",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Down,
        "9.9964e99",
        "0x1.248E+83",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Up,
        "1.001e100",
        "0x1.25E+83",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Nearest,
        "9.9964e99",
        "0x1.248E+83",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Floor,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Ceiling,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Down,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Up,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Nearest,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        64,
        Nearest,
        "118.679572804144489957",
        "0x76.adf87bb90d35808",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Floor,
        "118.62",
        "0x76.a",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Ceiling,
        "118.8",
        "0x76.c",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Down,
        "118.62",
        "0x76.a",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Up,
        "118.8",
        "0x76.c",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Nearest,
        "118.62",
        "0x76.a",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        64,
        Nearest,
        "0.53169161977901767911",
        "0x0.881cf1268215f75e",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "0.531",
        "0x0.88",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "0.53223",
        "0x0.884",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "0.531",
        "0x0.88",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "0.53223",
        "0x0.884",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "0.531",
        "0x0.88",
        Less,
    );
    test("-0.25", "-0x0.4", 1, Nearest, "0.50", "0x0.8", Greater);
    test(
        "-0.25",
        "-0x0.4",
        64,
        Nearest,
        "0.48128846639758644398",
        "0x0.7b35b88f1e9ffcb6",
        Greater,
    );
    test("-0.25", "-0x0.4", 10, Floor, "0.48096", "0x0.7b2", Less);
    test("-0.25", "-0x0.4", 10, Ceiling, "0.4814", "0x0.7b4", Greater);
    test("-0.25", "-0x0.4", 10, Down, "0.48096", "0x0.7b2", Less);
    test("-0.25", "-0x0.4", 10, Up, "0.4814", "0x0.7b4", Greater);
    test("-0.25", "-0x0.4", 10, Nearest, "0.4814", "0x0.7b4", Greater);
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        64,
        Nearest,
        "0.46522789553303524953",
        "0x0.77192ce480549972",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Floor,
        "0.4648",
        "0x0.77",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Ceiling,
        "0.46533",
        "0x0.772",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Down,
        "0.4648",
        "0x0.77",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Up,
        "0.46533",
        "0x0.772",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Nearest,
        "0.46533",
        "0x0.772",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        64,
        Nearest,
        "0.56713967145458530597",
        "0x0.913010c529607e94",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Floor,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Ceiling,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Down,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Up,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Nearest,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test("1.0", "0x1.0", 1, Down, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 1, Down, "0.50", "0x0.8", Less);
    test("-0.25", "-0x0.4", 1, Down, "0.25", "0x0.4", Less);
    test(
        "2.3281685280997921493337341223879518220630854242143616671023085e4835",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E+4015",
        84,
        Ceiling,
        "2.32816852809979214933373423e4835",
        "0x6.b014710df6d8d0fb19018E+4015",
        Greater,
    );
    test(
        "8.0125492416043474e10778525511745053023",
        "0x4.f38a550e9d4aE+8951371679731519077",
        13,
        Up,
        "8.0139e10778525511745053023",
        "0x4.f3cE+8951371679731519077",
        Greater,
    );
    test(
        "-0.367879441171442321595523770161460867445811131031767834507837",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c77333",
        10,
        Nearest,
        "0.44385",
        "0x0.71a",
        Less,
    );
    test(
        "-0.3678794411714423215955237701614608674458111310317678345078373",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c773331",
        10,
        Nearest,
        "0.44385",
        "0x0.71a",
        Less,
    );
    test("-1.0", "-0x1.0", 10, Nearest, "0.2783", "0x0.474", Less);
    test(
        "-10.0",
        "-0xa.0",
        20,
        Floor,
        "0.000045397843",
        "0x0.0002f9a64",
        Less,
    );
    test(
        "-100.0",
        "-0x64.0",
        20,
        Ceiling,
        "3.72008e-44",
        "0xd.461E-37",
        Greater,
    );
    test(
        "-1000.0",
        "-0x3e8.0",
        20,
        Nearest,
        "5.0759604e-435",
        "0x2.78844E-361",
        Greater,
    );
    test("1.0", "0x1.0", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.56738", "0x0.914", Greater);
    test("100.0", "0x64.0", 20, Up, "95.441528", "0x5f.7108", Greater);
    // - omega_encl residual below -1/2
    // - omega_encl residual may be negative
    // - omega_encl residual not positive
    // - omega_refine no steps
    // - omega_seed f64 seed from x - ln x
    // - wright_bracket enclosure accepted
    // - wright_omega_huge applies
    // - wright_omega_huge undecided
    test(
        "1.3292279957849158729038420e36",
        "0x1.0000000000000000001fcE+30",
        1,
        Up,
        "2.7e36",
        "0x2.0E+30",
        Greater,
    );
    // - omega_encl residual at least -1/2
    // - omega_encl residual may be positive
    // - omega_encl residual not negative
    // - omega_refine steps
    // - omega_seed f64 seed from the tangent at 0
    // - wright_omega_huge not large enough
    test("0.062", "0x0.1", 4, Up, "0.62", "0x0.a", Greater);
    // - omega_seed f64 seed from e^x
    test("-1.12", "-0x1.2", 1, Up, "0.50", "0x0.8", Greater);
    // - wright_bracket very negative argument
    test("-26.0", "-0x1a.0", 1, Up, "7.3e-12", "0x8.0E-10", Greater);
}

// Arguments near the top of the range of `f64` at precisions too high for the large-argument path,
// whose results are too long to write out, are compared with the reference.
#[test]
fn test_wright_omega_prec_round_reference() {
    let test = |x_hex: &str, prec: u64| {
        let x = hex(x_hex);
        for rm in [Floor, Ceiling, Nearest] {
            assert!(same_pair(
                &wright_omega_prec_round_all(&x, prec, rm),
                &wright_omega_prec_round_naive(&x, prec, rm)
            ));
        }
    };
    test("0x1.0E+255", 1000);
    // - omega_seed 64-bit seed x - ln x
    test("0x1.0E+258", 1100);
}

#[test]
fn test_wright_omega_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = wright_omega_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1", 53, Nearest, "1.0", "0x1.0", Equal);
    test("1", 10, Floor, "1.0", "0x1.0", Equal);
    test("1", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("1", 10, Down, "1.0", "0x1.0", Equal);
    test("1", 10, Up, "1.0", "0x1.0", Equal);
    test("1", 10, Nearest, "1.0", "0x1.0", Equal);
    test(
        "1/3",
        53,
        Nearest,
        "0.69589287666479105",
        "0x0.b226091acb6d78",
        Greater,
    );
    test("1/3", 10, Floor, "0.6953", "0x0.b2", Less);
    test("1/3", 10, Ceiling, "0.69629", "0x0.b24", Greater);
    test("1/3", 10, Down, "0.6953", "0x0.b2", Less);
    test("1/3", 10, Up, "0.69629", "0x0.b24", Greater);
    test("1/3", 10, Nearest, "0.69629", "0x0.b24", Greater);
    test(
        "-1/3",
        53,
        Nearest,
        "0.45472628316922337",
        "0x0.7468f112d7ec5",
        Less,
    );
    test("-1/3", 10, Floor, "0.45459", "0x0.746", Less);
    test("-1/3", 10, Ceiling, "0.4551", "0x0.748", Greater);
    test("-1/3", 10, Down, "0.45459", "0x0.746", Less);
    test("-1/3", 10, Up, "0.4551", "0x0.748", Greater);
    test("-1/3", 10, Nearest, "0.45459", "0x0.746", Less);
    test(
        "22/7",
        53,
        Nearest,
        "2.3069366318254239",
        "0x2.4e93662ba2746",
        Greater,
    );
    test("22/7", 10, Floor, "2.305", "0x2.4e", Less);
    test("22/7", 10, Ceiling, "2.3086", "0x2.4f", Greater);
    test("22/7", 10, Down, "2.305", "0x2.4e", Less);
    test("22/7", 10, Up, "2.3086", "0x2.4f", Greater);
    test("22/7", 10, Nearest, "2.3086", "0x2.4f", Greater);
    test(
        "-1/4",
        53,
        Nearest,
        "0.48128846639758643",
        "0x0.7b35b88f1e9ffc",
        Less,
    );
    test("-1/4", 10, Floor, "0.48096", "0x0.7b2", Less);
    test("-1/4", 10, Ceiling, "0.4814", "0x0.7b4", Greater);
    test("-1/4", 10, Down, "0.48096", "0x0.7b2", Less);
    test("-1/4", 10, Up, "0.4814", "0x0.7b4", Greater);
    test("-1/4", 10, Nearest, "0.4814", "0x0.7b4", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "0.56678146783229544",
        "0x0.9118971c518ec8",
        Greater,
    );
    test("-1/1000", 10, Floor, "0.5664", "0x0.91", Less);
    test("-1/1000", 10, Ceiling, "0.56738", "0x0.914", Greater);
    test("-1/1000", 10, Down, "0.5664", "0x0.91", Less);
    test("-1/1000", 10, Up, "0.56738", "0x0.914", Greater);
    test("-1/1000", 10, Nearest, "0.5664", "0x0.91", Less);
    test(
        "1000000000000000000000000000000",
        53,
        Nearest,
        "1.0000000000000000e30",
        "0xc.9f2c9cd04675E+24",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "9.9902e29",
        "0xc.9cE+24",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "1.000e30",
        "0xc.aE+24",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "9.9902e29",
        "0xc.9cE+24",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "1.000e30",
        "0xc.aE+24",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Nearest,
        "1.000e30",
        "0xc.aE+24",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        53,
        Nearest,
        "0.56714329040978384",
        "0x0.91304d7c74b2b8",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Floor,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Ceiling,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Down,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Up,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Nearest,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        53,
        Nearest,
        "0.56714329040978384",
        "0x0.91304d7c74b2b8",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Floor,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Ceiling,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Down,
        "0.5664",
        "0x0.91",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Up,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Nearest,
        "0.56738",
        "0x0.914",
        Greater,
    );
    test(
        "-11/30",
        53,
        Nearest,
        "0.44438878688970168",
        "0x0.71c376aa667f74",
        Greater,
    );
    test("-11/30", 10, Floor, "0.4443", "0x0.71c", Less);
    test("-11/30", 10, Ceiling, "0.44482", "0x0.71e", Greater);
    test("-11/30", 10, Down, "0.4443", "0x0.71c", Less);
    test("-11/30", 10, Up, "0.44482", "0x0.71e", Greater);
    test("-11/30", 10, Nearest, "0.4443", "0x0.71c", Less);
    test("-3/8", 10, Nearest, "0.44189", "0x0.712", Greater);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "0.44401576630745326",
        "0x0.71ab046b32ea7c",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "0.44385",
        "0x0.71a",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "0.4443",
        "0x0.71c",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "0.44385",
        "0x0.71a",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "0.4443",
        "0x0.71c",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "0.44385",
        "0x0.71a",
        Less,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "0.44385",
        "0x0.71a",
        Less,
    );
    test("1", 10, Exact, "1.0", "0x1.0", Equal);
    test("0", 10, Nearest, "0.56738", "0x0.914", Greater);
    test("-10", 20, Nearest, "0.000045397843", "0x0.0002f9a64", Less);
    test("-1/3", 20, Floor, "0.4547262", "0x0.7468f", Less);
    test("1000", 20, Ceiling, "993.0996", "0x3e1.198", Greater);
    // - omega_encl residual at least -1/2
    // - omega_encl residual below -1/2
    // - omega_encl residual may be negative
    // - omega_encl residual may be positive
    // - omega_encl residual not negative
    // - omega_refine steps
    // - omega_seed f64 seed from x - ln x
    // - wright_bracket enclosure accepted
    // - wright_omega_huge applies
    // - wright_omega_huge undecided
    // - wright_omega_rational_helper large-argument guard
    // - ziv_loop_certified_from retry
    test(
        "85070591730234615865843651857942052927/16",
        2,
        Down,
        "4.0e36",
        "0x3.0E+30",
        Less,
    );
    // - omega_encl residual not positive
    // - omega_refine no steps
    // - omega_seed f64 seed from the tangent at 0
    test("0", 1, Up, "1.0", "0x1.0", Greater);
    // - wright_bracket very negative argument
    // - wright_omega_huge not large enough
    test(
        "-6157915351281569205661",
        1,
        Floor,
        "3.9e-2674348657088910078605",
        "0x4.0E-2220998484876984737334",
        Less,
    );
    // - omega_seed f64 seed from e^x
    test("-2", 1, Up, "0.12", "0x0.2", Greater);
}

#[test]
#[should_panic]
fn wright_omega_prec_round_fail_1() {
    BigExponentFloat::ONE.wright_omega_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn wright_omega_prec_round_fail_2() {
    BigExponentFloat::from(0.5).wright_omega_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn wright_omega_prec_round_fail_3() {
    BigExponentFloat::ZERO.wright_omega_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_round_fail_1() {
    BigExponentFloat::wright_omega_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_round_fail_2() {
    BigExponentFloat::wright_omega_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn wright_omega_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = wright_omega_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !x.is_nan() {
        // omega is positive and increasing, and omega(1) = 1.
        assert!(r.0 >= 0u32);
        if *x < 1u32 {
            assert!(r.0 <= 1u32);
        } else {
            assert!(r.0 >= 1u32);
        }
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The reference computes W_0(e^x), in time that grows with x.
        if x.is_finite() && *x != 1u32 && reference_is_cheap(x) {
            assert!(same_pair(&wright_omega_prec_round_naive(x, prec, rm), &r));
        }
        // The result lies between the results rounded down and up.
        let floor = wright_omega_prec_round_all(x, prec, Floor);
        let ceiling = wright_omega_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&wright_omega_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = wright_omega_prec_round_all(x, prec + 16, rm);
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
            &wright_omega_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn wright_omega_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_25().test_properties(
        |(x, prec, rm)| {
            wright_omega_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_25().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            wright_omega_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn wright_omega_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.wright_omega_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = wright_omega_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn wright_omega_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_25().test_properties(|(x, prec, rm)| {
        let r = wright_omega_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        assert!(r.0 >= 0u32);
        if x < 1u32 {
            assert!(r.0 <= 1u32);
        } else {
            assert!(r.0 >= 1u32);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if x != 1u32 && x.le_abs(&256u32) {
                assert!(same_pair(
                    &wright_omega_rational_prec_round_naive(&x, prec, rm),
                    &r
                ));
            }
            if rm != Nearest {
                let (y, o) = wright_omega_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn wright_omega_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_25().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::wright_omega_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = wright_omega_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
